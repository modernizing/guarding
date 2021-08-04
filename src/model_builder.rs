use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use guarding_core::domain::code_file::CodeFile;
use crate::identify::code_ident::CodeIdent;
use crate::identify::java_ident::JavaIdent;
use crate::identify::js_ident::JsIdent;
use crate::identify::rust_ident::RustIdent;

pub struct ModelBuilder {

}

impl ModelBuilder {
    pub fn build_models_by_dir(code_dir: PathBuf) -> Vec<CodeFile> {
        let mut models = vec![];
        for entry in WalkDir::new(code_dir) {
            let entry = entry.unwrap();
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            if let None = path.extension() {
                continue;
            }

            ModelBuilder::build_model_by_file(&mut models, path)
        }
        models
    }

    pub fn build_model_by_file(models: &mut Vec<CodeFile>, path: &Path) {
        let ext = path.extension().unwrap().to_str().unwrap();
        let content = fs::read_to_string(path).expect("not such file");
        let path = format!("{}", path.display());

        match ext {
            "java" => {
                let mut file = JavaIdent::parse(content.as_str());
                file.path = path;
                models.push(file);
            }
            "js" => {
                let mut file = JsIdent::parse(content.as_str());
                file.path = path;
                models.push(file);
            }
            "rs" => {
                let mut file = RustIdent::parse(content.as_str());
                file.path = path;
                models.push(file);
            }
            &_ => {}
        }
    }
}