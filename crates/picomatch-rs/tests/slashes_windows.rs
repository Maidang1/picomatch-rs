mod support;

use picomatch_rs::CompileOptions;

use support::{assert_is_match, default_compile_options};

fn windows_options() -> CompileOptions {
    let mut opts = CompileOptions::default();
    opts.windows = true;
    opts
}

// JS: makeRe('**/path/**', { windows: true }).test('C:\\Users\\...')
// Equivalent to is_match since makeRe is used internally.
#[test]
fn should_match_absolute_windows_paths_with_regex_from_make_re() {
    assert_is_match(
        "C:\\Users\\user\\Projects\\project\\path\\image.jpg",
        "**/path/**",
        windows_options(),
        true,
    );
}

#[test]
fn should_match_windows_path_separators_with_string_literal() {
    assert_is_match("a\\a", "(a/b)", windows_options(), false);
    assert_is_match("a\\b", "(a/b)", windows_options(), true);
    assert_is_match("a\\c", "(a/b)", windows_options(), false);
    assert_is_match("b\\a", "(a/b)", windows_options(), false);
    assert_is_match("b\\b", "(a/b)", windows_options(), false);
    assert_is_match("b\\c", "(a/b)", windows_options(), false);

    assert_is_match("a\\a", "a/b", windows_options(), false);
    assert_is_match("a\\b", "a/b", windows_options(), true);
    assert_is_match("a\\c", "a/b", windows_options(), false);
    assert_is_match("b\\a", "a/b", windows_options(), false);
    assert_is_match("b\\b", "a/b", windows_options(), false);
    assert_is_match("b\\c", "a/b", windows_options(), false);
}

#[test]
fn should_not_match_literal_backslashes_with_literal_forward_slashes_when_windows_disabled() {
    let opts = default_compile_options();
    assert_is_match("a\\a", "a\\b", opts.clone(), false);
    assert_is_match("a\\b", "a\\b", opts.clone(), true);
    assert_is_match("a\\c", "a\\b", opts.clone(), false);
    assert_is_match("b\\a", "a\\b", opts.clone(), false);
    assert_is_match("b\\b", "a\\b", opts.clone(), false);
    assert_is_match("b\\c", "a\\b", opts.clone(), false);

    assert_is_match("a\\a", "a/b", opts.clone(), false);
    assert_is_match("a\\b", "a/b", opts.clone(), false);
    assert_is_match("a\\c", "a/b", opts.clone(), false);
    assert_is_match("b\\a", "a/b", opts.clone(), false);
    assert_is_match("b\\b", "a/b", opts.clone(), false);
    assert_is_match("b\\c", "a/b", opts.clone(), false);
}

#[test]
fn should_match_an_array_of_literal_strings() {
    assert_is_match("a\\a", "(a/b)", windows_options(), false);
    assert_is_match("a\\b", "(a/b)", windows_options(), true);
    assert_is_match("a\\c", "(a/b)", windows_options(), false);
    assert_is_match("b\\a", "(a/b)", windows_options(), false);
    assert_is_match("b\\b", "(a/b)", windows_options(), false);
    assert_is_match("b\\c", "(a/b)", windows_options(), false);
}

