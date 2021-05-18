extern crate pest;
#[macro_use]
extern crate pest_derive;

extern crate serde;

use tree_sitter::Language;
use crate::parser::ast::{GuardRule, RuleScope, RuleLevel, Expr, RuleAssert, Operator};
use crate::identify::code_model::CodeFile;
use std::collections::HashMap;
use std::path::PathBuf;
use walkdir::WalkDir;
use std::fs;
use crate::identify::java_ident::JavaIdent;

extern "C" { fn tree_sitter_rust() -> Language; }
extern "C" { fn tree_sitter_java() -> Language; }
extern "C" { fn tree_sitter_javascript() -> Language; }

pub mod identify;
pub mod parser;

fn main() {

}

fn execute(content: String, code_dir: PathBuf) -> HashMap<usize, String> {
    let rules = parser::parse(content.as_str());
    let mut models = vec![];
    for entry in WalkDir::new(code_dir) {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() {
            continue;
        }

        if let None = entry.path().extension() {
            continue;
        }

        let ext = entry.path().extension().unwrap().to_str().unwrap();
        match ext {
            ".java" => {
                let content = fs::read_to_string(entry.path()).expect("not such file");
                models.push(JavaIdent::parse(content.as_str()));
            }
            &_ => {}
        }
    }

    let mut errors: HashMap<usize, String> = Default::default();
    rules.into_iter().enumerate().for_each(|(i, rule)| {
        capture(rule, &models, i, &mut errors);
    });
    errors
}

pub fn capture(rule: GuardRule, models: &Vec<CodeFile>, index: usize, errors: &mut HashMap<usize, String>) {
    let mut filtered_models: Vec<CodeFile> = vec![];

    // 1. filter by scopes
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

    // 2. run expression for evaluation
    match &rule.expr {
        Expr::Call(_) => {}
        Expr::PropsCall(props) => {
            match props[0].as_str() {
                "file" => {
                    match props[1].as_str() {
                        "len" => {
                            let size = get_assert_sized(&rule);
                            match &rule.ops[0] {
                                Operator::Gt => {
                                    if size > filtered_models.len() {
                                        let msg = format!("file.len = {}, expected len: > {}", filtered_models.len(), size);
                                        errors.insert(index, msg);
                                    }
                                }
                                Operator::Gte => {}
                                Operator::Lt => {
                                    if size < filtered_models.len() {
                                        let msg = format!("file.len = {}, expected: < len {}", filtered_models.len(), size);
                                        errors.insert(index, msg);
                                    }
                                }
                                Operator::Lte => {}
                                Operator::Eq => {}
                                Operator::NotEq => {}
                                Operator::And => {}
                                Operator::Or => {}
                                Operator::Not => {}
                                Operator::StartsWith => {}
                                Operator::Endswith => {}
                                Operator::Contains => {}
                                Operator::ResideIn => {}
                                Operator::Accessed => {}
                                Operator::DependBy => {}
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

    // todo: 3. run assert
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
    use crate::{execute};

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
        let code_dir = test_dir();

        let errors = execute(content, code_dir);

        assert_eq!(1, errors.len());
    }
}