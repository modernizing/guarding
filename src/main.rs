extern crate pest;
#[macro_use]
extern crate pest_derive;

extern crate serde;

use tree_sitter::Language;

extern "C" { fn tree_sitter_rust() -> Language; }
extern "C" { fn tree_sitter_java() -> Language; }
extern "C" { fn tree_sitter_javascript() -> Language; }

pub mod identify;
pub mod parser;

fn main() {

}
