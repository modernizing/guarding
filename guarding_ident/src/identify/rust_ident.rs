use std::collections::HashMap;

use tree_sitter::{Node, Parser, Query, QueryCursor};

use guarding_core::domain::code_file::CodeFile;
use guarding_core::domain::code_function::CodeFunction;
use guarding_core::domain::code_class::CodeClass;
use crate::code_ident::CodeIdent;

const RUST_QUERY: &'static str = "
(use_declaration
	(scoped_identifier) @import-name)

(struct_item
	name: (type_identifier) @struct-name
    body: (field_declaration_list
    	(field_declaration
			name: (field_identifier) @field-name
            type: (type_identifier) @type-name
    ))?
)

(impl_item
    trait: (type_identifier)? @trait-name
	type: (type_identifier) @impl-struct-name
    body: (declaration_list (
	    (function_item
        	name: (identifier) @impl-function-name
        ))
    )
)
";


pub struct RustIdent {
    parser: Parser,
    query: Query
}

impl RustIdent {
    fn new() -> RustIdent {
        let mut parser = Parser::new();

        let language = tree_sitter_rust::language();
        parser.set_language(language).unwrap();

        let query = Query::new(language, &RUST_QUERY)
            .map_err(|e| println!("{}", format!("Query compilation failed: {:?}", e))).unwrap();
        RustIdent { parser, query }
    }
}

impl RustIdent {
    fn do_parse(code: &str, ident: &mut RustIdent) -> CodeFile {
        let text_callback = |n: Node| &code[n.byte_range()];
        let tree = ident.parser.parse(code, None).unwrap();

        let mut query_cursor = QueryCursor::new();
        let captures = query_cursor.captures(&ident.query, tree.root_node(), text_callback);

        let mut code_file = CodeFile::default();
        let mut last_class_end_line = 0;
        let mut class = CodeClass::default();

        let mut last_impl_struct_name = "".to_string();
        let mut last_trait_name = "".to_string();
        let mut impl_functions: HashMap<String, Vec<CodeFunction>> = Default::default();
        let mut trait_struct_map: HashMap<String, String> = Default::default();

        for (mat, capture_index) in captures {
            let capture = mat.captures[capture_index];
            let capture_name = &ident.query.capture_names()[capture.index as usize];

            let text = capture.node.utf8_text((&code).as_ref()).unwrap_or("");
            match capture_name.as_str() {
                "import-name" => {
                    code_file.imports.push(text.to_string());
                },
                "struct-name" => {
                    class.name = text.to_string();
                    let struct_node = capture.node;
                    last_class_end_line = struct_node.end_position().row;
                    RustIdent::insert_location(&mut class, struct_node);
                },
                "impl-struct-name" => {
                    last_impl_struct_name = text.to_string();
                    if last_trait_name != "" {
                        trait_struct_map.insert(text.to_string(), last_trait_name);
                    }
                    last_trait_name = "".to_string();
                }
                "impl-function-name" => {
                    let function = RustIdent::create_function(capture, text);
                    impl_functions
                        .entry(last_impl_struct_name.clone())
                        .or_insert_with(Vec::new)
                        .push(function);
                }
                "trait-name" => {
                    last_trait_name = text.to_string();
                }
                &_ => {

                }
            }

            if capture.node.start_position().row >= last_class_end_line {
                if !class.name.is_empty() {
                    code_file.classes.push(class.clone());
                    class = CodeClass::default();
                }
            }
        }

        for clz in code_file.classes.iter_mut() {
            if let Some(function) = impl_functions.get(clz.name.as_str()) {
                clz.functions = function.clone();
            }

            if let Some(trait_name) = trait_struct_map.get(clz.name.as_str()) {
                clz.implements.push(trait_name.to_string());
            }
        }

        code_file
    }
}

impl CodeIdent for RustIdent {
    fn parse(code: &str) -> CodeFile {
        let mut ident = RustIdent::new();
        RustIdent::do_parse(code, &mut ident)
    }
}

#[cfg(test)]
mod tests {
    use crate::code_ident::CodeIdent;
    use crate::rust_ident::RustIdent;

    #[test]
    fn should_parse_import() {
        let source_code = "use crate::identify::rust_ident::RustIdent;
";
        let file = RustIdent::parse(source_code);
        assert_eq!(1, file.imports.len());
    }

    #[test]
    fn should_parse_basic_struct() {
        let source_code = "pub struct RustIdent {
}
";
        let file = RustIdent::parse(source_code);
        assert_eq!(1, file.classes.len());
    }

    #[test]
    fn should_parse_struct() {
        let source_code = "pub struct RustIdent {}

impl RustIdent {
    pub fn parse(code: &str) -> CodeFile {
        CodeFile::default()
    }
}
";
        let file = RustIdent::parse(source_code);

        assert_eq!(1, file.classes.len());
        assert_eq!("RustIdent", file.classes[0].name);
        let functions = &file.classes[0].functions;
        assert_eq!(1, functions.len());
        assert_eq!("parse", functions[0].name);
    }

    #[test]
    fn should_parse_for_trait_impl() {
        let source_code = "pub struct RustIdent {}

impl Default for RustIdent {
    fn default() -> Self {
        RustIdent {}
    }
}
";
        let file = RustIdent::parse(source_code);

        assert_eq!("Default", file.classes[0].implements[0]);
    }
}
