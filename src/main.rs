use ignore::{types::TypesBuilder, WalkBuilder};
use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

use clap::Parser;
use dtsfmt::{
    config::{Config, Filename},
    emitter::{create_emitter, EmitMode, Emitter, FormattedFile},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Check for formatting errors without writing to the file
    #[arg(long, short)]
    check: bool,

    /// How to emit the results
    #[arg(long, default_value = "files")]
    emit: EmitMode,

    /// The file to format
    #[arg(index = 1, value_name = "FILE")]
    file_path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let filename = match cli.file_path {
        Some(file) => Filename::Real(file.clone()),
        None => Filename::Stdin,
    };

    // If the file is real, we can read from it's directory to find the config
    // file. If it's stdin, we can't, so we just use the current directory.
    let config_path = match &filename {
        Filename::Real(path) => path.to_path_buf(),
        Filename::Stdin => std::env::current_dir().unwrap(),
    };

    let config = Config::parse(&config_path);
    let mut emitter = create_emitter(cli.emit);

    match &filename {
        Filename::Real(path) => {
            let mut types = TypesBuilder::new();
            types.add_defaults();
            types.add("devicetree", "*.keymap").unwrap();
            types.select("devicetree");

            for result in WalkBuilder::new(path)
                .types(types.build().unwrap())
                .add_custom_ignore_filename(".dtsfmtignore")
                .hidden(false)
                .build()
            {
                let result = result.expect("Failed to walk directory");
                if !result.file_type().map_or(false, |ft| ft.is_file()) {
                    continue;
                }

                let path = result.path();
                let buffer = fs::read_to_string(&path).expect("Failed to read file");

                format(
                    Filename::Real(path.to_path_buf()),
                    buffer,
                    &mut emitter,
                    &config,
                );
            }
        }
        Filename::Stdin => {
            let mut buffer = String::new();

            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Failed to read stdin");

            format(filename, buffer, &mut emitter, &config);
        }
    };
}

fn format(filename: Filename, source: String, emitter: &mut Box<dyn Emitter>, config: &Config) {
    let output = dtsfmt::printer::print(&source, &config.layout);
    let result = FormattedFile {
        filename: &filename,
        original_text: &source,
        formatted_text: &output,
    };

    emitter
        .emit_formatted_file(result)
        .expect("Failed to emit formatted file");
}
