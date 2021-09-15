use tree_sitter::{Node, Parser, Query, QueryCursor};

use guarding_core::domain::code_file::CodeFile;
use guarding_core::domain::code_class::CodeClass;
use crate::code_ident::CodeIdent;

const JS_QUERY: &'static str = "
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

pub struct JsIdent {
    parser: Parser,
    query: Query,
}


impl JsIdent {
    fn new() -> JsIdent {
        let mut parser = Parser::new();

        let language = tree_sitter_javascript::language();
        parser.set_language(language).unwrap();

        let query = Query::new(language, &JS_QUERY)
            .map_err(|e| println!("{}", format!("Query compilation failed: {:?}", e))).unwrap();
        JsIdent { parser, query }
    }
}

impl JsIdent {
    fn do_parse(code: &str, ident: &mut JsIdent) -> CodeFile {
        let text_callback = |n: Node| &code[n.byte_range()];
        let tree = ident.parser.parse(code, None).unwrap();

        let mut query_cursor = QueryCursor::new();
        let captures = query_cursor.captures(&ident.query, tree.root_node(), text_callback);

        let mut code_file = CodeFile::default();
        let mut last_class_end_line = 0;
        let mut class = CodeClass::default();

        for (mat, capture_index) in captures {
            let capture = mat.captures[capture_index];
            let capture_name = &ident.query.capture_names()[capture.index as usize];

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
                    class.functions.push(JsIdent::create_function(capture, text));
                }
                "function-name" => {
                    code_file.functions.push(JsIdent::create_function(capture, text));
                }
                "import-name" => {}
                "parameter" => {}
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
}

impl CodeIdent for JsIdent {
    fn parse(code: &str) -> CodeFile {
        let mut ident = JsIdent::new();
        JsIdent::do_parse(code, &mut ident)
    }
}

#[cfg(test)]
mod tests {
    use crate::code_ident::CodeIdent;
    use crate::js_ident::JsIdent;

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

        let funcs = &file.functions[0];
        assert_eq!("abc", funcs.name);

        assert_eq!(0, funcs.start.row);
        assert_eq!(0, funcs.start.column);
        assert_eq!(2, funcs.end.row);
        assert_eq!(1, funcs.end.column);
    }
}
