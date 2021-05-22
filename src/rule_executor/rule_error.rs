use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleError {
    pub expected: String,
    pub actual: String,
    pub error_type: String,
    pub msg: String,
    pub items: Vec<String>,
    pub rule_index: usize,
}

impl RuleError {
    pub fn new(error_type: &str, index: usize) -> RuleError {
        RuleError {
            expected: "".to_string(),
            actual: "".to_string(),
            error_type: error_type.to_string(),
            msg: "".to_string(),
            items: vec![],
            rule_index: index
        }
    }
}

impl Default for RuleError {
    fn default() -> Self {
        RuleError {
            expected: "".to_string(),
            actual: "".to_string(),
            error_type: "".to_string(),
            msg: "".to_string(),
            items: vec![],
            rule_index: 0
        }
    }
}
