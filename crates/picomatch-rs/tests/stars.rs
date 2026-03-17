mod support;

use picomatch_rs::CompileOptions;

use support::{assert_is_match, assert_is_match_any, default_compile_options};

#[test]
fn respects_dots_defined_in_glob_pattern() {
    assert_is_match("z.js", "z*", default_compile_options(), true);
    assert_is_match("zzjs", "z*.js", default_compile_options(), false);
    assert_is_match("zzjs", "*z.js", default_compile_options(), false);
}

#[test]
fn single_stars_match_anything_except_slashes_and_leading_dots() {
    for (input, pattern, expected) in [
        ("a/b/c/z.js", "*.js", false),
        ("a/b/z.js", "*.js", false),
        ("a/z.js", "*.js", false),
        ("z.js", "*.js", true),
        ("a/.ab", "*/*", false),
        (".ab", "*", false),
        ("z.js", "z*.js", true),
        ("a/z", "*/*", true),
        ("a/z.js", "*/z*.js", true),
        ("a/z.js", "a/z*.js", true),
        ("ab", "*", true),
        ("abc", "*", true),
        ("bar", "f*", false),
        ("foo", "*r", false),
        ("foo", "b*", false),
        ("foo/bar", "*", false),
        ("abc", "*c", true),
        ("abc", "a*", true),
        ("abc", "a*c", true),
        ("bar", "*r", true),
        ("bar", "b*", true),
        ("foo", "f*", true),
    ] {
        assert_is_match(input, pattern, default_compile_options(), expected);
    }
}

#[test]
fn single_stars_match_spaces() {
    assert_is_match("one abc two", "*abc*", default_compile_options(), true);
    assert_is_match("a         b", "a*b", default_compile_options(), true);
}

#[test]
fn supports_multiple_non_consecutive_stars_in_a_path_segment() {
    for (input, pattern, expected) in [
        ("foo", "*a*", false),
        ("bar", "*a*", true),
        ("oneabctwo", "*abc*", true),
        ("a-b.c-d", "*-bc-*", false),
        ("a-b.c-d", "*-*.*-*", true),
        ("a-b.c-d", "*-b*c-*", true),
        ("a-b.c-d", "*-b.c-*", true),
        ("a-b.c-d", "*.*", true),
        ("a-b.c-d", "*.*-*", true),
        ("a-b.c-d", "*.*-d", true),
        ("a-b.c-d", "*.c-*", true),
        ("a-b.c-d", "*b.*d", true),
        ("a-b.c-d", "a*.c*", true),
        ("a-b.c-d", "a-*.*-d", true),
        ("a.b", "*.*", true),
        ("a.b", "*.b", true),
        ("a.b", "a.*", true),
        ("a.b", "a.b", true),
    ] {
        assert_is_match(input, pattern, default_compile_options(), expected);
    }
}

#[test]
fn supports_multiple_stars_in_a_segment() {
    for (input, pattern, expected) in [
        ("a-b.c-d", "**-bc-**", false),
        ("a-b.c-d", "**-**.**-**", true),
        ("a-b.c-d", "**-b**c-**", true),
        ("a-b.c-d", "**-b.c-**", true),
        ("a-b.c-d", "**.**", true),
        ("a-b.c-d", "**.**-**", true),
        ("a-b.c-d", "**.**-d", true),
        ("a-b.c-d", "**.c-**", true),
        ("a-b.c-d", "**b.**d", true),
        ("a-b.c-d", "a**.c**", true),
        ("a-b.c-d", "a-**.**-d", true),
        ("a.b", "**.**", true),
        ("a.b", "**.b", true),
        ("a.b", "a.**", true),
        ("a.b", "a.b", true),
    ] {
        assert_is_match(input, pattern, default_compile_options(), expected);
    }
}

