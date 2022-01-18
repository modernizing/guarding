use tree_sitter::{Node, Parser, Query, QueryCursor};
use guarding_core::domain::code_class::CodeClass;
use guarding_core::domain::code_file::CodeFile;
use guarding_core::domain::code_function::CodeFunction;
use crate::code_ident::CodeIdent;

const C_SHARP_QUERY: &'static str = "
(using_directive
	(qualified_name) @import-name)

(class_declaration
    (attribute_list (attribute name: (identifier) @annotation.name))?
    name: (identifier) @class-name
    bases: (base_list ((identifier) @impl-name))?
    body: (declaration_list
    	(property_declaration
        	type: (identifier) @prop-type
            name: (identifier) @prop-name
        )?
        (method_declaration
        	name: (identifier) @method-name
            parameters: (parameter_list (parameter
            	type: (identifier) @param-type
                name: (identifier) @param-name
            ))?
        )?
    ) @body
)";


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
        let mut class = CodeClass::default();

        let capture_names = ident.query.capture_names();

        let mut iters = captures.into_iter();
        while let Some((mat, capture_index)) = iters.next() {
            let capture = mat.captures[capture_index];
            let capture_name: &str = &capture_names[capture.index as usize];

            let text = capture.node.utf8_text((&code).as_ref()).unwrap_or("");
            match capture_name {
                "import-name" => {
                    code_file.imports.push(text.to_string());
                }
                "class-name" => {
                    class.name = text.to_string();
                    class.package = code_file.package.clone();

                    let class_node = capture.node.parent().unwrap();
                    CSharpIdent::insert_location(&mut class, class_node);
                }
                "impl-name" => {
                    class.implements.push(text.to_string());
                }
                "method-name" => {
                    let mut function = CodeFunction::default();
                    function.name = text.to_string();
                    class.functions.push(function);

                    if !class.name.is_empty() {
                        code_file.classes.push(class.clone());
                        class = CodeClass::default();
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

        if class.name != "" {
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

    #[test]
    fn should_parse_class_impl_name() {
        let source_code = "public class SharpingClassVisitor: CSharpSyntaxWalker, DemoInterface { }";

        let file = CSharpIdent::parse(source_code);
        assert_eq!(1, file.classes.len());

        assert_eq!(2, file.classes[0].implements.len());
        assert_eq!("CSharpSyntaxWalker", file.classes[0].implements[0]);
        assert_eq!("DemoInterface", file.classes[0].implements[1]);
    }

    #[test]
    fn should_parse_class_functions() {
        let source_code = "[ApiController]
public class SharpingClassVisitor {
  public Domain domain { get; set; }
  public void VisitClassDeclaration(ClassDeclarationSyntax node){

  }
}";

        let file = CSharpIdent::parse(source_code);
        assert_eq!(1, file.classes.len());

        assert_eq!(1, file.classes[0].functions.len());
        assert_eq!("VisitClassDeclaration", file.classes[0].functions[0].name);
    }
}