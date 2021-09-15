use std::path::PathBuf;

pub struct PackageUnify {}

impl PackageUnify {
    pub fn from_path(path: PathBuf) -> String {
        let mut package = "".to_string();
        let path_iter = path.iter();
        path_iter.for_each(|str| {
            package = format!("{}.{}", package, str.to_str().expect("error path"));
        });

        package.remove(0);
        package
    }

    pub fn from_rust_import(str: &str, remove_last: bool) -> String {
        let mut package = "".to_string();

        let mut vec = str.split("::").collect::<Vec<&str>>();
        if remove_last {
            vec.remove(vec.len() - 1);
        }

        vec.iter().for_each(|sub| {
            package = format!("{}.{}", package, sub);
        });

        package.remove(0);
        package
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::support::package_unify::PackageUnify;

    #[test]
    fn should_convert_path_to_package() {
        let buf = PathBuf::from("../../../src").join("core").join("domain");
        assert_eq!(".........src.core.domain".to_string(), PackageUnify::from_path(buf));
    }

    #[test]
    fn should_convert_rust_import() {
        let imp = "std::path::PathBuf";
        assert_eq!("std.path".to_string(), PackageUnify::from_rust_import(imp, true));

        let imp = "std::path";
        assert_eq!("std.path".to_string(), PackageUnify::from_rust_import(imp, false));
    }
}