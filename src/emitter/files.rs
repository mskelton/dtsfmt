use super::*;
use std::fs;

#[derive(Debug, Default)]
pub struct FilesEmitter {}

impl FilesEmitter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Emitter for FilesEmitter {
    fn emit_check(
        &mut self,
        FormattedFile { filename, .. }: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error> {
        println!("{}", ensure_real_path(filename).display());

        Ok(EmitterResult::default())
    }

    fn emit_formatted_file(
        &mut self,
        FormattedFile {
            filename,
            original_text,
            formatted_text,
        }: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error> {
        // Write text directly over original file if there is a diff.
        let filename = ensure_real_path(filename);

        if original_text != formatted_text {
            fs::write(filename, formatted_text)?;
        }

        Ok(EmitterResult::default())
    }
}
