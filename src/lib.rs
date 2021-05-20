extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate serde;

use tree_sitter::Language;

extern "C" { fn tree_sitter_rust() -> Language; }

extern "C" { fn tree_sitter_java() -> Language; }

extern "C" { fn tree_sitter_javascript() -> Language; }

pub mod identify;
pub mod parser;
pub mod rule_executor;
pub mod package_matcher;

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

        let errors = RuleExecutor::execute(content, code_dir);

        assert_eq!(1, errors.len());
        assert_eq!("50".to_string(), errors[0].expected);
        assert_eq!("26".to_string(), errors[0].actual);
        assert_eq!("file.len size".to_string(), errors[0].error_type);
        assert_eq!("file.len = 26, expected: len > 50".to_string(), errors[0].msg);
    }

    #[test]
    fn should_get_errors_when_lt() {
        let content = "package(\".\")::file.len should = 27;";
        let errors = RuleExecutor::execute(content.to_string(), test_dir());

        assert_eq!(1, errors.len());
    }

    #[test]
    fn should_support_filter() {
        let content = "package(\"com.phodal.pepper.refactor.parser\")::file.len should = 3;";
        let errors = RuleExecutor::execute(content.to_string(), test_dir());

        assert_eq!(0, errors.len());
    }

    #[test]
    fn should_support_for_class_filter() {
        let content = "class(\".\")::len should < 25;
class(\".\")::len should > 20;";
        let errors = RuleExecutor::execute(content.to_string(), test_dir());

        assert_eq!(0, errors.len());
    }

    #[test]
    fn should_support_for_extends_count() {
        let content = "class(implementation \"BaseParser\")::len = 2";
        let errors = RuleExecutor::execute(content.to_string(), test_dir());

        assert_eq!(0, errors.len());
    }

    #[test]
    fn should_support_for_extends_ends_with() {
        let content = "class(implementation \"BaseParser\")::name should endsWith \"Parser2\";";
        let errors = RuleExecutor::execute(content.to_string(), test_dir());
        assert_eq!(1, errors.len());
        assert_eq!(2, errors[0].items.len());

        let correct_content = "class(implementation \"BaseParser\")::name should endsWith \"Parser\";";
        let errors = RuleExecutor::execute(correct_content.to_string(), test_dir());
        assert_eq!(0, errors.len());
    }

    #[test]
    fn should_support_for_starts_with() {
        let content = "class(implementation \"BaseParser\")::name should startsWith \"Json\";";
        let errors = RuleExecutor::execute(content.to_string(), test_dir());
        assert_eq!(1, errors.len());
        assert_eq!(1, errors[0].items.len());
    }

    #[test]
    fn should_support_for_contains() {
        let content = "class(implementation \"BaseParser\")::name should contains \"Lexer\";";
        let errors = RuleExecutor::execute(content.to_string(), test_dir());
        assert_eq!(1, errors.len());
        assert_eq!(2, errors[0].items.len());

        let content = "class(implementation \"BaseParser\")::name should contains \"Parser\";";
        let errors = RuleExecutor::execute(content.to_string(), test_dir());
        assert_eq!(0, errors.len());
    }
}