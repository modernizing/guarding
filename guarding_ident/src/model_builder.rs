use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use guarding_core::domain::code_file::CodeFile;
use crate::identify::c_sharp_ident::CSharpIdent;
use crate::identify::code_ident::CodeIdent;
use crate::identify::java_ident::JavaIdent;
use crate::identify::js_ident::JsIdent;
use crate::identify::rust_ident::RustIdent;

pub struct ModelBuilder {}

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
        let file_name = path.file_name().unwrap().to_str().unwrap();

        match ext {
            "java" => {
                let mut file = JavaIdent::parse(ModelBuilder::read_content(path).as_str());
                file.path = ModelBuilder::format_path(path);
                file.file_name = file_name.to_string();
                models.push(file);
            }
            "js" => {
                let mut file = JsIdent::parse(ModelBuilder::read_content(path).as_str());
                file.path = format!("{}", path.display());
                file.file_name = file_name.to_string();
                models.push(file);
            }
            "rs" => {
                let mut file = RustIdent::parse(ModelBuilder::read_content(path).as_str());
                file.path = format!("{}", path.display());
                file.file_name = file_name.to_string();
                models.push(file);
            }
            "cs" => {
                let mut file = CSharpIdent::parse(ModelBuilder::read_content(path).as_str());
                file.path = format!("{}", path.display());
                file.file_name = file_name.to_string();
                models.push(file);
            }
            &_ => {}
        }
    }

    fn read_content(path: &Path) -> String {
        fs::read_to_string(path).expect("not such file")
    }

    fn format_path(path: &Path) -> String {
        format!("{}", path.display())
    }
}


#[cfg(test)]
mod tests {
    use std::env;
    use crate::ModelBuilder;

    #[test]
    fn should_parse_current_dir() {
        let dir = env::current_dir().unwrap();
        let models = ModelBuilder::build_models_by_dir(dir);

        assert!(models.len() > 0);
    }
}