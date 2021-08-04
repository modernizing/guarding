use serde::{Deserialize, Serialize};

use crate::domain::code_function::CodeFunction;
use crate::domain::code_class::CodeClass;

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeFile {
    pub name: String,
    pub path: String,
    pub package: String,
    pub imports: Vec<String>,
    pub classes: Vec<CodeClass>,
    pub functions: Vec<CodeFunction>,
}

impl Default for CodeFile {
    fn default() -> Self {
        CodeFile {
            name: "".to_string(),
            path: "".to_string(),
            package: "".to_string(),
            imports: vec![],
            classes: vec![],
            functions: vec![],
        }
    }
}
