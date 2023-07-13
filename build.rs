use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let dir: PathBuf = ["tree-sitter-devicetree", "src"].iter().collect();

    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .compile("tree-sitter-devicetree");
}
