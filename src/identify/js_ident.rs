use crate::{tree_sitter_javascript};
use tree_sitter::{Parser, Tree, Node};

pub struct JsIdent {

}

fn get_all_nodes(tree: &Tree) -> Vec<Node> {
    let mut result = Vec::new();

    let mut cursor = tree.walk();
    let mut did_visit_children = false;
    loop {
        let node = cursor.node();
        let is_named = node.is_named();
        if did_visit_children {
            if cursor.goto_next_sibling() {
                did_visit_children = false;
            } else if cursor.goto_parent() {
                did_visit_children = true;
            } else {
                break;
            }
        } else {
            if is_named {
                result.push(node);
            }
            if cursor.goto_first_child() {
                did_visit_children = false;
            } else {
                did_visit_children = true;
            }
        }
    }
    return result;
}


impl JsIdent {
    pub fn parse(code: &str) {
        let mut parser = Parser::new();

        let language = unsafe { tree_sitter_javascript() };
        parser.set_language(language).unwrap();

        let tree = parser.parse(code, None).unwrap();
        let nodes_before = get_all_nodes(&tree);

        for node in &nodes_before {
            match node.kind() {
                "import_statement" => {
                    // println!("{:?}", node.child(0));
                    // println!("{:?}", node.child(1));
                    // println!("{:?}", node.child(2));
                    // println!("{:?}", node.child(3));
                    // println!("{:?}", node.child_by_field_name("string"));
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::identify::js_ident::JsIdent;

    #[test]
    fn should_parse_import() {
        let source_code = "import {sayHi} from './say.js'";
        JsIdent::parse(source_code);
    }
}