#[test]
fn should_not_match_backslashes_with_forward_slashes_when_windows_disabled() {
    let opts = default_compile_options();
    assert_is_match("a\\a", "a/(a|c)", opts.clone(), false);
    assert_is_match("a\\b", "a/(a|c)", opts.clone(), false);
    assert_is_match("a\\c", "a/(a|c)", opts.clone(), false);
    assert_is_match("a\\a", "a/(a|b|c)", opts.clone(), false);
    assert_is_match("a\\b", "a/(a|b|c)", opts.clone(), false);
    assert_is_match("a\\c", "a/(a|b|c)", opts.clone(), false);
    assert_is_match("a\\a", "(a\\b)", opts.clone(), false);
    // JS: isMatch('a\\b', '(a\\\\b)', { windows: false }) => true
    // Pattern string is (a\\b) — backslash escaped in the glob pattern
    assert_is_match("a\\b", "(a\\\\b)", opts.clone(), true);
    assert_is_match("a\\c", "(a\\b)", opts.clone(), false);
    assert_is_match("b\\a", "(a\\b)", opts.clone(), false);
    assert_is_match("b\\b", "(a\\b)", opts.clone(), false);
    assert_is_match("b\\c", "(a\\b)", opts.clone(), false);
    assert_is_match("a\\a", "(a/b)", opts.clone(), false);
    assert_is_match("a\\b", "(a/b)", opts.clone(), false);
    assert_is_match("a\\c", "(a/b)", opts.clone(), false);
    assert_is_match("b\\a", "(a/b)", opts.clone(), false);
    assert_is_match("b\\b", "(a/b)", opts.clone(), false);
    assert_is_match("b\\c", "(a/b)", opts.clone(), false);

    assert_is_match("a\\a", "a/c", opts.clone(), false);
    assert_is_match("a\\b", "a/c", opts.clone(), false);
    assert_is_match("a\\c", "a/c", opts.clone(), false);
    assert_is_match("b\\a", "a/c", opts.clone(), false);
    assert_is_match("b\\b", "a/c", opts.clone(), false);
    assert_is_match("b\\c", "a/c", opts.clone(), false);
}

#[test]
fn should_match_backslashes_with_regex_logical_or() {
    assert_is_match("a\\a", "a/(a|c)", windows_options(), true);
    assert_is_match("a\\b", "a/(a|c)", windows_options(), false);
    assert_is_match("a\\c", "a/(a|c)", windows_options(), true);

    assert_is_match("a\\a", "a/(a|b|c)", windows_options(), true);
    assert_is_match("a\\b", "a/(a|b|c)", windows_options(), true);
    assert_is_match("a\\c", "a/(a|b|c)", windows_options(), true);
}

#[test]
fn should_support_matching_backslashes_with_regex_ranges() {
    assert_is_match("a\\a", "a/[b-c]", windows_options(), false);
    assert_is_match("a\\b", "a/[b-c]", windows_options(), true);
    assert_is_match("a\\c", "a/[b-c]", windows_options(), true);
    assert_is_match("a\\x\\y", "a/[b-c]", windows_options(), false);
    assert_is_match("a\\x", "a/[b-c]", windows_options(), false);

    assert_is_match("a\\a", "a/[a-z]", windows_options(), true);
    assert_is_match("a\\b", "a/[a-z]", windows_options(), true);
    assert_is_match("a\\c", "a/[a-z]", windows_options(), true);
    assert_is_match("a\\x\\y", "a/[a-z]", windows_options(), false);
    assert_is_match("a\\x\\y", "a/[a-z]/y", windows_options(), true);
    assert_is_match("a\\x", "a/[a-z]", windows_options(), true);

    let opts = default_compile_options();
    assert_is_match("a\\a", "a/[b-c]", opts.clone(), false);
    assert_is_match("a\\b", "a/[b-c]", opts.clone(), false);
    assert_is_match("a\\c", "a/[b-c]", opts.clone(), false);
    assert_is_match("a\\x\\y", "a/[b-c]", opts.clone(), false);
    assert_is_match("a\\x", "a/[b-c]", opts.clone(), false);

    assert_is_match("a\\a", "a/[a-z]", opts.clone(), false);
    assert_is_match("a\\b", "a/[a-z]", opts.clone(), false);
    assert_is_match("a\\c", "a/[a-z]", opts.clone(), false);
    assert_is_match("a\\x\\y", "a/[a-z]", opts.clone(), false);
    assert_is_match("a\\x", "a/[a-z]", opts.clone(), false);
}

