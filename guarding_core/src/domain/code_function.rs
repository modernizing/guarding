use serde::{Deserialize, Serialize};

use crate::domain::CodePoint;
use crate::domain::Location;

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeFunction {
    pub name: String,
    // todo: thinking in access
    pub vars: Vec<String>,
    pub start: CodePoint,
    pub end: CodePoint
}

impl Default for CodeFunction {
    fn default() -> Self {
        CodeFunction {
            name: "".to_string(),
            vars: vec![],
            start: Default::default(),
            end: Default::default()
        }
    }
}

impl Location for CodeFunction {
    fn set_start(&mut self, row: usize, column: usize) {
        self.start.row = row;
        self.start.column = column;
    }

    fn set_end(&mut self, row: usize, column: usize) {
        self.end.row = row;
        self.end.column = column;
    }
}
