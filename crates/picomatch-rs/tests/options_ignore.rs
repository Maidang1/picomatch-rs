//! Migration of `test/options.ignore.js` to Rust.
//!
//! # Notes on `ignore` option
//!
//! The JS `ignore` option accepts one or more glob patterns that should be
//! excluded *after* the main pattern has matched. Rust `CompileOptions` does not
//! expose an `ignore` field — the feature is still JS-only (see process.md).
//!
//! Migration strategy: we emulate `ignore` at the test level by first matching
//! with `is_match(input, pattern)` and then checking whether any of the ignore
//! patterns also match. If an ignore pattern matches, the overall result is
//! `false`. This preserves the behavioural equivalence of every test case.

mod support;

use picomatch_rs::{is_match, CompileOptions};
use support::default_compile_options;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Emulates JS `isMatch(input, pattern, { ignore, ...opts })`.
///
/// Returns `true` only when `input` matches `pattern` AND does not match
/// any of the `ignore` patterns. All patterns are evaluated with the same
/// `opts`.
fn is_match_with_ignore(
    input: &str,
    pattern: &str,
    opts: &CompileOptions,
    ignore: &[&str],
) -> bool {
    let matched = is_match(input, pattern, opts).unwrap_or(false);
    if !matched {
        return false;
    }
    for ig in ignore {
        if is_match(input, ig, opts).unwrap_or(false) {
            return false;
        }
    }
    true
}

/// Emulates JS `match(fixtures, pattern, { ignore, ...opts })`.
///
/// Filters fixtures: keeps items that match `pattern` but do not match any
/// of the `ignore` patterns. Uses a `Set` for dedup (mirrors JS helper).
fn match_with_ignore(
    fixtures: &[&str],
    pattern: &str,
    opts: &CompileOptions,
    ignore: &[&str],
) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for fixture in fixtures {
        if seen.contains(*fixture) {
            continue;
        }
        seen.insert(*fixture);
        if is_match_with_ignore(fixture, pattern, opts, ignore) {
            result.push(fixture.to_string());
        }
    }
    result.sort();
    result
}

/// Emulates JS `match(fixtures, pattern, opts)` — no ignore, but still uses
/// negation patterns (like `!b/a`).
fn match_list(fixtures: &[&str], pattern: &str, opts: &CompileOptions) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for fixture in fixtures {
        if seen.contains(*fixture) {
            continue;
        }
        seen.insert(*fixture);
        if is_match(fixture, pattern, opts).unwrap_or(false) {
            result.push(fixture.to_string());
        }
    }
    result.sort();
    result
}

