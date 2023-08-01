use std::collections::VecDeque;

use tree_sitter::TreeCursor;

use crate::{
    context::Context,
    layouts::{self, KeyboardLayoutType},
    parser::parse,
    utils::{get_text, lookahead, lookbehind, pad_right, print_indent, sep},
};

fn traverse(
    writer: &mut String,
    source: &String,
    cursor: &mut TreeCursor,
    ctx: &Context,
) {
    let node = cursor.node();

    match node.kind() {
        "comment" => {
            // Add a newline before the comment if the previous node is not a comment
            if lookbehind(cursor).map_or(false, |n| n.kind() != "comment") {
                sep(writer);
            }

            print_indent(writer, ctx);
            let comment = get_text(source, cursor);

            // Only reformat single line comments, multi line comments are a
            // lot tougher to format properly.
            match comment.starts_with("//") {
                true => {
                    writer.push_str("// ");
                    writer.push_str(comment.trim_start_matches("//").trim());
                }
                false => writer.push_str(comment),
            }

            writer.push('\n');
        }
        "preproc_include" => {
            cursor.goto_first_child();
            print_indent(writer, ctx);
            writer.push_str("#include ");

            cursor.goto_next_sibling();
            writer.push_str(get_text(source, cursor));
            writer.push('\n');

            cursor.goto_parent();

            // Add a newline if this is the last include
            if lookahead(cursor)
                .map_or(false, |n| n.kind() != "preproc_include")
            {
                writer.push('\n');
            }
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

            // Add a newline if the next item is a node
            if lookahead(cursor).map_or(false, |n| n.kind() == "node") {
                writer.push('\n');
            }
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

fn collect_bindings(
    cursor: &mut TreeCursor,
    source: &String,
    ctx: &Context,
) -> VecDeque<String> {
    let mut buf: VecDeque<String> = VecDeque::new();
    let mut item = String::new();

    while cursor.goto_next_sibling() {
        match cursor.node().kind() {
            ">" => break,
            _ => {
                let text = get_text(source, cursor).trim();

                // If this is a new binding, add a new item to the buffer
                if !item.is_empty() && text.starts_with("&") {
                    buf.push_back(item);
                    item = String::new();
                }

                // Add a space between each piece of text
                if !item.is_empty() {
                    item.push(' ');
                }

                // Add the current piece of text to the buffer
                item.push_str(text);
            }
        }
    }

    // Add the last item to the buffer
    buf.push_back(item);

    // Move the items from the temporary buffer into a new vector that contains
    // the empty key spaces.
    ctx.layout
        .bindings
        .iter()
        .map(|is_key| match is_key {
            1 => buf.pop_front().unwrap_or(String::new()),
            _ => String::new(),
        })
        .collect()
}

/// Calculate the maximum size of each column in the bindings table.
fn calculate_sizes(buf: &VecDeque<String>, row_size: usize) -> Vec<usize> {
    let mut sizes = Vec::new();

    for i in 0..row_size {
        let mut max = 0;

        for j in (i..buf.len()).step_by(row_size) {
            let len = buf[j].len();

            if len > max {
                max = len;
            }
        }

        sizes.push(max);
    }

    sizes
}

fn print_bindings(
    writer: &mut String,
    source: &String,
    cursor: &mut TreeCursor,
    ctx: &Context,
) {
    cursor.goto_first_child();
    writer.push_str("<");

    let buf = collect_bindings(cursor, source, ctx);
    let row_size = ctx.layout.row_size();
    let sizes = calculate_sizes(&buf, row_size);

    buf.iter().enumerate().for_each(|(i, item)| {
        let col = i % row_size;

        // Add a newline at the start of each row
        if col == 0 {
            writer.push('\n');
            print_indent(writer, ctx);
        }

        // Don't add padding to the last binding in the row
        let padding = match (i + 1) % row_size == 0 {
            true => 0,
            false => sizes[col] + 3,
        };

        writer.push_str(&pad_right(&item, padding));
    });

    // Close the bindings
    writer.push('\n');
    print_indent(writer, &ctx.dec(1));
    writer.push('>');

    cursor.goto_parent();
}

pub fn print(source: &String, layout: &KeyboardLayoutType) -> String {
    let mut writer = String::new();
    let tree = parse(source.clone());
    let mut cursor = tree.walk();

    let layout = layouts::get_layout(layout);
    let ctx =
        Context { indent: 0, bindings: false, keymap: false, layout: &layout };

    // The first node is the root document node, so we have to traverse all it's
    // children with the same indentation level.
    cursor.goto_first_child();
    traverse(&mut writer, &source, &mut cursor, &ctx);

    while cursor.goto_next_sibling() {
        traverse(&mut writer, &source, &mut cursor, &ctx);
    }

    return writer;
}
