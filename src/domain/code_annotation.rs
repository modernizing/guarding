use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeAnnotation {
    pub name: String,
    pub key_values: Vec<AnnotationKeyValue>
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnnotationKeyValue {
    pub key: String,
    pub values: Vec<AnnotationKeyValue>
}
