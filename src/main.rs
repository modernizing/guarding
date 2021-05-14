use tree_sitter::{Language, Tree, Node};
// extern "C" { fn tree_sitter_c() -> Language; }
// extern "C" { fn tree_sitter_rust() -> Language; }

extern "C" { fn tree_sitter_javascript() -> Language; }

pub mod code_model;
pub mod identify;

fn main() {

}

#[allow(unused_assignments)]
fn get_all_nodes(tree: &Tree) -> Vec<Node> {
    let mut result = Vec::new();

    let mut cursor = tree.walk();
    // let mut needs_newline = false;
    // let mut indent_level = 0;
    let mut did_visit_children = false;
    loop {
        let node = cursor.node();
        let is_named = node.is_named();
        if did_visit_children {
            if is_named {
                // needs_newline = true;
            }
            if cursor.goto_next_sibling() {
                did_visit_children = false;
            } else if cursor.goto_parent() {
                did_visit_children = true;
                // indent_level -= 1;
            } else {
                break;
            }
        } else {
            if is_named {
                result.push(node);
                // needs_newline = true;
            }
            if cursor.goto_first_child() {
                did_visit_children = false;
                // indent_level += 1;
            } else {
                did_visit_children = true;
            }
        }
    }
    return result;
}
