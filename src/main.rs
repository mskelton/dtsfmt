use tree_sitter::{Language, Parser, Tree};

extern "C" {
    fn tree_sitter_devicetree() -> Language;
}

fn parse(source: String) -> Tree {
    let language = unsafe { tree_sitter_devicetree() };
    let mut parser = Parser::new();

    parser.set_language(language).unwrap();
    return parser.parse(source, None).unwrap();
}

fn main() {
    let source = String::from("#include <hi.dtsi>");
    let tree = parse(source);
    let root_node = tree.root_node();

    assert_eq!(root_node.kind(), "document");
    assert_eq!(root_node.start_position().column, 0);
    assert_eq!(root_node.end_position().column, 18);
}
