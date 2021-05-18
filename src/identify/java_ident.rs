use tree_sitter::{Node, Parser, Query, QueryCursor, QueryCapture};

use crate::identify::code_model::{CodeClass, CodeFile, CodeFunction};
use crate::identify::code_model::Location;
use crate::{tree_sitter_java};

pub struct JavaIdent {

}

impl JavaIdent {
    pub fn parse(code: &str) -> CodeFile {
        let query_source = "
(import_declaration
	(scoped_identifier) @import-name)

(class_declaration
	name: (identifier) @class-name
    interfaces: (super_interfaces (interface_type_list (type_identifier)  @impl-name))
)

";
        let mut parser = Parser::new();

        let language = unsafe { tree_sitter_java() };
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
        let mut is_last_node = false;

        for (mat, capture_index) in captures {
            let capture = mat.captures[capture_index];
            let capture_name = &query.capture_names()[capture.index as usize];

            let text = capture.node.utf8_text((&code).as_ref()).unwrap_or("");
            match capture_name.as_str() {
                "import-name" => {
                    code_file.imports.push(text.to_string());
                },
                "class-name" => {
                    class.name = text.to_string();
                    let class_node = capture.node.parent().unwrap();
                    println!("{:?}", class_node);
                    last_class_end_line = class_node.end_position().row;
                    JavaIdent::insert_location(&mut class, class_node);
                    if !is_last_node {
                        is_last_node = true;
                    }
                },
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

        if is_last_node {
            code_file.classes.push(class.clone());
        }

        code_file
    }

    #[allow(dead_code)]
    fn create_function(capture: QueryCapture, text: &str) -> CodeFunction {
        let mut function = CodeFunction::default();
        function.name = text.to_string();

        let node = capture.node.parent().unwrap();
        JavaIdent::insert_location(&mut function, node);
        function
    }

    #[allow(dead_code)]
    fn insert_location<T: Location>(model: &mut T, node: Node) {
        model.set_start(node.start_position().row, node.start_position().column);
        model.set_end(node.end_position().row, node.end_position().column);
    }
}

#[cfg(test)]
mod tests {
    use crate::identify::java_ident::JavaIdent;

    #[test]
    fn should_parse_import() {
        let source_code = "import java.lang.System;
import java.io.InputStream;
import payroll.Employee;
";
        let file = JavaIdent::parse(source_code);
        assert_eq!(3, file.imports.len());
    }

    #[test]
    fn should_parse_java_class() {
        let source_code = "class DateTimeImpl implements DateTime {
    @Override
    public Date getDate() {
        return new Date();
    }
}";
        let file = JavaIdent::parse(source_code);
        assert_eq!(1, file.classes.len());
    }
}