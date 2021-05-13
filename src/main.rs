use tree_sitter::{Parser, Language};
// extern "C" { fn tree_sitter_c() -> Language; }
// extern "C" { fn tree_sitter_rust() -> Language; }

extern "C" { fn tree_sitter_javascript() -> Language; }

fn main() {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_javascript() };
    parser.set_language(language).unwrap();

    let source_code = "import {sayHi} from './say.js';
const x = 1;
console.log(x);";
    let tree = parser.parse(source_code, None).unwrap();

    println!("{:?}", tree.root_node().to_sexp());
    println!("{:?}", tree.root_node().child(0));
    println!("{:?}", tree.root_node().child(1));
}
