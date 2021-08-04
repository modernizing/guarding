extern crate serde;

use std::path::PathBuf;
use crate::rule_executor::{RuleErrorMsg, RuleExecutor};
use guarding_parser::parser;
use crate::rule_executor::model_builder::ModelBuilder;

pub mod identify;
pub mod rule_executor;
pub mod domain;

pub fn exec_guarding(rule_content: String, code_dir: PathBuf) -> Vec<RuleErrorMsg> {
    match parser::parse(rule_content.as_str()) {
        Err(e) => {
            println!("{}", e);
            vec![]
        },
        Ok(rules) => {
            let models = ModelBuilder::build_models_by_dir(code_dir);

            let mut executor = RuleExecutor::new(models, rules);
            executor.run();

            return executor.errors;
        }
    }
}


#[cfg(test)]
mod tests;