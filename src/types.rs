use regex::{Error, Regex};

use crate::utils::StringOrArray;

pub struct PicomatchOptions {
    basename: Option<bool>,
    bash: Option<bool>,
    capture: Option<bool>,
    pub contains: Option<bool>,
    cwd: Option<String>,
    pub debug: Option<bool>,
    dot: Option<bool>,
    // expandRange: todo function
    failglob: Option<bool>,
    fastpaths: Option<bool>,
    flags: Option<bool>,
    // format todo function
    ignore: Option<StringOrArray>,
    keep_quotes: Option<bool>,
    literal_brackets: Option<bool>,
    match_base: Option<bool>,
    max_length: Option<bool>,
    nobrace: Option<bool>,
    nobracket: Option<bool>,
    pub nocase: Option<bool>,
    nodupes: Option<bool>,
    noext: Option<bool>,
    noextglob: Option<bool>,
    noglobstar: Option<bool>,
    nonegate: Option<bool>,
    noquantifiers: Option<bool>,
    // on_ignore: todo
    // onMatch
    // onResult
    pub posix: Option<bool>,
    posix_slashes: Option<bool>,
    prepend: Option<bool>,
    regex: Option<bool>,
    strict_brackets: Option<bool>,
    strict_slashes: Option<bool>,
    unescape: Option<bool>,
    unixify: Option<bool>,
    pub windows: Option<bool>,
}

pub struct State {
    pub output: String,
    pub negated: bool,
}

pub struct CompileReResult {
    pub regex: Result<Regex, Error>,
    pub state: Option<State>,
}

pub enum CompileReReturn {
    String(String),
    CompileReResult(CompileReResult),
}

impl CompileReReturn {
    pub fn is_string(&self) -> bool {
        match self {
            CompileReReturn::String(_) => true,
            _ => false,
        }
    }
}
