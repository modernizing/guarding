use serde::{Deserialize, Serialize};

use crate::domain::code_package::CodePackage;

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeModule {
    pub name: String,
    pub path: String,
    pub package: Vec<CodePackage>,
}
