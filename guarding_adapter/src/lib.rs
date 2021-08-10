use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use guarding_core::domain::code_file::CodeFile;
use guarding_core::rule_executor::{RuleExecutor};
use guarding_parser::parser;

#[no_mangle]
pub extern fn from_string(models: *const c_char, rules: *const c_char) -> *mut c_char {
    let rule_str = unsafe {
        assert!(!rules.is_null());

        CStr::from_ptr(rules)
    };

    let rules = rule_str.to_str().unwrap();
    let guard_rules = parser::parse(rules).unwrap();

    let model = unsafe {
        assert!(!models.is_null());

        CStr::from_ptr(models)
    };

    let model_str = model.to_str().unwrap();
    let code_files: Vec<CodeFile> = serde_json::from_str(model_str).unwrap();

    let mut executor = RuleExecutor::new(code_files, guard_rules);
    executor.run();

    let json = serde_json::to_string(&executor.errors).unwrap();
    let c_str = CString::new(json).unwrap();
    // println!("{:?}", c_str);
    c_str.into_raw()
}