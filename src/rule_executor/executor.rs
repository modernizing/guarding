use std::fs;
use std::path::PathBuf;

use walkdir::WalkDir;

use crate::domain::code_class::CodeClass;
use crate::domain::code_file::CodeFile;
use crate::identify::code_ident::CodeIdent;
use crate::identify::java_ident::JavaIdent;
use crate::identify::js_ident::JsIdent;
use crate::identify::rust_ident::RustIdent;
use crate::parser;
use crate::parser::ast::{Expr, GuardRule, Operator, RuleAssert, RuleLevel, RuleScope};
use crate::rule_executor::package_matcher::is_package_match;
use crate::rule_executor::rule_error::RuleError;

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
            let path = format!("{}", entry.path().display());

            match ext {
                "java" => {
                    let mut file = JavaIdent::parse(content.as_str());
                    file.path = path;
                    models.push(file);
                }
                "js" => {
                    let mut file = JsIdent::parse(content.as_str());
                    file.path = path;
                    models.push(file);
                }
                "rs" => {
                    let mut file = RustIdent::parse(content.as_str());
                    file.path = path;
                    models.push(file);
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

        // filter package to filter package assert
        // - accessed(["..controller..", "..service.."]);
        // - dependBy ""
        if self.capture_package_to_package(&rule, index) {
            return;
        }

        // filter package to class assert
        // filter class to class assert
        self.filter_classes_by_scope(&rule, &mut filtered_models);

        self.execute_classes_assert(&rule, index, filtered_models)
    }

    fn filter_classes_by_scope(&mut self, rule: &&GuardRule, filtered_models: &mut Vec<CodeClass>) {
        match &rule.scope {
            RuleScope::PathDefine(str) => {
                if str.as_str() == "." {
                    for file in &self.models {
                        filtered_models.extend(file.classes.clone());
                    }
                } else {
                    for file in &self.filter_classes_by_package_identifier(str) {
                        filtered_models.extend(file.classes.clone());
                    }
                }
            }
            RuleScope::Implementation(str) => {
                &self.models.iter().for_each(|file| {
                    let classes: Vec<CodeClass> = file.classes.iter()
                        .filter(|class| {
                            class.implements.contains(str)
                        })
                        .map(|s| s.clone())
                        .collect();
                    filtered_models.extend(classes);
                });
            }
            _ => {}
        }
    }

    fn execute_classes_assert(&mut self, rule: &&GuardRule, index: usize, filtered_models: Vec<CodeClass>) {
        match &rule.expr {
            Expr::PropsCall(props) => {
                match props[0].as_str() {
                    "len" => {
                        let size = RuleExecutor::get_assert_sized(&rule);
                        self.process_len(index, size, &rule.ops, filtered_models.len())
                    }
                    "name" => {
                        let string = RuleExecutor::get_assert_string(&rule);
                        self.process_name(index, &rule.ops, filtered_models, string)
                    }
                    _ => {
                        println!("todo: expr {:?}", props[0].as_str());
                    }
                }
            }
            Expr::Identifier(ident) => {
                match ident.as_str() {
                    "" => {
                        let (has_capture, _level, ident) = RuleExecutor::get_package_level(&rule);
                        if has_capture {
                            self.process_package_captures(index, &rule.ops, filtered_models, ident)
                        } else {
                            println!("Empty Identifier: {:?}", ident);
                        }
                    }
                    &_ => {
                        println!("Expr::Identifier: {:?}", ident);
                    }
                }
            }
        }
    }

    fn capture_package_to_package(&mut self, rule: &&GuardRule, index: usize) -> bool {
        let mut has_capture_assert = false;

        let mut assert_models: Vec<CodeFile> = vec![];

        let operator = &rule.ops[0];
        match operator {
            Operator::Accessed => {
                match &rule.assert {
                    RuleAssert::Stringed(pkg_identifier) => {
                        assert_models = self.filter_classes_by_package_identifier(pkg_identifier);
                    }
                    RuleAssert::ArrayStringed(identifiers) => {
                        for ident in identifiers {
                            assert_models.extend(self.filter_classes_by_package_identifier(ident));
                        }
                    }
                    _ => {}
                }

                has_capture_assert = true;
            }
            Operator::DependBy => {
                has_capture_assert = true;
            }
            _ => {}
        }

        let mut pkg_identifier = "".to_string();
        match &rule.scope {
            RuleScope::PathDefine(str) => {
                pkg_identifier = str.clone();
            }
            _ => {}
        }

        let mut error = RuleError::new("access", index);
        let mut assert_success = true;

        match operator {
            Operator::Accessed => {
                let paths = self.search_by_access(&mut assert_models, pkg_identifier);
                if paths.len() > 0 {
                    assert_success = false;
                    paths.iter().for_each(|p| {
                        error.items.push(p.clone());
                    });
                }
            }
            _ => {}
        }

        if !assert_success {
            self.errors.push(error);
        }

        has_capture_assert
    }

    fn search_by_access(&mut self, assert_models: &mut Vec<CodeFile>, pkg_identifier: String) -> Vec<String> {
        let mut error_paths = vec![];
        self.models.iter().for_each(|clz| {
            for imp in &clz.imports {
                let is_file_import = is_package_match(pkg_identifier.clone(), imp.as_str());
                if is_file_import {
                    let mut has_file_in_assert = false;
                    &assert_models.iter().for_each(|file| {
                        if file.path == clz.path {
                            has_file_in_assert = true;
                        }
                    });

                    if !has_file_in_assert {
                        error_paths.push(clz.path.clone());
                    }
                }
            }
        });

        error_paths
    }

    fn capture_package(&mut self, rule: &GuardRule, index: usize) {
        let mut filtered_models: Vec<CodeFile> = vec![];

        match &rule.scope {
            RuleScope::PathDefine(str) => {
                let path = str.as_str();
                if path == "." {
                    filtered_models = self.models.clone();
                } else {
                    filtered_models = self.filter_classes_by_package_identifier(str);
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
                        self.process_len(index, size, &rule.ops, filtered_models.len())
                    }
                    "file" => {
                        match props[1].as_str() {
                            "len" => {
                                let size = RuleExecutor::get_assert_sized(&rule);
                                self.process_len(index, size, &rule.ops, filtered_models.len())
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

    fn filter_classes_by_package_identifier(&mut self, str: &String) -> Vec<CodeFile> {
        self.models.iter()
            .filter(|s| { is_package_match(str.to_string(), s.package.as_str()) })
            .map(|s| { s.clone() })
            .collect()
    }

    fn process_package_captures(&mut self, index: usize, all_ops: &Vec<Operator>, models: Vec<CodeClass>, identifier: String) {
        let mut ops = &all_ops[0];
        let mut has_not = false;
        match ops {
            Operator::Not => {
                ops = &all_ops[1];
                has_not = true;
            }
            _ => {}
        }

        let mut error = RuleError::new("file name", index);

        let mut assert_success = true;
        match ops {
            Operator::Inside |
            Operator::ResideIn => {
                error.msg = format!("resideIn: {:?}", identifier);
                models.iter().for_each(|clz| {
                    let mut package_match = is_package_match(identifier.clone(), clz.package.as_str());
                    if has_not {
                        package_match = !package_match;
                    }

                    if !package_match {
                        let item = format!("path: {}, name: {}", clz.package.clone(), clz.name.clone());
                        error.items.push(item);
                        assert_success = false;
                    }
                });
            }
            _ => {}
        }

        if !assert_success {
            self.errors.insert(index, error);
        }
    }

    fn process_name(&mut self, index: usize, all_ops: &Vec<Operator>, models: Vec<CodeClass>, excepted: String) {
        let mut ops = &all_ops[0];
        let mut has_not = false;
        match ops {
            Operator::Not => {
                ops = &all_ops[1];
                has_not = true;
            }
            _ => {}
        }

        let mut error = RuleError::new("file name", index);

        let match_func: fn(String, &String) -> bool;
        fn starts_with(input: String, condition: &String) -> bool {
            return input.starts_with(condition);
        }
        fn ends_with(input: String, condition: &String) -> bool {
            return input.ends_with(condition);
        }
        fn contains(input: String, condition: &String) -> bool {
            return input.contains(condition);
        }

        let mut assert_success = true;
        match ops {
            Operator::StartsWith => {
                error.msg = format!("startsWith: {:?}", excepted);
                match_func = starts_with;
            }
            Operator::Endswith => {
                error.msg = format!("endsWith: {:?}", excepted);
                match_func = ends_with;
            }
            Operator::Contains => {
                error.msg = format!("contains: {:?}", excepted);
                match_func = contains;
            }
            _ => { return; }
        }

        models.iter().for_each(|clz| {
            let mut is_starts_with = match_func(clz.name.clone(), &excepted);
            if has_not {
                is_starts_with = !is_starts_with
            }
            if !is_starts_with {
                assert_success = false;
                let item = format!("path: {}, name: {}", clz.package.clone(), clz.name.clone());
                error.items.push(item)
            }
        });

        if !assert_success {
            self.errors.insert(index, error);
        }
    }

    fn process_len(&mut self, index: usize, excepted_size: usize, all_ops: &Vec<Operator>, actual_size: usize) {
        let mut ops = &all_ops[0];
        let mut has_not = false;
        match ops {
            Operator::Not => {
                ops = &all_ops[1];
                has_not = true;
            }
            _ => {}
        }

        let mut error = RuleError::new("file.len size", index);
        error.expected = excepted_size.to_string();
        error.actual = actual_size.to_string();

        let is_assert_success: fn(usize, usize) -> bool;
        fn gt(actual: usize, excepted: usize) -> bool {
            return actual > excepted;
        }
        fn gte(actual: usize, excepted: usize) -> bool {
            return actual >= excepted;
        }
        fn lt(actual: usize, excepted: usize) -> bool {
            return actual < excepted;
        }
        fn lte(actual: usize, excepted: usize) -> bool {
            return actual <= excepted;
        }
        fn eq(actual: usize, excepted: usize) -> bool {
            return actual == excepted;
        }

        match ops {
            Operator::Gt => {
                error.msg = format!("file.len = {}, expected: len > {}", actual_size, excepted_size);
                is_assert_success = gt;
            }
            Operator::Gte => {
                error.msg = format!("file.len = {}, expected: len >= {}", actual_size, excepted_size);
                is_assert_success = gte;
            }
            Operator::Lt => {
                error.msg = format!("file.len = {}, expected: len < {}", actual_size, excepted_size);
                is_assert_success = lt;
            }
            Operator::Lte => {
                error.msg = format!("file.len = {}, expected: len <=  {}", actual_size, excepted_size);
                is_assert_success = lte;
            }
            Operator::Eq => {
                error.msg = format!("file.len = {}, expected: len = {}", actual_size, excepted_size);
                is_assert_success = eq;
            }
            _ => { return; }
        }

        let mut is_assert_fail = !is_assert_success(actual_size, excepted_size);
        if has_not {
            is_assert_fail = !is_assert_fail;
        }

        if is_assert_fail {
            self.errors.push(error);
        }
    }

    fn get_assert_sized(rule: &GuardRule) -> usize {
        let mut size = 0;
        match &rule.assert {
            RuleAssert::Sized(sized) => {
                size = *sized;
            }
            _ => {}
        }
        size
    }

    fn get_assert_string(rule: &GuardRule) -> String {
        let mut string = "".to_string();
        match &rule.assert {
            RuleAssert::Stringed(str) => {
                string = str.clone();
            }
            _ => {}
        }
        string
    }

    fn get_package_level(rule: &GuardRule) -> (bool, RuleLevel, String) {
        let mut string = "".to_string();
        let mut level = RuleLevel::Package;
        let mut has_capture = false;
        match &rule.assert {
            RuleAssert::Leveled(lv, package_ident) => {
                has_capture = true;
                level = lv.clone();
                string = package_ident.clone();
            }
            _ => {}
        }

        return (has_capture, level, string);
    }
}