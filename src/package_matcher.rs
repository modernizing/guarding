use regex::{Regex};

pub fn matches(package_identifier: String, text: &str) -> bool {
    let package = convert_to_regex(package_identifier);
    let regex = Regex::new(package.as_str())
        .expect("regex error");

    regex.is_match(text)
}

pub fn convert_to_regex(package_identifier: String) -> String {
    package_identifier
        .replace("*", "\\w+")
        .replace(".", "\\.")
        .replace("\\.\\.", "(?:(?:^\\w*)?\\.(?:\\w+\\.)*(?:\\w*$)?)?")
}

#[cfg(test)]
mod tests {
    use crate::package_matcher::matches;

    #[test]
    fn should_working_in_process() {
        assert_eq!(true, matches("..".to_string(), "com.phodal.zero"));
        assert_eq!(true, matches("com.(*)..service.(**)".to_string(), "com.mycompany.some.service.special.name"));
    }
}