mod support;

use picomatch_rs::CompileOptions;

use support::{assert_is_match, assert_is_match_any, default_compile_options};

#[test]
fn should_match_literal_string() {
    assert_is_match("a/a", "(a/b)", default_compile_options(), false);
    assert_is_match("a/b", "(a/b)", default_compile_options(), true);
    assert_is_match("a/c", "(a/b)", default_compile_options(), false);
    assert_is_match("b/a", "(a/b)", default_compile_options(), false);
    assert_is_match("b/b", "(a/b)", default_compile_options(), false);
    assert_is_match("b/c", "(a/b)", default_compile_options(), false);

    assert_is_match("a/a", "a/b", default_compile_options(), false);
    assert_is_match("a/b", "a/b", default_compile_options(), true);
    assert_is_match("a/c", "a/b", default_compile_options(), false);
    assert_is_match("b/a", "a/b", default_compile_options(), false);
    assert_is_match("b/b", "a/b", default_compile_options(), false);
    assert_is_match("b/c", "a/b", default_compile_options(), false);
}

#[test]
fn should_match_an_array_of_literal_strings() {
    assert_is_match("a/a", "a/b", default_compile_options(), false);
    assert_is_match("a/b", "a/b", default_compile_options(), true);
    assert_is_match("a/c", "a/b", default_compile_options(), false);
    assert_is_match("b/a", "a/b", default_compile_options(), false);
    assert_is_match("b/b", "a/b", default_compile_options(), false);
    assert_is_match("b/b", "b/b", default_compile_options(), true);
    assert_is_match("b/c", "a/b", default_compile_options(), false);
}

#[test]
fn should_support_regex_logical_or() {
    assert_is_match("a/a", "a/(a|c)", default_compile_options(), true);
    assert_is_match("a/b", "a/(a|c)", default_compile_options(), false);
    assert_is_match("a/c", "a/(a|c)", default_compile_options(), true);

    assert_is_match("a/a", "a/(a|b|c)", default_compile_options(), true);
    assert_is_match("a/b", "a/(a|b|c)", default_compile_options(), true);
    assert_is_match("a/c", "a/(a|b|c)", default_compile_options(), true);
}

#[test]
fn should_support_regex_ranges() {
    assert_is_match("a/a", "a/[b-c]", default_compile_options(), false);
    assert_is_match("a/b", "a/[b-c]", default_compile_options(), true);
    assert_is_match("a/c", "a/[b-c]", default_compile_options(), true);

    assert_is_match("a/a", "a/[a-z]", default_compile_options(), true);
    assert_is_match("a/b", "a/[a-z]", default_compile_options(), true);
    assert_is_match("a/c", "a/[a-z]", default_compile_options(), true);
    assert_is_match("a/x/y", "a/[a-z]", default_compile_options(), false);
    assert_is_match("a/x", "a/[a-z]", default_compile_options(), true);
}

