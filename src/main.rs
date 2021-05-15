extern crate serde;

use tree_sitter::{Language};

// extern "C" { fn tree_sitter_c() -> Language; }
// extern "C" { fn tree_sitter_rust() -> Language; }

extern "C" { fn tree_sitter_javascript() -> Language; }

pub mod code_model;
pub mod identify;

fn main() {

}
