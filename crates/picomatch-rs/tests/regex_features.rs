mod support;

use picomatch_rs::CompileOptions;

use support::{assert_is_match, default_compile_options};

#[test]
fn should_support_word_boundaries() {
    assert_is_match("a", "a\\b", default_compile_options(), true);
}

#[test]
fn should_support_word_boundaries_in_parens() {
    assert_is_match("a", "(a\\b)", default_compile_options(), true);
}

#[test]
fn should_support_regex_lookbehinds() {
    assert_is_match(
        "foo/cbaz",
        "foo/*(?<!d)baz",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "foo/cbaz",
        "foo/*(?<!c)baz",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/cbaz",
        "foo/*(?<=d)baz",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/cbaz",
        "foo/*(?<=c)baz",
        default_compile_options(),
        true,
    );
}

#[test]
fn should_support_regex_backreferences() {
    assert_is_match("1/2", "(*)/\\1", default_compile_options(), false);
    assert_is_match("1/1", "(*)/\\1", default_compile_options(), true);
    assert_is_match(
        "1/1/1/1",
        "(*)/\\1/\\1/\\1",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "1/11/111/1111",
        "(*)/\\1/\\1/\\1",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "1/11/111/1111",
        "(*)/(\\1)+/(\\1)+/(\\1)+",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "1/2/1/1",
        "(*)/\\1/\\1/\\1",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "1/1/2/1",
        "(*)/\\1/\\1/\\1",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "1/1/1/2",
        "(*)/\\1/\\1/\\1",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "1/1/1/1",
        "(*)/\\1/(*)/\\2",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "1/1/2/1",
        "(*)/\\1/(*)/\\2",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "1/1/2/2",
        "(*)/\\1/(*)/\\2",
        default_compile_options(),
        true,
    );
}

#[test]
fn should_not_match_with_character_classes_when_disabled() {
    let opts = CompileOptions {
        nobracket: true,
        ..CompileOptions::default()
    };
    assert_is_match("a/a", "a/[a-z]", opts.clone(), false);
    assert_is_match("a/b", "a/[a-z]", opts.clone(), false);
    assert_is_match("a/c", "a/[a-z]", opts.clone(), false);
}

#[test]
fn should_match_with_character_classes_by_default() {
    assert_is_match("a/a", "a/[a-z]", default_compile_options(), true);
    assert_is_match("a/b", "a/[a-z]", default_compile_options(), true);
    assert_is_match("a/c", "a/[a-z]", default_compile_options(), true);

    assert_is_match("foo/bar", "**/[jkl]*", default_compile_options(), false);
    assert_is_match("foo/jar", "**/[jkl]*", default_compile_options(), true);

    assert_is_match("foo/bar", "**/[^jkl]*", default_compile_options(), true);
    assert_is_match("foo/jar", "**/[^jkl]*", default_compile_options(), false);

    assert_is_match("foo/bar", "**/[abc]*", default_compile_options(), true);
    assert_is_match("foo/jar", "**/[abc]*", default_compile_options(), false);

    assert_is_match("foo/bar", "**/[^abc]*", default_compile_options(), false);
    assert_is_match("foo/jar", "**/[^abc]*", default_compile_options(), true);

    assert_is_match("foo/bar", "**/[abc]ar", default_compile_options(), true);
    assert_is_match("foo/jar", "**/[abc]ar", default_compile_options(), false);
}

#[test]
fn should_match_character_classes() {
    assert_is_match("abc", "a[bc]d", default_compile_options(), false);
    assert_is_match("abd", "a[bc]d", default_compile_options(), true);
}

#[test]
fn should_match_character_class_alphabetical_ranges() {
    assert_is_match("abc", "a[b-d]e", default_compile_options(), false);
    assert_is_match("abd", "a[b-d]e", default_compile_options(), false);
    assert_is_match("abe", "a[b-d]e", default_compile_options(), true);
    assert_is_match("ac", "a[b-d]e", default_compile_options(), false);
    assert_is_match("a-", "a[b-d]e", default_compile_options(), false);

    assert_is_match("abc", "a[b-d]", default_compile_options(), false);
    assert_is_match("abd", "a[b-d]", default_compile_options(), false);
    assert_is_match("abd", "a[b-d]+", default_compile_options(), true);
    assert_is_match("abe", "a[b-d]", default_compile_options(), false);
    assert_is_match("ac", "a[b-d]", default_compile_options(), true);
    assert_is_match("a-", "a[b-d]", default_compile_options(), false);
}

#[test]
fn should_match_character_classes_with_leading_dashes() {
    assert_is_match("abc", "a[-c]", default_compile_options(), false);
    assert_is_match("ac", "a[-c]", default_compile_options(), true);
    assert_is_match("a-", "a[-c]", default_compile_options(), true);
}

