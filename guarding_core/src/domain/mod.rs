use serde::{Deserialize, Serialize};

use code_class::CodeClass;

pub mod code_import;
pub mod code_file;
pub mod code_package;
pub mod code_module;
pub mod code_class;
pub mod code_function;
pub mod code_annotation;

impl Location for CodeClass {
    fn set_start(&mut self, row: usize, column: usize) {
        self.start.row = row;
        self.start.column = column;
    }

    fn set_end(&mut self, row: usize, column: usize) {
        self.end.row = row;
        self.end.column = column;
    }
}

pub trait Location {
    fn set_start(&mut self, row: usize, column: usize);
    fn set_end(&mut self, row: usize, column: usize);
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodePoint {
    pub row: usize,
    pub column: usize
}

impl Default for CodePoint {
    fn default() -> Self {
        CodePoint {
            row: 0,
            column: 0
        }
    }
}
