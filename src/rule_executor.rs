use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::identify::code_model::CodeFile;
use crate::identify::java_ident::JavaIdent;
use crate::identify::js_ident::JsIdent;
use crate::identify::rust_ident::RustIdent;
use crate::parser::ast::{Expr, GuardRule, Operator, RuleAssert, RuleLevel, RuleScope};
use crate::parser;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleError {
    pub expected: String,
    pub actual: String,
    pub error_type: String,
    pub msg: String,
    pub rule: usize,
}

#[derive(Debug, Clone)]
pub struct RuleExecutor {
    pub errors: Vec<RuleError>,
    pub rules: Vec<GuardRule>,
    pub models: Vec<CodeFile>,
}

impl Default for RuleExecutor {
    fn default() -> Self {
        RuleExecutor {
            errors: Default::default(),
            rules: vec![],
            models: vec![],
        }
    }
}

impl RuleExecutor {
    pub fn execute(rule_content: String, code_dir: PathBuf) -> Vec<RuleError> {
        let rules = parser::parse(rule_content.as_str());
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
            let content = fs::read_to_string(entry.path()).expect("not such file");

            match ext {
                "java" => {
                    models.push(JavaIdent::parse(content.as_str()));
                }
                "js" => {
                    models.push(JsIdent::parse(content.as_str()));
                }
                "rs" => {
                    models.push(RustIdent::parse(content.as_str()));
                }
                &_ => {}
            }
        }

        let mut executor = RuleExecutor::default();
        executor.models = models;
        executor.rules = rules;

        executor.run();
        executor.errors
    }

    pub fn run(&mut self) {
        self.rules
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(i, rule)| {
                self.capture(rule, i);
            });
    }

    pub fn capture(&mut self, rule: GuardRule, index: usize) {
        let mut filtered_models: Vec<CodeFile> = vec![];

        // 1. filter by scopes
        match &rule.level {
            RuleLevel::Package => {
                match &rule.scope {
                    RuleScope::All => {}
                    RuleScope::PathDefine(str) => {
                        if str.as_str() == "." {
                            filtered_models = self.models.clone();
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
                                let size = RuleExecutor::get_assert_sized(&rule);
                                let mut error = RuleError {
                                    expected: format!("{}", size),
                                    actual: format!("{}", filtered_models.len()),
                                    error_type: "file.len size".to_string(),
                                    msg: "".to_string(),
                                    rule: index
                                };

                                match &rule.ops[0] {
                                    Operator::Gt => {
                                        if size > filtered_models.len() {
                                            error.msg = format!("file.len = {}, expected: len > {}", filtered_models.len(), size);
                                        }
                                    }
                                    Operator::Gte => {
                                        if size >= filtered_models.len() {
                                            error.msg = format!("file.len = {}, expected: len >= {}", filtered_models.len(), size);
                                        }
                                    }
                                    Operator::Lt => {
                                        if size < filtered_models.len() {
                                            error.msg = format!("file.len = {}, expected: len < {}", filtered_models.len(), size);
                                        }
                                    }
                                    Operator::Lte => {
                                        if size <= filtered_models.len() {
                                            error.msg = format!("file.len = {}, expected: len <=  {}", filtered_models.len(), size);
                                        }
                                    }
                                    Operator::Eq => {
                                        if size != filtered_models.len() {
                                            error.msg = format!("file.len = {}, expected: len = {}", filtered_models.len(), size);
                                        }
                                    }
                                    _ => {}
                                }

                                if !error.msg.is_empty() {
                                    self.errors.push(error);
                                }
                            }
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
}