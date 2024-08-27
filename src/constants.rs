use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    pub static ref REPLACEMENTS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("***", "*");
        m.insert("**/**", "**");
        m.insert("**/**/**", "**");
        m
    };
}

lazy_static! {
    pub static ref MAX_LENGTH: i32 = 1024 * 64;
}

lazy_static! {
    pub static ref WIN_SLASH: &'static str = "\\\\/";
    pub static ref WIN_NO_SLASH: &'static str = "[^\\\\/]";
    pub static ref DOT_LITERAL: &'static str = "\\.";
    pub static ref PLUS_LITERAL: &'static str = "\\+";
    pub static ref QMARK_LITERAL: &'static str = "\\?";
    pub static ref SLASH_LITERAL: &'static str = "\\/";
    pub static ref ONE_CHAR: &'static str = "(?=.)";
    pub static ref QMARK: &'static str = "[^/]";
    pub static ref END_ANCHOR: &'static str = "(?:\\/|$)";
    pub static ref START_ANCHOR: &'static str = "(?:^|\\/)";
    pub static ref DOTS_SLASH: &'static str = "\\.{1,2}(?:\\/|$)";
    pub static ref NO_DOT: &'static str = "(?!\\.)";
    pub static ref NO_DOTS: &'static str = "(?!^(?:^|\\/)(?:\\.{1,2}(?:\\/|$)))";
    pub static ref NO_DOT_SLASH: &'static str = "(?!\\.{0,1}(?:\\/|$))";
    pub static ref NO_DOTS_SLASH: &'static str = "(?!\\.{1,2}(?:\\/|$))";
    pub static ref QMARK_NO_DOT: &'static str = "[^./]";
    pub static ref STAR: &'static str = "[^/]*?";
    pub static ref SEP: &'static str = "/";
    pub static ref POSIX_CHARS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("DOT_LITERAL", DOT_LITERAL.clone());
        m.insert("PLUS_LITERAL", PLUS_LITERAL.clone());
        m.insert("QMARK_LITERAL", QMARK_LITERAL.clone());
        m.insert("SLASH_LITERAL", SLASH_LITERAL.clone());
        m.insert("ONE_CHAR", ONE_CHAR.clone());
        m.insert("QMARK", QMARK.clone());
        m.insert("END_ANCHOR", END_ANCHOR.clone());
        m.insert("START_ANCHOR", START_ANCHOR.clone());
        m.insert("DOTS_SLASH", DOTS_SLASH.clone());
        m.insert("NO_DOT", NO_DOT.clone());
        m.insert("NO_DOTS", NO_DOTS.clone());
        m.insert("NO_DOT_SLASH", NO_DOT_SLASH.clone());
        m.insert("NO_DOTS_SLASH", NO_DOTS_SLASH.clone());
        m.insert("QMARK_NO_DOT", QMARK_NO_DOT.clone());
        m.insert("STAR", STAR.clone());
        m.insert("SEP", SEP.clone());
        m
    };
    pub static ref WINDOWS_CHARS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.extend(POSIX_CHARS.clone());
        let slash_literal = format!("[{}]", WIN_SLASH.clone());
        let star = format!("{}*?", WIN_NO_SLASH.clone());
        let dots_slash = format!(
            "{}{}(?:[{}]|$)",
            DOT_LITERAL.clone(),
            "{1,2}",
            WIN_SLASH.clone()
        );
        let no_dot = format!("(?!{})", DOT_LITERAL.clone());
        let no_dots = format!(
            "(?!(?:^|[{}]){}{}(?:[{}]|$))",
            WIN_SLASH.clone(),
            DOT_LITERAL.clone(),
            "{1,2}",
            WIN_SLASH.clone()
        );
        let no_dot_slash = format!(
            "(?!{}{}(?:[{}]|$))",
            DOT_LITERAL.clone(),
            "{0,1}",
            WIN_SLASH.clone()
        );
        let no_dots_slash = format!(
            "(?!{}{}(?:[{}]|$))",
            DOT_LITERAL.clone(),
            "{1,2}",
            WIN_SLASH.clone()
        );
        let qmark_no_dot = format!("[^.{}]", WIN_SLASH.clone());
        let start_anchor = format!("(?:^|[{}])", WIN_SLASH.clone());
        let end_anchor = format!("(?:[{}]|$)", WIN_SLASH.clone());
        m.insert("SLASH_LITERAL", Box::leak(slash_literal.into_boxed_str()));
        m.insert("QMARK", WIN_NO_SLASH.clone());
        m.insert("STAR", Box::leak(star.into_boxed_str()));
        m.insert("DOTS_SLASH", Box::leak(dots_slash.into_boxed_str()));
        m.insert("NO_DOT", Box::leak(no_dot.into_boxed_str()));
        m.insert("NO_DOTS", Box::leak(no_dots.into_boxed_str()));
        m.insert("NO_DOT_SLASH", Box::leak(no_dot_slash.into_boxed_str()));
        m.insert("NO_DOTS_SLASH", Box::leak(no_dots_slash.into_boxed_str()));
        m.insert("QMARK_NO_DOT", Box::leak(qmark_no_dot.into_boxed_str()));
        m.insert("START_ANCHOR", Box::leak(start_anchor.into_boxed_str()));
        m.insert("END_ANCHOR", Box::leak(end_anchor.into_boxed_str()));
        m.insert("SEP", "\\\\");
        m
    };
}