#[test]
fn should_not_match_slashes_with_single_stars() {
    // windows=true, pattern "*"
    assert_is_match("a", "*", windows_options(), true);
    assert_is_match("b", "*", windows_options(), true);
    assert_is_match("a\\a", "*", windows_options(), false);
    assert_is_match("a\\b", "*", windows_options(), false);
    assert_is_match("a\\c", "*", windows_options(), false);
    assert_is_match("a\\x", "*", windows_options(), false);
    assert_is_match("a\\a\\a", "*", windows_options(), false);
    assert_is_match("a\\a\\b", "*", windows_options(), false);
    assert_is_match("a\\a\\a\\a", "*", windows_options(), false);
    assert_is_match("a\\a\\a\\a\\a", "*", windows_options(), false);
    assert_is_match("x\\y", "*", windows_options(), false);
    assert_is_match("z\\z", "*", windows_options(), false);

    // windows=true, pattern "*/*"
    assert_is_match("a", "*/*", windows_options(), false);
    assert_is_match("b", "*/*", windows_options(), false);
    assert_is_match("a\\a", "*/*", windows_options(), true);
    assert_is_match("a\\b", "*/*", windows_options(), true);
    assert_is_match("a\\c", "*/*", windows_options(), true);
    assert_is_match("a\\x", "*/*", windows_options(), true);
    assert_is_match("a\\a\\a", "*/*", windows_options(), false);
    assert_is_match("a\\a\\b", "*/*", windows_options(), false);
    assert_is_match("a\\a\\a\\a", "*/*", windows_options(), false);
    assert_is_match("a\\a\\a\\a\\a", "*/*", windows_options(), false);
    assert_is_match("x\\y", "*/*", windows_options(), true);
    assert_is_match("z\\z", "*/*", windows_options(), true);

    // windows=true, pattern "*/*/*"
    assert_is_match("a", "*/*/*", windows_options(), false);
    assert_is_match("b", "*/*/*", windows_options(), false);
    assert_is_match("a\\a", "*/*/*", windows_options(), false);
    assert_is_match("a\\b", "*/*/*", windows_options(), false);
    assert_is_match("a\\c", "*/*/*", windows_options(), false);
    assert_is_match("a\\x", "*/*/*", windows_options(), false);
    assert_is_match("a\\a\\a", "*/*/*", windows_options(), true);
    assert_is_match("a\\a\\b", "*/*/*", windows_options(), true);
    assert_is_match("a\\a\\a\\a", "*/*/*", windows_options(), false);
    assert_is_match("a\\a\\a\\a\\a", "*/*/*", windows_options(), false);
    assert_is_match("x\\y", "*/*/*", windows_options(), false);
    assert_is_match("z\\z", "*/*/*", windows_options(), false);

    // windows=true, pattern "*/*/*/*"
    assert_is_match("a", "*/*/*/*", windows_options(), false);
    assert_is_match("b", "*/*/*/*", windows_options(), false);
    assert_is_match("a\\a", "*/*/*/*", windows_options(), false);
    assert_is_match("a\\b", "*/*/*/*", windows_options(), false);
    assert_is_match("a\\c", "*/*/*/*", windows_options(), false);
    assert_is_match("a\\x", "*/*/*/*", windows_options(), false);
    assert_is_match("a\\a\\a", "*/*/*/*", windows_options(), false);
    assert_is_match("a\\a\\b", "*/*/*/*", windows_options(), false);
    assert_is_match("a\\a\\a\\a", "*/*/*/*", windows_options(), true);
    assert_is_match("a\\a\\a\\a\\a", "*/*/*/*", windows_options(), false);
    assert_is_match("x\\y", "*/*/*/*", windows_options(), false);
    assert_is_match("z\\z", "*/*/*/*", windows_options(), false);

    // windows=true, pattern "*/*/*/*/*"
    assert_is_match("a", "*/*/*/*/*", windows_options(), false);
    assert_is_match("b", "*/*/*/*/*", windows_options(), false);
    assert_is_match("a\\a", "*/*/*/*/*", windows_options(), false);
    assert_is_match("a\\b", "*/*/*/*/*", windows_options(), false);
    assert_is_match("a\\c", "*/*/*/*/*", windows_options(), false);
    assert_is_match("a\\x", "*/*/*/*/*", windows_options(), false);
    assert_is_match("a\\a\\a", "*/*/*/*/*", windows_options(), false);
    assert_is_match("a\\a\\b", "*/*/*/*/*", windows_options(), false);
    assert_is_match("a\\a\\a\\a", "*/*/*/*/*", windows_options(), false);
    assert_is_match("a\\a\\a\\a\\a", "*/*/*/*/*", windows_options(), true);
    assert_is_match("x\\y", "*/*/*/*/*", windows_options(), false);
    assert_is_match("z\\z", "*/*/*/*/*", windows_options(), false);

    // windows=true, pattern "a/*"
    assert_is_match("a", "a/*", windows_options(), false);
    assert_is_match("b", "a/*", windows_options(), false);
    assert_is_match("a\\a", "a/*", windows_options(), true);
    assert_is_match("a\\b", "a/*", windows_options(), true);
    assert_is_match("a\\c", "a/*", windows_options(), true);
    assert_is_match("a\\x", "a/*", windows_options(), true);
    assert_is_match("a\\a\\a", "a/*", windows_options(), false);
    assert_is_match("a\\a\\b", "a/*", windows_options(), false);
    assert_is_match("a\\a\\a\\a", "a/*", windows_options(), false);
    assert_is_match("a\\a\\a\\a\\a", "a/*", windows_options(), false);
    assert_is_match("x\\y", "a/*", windows_options(), false);
    assert_is_match("z\\z", "a/*", windows_options(), false);

    // windows=true, pattern "a/*/*"
    assert_is_match("a", "a/*/*", windows_options(), false);
    assert_is_match("b", "a/*/*", windows_options(), false);
    assert_is_match("a\\a", "a/*/*", windows_options(), false);
    assert_is_match("a\\b", "a/*/*", windows_options(), false);
    assert_is_match("a\\c", "a/*/*", windows_options(), false);
    assert_is_match("a\\x", "a/*/*", windows_options(), false);
    assert_is_match("a\\a\\a", "a/*/*", windows_options(), true);
    assert_is_match("a\\a\\b", "a/*/*", windows_options(), true);
    assert_is_match("a\\a\\a\\a", "a/*/*", windows_options(), false);
    assert_is_match("a\\a\\a\\a\\a", "a/*/*", windows_options(), false);
    assert_is_match("x\\y", "a/*/*", windows_options(), false);
    assert_is_match("z\\z", "a/*/*", windows_options(), false);

    // windows=true, pattern "a/*/*/*"
    assert_is_match("a", "a/*/*/*", windows_options(), false);
    assert_is_match("b", "a/*/*/*", windows_options(), false);
    assert_is_match("a\\a", "a/*/*/*", windows_options(), false);
    assert_is_match("a\\b", "a/*/*/*", windows_options(), false);
    assert_is_match("a\\c", "a/*/*/*", windows_options(), false);
    assert_is_match("a\\x", "a/*/*/*", windows_options(), false);
    assert_is_match("a\\a\\a", "a/*/*/*", windows_options(), false);
    assert_is_match("a\\a\\b", "a/*/*/*", windows_options(), false);
    assert_is_match("a\\a\\a\\a", "a/*/*/*", windows_options(), true);
    assert_is_match("a\\a\\a\\a\\a", "a/*/*/*", windows_options(), false);
    assert_is_match("x\\y", "a/*/*/*", windows_options(), false);
    assert_is_match("z\\z", "a/*/*/*", windows_options(), false);

    // windows=true, pattern "a/*/*/*/*"
    assert_is_match("a", "a/*/*/*/*", windows_options(), false);
    assert_is_match("b", "a/*/*/*/*", windows_options(), false);
    assert_is_match("a\\a", "a/*/*/*/*", windows_options(), false);
    assert_is_match("a\\b", "a/*/*/*/*", windows_options(), false);
    assert_is_match("a\\c", "a/*/*/*/*", windows_options(), false);
    assert_is_match("a\\x", "a/*/*/*/*", windows_options(), false);
    assert_is_match("a\\a\\a", "a/*/*/*/*", windows_options(), false);
    assert_is_match("a\\a\\b", "a/*/*/*/*", windows_options(), false);
    assert_is_match("a\\a\\a\\a", "a/*/*/*/*", windows_options(), false);
    assert_is_match("a\\a\\a\\a\\a", "a/*/*/*/*", windows_options(), true);
    assert_is_match("x\\y", "a/*/*/*/*", windows_options(), false);
    assert_is_match("z\\z", "a/*/*/*/*", windows_options(), false);

    // windows=true, pattern "a/*/a"
    assert_is_match("a", "a/*/a", windows_options(), false);
    assert_is_match("b", "a/*/a", windows_options(), false);
    assert_is_match("a\\a", "a/*/a", windows_options(), false);
    assert_is_match("a\\b", "a/*/a", windows_options(), false);
    assert_is_match("a\\c", "a/*/a", windows_options(), false);
    assert_is_match("a\\x", "a/*/a", windows_options(), false);
    assert_is_match("a\\a\\a", "a/*/a", windows_options(), true);
    assert_is_match("a\\a\\b", "a/*/a", windows_options(), false);
    assert_is_match("a\\a\\a\\a", "a/*/a", windows_options(), false);
    assert_is_match("a\\a\\a\\a\\a", "a/*/a", windows_options(), false);
    assert_is_match("x\\y", "a/*/a", windows_options(), false);
    assert_is_match("z\\z", "a/*/a", windows_options(), false);

    // windows=true, pattern "a/*/b"
    assert_is_match("a", "a/*/b", windows_options(), false);
    assert_is_match("b", "a/*/b", windows_options(), false);
    assert_is_match("a\\a", "a/*/b", windows_options(), false);
    assert_is_match("a\\b", "a/*/b", windows_options(), false);
    assert_is_match("a\\c", "a/*/b", windows_options(), false);
    assert_is_match("a\\x", "a/*/b", windows_options(), false);
    assert_is_match("a\\a\\a", "a/*/b", windows_options(), false);
    assert_is_match("a\\a\\b", "a/*/b", windows_options(), true);
    assert_is_match("a\\a\\a\\a", "a/*/b", windows_options(), false);
    assert_is_match("a\\a\\a\\a\\a", "a/*/b", windows_options(), false);
    assert_is_match("x\\y", "a/*/b", windows_options(), false);
    assert_is_match("z\\z", "a/*/b", windows_options(), false);

    // windows=false sections — backslashes are NOT path separators
    let opts = default_compile_options();
    for pattern in &["*/*", "*/*/*", "*/*/*/*", "*/*/*/*/*"] {
        assert_is_match("a", pattern, opts.clone(), false);
        assert_is_match("b", pattern, opts.clone(), false);
        assert_is_match("a\\a", pattern, opts.clone(), false);
        assert_is_match("a\\b", pattern, opts.clone(), false);
        assert_is_match("a\\c", pattern, opts.clone(), false);
        assert_is_match("a\\x", pattern, opts.clone(), false);
        assert_is_match("a\\a\\a", pattern, opts.clone(), false);
        assert_is_match("a\\a\\b", pattern, opts.clone(), false);
        assert_is_match("a\\a\\a\\a", pattern, opts.clone(), false);
        assert_is_match("a\\a\\a\\a\\a", pattern, opts.clone(), false);
        assert_is_match("x\\y", pattern, opts.clone(), false);
        assert_is_match("z\\z", pattern, opts.clone(), false);
    }

    for pattern in &["a/*", "a/*/*", "a/*/*/*", "a/*/*/*/*", "a/*/a", "a/*/b"] {
        assert_is_match("a", pattern, opts.clone(), false);
        assert_is_match("b", pattern, opts.clone(), false);
        assert_is_match("a\\a", pattern, opts.clone(), false);
        assert_is_match("a\\b", pattern, opts.clone(), false);
        assert_is_match("a\\c", pattern, opts.clone(), false);
        assert_is_match("a\\x", pattern, opts.clone(), false);
        assert_is_match("a\\a\\a", pattern, opts.clone(), false);
        assert_is_match("a\\a\\b", pattern, opts.clone(), false);
        assert_is_match("a\\a\\a\\a", pattern, opts.clone(), false);
        assert_is_match("a\\a\\a\\a\\a", pattern, opts.clone(), false);
        assert_is_match("x\\y", pattern, opts.clone(), false);
        assert_is_match("z\\z", pattern, opts.clone(), false);
    }
}

