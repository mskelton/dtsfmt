use ignore::gitignore::GitignoreBuilder;
use ignore::{types::TypesBuilder, WalkBuilder};
use std::path::Path;
use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

use clap::Parser;
use dtsfmt::{
    config::Config,
    emitter::{self, create_emitter, Emitter, FormattedFile},
};

#[derive(PartialEq)]
enum FormattingStatus {
    Changed,
    Unchanged,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Check for formatting errors without writing to the file
    #[arg(long, short)]
    check: bool,

    /// Read from stdin instead of a file and emit to stdout
    #[arg(long)]
    stdin: bool,

    /// The file to format
    #[arg(index = 1, value_name = "FILE")]
    file_path: PathBuf,
}

fn format_fs(cli: &Cli, config: &Config) -> bool {
    let mut emitter = create_emitter(false);
    let mut has_errors = false;

    let mut types = TypesBuilder::new();
    types.add_defaults();
    types.add("devicetree", "*.keymap").unwrap();
    types.select("devicetree");

    for result in WalkBuilder::new(&cli.file_path)
        .types(types.build().unwrap())
        .add_custom_ignore_filename(".dtsfmtignore")
        .standard_filters(false)
        .build()
    {
        let result = result.expect("Failed to walk directory");
        if !result.file_type().map_or(false, |ft| ft.is_file()) {
            continue;
        }

        // Read the file contents from the file
        let path = result.path();
        let buffer = fs::read_to_string(&path).expect("Failed to read file");

        let status = format(
            path.to_path_buf(),
            buffer,
            &mut emitter,
            &config,
            cli.check,
        );

        has_errors |= status == FormattingStatus::Changed;
    }

    return has_errors;
}

fn format_stdin(cli: &Cli, config: &Config) -> bool {
    let mut emitter = create_emitter(true);
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Failed to read stdin");

    // If the file is ignored, we need to print the original content unchanged
    // since we still need to return content when running in stdin mode.
    let status = if is_ignored(&cli.file_path) {
        print_original(cli, &mut emitter, &buffer)
    } else {
        format(PathBuf::from("stdin"), buffer, &mut emitter, &config, cli.check)
    };

    return status == FormattingStatus::Changed;
}

fn main() {
    let cli = Cli::parse();
    let config = Config::parse(&cli.file_path.to_path_buf());

    let has_errors = if cli.stdin {
        format_stdin(&cli, &config)
    } else {
        format_fs(&cli, &config)
    };

    if cli.check && has_errors {
        println!("\nErrors found while formatting!");
        std::process::exit(1);
    }
}

fn is_ignored(filename: &PathBuf) -> bool {
    let mut builder =
        GitignoreBuilder::new(filename.parent().unwrap().parent().unwrap());
    builder.add(Path::new(".dtsfmtignore"));
    let gitignore = builder.build().unwrap();

    return gitignore.matched(filename, false).is_ignore();
}

fn print_original(
    cli: &Cli,
    emitter: &mut Box<dyn Emitter>,
    buffer: &String,
) -> FormattingStatus {
    let file = FormattedFile {
        filename: &PathBuf::from("stdin"),
        original_text: &buffer,
        formatted_text: &buffer,
    };

    return emit(emitter, file, &buffer, &buffer, cli.check);
}

fn format(
    filename: PathBuf,
    source: String,
    emitter: &mut Box<dyn Emitter>,
    config: &Config,
    check: bool,
) -> FormattingStatus {
    let output = dtsfmt::printer::print(&source, &config.layout);
    let result = FormattedFile {
        filename: &filename,
        original_text: &source,
        formatted_text: &output,
    };

    return emit(emitter, result, &output, &source, check);
}

fn emit(
    emitter: &mut Box<dyn Emitter>,
    result: FormattedFile,
    output: &String,
    source: &String,
    check: bool,
) -> FormattingStatus {
    // When the --check flag is false, we emit the changes.
    if !check {
        emitter
            .emit_formatted_file(result)
            .expect("Failed to emit formatted file");

        return FormattingStatus::Changed;
    }

    if output != source {
        emitter.emit_check(result).expect("Failed to emit check result");

        return FormattingStatus::Changed;
    }

    return FormattingStatus::Unchanged;
}