#[test]
fn should_match_character_classes_with_trailing_dashes() {
    assert_is_match("abc", "a[c-]", default_compile_options(), false);
    assert_is_match("ac", "a[c-]", default_compile_options(), true);
    assert_is_match("a-", "a[c-]", default_compile_options(), true);
}

#[test]
fn should_match_bracket_literals() {
    assert_is_match("a]c", "a[]]c", default_compile_options(), true);
    assert_is_match("a]c", "a]c", default_compile_options(), true);
    assert_is_match("a]", "a]", default_compile_options(), true);

    assert_is_match("a[c", "a[\\[]c", default_compile_options(), true);
    assert_is_match("a[c", "a[c", default_compile_options(), true);
    assert_is_match("a[", "a[", default_compile_options(), true);
}

#[test]
fn should_support_negated_character_classes() {
    assert_is_match("a]", "a[^bc]d", default_compile_options(), false);
    assert_is_match("acd", "a[^bc]d", default_compile_options(), false);
    assert_is_match("aed", "a[^bc]d", default_compile_options(), true);
    assert_is_match("azd", "a[^bc]d", default_compile_options(), true);
    assert_is_match("ac", "a[^bc]d", default_compile_options(), false);
    assert_is_match("a-", "a[^bc]d", default_compile_options(), false);
}

#[test]
fn should_match_negated_dashes() {
    assert_is_match("abc", "a[^-b]c", default_compile_options(), false);
    assert_is_match("adc", "a[^-b]c", default_compile_options(), true);
    assert_is_match("a-c", "a[^-b]c", default_compile_options(), false);
}

#[test]
fn should_match_negated_pm() {
    assert_is_match("a-c", "a[^\\]b]c", default_compile_options(), true);
    assert_is_match("abc", "a[^\\]b]c", default_compile_options(), false);
    assert_is_match("a]c", "a[^\\]b]c", default_compile_options(), false);
    assert_is_match("adc", "a[^\\]b]c", default_compile_options(), true);
}

#[test]
fn should_match_alpha_numeric_characters() {
    assert_is_match("0123e45g78", "[\\de]+", default_compile_options(), false);
    assert_is_match("0123e456", "[\\de]+", default_compile_options(), true);
    assert_is_match("01234", "[\\de]+", default_compile_options(), true);
}

#[test]
fn should_support_valid_regex_ranges() {
    assert_is_match("a/a", "a/[b-c]", default_compile_options(), false);
    assert_is_match("a/z", "a/[b-c]", default_compile_options(), false);
    assert_is_match("a/b", "a/[b-c]", default_compile_options(), true);
    assert_is_match("a/c", "a/[b-c]", default_compile_options(), true);
    assert_is_match("a/b", "[a-z]/[a-z]", default_compile_options(), true);
    assert_is_match("a/z", "[a-z]/[a-z]", default_compile_options(), true);
    assert_is_match("z/z", "[a-z]/[a-z]", default_compile_options(), true);
    assert_is_match("a/x/y", "a/[a-z]", default_compile_options(), false);

    assert_is_match("a.a", "[a-b].[a-b]", default_compile_options(), true);
    assert_is_match("a.b", "[a-b].[a-b]", default_compile_options(), true);
    assert_is_match("a.a.a", "[a-b].[a-b]", default_compile_options(), false);
    assert_is_match("c.a", "[a-b].[a-b]", default_compile_options(), false);
    assert_is_match("d.a.d", "[a-b].[a-b]", default_compile_options(), false);
    assert_is_match("a.bb", "[a-b].[a-b]", default_compile_options(), false);
    assert_is_match("a.ccc", "[a-b].[a-b]", default_compile_options(), false);

    assert_is_match("a.a", "[a-d].[a-b]", default_compile_options(), true);
    assert_is_match("a.b", "[a-d].[a-b]", default_compile_options(), true);
    assert_is_match("a.a.a", "[a-d].[a-b]", default_compile_options(), false);
    assert_is_match("c.a", "[a-d].[a-b]", default_compile_options(), true);
    assert_is_match("d.a.d", "[a-d].[a-b]", default_compile_options(), false);
    assert_is_match("a.bb", "[a-d].[a-b]", default_compile_options(), false);
    assert_is_match("a.ccc", "[a-d].[a-b]", default_compile_options(), false);

    assert_is_match("a.a", "[a-d]*.[a-b]", default_compile_options(), true);
    assert_is_match("a.b", "[a-d]*.[a-b]", default_compile_options(), true);
    assert_is_match("a.a.a", "[a-d]*.[a-b]", default_compile_options(), true);
    assert_is_match("c.a", "[a-d]*.[a-b]", default_compile_options(), true);
    assert_is_match("d.a.d", "[a-d]*.[a-b]", default_compile_options(), false);
    assert_is_match("a.bb", "[a-d]*.[a-b]", default_compile_options(), false);
    assert_is_match("a.ccc", "[a-d]*.[a-b]", default_compile_options(), false);
}

