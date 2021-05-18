use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeModule {
    pub name: String,
    pub path: String,
    pub package: Vec<CodePackage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodePackage {
    pub name: String,
    pub path: String,
    pub class: Vec<CodeClass>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeFile {
    pub name: String,
    pub path: String,
    pub imports: Vec<String>,
    pub classes: Vec<CodeClass>,
    pub functions: Vec<CodeFunction>,
}

impl Default for CodeFile {
    fn default() -> Self {
        CodeFile {
            name: "".to_string(),
            path: "".to_string(),
            imports: vec![],
            classes: vec![],
            functions: vec![],
        }
    }
}

pub struct CodeImport {
    pub name: String,
    pub import: String,
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeClass {
    pub name: String,
    pub path: String,
    pub extends: Vec<String>,
    pub implements: Vec<String>,
    pub constant: Vec<ClassConstant>,
    pub functions: Vec<CodeFunction>,
    pub start: CodePoint,
    pub end: CodePoint
}

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

impl Default for CodeClass {
    fn default() -> Self {
        CodeClass {
            name: "".to_string(),
            path: "".to_string(),
            extends: vec![],
            implements: vec![],
            constant: vec![],
            functions: vec![],
            start: Default::default(),
            end: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassConstant {
    pub name: String,
    pub typ: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeFunction {
    pub name: String,
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

pub trait Location {
    fn set_start(&mut self, row: usize, column: usize);
    fn set_end(&mut self, row: usize, column: usize);
}
