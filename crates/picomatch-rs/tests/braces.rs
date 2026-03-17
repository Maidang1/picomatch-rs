mod support;

use support::{assert_is_match, assert_match_list, default_compile_options};

#[test]
fn should_not_match_with_brace_patterns_when_disabled() {
    let opts = default_compile_options();

    // Test match list with nobrace option
    assert_match_list(
        &["a", "b", "c"],
        "{a,b,c,d}",
        opts.clone(),
        &["a", "b", "c"],
    );
    assert_match_list(
        &["a", "b", "c"],
        "{a,b,c,d}",
        opts.clone().with_nobrace(true),
        &[],
    );
    assert_match_list(
        &["1", "2", "3"],
        "{1..2}",
        opts.clone().with_nobrace(true),
        &[],
    );

    // Test isMatch with nobrace option
    assert!(!is_match_case(
        "a/a",
        "a/{a,b}",
        opts.clone().with_nobrace(true)
    ));
    assert!(!is_match_case(
        "a/b",
        "a/{a,b}",
        opts.clone().with_nobrace(true)
    ));
    assert!(!is_match_case(
        "a/c",
        "a/{a,b}",
        opts.clone().with_nobrace(true)
    ));
    assert!(!is_match_case(
        "b/b",
        "a/{a,b}",
        opts.clone().with_nobrace(true)
    ));
    assert!(!is_match_case(
        "b/b",
        "a/{a,b,c}",
        opts.clone().with_nobrace(true)
    ));
    assert!(!is_match_case(
        "a/c",
        "a/{a,b,c}",
        opts.clone().with_nobrace(true)
    ));
    assert!(!is_match_case(
        "a/a",
        "a/{a..c}",
        opts.clone().with_nobrace(true)
    ));
    assert!(!is_match_case(
        "a/b",
        "a/{a..c}",
        opts.clone().with_nobrace(true)
    ));
    assert!(!is_match_case(
        "a/c",
        "a/{a..c}",
        opts.clone().with_nobrace(true)
    ));
}

#[test]
fn should_treat_single_set_braces_as_literals() {
    let opts = default_compile_options();
    assert_is_match("a {abc} b", "a {abc} b", opts.clone(), true);
    assert_is_match("a {a-b-c} b", "a {a-b-c} b", opts.clone(), true);
    assert_is_match("a {a.c} b", "a {a.c} b", opts.clone(), true);
}

#[test]
fn should_match_literal_braces_when_escaped() {
    let opts = default_compile_options();
    assert_is_match("a {1,2}", r"a \{1,2\}", opts.clone(), true);
    assert_is_match("a {a..b}", r"a \{a..b\}", opts.clone(), true);
}

#[test]
fn should_match_using_brace_patterns() {
    let opts = default_compile_options();
    assert!(!is_match_case("a/c", "a/{a,b}", opts.clone()));
    assert!(!is_match_case("b/b", "a/{a,b,c}", opts.clone()));
    assert!(!is_match_case("b/b", "a/{a,b}", opts.clone()));
    assert_is_match("a/a", "a/{a,b}", opts.clone(), true);
    assert_is_match("a/b", "a/{a,b}", opts.clone(), true);
    assert_is_match("a/c", "a/{a,b,c}", opts.clone(), true);
}

#[test]
fn should_support_brace_ranges() {
    let opts = default_compile_options();
    assert_is_match("a/a", "a/{a..c}", opts.clone(), true);
    assert_is_match("a/b", "a/{a..c}", opts.clone(), true);
    assert_is_match("a/c", "a/{a..c}", opts.clone(), true);
}

#[test]
fn should_support_kleene_stars() {
    let opts = default_compile_options();
    assert_is_match("ab", "{ab,c}*", opts.clone(), true);
    assert_is_match("abab", "{ab,c}*", opts.clone(), true);
    assert_is_match("abc", "{ab,c}*", opts.clone(), true);
    assert_is_match("c", "{ab,c}*", opts.clone(), true);
    assert_is_match("cab", "{ab,c}*", opts.clone(), true);
    assert_is_match("cc", "{ab,c}*", opts.clone(), true);
    assert_is_match("ababab", "{ab,c}*", opts.clone(), true);
    assert_is_match("ababc", "{ab,c}*", opts.clone(), true);
    assert_is_match("abcab", "{ab,c}*", opts.clone(), true);
    assert_is_match("abcc", "{ab,c}*", opts.clone(), true);
    assert_is_match("cabab", "{ab,c}*", opts.clone(), true);
    assert_is_match("cabc", "{ab,c}*", opts.clone(), true);
    assert_is_match("ccab", "{ab,c}*", opts.clone(), true);
    assert_is_match("ccc", "{ab,c}*", opts.clone(), true);
}