fn sorted(input: &[&str]) -> Vec<String> {
    let mut v: Vec<String> = input.iter().map(|s| s.to_string()).collect();
    v.sort();
    v
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Corresponds to: it('should not match ignored patterns', ...)
#[test]
fn should_not_match_ignored_patterns() {
    let opts = default_compile_options();

    // assert(isMatch('a+b/src/glimini.js', 'a+b/src/*.js', { ignore: ['**/f*'] }));
    assert!(is_match_with_ignore(
        "a+b/src/glimini.js",
        "a+b/src/*.js",
        &opts,
        &["**/f*"]
    ));

    // assert(!isMatch('a+b/src/glimini.js', 'a+b/src/*.js', { ignore: ['**/g*'] }));
    assert!(!is_match_with_ignore(
        "a+b/src/glimini.js",
        "a+b/src/*.js",
        &opts,
        &["**/g*"]
    ));

    // assert(isMatch('+b/src/glimini.md', '+b/src/*', { ignore: ['**/*.js'] }));
    assert!(is_match_with_ignore(
        "+b/src/glimini.md",
        "+b/src/*",
        &opts,
        &["**/*.js"]
    ));

    // assert(!isMatch('+b/src/glimini.js', '+b/src/*', { ignore: ['**/*.js'] }));
    assert!(!is_match_with_ignore(
        "+b/src/glimini.js",
        "+b/src/*",
        &opts,
        &["**/*.js"]
    ));
}

/// Corresponds to: it('should filter out ignored patterns', ...)
#[test]
fn should_filter_out_ignored_patterns() {
    let globs: &[&str] = &[
        ".a",
        ".a/a",
        ".a/a/a",
        ".a/a/a/a",
        "a",
        "a/.a",
        "a/a",
        "a/a/.a",
        "a/a/a",
        "a/a/a/a",
        "a/a/a/a/a",
        "a/a/b",
        "a/b",
        "a/b/c",
        "a/c",
        "a/x",
        "b",
        "b/b/b",
        "b/b/c",
        "c/c/c",
        "e/f/g",
        "h/i/a",
        "x/x/x",
        "x/y",
        "z/z",
        "z/z/z",
    ];

    let opts_strict = CompileOptions {
        strict_slashes: true,
        ..Default::default()
    };

    // match(globs, '*', { ignore: ['a/**'], strictSlashes: true }) => ['a', 'b']
    assert_eq!(
        match_with_ignore(globs, "*", &opts_strict, &["a/**"]),
        sorted(&["a", "b"]),
    );

    // match(globs, '*', { ignore: ['a/**'], strictSlashes: false }) => ['b']
    let opts_no_strict = CompileOptions {
        strict_slashes: false,
        ..Default::default()
    };
    assert_eq!(
        match_with_ignore(globs, "*", &opts_no_strict, &["a/**"]),
        sorted(&["b"]),
    );

    // match(globs, '*', { ignore: '**/a' }) => ['b']
    assert_eq!(
        match_with_ignore(globs, "*", &default_compile_options(), &["**/a"]),
        sorted(&["b"]),
    );

    // match(globs, '*/*', opts) => ['x/y', 'z/z']
    assert_eq!(
        match_with_ignore(globs, "*/*", &opts_strict, &["a/**"]),
        sorted(&["x/y", "z/z"]),
    );

    // match(globs, '*/*/*', opts) => ['b/b/b', 'b/b/c', 'c/c/c', 'e/f/g', 'h/i/a', 'x/x/x', 'z/z/z']
    assert_eq!(
        match_with_ignore(globs, "*/*/*", &opts_strict, &["a/**"]),
        sorted(&["b/b/b", "b/b/c", "c/c/c", "e/f/g", "h/i/a", "x/x/x", "z/z/z"]),
    );

    // match(globs, '*/*/*/*', opts) => []
    assert_eq!(
        match_with_ignore(globs, "*/*/*/*", &opts_strict, &["a/**"]),
        sorted(&[]),
    );

    // match(globs, '*/*/*/*/*', opts) => []
    assert_eq!(
        match_with_ignore(globs, "*/*/*/*/*", &opts_strict, &["a/**"]),
        sorted(&[]),
    );

    // match(globs, 'a/*', opts) => []
    assert_eq!(
        match_with_ignore(globs, "a/*", &opts_strict, &["a/**"]),
        sorted(&[]),
    );

    // match(globs, '**/*/x', opts) => ['x/x/x']
    assert_eq!(
        match_with_ignore(globs, "**/*/x", &opts_strict, &["a/**"]),
        sorted(&["x/x/x"]),
    );

    // match(globs, '**/*/[b-z]', opts) => ['b/b/b', 'b/b/c', 'c/c/c', 'e/f/g', 'x/x/x', 'x/y', 'z/z', 'z/z/z']
    assert_eq!(
        match_with_ignore(globs, "**/*/[b-z]", &opts_strict, &["a/**"]),
        sorted(&["b/b/b", "b/b/c", "c/c/c", "e/f/g", "x/x/x", "x/y", "z/z", "z/z/z"]),
    );
}

/// The dot-related assertions from: it('should filter out ignored patterns', ...)
#[test]
fn should_filter_with_dot_option() {
    let globs: &[&str] = &[
        ".a",
        ".a/a",
        ".a/a/a",
        ".a/a/a/a",
        "a",
        "a/.a",
        "a/a",
        "a/a/.a",
        "a/a/a",
        "a/a/a/a",
        "a/a/a/a/a",
        "a/a/b",
        "a/b",
        "a/b/c",
        "a/c",
        "a/x",
        "b",
        "b/b/b",
        "b/b/c",
        "c/c/c",
        "e/f/g",
        "h/i/a",
        "x/x/x",
        "x/y",
        "z/z",
        "z/z/z",
    ];

    // match(globs, '*', { ignore: '**/a', dot: true }) => ['.a', 'b']
    let dot_ignore_a = CompileOptions {
        dot: true,
        ..Default::default()
    };
    assert_eq!(
        match_with_ignore(globs, "*", &dot_ignore_a, &["**/a"]),
        sorted(&[".a", "b"]),
    );

    // dotOpts = { ignore: ['a/**'], strictSlashes: true, dot: true }
    let dot_opts = CompileOptions {
        strict_slashes: true,
        dot: true,
        ..Default::default()
    };

    // match(globs, '*', dotOpts) => ['.a', 'a', 'b']
    assert_eq!(
        match_with_ignore(globs, "*", &dot_opts, &["a/**"]),
        sorted(&[".a", "a", "b"]),
    );

    // match(globs, '*/*', dotOpts) => ['.a/a', 'x/y', 'z/z']
    assert_eq!(
        match_with_ignore(globs, "*/*", &dot_opts, &["a/**"]),
        sorted(&[".a/a", "x/y", "z/z"]),
    );

    // match(globs, '*/*/*', dotOpts) => ['.a/a/a', 'b/b/b', 'b/b/c', 'c/c/c', 'e/f/g', 'h/i/a', 'x/x/x', 'z/z/z']
    assert_eq!(
        match_with_ignore(globs, "*/*/*", &dot_opts, &["a/**"]),
        sorted(&[".a/a/a", "b/b/b", "b/b/c", "c/c/c", "e/f/g", "h/i/a", "x/x/x", "z/z/z"]),
    );

    // match(globs, '*/*/*/*', dotOpts) => ['.a/a/a/a']
    assert_eq!(
        match_with_ignore(globs, "*/*/*/*", &dot_opts, &["a/**"]),
        sorted(&[".a/a/a/a"]),
    );

    // match(globs, '*/*/*/*/*', dotOpts) => []
    assert_eq!(
        match_with_ignore(globs, "*/*/*/*/*", &dot_opts, &["a/**"]),
        sorted(&[]),
    );

    // match(globs, 'a/*', dotOpts) => []
    assert_eq!(
        match_with_ignore(globs, "a/*", &dot_opts, &["a/**"]),
        sorted(&[]),
    );

    // match(globs, '**/*/x', dotOpts) => ['x/x/x']
    assert_eq!(
        match_with_ignore(globs, "**/*/x", &dot_opts, &["a/**"]),
        sorted(&["x/x/x"]),
    );
}

/// Micromatch issue #79 and negation/ignore tests
#[test]
fn should_handle_micromatch_issue_79_and_negation_patterns() {
    let negations: &[&str] = &["a/a", "a/b", "a/c", "a/d", "a/e", "b/a", "b/b", "b/c"];
    let opts = default_compile_options();
    let opts_strict = CompileOptions {
        strict_slashes: true,
        ..Default::default()
    };

    // see https://github.com/jonschlinkert/micromatch/issues/79
    // match(['foo.js', 'a/foo.js'], '**/foo.js') => ['foo.js', 'a/foo.js']
    assert_eq!(
        match_list(&["foo.js", "a/foo.js"], "**/foo.js", &opts),
        sorted(&["foo.js", "a/foo.js"]),
    );
    // match(['foo.js', 'a/foo.js'], '**/foo.js', { dot: true }) => ['foo.js', 'a/foo.js']
    let dot_opts = CompileOptions {
        dot: true,
        ..Default::default()
    };
    assert_eq!(
        match_list(&["foo.js", "a/foo.js"], "**/foo.js", &dot_opts),
        sorted(&["foo.js", "a/foo.js"]),
    );

    // match(negations, '!b/a', opts) => ['b/b', 'b/c']
    assert_eq!(
        match_with_ignore(negations, "!b/a", &opts_strict, &["a/**"]),
        sorted(&["b/b", "b/c"]),
    );

    // match(negations, '!b/(a)', opts) => ['b/b', 'b/c']
    assert_eq!(
        match_with_ignore(negations, "!b/(a)", &opts_strict, &["a/**"]),
        sorted(&["b/b", "b/c"]),
    );

    // match(negations, '!(b/(a))', opts) => ['b/b', 'b/c']
    assert_eq!(
        match_with_ignore(negations, "!(b/(a))", &opts_strict, &["a/**"]),
        sorted(&["b/b", "b/c"]),
    );

    // match(negations, '!(b/a)', opts) => ['b/b', 'b/c']
    assert_eq!(
        match_with_ignore(negations, "!(b/a)", &opts_strict, &["a/**"]),
        sorted(&["b/b", "b/c"]),
    );

    // match(negations, '**') => negations ('nothing is ignored')
    assert_eq!(match_list(negations, "**", &opts), sorted(negations),);

    // match(negations, '**', { ignore: ['*/b', '*/a'] }) => ['a/c', 'a/d', 'a/e', 'b/c']
    assert_eq!(
        match_with_ignore(negations, "**", &opts, &["*/b", "*/a"]),
        sorted(&["a/c", "a/d", "a/e", "b/c"]),
    );

    // match(negations, '**', { ignore: ['**'] }) => []
    assert_eq!(
        match_with_ignore(negations, "**", &opts, &["**"]),
        sorted(&[]),
    );
}