#[test]
fn returns_true_when_one_of_the_given_patterns_matches_the_string() {
    for (input, pattern) in [
        ("/ab", "*/*"),
        (".", "."),
        ("/ab", "/*"),
        ("/ab", "/??"),
        ("/ab", "/?b"),
        ("/cd", "/*"),
        ("a", "a"),
        ("a/.b", "a/.*"),
        ("a/b", "?/?"),
        ("a/b/c/d/e/j/n/p/o/z/c.md", "a/**/j/**/z/*.md"),
        ("a/b/c/d/e/z/c.md", "a/**/z/*.md"),
        ("a/b/c/xyz.md", "a/b/c/*.md"),
        ("a/b/c/xyz.md", "a/b/c/*.md"),
        ("a/b/z/.a", "a/*/z/.a"),
        ("a/bb.bb/aa/b.b/aa/c/xyz.md", "a/**/c/*.md"),
        ("a/bb.bb/aa/bb/aa/c/xyz.md", "a/**/c/*.md"),
        ("a/bb.bb/c/xyz.md", "a/*/c/*.md"),
        ("a/bb/c/xyz.md", "a/*/c/*.md"),
        ("a/bbbb/c/xyz.md", "a/*/c/*.md"),
        ("aaa", "*"),
        ("ab", "*"),
        ("ab", "ab"),
    ] {
        assert_is_match(input, pattern, default_compile_options(), true);
    }

    assert_is_match("a/.b", "a/", default_compile_options(), false);
    assert_is_match("a/b/z/.a", "bz", default_compile_options(), false);
}

#[test]
fn returns_false_when_the_path_does_not_match_the_pattern() {
    for (input, patterns) in [
        ("/ab", &["*/"][..]),
        ("/ab", &["*/a"][..]),
        ("/ab", &["/"][..]),
        ("/ab", &["/?"][..]),
        ("/ab", &["/a"][..]),
        ("/ab", &["?/?"][..]),
        ("/ab", &["a/*"][..]),
        ("a/.b", &["a/"][..]),
        ("a/b/c", &["a/*"][..]),
        ("a/b/c", &["a/b"][..]),
        ("a/b/c/d/e/z/c.md", &["b/c/d/e"][..]),
        ("a/b/z/.a", &["b/z"][..]),
        ("ab", &["*/*"][..]),
        ("ab", &["/a"][..]),
        ("ab", &["a"][..]),
        ("ab", &["b"][..]),
        ("ab", &["c"][..]),
        ("abcd", &["ab"][..]),
        ("abcd", &["bc"][..]),
        ("abcd", &["c"][..]),
        ("abcd", &["cd"][..]),
        ("abcd", &["d"][..]),
        ("abcd", &["f"][..]),
        ("ef", &["/*"][..]),
    ] {
        assert_is_match_any(input, patterns, default_compile_options(), false);
    }
}

#[test]
fn matches_a_path_segment_for_each_single_star() {
    for (input, pattern, expected) in [
        ("aaa", "*/*/*", false),
        ("aaa/bb/aa/rr", "*/*/*", false),
        ("aaa/bba/ccc", "aaa*", false),
        ("aaa/bba/ccc", "aaa**", false),
        ("aaa/bba/ccc", "aaa/*", false),
        ("aaa/bba/ccc", "aaa/*ccc", false),
        ("aaa/bba/ccc", "aaa/*z", false),
        ("aaa/bbb", "*/*/*", false),
        ("ab/zzz/ejkl/hi", "*/*jk*/*i", false),
        ("aaa/bba/ccc", "*/*/*", true),
        ("aaa/bba/ccc", "aaa/**", true),
        ("aaa/bbb", "aaa/*", true),
        ("ab/zzz/ejkl/hi", "*/*z*/*/*i", true),
        ("abzzzejklhi", "*j*i", true),
    ] {
        assert_is_match(input, pattern, default_compile_options(), expected);
    }
}