#[test]
fn should_not_convert_braces_inside_brackets() {
    let opts = default_compile_options();
    assert_is_match("foo{}baz", "foo[{a,b}]+baz", opts.clone(), true);
    assert_is_match("{a}{b}{c}", "[abc{}]+", opts.clone(), true);
}

#[test]
fn should_support_braces_containing_slashes() {
    let opts = default_compile_options();
    assert_is_match("a", "{/,}a/**", opts.clone(), true);
    assert_is_match("aa.txt", "a{a,b/}*.txt", opts.clone(), true);
    assert_is_match("ab/.txt", "a{a,b/}*.txt", opts.clone(), true);
    assert_is_match("ab/a.txt", "a{a,b/}*.txt", opts.clone(), true);
    assert_is_match("a/", "a/**{/,}", opts.clone(), true);
    assert_is_match("a/a", "a/**{/,}", opts.clone(), true);
    assert_is_match("a/a/", "a/**{/,}", opts.clone(), true);
}

#[test]
fn should_support_braces_with_empty_elements() {
    let opts = default_compile_options();
    assert!(!is_match_case("abc.txt", "a{,b}.txt", opts.clone()));
    assert!(!is_match_case("abc.txt", "a{a,b,}.txt", opts.clone()));
    assert!(!is_match_case("abc.txt", "a{b,}.txt", opts.clone()));
    assert_is_match("a.txt", "a{,b}.txt", opts.clone(), true);
    assert_is_match("a.txt", "a{b,}.txt", opts.clone(), true);
    assert_is_match("aa.txt", "a{a,b,}.txt", opts.clone(), true);
    assert_is_match("aa.txt", "a{a,b,}.txt", opts.clone(), true);
    assert_is_match("ab.txt", "a{,b}.txt", opts.clone(), true);
    assert_is_match("ab.txt", "a{b,}.txt", opts.clone(), true);
}

#[test]
fn should_support_braces_with_slashes_and_empty_elements() {
    let opts = default_compile_options();
    assert_is_match("a.txt", "a{,/}*.txt", opts.clone(), true);
    assert_is_match("ab.txt", "a{,/}*.txt", opts.clone(), true);
    assert_is_match("a/b.txt", "a{,/}*.txt", opts.clone(), true);
    assert_is_match("a/ab.txt", "a{,/}*.txt", opts.clone(), true);
}

#[test]
fn should_support_braces_with_stars() {
    let opts = default_compile_options();
    assert_is_match("a.txt", r"a{,.*{foo,db},\\(bar\)}.txt", opts.clone(), true);
    assert!(!is_match_case(
        "adb.txt",
        r"a{,.*{foo,db},\\(bar\)}.txt",
        opts.clone()
    ));
    assert_is_match(
        "a.db.txt",
        r"a{,.*{foo,db},\\(bar\)}.txt",
        opts.clone(),
        true,
    );

    assert_is_match("a.txt", r"a{,*.{foo,db},\\(bar\)}.txt", opts.clone(), true);
    assert!(!is_match_case(
        "adb.txt",
        r"a{,*.{foo,db},\\(bar\)}.txt",
        opts.clone()
    ));
    assert_is_match(
        "a.db.txt",
        r"a{,*.{foo,db},\\(bar\)}.txt",
        opts.clone(),
        true,
    );

    assert_is_match("a", r"a{,.*{foo,db},\\(bar\)}", opts.clone(), true);
    assert!(!is_match_case(
        "adb",
        r"a{,.*{foo,db},\\(bar\)}",
        opts.clone()
    ));
    assert_is_match("a.db", r"a{,.*{foo,db},\\(bar\)}", opts.clone(), true);

    assert_is_match("a", r"a{,*.{foo,db},\\(bar\)}", opts.clone(), true);
    assert!(!is_match_case(
        "adb",
        r"a{,*.{foo,db},\\(bar\)}",
        opts.clone()
    ));
    assert_is_match("a.db", r"a{,*.{foo,db},\\(bar\)}", opts.clone(), true);

    assert!(!is_match_case("a", r"{,.*{foo,db},\\(bar\)}", opts.clone()));
    assert!(!is_match_case(
        "adb",
        r"{,.*{foo,db},\\(bar\)}",
        opts.clone()
    ));
    assert!(!is_match_case(
        "a.db",
        r"{,.*{foo,db},\\(bar\)}",
        opts.clone()
    ));
    assert_is_match(".db", r"{,.*{foo,db},\\(bar\)}", opts.clone(), true);

    assert!(!is_match_case("a", r"{,*.{foo,db},\\(bar\)}", opts.clone()));
    assert_is_match("a", r"{*,*.{foo,db},\\(bar\)}", opts.clone(), true);
    assert!(!is_match_case(
        "adb",
        r"{,*.{foo,db},\\(bar\)}",
        opts.clone()
    ));
    assert_is_match("a.db", r"{,*.{foo,db},\\(bar\)}", opts.clone(), true);
}

