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

#[derive(Debug, Default)]
pub struct State {
    pub prefix: Option<String>,
    pub negated: bool,
}
pub trait HasPrefixAndNegated {
    fn prefix(&self) -> &Option<String>;
    fn negated(&self) -> bool;
    fn set_prefix(&mut self, prefix: String);
}

impl HasPrefixAndNegated for State {
    fn prefix(&self) -> &Option<String> {
        &self.prefix
    }

    fn negated(&self) -> bool {
        self.negated
    }

    fn set_prefix(&mut self, prefix: String) {
        self.prefix = Some(prefix);
    }
}

/// Remove prefix from a string and update state
pub fn remove_prefix<T: HasPrefixAndNegated>(input: &str, state: &mut T) -> String {
    let mut output = input.to_string();
    if output.starts_with("./") {
        output = output[2..].to_string();
        state.set_prefix("./".to_string());
    }
    output
}

/// Slice a string
pub fn slice(s: &str, start: isize, end: Option<isize>) -> String {
    let len = s.chars().count() as isize;

    // 处理负数索引
    let start = if start < 0 {
        std::cmp::max(len + start, 0)
    } else {
        std::cmp::min(start, len)
    } as usize;

    let end = match end {
        Some(e) => {
            let e = if e < 0 {
                std::cmp::max(len + e, 0)
            } else {
                std::cmp::min(e, len)
            };
            e as usize
        }
        None => s.chars().count(),
    };

    // 确保 end >= start
    let end = std::cmp::max(start, end);

    // 收集字符到新的字符串
    s.chars().skip(start).take(end - start).collect()
}
