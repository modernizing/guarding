use tree_sitter::{Node, QueryCapture};

use crate::domain::code_function::CodeFunction;
use crate::domain::code_file::CodeFile;
use crate::domain::Location;
use crate::identify::rust_ident::RustIdent;

pub trait CodeIdent {
    fn parse(code: &str) -> CodeFile;

    fn insert_location<T: Location>(model: &mut T, node: Node) {
        model.set_start(node.start_position().row, node.start_position().column);
        model.set_end(node.end_position().row, node.end_position().column);
    }

    fn create_function(capture: QueryCapture, text: &str) -> CodeFunction {
        let mut function = CodeFunction::default();
        function.name = text.to_string();

        let node = capture.node.parent().unwrap();
        RustIdent::insert_location(&mut function, node);
        function
    }
}