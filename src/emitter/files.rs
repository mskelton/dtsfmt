use std::fs;

use console::Style;
use similar::{ChangeTag, TextDiff};

use super::*;

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
        FormattedFile {
            filename,
            original_text,
            formatted_text,
        }: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error> {
        println!("{}", filename.display());

        let diff = TextDiff::from_lines(original_text, formatted_text);

        for op in diff.ops() {
            for change in diff.iter_changes(op) {
                let (sign, style) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().red()),
                    ChangeTag::Insert => ("+", Style::new().green()),
                    ChangeTag::Equal => continue,
                };

                print!(
                    "{}{}",
                    style.apply_to(sign).bold(),
                    style.apply_to(change),
                );
            }
        }

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
        if original_text != formatted_text {
            fs::write(filename, formatted_text)?;
        }

        Ok(EmitterResult::default())
    }
}
