pub struct CodeModule {
    pub name: String,
    pub path: String,
    pub package: Vec<CodePackage>,
}

pub struct CodePackage {
    pub name: String,
    pub path: String,
    pub class: Vec<CodeClass>,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct CodeClass {
    pub name: String,
    pub path: String,
    pub constant: Vec<ClassConstant>,
    pub functions: Vec<CodeFunction>,
}

impl Default for CodeClass {
    fn default() -> Self {
        CodeClass {
            name: "".to_string(),
            path: "".to_string(),
            constant: vec![],
            functions: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClassConstant {
    pub name: String,
    pub typ: String,
}

#[derive(Clone, Debug)]
pub struct CodeFunction {
    pub name: String,
    pub vars: Vec<String>,
}

impl Default for CodeFunction {
    fn default() -> Self {
        CodeFunction {
            name: "".to_string(),
            vars: vec![],
        }
    }
}

