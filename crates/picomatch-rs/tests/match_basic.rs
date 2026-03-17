mod support;

use picomatch_rs::CompileOptions;

use support::{assert_is_match, assert_is_match_any, assert_match_list, default_compile_options};

#[test]
fn validates_empty_patterns() {
    let err = picomatch_rs::is_match("foo", "", &default_compile_options()).unwrap_err();
    assert!(matches!(err, picomatch_rs::MatchError::EmptyPattern));
}

#[test]
fn matches_non_globs() {
    for (input, pattern) in [
        ("/a", "/a"),
        ("/a/", "/a/"),
        ("/a/a", "/a/a"),
        ("a", "a"),
        ("a/a", "a/a"),
        ("a/a/a", "a/a/a"),
        (".", "."),
        ("..", ".."),
        ("...", "..."),
        ("abc*", "abc\\*"),
    ] {
        assert_is_match(input, pattern, default_compile_options(), true);
    }

    for (input, pattern) in [
        ("/ab", "/a"),
        ("a/a", "a/b"),
        ("a/a", "a/c"),
        ("aaa", "aa"),
        ("ab", "/a"),
        ("ab", "a"),
        ("...", ".."),
        ("....", "..."),
        ("abc", "abc\\*"),
    ] {
        assert_is_match(input, pattern, default_compile_options(), false);
    }
}

#[test]
fn matches_windows_literals() {
    let options = CompileOptions {
        windows: true,
        ..CompileOptions::default()
    };

    assert_is_match("aaa\\bbb", "aaa/bbb", options.clone(), true);
    assert_is_match("aaa/bbb", "aaa/bbb", options, true);
}

#[test]
fn matches_any_pattern_in_a_list() {
    assert_is_match_any("a", &["a", "foo"], default_compile_options(), true);
    assert_is_match_any("ab", &["*", "foo", "bar"], default_compile_options(), true);
    assert_is_match_any("ab", &["*b", "foo", "bar"], default_compile_options(), true);
    assert_is_match_any("ab", &["a*", "foo", "bar"], default_compile_options(), true);
    assert_is_match_any("/ab", &["/a", "foo"], default_compile_options(), false);
    assert_is_match_any("a/b/c", &["a/b", "foo"], default_compile_options(), false);
    assert_is_match_any(
        "ab",
        &["*/*", "foo", "bar"],
        default_compile_options(),
        false,
    );
}

#[test]
fn matches_basic_extensions_and_wildcards() {
    for (input, pattern) in [
        (".c.md", ".*.md"),
        (".md", ".md"),
        ("a/b/c.js", "a/**/*.*"),
        ("a/b/c.md", "**/*.md"),
        ("a/b/c.md", "a/*/*.md"),
        ("c.md", "*.md"),
        ("z.js", "*.js"),
        ("z.js", "z*.js"),
        ("a/z.js", "*/z*.js"),
        ("a/z.js", "a/z*.js"),
    ] {
        assert_is_match(input, pattern, default_compile_options(), true);
    }

    for (input, pattern) in [
        (".c.md", "*.md"),
        (".md", "*.md"),
        ("a/b/c.md", "*.md"),
        ("a/b/c.md", "a/*.md"),
        ("a/b/c/c.md", "*.md"),
        ("a/b/c/z.js", "*.js"),
        ("a/b/z.js", "*.js"),
        ("a/z.js", "*.js"),
    ] {
        assert_is_match(input, pattern, default_compile_options(), false);
    }
}

#[test]
fn filters_simple_question_mark_cases() {
    assert_match_list(&["?", "??", "???"], "?", default_compile_options(), &["?"]);
    assert_match_list(
        &["?", "??", "???"],
        "??",
        default_compile_options(),
        &["??"],
    );
    assert_match_list(
        &["?", "??", "???"],
        "???",
        default_compile_options(),
        &["???"],
    );
}
