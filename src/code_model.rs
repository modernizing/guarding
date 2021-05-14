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

pub struct CodeClass {
    pub name: String,
    pub path: String,
    pub constant: Vec<ClassConstant>,
    pub method: Vec<CodeMethod>,
}

pub struct ClassConstant {
    pub name: String,
    pub typ: String,
}

pub struct CodeMethod {
    pub name: String,
    pub vars: Vec<String>,
}

