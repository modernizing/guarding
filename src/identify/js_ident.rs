use crate::{get_all_nodes, tree_sitter_javascript};
use tree_sitter::Parser;

pub struct JsIdent {

}

impl JsIdent {
    pub fn parse(code: &str) {
        let mut parser = Parser::new();

        let language = unsafe { tree_sitter_javascript() };
        parser.set_language(language).unwrap();

        let tree = parser.parse(code, None).unwrap();
        let nodes_before = get_all_nodes(&tree);
        let mut cursor = tree.walk();

        for node in &nodes_before {
            match node.kind() {
                "import_statement" => {
                    println!("{:?}", node.child(0));
                    println!("{:?}", node.child(1));
                    println!("{:?}", node.child(2));
                    println!("{:?}", node.child(3));
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