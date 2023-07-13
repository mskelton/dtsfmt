pub use self::files::*;
pub use self::stdout::*;

use std::{io, path::Path};

use crate::config::Filename;

mod files;
mod stdout;

pub struct FormattedFile<'a> {
    pub filename: &'a Filename,
    pub original_text: &'a str,
    pub formatted_text: &'a str,
}

#[derive(Debug, Default)]
pub struct EmitterResult {}

pub trait Emitter {
    fn emit_formatted_file(
        &mut self,
        formatted_file: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error>;
}

/// What dtsfmt should emit. Mostly corresponds to the `--emit` command line
/// option.
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum EmitMode {
    /// Emits to files
    Files,
    /// Writes the output to stdout
    Stdout,
}

fn ensure_real_path(filename: &Filename) -> &Path {
    match *filename {
        Filename::Real(ref path) => path,
        _ => panic!("cannot format `{}` and emit to files", filename),
    }
}

pub fn create_emitter<'a>(emit_mode: EmitMode) -> Box<dyn Emitter + 'a> {
    match emit_mode {
        EmitMode::Files => Box::new(FilesEmitter::new()),
        EmitMode::Stdout => Box::new(StdoutEmitter::new()),
    }
}
