extern crate serde;

use std::path::PathBuf;

use guarding_core::rule_executor::{RuleErrorMsg, RuleExecutor};
use guarding_parser::parser;
use model_builder::ModelBuilder;
use guarding_parser::ast::GuardRule;
use guarding_core::domain::code_file::CodeFile;

pub mod identify;
pub mod model_builder;

pub fn exec_guarding(rule_content: String, code_dir: PathBuf) -> Vec<RuleErrorMsg> {
    match parser::parse(rule_content.as_str()) {
        Err(e) => {
            println!("{}", e);
            vec![]
        },
        Ok(rules) => {
            let models = ModelBuilder::build_models_by_dir(code_dir);
            exec(rules, models)
        }
    }
}

fn exec(rules: Vec<GuardRule>, models: Vec<CodeFile>) -> Vec<RuleErrorMsg> {
    let mut executor = RuleExecutor::new(models, rules);
    executor.run();

    return executor.errors;
}


#[cfg(test)]
mod tests;
