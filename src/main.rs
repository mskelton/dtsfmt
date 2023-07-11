mod config;
mod context;
mod emitter;
mod layouts;
mod parser;
mod printer;
mod utils;

use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

use clap::Parser;
use config::Filename;
use emitter::{Emitter, FormattedFile};
use layouts::KeyboardLayoutType;

/// What dtsfmt should emit. Mostly corresponds to the `--emit` command line
/// option.
#[derive(clap::ValueEnum, Clone, Debug)]
enum EmitMode {
    /// Emits to files
    Files,
    /// Writes the output to stdout
    Stdout,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The keyboard layout to use
    #[arg(long, default_value = "kinesis:adv360")]
    layout: KeyboardLayoutType,

    /// Check for formatting errors without writing to the file
    #[arg(long)]
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

    let layout = layouts::get_layout(cli.layout);
    let output = printer::print(&source, &layout);
    let result = FormattedFile {
        filename: &filename,
        original_text: &source,
        formatted_text: &output,
    };

    create_emitter(cli.emit)
        .emit_formatted_file(result)
        .expect("Failed to emit formatted file");
}

fn create_emitter<'a>(emit_mode: EmitMode) -> Box<dyn Emitter + 'a> {
    match emit_mode {
        EmitMode::Files => Box::new(emitter::FilesEmitter::new()),
        EmitMode::Stdout => Box::new(emitter::StdoutEmitter::new()),
    }
}
