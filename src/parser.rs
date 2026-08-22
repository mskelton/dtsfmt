use tree_sitter::{Language, Parser, Tree};

extern "C" {
    fn tree_sitter_devicetree() -> Language;
}

pub fn parse(source: String) -> Tree {
    let language = unsafe { tree_sitter_devicetree() };
    let mut parser = Parser::new();

    parser.set_language(&language).unwrap();
    parser.parse(source, None).unwrap()
}
