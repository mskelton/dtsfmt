pub(crate) use self::files::*;
pub(crate) use self::stdout::*;

use std::{io, path::Path};

use crate::config::Filename;

mod files;
mod stdout;

pub(crate) struct FormattedFile<'a> {
    pub(crate) filename: &'a Filename,
    pub(crate) original_text: &'a str,
    pub(crate) formatted_text: &'a str,
}

#[derive(Debug, Default)]
pub(crate) struct EmitterResult {}

pub(crate) trait Emitter {
    fn emit_formatted_file(
        &mut self,
        formatted_file: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error>;
}

fn ensure_real_path(filename: &Filename) -> &Path {
    match *filename {
        Filename::Real(ref path) => path,
        _ => panic!("cannot format `{}` and emit to files", filename),
    }
}
