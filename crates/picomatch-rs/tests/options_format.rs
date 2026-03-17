//! Migration of `test/options.format.js` to Rust.
//!
//! # Notes on `format` option
//!
//! The JS `format` option accepts a callback `(str) => string` that transforms the input
//! string before matching. In this test file, the format function is:
//!   `str => str.replace(/\\/g, '/').replace(/^\.\//, '')`
//! i.e. it normalises backslashes to forward slashes and strips a leading `./`.
//!
//! Rust does not expose a `format` callback in `CompileOptions`. Instead, we apply the
//! same transformation to the input manually before calling `is_match`, which preserves
//! the behavioural equivalence of every test case.

mod support;

use picomatch_rs::{is_match, CompileOptions};
use support::assert_is_match;

/// Applies the same transformation as the JS `format` option used in this test:
///   str => str.replace(/\\/g, '/').replace(/^\.\//, '')
fn format(s: &str) -> String {
    let s = s.replace('\\', "/");
    s.strip_prefix("./").unwrap_or(&s).to_string()
}

fn strict_slashes_opts() -> CompileOptions {
    CompileOptions {
        strict_slashes: true,
        ..Default::default()
    }
}

/// Filters a fixture list through `is_match`, applying `format` to each input.
/// Uses a set to deduplicate (mirrors the JS `match` helper which uses a `Set`).
/// Returns the deduplicated formatted strings that matched, sorted.
fn match_with_format(fixtures: &[&str], pattern: &str, opts: &CompileOptions) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for fixture in fixtures {
        let formatted = format(fixture);
        if seen.contains(&formatted) {
            continue;
        }
        match is_match(&formatted, pattern, opts) {
            Ok(true) => {
                seen.insert(formatted.clone());
                result.push(formatted);
            }
            Ok(false) => {
                seen.insert(formatted);
            }
            Err(err) => {
                panic!("is_match({fixture:?} -> {formatted:?}, {pattern:?}) failed: {err:?}")
            }
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

/// Corresponds to:
///   it('should match the string returned by options.format', ...)
///
/// Original JS opts: `{ format: str => str.replace(/\\/g, '/').replace(/^\.\//, ''), strictSlashes: true }`
///
/// Note: `format` is a JS-only callback. We apply the same transformation
/// to the input manually in Rust to achieve equivalent test semantics.
#[test]
fn should_match_the_string_returned_by_options_format() {
    let opts = strict_slashes_opts();

    // --- isMatch assertions (negative cases) ---
    // assert(!isMatch('./.a', '*.a', opts));
    assert_is_match(&format("./.a"), "*.a", opts.clone(), false);
    // assert(!isMatch('./.a', './*.a', opts));
    assert_is_match(&format("./.a"), "./*.a", opts.clone(), false);
    // assert(!isMatch('./.a', 'a/**/z/*.md', opts));
    assert_is_match(&format("./.a"), "a/**/z/*.md", opts.clone(), false);
    // assert(!isMatch('./a/b/c/d/e/z/c.md', './a/**/j/**/z/*.md', opts));
    assert_is_match(
        &format("./a/b/c/d/e/z/c.md"),
        "./a/**/j/**/z/*.md",
        opts.clone(),
        false,
    );
    // assert(!isMatch('./a/b/c/j/e/z/c.txt', './a/**/j/**/z/*.md', opts));
    assert_is_match(
        &format("./a/b/c/j/e/z/c.txt"),
        "./a/**/j/**/z/*.md",
        opts.clone(),
        false,
    );
    // assert(!isMatch('a/b/c/d/e/z/c.md', './a/**/j/**/z/*.md', opts));
    assert_is_match(
        &format("a/b/c/d/e/z/c.md"),
        "./a/**/j/**/z/*.md",
        opts.clone(),
        false,
    );

    // --- isMatch assertions (positive cases) ---
    // assert(isMatch('./.a', './.a', opts));
    assert_is_match(&format("./.a"), "./.a", opts.clone(), true);
    // assert(isMatch('./a/b/c.md', 'a/**/*.md', opts));
    assert_is_match(&format("./a/b/c.md"), "a/**/*.md", opts.clone(), true);
    // assert(isMatch('./a/b/c/d/e/j/n/p/o/z/c.md', './a/**/j/**/z/*.md', opts));
    assert_is_match(
        &format("./a/b/c/d/e/j/n/p/o/z/c.md"),
        "./a/**/j/**/z/*.md",
        opts.clone(),
        true,
    );
    // assert(isMatch('./a/b/c/d/e/z/c.md', '**/*.md', opts));
    assert_is_match(&format("./a/b/c/d/e/z/c.md"), "**/*.md", opts.clone(), true);
    // assert(isMatch('./a/b/c/d/e/z/c.md', './a/**/z/*.md', opts));
    assert_is_match(
        &format("./a/b/c/d/e/z/c.md"),
        "./a/**/z/*.md",
        opts.clone(),
        true,
    );
    // assert(isMatch('./a/b/c/d/e/z/c.md', 'a/**/z/*.md', opts));
    assert_is_match(
        &format("./a/b/c/d/e/z/c.md"),
        "a/**/z/*.md",
        opts.clone(),
        true,
    );
    // assert(isMatch('./a/b/c/j/e/z/c.md', './a/**/j/**/z/*.md', opts));
    assert_is_match(
        &format("./a/b/c/j/e/z/c.md"),
        "./a/**/j/**/z/*.md",
        opts.clone(),
        true,
    );
    // assert(isMatch('./a/b/c/j/e/z/c.md', 'a/**/j/**/z/*.md', opts));
    assert_is_match(
        &format("./a/b/c/j/e/z/c.md"),
        "a/**/j/**/z/*.md",
        opts.clone(),
        true,
    );
    // assert(isMatch('./a/b/z/.a', './a/**/z/.a', opts));
    assert_is_match(&format("./a/b/z/.a"), "./a/**/z/.a", opts.clone(), true);
    // assert(isMatch('./a/b/z/.a', 'a/**/z/.a', opts));
    assert_is_match(&format("./a/b/z/.a"), "a/**/z/.a", opts.clone(), true);
    // assert(isMatch('.a', './.a', opts));
    assert_is_match(&format(".a"), "./.a", opts.clone(), true);
    // assert(isMatch('a/b/c.md', './a/**/*.md', opts));
    assert_is_match(&format("a/b/c.md"), "./a/**/*.md", opts.clone(), true);
    // assert(isMatch('a/b/c.md', 'a/**/*.md', opts));
    assert_is_match(&format("a/b/c.md"), "a/**/*.md", opts.clone(), true);
    // assert(isMatch('a/b/c/d/e/z/c.md', 'a/**/z/*.md', opts));
    assert_is_match(
        &format("a/b/c/d/e/z/c.md"),
        "a/**/z/*.md",
        opts.clone(),
        true,
    );
    // assert(isMatch('a/b/c/j/e/z/c.md', 'a/**/j/**/z/*.md', opts));
    assert_is_match(
        &format("a/b/c/j/e/z/c.md"),
        "a/**/j/**/z/*.md",
        opts.clone(),
        true,
    );
    // assert(isMatch('./a', '*', opts));
    assert_is_match(&format("./a"), "*", opts.clone(), true);

    // assert(isMatch('./foo/bar.js', '**/foo/**', opts));
    assert_is_match(&format("./foo/bar.js"), "**/foo/**", opts.clone(), true);
    // assert(isMatch('./foo/bar.js', './**/foo/**', opts));
    assert_is_match(&format("./foo/bar.js"), "./**/foo/**", opts.clone(), true);

    // assert(isMatch('.\\foo\\bar.js', '**/foo/**', { ...opts, windows: false }));
    // NOTE: With windows: false, backslashes are treated as escape chars, not separators.
    // After format(), '.\\foo\\bar.js' becomes './foo/bar.js' -> 'foo/bar.js'.
    let non_windows_opts = CompileOptions {
        strict_slashes: true,
        windows: false,
        ..Default::default()
    };
    assert_is_match(
        &format(".\\foo\\bar.js"),
        "**/foo/**",
        non_windows_opts,
        true,
    );

    // assert(isMatch('.\\foo\\bar.js', './**/foo/**', opts));
    assert_is_match(&format(".\\foo\\bar.js"), "./**/foo/**", opts.clone(), true);
}

#[test]
fn should_filter_fixtures_with_format() {
    let opts = strict_slashes_opts();
    let fixtures: &[&str] = &[
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

    // equal(match(fixtures, '*', opts), ['a', 'b']);
    assert_eq!(match_with_format(fixtures, "*", &opts), sorted(&["a", "b"]),);

    // equal(match(fixtures, '**/a/**', opts), ['a/a', 'a/c', 'a/b', 'a/x', 'a/a/a', 'a/a/b', 'a/a/a/a', 'a/a/a/a/a']);
    assert_eq!(
        match_with_format(fixtures, "**/a/**", &opts),
        sorted(&[
            "a/a",
            "a/c",
            "a/b",
            "a/x",
            "a/a/a",
            "a/a/b",
            "a/a/a/a",
            "a/a/a/a/a"
        ]),
    );

    // equal(match(fixtures, '*/*', opts), ['a/a', 'a/b', 'a/c', 'a/x', 'x/y', 'z/z']);
    assert_eq!(
        match_with_format(fixtures, "*/*", &opts),
        sorted(&["a/a", "a/b", "a/c", "a/x", "x/y", "z/z"]),
    );

    // equal(match(fixtures, '*/*/*', opts), ['a/a/a', 'a/a/b']);
    assert_eq!(
        match_with_format(fixtures, "*/*/*", &opts),
        sorted(&["a/a/a", "a/a/b"]),
    );

    // equal(match(fixtures, '*/*/*/*', opts), ['a/a/a/a']);
    assert_eq!(
        match_with_format(fixtures, "*/*/*/*", &opts),
        sorted(&["a/a/a/a"]),
    );

    // equal(match(fixtures, '*/*/*/*/*', opts), ['a/a/a/a/a']);
    assert_eq!(
        match_with_format(fixtures, "*/*/*/*/*", &opts),
        sorted(&["a/a/a/a/a"]),
    );

    // Duplicated in JS — included for completeness
    // equal(match(fixtures, '*', opts), ['a', 'b']);
    assert_eq!(match_with_format(fixtures, "*", &opts), sorted(&["a", "b"]),);

    // equal(match(fixtures, '**/a/**', opts), ['a/a', 'a/c', 'a/b', 'a/x', 'a/a/a', 'a/a/b', 'a/a/a/a', 'a/a/a/a/a']);
    assert_eq!(
        match_with_format(fixtures, "**/a/**", &opts),
        sorted(&[
            "a/a",
            "a/c",
            "a/b",
            "a/x",
            "a/a/a",
            "a/a/b",
            "a/a/a/a",
            "a/a/a/a/a"
        ]),
    );

    // equal(match(fixtures, 'a/*/a', opts), ['a/a/a']);
    assert_eq!(
        match_with_format(fixtures, "a/*/a", &opts),
        sorted(&["a/a/a"]),
    );

    // equal(match(fixtures, 'a/*', opts), ['a/a', 'a/b', 'a/c', 'a/x']);
    assert_eq!(
        match_with_format(fixtures, "a/*", &opts),
        sorted(&["a/a", "a/b", "a/c", "a/x"]),
    );

    // equal(match(fixtures, 'a/*/*', opts), ['a/a/a', 'a/a/b']);
    assert_eq!(
        match_with_format(fixtures, "a/*/*", &opts),
        sorted(&["a/a/a", "a/a/b"]),
    );

    // equal(match(fixtures, 'a/*/*/*', opts), ['a/a/a/a']);
    assert_eq!(
        match_with_format(fixtures, "a/*/*/*", &opts),
        sorted(&["a/a/a/a"]),
    );

    // equal(match(fixtures, 'a/*/*/*/*', opts), ['a/a/a/a/a']);
    assert_eq!(
        match_with_format(fixtures, "a/*/*/*/*", &opts),
        sorted(&["a/a/a/a/a"]),
    );

    // equal(match(fixtures, 'a/*/a', opts), ['a/a/a']);  (duplicate)
    assert_eq!(
        match_with_format(fixtures, "a/*/a", &opts),
        sorted(&["a/a/a"]),
    );
}