#[test]
fn should_support_valid_regex_ranges_with_glob_negation_patterns() {
    assert_is_match("a.a", "!*.[a-b]", default_compile_options(), false);
    assert_is_match("a.b", "!*.[a-b]", default_compile_options(), false);
    assert_is_match("a.a.a", "!*.[a-b]", default_compile_options(), false);
    assert_is_match("c.a", "!*.[a-b]", default_compile_options(), false);
    assert_is_match("d.a.d", "!*.[a-b]", default_compile_options(), true);
    assert_is_match("a.bb", "!*.[a-b]", default_compile_options(), true);
    assert_is_match("a.ccc", "!*.[a-b]", default_compile_options(), true);

    assert_is_match("a.a", "!*.[a-b]*", default_compile_options(), false);
    assert_is_match("a.b", "!*.[a-b]*", default_compile_options(), false);
    assert_is_match("a.a.a", "!*.[a-b]*", default_compile_options(), false);
    assert_is_match("c.a", "!*.[a-b]*", default_compile_options(), false);
    assert_is_match("d.a.d", "!*.[a-b]*", default_compile_options(), false);
    assert_is_match("a.bb", "!*.[a-b]*", default_compile_options(), false);
    assert_is_match("a.ccc", "!*.[a-b]*", default_compile_options(), true);

    assert_is_match("a.a", "![a-b].[a-b]", default_compile_options(), false);
    assert_is_match("a.b", "![a-b].[a-b]", default_compile_options(), false);
    assert_is_match("a.a.a", "![a-b].[a-b]", default_compile_options(), true);
    assert_is_match("c.a", "![a-b].[a-b]", default_compile_options(), true);
    assert_is_match("d.a.d", "![a-b].[a-b]", default_compile_options(), true);
    assert_is_match("a.bb", "![a-b].[a-b]", default_compile_options(), true);
    assert_is_match("a.ccc", "![a-b].[a-b]", default_compile_options(), true);

    assert_is_match("a.a", "![a-b]+.[a-b]+", default_compile_options(), false);
    assert_is_match("a.b", "![a-b]+.[a-b]+", default_compile_options(), false);
    assert_is_match("a.a.a", "![a-b]+.[a-b]+", default_compile_options(), true);
    assert_is_match("c.a", "![a-b]+.[a-b]+", default_compile_options(), true);
    assert_is_match("d.a.d", "![a-b]+.[a-b]+", default_compile_options(), true);
    assert_is_match("a.bb", "![a-b]+.[a-b]+", default_compile_options(), false);
    assert_is_match("a.ccc", "![a-b]+.[a-b]+", default_compile_options(), true);
}

