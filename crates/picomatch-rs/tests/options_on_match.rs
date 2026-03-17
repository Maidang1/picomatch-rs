//! Migration of `test/options.onMatch.js` to Rust.
//!
//! # Notes on `onMatch` option
//!
//! The JS `onMatch` option is a callback invoked when a match is found. It allows
//! inspecting or modifying the match result. In Rust, we don't expose this callback.
//! Instead, we emulate its behavior by manually transforming strings at the test level.

mod support;

use picomatch_rs::{is_match, CompileOptions};
use std::collections::HashSet;
use support::default_compile_options;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Emulates the `format` function from JS: `str.replace(/^\.\//, '')`
fn format(s: &str) -> String {
    if s.starts_with("./") {
        s[2..].to_string()
    } else {
        s.to_string()
    }
}

/// Emulates the `onMatch` logic from JS:
/// strips leading `./` or `.\` if length > 2
fn on_match_format(s: &str) -> String {
    if s.len() > 2 && (s.starts_with("./") || s.starts_with(".\\")) {
        s[2..].to_string()
    } else {
        s.to_string()
    }
}

/// Emulates `isMatch(input, pattern, { format })`
fn is_match_with_format(input: &str, pattern: &str, opts: &CompileOptions) -> bool {
    is_match(&format(input), pattern, opts).unwrap_or(false)
}

/// Emulates `match(fixtures, pattern, options())` where options() includes format and onMatch.
/// Note: onMatch in the JS test adds the formatted/stripped string to the result set.
/// match() helper in support/match.js uses a Set to collect results.
fn match_with_on_match(fixtures: &[&str], pattern: &str, opts: &CompileOptions) -> Vec<String> {
    let mut result_set = HashSet::new();

    for &fixture in fixtures {
        // match() uses picomatch(pattern, opts)(fixture)
        // Inside picomatch, format is applied to the input string.
        let formatted_for_match = format(fixture);

        if is_match(&formatted_for_match, pattern, opts).unwrap_or(false) {
            // If it matches, onMatch is called.
            // In the JS test, onMatch receives 'output' which is the result of format(input).
            // Then onMatch applies its own slice(2) logic to that output.
            // Wait, let's look at JS code again.
            // line 116 in picomatch.js: const { isMatch, match, output } = picomatch.test(input, regex, options, { glob, posix });
            // picomatch.test (line 176) applies format.
            // So 'output' in onMatch is already formatted.
            let output = formatted_for_match;
            let final_output = on_match_format(&output);
            result_set.insert(final_output);
        }
    }

    let mut result: Vec<String> = result_set.into_iter().collect();
    result.sort();
    result
}

