use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

use clap::Parser;
use dtsfmt::{
    config::{Config, Filename},
    emitter::{create_emitter, EmitMode, FormattedFile},
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
    let source = match &filename {
        Filename::Real(path) => fs::read_to_string(&path).expect("Failed to read file"),
        Filename::Stdin => {
            let mut buffer = String::new();

            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Failed to read stdin");

            buffer
        }
    };

    let output = dtsfmt::printer::print(&source, config.layout);
    let result = FormattedFile {
        filename: &filename,
        original_text: &source,
        formatted_text: &output,
    };

    create_emitter(cli.emit)
        .emit_formatted_file(result)
        .expect("Failed to emit formatted file");
}