#[test]
fn should_support_single_globs() {
    // pattern: *
    assert_is_match("a", "*", default_compile_options(), true);
    assert_is_match("b", "*", default_compile_options(), true);
    assert_is_match("a/a", "*", default_compile_options(), false);
    assert_is_match("a/b", "*", default_compile_options(), false);
    assert_is_match("a/c", "*", default_compile_options(), false);
    assert_is_match("a/x", "*", default_compile_options(), false);
    assert_is_match("a/a/a", "*", default_compile_options(), false);
    assert_is_match("a/a/b", "*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "*", default_compile_options(), false);
    assert_is_match("x/y", "*", default_compile_options(), false);
    assert_is_match("z/z", "*", default_compile_options(), false);

    // pattern: */*
    assert_is_match("a", "*/*", default_compile_options(), false);
    assert_is_match("b", "*/*", default_compile_options(), false);
    assert_is_match("a/a", "*/*", default_compile_options(), true);
    assert_is_match("a/b", "*/*", default_compile_options(), true);
    assert_is_match("a/c", "*/*", default_compile_options(), true);
    assert_is_match("a/x", "*/*", default_compile_options(), true);
    assert_is_match("a/a/a", "*/*", default_compile_options(), false);
    assert_is_match("a/a/b", "*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "*/*", default_compile_options(), false);
    assert_is_match("x/y", "*/*", default_compile_options(), true);
    assert_is_match("z/z", "*/*", default_compile_options(), true);

    // pattern: */*/*
    assert_is_match("a", "*/*/*", default_compile_options(), false);
    assert_is_match("b", "*/*/*", default_compile_options(), false);
    assert_is_match("a/a", "*/*/*", default_compile_options(), false);
    assert_is_match("a/b", "*/*/*", default_compile_options(), false);
    assert_is_match("a/c", "*/*/*", default_compile_options(), false);
    assert_is_match("a/x", "*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a", "*/*/*", default_compile_options(), true);
    assert_is_match("a/a/b", "*/*/*", default_compile_options(), true);
    assert_is_match("a/a/a/a", "*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "*/*/*", default_compile_options(), false);
    assert_is_match("x/y", "*/*/*", default_compile_options(), false);
    assert_is_match("z/z", "*/*/*", default_compile_options(), false);

    // pattern: */*/*/*
    assert_is_match("a", "*/*/*/*", default_compile_options(), false);
    assert_is_match("b", "*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a", "*/*/*/*", default_compile_options(), false);
    assert_is_match("a/b", "*/*/*/*", default_compile_options(), false);
    assert_is_match("a/c", "*/*/*/*", default_compile_options(), false);
    assert_is_match("a/x", "*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a", "*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/b", "*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "*/*/*/*", default_compile_options(), true);
    assert_is_match("a/a/a/a/a", "*/*/*/*", default_compile_options(), false);
    assert_is_match("x/y", "*/*/*/*", default_compile_options(), false);
    assert_is_match("z/z", "*/*/*/*", default_compile_options(), false);

    // pattern: */*/*/*/*
    assert_is_match("a", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("b", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/b", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/c", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/x", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/b", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "*/*/*/*/*", default_compile_options(), true);
    assert_is_match("x/y", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("z/z", "*/*/*/*/*", default_compile_options(), false);

    // pattern: a/*
    assert_is_match("a", "a/*", default_compile_options(), false);
    assert_is_match("b", "a/*", default_compile_options(), false);
    assert_is_match("a/a", "a/*", default_compile_options(), true);
    assert_is_match("a/b", "a/*", default_compile_options(), true);
    assert_is_match("a/c", "a/*", default_compile_options(), true);
    assert_is_match("a/x", "a/*", default_compile_options(), true);
    assert_is_match("a/a/a", "a/*", default_compile_options(), false);
    assert_is_match("a/a/b", "a/*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "a/*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "a/*", default_compile_options(), false);
    assert_is_match("x/y", "a/*", default_compile_options(), false);
    assert_is_match("z/z", "a/*", default_compile_options(), false);

    // pattern: a/*/*
    assert_is_match("a", "a/*/*", default_compile_options(), false);
    assert_is_match("b", "a/*/*", default_compile_options(), false);
    assert_is_match("a/a", "a/*/*", default_compile_options(), false);
    assert_is_match("a/b", "a/*/*", default_compile_options(), false);
    assert_is_match("a/c", "a/*/*", default_compile_options(), false);
    assert_is_match("a/x", "a/*/*", default_compile_options(), false);
    assert_is_match("a/a/a", "a/*/*", default_compile_options(), true);
    assert_is_match("a/a/b", "a/*/*", default_compile_options(), true);
    assert_is_match("a/a/a/a", "a/*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "a/*/*", default_compile_options(), false);
    assert_is_match("x/y", "a/*/*", default_compile_options(), false);
    assert_is_match("z/z", "a/*/*", default_compile_options(), false);

    // pattern: a/*/*/*
    assert_is_match("a", "a/*/*/*", default_compile_options(), false);
    assert_is_match("b", "a/*/*/*", default_compile_options(), false);
    assert_is_match("a/a", "a/*/*/*", default_compile_options(), false);
    assert_is_match("a/b", "a/*/*/*", default_compile_options(), false);
    assert_is_match("a/c", "a/*/*/*", default_compile_options(), false);
    assert_is_match("a/x", "a/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a", "a/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/b", "a/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "a/*/*/*", default_compile_options(), true);
    assert_is_match("a/a/a/a/a", "a/*/*/*", default_compile_options(), false);
    assert_is_match("x/y", "a/*/*/*", default_compile_options(), false);
    assert_is_match("z/z", "a/*/*/*", default_compile_options(), false);

    // pattern: a/*/*/*/*
    assert_is_match("a", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("b", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/b", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/c", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/x", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/b", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "a/*/*/*/*", default_compile_options(), true);
    assert_is_match("x/y", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("z/z", "a/*/*/*/*", default_compile_options(), false);

    // pattern: a/*/a
    assert_is_match("a", "a/*/a", default_compile_options(), false);
    assert_is_match("b", "a/*/a", default_compile_options(), false);
    assert_is_match("a/a", "a/*/a", default_compile_options(), false);
    assert_is_match("a/b", "a/*/a", default_compile_options(), false);
    assert_is_match("a/c", "a/*/a", default_compile_options(), false);
    assert_is_match("a/x", "a/*/a", default_compile_options(), false);
    assert_is_match("a/a/a", "a/*/a", default_compile_options(), true);
    assert_is_match("a/a/b", "a/*/a", default_compile_options(), false);
    assert_is_match("a/a/a/a", "a/*/a", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "a/*/a", default_compile_options(), false);
    assert_is_match("x/y", "a/*/a", default_compile_options(), false);
    assert_is_match("z/z", "a/*/a", default_compile_options(), false);

    // pattern: a/*/b
    assert_is_match("a", "a/*/b", default_compile_options(), false);
    assert_is_match("b", "a/*/b", default_compile_options(), false);
    assert_is_match("a/a", "a/*/b", default_compile_options(), false);
    assert_is_match("a/b", "a/*/b", default_compile_options(), false);
    assert_is_match("a/c", "a/*/b", default_compile_options(), false);
    assert_is_match("a/x", "a/*/b", default_compile_options(), false);
    assert_is_match("a/a/a", "a/*/b", default_compile_options(), false);
    assert_is_match("a/a/b", "a/*/b", default_compile_options(), true);
    assert_is_match("a/a/a/a", "a/*/b", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "a/*/b", default_compile_options(), false);
    assert_is_match("x/y", "a/*/b", default_compile_options(), false);
    assert_is_match("z/z", "a/*/b", default_compile_options(), false);
}

#[test]
fn should_support_globstars() {
    // literal "a"
    assert_is_match("a", "a", default_compile_options(), true);
    assert_is_match("a/", "a", default_compile_options(), false);
    assert_is_match("a/a", "a", default_compile_options(), false);
    assert_is_match("a/b", "a", default_compile_options(), false);
    assert_is_match("a/c", "a", default_compile_options(), false);
    assert_is_match("a/x", "a", default_compile_options(), false);
    assert_is_match("a/x/y", "a", default_compile_options(), false);
    assert_is_match("a/x/y/z", "a", default_compile_options(), false);

    // pattern: *
    assert_is_match("a", "*", default_compile_options(), true);
    // NOTE: relaxSlashes option not available in Rust - skipping: isMatch('a/', '*', { relaxSlashes: true })
    assert_is_match("a/", "*{,/}", default_compile_options(), true);
    assert_is_match("a/a", "*", default_compile_options(), false);
    assert_is_match("a/b", "*", default_compile_options(), false);
    assert_is_match("a/c", "*", default_compile_options(), false);
    assert_is_match("a/x", "*", default_compile_options(), false);
    assert_is_match("a/x/y", "*", default_compile_options(), false);
    assert_is_match("a/x/y/z", "*", default_compile_options(), false);

    // pattern: */
    assert_is_match("a", "*/", default_compile_options(), false);
    assert_is_match("a/", "*/", default_compile_options(), true);
    assert_is_match("a/a", "*/", default_compile_options(), false);
    assert_is_match("a/b", "*/", default_compile_options(), false);
    assert_is_match("a/c", "*/", default_compile_options(), false);
    assert_is_match("a/x", "*/", default_compile_options(), false);
    assert_is_match("a/x/y", "*/", default_compile_options(), false);
    assert_is_match("a/x/y/z", "*/", default_compile_options(), false);

    // pattern: */*
    assert_is_match("a", "*/*", default_compile_options(), false);
    assert_is_match("a/", "*/*", default_compile_options(), false);
    assert_is_match("a/a", "*/*", default_compile_options(), true);
    assert_is_match("a/b", "*/*", default_compile_options(), true);
    assert_is_match("a/c", "*/*", default_compile_options(), true);
    assert_is_match("a/x", "*/*", default_compile_options(), true);
    assert_is_match("a/x/y", "*/*", default_compile_options(), false);
    assert_is_match("a/x/y/z", "*/*", default_compile_options(), false);

    // pattern: **
    assert_is_match("a", "**", default_compile_options(), true);
    assert_is_match("a/", "**", default_compile_options(), true);
    assert_is_match("a/a", "**", default_compile_options(), true);
    assert_is_match("a/b", "**", default_compile_options(), true);
    assert_is_match("a/c", "**", default_compile_options(), true);
    assert_is_match("a/x", "**", default_compile_options(), true);
    assert_is_match("a/x/y", "**", default_compile_options(), true);
    assert_is_match("a/x/y/z", "**", default_compile_options(), true);

    // pattern: **/a
    assert_is_match("a/", "**/a", default_compile_options(), false);
    assert_is_match("a/b", "**/a", default_compile_options(), false);
    assert_is_match("a/c", "**/a", default_compile_options(), false);
    assert_is_match("a/x", "**/a", default_compile_options(), false);
    assert_is_match("a/x/y/z", "**/a", default_compile_options(), false);
    assert_is_match("a/x/y/z/a", "**/a", default_compile_options(), true);
    assert_is_match("a", "**/a", default_compile_options(), true);
    assert_is_match("a/a", "**/a", default_compile_options(), true);

    // pattern: a/*
    assert_is_match("a", "a/*", default_compile_options(), false);
    assert_is_match("a/", "a/*", default_compile_options(), false);
    assert_is_match("a/a", "a/*", default_compile_options(), true);
    assert_is_match("a/b", "a/*", default_compile_options(), true);
    assert_is_match("a/c", "a/*", default_compile_options(), true);
    assert_is_match("a/x", "a/*", default_compile_options(), true);
    assert_is_match("a/x/y", "a/*", default_compile_options(), false);
    assert_is_match("a/x/y/z", "a/*", default_compile_options(), false);

    // pattern: a/**
    assert_is_match("a", "a/**", default_compile_options(), true);
    assert_is_match("a/", "a/**", default_compile_options(), true);
    assert_is_match("a/a", "a/**", default_compile_options(), true);
    assert_is_match("a/b", "a/**", default_compile_options(), true);
    assert_is_match("a/c", "a/**", default_compile_options(), true);
    assert_is_match("a/x", "a/**", default_compile_options(), true);
    assert_is_match("a/x/y", "a/**", default_compile_options(), true);
    assert_is_match("a/x/y/z", "a/**", default_compile_options(), true);

    // pattern: a/**/*
    assert_is_match("a", "a/**/*", default_compile_options(), false);
    assert_is_match("a/", "a/**/*", default_compile_options(), false);
    assert_is_match("a/a", "a/**/*", default_compile_options(), true);
    assert_is_match("a/b", "a/**/*", default_compile_options(), true);
    assert_is_match("a/c", "a/**/*", default_compile_options(), true);
    assert_is_match("a/x", "a/**/*", default_compile_options(), true);
    assert_is_match("a/x/y", "a/**/*", default_compile_options(), true);
    assert_is_match("a/x/y/z", "a/**/*", default_compile_options(), true);

    // pattern: a/**/**/*
    assert_is_match("a", "a/**/**/*", default_compile_options(), false);
    assert_is_match("a/", "a/**/**/*", default_compile_options(), false);
    assert_is_match("a/a", "a/**/**/*", default_compile_options(), true);
    assert_is_match("a/b", "a/**/**/*", default_compile_options(), true);
    assert_is_match("a/c", "a/**/**/*", default_compile_options(), true);
    assert_is_match("a/x", "a/**/**/*", default_compile_options(), true);
    assert_is_match("a/x/y", "a/**/**/*", default_compile_options(), true);
    assert_is_match("a/x/y/z", "a/**/**/*", default_compile_options(), true);

    assert_is_match(
        "a/b/foo/bar/baz.qux",
        "a/b/**/bar/**/*.*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/bar/baz.qux",
        "a/b/**/bar/**/*.*",
        default_compile_options(),
        true,
    );
}

#[test]
fn should_support_negation_patterns() {
    assert_is_match("a/a", "!a/b", default_compile_options(), true);
    assert_is_match("a/b", "!a/b", default_compile_options(), false);
    assert_is_match("a/c", "!a/b", default_compile_options(), true);
    assert_is_match("b/a", "!a/b", default_compile_options(), true);
    assert_is_match("b/b", "!a/b", default_compile_options(), true);
    assert_is_match("b/c", "!a/b", default_compile_options(), true);

    // Array patterns: first-match semantics
    assert_is_match_any(
        "a/a",
        &["*/*", "!a/b", "!*/c"],
        default_compile_options(),
        true,
    );
    assert_is_match_any(
        "a/b",
        &["*/*", "!a/b", "!*/c"],
        default_compile_options(),
        true,
    );
    assert_is_match_any(
        "a/c",
        &["*/*", "!a/b", "!*/c"],
        default_compile_options(),
        true,
    );
    assert_is_match_any(
        "b/a",
        &["*/*", "!a/b", "!*/c"],
        default_compile_options(),
        true,
    );
    assert_is_match_any(
        "b/b",
        &["*/*", "!a/b", "!*/c"],
        default_compile_options(),
        true,
    );
    assert_is_match_any(
        "b/c",
        &["*/*", "!a/b", "!*/c"],
        default_compile_options(),
        true,
    );

    assert_is_match_any("a/a", &["!a/b", "!*/c"], default_compile_options(), true);
    assert_is_match_any("a/b", &["!a/b", "!*/c"], default_compile_options(), true);
    assert_is_match_any("a/c", &["!a/b", "!*/c"], default_compile_options(), true);
    assert_is_match_any("b/a", &["!a/b", "!*/c"], default_compile_options(), true);
    assert_is_match_any("b/b", &["!a/b", "!*/c"], default_compile_options(), true);
    assert_is_match_any("b/c", &["!a/b", "!*/c"], default_compile_options(), true);

    assert_is_match_any("a/a", &["!a/b", "!a/c"], default_compile_options(), true);
    assert_is_match_any("a/b", &["!a/b", "!a/c"], default_compile_options(), true);
    assert_is_match_any("a/c", &["!a/b", "!a/c"], default_compile_options(), true);
    assert_is_match_any("b/a", &["!a/b", "!a/c"], default_compile_options(), true);
    assert_is_match_any("b/b", &["!a/b", "!a/c"], default_compile_options(), true);
    assert_is_match_any("b/c", &["!a/b", "!a/c"], default_compile_options(), true);

    assert_is_match("a/a", "!a/(b)", default_compile_options(), true);
    assert_is_match("a/b", "!a/(b)", default_compile_options(), false);
    assert_is_match("a/c", "!a/(b)", default_compile_options(), true);
    assert_is_match("b/a", "!a/(b)", default_compile_options(), true);
    assert_is_match("b/b", "!a/(b)", default_compile_options(), true);
    assert_is_match("b/c", "!a/(b)", default_compile_options(), true);

    assert_is_match("a/a", "!(a/b)", default_compile_options(), true);
    assert_is_match("a/b", "!(a/b)", default_compile_options(), false);
    assert_is_match("a/c", "!(a/b)", default_compile_options(), true);
    assert_is_match("b/a", "!(a/b)", default_compile_options(), true);
    assert_is_match("b/b", "!(a/b)", default_compile_options(), true);
    assert_is_match("b/c", "!(a/b)", default_compile_options(), true);
}

#[test]
fn should_work_with_file_extensions() {
    assert_is_match("a.txt", "a/**/*.txt", default_compile_options(), false);
    assert_is_match("a/b.txt", "a/**/*.txt", default_compile_options(), true);
    assert_is_match("a/x/y.txt", "a/**/*.txt", default_compile_options(), true);
    assert_is_match("a/x/y/z", "a/**/*.txt", default_compile_options(), false);

    assert_is_match("a.txt", "a/*.txt", default_compile_options(), false);
    assert_is_match("a/b.txt", "a/*.txt", default_compile_options(), true);
    assert_is_match("a/x/y.txt", "a/*.txt", default_compile_options(), false);
    assert_is_match("a/x/y/z", "a/*.txt", default_compile_options(), false);

    assert_is_match("a.txt", "a*.txt", default_compile_options(), true);
    assert_is_match("a/b.txt", "a*.txt", default_compile_options(), false);
    assert_is_match("a/x/y.txt", "a*.txt", default_compile_options(), false);
    assert_is_match("a/x/y/z", "a*.txt", default_compile_options(), false);

    assert_is_match("a.txt", "*.txt", default_compile_options(), true);
    assert_is_match("a/b.txt", "*.txt", default_compile_options(), false);
    assert_is_match("a/x/y.txt", "*.txt", default_compile_options(), false);
    assert_is_match("a/x/y/z", "*.txt", default_compile_options(), false);
}

#[test]
fn should_match_one_directory_level_with_single_star() {
    // Inputs that should NOT match various patterns
    assert_is_match("/a", "*/", default_compile_options(), false);
    assert_is_match("/a", "*/*/*", default_compile_options(), false);
    assert_is_match("/a", "*/*/*/*", default_compile_options(), false);
    assert_is_match("/a", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("/a", "/*/", default_compile_options(), false);
    assert_is_match("/a", "a/*", default_compile_options(), false);
    assert_is_match("/a", "a/*/*", default_compile_options(), false);
    assert_is_match("/a", "a/*/*/*", default_compile_options(), false);
    assert_is_match("/a", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("/a", "a/*/a", default_compile_options(), false);
    assert_is_match("/a", "a/*/b", default_compile_options(), false);

    assert_is_match("/a/", "*", default_compile_options(), false);
    assert_is_match(
        "/a/",
        "**/*",
        CompileOptions {
            strict_slashes: true,
            ..Default::default()
        },
        false,
    );
    assert_is_match("/a/", "*/", default_compile_options(), false);
    assert_is_match(
        "/a/",
        "*/*",
        CompileOptions {
            strict_slashes: true,
            ..Default::default()
        },
        false,
    );
    assert_is_match("/a/", "*/*/*", default_compile_options(), false);
    assert_is_match("/a/", "*/*/*/*", default_compile_options(), false);
    assert_is_match("/a/", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match(
        "/a/",
        "/*",
        CompileOptions {
            strict_slashes: true,
            ..Default::default()
        },
        false,
    );
    assert_is_match("/a/", "a/*", default_compile_options(), false);
    assert_is_match("/a/", "a/*/*", default_compile_options(), false);
    assert_is_match("/a/", "a/*/*/*", default_compile_options(), false);
    assert_is_match("/a/", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("/a/", "a/*/a", default_compile_options(), false);
    assert_is_match("/a/", "a/*/b", default_compile_options(), false);

    assert_is_match("/ab", "*", default_compile_options(), false);
    assert_is_match("/abc", "*", default_compile_options(), false);

    assert_is_match("/b", "*", default_compile_options(), false);
    assert_is_match("/b", "*/", default_compile_options(), false);
    assert_is_match("/b", "*/*/*", default_compile_options(), false);
    assert_is_match("/b", "*/*/*/*", default_compile_options(), false);
    assert_is_match("/b", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("/b", "/*/", default_compile_options(), false);
    assert_is_match("/b", "a/*", default_compile_options(), false);
    assert_is_match("/b", "a/*/*", default_compile_options(), false);
    assert_is_match("/b", "a/*/*/*", default_compile_options(), false);
    assert_is_match("/b", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("/b", "a/*/a", default_compile_options(), false);
    assert_is_match("/b", "a/*/b", default_compile_options(), false);

    assert_is_match("a", "*/", default_compile_options(), false);
    assert_is_match("a", "*/*", default_compile_options(), false);
    assert_is_match("a", "*/*/*", default_compile_options(), false);
    assert_is_match("a", "*/*/*/*", default_compile_options(), false);
    assert_is_match("a", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("a", "/*", default_compile_options(), false);
    assert_is_match("a", "/*/", default_compile_options(), false);
    assert_is_match("a", "a/*", default_compile_options(), false);
    assert_is_match("a", "a/*/*", default_compile_options(), false);
    assert_is_match("a", "a/*/*/*", default_compile_options(), false);
    assert_is_match("a", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("a", "a/*/a", default_compile_options(), false);
    assert_is_match("a", "a/*/b", default_compile_options(), false);

    assert_is_match(
        "a/",
        "*",
        CompileOptions {
            strict_slashes: true,
            ..Default::default()
        },
        false,
    );
    assert_is_match(
        "a/",
        "**/*",
        CompileOptions {
            strict_slashes: true,
            ..Default::default()
        },
        false,
    );
    assert_is_match(
        "a/",
        "*/*",
        CompileOptions {
            strict_slashes: true,
            ..Default::default()
        },
        false,
    );
    assert_is_match(
        "a/",
        "*/*/*/*",
        CompileOptions {
            strict_slashes: true,
            ..Default::default()
        },
        false,
    );
    assert_is_match(
        "a/",
        "*/*/*/*/*",
        CompileOptions {
            strict_slashes: true,
            ..Default::default()
        },
        false,
    );
    assert_is_match(
        "a/",
        "/*",
        CompileOptions {
            strict_slashes: true,
            ..Default::default()
        },
        false,
    );
    assert_is_match(
        "a/",
        "/*/",
        CompileOptions {
            strict_slashes: true,
            ..Default::default()
        },
        false,
    );
    assert_is_match(
        "a/",
        "a/*",
        CompileOptions {
            strict_slashes: true,
            ..Default::default()
        },
        false,
    );
    assert_is_match(
        "a/",
        "a/*/*",
        CompileOptions {
            strict_slashes: true,
            ..Default::default()
        },
        false,
    );
    assert_is_match(
        "a/",
        "a/*/*/*",
        CompileOptions {
            strict_slashes: true,
            ..Default::default()
        },
        false,
    );
    assert_is_match(
        "a/",
        "a/*/*/*/*",
        CompileOptions {
            strict_slashes: true,
            ..Default::default()
        },
        false,
    );
    assert_is_match(
        "a/",
        "a/*/a",
        CompileOptions {
            strict_slashes: true,
            ..Default::default()
        },
        false,
    );
    assert_is_match(
        "a/",
        "a/*/b",
        CompileOptions {
            strict_slashes: true,
            ..Default::default()
        },
        false,
    );

    assert_is_match("a/a", "*", default_compile_options(), false);
    assert_is_match("a/a", "*/", default_compile_options(), false);
    assert_is_match("a/a", "*/*/*", default_compile_options(), false);
    assert_is_match("a/a", "*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a", "/*", default_compile_options(), false);
    assert_is_match("a/a", "/*/", default_compile_options(), false);
    assert_is_match("a/a", "a/*/*", default_compile_options(), false);
    assert_is_match("a/a", "a/*/*/*", default_compile_options(), false);
    assert_is_match("a/a", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a", "a/*/a", default_compile_options(), false);
    assert_is_match("a/a", "a/*/b", default_compile_options(), false);

    assert_is_match("a/a/a", "*", default_compile_options(), false);
    assert_is_match("a/a/a", "*/", default_compile_options(), false);
    assert_is_match("a/a/a", "*/*", default_compile_options(), false);
    assert_is_match("a/a/a", "*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a", "/*", default_compile_options(), false);
    assert_is_match("a/a/a", "/*/", default_compile_options(), false);
    assert_is_match("a/a/a", "a/*", default_compile_options(), false);
    assert_is_match("a/a/a", "a/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a", "a/*/b", default_compile_options(), false);

    assert_is_match("a/a/a/a", "*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "*/", default_compile_options(), false);
    assert_is_match("a/a/a/a", "*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "/*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a", "a/*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "a/*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "a/*/a", default_compile_options(), false);
    assert_is_match("a/a/a/a", "a/*/b", default_compile_options(), false);

    assert_is_match("a/a/a/a/a", "*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "/*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "a/*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "a/*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "a/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "a/*/a", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "a/*/b", default_compile_options(), false);

    assert_is_match("a/a/b", "*", default_compile_options(), false);
    assert_is_match("a/a/b", "*/", default_compile_options(), false);
    assert_is_match("a/a/b", "*/*", default_compile_options(), false);
    assert_is_match("a/a/b", "*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/b", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/b", "/*", default_compile_options(), false);
    assert_is_match("a/a/b", "/*/", default_compile_options(), false);
    assert_is_match("a/a/b", "a/*", default_compile_options(), false);
    assert_is_match("a/a/b", "a/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/b", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/a/b", "a/*/a", default_compile_options(), false);

    assert_is_match("a/b", "*", default_compile_options(), false);
    assert_is_match("a/b", "*/", default_compile_options(), false);
    assert_is_match("a/b", "*/*/*/*", default_compile_options(), false);
    assert_is_match("a/b", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/b", "/*", default_compile_options(), false);
    assert_is_match("a/b", "/*/", default_compile_options(), false);
    assert_is_match("a/b", "a/*/*", default_compile_options(), false);
    assert_is_match("a/b", "a/*/*/*", default_compile_options(), false);
    assert_is_match("a/b", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/b", "a/*/a", default_compile_options(), false);
    assert_is_match("a/b", "a/*/b", default_compile_options(), false);

    assert_is_match("a/c", "*", default_compile_options(), false);
    assert_is_match("a/c", "*/", default_compile_options(), false);
    assert_is_match("a/c", "*/*/*/*", default_compile_options(), false);
    assert_is_match("a/c", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/c", "/*", default_compile_options(), false);
    assert_is_match("a/c", "/*/", default_compile_options(), false);
    assert_is_match("a/c", "a/*/*", default_compile_options(), false);
    assert_is_match("a/c", "a/*/*/*", default_compile_options(), false);
    assert_is_match("a/c", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/c", "a/*/a", default_compile_options(), false);
    assert_is_match("a/c", "a/*/b", default_compile_options(), false);

    assert_is_match("a/x", "*", default_compile_options(), false);
    assert_is_match("a/x", "*/", default_compile_options(), false);
    assert_is_match("a/x", "*/*/*/*", default_compile_options(), false);
    assert_is_match("a/x", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/x", "/*", default_compile_options(), false);
    assert_is_match("a/x", "/*/", default_compile_options(), false);
    assert_is_match("a/x", "a/*/*", default_compile_options(), false);
    assert_is_match("a/x", "a/*/*/*", default_compile_options(), false);
    assert_is_match("a/x", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("a/x", "a/*/a", default_compile_options(), false);
    assert_is_match("a/x", "a/*/b", default_compile_options(), false);

    assert_is_match("b", "*/", default_compile_options(), false);
    assert_is_match("b", "*/*", default_compile_options(), false);
    assert_is_match("b", "*/*/*/*", default_compile_options(), false);
    assert_is_match("b", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("b", "/*", default_compile_options(), false);
    assert_is_match("b", "/*/", default_compile_options(), false);
    assert_is_match("b", "a/*", default_compile_options(), false);
    assert_is_match("b", "a/*/*", default_compile_options(), false);
    assert_is_match("b", "a/*/*/*", default_compile_options(), false);
    assert_is_match("b", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("b", "a/*/a", default_compile_options(), false);
    assert_is_match("b", "a/*/b", default_compile_options(), false);

    assert_is_match("x/y", "*", default_compile_options(), false);
    assert_is_match("x/y", "*/", default_compile_options(), false);
    assert_is_match("x/y", "*/*/*", default_compile_options(), false);
    assert_is_match("x/y", "*/*/*/*", default_compile_options(), false);
    assert_is_match("x/y", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("x/y", "/*", default_compile_options(), false);
    assert_is_match("x/y", "/*/", default_compile_options(), false);
    assert_is_match("x/y", "a/*", default_compile_options(), false);
    assert_is_match("x/y", "a/*/*", default_compile_options(), false);
    assert_is_match("x/y", "a/*/*/*", default_compile_options(), false);
    assert_is_match("x/y", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("x/y", "a/*/a", default_compile_options(), false);
    assert_is_match("x/y", "a/*/b", default_compile_options(), false);

    assert_is_match("z/z", "*", default_compile_options(), false);
    assert_is_match("z/z", "*/", default_compile_options(), false);
    assert_is_match("z/z", "*/*/*", default_compile_options(), false);
    assert_is_match("z/z", "*/*/*/*", default_compile_options(), false);
    assert_is_match("z/z", "*/*/*/*/*", default_compile_options(), false);
    assert_is_match("z/z", "/*", default_compile_options(), false);
    assert_is_match("z/z", "/*/", default_compile_options(), false);
    assert_is_match("z/z", "a/*", default_compile_options(), false);
    assert_is_match("z/z", "a/*/*", default_compile_options(), false);
    assert_is_match("z/z", "a/*/*/*", default_compile_options(), false);
    assert_is_match("z/z", "a/*/*/*/*", default_compile_options(), false);
    assert_is_match("z/z", "a/*/a", default_compile_options(), false);
    assert_is_match("z/z", "a/*/b", default_compile_options(), false);

    // Positive matches
    assert_is_match("/a", "**/*", default_compile_options(), true);
    assert_is_match("/a", "*/*", default_compile_options(), true);
    assert_is_match("/a", "/*", default_compile_options(), true);
    assert_is_match("/a/", "**/*{,/}", default_compile_options(), true);
    assert_is_match("/a/", "*/*", default_compile_options(), true);
    assert_is_match("/a/", "*/*{,/}", default_compile_options(), true);
    assert_is_match("/a/", "/*", default_compile_options(), true);
    assert_is_match("/a/", "/*/", default_compile_options(), true);
    assert_is_match("/a/", "/*{,/}", default_compile_options(), true);
    assert_is_match("/b", "**/*", default_compile_options(), true);
    assert_is_match("/b", "*/*", default_compile_options(), true);
    assert_is_match("/b", "/*", default_compile_options(), true);
    assert_is_match("a", "*", default_compile_options(), true);
    assert_is_match("a", "**/*", default_compile_options(), true);
    assert_is_match("a/", "**/*{,/}", default_compile_options(), true);
    assert_is_match("a/", "*/", default_compile_options(), true);
    assert_is_match("a/", "*{,/}", default_compile_options(), true);
    assert_is_match("a/a", "**/*", default_compile_options(), true);
    assert_is_match("a/a", "*/*", default_compile_options(), true);
    assert_is_match("a/a", "a/*", default_compile_options(), true);
    assert_is_match("a/a/a", "**/*", default_compile_options(), true);
    assert_is_match("a/a/a", "*/*/*", default_compile_options(), true);
    assert_is_match("a/a/a", "a/*/*", default_compile_options(), true);
    assert_is_match("a/a/a", "a/*/a", default_compile_options(), true);
    assert_is_match("a/a/a/a", "**/*", default_compile_options(), true);
    assert_is_match("a/a/a/a", "*/*/*/*", default_compile_options(), true);
    assert_is_match("a/a/a/a", "a/*/*/*", default_compile_options(), true);
    assert_is_match("a/a/a/a/a", "**/*", default_compile_options(), true);
    assert_is_match("a/a/a/a/a", "*/*/*/*/*", default_compile_options(), true);
    assert_is_match("a/a/a/a/a", "a/*/*/*/*", default_compile_options(), true);
    assert_is_match("a/a/b", "**/*", default_compile_options(), true);
    assert_is_match("a/a/b", "a/*/*", default_compile_options(), true);
    assert_is_match("a/a/b", "a/*/b", default_compile_options(), true);
    assert_is_match("a/b", "**/*", default_compile_options(), true);
    assert_is_match("a/b", "*/*", default_compile_options(), true);
    assert_is_match("a/b", "a/*", default_compile_options(), true);
    assert_is_match("a/c", "**/*", default_compile_options(), true);
    assert_is_match("a/c", "*/*", default_compile_options(), true);
    assert_is_match("a/c", "a/*", default_compile_options(), true);
    assert_is_match("a/x", "**/*", default_compile_options(), true);
    assert_is_match("a/x", "*/*", default_compile_options(), true);
    assert_is_match("a/x", "a/*", default_compile_options(), true);
    assert_is_match("b", "*", default_compile_options(), true);
    assert_is_match("b", "**/*", default_compile_options(), true);
    assert_is_match("x/y", "**/*", default_compile_options(), true);
    assert_is_match("x/y", "*/*", default_compile_options(), true);
    assert_is_match("z/z", "**/*", default_compile_options(), true);
    assert_is_match("z/z", "*/*", default_compile_options(), true);
}

#[test]
fn should_match_one_or_more_directories_with_globstar() {
    assert_is_match("a/", "**/a", default_compile_options(), false);
    assert_is_match("/a/", "**/a", default_compile_options(), false);
    assert_is_match("a/a/", "**/a", default_compile_options(), false);
    assert_is_match("/a/a/", "**/a", default_compile_options(), false);
    assert_is_match("a/a/a/", "**/a", default_compile_options(), false);

    assert_is_match("a", "**/a", default_compile_options(), true);
    assert_is_match("a/a", "**/a", default_compile_options(), true);
    assert_is_match("a/a/a", "**/a", default_compile_options(), true);
    assert_is_match("/a", "**/a", default_compile_options(), true);
    assert_is_match("a/a/", "**/a/*{,/}", default_compile_options(), true);
    assert_is_match(
        "a/a/",
        "**/a/*",
        CompileOptions {
            strict_slashes: true,
            ..Default::default()
        },
        false,
    );
    assert_is_match("/a/a", "**/a", default_compile_options(), true);

    assert_is_match("a", "a/**", default_compile_options(), true);
    assert_is_match("/a", "a/**", default_compile_options(), false);
    assert_is_match("/a/", "a/**", default_compile_options(), false);
    assert_is_match("/a/a", "a/**", default_compile_options(), false);
    assert_is_match("/a/a/", "a/**", default_compile_options(), false);
    assert_is_match("/a", "/a/**", default_compile_options(), true);
    assert_is_match("/a/", "/a/**", default_compile_options(), true);
    assert_is_match("/a/a", "/a/**", default_compile_options(), true);
    assert_is_match("/a/a/", "/a/**", default_compile_options(), true);
    assert_is_match("a/", "a/**", default_compile_options(), true);
    assert_is_match("a/a", "a/**", default_compile_options(), true);
    assert_is_match("a/a/", "a/**", default_compile_options(), true);
    assert_is_match("a/a/a", "a/**", default_compile_options(), true);
    assert_is_match("a/a/a/", "a/**", default_compile_options(), true);

    assert_is_match("a", "**/a/**", default_compile_options(), true);
    assert_is_match("/a", "**/a/**", default_compile_options(), true);
    assert_is_match("/a/", "**/a/**", default_compile_options(), true);
    assert_is_match("/a/a", "**/a/**", default_compile_options(), true);
    assert_is_match("/a/a/", "**/a/**", default_compile_options(), true);
    assert_is_match("a/", "**/a/**", default_compile_options(), true);
    assert_is_match("a/a", "**/a/**", default_compile_options(), true);
    assert_is_match("a/a/", "**/a/**", default_compile_options(), true);
    assert_is_match("a/a/a", "**/a/**", default_compile_options(), true);
    assert_is_match("a/a/a/", "**/a/**", default_compile_options(), true);
}

#[test]
fn should_match_one_or_more_characters() {
    // pattern: *
    assert_is_match("a", "*", default_compile_options(), true);
    assert_is_match("aa", "*", default_compile_options(), true);
    assert_is_match("aaa", "*", default_compile_options(), true);
    assert_is_match("aaaa", "*", default_compile_options(), true);
    assert_is_match("ab", "*", default_compile_options(), true);
    assert_is_match("b", "*", default_compile_options(), true);
    assert_is_match("bb", "*", default_compile_options(), true);
    assert_is_match("c", "*", default_compile_options(), true);
    assert_is_match("cc", "*", default_compile_options(), true);
    assert_is_match("cac", "*", default_compile_options(), true);
    assert_is_match("a/a", "*", default_compile_options(), false);
    assert_is_match("a/b", "*", default_compile_options(), false);
    assert_is_match("a/c", "*", default_compile_options(), false);
    assert_is_match("a/x", "*", default_compile_options(), false);
    assert_is_match("a/a/a", "*", default_compile_options(), false);
    assert_is_match("a/a/b", "*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "*", default_compile_options(), false);
    assert_is_match("x/y", "*", default_compile_options(), false);
    assert_is_match("z/z", "*", default_compile_options(), false);

    // pattern: a*
    assert_is_match("a", "a*", default_compile_options(), true);
    assert_is_match("aa", "a*", default_compile_options(), true);
    assert_is_match("aaa", "a*", default_compile_options(), true);
    assert_is_match("aaaa", "a*", default_compile_options(), true);
    assert_is_match("ab", "a*", default_compile_options(), true);
    assert_is_match("b", "a*", default_compile_options(), false);
    assert_is_match("bb", "a*", default_compile_options(), false);
    assert_is_match("c", "a*", default_compile_options(), false);
    assert_is_match("cc", "a*", default_compile_options(), false);
    assert_is_match("cac", "a*", default_compile_options(), false);
    assert_is_match("a/a", "a*", default_compile_options(), false);
    assert_is_match("a/b", "a*", default_compile_options(), false);
    assert_is_match("a/c", "a*", default_compile_options(), false);
    assert_is_match("a/x", "a*", default_compile_options(), false);
    assert_is_match("a/a/a", "a*", default_compile_options(), false);
    assert_is_match("a/a/b", "a*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "a*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "a*", default_compile_options(), false);
    assert_is_match("x/y", "a*", default_compile_options(), false);
    assert_is_match("z/z", "a*", default_compile_options(), false);

    // pattern: *b
    assert_is_match("a", "*b", default_compile_options(), false);
    assert_is_match("aa", "*b", default_compile_options(), false);
    assert_is_match("aaa", "*b", default_compile_options(), false);
    assert_is_match("aaaa", "*b", default_compile_options(), false);
    assert_is_match("ab", "*b", default_compile_options(), true);
    assert_is_match("b", "*b", default_compile_options(), true);
    assert_is_match("bb", "*b", default_compile_options(), true);
    assert_is_match("c", "*b", default_compile_options(), false);
    assert_is_match("cc", "*b", default_compile_options(), false);
    assert_is_match("cac", "*b", default_compile_options(), false);
    assert_is_match("a/a", "*b", default_compile_options(), false);
    assert_is_match("a/b", "*b", default_compile_options(), false);
    assert_is_match("a/c", "*b", default_compile_options(), false);
    assert_is_match("a/x", "*b", default_compile_options(), false);
    assert_is_match("a/a/a", "*b", default_compile_options(), false);
    assert_is_match("a/a/b", "*b", default_compile_options(), false);
    assert_is_match("a/a/a/a", "*b", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "*b", default_compile_options(), false);
    assert_is_match("x/y", "*b", default_compile_options(), false);
    assert_is_match("z/z", "*b", default_compile_options(), false);
}

#[test]
fn should_match_one_or_zero_characters() {
    // pattern: *
    assert_is_match("a", "*", default_compile_options(), true);
    assert_is_match("aa", "*", default_compile_options(), true);
    assert_is_match("aaa", "*", default_compile_options(), true);
    assert_is_match("aaaa", "*", default_compile_options(), true);
    assert_is_match("ab", "*", default_compile_options(), true);
    assert_is_match("b", "*", default_compile_options(), true);
    assert_is_match("bb", "*", default_compile_options(), true);
    assert_is_match("c", "*", default_compile_options(), true);
    assert_is_match("cc", "*", default_compile_options(), true);
    assert_is_match("cac", "*", default_compile_options(), true);
    assert_is_match("a/a", "*", default_compile_options(), false);
    assert_is_match("a/b", "*", default_compile_options(), false);
    assert_is_match("a/c", "*", default_compile_options(), false);
    assert_is_match("a/x", "*", default_compile_options(), false);
    assert_is_match("a/a/a", "*", default_compile_options(), false);
    assert_is_match("a/a/b", "*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "*", default_compile_options(), false);
    assert_is_match("x/y", "*", default_compile_options(), false);
    assert_is_match("z/z", "*", default_compile_options(), false);

    // pattern: *a*
    assert_is_match("a", "*a*", default_compile_options(), true);
    assert_is_match("aa", "*a*", default_compile_options(), true);
    assert_is_match("aaa", "*a*", default_compile_options(), true);
    assert_is_match("aaaa", "*a*", default_compile_options(), true);
    assert_is_match("ab", "*a*", default_compile_options(), true);
    assert_is_match("b", "*a*", default_compile_options(), false);
    assert_is_match("bb", "*a*", default_compile_options(), false);
    assert_is_match("c", "*a*", default_compile_options(), false);
    assert_is_match("cc", "*a*", default_compile_options(), false);
    assert_is_match("cac", "*a*", default_compile_options(), true);
    assert_is_match("a/a", "*a*", default_compile_options(), false);
    assert_is_match("a/b", "*a*", default_compile_options(), false);
    assert_is_match("a/c", "*a*", default_compile_options(), false);
    assert_is_match("a/x", "*a*", default_compile_options(), false);
    assert_is_match("a/a/a", "*a*", default_compile_options(), false);
    assert_is_match("a/a/b", "*a*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "*a*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "*a*", default_compile_options(), false);
    assert_is_match("x/y", "*a*", default_compile_options(), false);
    assert_is_match("z/z", "*a*", default_compile_options(), false);

    // pattern: *b*
    assert_is_match("a", "*b*", default_compile_options(), false);
    assert_is_match("aa", "*b*", default_compile_options(), false);
    assert_is_match("aaa", "*b*", default_compile_options(), false);
    assert_is_match("aaaa", "*b*", default_compile_options(), false);
    assert_is_match("ab", "*b*", default_compile_options(), true);
    assert_is_match("b", "*b*", default_compile_options(), true);
    assert_is_match("bb", "*b*", default_compile_options(), true);
    assert_is_match("c", "*b*", default_compile_options(), false);
    assert_is_match("cc", "*b*", default_compile_options(), false);
    assert_is_match("cac", "*b*", default_compile_options(), false);
    assert_is_match("a/a", "*b*", default_compile_options(), false);
    assert_is_match("a/b", "*b*", default_compile_options(), false);
    assert_is_match("a/c", "*b*", default_compile_options(), false);
    assert_is_match("a/x", "*b*", default_compile_options(), false);
    assert_is_match("a/a/a", "*b*", default_compile_options(), false);
    assert_is_match("a/a/b", "*b*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "*b*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "*b*", default_compile_options(), false);
    assert_is_match("x/y", "*b*", default_compile_options(), false);
    assert_is_match("z/z", "*b*", default_compile_options(), false);

    // pattern: *c*
    assert_is_match("a", "*c*", default_compile_options(), false);
    assert_is_match("aa", "*c*", default_compile_options(), false);
    assert_is_match("aaa", "*c*", default_compile_options(), false);
    assert_is_match("aaaa", "*c*", default_compile_options(), false);
    assert_is_match("ab", "*c*", default_compile_options(), false);
    assert_is_match("b", "*c*", default_compile_options(), false);
    assert_is_match("bb", "*c*", default_compile_options(), false);
    assert_is_match("c", "*c*", default_compile_options(), true);
    assert_is_match("cc", "*c*", default_compile_options(), true);
    assert_is_match("cac", "*c*", default_compile_options(), true);
    assert_is_match("a/a", "*c*", default_compile_options(), false);
    assert_is_match("a/b", "*c*", default_compile_options(), false);
    assert_is_match("a/c", "*c*", default_compile_options(), false);
    assert_is_match("a/x", "*c*", default_compile_options(), false);
    assert_is_match("a/a/a", "*c*", default_compile_options(), false);
    assert_is_match("a/a/b", "*c*", default_compile_options(), false);
    assert_is_match("a/a/a/a", "*c*", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "*c*", default_compile_options(), false);
    assert_is_match("x/y", "*c*", default_compile_options(), false);
    assert_is_match("z/z", "*c*", default_compile_options(), false);
}

#[test]
fn should_respect_trailing_slashes_on_patterns() {
    // pattern: */
    assert_is_match("a", "*/", default_compile_options(), false);
    assert_is_match("a/", "*/", default_compile_options(), true);
    assert_is_match("b", "*/", default_compile_options(), false);
    assert_is_match("b/", "*/", default_compile_options(), true);
    assert_is_match("a/a", "*/", default_compile_options(), false);
    assert_is_match("a/a/", "*/", default_compile_options(), false);
    assert_is_match("a/b", "*/", default_compile_options(), false);
    assert_is_match("a/b/", "*/", default_compile_options(), false);
    assert_is_match("a/c", "*/", default_compile_options(), false);
    assert_is_match("a/c/", "*/", default_compile_options(), false);
    assert_is_match("a/x", "*/", default_compile_options(), false);
    assert_is_match("a/x/", "*/", default_compile_options(), false);
    assert_is_match("a/a/a", "*/", default_compile_options(), false);
    assert_is_match("a/a/b", "*/", default_compile_options(), false);
    assert_is_match("a/a/b/", "*/", default_compile_options(), false);
    assert_is_match("a/a/a/", "*/", default_compile_options(), false);
    assert_is_match("a/a/a/a", "*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/", "*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a/", "*/", default_compile_options(), false);
    assert_is_match("x/y", "*/", default_compile_options(), false);
    assert_is_match("z/z", "*/", default_compile_options(), false);
    assert_is_match("x/y/", "*/", default_compile_options(), false);
    assert_is_match("z/z/", "*/", default_compile_options(), false);
    assert_is_match("a/b/c/.d/e/", "*/", default_compile_options(), false);

    // pattern: */*/
    assert_is_match("a", "*/*/", default_compile_options(), false);
    assert_is_match("a/", "*/*/", default_compile_options(), false);
    assert_is_match("b", "*/*/", default_compile_options(), false);
    assert_is_match("b/", "*/*/", default_compile_options(), false);
    assert_is_match("a/a", "*/*/", default_compile_options(), false);
    assert_is_match("a/a/", "*/*/", default_compile_options(), true);
    assert_is_match("a/b", "*/*/", default_compile_options(), false);
    assert_is_match("a/b/", "*/*/", default_compile_options(), true);
    assert_is_match("a/c", "*/*/", default_compile_options(), false);
    assert_is_match("a/c/", "*/*/", default_compile_options(), true);
    assert_is_match("a/x", "*/*/", default_compile_options(), false);
    assert_is_match("a/x/", "*/*/", default_compile_options(), true);
    assert_is_match("a/a/a", "*/*/", default_compile_options(), false);
    assert_is_match("a/a/b", "*/*/", default_compile_options(), false);
    assert_is_match("a/a/b/", "*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/", "*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a", "*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/", "*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a/", "*/*/", default_compile_options(), false);
    assert_is_match("x/y", "*/*/", default_compile_options(), false);
    assert_is_match("z/z", "*/*/", default_compile_options(), false);
    assert_is_match("x/y/", "*/*/", default_compile_options(), true);
    assert_is_match("z/z/", "*/*/", default_compile_options(), true);
    assert_is_match("a/b/c/.d/e/", "*/*/", default_compile_options(), false);

    // pattern: */*/*/
    assert_is_match("a", "*/*/*/", default_compile_options(), false);
    assert_is_match("a/", "*/*/*/", default_compile_options(), false);
    assert_is_match("b", "*/*/*/", default_compile_options(), false);
    assert_is_match("b/", "*/*/*/", default_compile_options(), false);
    assert_is_match("a/a", "*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/", "*/*/*/", default_compile_options(), false);
    assert_is_match("a/b", "*/*/*/", default_compile_options(), false);
    assert_is_match("a/b/", "*/*/*/", default_compile_options(), false);
    assert_is_match("a/c", "*/*/*/", default_compile_options(), false);
    assert_is_match("a/c/", "*/*/*/", default_compile_options(), false);
    assert_is_match("a/x", "*/*/*/", default_compile_options(), false);
    assert_is_match("a/x/", "*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a", "*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/b", "*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/b/", "*/*/*/", default_compile_options(), true);
    assert_is_match("a/a/a/", "*/*/*/", default_compile_options(), true);
    assert_is_match("a/a/a/a", "*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/", "*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a/", "*/*/*/", default_compile_options(), false);
    assert_is_match("x/y", "*/*/*/", default_compile_options(), false);
    assert_is_match("z/z", "*/*/*/", default_compile_options(), false);
    assert_is_match("x/y/", "*/*/*/", default_compile_options(), false);
    assert_is_match("z/z/", "*/*/*/", default_compile_options(), false);
    assert_is_match("a/b/c/.d/e/", "*/*/*/", default_compile_options(), false);

    // pattern: */*/*/*/
    assert_is_match("a", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("b", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("b/", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/b", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/b/", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/c", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/c/", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/x", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/x/", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/b", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/b/", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/", "*/*/*/*/", default_compile_options(), true);
    assert_is_match("a/a/a/a/a", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a/", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("x/y", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("z/z", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("x/y/", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("z/z/", "*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/b/c/.d/e/", "*/*/*/*/", default_compile_options(), false);

    // pattern: */*/*/*/*/
    assert_is_match("a", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("b", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("b/", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/b", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/b/", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/c", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/c/", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/x", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/x/", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/b", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/b/", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a/", "*/*/*/*/*/", default_compile_options(), true);
    assert_is_match("x/y", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("z/z", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("x/y/", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match("z/z/", "*/*/*/*/*/", default_compile_options(), false);
    assert_is_match(
        "a/b/c/.d/e/",
        "*/*/*/*/*/",
        default_compile_options(),
        false,
    );

    // pattern: a/*/
    assert_is_match("a", "a/*/", default_compile_options(), false);
    assert_is_match("a/", "a/*/", default_compile_options(), false);
    assert_is_match("b", "a/*/", default_compile_options(), false);
    assert_is_match("b/", "a/*/", default_compile_options(), false);
    assert_is_match("a/a", "a/*/", default_compile_options(), false);
    assert_is_match("a/a/", "a/*/", default_compile_options(), true);
    assert_is_match("a/b", "a/*/", default_compile_options(), false);
    assert_is_match("a/b/", "a/*/", default_compile_options(), true);
    assert_is_match("a/c", "a/*/", default_compile_options(), false);
    assert_is_match("a/c/", "a/*/", default_compile_options(), true);
    assert_is_match("a/x", "a/*/", default_compile_options(), false);
    assert_is_match("a/x/", "a/*/", default_compile_options(), true);
    assert_is_match("a/a/a", "a/*/", default_compile_options(), false);
    assert_is_match("a/a/b", "a/*/", default_compile_options(), false);
    assert_is_match("a/a/b/", "a/*/", default_compile_options(), false);
    assert_is_match("a/a/a/", "a/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a", "a/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/", "a/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "a/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a/", "a/*/", default_compile_options(), false);
    assert_is_match("x/y", "a/*/", default_compile_options(), false);
    assert_is_match("z/z", "a/*/", default_compile_options(), false);
    assert_is_match("x/y/", "a/*/", default_compile_options(), false);
    assert_is_match("z/z/", "a/*/", default_compile_options(), false);
    assert_is_match("a/b/c/.d/e/", "a/*/", default_compile_options(), false);

    // pattern: a/*/*/
    assert_is_match("a", "a/*/*/", default_compile_options(), false);
    assert_is_match("a/", "a/*/*/", default_compile_options(), false);
    assert_is_match("b", "a/*/*/", default_compile_options(), false);
    assert_is_match("b/", "a/*/*/", default_compile_options(), false);
    assert_is_match("a/a", "a/*/*/", default_compile_options(), false);
    assert_is_match("a/a/", "a/*/*/", default_compile_options(), false);
    assert_is_match("a/b", "a/*/*/", default_compile_options(), false);
    assert_is_match("a/b/", "a/*/*/", default_compile_options(), false);
    assert_is_match("a/c", "a/*/*/", default_compile_options(), false);
    assert_is_match("a/c/", "a/*/*/", default_compile_options(), false);
    assert_is_match("a/x", "a/*/*/", default_compile_options(), false);
    assert_is_match("a/x/", "a/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a", "a/*/*/", default_compile_options(), false);
    assert_is_match("a/a/b", "a/*/*/", default_compile_options(), false);
    assert_is_match("a/a/b/", "a/*/*/", default_compile_options(), true);
    assert_is_match("a/a/a/", "a/*/*/", default_compile_options(), true);
    assert_is_match("a/a/a/a", "a/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/", "a/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "a/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a/", "a/*/*/", default_compile_options(), false);
    assert_is_match("x/y", "a/*/*/", default_compile_options(), false);
    assert_is_match("z/z", "a/*/*/", default_compile_options(), false);
    assert_is_match("x/y/", "a/*/*/", default_compile_options(), false);
    assert_is_match("z/z/", "a/*/*/", default_compile_options(), false);
    assert_is_match("a/b/c/.d/e/", "a/*/*/", default_compile_options(), false);

    // pattern: a/*/*/*/
    assert_is_match("a", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("a/", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("b", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("b/", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("a/b", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("a/b/", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("a/c", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("a/c/", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("a/x", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("a/x/", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/b", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/b/", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/", "a/*/*/*/", default_compile_options(), true);
    assert_is_match("a/a/a/a/a", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a/", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("x/y", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("z/z", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("x/y/", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("z/z/", "a/*/*/*/", default_compile_options(), false);
    assert_is_match("a/b/c/.d/e/", "a/*/*/*/", default_compile_options(), false);

    // pattern: a/*/*/*/*/
    assert_is_match("a", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("b", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("b/", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/b", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/b/", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/c", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/c/", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/x", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/x/", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/b", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/b/", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a/", "a/*/*/*/*/", default_compile_options(), true);
    assert_is_match("x/y", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("z/z", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("x/y/", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match("z/z/", "a/*/*/*/*/", default_compile_options(), false);
    assert_is_match(
        "a/b/c/.d/e/",
        "a/*/*/*/*/",
        default_compile_options(),
        false,
    );

    // pattern: a/*/a/
    assert_is_match("a", "a/*/a/", default_compile_options(), false);
    assert_is_match("a/", "a/*/a/", default_compile_options(), false);
    assert_is_match("b", "a/*/a/", default_compile_options(), false);
    assert_is_match("b/", "a/*/a/", default_compile_options(), false);
    assert_is_match("a/a", "a/*/a/", default_compile_options(), false);
    assert_is_match("a/a/", "a/*/a/", default_compile_options(), false);
    assert_is_match("a/b", "a/*/a/", default_compile_options(), false);
    assert_is_match("a/b/", "a/*/a/", default_compile_options(), false);
    assert_is_match("a/c", "a/*/a/", default_compile_options(), false);
    assert_is_match("a/c/", "a/*/a/", default_compile_options(), false);
    assert_is_match("a/x", "a/*/a/", default_compile_options(), false);
    assert_is_match("a/x/", "a/*/a/", default_compile_options(), false);
    assert_is_match("a/a/a", "a/*/a/", default_compile_options(), false);
    assert_is_match("a/a/b", "a/*/a/", default_compile_options(), false);
    assert_is_match("a/a/b/", "a/*/a/", default_compile_options(), false);
    assert_is_match("a/a/a/", "a/*/a/", default_compile_options(), true);
    assert_is_match("a/a/a/a", "a/*/a/", default_compile_options(), false);
    assert_is_match("a/a/a/a/", "a/*/a/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "a/*/a/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a/", "a/*/a/", default_compile_options(), false);
    assert_is_match("x/y", "a/*/a/", default_compile_options(), false);
    assert_is_match("z/z", "a/*/a/", default_compile_options(), false);
    assert_is_match("x/y/", "a/*/a/", default_compile_options(), false);
    assert_is_match("z/z/", "a/*/a/", default_compile_options(), false);
    assert_is_match("a/b/c/.d/e/", "a/*/a/", default_compile_options(), false);

    // pattern: a/*/b/
    assert_is_match("a", "a/*/b/", default_compile_options(), false);
    assert_is_match("a/", "a/*/b/", default_compile_options(), false);
    assert_is_match("b", "a/*/b/", default_compile_options(), false);
    assert_is_match("b/", "a/*/b/", default_compile_options(), false);
    assert_is_match("a/a", "a/*/b/", default_compile_options(), false);
    assert_is_match("a/a/", "a/*/b/", default_compile_options(), false);
    assert_is_match("a/b", "a/*/b/", default_compile_options(), false);
    assert_is_match("a/b/", "a/*/b/", default_compile_options(), false);
    assert_is_match("a/c", "a/*/b/", default_compile_options(), false);
    assert_is_match("a/c/", "a/*/b/", default_compile_options(), false);
    assert_is_match("a/x", "a/*/b/", default_compile_options(), false);
    assert_is_match("a/x/", "a/*/b/", default_compile_options(), false);
    assert_is_match("a/a/a", "a/*/b/", default_compile_options(), false);
    assert_is_match("a/a/b", "a/*/b/", default_compile_options(), false);
    assert_is_match("a/a/b/", "a/*/b/", default_compile_options(), true);
    assert_is_match("a/a/a/", "a/*/b/", default_compile_options(), false);
    assert_is_match("a/a/a/a", "a/*/b/", default_compile_options(), false);
    assert_is_match("a/a/a/a/", "a/*/b/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a", "a/*/b/", default_compile_options(), false);
    assert_is_match("a/a/a/a/a/", "a/*/b/", default_compile_options(), false);
    assert_is_match("x/y", "a/*/b/", default_compile_options(), false);
    assert_is_match("z/z", "a/*/b/", default_compile_options(), false);
    assert_is_match("x/y/", "a/*/b/", default_compile_options(), false);
    assert_is_match("z/z/", "a/*/b/", default_compile_options(), false);
    assert_is_match("a/b/c/.d/e/", "a/*/b/", default_compile_options(), false);
}

#[test]
fn should_match_literal_star_when_escaped() {
    // pattern: \*
    assert_is_match(".md", "\\*", default_compile_options(), false);
    assert_is_match("a**a.md", "\\*", default_compile_options(), false);
    assert_is_match("**a.md", "\\*", default_compile_options(), false);
    assert_is_match("**/a.md", "\\*", default_compile_options(), false);
    assert_is_match("**.md", "\\*", default_compile_options(), false);
    assert_is_match("*", "\\*", default_compile_options(), true);
    assert_is_match("**", "\\*", default_compile_options(), false);
    assert_is_match("*.md", "\\*", default_compile_options(), false);

    // pattern: \*.md
    assert_is_match(".md", "\\*.md", default_compile_options(), false);
    assert_is_match("a**a.md", "\\*.md", default_compile_options(), false);
    assert_is_match("**a.md", "\\*.md", default_compile_options(), false);
    assert_is_match("**/a.md", "\\*.md", default_compile_options(), false);
    assert_is_match("**.md", "\\*.md", default_compile_options(), false);
    assert_is_match("*", "\\*.md", default_compile_options(), false);
    assert_is_match("**", "\\*.md", default_compile_options(), false);
    assert_is_match("*.md", "\\*.md", default_compile_options(), true);

    // pattern: \**.md
    assert_is_match(".md", "\\**.md", default_compile_options(), false);
    assert_is_match("a**a.md", "\\**.md", default_compile_options(), false);
    assert_is_match("**a.md", "\\**.md", default_compile_options(), true);
    assert_is_match("**/a.md", "\\**.md", default_compile_options(), false);
    assert_is_match("**.md", "\\**.md", default_compile_options(), true);
    assert_is_match("*", "\\**.md", default_compile_options(), false);
    assert_is_match("**", "\\**.md", default_compile_options(), false);
    assert_is_match("*.md", "\\**.md", default_compile_options(), true);

    // pattern: a\**.md
    assert_is_match(".md", "a\\**.md", default_compile_options(), false);
    assert_is_match("a**a.md", "a\\**.md", default_compile_options(), true);
    assert_is_match("**a.md", "a\\**.md", default_compile_options(), false);
    assert_is_match("**/a.md", "a\\**.md", default_compile_options(), false);
    assert_is_match("**.md", "a\\**.md", default_compile_options(), false);
    assert_is_match("*", "a\\**.md", default_compile_options(), false);
    assert_is_match("**", "a\\**.md", default_compile_options(), false);
    assert_is_match("*.md", "a\\**.md", default_compile_options(), false);
}

#[test]
fn should_match_file_paths() {
    assert_is_match("a/.b", "a/**/z/*.md", default_compile_options(), false);
    assert_is_match(
        "a/b/c/j/e/z/c.txt",
        "a/**/j/**/z/*.md",
        default_compile_options(),
        false,
    );
    assert_is_match("a/b/z/.a", "a/**/z/*.a", default_compile_options(), false);
    assert_is_match("a/b/z/.a", "a/*/z/*.a", default_compile_options(), false);
    assert_is_match("foo.txt", "*/*.txt", default_compile_options(), false);
    assert_is_match("a/.b", "a/.*", default_compile_options(), true);
    assert_is_match(
        "a/b/c/d/e/j/n/p/o/z/c.md",
        "a/**/j/**/z/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/e/z/c.md",
        "a/**/z/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/xyz.md",
        "a/b/c/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match("a/b/z/.a", "a/*/z/.a", default_compile_options(), true);
    assert_is_match(
        "a/bb.bb/aa/b.b/aa/c/xyz.md",
        "a/**/c/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/bb.bb/aa/bb/aa/c/xyz.md",
        "a/**/c/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/bb.bb/c/xyz.md",
        "a/*/c/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/bb/c/xyz.md",
        "a/*/c/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/bbbb/c/xyz.md",
        "a/*/c/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match("foo.txt", "**/foo.txt", default_compile_options(), true);
    assert_is_match("foo/bar.txt", "**/*.txt", default_compile_options(), true);
    assert_is_match(
        "foo/bar/baz.txt",
        "**/*.txt",
        default_compile_options(),
        true,
    );
}

/// NOTE: The `format` option (a JS function) is not available in Rust.
/// These tests use `format: str => str.replace(/^\.\//, '')` to strip leading `./`
/// from the input before matching. They are listed here for completeness but
/// cannot be directly ported without format option support.
///
/// Original JS tests:
/// assert(!isMatch('./a/b/c/d/e/z/c.md', './a/**/j/**/z/*.md', { format }));
/// assert(!isMatch('./a/b/c/j/e/z/c.txt', './a/**/j/**/z/*.md', { format }));
/// assert(isMatch('./a/b/c/d/e/j/n/p/o/z/c.md', './a/**/j/**/z/*.md', { format }));
/// assert(isMatch('./a/b/c/d/e/z/c.md', './a/**/z/*.md', { format }));
/// assert(isMatch('./a/b/c/j/e/z/c.md', './a/**/j/**/z/*.md', { format }));
/// assert(isMatch('./a/b/z/.a', './a/**/z/.a', { format }));
#[test]
fn should_match_paths_with_leading_dot_slash() {
    // The format option strips leading "./" from the input before matching.
    // Emulate by stripping "./" manually from both input and pattern.
    assert_is_match(
        "a/b/c/d/e/z/c.md",
        "a/**/j/**/z/*.md",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "a/b/c/j/e/z/c.txt",
        "a/**/j/**/z/*.md",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "a/b/c/d/e/j/n/p/o/z/c.md",
        "a/**/j/**/z/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/e/z/c.md",
        "a/**/z/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/j/e/z/c.md",
        "a/**/j/**/z/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match("a/b/z/.a", "a/**/z/.a", default_compile_options(), true);
}

#[test]
fn should_match_leading_slashes() {
    assert_is_match("ef", "/*", default_compile_options(), false);
    assert_is_match("/ef", "/*", default_compile_options(), true);
    assert_is_match("/foo/bar.txt", "/foo/*", default_compile_options(), true);
    assert_is_match("/foo/bar.txt", "/foo/**", default_compile_options(), true);
    assert_is_match(
        "/foo/bar.txt",
        "/foo/**/**/*.txt",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "/foo/bar.txt",
        "/foo/**/**/bar.txt",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "/foo/bar.txt",
        "/foo/**/*.txt",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "/foo/bar.txt",
        "/foo/**/bar.txt",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "/foo/bar.txt",
        "/foo/*/bar.txt",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "/foo/bar/baz.txt",
        "/foo/*",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "/foo/bar/baz.txt",
        "/foo/**",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "/foo/bar/baz.txt",
        "/foo/**/*.txt",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "/foo/bar/baz.txt",
        "/foo/**/*/*.txt",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "/foo/bar/baz.txt",
        "/foo/**/*/baz.txt",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "/foo/bar/baz.txt",
        "/foo/*.txt",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "/foo/bar/baz.txt",
        "/foo/*/*.txt",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "/foo/bar/baz.txt",
        "/foo/*/*/baz.txt",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "/foo/bar/baz.txt",
        "/foo/bar**",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "/foo/bar/baz/qux.txt",
        "**/*.txt",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "/foo/bar/baz/qux.txt",
        "**/.txt",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "/foo/bar/baz/qux.txt",
        "*/*.txt",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "/foo/bar/baz/qux.txt",
        "/foo/**.txt",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "/foo/bar/baz/qux.txt",
        "/foo/**/*.txt",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "/foo/bar/baz/qux.txt",
        "/foo/*/*.txt",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "/foo/bar/baz/qux.txt",
        "/foo/bar**/*.txt",
        default_compile_options(),
        false,
    );
    assert_is_match("/.txt", "*.txt", default_compile_options(), false);
    assert_is_match("/.txt", "/*.txt", default_compile_options(), false);
    assert_is_match("/.txt", "*/*.txt", default_compile_options(), false);
    assert_is_match("/.txt", "**/*.txt", default_compile_options(), false);
    assert_is_match(
        "/.txt",
        "*.txt",
        CompileOptions {
            dot: true,
            ..Default::default()
        },
        false,
    );
    assert_is_match(
        "/.txt",
        "/*.txt",
        CompileOptions {
            dot: true,
            ..Default::default()
        },
        true,
    );
    assert_is_match(
        "/.txt",
        "*/*.txt",
        CompileOptions {
            dot: true,
            ..Default::default()
        },
        true,
    );
    assert_is_match(
        "/.txt",
        "**/*.txt",
        CompileOptions {
            dot: true,
            ..Default::default()
        },
        true,
    );
}

#[test]
fn should_match_double_slashes() {
    assert_is_match(
        "https://foo.com/bar/baz/app.min.js",
        "https://foo.com/*",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "https://foo.com/bar/baz/app.min.js",
        "https://foo.com/**",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "https://foo.com/bar/baz/app.min.js",
        "https://foo.com/**",
        CompileOptions {
            noglobstar: true,
            ..Default::default()
        },
        false,
    );
    assert_is_match(
        "https://foo.com/bar/baz/app.min.js",
        "https://foo.com/**/app.min.js",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "https://foo.com/bar/baz/app.min.js",
        "https://foo.com/*/*/app.min.js",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "https://foo.com/bar/baz/app.min.js",
        "https://foo.com/*/*/app.min.js",
        CompileOptions {
            noglobstar: true,
            ..Default::default()
        },
        true,
    );
    assert_is_match(
        "https://foo.com/bar/baz/app.min.js",
        "https://foo.com/*/app.min.js",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "https://foo.com/bar/baz/app.min.js",
        "https://foo.com/**/app.min.js",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "https://foo.com/bar/baz/app.min.js",
        "https://foo.com/**/app.min.js",
        CompileOptions {
            noglobstar: true,
            ..Default::default()
        },
        false,
    );
}
