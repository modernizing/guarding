use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeImport {
    pub name: String,
    pub import: String,
    pub source: String,
}