#[test]
fn should_support_globstars() {
    assert_is_match("a\\a", "a/**", windows_options(), true);
    assert_is_match("a\\b", "a/**", windows_options(), true);
    assert_is_match("a\\c", "a/**", windows_options(), true);
    assert_is_match("a\\x", "a/**", windows_options(), true);
    assert_is_match("a\\x\\y", "a/**", windows_options(), true);
    assert_is_match("a\\x\\y\\z", "a/**", windows_options(), true);

    assert_is_match("a\\a", "a/**/*", windows_options(), true);
    assert_is_match("a\\b", "a/**/*", windows_options(), true);
    assert_is_match("a\\c", "a/**/*", windows_options(), true);
    assert_is_match("a\\x", "a/**/*", windows_options(), true);
    assert_is_match("a\\x\\y", "a/**/*", windows_options(), true);
    assert_is_match("a\\x\\y\\z", "a/**/*", windows_options(), true);

    assert_is_match("a\\a", "a/**/**/*", windows_options(), true);
    assert_is_match("a\\b", "a/**/**/*", windows_options(), true);
    assert_is_match("a\\c", "a/**/**/*", windows_options(), true);
    assert_is_match("a\\x", "a/**/**/*", windows_options(), true);
    assert_is_match("a\\x\\y", "a/**/**/*", windows_options(), true);
    assert_is_match("a\\x\\y\\z", "a/**/**/*", windows_options(), true);
}

