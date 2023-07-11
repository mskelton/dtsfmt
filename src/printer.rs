use tree_sitter::TreeCursor;

use crate::{
    context::Context,
    layouts::KeyboardLayout,
    parser::parse,
    utils::{get_text, lookbehind, pad_right, print_indent},
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
            let name = get_text(source, cursor);
            writer.push_str(name);
            writer.push_str(" {\n");

            // Node body
            while cursor.goto_next_sibling() {
                let ctx = ctx.inc(1);

                // When we find the keymap node, we need to set the keymap flag
                // so we can properly print the binding cells.
                let ctx = match name {
                    "keymap" => ctx.keymap(),
                    _ => ctx,
                };

                traverse(writer, &source, cursor, &ctx);
            }

            // Node closing
            print_indent(writer, ctx);
            writer.push_str("};\n");
            cursor.goto_parent();
        }
        "property" => {
            cursor.goto_first_child();
            print_indent(writer, ctx);

            let name = get_text(source, cursor);
            writer.push_str(name);

            cursor.goto_next_sibling();
            writer.push_str(" = ");

            while cursor.goto_next_sibling() {
                // When we are inside a bindings node, we want to increase the
                // indentation level and print the bindings according to the
                // keyboard layout.
                let ctx = match name {
                    "bindings" => ctx.inc(1).bindings(),
                    _ => ctx.with_indent(0),
                };

                match cursor.node().kind() {
                    "," => writer.push_str(", "),
                    ";" => break,
                    _ => traverse(writer, &source, cursor, &ctx),
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

            // Keymap bindings are a special snowflake
            if ctx.keymap && ctx.bindings {
                print_bindings(writer, source, cursor, ctx);
                return;
            }

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

fn print_bindings(writer: &mut String, source: &String, cursor: &mut TreeCursor, ctx: &Context) {
    cursor.goto_first_child();
    writer.push_str("<\n");
    print_indent(writer, ctx);

    let mut index = 0;
    let mut buf = String::new();

    while cursor.goto_next_sibling() {
        match cursor.node().kind() {
            ">" => break,
            _ => {
                let text = get_text(source, cursor).trim();
                if !buf.is_empty() && text.starts_with("&") {
                    // Determine the column span of the current binding from
                    // the specified keyboard layout.
                    let col_span = ctx.layout.bindings.get(index).unwrap_or(&0);

                    // Determine if the current binding is the last key in the row
                    let hit_breakpoint = ctx.layout.breakpoints.contains(&index);

                    // Increment the index since this is the start of a new key
                    index += 1;

                    // Don't add padding to the last binding in the row
                    let padding = match hit_breakpoint {
                        true => 0,
                        false => 20 * (col_span + 1),
                    };

                    // Flush the buffer
                    writer.push_str(&pad_right(&buf.trim(), padding));

                    // After flushing the buffer to the writer, we need to
                    // clear it for the next binding.
                    buf.clear();

                    // Add a newline if we are at a breakpoint
                    if hit_breakpoint {
                        writer.push('\n');
                        print_indent(writer, ctx);
                    }
                }

                // Add the current piece of text to the buffer
                buf.push_str(text);
                buf.push(' ');
            }
        }
    }

    // Flush the final buffer
    writer.push_str(&buf.trim());

    // Close the bindings
    writer.push('\n');
    print_indent(writer, &ctx.dec(1));
    writer.push('>');

    cursor.goto_parent();
}

pub fn print(source: &String, layout: &KeyboardLayout) -> String {
    let mut writer = String::new();
    let tree = parse(source.clone());
    let mut cursor = tree.walk();

    let ctx = Context {
        indent: 0,
        bindings: false,
        keymap: false,
        layout,
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
