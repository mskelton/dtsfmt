use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use clap::Parser;
use dtsfmt::config::Config;
use dtsfmt::emitter::{create_emitter, Emitter, FormattedFile};
use ignore::gitignore::GitignoreBuilder;
use ignore::types::TypesBuilder;
use ignore::WalkBuilder;

#[derive(PartialEq)]
enum FormattingStatus {
    Changed,
    Unchanged,
}

#[derive(Clone, Parser)]
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
    file_path: Option<PathBuf>,
}

fn format_fs(cli: &Cli, config: &Config, dir_path: &Path) -> bool {
    let mut emitter = create_emitter(false);
    let mut has_errors = false;

    let mut types = TypesBuilder::new();
    types.add_defaults();
    types.add("devicetree", "*.keymap").unwrap();
    types.select("devicetree");

    for result in WalkBuilder::new(dir_path)
        .types(types.build().unwrap())
        .add_custom_ignore_filename(".dtsfmtignore")
        .standard_filters(false)
        .build()
    {
        let result = result.expect("Failed to walk directory");
        if !result.file_type().is_some_and(|ft| ft.is_file()) {
            continue;
        }

        // Read the file contents from the file
        let path = result.path();
        let buffer = fs::read_to_string(path).expect("Failed to read file");

        let status =
            format(path.to_path_buf(), buffer, &mut emitter, config, cli.check);

        has_errors |= status == FormattingStatus::Changed;
    }

    has_errors
}

fn format_stdin(cli: &Cli, config: &Config, dir_path: &Path) -> bool {
    let mut emitter = create_emitter(true);
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Failed to read stdin");

    // If the file is ignored, we need to print the original content unchanged
    // since we still need to return content when running in stdin mode.
    let status = if is_ignored(dir_path) {
        print_original(cli, &mut emitter, &buffer)
    } else {
        format(PathBuf::from("stdin"), buffer, &mut emitter, config, cli.check)
    };

    status == FormattingStatus::Changed
}

fn main() {
    let cli = Cli::parse();

    let cfg_path = match &cli.file_path {
        Some(path) => path.clone(),
        None => std::env::current_dir().expect("Couldn't read CWD"),
    };
    let config = Config::parse(&cfg_path);

    // If no path was specified (likely with --stdin) then default to the
    // current working directory.
    let dir_path = match &cli.file_path {
        Some(path) => path.clone(),
        None => std::env::current_dir().expect("Couldn't read CWD"),
    };

    let has_errors = if cli.stdin {
        format_stdin(&cli, &config, &dir_path)
    } else {
        format_fs(&cli, &config, &dir_path)
    };

    if cli.check && has_errors {
        println!("\nErrors found while formatting!");
        std::process::exit(1);
    }
}

/// Find all `.dtsfmtignore` files in the parent directories of the given path.
fn find_ignore_files(start_path: &Path) -> Vec<PathBuf> {
    let mut ignore_files = Vec::new();
    let mut current_path = start_path.to_path_buf();

    while let Some(parent) = current_path.parent() {
        let ignore_path = parent.join(".dtsfmtignore");
        if ignore_path.exists() {
            ignore_files.push(ignore_path);
        }

        current_path = parent.to_path_buf();
    }

    ignore_files
}

/// Finds the project root by looking for a `.dtsfmtrc.toml` file in the parent
/// directories.
fn find_project_root(start_path: &Path) -> PathBuf {
    let mut current_path = start_path.to_path_buf();

    while let Some(parent) = current_path.parent() {
        let config_path = parent.join(".dtsfmtrc.toml");
        if config_path.exists() {
            return parent.to_path_buf();
        }

        current_path = parent.to_path_buf();
    }

    current_path
}

/// Checks if a given file is ignored. This is only necessary in stdin mode
/// where we don't use the `WalkBuilder` to filter out ignored files.
fn is_ignored(file_path: &Path) -> bool {
    let ignore_files = find_ignore_files(file_path.parent().unwrap());
    let root_path = find_project_root(file_path);
    let mut builder = GitignoreBuilder::new(root_path);

    for ignore_file in ignore_files {
        builder.add(ignore_file);
    }

    let ignore = builder.build().unwrap();
    ignore.matched_path_or_any_parents(file_path, false).is_ignore()
}

/// Prints the original contents of the file to stdout. This is necessary when
/// running in stdin mode and the file is ignored.
fn print_original(
    cli: &Cli,
    emitter: &mut Box<dyn Emitter>,
    buffer: &String,
) -> FormattingStatus {
    let file = FormattedFile {
        filename: &PathBuf::from("stdin"),
        original_text: buffer,
        formatted_text: buffer,
    };

    emit(emitter, file, buffer, buffer, cli.check)
}

/// Formats the given source code and emits the result.
fn format(
    filename: PathBuf,
    source: String,
    emitter: &mut Box<dyn Emitter>,
    config: &Config,
    check: bool,
) -> FormattingStatus {
    let output = dtsfmt::printer::print(&source, config);
    let result = FormattedFile {
        filename: &filename,
        original_text: &source,
        formatted_text: &output,
    };

    emit(emitter, result, &output, &source, check)
}

/// Emits the output of formatting either in check mode or by writing to the
/// file.
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

    FormattingStatus::Unchanged
}
