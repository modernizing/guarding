use std::fs;
use std::path::PathBuf;

use guarding::rule_executor::RuleExecutor;

fn main() {
    let buf = PathBuf::from(".");
    let guarding = buf.clone().join("guarding.guarding");
    let content = fs::read_to_string(guarding).unwrap();
    RuleExecutor::execute(content, buf);
}
