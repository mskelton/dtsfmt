use std::io;
use std::path::PathBuf;

pub use self::files::*;
pub use self::stdout::*;

mod files;
mod stdout;

pub struct FormattedFile<'a> {
    pub filename: &'a PathBuf,
    pub original_text: &'a str,
    pub formatted_text: &'a str,
}

#[derive(Debug, Default)]
pub struct EmitterResult {}

pub trait Emitter {
    fn emit_check(
        &mut self,
        formatted_file: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error>;

    fn emit_formatted_file(
        &mut self,
        formatted_file: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error>;
}

pub fn create_emitter<'a>(stdin: bool) -> Box<dyn Emitter + 'a> {
    match stdin {
        true => Box::new(StdoutEmitter::new()),
        false => Box::new(FilesEmitter::new()),
    }
}
