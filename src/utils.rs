use core::str;
use std::collections::HashMap;

use crate::{constants::*, parse::ParseState};

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

pub fn glob_chars(win32: bool) -> HashMap<&'static str, &'static str> {
    if win32 {
        WINDOWS_CHARS.clone()
    } else {
        POSIX_CHARS.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ExtglobChar {
    pub char_type: &'static str,
    pub open: &'static str,
    pub close: &'static str,
}

pub fn extglob_chars(chars: &HashMap<&str, &str>) -> HashMap<char, ExtglobChar> {
    let star = chars.get("STAR").unwrap_or(&"*");

    let mut map = HashMap::new();
    map.insert(
        '!',
        ExtglobChar {
            char_type: "negate",
            open: "(?:(?!(?:",
            close: Box::leak(format!(")){}))", star).into_boxed_str()),
        },
    );
    map.insert(
        '?',
        ExtglobChar {
            char_type: "qmark",
            open: "(?:",
            close: ")?",
        },
    );
    map.insert(
        '+',
        ExtglobChar {
            char_type: "plus",
            open: "(?:",
            close: ")+",
        },
    );
    map.insert(
        '*',
        ExtglobChar {
            char_type: "star",
            open: "(?:",
            close: ")*",
        },
    );
    map.insert(
        '@',
        ExtglobChar {
            char_type: "at",
            open: "(?:",
            close: ")",
        },
    );

    map
}

pub fn remove_back_slashes(input: &String) -> String {
    REGEX_REMOVE_BACKSLASH.replace_all(&input, "").to_string()
}

pub trait HasPrefix {
    fn has_prefix(&self, prefix: &str) -> &String;
    fn set_prefix(&mut self, prefix: String);
}

pub fn remove_prefix<'a, T: HasPrefix>(input: &'a str, state: &'a mut T) -> &'a str {
    let mut output = input;
    if output.starts_with("./") {
        output = &output[2..];
        state.set_prefix("./".to_string());
    }
    output
}
