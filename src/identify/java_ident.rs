use tree_sitter::{Node, Parser, Query, QueryCursor};

use crate::tree_sitter_java;
use crate::identify::code_model::{CodeClass, CodeFile};
use crate::identify::code_ident::CodeIdent;

pub struct JavaIdent {}

impl CodeIdent for JavaIdent {
    fn parse(code: &str) -> CodeFile {
        let query_source = "
(package_declaration
	(scoped_identifier) @package-name)

(import_declaration
	(scoped_identifier) @import-name)

(class_declaration
	name: (identifier) @class-name
    interfaces: (super_interfaces (interface_type_list (type_identifier)  @impl-name))?
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
                "package-name" => {
                    code_file.package = text.to_string();
                }
                "import-name" => {
                    code_file.imports.push(text.to_string());
                }
                "class-name" => {
                    class.name = text.to_string();
                    let class_node = capture.node.parent().unwrap();
                    last_class_end_line = class_node.end_position().row;
                    JavaIdent::insert_location(&mut class, class_node);
                    if !is_last_node {
                        is_last_node = true;
                    }
                }
                "impl-name" => {
                    class.implements.push(text.to_string());
                }
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

        if is_last_node {
            code_file.classes.push(class.clone());
        }

        code_file
    }
}

#[cfg(test)]
mod tests {
    use crate::identify::java_ident::JavaIdent;
    use crate::identify::code_ident::CodeIdent;

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
    fn should_parse_impl_java_class() {
        let source_code = "class DateTimeImpl implements DateTime {
    @Override
    public Date getDate() {
        return new Date();
    }
}";
        let file = JavaIdent::parse(source_code);
        assert_eq!(1, file.classes.len());
        assert_eq!("DateTimeImpl", file.classes[0].name);
        assert_eq!(1, file.classes[0].implements.len());
        assert_eq!("DateTime", file.classes[0].implements[0]);
    }

    #[test]
    fn should_parse_normal_java_class() {
        let source_code = "class DateTimeImpl {
    public Date getDate() {
        return new Date();
    }
}";
        let file = JavaIdent::parse(source_code);
        assert_eq!(1, file.classes.len());
        assert_eq!("DateTimeImpl", file.classes[0].name);
    }

    #[test]
    fn should_support_package_name() {
        let source_code = "package com.phodal.pepper.powermock;
";
        let file = JavaIdent::parse(source_code);
        assert_eq!("com.phodal.pepper.powermock", file.package);
    }
}