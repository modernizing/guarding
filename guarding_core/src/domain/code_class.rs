use serde::{Deserialize, Serialize};

use crate::domain::code_function::CodeFunction;
use crate::domain::CodePoint;

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeClass {
    pub name: String,
    pub package: String,
    pub extends: Vec<String>,
    pub implements: Vec<String>,
    pub constant: Vec<ClassConstant>,
    pub functions: Vec<CodeFunction>,
    pub start: CodePoint,
    pub end: CodePoint
}

impl Default for CodeClass {
    fn default() -> Self {
        CodeClass {
            name: "".to_string(),
            package: "".to_string(),
            extends: vec![],
            implements: vec![],
            constant: vec![],
            functions: vec![],
            start: Default::default(),
            end: Default::default()
        }
    }
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassConstant {
    pub name: String,
    pub typ: String,
}