#[test]
fn supports_single_globs() {
    for (input, pattern, expected) in [
        ("a", "*", true),
        ("b", "*", true),
        ("a/a", "*", false),
        ("a/a/a", "*", false),
        ("a/a/b", "*", false),
        ("a/a/a/a", "*", false),
        ("a/a/a/a/a", "*", false),
        ("a", "*/*", false),
        ("a/a", "*/*", true),
        ("a/a/a", "*/*", false),
        ("a", "*/*/*", false),
        ("a/a", "*/*/*", false),
        ("a/a/a", "*/*/*", true),
        ("a/a/a/a", "*/*/*", false),
        ("a", "*/*/*/*", false),
        ("a/a", "*/*/*/*", false),
        ("a/a/a", "*/*/*/*", false),
        ("a/a/a/a", "*/*/*/*", true),
        ("a/a/a/a/a", "*/*/*/*", false),
        ("a", "*/*/*/*/*", false),
        ("a/a", "*/*/*/*/*", false),
        ("a/a/a", "*/*/*/*/*", false),
        ("a/a/b", "*/*/*/*/*", false),
        ("a/a/a/a", "*/*/*/*/*", false),
        ("a/a/a/a/a", "*/*/*/*/*", true),
        ("a/a/a/a/a/a", "*/*/*/*/*", false),
        ("a", "a/*", false),
        ("a/a", "a/*", true),
        ("a/a/a", "a/*", false),
        ("a/a/a/a", "a/*", false),
        ("a/a/a/a/a", "a/*", false),
        ("a", "a/*/*", false),
        ("a/a", "a/*/*", false),
        ("a/a/a", "a/*/*", true),
        ("b/a/a", "a/*/*", false),
        ("a/a/a/a", "a/*/*", false),
        ("a/a/a/a/a", "a/*/*", false),
        ("a", "a/*/*/*", false),
        ("a/a", "a/*/*/*", false),
        ("a/a/a", "a/*/*/*", false),
        ("a/a/a/a", "a/*/*/*", true),
        ("a/a/a/a/a", "a/*/*/*", false),
        ("a", "a/*/*/*/*", false),
        ("a/a", "a/*/*/*/*", false),
        ("a/a/a", "a/*/*/*/*", false),
        ("a/a/b", "a/*/*/*/*", false),
        ("a/a/a/a", "a/*/*/*/*", false),
        ("a/a/a/a/a", "a/*/*/*/*", true),
        ("a", "a/*/a", false),
        ("a/a", "a/*/a", false),
        ("a/a/a", "a/*/a", true),
        ("a/a/b", "a/*/a", false),
        ("a/a/a/a", "a/*/a", false),
        ("a/a/a/a/a", "a/*/a", false),
        ("a", "a/*/b", false),
        ("a/a", "a/*/b", false),
        ("a/a/a", "a/*/b", false),
        ("a/a/b", "a/*/b", true),
        ("a/a/a/a", "a/*/b", false),
        ("a/a/a/a/a", "a/*/b", false),
    ] {
        assert_is_match(input, pattern, default_compile_options(), expected);
    }
}

#[test]
fn only_matches_a_single_folder_per_star_when_globstars_are_used() {
    for (input, expected) in [
        ("a", false),
        ("a/a/b", false),
        ("a/a", true),
        ("a/a/a", true),
        ("a/a/a/a", true),
        ("a/a/a/a/a", true),
    ] {
        assert_is_match(input, "*/**/a", default_compile_options(), expected);
    }
}

#[test]
fn does_not_match_a_trailing_slash_when_a_star_is_last_char() {
    for (input, pattern, expected) in [
        ("a", "*/", false),
        ("a", "*/*", false),
        ("a", "a/*", false),
        ("a/", "*/*", false),
        ("a/", "a/*", false),
        ("a/a", "*", false),
        ("a/a", "*/", false),
        ("a/x/y", "*/", false),
        ("a/x/y", "*/*", false),
        ("a/x/y", "a/*", false),
        ("a/", "*", true),
        ("a/", "*/", true),
        ("a/", "*{,/}", true),
        ("a/a", "*/*", true),
        ("a/a", "a/*", true),
        ("a", "*", true),
    ] {
        assert_is_match(input, pattern, default_compile_options(), expected);
    }

    let strict_slashes = CompileOptions {
        strict_slashes: true,
        ..CompileOptions::default()
    };
    assert_is_match("a/", "*", strict_slashes, false);
}

#[test]
fn works_with_file_extensions() {
    for (input, pattern, expected) in [
        ("a.txt", "a/**/*.txt", false),
        ("a/x/y.txt", "a/**/*.txt", true),
        ("a/x/y/z", "a/**/*.txt", false),
        ("a.txt", "a/*.txt", false),
        ("a/b.txt", "a/*.txt", true),
        ("a/x/y.txt", "a/*.txt", false),
        ("a/x/y/z", "a/*.txt", false),
        ("a.txt", "a*.txt", true),
        ("a/b.txt", "a*.txt", false),
        ("a/x/y.txt", "a*.txt", false),
        ("a/x/y/z", "a*.txt", false),
        ("a.txt", "*.txt", true),
        ("a/b.txt", "*.txt", false),
        ("a/x/y.txt", "*.txt", false),
        ("a/x/y/z", "*.txt", false),
    ] {
        assert_is_match(input, pattern, default_compile_options(), expected);
    }
}

