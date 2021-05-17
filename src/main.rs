extern crate pest;
#[macro_use]
extern crate pest_derive;

extern crate serde;

use tree_sitter::Language;
use crate::parser::ast::{GuardRule, RuleScope, RuleLevel, Expr, RuleAssert};
use crate::identify::code_model::CodeFile;

extern "C" { fn tree_sitter_rust() -> Language; }
extern "C" { fn tree_sitter_java() -> Language; }
extern "C" { fn tree_sitter_javascript() -> Language; }

pub mod identify;
pub mod parser;

fn main() {

}

pub fn capture(rule: GuardRule, models: &Vec<CodeFile>) {
    let mut filtered_models: Vec<CodeFile> = vec![];

    // filter by scopes
    match &rule.level {
        RuleLevel::Package => {
            match &rule.scope {
                RuleScope::All => {}
                RuleScope::PathDefine(str) => {
                    if str.as_str() == "." {
                        filtered_models = models.clone();
                    };
                }
                RuleScope::Extend(_) => {}
                RuleScope::Assignable(_) => {}
                RuleScope::Implementation(_) => {}
                RuleScope::MatchRegex(_) => {}
            }
        }
        RuleLevel::Module => {}
        RuleLevel::Function => {}
        RuleLevel::Class => {}
        RuleLevel::Struct => {}
        RuleLevel::File => {}
    };

    match &rule.expr {
        Expr::Call(_) => {}
        Expr::PropsCall(props) => {
            match props[0].as_str() {
                "file" => {
                    match props[1].as_str() {
                        "len" => {
                            let size = get_assert_sized(&rule);
                            if filtered_models.len() > size {
                                println!("error for rule: {:?}", &rule);
                            }
                        },
                        &_ => {}
                    };
                }
                &_ => {}
            }
        }
        Expr::Identifier(_) => {}
    }
}

fn get_assert_sized(rule: &GuardRule) -> usize {
    let mut size = 0;
    match &rule.assert {
        RuleAssert::Empty => {}
        RuleAssert::Stringed(_) => {}
        RuleAssert::Leveled(_, _) => {}
        RuleAssert::Sized(sized) => {
            size = *sized;
        }
    }
    size
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::fs;
    use crate::{parser, capture};
    use walkdir::WalkDir;
    use crate::identify::java_ident::JavaIdent;

    fn test_dir() -> PathBuf {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let test_dir = root_dir.join("_fixtures")
            .join("java");

        test_dir
    }

    #[test]
    fn should_working_in_process() {
        let path = test_dir().join("size.guarding");
        let content = fs::read_to_string(path).expect("not file");
        let vec = parser::parse(content.as_str());


        let mut models = vec![];
        for entry in WalkDir::new(test_dir()) {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                let file_name = entry.file_name().to_str().expect("error file name");
                if file_name.ends_with(".java") {
                    let content = fs::read_to_string(entry.path()).expect("not such file");
                    models.push(JavaIdent::parse(content.as_str()));
                }
            }
        }

        for rule in vec {
            capture(rule, &models);
        }
    }
}