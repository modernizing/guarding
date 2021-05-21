use std::fs;
use std::path::PathBuf;

use crate::rule_executor::RuleExecutor;

fn test_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("_fixtures")
        .join("java")
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
fn should_support_for_reside_in() {
    let content = "class(implementation \"BaseParser\") resideIn package(\"....parser2\");";
    let errors = RuleExecutor::execute(content.to_string(), test_dir());

    assert_eq!(1, errors.len());

    let content = "class(implementation \"BaseParser\") resideIn package(\"....parser\");";
    let errors = RuleExecutor::execute(content.to_string(), test_dir());

    assert_eq!(0, errors.len());
}

#[test]
fn should_support_for_not_reside_in() {
    let content = "class(implementation \"BaseParser\") not resideIn package(\"....parser2\");";
    let errors = RuleExecutor::execute(content.to_string(), test_dir());

    assert_eq!(0, errors.len());

    let content = "class(implementation \"BaseParser\") not resideIn package(\"....parser\");";
    let errors = RuleExecutor::execute(content.to_string(), test_dir());

    assert_eq!(1, errors.len());
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

#[test]
fn should_support_for_not_contains() {
    let content = "class(implementation \"BaseParser\")::name should not contains \"Lexer\";";
    let errors = RuleExecutor::execute(content.to_string(), test_dir());
    assert_eq!(0, errors.len());

    let content = "class(implementation \"BaseParser\")::name should not contains \"Parser\";";
    let errors = RuleExecutor::execute(content.to_string(), test_dir());
    assert_eq!(1, errors.len());
}

#[test]
fn should_support_for_accessed() {
    let content = "class(\"java.util.Map\") only accessed([\"com.phodal.pepper.refactor.staticclass\"]);";
    let errors = RuleExecutor::execute(content.to_string(), test_dir());

    assert_eq!(1, errors.len());
    assert!(errors[0].items[0].contains("MyDictionary.java"))
}
