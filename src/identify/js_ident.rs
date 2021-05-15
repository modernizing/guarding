use tree_sitter::{Node, Parser, Query, QueryCursor, QueryCapture};

use crate::identify::code_model::{CodeClass, CodeFile, CodeFunction};
use crate::identify::code_model::Location;
use crate::tree_sitter_javascript;

pub struct JsIdent {

}

impl JsIdent {
    pub fn parse(code: &str) -> CodeFile {
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
      name: (property_identifier) @class-method-name
      parameters: (formal_parameters (identifier)? @parameter)
    )
  )
)

(program (function_declaration
      name: * @function-name))
";
        let mut parser = Parser::new();

        let language = unsafe { tree_sitter_javascript() };
        parser.set_language(language).unwrap();
        let text_callback = |n: Node| &code[n.byte_range()];

        let tree = parser.parse(code, None).unwrap();

        let query = Query::new(language, &query_source)
            .map_err(|e| println!("{}", format!("Query compilation failed: {:?}", e))).unwrap();

        let mut query_cursor = QueryCursor::new();
        let captures = query_cursor.captures(&query, tree.root_node(), text_callback);

        let mut code_file = CodeFile::default();
        let mut last_class_end_line = 0;
        let mut class = CodeClass::default();

        for (mat, capture_index) in captures {
            let capture = mat.captures[capture_index];
            let capture_name = &query.capture_names()[capture.index as usize];

            let text = capture.node.utf8_text((&code).as_ref()).unwrap_or("");
            match capture_name.as_str() {
                "source" => {
                    code_file.imports.push(text.to_string());
                }
                "class-name" => {
                    class.name = text.to_string();
                    let class_node = capture.node.parent().unwrap();
                    last_class_end_line = class_node.end_position().row;
                    JsIdent::insert_location(&mut class, class_node);
                }
                "class-method-name" => {
                    let function = JsIdent::create_function(capture, text);

                    class.functions.push(function);
                }
                "function-name" => {
                    let function = JsIdent::create_function(capture, text);

                    code_file.functions.push(function);
                }
                "import-name" => {},
                "parameter" => {},
                &_ => {
                    println!(
                        "    pattern: {}, capture: {}, row: {}, text: {:?}",
                        mat.pattern_index,
                        capture_name,
                        capture.node.start_position().row,
                        capture.node.utf8_text((&code).as_ref()).unwrap_or("")
                    );
                }
            }

            if capture.node.start_position().row >= last_class_end_line {
                if !class.name.is_empty() {
                    code_file.classes.push(class.clone());
                    class = CodeClass::default();
                }
            }
        }

        code_file
    }

    fn create_function(capture: QueryCapture, text: &str) -> CodeFunction {
        let mut function = CodeFunction::default();
        function.name = text.to_string();

        let node = capture.node.parent().unwrap();
        JsIdent::insert_location(&mut function, node);
        function
    }

    fn insert_location<T: Location>(model: &mut T, node: Node) {
        model.set_start(node.start_position().row, node.start_position().column);
        model.set_end(node.end_position().row, node.end_position().column);
    }
}

#[cfg(test)]
mod tests {
    use crate::identify::js_ident::JsIdent;

    #[test]
    fn should_parse_import() {
        let source_code = "import {sayHi} from './say.js'

class Rectangle {
  constructor(height, width) {
    this.height = height;
    this.width = width;
  }
}

function abc() {

}
";
        let file = JsIdent::parse(source_code);
        println!("{:?}", file);

        let funcs = &file.functions[0];

        let class = &file.classes[0];
        assert_eq!("Rectangle", class.name);
        assert_eq!(0, class.start.column);
        assert_eq!(2, class.start.row);
        assert_eq!(7, class.end.row);
        assert_eq!(1, class.end.column);
        assert_eq!("constructor", class.functions[0].name);

        assert_eq!("abc", funcs.name);
    }

    #[test]
    fn should_parse_func_location() {
        let source_code = "function abc() {

}
";
        let file = JsIdent::parse(source_code);
        println!("{:?}", file);

        let funcs = &file.functions[0];
        assert_eq!("abc", funcs.name);

        assert_eq!(0, funcs.start.row);
        assert_eq!(0, funcs.start.column);
        assert_eq!(2, funcs.end.row);
        assert_eq!(1, funcs.end.column);
    }
}