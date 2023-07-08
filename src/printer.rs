use tree_sitter::TreeCursor;

use crate::{
    context::Context,
    parser::parse,
    utils::{get_text, lookbehind, print_indent},
};

fn traverse(writer: &mut String, source: &String, cursor: &mut TreeCursor, ctx: &Context) {
    let node = cursor.node();

    match node.kind() {
        "comment" => {
            // Add a newline before the comment if the previous node is not a comment
            if lookbehind(cursor).map_or(false, |n| n.kind() != "comment") {
                writer.push('\n');
            }

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

            // Name
            cursor.goto_next_sibling();
            writer.push_str(get_text(source, cursor));
            writer.push(' ');

            // Value
            cursor.goto_next_sibling();
            writer.push_str(get_text(source, cursor));

            writer.push('\n');
            cursor.goto_parent();
        }
        "preproc_function_def" => {
            cursor.goto_first_child();
            writer.push_str("#define ");

            // Function and args
            for _ in 0..2 {
                cursor.goto_next_sibling();
                writer.push_str(get_text(source, cursor));
            }
            writer.push(' ');

            // Value
            cursor.goto_next_sibling();
            writer.push_str(get_text(source, cursor));

            writer.push('\n');
            cursor.goto_parent();
        }
        "labeled_item" => {
            cursor.goto_first_child();
            print_indent(writer, ctx);
            writer.push_str(get_text(source, cursor));
            writer.push_str(": ");

            while cursor.goto_next_sibling() {
                traverse(writer, &source, cursor, ctx);
            }

            cursor.goto_parent();
        }
        "node" => {
            // If the previous node is a labeled_item, then the labeled_item will
            // contain the indentation rather than the node.
            if lookbehind(cursor).map_or(false, |n| n.kind() != ":") {
                print_indent(writer, ctx);
            }

            cursor.goto_first_child();

            // Node name and opening
            writer.push_str(get_text(source, cursor));
            writer.push_str(" {\n");

            // Node body
            while cursor.goto_next_sibling() {
                traverse(writer, &source, cursor, &ctx.inc(1));
            }

            // Node closing
            print_indent(writer, ctx);
            writer.push_str("};\n");
            cursor.goto_parent();
        }
        "property" => {
            cursor.goto_first_child();
            print_indent(writer, ctx);
            writer.push_str(get_text(source, cursor));

            cursor.goto_next_sibling();
            writer.push_str(" = ");

            while cursor.goto_next_sibling() {
                match cursor.node().kind() {
                    "," => writer.push_str(", "),
                    ";" => break,
                    _ => traverse(writer, &source, cursor, &ctx.with_indent(0)),
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
                traverse(writer, &source, cursor, ctx);

                while cursor.goto_next_sibling() {
                    traverse(writer, &source, cursor, &ctx.inc(1));
                }

                cursor.goto_parent();
            }
        }
    };
}

pub fn print(source: String) -> String {
    let mut writer = String::new();
    let tree = parse(source.clone());
    let mut cursor = tree.walk();
    let ctx = Context {
        indent: 0,
        keymap: false,
    };

    // The first node is the root document node, so we have to traverse all it's
    // children with the same indentation level.
    cursor.goto_first_child();
    traverse(&mut writer, &source, &mut cursor, &ctx);

    while cursor.goto_next_sibling() {
        traverse(&mut writer, &source, &mut cursor, &ctx);
    }

    return writer;
}