#[test]
fn should_support_valid_regex_ranges_in_negated_character_classes() {
    assert_is_match("a.a", "*.[^a-b]", default_compile_options(), false);
    assert_is_match("a.b", "*.[^a-b]", default_compile_options(), false);
    assert_is_match("a.a.a", "*.[^a-b]", default_compile_options(), false);
    assert_is_match("c.a", "*.[^a-b]", default_compile_options(), false);
    assert_is_match("d.a.d", "*.[^a-b]", default_compile_options(), true);
    assert_is_match("a.bb", "*.[^a-b]", default_compile_options(), false);
    assert_is_match("a.ccc", "*.[^a-b]", default_compile_options(), false);

    assert_is_match("a.a", "a.[^a-b]*", default_compile_options(), false);
    assert_is_match("a.b", "a.[^a-b]*", default_compile_options(), false);
    assert_is_match("a.a.a", "a.[^a-b]*", default_compile_options(), false);
    assert_is_match("c.a", "a.[^a-b]*", default_compile_options(), false);
    assert_is_match("d.a.d", "a.[^a-b]*", default_compile_options(), false);
    assert_is_match("a.bb", "a.[^a-b]*", default_compile_options(), false);
    assert_is_match("a.ccc", "a.[^a-b]*", default_compile_options(), true);
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
fn should_support_regex_character_classes_inside_extglobs() {
    assert_is_match("foo/bar", "**/!([a-k])*", default_compile_options(), false);
    assert_is_match("foo/jar", "**/!([a-k])*", default_compile_options(), false);

    assert_is_match("foo/bar", "**/!([a-i])*", default_compile_options(), false);
    assert_is_match("foo/bar", "**/!([c-i])*", default_compile_options(), true);
    assert_is_match("foo/jar", "**/!([a-i])*", default_compile_options(), true);
}

#[test]
fn should_support_regex_capture_groups() {
    assert_is_match(
        "a/bb/c/dd/e.md",
        "a/??/?/(dd)/e.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/e.md",
        "a/?/c/?/(e|f).md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/f.md",
        "a/?/c/?/(e|f).md",
        default_compile_options(),
        true,
    );
}

#[test]
fn should_support_regex_capture_groups_with_slashes() {
    assert_is_match("a/a", "(a/b)", default_compile_options(), false);
    assert_is_match("a/b", "(a/b)", default_compile_options(), true);
    assert_is_match("a/c", "(a/b)", default_compile_options(), false);
    assert_is_match("b/a", "(a/b)", default_compile_options(), false);
    assert_is_match("b/b", "(a/b)", default_compile_options(), false);
    assert_is_match("b/c", "(a/b)", default_compile_options(), false);
}

#[test]
fn should_support_regex_non_capture_groups() {
    assert_is_match(
        "a/bb/c/dd/e.md",
        "a/**/(?:dd)/e.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/e.md",
        "a/?/c/?/(?:e|f).md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/f.md",
        "a/?/c/?/(?:e|f).md",
        default_compile_options(),
        true,
    );
}

#[test]
fn should_support_regex_quantifiers_by_escaping_braces() {
    let unescape_opts = CompileOptions {
        unescape: true,
        ..CompileOptions::default()
    };
    assert_is_match("a   ", "a \\{1,5\\}", unescape_opts.clone(), true);
    assert_is_match("a   ", "a \\{1,2\\}", unescape_opts.clone(), false);
    assert_is_match("a   ", "a \\{1,2\\}", default_compile_options(), false);
}

// Note: "should basename paths" from test/regex-features.js is tested in
// crates/picomatch-rs/src/matcher.rs::tests::should_basename_paths
// because the `basename` utility function is private to that module.

#[test]
fn should_support_extglobs_with_regex_quantifiers() {
    let unescape_opts = CompileOptions {
        unescape: true,
        ..CompileOptions::default()
    };
    assert_is_match("a  ", "@(!(a) \\{1,2\\})*", unescape_opts.clone(), false);
    assert_is_match("a ", "@(!(a) \\{1,2\\})*", unescape_opts.clone(), false);
    assert_is_match("a", "@(!(a) \\{1,2\\})*", unescape_opts.clone(), false);
    assert_is_match("aa", "@(!(a) \\{1,2\\})*", unescape_opts.clone(), false);
    assert_is_match("aaa", "@(!(a) \\{1,2\\})*", unescape_opts.clone(), false);
    assert_is_match("b", "@(!(a) \\{1,2\\})*", unescape_opts.clone(), false);
    assert_is_match("bb", "@(!(a) \\{1,2\\})*", unescape_opts.clone(), false);
    assert_is_match("bbb", "@(!(a) \\{1,2\\})*", unescape_opts.clone(), false);
    assert_is_match(" a ", "@(!(a) \\{1,2\\})*", unescape_opts.clone(), true);
    assert_is_match("b  ", "@(!(a) \\{1,2\\})*", unescape_opts.clone(), true);
    assert_is_match("b ", "@(!(a) \\{1,2\\})*", unescape_opts.clone(), true);

    assert_is_match(
        "a   ",
        "@(!(a \\{1,2\\}))*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a   b",
        "@(!(a \\{1,2\\}))*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a  b",
        "@(!(a \\{1,2\\}))*",
        default_compile_options(),
        true,
    );
    assert_is_match("a  ", "@(!(a \\{1,2\\}))*", default_compile_options(), true);
    assert_is_match("a ", "@(!(a \\{1,2\\}))*", default_compile_options(), true);
    assert_is_match("a", "@(!(a \\{1,2\\}))*", default_compile_options(), true);
    assert_is_match("aa", "@(!(a \\{1,2\\}))*", default_compile_options(), true);
    assert_is_match("b", "@(!(a \\{1,2\\}))*", default_compile_options(), true);
    assert_is_match("bb", "@(!(a \\{1,2\\}))*", default_compile_options(), true);
    assert_is_match(" a ", "@(!(a \\{1,2\\}))*", default_compile_options(), true);
    assert_is_match("b  ", "@(!(a \\{1,2\\}))*", default_compile_options(), true);
    assert_is_match("b ", "@(!(a \\{1,2\\}))*", default_compile_options(), true);
}
