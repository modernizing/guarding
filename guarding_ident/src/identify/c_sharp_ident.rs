use tree_sitter::{Node, Parser, Query, QueryCursor};
use guarding_core::domain::code_class::CodeClass;
use guarding_core::domain::code_file::CodeFile;
use crate::code_ident::CodeIdent;

const C_SHARP_QUERY: &'static str = "
(using_directive
	(qualified_name) @import-name)

(class_declaration
    name: (identifier) @class-name
)
";


pub struct CSharpIdent {
    parser: Parser,
    query: Query
}

impl CSharpIdent {
    pub fn new() -> CSharpIdent {
        let mut parser = Parser::new();
        let language = tree_sitter_c_sharp::language();
        parser.set_language(language).unwrap();

        let query = Query::new(language, &C_SHARP_QUERY)
            .map_err(|e| println!("{}", format!("Query compilation failed: {:?}", e))).unwrap();

        CSharpIdent {
            parser,
            query
        }
    }

    fn do_parse(code: &&str, ident: &mut CSharpIdent) -> CodeFile {
        let tree = ident.parser.parse(code, None).unwrap();
        let text_callback = |n: Node| &code[n.byte_range()];
        let mut query_cursor = QueryCursor::new();
        let captures = query_cursor.captures(&ident.query, tree.root_node(), text_callback);

        let mut code_file = CodeFile::default();
        #[allow(unused_mut)]
        let mut class = CodeClass::default();
        #[allow(unused_mut)]
        let mut is_last_node = false;

        let capture_names = ident.query.capture_names();

        for (mat, capture_index) in captures {
            let capture = mat.captures[capture_index];
            let capture_name = &capture_names[capture.index as usize];

            let text = capture.node.utf8_text((&code).as_ref()).unwrap_or("");
            match capture_name.as_str() {
                "import-name" => {
                    code_file.imports.push(text.to_string());
                }
                "class-name" => {
                    if !class.name.is_empty() {
                        code_file.classes.push(class.clone());
                        class = CodeClass::default();
                    }

                    class.name = text.to_string();
                    class.package = code_file.package.clone();

                    let class_node = capture.node.parent().unwrap();
                    CSharpIdent::insert_location(&mut class, class_node);
                    if !is_last_node {
                        is_last_node = true;
                    }
                }

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
        }

        if is_last_node {
            code_file.classes.push(class.clone());
        }

        code_file
    }
}

impl CodeIdent for CSharpIdent {
    fn parse(code: &str) -> CodeFile {
        let mut ident = CSharpIdent::new();
        CSharpIdent::do_parse(&code, &mut ident)
    }
}

#[cfg(test)]
mod tests {
    use crate::code_ident::CodeIdent;
    use crate::identify::c_sharp_ident::CSharpIdent;

    #[test]
    fn should_parse_import() {
        let source_code = "using Microsoft.CodeAnalysis;";

        let file = CSharpIdent::parse(source_code);
        assert_eq!(1, file.imports.len());
    }

    #[test]
    fn should_parse_class_name() {
        let source_code = "public class SharpingClassVisitor { }";

        let file = CSharpIdent::parse(source_code);
        assert_eq!(1, file.classes.len());
        assert_eq!("SharpingClassVisitor", file.classes[0].name);
    }
}