#[test]
fn should_support_braces_in_patterns_with_globstars() {
    let opts = default_compile_options();
    assert!(!is_match_case(
        "a/b/c/xyz.md",
        "a/b/**/c{d,e}/**/xyz.md",
        opts.clone()
    ));
    assert!(!is_match_case(
        "a/b/d/xyz.md",
        "a/b/**/c{d,e}/**/xyz.md",
        opts.clone()
    ));
    assert_is_match(
        "a/b/cd/xyz.md",
        "a/b/**/c{d,e}/**/xyz.md",
        opts.clone(),
        true,
    );
    assert_is_match(
        "a/b/c/xyz.md",
        "a/b/**/{c,d,e}/**/xyz.md",
        opts.clone(),
        true,
    );
    assert_is_match(
        "a/b/d/xyz.md",
        "a/b/**/{c,d,e}/**/xyz.md",
        opts.clone(),
        true,
    );
}

#[test]
fn should_support_braces_with_globstars_slashes_and_empty_elements() {
    let opts = default_compile_options();
    assert_is_match("a.txt", "a{,/**/}*.txt", opts.clone(), true);
    assert_is_match("a/b.txt", "a{,/**/,/}*.txt", opts.clone(), true);
    assert_is_match("a/x/y.txt", "a{,/**/}*.txt", opts.clone(), true);
    assert!(!is_match_case("a/x/y/z", "a{,/**/}*.txt", opts.clone()));
}

#[test]
fn should_support_braces_with_globstars_and_empty_elements() {
    let opts = default_compile_options();
    assert_is_match(
        "a/b/foo/bar/baz.qux",
        "a/b{,/**}/bar{,/**}/*.*",
        opts.clone(),
        true,
    );
    assert_is_match(
        "a/b/bar/baz.qux",
        "a/b{,/**}/bar{,/**}/*.*",
        opts.clone(),
        true,
    );
}

#[test]
fn should_support_kleene_plus() {
    let opts = default_compile_options();
    assert_is_match("ab", "{ab,c}+", opts.clone(), true);
    assert_is_match("abab", "{ab,c}+", opts.clone(), true);
    assert_is_match("abc", "{ab,c}+", opts.clone(), true);
    assert_is_match("c", "{ab,c}+", opts.clone(), true);
    assert_is_match("cab", "{ab,c}+", opts.clone(), true);
    assert_is_match("cc", "{ab,c}+", opts.clone(), true);
    assert_is_match("ababab", "{ab,c}+", opts.clone(), true);
    assert_is_match("ababc", "{ab,c}+", opts.clone(), true);
    assert_is_match("abcab", "{ab,c}+", opts.clone(), true);
    assert_is_match("abcc", "{ab,c}+", opts.clone(), true);
    assert_is_match("cabab", "{ab,c}+", opts.clone(), true);
    assert_is_match("cabc", "{ab,c}+", opts.clone(), true);
    assert_is_match("ccab", "{ab,c}+", opts.clone(), true);
    assert_is_match("ccc", "{ab,c}+", opts.clone(), true);
    assert_is_match("ccc", "{a,b,c}+", opts.clone(), true);

    assert_is_match("a", "{a,b,c}+", opts.clone(), true);
    assert_is_match("b", "{a,b,c}+", opts.clone(), true);
    assert_is_match("c", "{a,b,c}+", opts.clone(), true);
    assert_is_match("aa", "{a,b,c}+", opts.clone(), true);
    assert_is_match("ab", "{a,b,c}+", opts.clone(), true);
    assert_is_match("ac", "{a,b,c}+", opts.clone(), true);
    assert_is_match("ba", "{a,b,c}+", opts.clone(), true);
    assert_is_match("bb", "{a,b,c}+", opts.clone(), true);
    assert_is_match("bc", "{a,b,c}+", opts.clone(), true);
    assert_is_match("ca", "{a,b,c}+", opts.clone(), true);
    assert_is_match("cb", "{a,b,c}+", opts.clone(), true);
    assert_is_match("cc", "{a,b,c}+", opts.clone(), true);
    assert_is_match("aaa", "{a,b,c}+", opts.clone(), true);
    assert_is_match("aab", "{a,b,c}+", opts.clone(), true);
    assert_is_match("abc", "{a,b,c}+", opts.clone(), true);
}

