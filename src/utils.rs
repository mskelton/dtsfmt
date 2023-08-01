use tree_sitter::{Node, TreeCursor};

use crate::context::Context;

pub fn lookbehind<'a>(cursor: &'a TreeCursor) -> Option<Node<'a>> {
    cursor.node().prev_sibling().or_else(|| {
        // If the previous sibling is not found, we traverse up the tree to the
        // parent, find the current location of the node, and then find the
        // previous sibling of the node.
        cursor.node().parent().and_then(|parent| {
            let mut cur = cursor.clone();
            cur.reset(parent);

            return parent.children(&mut cur).find(|n| n.eq(&cursor.node()));
        })
    })
}

pub fn lookahead<'a>(cursor: &'a TreeCursor) -> Option<Node<'a>> {
    cursor.node().next_sibling().or_else(|| {
        // If the next sibling is not found, we traverse up the tree to the
        // parent, find the current location of the node, and then find the
        // next sibling of the node.
        cursor.node().parent().and_then(|parent| {
            let mut cur = cursor.clone();
            cur.reset(parent);

            return parent.children(&mut cur).find(|n| n.eq(&cursor.node()));
        })
    })
}

pub fn print_indent(writer: &mut String, ctx: &Context) {
    if let Ok(size) = ctx.indent.try_into() {
        writer.push_str("  ".repeat(size).as_str());
    }
}

pub fn sep(writer: &mut String) {
    if !writer.ends_with("\n\n") {
        writer.push('\n');
    }
}

pub fn get_text<'a>(source: &'a String, cursor: &mut TreeCursor) -> &'a str {
    return cursor.node().utf8_text(source.as_bytes()).unwrap_or("").trim();
}

pub fn pad_right(string: &str, size: usize) -> String {
    format!("{:width$}", string, width = size)
}