fn sorted(input: Vec<&str>) -> Vec<String> {
    let mut v: Vec<String> = input.into_iter().map(|s| s.to_string()).collect();
    v.sort();
    v
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn should_call_options_on_match_on_each_matching_string() {
    let fixtures = [
        "a",
        "./a",
        "b",
        "a/a",
        "./a/b",
        "a/c",
        "./a/x",
        "./a/a/a",
        "a/a/b",
        "./a/a/a/a",
        "./a/a/a/a/a",
        "x/y",
        "./z/z",
    ];
    let opts = default_compile_options();

    // isMatch assertions with { format }
    assert!(!is_match_with_format("./.a", "*.a", &opts));
    assert!(!is_match_with_format("./.a", "./*.a", &opts));
    assert!(!is_match_with_format("./.a", "a/**/z/*.md", &opts));
    assert!(!is_match_with_format(
        "./a/b/c/d/e/z/c.md",
        "./a/**/j/**/z/*.md",
        &opts
    ));
    assert!(!is_match_with_format(
        "./a/b/c/j/e/z/c.txt",
        "./a/**/j/**/z/*.md",
        &opts
    ));
    assert!(!is_match_with_format(
        "a/b/c/d/e/z/c.md",
        "./a/**/j/**/z/*.md",
        &opts
    ));

    assert!(is_match_with_format("./.a", "./.a", &opts));
    assert!(is_match_with_format("./a/b/c.md", "a/**/*.md", &opts));
    assert!(is_match_with_format(
        "./a/b/c/d/e/j/n/p/o/z/c.md",
        "./a/**/j/**/z/*.md",
        &opts
    ));
    assert!(is_match_with_format("./a/b/c/d/e/z/c.md", "**/*.md", &opts));
    assert!(is_match_with_format(
        "./a/b/c/d/e/z/c.md",
        "./a/**/z/*.md",
        &opts
    ));
    assert!(is_match_with_format(
        "./a/b/c/d/e/z/c.md",
        "a/**/z/*.md",
        &opts
    ));
    assert!(is_match_with_format(
        "./a/b/c/j/e/z/c.md",
        "./a/**/j/**/z/*.md",
        &opts
    ));
    assert!(is_match_with_format(
        "./a/b/c/j/e/z/c.md",
        "a/**/j/**/z/*.md",
        &opts
    ));
    assert!(is_match_with_format("./a/b/z/.a", "./a/**/z/.a", &opts));
    assert!(is_match_with_format("./a/b/z/.a", "a/**/z/.a", &opts));
    assert!(is_match_with_format(".a", "./.a", &opts));
    assert!(is_match_with_format("a/b/c.md", "./a/**/*.md", &opts));
    assert!(is_match_with_format("a/b/c.md", "a/**/*.md", &opts));
    assert!(is_match_with_format(
        "a/b/c/d/e/z/c.md",
        "a/**/z/*.md",
        &opts
    ));
    assert!(is_match_with_format(
        "a/b/c/j/e/z/c.md",
        "a/**/j/**/z/*.md",
        &opts
    ));

    // match() assertions with options() (format + onMatch)
    assert_eq!(
        match_with_on_match(&fixtures, "*", &opts),
        sorted(vec!["a", "b"])
    );
    assert_eq!(
        match_with_on_match(&fixtures, "**/a/**", &opts),
        sorted(vec![
            "a",
            "a/a",
            "a/c",
            "a/b",
            "a/x",
            "a/a/a",
            "a/a/b",
            "a/a/a/a",
            "a/a/a/a/a"
        ])
    );
    assert_eq!(
        match_with_on_match(&fixtures, "*/*", &opts),
        sorted(vec!["a/a", "a/b", "a/c", "a/x", "x/y", "z/z"])
    );
    assert_eq!(
        match_with_on_match(&fixtures, "*/*/*", &opts),
        sorted(vec!["a/a/a", "a/a/b"])
    );
    assert_eq!(
        match_with_on_match(&fixtures, "*/*/*/*", &opts),
        sorted(vec!["a/a/a/a"])
    );
    assert_eq!(
        match_with_on_match(&fixtures, "*/*/*/*/*", &opts),
        sorted(vec!["a/a/a/a/a"])
    );
    assert_eq!(
        match_with_on_match(&fixtures, "./*", &opts),
        sorted(vec!["a", "b"])
    );
    assert_eq!(
        match_with_on_match(&fixtures, "./**/a/**", &opts),
        sorted(vec![
            "a",
            "a/a",
            "a/b",
            "a/c",
            "a/x",
            "a/a/a",
            "a/a/b",
            "a/a/a/a",
            "a/a/a/a/a"
        ])
    );
    assert_eq!(
        match_with_on_match(&fixtures, "./a/*/a", &opts),
        sorted(vec!["a/a/a"])
    );
    assert_eq!(
        match_with_on_match(&fixtures, "a/*", &opts),
        sorted(vec!["a/a", "a/b", "a/c", "a/x"])
    );
    assert_eq!(
        match_with_on_match(&fixtures, "a/*/*", &opts),
        sorted(vec!["a/a/a", "a/a/b"])
    );
    assert_eq!(
        match_with_on_match(&fixtures, "a/*/*/*", &opts),
        sorted(vec!["a/a/a/a"])
    );
    assert_eq!(
        match_with_on_match(&fixtures, "a/*/*/*/*", &opts),
        sorted(vec!["a/a/a/a/a"])
    );
    assert_eq!(
        match_with_on_match(&fixtures, "a/*/a", &opts),
        sorted(vec!["a/a/a"])
    );
}
