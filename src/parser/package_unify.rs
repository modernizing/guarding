use std::path::PathBuf;

pub fn path_to_package(path: PathBuf) -> String {
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::parser::package_unify::path_to_package;

    #[test]
    fn should_convert_path_to_package() {
        let buf = PathBuf::from("src").join("core").join("domain");
        assert_eq!("src.core.domain".to_string(), path_to_package(buf));
    }
}