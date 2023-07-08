use tree_sitter::TreeCursor;

fn get_text<'a>(source: &'a String, cursor: &mut TreeCursor) -> &'a str {
    return cursor.node().utf8_text(source.as_bytes()).unwrap_or("");
}

fn traverse(writer: &mut String, source: &String, cursor: &mut TreeCursor, indent: usize) {
    let node = cursor.node();

    match node.kind() {
        "comment" => {
            writer.push_str(get_text(source, cursor));
            writer.push('\n');
        }
        "preproc_include" => {
            cursor.goto_first_child();
            writer.push_str("#include ");

            cursor.goto_next_sibling();
            writer.push_str(get_text(source, cursor));
            writer.push('\n');

            cursor.goto_parent();
        }
        "preproc_def" => {
            cursor.goto_first_child();
            writer.push_str("#define ");

            while cursor.goto_next_sibling() && cursor.node().kind() != "\n" {
                writer.push_str(get_text(source, cursor));
            }

            writer.push('\n');
            cursor.goto_parent();
        }
        "preproc_function_def" => {
            cursor.goto_first_child();
            writer.push_str("#define ");

            while cursor.goto_next_sibling() {
                writer.push_str(get_text(source, cursor));
            }

            cursor.goto_parent();
        }
        "labeled_item" => {
            cursor.goto_first_child();
            writer.push_str(get_text(source, cursor));
            writer.push_str(": ");

            while cursor.goto_next_sibling() {
                traverse(writer, &source, cursor, indent);
            }
        }
        "node" => {
            cursor.goto_first_child();
            writer.push_str(get_text(source, cursor));
            writer.push_str(" {\n");

            while cursor.goto_next_sibling() {
                traverse(writer, &source, cursor, indent + 1);
            }

            writer.push_str("};\n");
            cursor.goto_parent();
        }
        "property" => {
            cursor.goto_first_child();
            writer.push_str(get_text(source, cursor));

            cursor.goto_next_sibling();
            writer.push_str(" = ");

            while cursor.goto_next_sibling() {
                match cursor.node().kind() {
                    "," => writer.push_str(", "),
                    ";" => break,
                    _ => traverse(writer, &source, cursor, 0),
                }
            }

            writer.push_str(";\n");
            cursor.goto_parent();
        }
        "string_literal" => {
            writer.push_str(get_text(source, cursor));
        }
        "integer_cells" => {
            cursor.goto_first_child();
            writer.push('<');

            let mut first = true;
            while cursor.goto_next_sibling() {
                match cursor.node().kind() {
                    ">" => break,
                    _ => {
                        if first {
                            first = false;
                        } else {
                            writer.push(' ');
                        }

                        writer.push_str(get_text(source, cursor));
                    }
                }
            }

            writer.push('>');
            cursor.goto_parent();
        }
        _ => {
            if cursor.goto_first_child() {
                traverse(writer, &source, cursor, indent);

                while cursor.goto_next_sibling() {
                    traverse(writer, &source, cursor, indent + 1);
                }

                cursor.goto_parent();
            }
        }
    };
}

pub fn format(source: String) -> String {
    let mut buf = String::new();

    let tree = crate::parser::parse(source.clone());
    let mut cursor = tree.walk();

    traverse(&mut buf, &source, &mut cursor, 0);
    return buf;
}
