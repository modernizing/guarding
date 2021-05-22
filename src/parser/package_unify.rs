use std::path::PathBuf;

pub struct PackageUnify {}

impl PackageUnify {
    pub fn from_path(path: PathBuf) -> String {
        let mut package = "".to_string();
        let path_iter = path.iter();
        path_iter.for_each(|str| {
            match str.to_str() {
                None => {}
                Some(sub) => {
                    package = format!("{}.{}", package, sub);
                }
            }
        });

        package.remove(0);
        package
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::parser::package_unify::PackageUnify;

    #[test]
    fn should_convert_path_to_package() {
        let buf = PathBuf::from("src").join("core").join("domain");
        assert_eq!("src.core.domain".to_string(), PackageUnify::from_path(buf));
    }
}