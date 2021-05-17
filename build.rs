use std::path::PathBuf;

fn main() {
    let js_dir: PathBuf = ["tree-sitter-javascript", "src"].iter().collect();
    cc::Build::new()
        .include(&js_dir)
        .file(js_dir.join("parser.c"))
        .file(js_dir.join("scanner.c"))
        .warnings(false)
        .compile("tree-sitter-javascript");

    let java_dir: PathBuf = ["tree-sitter-java", "src"].iter().collect();
    cc::Build::new()
        .include(&java_dir)
        .file(java_dir.join("parser.c"))
        .compile("tree-sitter-java");

    let rust_dir: PathBuf = ["tree-sitter-rust", "src"].iter().collect();
    cc::Build::new()
        .include(&rust_dir)
        .file(rust_dir.join("parser.c"))
        .file(rust_dir.join("scanner.c"))
        .warnings(false)
        .compile("tree-sitter-rust");
}