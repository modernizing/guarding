extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate serde;

use tree_sitter::Language;
use std::fs;
use std::path::PathBuf;
use crate::rule_executor::RuleExecutor;

extern "C" { fn tree_sitter_rust() -> Language; }

extern "C" { fn tree_sitter_java() -> Language; }

extern "C" { fn tree_sitter_javascript() -> Language; }

pub mod identify;
pub mod parser;
pub mod rule_executor;

fn main() {
    // test program
    let buf = PathBuf::from(".");
    let guarding = buf.clone().join("guarding.guarding");
    let content = fs::read_to_string(guarding).unwrap();
    RuleExecutor::execute(content, buf);
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use crate::rule_executor::RuleExecutor;

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

        let errors =  RuleExecutor::execute(content, code_dir);

        assert_eq!(1, errors.len());
    }

    #[test]
    fn should_get_errors_when_lt() {
        let code_dir = test_dir();
        let content = "package(\".\")::file.len should = 27;";
        let errors = RuleExecutor::execute(content.to_string(), code_dir);

        println!("{:?}", errors);
        assert_eq!(1, errors.len());
    }
}