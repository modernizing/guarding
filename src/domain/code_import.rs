use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeImport {
    pub name: String,
    pub import: String,
    pub source: String,
}
