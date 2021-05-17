extern crate pest;
#[macro_use]
extern crate pest_derive;

extern crate serde;

use tree_sitter::Language;

extern "C" { fn tree_sitter_rust() -> Language; }
extern "C" { fn tree_sitter_java() -> Language; }
extern "C" { fn tree_sitter_javascript() -> Language; }

pub mod identify;
pub mod parser;

fn main() {

}
#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::fs;
    use crate::parser;
    use walkdir::WalkDir;
    use crate::identify::java_ident::JavaIdent;

    fn test_dir() -> PathBuf {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let test_dir = root_dir.join("_fixtures")
            .join("java");

        test_dir
    }

    #[test]
    fn should_working_in_process() {
        let path = test_dir().join("size.guarding");
        let content = fs::read_to_string(path).expect("not file");
        let vec = parser::parse(content.as_str());


        let mut models = vec![];
        for entry in WalkDir::new(test_dir()) {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                let file_name = entry.file_name().to_str().expect("error file name");
                if file_name.ends_with(".java") {
                    let content = fs::read_to_string(entry.path()).expect("not such file");
                    models.push(JavaIdent::parse(content.as_str()));
                }
            }
        }

        println!("{:?}", models);
        for _rule in vec {
            // rule.capture(rule);
        }
    }
}