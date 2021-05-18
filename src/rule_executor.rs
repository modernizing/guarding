use crate::RuleError;
use std::collections::HashMap;

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
    pub errors: HashMap<usize, RuleError>
}