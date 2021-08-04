use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MismatchType {
    None,
    Access,
    FileName,
    FileSize,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RuleErrorMsg {
    pub expected: String,
    pub actual: String,
    pub mismatch_type: MismatchType,
    pub msg: String,
    pub items: Vec<String>,
    pub rule_index: usize,
}

impl RuleErrorMsg {
    pub fn new(mismatch_type: MismatchType, index: usize) -> RuleErrorMsg {
        RuleErrorMsg {
            expected: "".to_string(),
            actual: "".to_string(),
            mismatch_type,
            msg: "".to_string(),
            items: vec![],
            rule_index: index
        }
    }
}

impl Default for RuleErrorMsg {
    fn default() -> Self {
        RuleErrorMsg {
            expected: "".to_string(),
            actual: "".to_string(),
            mismatch_type: MismatchType::None,
            msg: "".to_string(),
            items: vec![],
            rule_index: 0
        }
    }
}