#[test]
fn does_not_match_slashes_when_globstars_are_not_exclusive_in_a_path_segment() {
    assert_is_match("foo/baz/bar", "foo**bar", default_compile_options(), false);
    assert_is_match("foobazbar", "foo**bar", default_compile_options(), true);
}

#[test]
fn matches_slashes_when_defined_in_braces() {
    assert_is_match("foo", "foo{,/**}", default_compile_options(), true);
}

#[test]
fn correctly_matches_slashes() {
    for (input, pattern, expected) in [
        ("a/b", "a*", false),
        ("a/a/bb", "a/**/b", false),
        ("a/bb", "a/**/b", false),
        ("foo", "*/**", false),
        ("foo/bar", "**/", false),
        ("foo/bar", "**/*/", false),
        ("foo/bar", "*/*/", false),
        ("/home/foo/..", "**/..", true),
        ("a", "**/a", true),
        ("a/a", "**", true),
        ("a/a", "a/**", true),
        ("a/", "a/**", true),
        ("a", "a/**", true),
        ("a/a", "**/", false),
        ("a", "**/a/**", true),
        ("a", "a/**", true),
        ("a/a", "**/", false),
        ("a/a", "*/**/a", true),
        ("a", "a/**", true),
        ("foo/", "*/**", true),
        ("foo/bar", "**/*", true),
        ("foo/bar", "*/*", true),
        ("foo/bar", "*/**", true),
        ("foo/bar/", "**/", true),
        ("foo/bar/", "**/*", true),
        ("foo/bar/", "**/*/", true),
        ("foo/bar/", "*/**", true),
        ("foo/bar/", "*/*/", true),
        ("bar/baz/foo", "*/foo", false),
        ("deep/foo/bar", "**/bar/*", false),
        ("deep/foo/bar/baz/x", "*/bar/**", false),
        ("ef", "/*", false),
        ("foo/bar", "foo?bar", false),
        ("foo/bar/baz", "**/bar*", false),
        ("foo/bar/baz", "**/bar**", false),
        ("foo/baz/bar", "foo**bar", false),
        ("foo/baz/bar", "foo*bar", false),
        ("foo", "foo/**", true),
        ("/ab", "/*", true),
        ("/cd", "/*", true),
        ("/ef", "/*", true),
        ("a/b/j/c/z/x.md", "a/**/j/**/z/*.md", true),
        ("a/j/z/x.md", "a/**/j/**/z/*.md", true),
        ("bar/baz/foo", "**/foo", true),
        ("deep/foo/bar/baz", "**/bar/*", true),
        ("deep/foo/bar/baz/", "**/bar/**", true),
        ("deep/foo/bar/baz/x", "**/bar/*/*", true),
        ("foo/b/a/z/bar", "foo/**/**/bar", true),
        ("foo/b/a/z/bar", "foo/**/bar", true),
        ("foo/bar", "foo/**/**/bar", true),
        ("foo/bar", "foo/**/bar", true),
        ("foo/bar/baz/x", "*/bar/**", true),
        ("foo/baz/bar", "foo/**/**/bar", true),
        ("foo/baz/bar", "foo/**/bar", true),
        ("XXX/foo", "**/foo", true),
    ] {
        assert_is_match(input, pattern, default_compile_options(), expected);
    }

    let strict_slashes = CompileOptions {
        strict_slashes: true,
        ..CompileOptions::default()
    };
    assert_is_match("foo/bar/", "**/*", strict_slashes, false);
}

#[test]
fn ignores_leading_dot_slash_when_defined_on_pattern() {
    assert_is_match("ab", "./*", default_compile_options(), true);
    assert_is_match("ab", "./*/", default_compile_options(), false);
    assert_is_match("ab/", "./*/", default_compile_options(), true);
}

#[test]
fn optionally_matches_trailing_slashes_with_braces() {
    for input in ["foo", "foo/bar"] {
        assert_is_match(input, "**/*", default_compile_options(), true);
    }

    for input in ["foo", "foo/", "foo/bar", "foo/bar/"] {
        assert_is_match(input, "**/*{,/}", default_compile_options(), true);
    }
}