lazy_static! {
    pub static ref POSIX_REGEX_SOURCE: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("alnum", "a-zA-Z0-9");
        m.insert("alpha", "a-zA-Z");
        m.insert("ascii", "\x00-\x7F");
        m.insert("blank", " \t");
        m.insert("cntrl", "\x00-\x1F\x7F");
        m.insert("digit", "0-9");
        m.insert("graph", "\x21-\x7E");
        m.insert("lower", "a-z");
        m.insert("print", "\x20-\x7E ");
        m.insert("punct", "\\-!\"#$%&\'()\\*+,./:;<=>?@[\\]^_`{|}~");
        m.insert("space", r"\t\r\n\v\f");
        m.insert("upper", "A-Z");
        m.insert("word", "A-Za-z0-9_");
        m.insert("xdigit", "A-Fa-f0-9");
        m
    };
}

lazy_static! {
    pub static ref REGEX_BACKSLASH: Regex = Regex::new(r"\\(?![*+?^${}(|)[\]])").unwrap();
    pub static ref REGEX_NON_SPECIAL_CHARS: Regex =
        Regex::new(r"^[^@![\].,$*+?^{}()|\\/]+").unwrap();
    pub static ref REGEX_SPECIAL_CHARS: Regex = Regex::new(r"[-*+?.^${}(|)[\]]").unwrap();
    pub static ref REGEX_SPECIAL_CHARS_BACKREF: Regex = Regex::new(r"(\\?)((\W)(\3*))").unwrap();
    pub static ref REGEX_SPECIAL_CHARS_GLOBAL: Regex = Regex::new(r"([-*+?.^${}(|)[\]])").unwrap();
    pub static ref REGEX_REMOVE_BACKSLASH: Regex = Regex::new(r"(?:\[.*?[^\\]\]|\\(?=.))").unwrap();
}

lazy_static! {
    // 数字字符
    pub static ref CHAR_0: u8 = 48; // 0
    pub static ref CHAR_9: u8 = 57; // 9

    // 字母字符
    pub static ref CHAR_UPPERCASE_A: u8 = 65; // A
    pub static ref CHAR_LOWERCASE_A: u8 = 97; // a
    pub static ref CHAR_UPPERCASE_Z: u8 = 90; // Z
    pub static ref CHAR_LOWERCASE_Z: u8 = 122; // z

    // 括号字符
    pub static ref CHAR_LEFT_PARENTHESES: i32 = 40; // (
    pub static ref CHAR_RIGHT_PARENTHESES: i32 = 41; // )

    // 星号字符
    pub static ref CHAR_ASTERISK: i32 = 42; // *

    // 非字母字符
    pub static ref CHAR_AMPERSAND: u8 = 38; // &
    pub static ref CHAR_AT: i32 = 64; // @
    pub static ref CHAR_BACKWARD_SLASH: i32 = 92; // \
    pub static ref CHAR_CARRIAGE_RETURN: u8 = 13; // \r
    pub static ref CHAR_CIRCUMFLEX_ACCENT: u8 = 94; // ^
    pub static ref CHAR_COLON: u8 = 58; // :
    pub static ref CHAR_COMMA: i32 = 44; // ,
    pub static ref CHAR_DOT: i32 = 46; // .
    pub static ref CHAR_DOUBLE_QUOTE: u8 = 34; // "
    pub static ref CHAR_EQUAL: u8 = 61; // =
    pub static ref CHAR_EXCLAMATION_MARK: i32 = 33;
    pub static ref CHAR_FORM_FEED: u8 = 12; // \f
    pub static ref CHAR_FORWARD_SLASH: i32 = 47; // /
    pub static ref CHAR_GRAVE_ACCENT: u8 = 96; // `
    pub static ref CHAR_HASH: u8 = 35; // #
    pub static ref CHAR_HYPHEN_MINUS: u8 = 45; // -
    pub static ref CHAR_LEFT_ANGLE_BRACKET: u8 = 60; // <
    pub static ref CHAR_LEFT_CURLY_BRACE: i32 = 123; // {
    pub static ref CHAR_LEFT_SQUARE_BRACKET: i32 = 91; // [
    pub static ref CHAR_LINE_FEED: u8 = 10; // \n
    pub static ref CHAR_NO_BREAK_SPACE: u16 = 160; // \u00A0
    pub static ref CHAR_PERCENT: u8 = 37; // %
    pub static ref CHAR_PLUS: i32 = 43; // +
    pub static ref CHAR_QUESTION_MARK: i32 = 63; // ?
    pub static ref CHAR_RIGHT_ANGLE_BRACKET: u8 = 62; // >
    pub static ref CHAR_RIGHT_CURLY_BRACE: i32 = 125; // }
    pub static ref CHAR_RIGHT_SQUARE_BRACKET: i32 = 93; // ]
    pub static ref CHAR_SEMICOLON: u8 = 59; // ;
    pub static ref CHAR_SINGLE_QUOTE: u8 = 39; // '
    pub static ref CHAR_SPACE: u8 = 32; //
    pub static ref CHAR_TAB: u8 = 9; // \t
    pub static ref CHAR_UNDERSCORE: u8 = 95; // _
    pub static ref CHAR_VERTICAL_LINE: u8 = 124; // |
    pub static ref CHAR_ZERO_WIDTH_NOBREAK_SPACE: u16 = 65279; // \uFEFF
}