#[test]
fn should_not_match_backslashes_with_globstars_when_disabled() {
    let opts = default_compile_options();
    assert_is_match("a\\a", "a/**", opts.clone(), false);
    assert_is_match("a\\b", "a/**", opts.clone(), false);
    assert_is_match("a\\c", "a/**", opts.clone(), false);
    assert_is_match("a\\x", "a/**", opts.clone(), false);
    assert_is_match("a\\x\\y", "a/**", opts.clone(), false);
    assert_is_match("a\\x\\y\\z", "a/**", opts.clone(), false);

    assert_is_match("a\\a", "a/**/*", opts.clone(), false);
    assert_is_match("a\\b", "a/**/*", opts.clone(), false);
    assert_is_match("a\\c", "a/**/*", opts.clone(), false);
    assert_is_match("a\\x", "a/**/*", opts.clone(), false);
    assert_is_match("a\\x\\y", "a/**/*", opts.clone(), false);
    assert_is_match("a\\x\\y\\z", "a/**/*", opts.clone(), false);

    assert_is_match("a\\a", "a/**/**/*", opts.clone(), false);
    assert_is_match("a\\b", "a/**/**/*", opts.clone(), false);
    assert_is_match("a\\c", "a/**/**/*", opts.clone(), false);
    assert_is_match("a\\x", "a/**/**/*", opts.clone(), false);
    assert_is_match("a\\x\\y", "a/**/**/*", opts.clone(), false);
    assert_is_match("a\\x\\y\\z", "a/**/**/*", opts.clone(), false);
}