#[test]
fn should_support_braces() {
    let opts = default_compile_options();
    assert_is_match("a", "{a,b,c}", opts.clone(), true);
    assert_is_match("b", "{a,b,c}", opts.clone(), true);
    assert_is_match("c", "{a,b,c}", opts.clone(), true);
    assert!(!is_match_case("aa", "{a,b,c}", opts.clone()));
    assert!(!is_match_case("bb", "{a,b,c}", opts.clone()));
    assert!(!is_match_case("cc", "{a,b,c}", opts.clone()));
}

/// Corresponds to:
///   it('should match special chars and expand ranges in parentheses', ...)
///
/// The JS test uses a function-valued `expandRange` callback backed by
/// `fill-range({ toRegex: true })`. Rust does not expose callback-based
/// `expandRange`, but the native brace-range expansion is semantically
/// equivalent for these patterns, so we assert the same matching behaviour
/// directly with the default compiler.
#[test]
fn should_match_special_chars_and_expand_ranges_in_parentheses() {
    let opts = default_compile_options();

    assert_is_match("foo/bar - 1", "*/* {4..10}", opts.clone(), false);
    assert_is_match(
        "foo/bar - copy (1)",
        r"*/* - * \({4..10}\)",
        opts.clone(),
        false,
    );
    assert_is_match("foo/bar (1)", r"*/* \({4..10}\)", opts.clone(), false);
    assert_is_match("foo/bar (4)", r"*/* \({4..10}\)", opts.clone(), true);
    assert_is_match("foo/bar (7)", r"*/* \({4..10}\)", opts.clone(), true);
    assert_is_match("foo/bar (42)", r"*/* \({4..10}\)", opts.clone(), false);
    assert_is_match("foo/bar (42)", r"*/* \({4..43}\)", opts.clone(), true);
    assert_is_match("foo/bar - copy [1]", r"*/* \[{0..5}\]", opts.clone(), true);
    assert_is_match(
        "foo/bar - foo + bar - copy [1]",
        r"*/* \[{0..5}\]",
        opts.clone(),
        true,
    );
    assert_is_match("foo/bar - 1", r"*/* \({4..10}\)", opts.clone(), false);
    assert_is_match(
        "foo/bar - copy (1)",
        r"*/* \({4..10}\)",
        opts.clone(),
        false,
    );
    assert_is_match("foo/bar (1)", r"*/* \({4..10}\)", opts.clone(), false);
    assert_is_match("foo/bar (4)", r"*/* \({4..10}\)", opts.clone(), true);
    assert_is_match("foo/bar (7)", r"*/* \({4..10}\)", opts.clone(), true);
    assert_is_match("foo/bar (42)", r"*/* \({4..10}\)", opts.clone(), false);
    assert_is_match(
        "foo/bar - copy [1]",
        r"*/* \({4..10}\)",
        opts.clone(),
        false,
    );
    assert_is_match(
        "foo/bar - foo + bar - copy [1]",
        r"*/* \({4..10}\)",
        opts.clone(),
        false,
    );
}

// Helper function to call is_match since we need different import
fn is_match_case(input: &str, pattern: &str, options: picomatch_rs::CompileOptions) -> bool {
    picomatch_rs::is_match(input, pattern, &options).unwrap_or(false)
}

// Add nobrace option support to CompileOptions
trait CompileOptionsExt {
    fn with_nobrace(self, value: bool) -> Self;
}

impl CompileOptionsExt for picomatch_rs::CompileOptions {
    fn with_nobrace(mut self, value: bool) -> Self {
        self.nobrace = value;
        self
    }
}
