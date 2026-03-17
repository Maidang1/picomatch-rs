mod support;

use picomatch_rs::CompileOptions;
use support::assert_is_match;

// Note: The majority of these cases are also covered by match_basic.rs.
// This file provides a complete, faithful translation from test/non-globs.js
// as a self-contained regression suite.

fn win_opts() -> CompileOptions {
    CompileOptions {
        windows: true,
        ..Default::default()
    }
}

#[test]
fn test_match_non_globs() {
    // negative cases
    assert_is_match("/ab", "/a", Default::default(), false);
    assert_is_match("a/a", "a/b", Default::default(), false);
    assert_is_match("a/a", "a/c", Default::default(), false);
    assert_is_match("a/b", "a/c", Default::default(), false);
    assert_is_match("a/c", "a/b", Default::default(), false);
    assert_is_match("aaa", "aa", Default::default(), false);
    assert_is_match("ab", "/a", Default::default(), false);
    assert_is_match("ab", "a", Default::default(), false);

    // positive cases – leading-slash paths
    assert_is_match("/a", "/a", Default::default(), true);
    assert_is_match("/a/", "/a/", Default::default(), true);
    assert_is_match("/a/a", "/a/a", Default::default(), true);
    assert_is_match("/a/a/", "/a/a/", Default::default(), true);
    assert_is_match("/a/a/a", "/a/a/a", Default::default(), true);
    assert_is_match("/a/a/a/", "/a/a/a/", Default::default(), true);
    assert_is_match("/a/a/a/a", "/a/a/a/a", Default::default(), true);
    assert_is_match("/a/a/a/a/a", "/a/a/a/a/a", Default::default(), true);

    // positive cases – relative paths
    assert_is_match("a", "a", Default::default(), true);
    assert_is_match("a/", "a/", Default::default(), true);
    assert_is_match("a/a", "a/a", Default::default(), true);
    assert_is_match("a/a/", "a/a/", Default::default(), true);
    assert_is_match("a/a/a", "a/a/a", Default::default(), true);
    assert_is_match("a/a/a/", "a/a/a/", Default::default(), true);
    assert_is_match("a/a/a/a", "a/a/a/a", Default::default(), true);
    assert_is_match("a/a/a/a/a", "a/a/a/a/a", Default::default(), true);
}

#[test]
fn test_match_literal_dots() {
    assert_is_match(".", ".", Default::default(), true);
    assert_is_match("..", "..", Default::default(), true);
    assert_is_match("...", "..", Default::default(), false);
    assert_is_match("...", "...", Default::default(), true);
    assert_is_match("....", "....", Default::default(), true);
    assert_is_match("....", "...", Default::default(), false);
}

#[test]
fn test_escaped_characters_as_literals() {
    assert_is_match("abc", "abc\\*", Default::default(), false);
    assert_is_match("abc*", "abc\\*", Default::default(), true);
}

#[test]
fn test_match_windows_paths() {
    assert_is_match("aaa\\bbb", "aaa/bbb", win_opts(), true);
    assert_is_match("aaa/bbb", "aaa/bbb", win_opts(), true);
}