#[test]
fn should_work_with_file_extensions() {
    assert_is_match("a.txt", "a*.txt", windows_options(), true);
    assert_is_match("a\\b.txt", "a*.txt", windows_options(), false);
    assert_is_match("a\\x\\y.txt", "a*.txt", windows_options(), false);
    assert_is_match("a\\x\\y\\z", "a*.txt", windows_options(), false);

    assert_is_match("a.txt", "a.txt", windows_options(), true);
    assert_is_match("a\\b.txt", "a.txt", windows_options(), false);
    assert_is_match("a\\x\\y.txt", "a.txt", windows_options(), false);
    assert_is_match("a\\x\\y\\z", "a.txt", windows_options(), false);

    assert_is_match("a.txt", "a/**/*.txt", windows_options(), false);
    assert_is_match("a\\b.txt", "a/**/*.txt", windows_options(), true);
    assert_is_match("a\\x\\y.txt", "a/**/*.txt", windows_options(), true);
    assert_is_match("a\\x\\y\\z", "a/**/*.txt", windows_options(), false);

    let opts = default_compile_options();
    assert_is_match("a.txt", "a/**/*.txt", opts.clone(), false);
    assert_is_match("a\\b.txt", "a/**/*.txt", opts.clone(), false);
    assert_is_match("a\\x\\y.txt", "a/**/*.txt", opts.clone(), false);
    assert_is_match("a\\x\\y\\z", "a/**/*.txt", opts.clone(), false);

    assert_is_match("a.txt", "a/*.txt", windows_options(), false);
    assert_is_match("a\\b.txt", "a/*.txt", windows_options(), true);
    assert_is_match("a\\x\\y.txt", "a/*.txt", windows_options(), false);
    assert_is_match("a\\x\\y\\z", "a/*.txt", windows_options(), false);

    assert_is_match("a.txt", "a/*.txt", opts.clone(), false);
    assert_is_match("a\\b.txt", "a/*.txt", opts.clone(), false);
    assert_is_match("a\\x\\y.txt", "a/*.txt", opts.clone(), false);
    assert_is_match("a\\x\\y\\z", "a/*.txt", opts.clone(), false);

    assert_is_match("a.txt", "a/*/*.txt", windows_options(), false);
    assert_is_match("a\\b.txt", "a/*/*.txt", windows_options(), false);
    assert_is_match("a\\x\\y.txt", "a/*/*.txt", windows_options(), true);
    assert_is_match("a\\x\\y\\z", "a/*/*.txt", windows_options(), false);

    assert_is_match("a.txt", "a/*/*.txt", opts.clone(), false);
    assert_is_match("a\\b.txt", "a/*/*.txt", opts.clone(), false);
    assert_is_match("a\\x\\y.txt", "a/*/*.txt", opts.clone(), false);
    assert_is_match("a\\x\\y\\z", "a/*/*.txt", opts.clone(), false);
}

