use std::path::PathBuf;

fn main() {
    let js_dir: PathBuf = ["tree-sitter-javascript", "src"].iter().collect();
    cc::Build::new()
        .include(&js_dir)
        .file(js_dir.join("parser.c"))
        .file(js_dir.join("scanner.c"))
        .compile("tree-sitter-javascript");

    let java_dir: PathBuf = ["tree-sitter-java", "src"].iter().collect();
    cc::Build::new()
        .include(&java_dir)
        .file(java_dir.join("parser.c"))
        .compile("tree-sitter-java");
}