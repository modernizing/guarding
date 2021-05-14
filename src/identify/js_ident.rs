use crate::{tree_sitter_javascript};
use tree_sitter::{Parser, Tree, Node, Query, QueryCursor};

pub struct JsIdent {

}

impl JsIdent {
    pub fn parse(code: &str) {
        let query_source = "
(import_specifier
	name: (identifier) @import-name)
(namespace_import (identifier) @import-name)
(import_statement
	source: (string) @source)
(import_clause (identifier) @import-name)

(class_declaration
  name: (identifier) @class-name
  body: (class_body
    (method_definition
      name: (property_identifier) @class-method-name)))

(function_declaration
      name: * @function-name)
";
        let mut parser = Parser::new();

        let language = unsafe { tree_sitter_javascript() };
        parser.set_language(language).unwrap();
        let text_callback = |n: Node| &code[n.byte_range()];

        let tree = parser.parse(code, None).unwrap();

        let query = Query::new(language, &query_source)
            .map_err(|e| println!("{}", format!("Query compilation failed: {:?}", e))).unwrap();

        let mut query_cursor = QueryCursor::new();
        for (mat, capture_index) in
        query_cursor.captures(&query, tree.root_node(), text_callback)
        {
            let capture = mat.captures[capture_index];
            let capture_name = &query.capture_names()[capture.index as usize];
            println!(
                "    pattern: {}, capture: {}, row: {}, text: {:?}",
                mat.pattern_index,
                capture_name,
                capture.node.start_position().row,
                capture.node.utf8_text((&code).as_ref()).unwrap_or("")
            );
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::identify::js_ident::JsIdent;

    #[test]
    fn should_parse_import() {
        let source_code = "import {sayHi} from './say.js'\

class Rectangle {
  constructor(height, width) {
    this.height = height;
    this.width = width;
  }
}

function abc() {

}

";
        JsIdent::parse(source_code);
    }
}