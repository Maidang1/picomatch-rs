#![allow(dead_code)]

use picomatch_rs::{
    is_match, is_match_any, make_re, scan, CompileOptions, MatchError, RegexDescriptor,
    ScanOptions, ScanState,
};

pub fn assert_scan(input: &str, options: ScanOptions, expected: ScanState) {
    let actual = scan(input, &options);
    assert_eq!(
        actual, expected,
        "scan({input:?}) did not match expected state"
    );
}

pub fn assert_make_re(input: &str, options: CompileOptions, expected: RegexDescriptor) {
    let actual = make_re(input, &options, expected.state.is_some())
        .unwrap_or_else(|| panic!("make_re({input:?}) returned None"));
    assert_eq!(
        actual, expected,
        "make_re({input:?}) did not match expected descriptor"
    );
}

pub fn assert_is_match(input: &str, pattern: &str, options: CompileOptions, expected: bool) {
    let actual = is_match(input, pattern, &options)
        .unwrap_or_else(|err| panic!("is_match({input:?}, {pattern:?}) failed: {err:?}"));
    assert_eq!(
        actual, expected,
        "unexpected match result for {input:?} against {pattern:?}"
    );
}

pub fn assert_is_match_any(
    input: &str,
    patterns: &[&str],
    options: CompileOptions,
    expected: bool,
) {
    let actual = is_match_any(input, patterns.iter().copied(), &options)
        .unwrap_or_else(|err| panic!("is_match_any({input:?}, {patterns:?}) failed: {err:?}"));
    assert_eq!(
        actual, expected,
        "unexpected match result for {input:?} against {patterns:?}"
    );
}

pub fn assert_match_list(
    fixtures: &[&str],
    pattern: &str,
    options: CompileOptions,
    expected: &[&str],
) {
    let mut matches = Vec::new();

    for fixture in fixtures {
        match is_match(fixture, pattern, &options) {
            Ok(true) => matches.push(normalize_match(fixture, &options)),
            Ok(false) => {}
            Err(err) => panic!("is_match({fixture:?}, {pattern:?}) failed: {err:?}"),
        }
    }

    assert_eq!(
        matches, expected,
        "unexpected filtered matches for pattern {pattern:?}"
    );
}

pub fn default_compile_options() -> CompileOptions {
    CompileOptions::default()
}

pub fn default_scan_options() -> ScanOptions {
    ScanOptions::default()
}

fn normalize_match(input: &str, options: &CompileOptions) -> String {
    if options.windows {
        input.replace('\\', "/")
    } else {
        input.to_string()
    }
}

#[allow(dead_code)]
pub fn unwrap_unsupported(result: Result<bool, MatchError>) -> bool {
    result.unwrap_or(false)
}
