#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleError {
    pub expected: String,
    pub actual: String,
    pub error_type: String,
    pub msg: String,
    pub items: Vec<String>,
    pub rule: usize,
}

impl RuleError {
    
}
