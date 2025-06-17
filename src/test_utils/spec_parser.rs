#[derive(PartialEq, Eq, Debug)]
pub struct Spec {
    pub message: String,
    pub file_text: String,
    pub expected_text: String,
    pub is_only: bool,
}

pub fn parse_specs(file_text: String) -> Vec<Spec> {
    let lines = file_text.split('\n').collect::<Vec<_>>();
    let spec_starts = get_spec_starts(&lines);
    let mut specs = Vec::new();

    for i in 0..spec_starts.len() {
        let start_index = spec_starts[i];
        let end_index = if spec_starts.len() == i + 1 {
            lines.len()
        } else {
            spec_starts[i + 1]
        };
        let message_line = lines[start_index];
        let spec = parse_single_spec(
            message_line,
            &lines[(start_index + 1)..end_index],
        );

        specs.push(spec);
    }

    return specs;

    fn get_spec_starts(lines: &[&str]) -> Vec<usize> {
        let mut result = Vec::new();
        let message_separator = get_message_separator();

        if !lines.first().unwrap().starts_with(message_separator) {
            panic!(
                "All spec files should start with a message. (ex. \
                 {message_separator} Message {message_separator})"
            );
        }

        for (i, line) in lines.iter().enumerate() {
            if line.starts_with(message_separator) {
                result.push(i);
            }
        }

        result
    }

    fn parse_single_spec(message_line: &str, lines: &[&str]) -> Spec {
        let file_text = lines.join("\n");
        let parts = file_text.split("[expect]").collect::<Vec<&str>>();
        let start_text = parts[0][0..parts[0].len() - "\n".len()].into(); // remove last newline
        let expected_text = parts[1]["\n".len()..].into(); // remove first newline
        let message_separator = get_message_separator();
        let lower_case_message_line = message_line.to_ascii_lowercase();

        Spec {
            message: message_line[message_separator.len()
                ..message_line.len() - message_separator.len()]
                .trim()
                .into(),
            file_text: start_text,
            expected_text,
            is_only: lower_case_message_line.contains("(only)"),
        }
    }

    fn get_message_separator() -> &'static str {
        "=="
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let specs = parse_specs(
            vec![
                "== message 1 ==",
                "start",
                "multiple",
                "",
                "[expect]",
                "expected",
                "multiple",
                "",
                "== message 2 (only) (skip) (skip-format-twice) ==",
                "start2",
                "",
                "[expect]",
                "expected2",
                "",
                "== message 3 (trace) ==",
                "test",
                "",
                "[expect]",
                "test",
                "",
            ]
            .join("\n"),
        );

        assert_eq!(specs.len(), 3);
        assert_eq!(
            specs[0],
            Spec {
                file_text: "start\nmultiple\n".into(),
                expected_text: "expected\nmultiple\n".into(),
                message: "message 1".into(),
                is_only: false,
            }
        );
        assert_eq!(
            specs[1],
            Spec {
                file_text: "start2\n".into(),
                expected_text: "expected2\n".into(),
                message: "message 2 (only) (skip) (skip-format-twice)".into(),
                is_only: true,
            }
        );
        assert_eq!(
            specs[2],
            Spec {
                file_text: "test\n".into(),
                expected_text: "test\n".into(),
                message: "message 3 (trace)".into(),
                is_only: false,
            }
        );
    }
}
