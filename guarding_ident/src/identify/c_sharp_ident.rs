use tree_sitter::{Parser, Query};

const C_SHARP_QUERY: &'static str = "
(package_declaration
	(scoped_identifier) @package-name)

(import_declaration
	(scoped_identifier) @import-name)

(method_declaration
	(modifiers
    	(annotation
    		name: (identifier) @annotation.name
            arguments: (annotation_argument_list)? @annotation.key_values
    	)
    )
)

(program
    (class_declaration
	    name: (identifier) @class-name
        interfaces: (super_interfaces (interface_type_list (type_identifier)  @impl-name))?
    )
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
}


#[cfg(test)]
mod tests {
    use crate::identify::c_sharp_ident::CSharpIdent;

    #[test]
    fn should_parse_import() {
        let source_code = "using Microsoft.CodeAnalysis;";

        let file = CSharpIdent::parse(source_code);
        assert_eq!(1, file.imports.len());
    }
}