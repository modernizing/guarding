use serde::{Deserialize, Serialize};

use crate::domain::code_class::CodeClass;

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodePackage {
    pub name: String,
    pub path: String,
    pub class: Vec<CodeClass>,
}
