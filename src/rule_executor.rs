use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::identify::code_ident::CodeIdent;
use crate::identify::code_model::{CodeClass, CodeFile};
use crate::identify::java_ident::JavaIdent;
use crate::identify::js_ident::JsIdent;
use crate::identify::rust_ident::RustIdent;
use crate::package_matcher::is_package_match;
use crate::parser;
use crate::parser::ast::{Expr, GuardRule, Operator, RuleAssert, RuleLevel, RuleScope};

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
        match &rule.level {
            RuleLevel::Package => {
                self.capture_package(&rule, index)
            }
            RuleLevel::Function => {
                println!("todo");
            }
            RuleLevel::Class => {
                self.capture_class(&rule, index)
            }
            RuleLevel::Struct => {
                println!("todo");
            }
        };

        // todo: 3. run assert
    }

    fn capture_class(&mut self, rule: &GuardRule, index: usize) {
        let mut filtered_models: Vec<CodeClass> = vec![];

        match &rule.scope {
            RuleScope::PathDefine(str) => {
                if str.as_str() == "." {
                    for file in &self.models {
                        filtered_models.extend(file.classes.clone());
                    }
                } else {
                    for file in &self.filter_by_package_identifier(str) {
                        filtered_models.extend(file.classes.clone());
                    }
                }
            }
            RuleScope::Implementation(str) => {
                &self.models.iter().for_each(|file| {
                    let classes: Vec<CodeClass> = file.classes.iter().filter(|class| {
                        class.implements.contains(str)
                    }).map(|s| s.clone())
                        .collect();
                    filtered_models.extend(classes);
                });
            }
            _ => {}
        }

        match &rule.expr {
            Expr::PropsCall(props) => {
                match props[0].as_str() {
                    "len" => {
                        let size = RuleExecutor::get_assert_sized(&rule);
                        let ops = &rule.ops[0];
                        self.processing_file_len(index, size, ops, filtered_models.len())
                    }
                    _ => {}
                }
            }
            Expr::Identifier(_) => {}
        }
    }

    fn capture_package(&mut self, rule: &GuardRule, index: usize) {
        let mut filtered_models: Vec<CodeFile> = vec![];

        match &rule.scope {
            RuleScope::PathDefine(str) => {
                let path = str.as_str();
                if path == "." {
                    filtered_models = self.models.clone();
                } else {
                    filtered_models = self.filter_by_package_identifier(str);
                };
            }
            RuleScope::MatchRegex(_) => {}
            _ => {}
        }

        // 2. run expression for evaluation
        match &rule.expr {
            Expr::PropsCall(props) => {
                match props[0].as_str() {
                    "len" => {
                        let size = RuleExecutor::get_assert_sized(&rule);
                        let ops = &rule.ops[0];
                        self.processing_file_len(index, size, ops, filtered_models.len())
                    }
                    "file" => {
                        match props[1].as_str() {
                            "len" => {
                                let size = RuleExecutor::get_assert_sized(&rule);
                                let ops = &rule.ops[0];
                                self.processing_file_len(index, size, ops, filtered_models.len())
                            }
                            &_ => {}
                        };
                    }
                    &_ => {}
                }
            }
            Expr::Identifier(_) => {}
        }
    }

    fn filter_by_package_identifier(&mut self, str: &String) -> Vec<CodeFile> {
        self.models.iter()
            .filter(|s| { is_package_match(str.to_string(), s.package.as_str()) })
            .map(|s| { s.clone() })
            .collect()
    }

    fn processing_file_len(&mut self, index: usize, excepted_size: usize, ops: &Operator, actual_len: usize) {
        let mut error = RuleError {
            expected: format!("{}", excepted_size),
            actual: format!("{}", actual_len),
            error_type: "file.len size".to_string(),
            msg: "".to_string(),
            rule: index,
        };

        match ops {
            Operator::Gt => {
                if excepted_size > actual_len {
                    error.msg = format!("file.len = {}, expected: len > {}", actual_len, excepted_size);
                }
            }
            Operator::Gte => {
                if excepted_size >= actual_len {
                    error.msg = format!("file.len = {}, expected: len >= {}", actual_len, excepted_size);
                }
            }
            Operator::Lt => {
                if excepted_size < actual_len {
                    error.msg = format!("file.len = {}, expected: len < {}", actual_len, excepted_size);
                }
            }
            Operator::Lte => {
                if excepted_size <= actual_len {
                    error.msg = format!("file.len = {}, expected: len <=  {}", actual_len, excepted_size);
                }
            }
            Operator::Eq => {
                if excepted_size != actual_len {
                    error.msg = format!("file.len = {}, expected: len = {}", actual_len, excepted_size);
                }
            }
            _ => {}
        }

        if !error.msg.is_empty() {
            self.errors.push(error);
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
}