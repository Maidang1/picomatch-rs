pub enum StringOrArray {
    String(String),
    Array(Vec<String>),
}

// 实现 to attry 方法
impl StringOrArray {
    pub fn to_array(&self) -> Vec<String> {
        match self {
            StringOrArray::String(s) => vec![s.to_string()],
            StringOrArray::Array(arr) => arr.clone(),
        }
    }
    pub fn is_array(&self) -> bool {
        match self {
            StringOrArray::Array(_) => true,
            StringOrArray::String(_) => false,
        }
    }
}