#[test]
fn should_support_negation_patterns() {
    assert_is_match("a", "!a/b", windows_options(), true);
    assert_is_match("a\\a", "!a/b", windows_options(), true);
    assert_is_match("a\\b", "!a/b", windows_options(), false);
    assert_is_match("a\\c", "!a/b", windows_options(), true);
    assert_is_match("b\\a", "!a/b", windows_options(), true);
    assert_is_match("b\\b", "!a/b", windows_options(), true);
    assert_is_match("b\\c", "!a/b", windows_options(), true);

    assert_is_match("a", "!*/c", windows_options(), true);
    assert_is_match("a\\a", "!*/c", windows_options(), true);
    assert_is_match("a\\b", "!*/c", windows_options(), true);
    assert_is_match("a\\c", "!*/c", windows_options(), false);
    assert_is_match("b\\a", "!*/c", windows_options(), true);
    assert_is_match("b\\b", "!*/c", windows_options(), true);
    assert_is_match("b\\c", "!*/c", windows_options(), false);

    assert_is_match("a", "!a/(b)", windows_options(), true);
    assert_is_match("a\\a", "!a/(b)", windows_options(), true);
    assert_is_match("a\\b", "!a/(b)", windows_options(), false);
    assert_is_match("a\\c", "!a/(b)", windows_options(), true);
    assert_is_match("b\\a", "!a/(b)", windows_options(), true);
    assert_is_match("b\\b", "!a/(b)", windows_options(), true);
    assert_is_match("b\\c", "!a/(b)", windows_options(), true);

    assert_is_match("a", "!(a/b)", windows_options(), true);
    assert_is_match("a\\a", "!(a/b)", windows_options(), true);
    assert_is_match("a\\b", "!(a/b)", windows_options(), false);
    assert_is_match("a\\c", "!(a/b)", windows_options(), true);
    assert_is_match("b\\a", "!(a/b)", windows_options(), true);
    assert_is_match("b\\b", "!(a/b)", windows_options(), true);
    assert_is_match("b\\c", "!(a/b)", windows_options(), true);
}
