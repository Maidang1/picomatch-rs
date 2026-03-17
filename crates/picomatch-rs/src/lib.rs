pub mod compile;
pub mod constants;
pub mod matcher;
pub mod scan;
pub mod utils;

pub use compile::{make_re, parse, CompileOptions, ParseState, ParseToken, RegexDescriptor};
pub use matcher::{compile_matcher, is_match, is_match_any, MatchError, Matcher};
pub use scan::{scan, ScanOptions, ScanState, ScanToken};
