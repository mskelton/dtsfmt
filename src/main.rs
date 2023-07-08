mod formatter;
mod parser;

use std::{fs, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Check for formatting errors without writing to the file
    #[arg(long)]
    check: bool,

    /// The file to format
    #[arg(index = 1, value_name = "FILE")]
    file_path: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let source = fs::read_to_string(cli.file_path).expect("Failed to read file");
    let output = formatter::format(source);

    print!("{}", output);
}
