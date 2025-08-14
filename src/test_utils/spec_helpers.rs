use std::fmt::Display;
use std::path::Path;

use console::Style;
use similar::{ChangeTag, TextDiff};

use super::get_specs_in_dir;
use crate::config::Config;
use crate::layouts::KeyboardLayoutType;
use crate::printer::print;

struct FailedTestResult {
    expected: String,
    actual: String,
    message: String,
}

struct DiffFailedMessage<'a> {
    expected: &'a str,
    actual: &'a str,
}

impl Display for DiffFailedMessage<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let diff = TextDiff::from_lines(self.expected, self.actual);

        for op in diff.ops() {
            for change in diff.iter_changes(op) {
                let (sign, style) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().green()),
                    ChangeTag::Insert => ("+", Style::new().red()),
                    ChangeTag::Equal => (" ", Style::new()),
                };

                write!(
                    f,
                    "{}{}",
                    style.apply_to(sign).bold(),
                    style.apply_to(change),
                )?;
            }
        }

        Ok(())
    }
}

pub fn run_specs(directory_path: &Path) {
    let specs = get_specs_in_dir(directory_path);
    let test_count = specs.len();
    let mut failed_tests = Vec::new();
    let config = Config {
        layout: KeyboardLayoutType::Adv360,
        warn_on_unhandled_tokens: false,
    };

    for (_, spec) in specs {
        let result = print(&spec.file_text, &config);

        if result != spec.expected_text {
            failed_tests.push(FailedTestResult {
                expected: spec.expected_text.clone(),
                actual: result,
                message: spec.message.clone(),
            });
        }
    }

    for failed_test in &failed_tests {
        println!("---");
        println!(
            "Failed:   {}\nExpected: `{:?}`\nActual:   `{:?}`\nDiff:\n{}",
            failed_test.message,
            failed_test.expected,
            failed_test.actual,
            DiffFailedMessage {
                actual: &failed_test.actual,
                expected: &failed_test.expected
            }
        );
    }

    if !failed_tests.is_empty() {
        println!("---");
        panic!(
            "{}/{} tests passed",
            test_count - failed_tests.len(),
            test_count
        );
    }
}
