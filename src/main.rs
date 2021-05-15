extern crate serde;

use tree_sitter::{Language};

extern "C" { fn tree_sitter_rust() -> Language; }
extern "C" { fn tree_sitter_java() -> Language; }
extern "C" { fn tree_sitter_javascript() -> Language; }

pub mod code_model;
pub mod location;
pub mod identify;

fn main() {

}
