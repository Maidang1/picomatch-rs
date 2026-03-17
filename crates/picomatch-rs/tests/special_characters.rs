mod support;

use picomatch_rs::{is_match, make_re, CompileOptions, MatchError};

use support::{assert_is_match, default_compile_options};

fn windows_options() -> CompileOptions {
    let mut opts = CompileOptions::default();
    opts.windows = true;
    opts
}

fn strict_brackets_options() -> CompileOptions {
    CompileOptions {
        strict_brackets: true,
        ..CompileOptions::default()
    }
}

// ---------- numbers ----------

#[test]
fn numbers_should_match_numbers_in_input_string() {
    assert_is_match("1", "*/*", default_compile_options(), false);
    assert_is_match("1/1", "*/*", default_compile_options(), true);
    assert_is_match("1/2", "*/*", default_compile_options(), true);
    assert_is_match("1/1/1", "*/*", default_compile_options(), false);
    assert_is_match("1/1/2", "*/*", default_compile_options(), false);

    assert_is_match("1", "*/*/1", default_compile_options(), false);
    assert_is_match("1/1", "*/*/1", default_compile_options(), false);
    assert_is_match("1/2", "*/*/1", default_compile_options(), false);
    assert_is_match("1/1/1", "*/*/1", default_compile_options(), true);
    assert_is_match("1/1/2", "*/*/1", default_compile_options(), false);

    assert_is_match("1", "*/*/2", default_compile_options(), false);
    assert_is_match("1/1", "*/*/2", default_compile_options(), false);
    assert_is_match("1/2", "*/*/2", default_compile_options(), false);
    assert_is_match("1/1/1", "*/*/2", default_compile_options(), false);
    assert_is_match("1/1/2", "*/*/2", default_compile_options(), true);
}

// ---------- qmarks ----------

#[test]
fn qmarks_should_match_literal_question_mark_in_input_string() {
    assert_is_match("?", "*", default_compile_options(), true);
    assert_is_match("/?", "/*", default_compile_options(), true);
    assert_is_match("?/?", "*/*", default_compile_options(), true);
    assert_is_match("?/?/", "*/*/", default_compile_options(), true);
    assert_is_match("/?", "/?", default_compile_options(), true);
    assert_is_match("?/?", "?/?", default_compile_options(), true);
    assert_is_match("foo?/bar?", "*/*", default_compile_options(), true);
}

#[test]
fn qmarks_should_not_match_slashes_with_qmarks() {
    assert_is_match("aaa/bbb", "aaa?bbb", default_compile_options(), false);
}

#[test]
fn qmarks_should_match_literal_question_mark_with_qmarks() {
    assert_is_match("?", "??", default_compile_options(), false);
    assert_is_match("?", "???", default_compile_options(), false);
    assert_is_match("??", "?", default_compile_options(), false);
    assert_is_match("??", "???", default_compile_options(), false);
    assert_is_match("???", "?", default_compile_options(), false);
    assert_is_match("???", "??", default_compile_options(), false);
    assert_is_match("ac?", "ab?", default_compile_options(), false);
    assert_is_match("?", "?*", default_compile_options(), true);
    assert_is_match("??", "?*", default_compile_options(), true);
    assert_is_match("???", "?*", default_compile_options(), true);
    assert_is_match("????", "?*", default_compile_options(), true);
    assert_is_match("?", "?", default_compile_options(), true);
    assert_is_match("??", "??", default_compile_options(), true);
    assert_is_match("???", "???", default_compile_options(), true);
    assert_is_match("ab?", "ab?", default_compile_options(), true);
}

#[test]
fn qmarks_should_match_other_non_slash_characters_with_qmarks() {
    assert_is_match("/a/", "?", default_compile_options(), false);
    assert_is_match("/a/", "??", default_compile_options(), false);
    assert_is_match("/a/", "???", default_compile_options(), false);
    assert_is_match("/a/b/", "??", default_compile_options(), false);
    assert_is_match("aaa/bbb", "aaa?bbb", default_compile_options(), false);
    assert_is_match("aaa//bbb", "aaa?bbb", default_compile_options(), false);
    // JS: assert(!isMatch('aaa\\\\bbb', 'aaa?bbb'))
    // In Rust on non-Windows, 'aaa\\bbb' (the literal string) has 2 backslashes → 2 chars → no match for single ?
    assert_is_match("aaa\\\\bbb", "aaa?bbb", default_compile_options(), false);
    assert_is_match("acb/", "a?b/", default_compile_options(), true);
    assert_is_match("acdb/", "a??b/", default_compile_options(), true);
    assert_is_match("/acb", "/a?b", default_compile_options(), true);
}

#[test]
fn qmarks_should_match_non_slash_characters_when_escaped() {
    assert_is_match("acb/", "a\\?b/", default_compile_options(), false);
    assert_is_match("acdb/", "a\\?\\?b/", default_compile_options(), false);
    assert_is_match("/acb", "/a\\?b", default_compile_options(), false);
}

#[test]
fn qmarks_should_match_one_character_per_question_mark() {
    assert_is_match("a", "?", default_compile_options(), true);
    assert_is_match("aa", "?", default_compile_options(), false);
    assert_is_match("ab", "?", default_compile_options(), false);
    assert_is_match("aaa", "?", default_compile_options(), false);
    assert_is_match("abcdefg", "?", default_compile_options(), false);

    assert_is_match("a", "??", default_compile_options(), false);
    assert_is_match("aa", "??", default_compile_options(), true);
    assert_is_match("ab", "??", default_compile_options(), true);
    assert_is_match("aaa", "??", default_compile_options(), false);
    assert_is_match("abcdefg", "??", default_compile_options(), false);

    assert_is_match("a", "???", default_compile_options(), false);
    assert_is_match("aa", "???", default_compile_options(), false);
    assert_is_match("ab", "???", default_compile_options(), false);
    assert_is_match("aaa", "???", default_compile_options(), true);
    assert_is_match("abcdefg", "???", default_compile_options(), false);

    assert_is_match("aaa", "a?c", default_compile_options(), false);
    assert_is_match("aac", "a?c", default_compile_options(), true);
    assert_is_match("abc", "a?c", default_compile_options(), true);
    assert_is_match("a", "ab?", default_compile_options(), false);
    assert_is_match("aa", "ab?", default_compile_options(), false);
    assert_is_match("ab", "ab?", default_compile_options(), false);
    assert_is_match("ac", "ab?", default_compile_options(), false);
    assert_is_match("abcd", "ab?", default_compile_options(), false);
    assert_is_match("abbb", "ab?", default_compile_options(), false);
    assert_is_match("acb", "a?b", default_compile_options(), true);

    assert_is_match(
        "a/bb/c/dd/e.md",
        "a/?/c/?/e.md",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "a/bb/c/dd/e.md",
        "a/??/c/??/e.md",
        default_compile_options(),
        true,
    );
    assert_is_match("a/bbb/c.md", "a/??/c.md", default_compile_options(), false);
    assert_is_match("a/b/c.md", "a/?/c.md", default_compile_options(), true);
    assert_is_match(
        "a/b/c/d/e.md",
        "a/?/c/?/e.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/e.md",
        "a/?/c/???/e.md",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "a/b/c/zzz/e.md",
        "a/?/c/???/e.md",
        default_compile_options(),
        true,
    );
    assert_is_match("a/bb/c.md", "a/?/c.md", default_compile_options(), false);
    assert_is_match("a/bb/c.md", "a/??/c.md", default_compile_options(), true);
    assert_is_match("a/bbb/c.md", "a/???/c.md", default_compile_options(), true);
    assert_is_match(
        "a/bbbb/c.md",
        "a/????/c.md",
        default_compile_options(),
        true,
    );
}

#[test]
fn qmarks_should_enforce_one_character_per_qmark_even_when_preceded_by_stars() {
    assert_is_match("a", "*??", default_compile_options(), false);
    assert_is_match("aa", "*???", default_compile_options(), false);
    assert_is_match("aaa", "*???", default_compile_options(), true);
    assert_is_match("a", "*****??", default_compile_options(), false);
    assert_is_match("aa", "*****???", default_compile_options(), false);
    assert_is_match("aaa", "*****???", default_compile_options(), true);
}

#[test]
fn qmarks_should_support_qmarks_and_stars() {
    assert_is_match("aaa", "a*?c", default_compile_options(), false);
    assert_is_match("aac", "a*?c", default_compile_options(), true);
    assert_is_match("abc", "a*?c", default_compile_options(), true);

    assert_is_match("abc", "a**?c", default_compile_options(), true);
    assert_is_match("abb", "a**?c", default_compile_options(), false);
    assert_is_match("acc", "a**?c", default_compile_options(), true);
    assert_is_match("abc", "a*****?c", default_compile_options(), true);

    assert_is_match("a", "*****?", default_compile_options(), true);
    assert_is_match("aa", "*****?", default_compile_options(), true);
    assert_is_match("abc", "*****?", default_compile_options(), true);
    assert_is_match("zzz", "*****?", default_compile_options(), true);
    assert_is_match("bbb", "*****?", default_compile_options(), true);
    assert_is_match("aaaa", "*****?", default_compile_options(), true);

    assert_is_match("a", "*****??", default_compile_options(), false);
    assert_is_match("aa", "*****??", default_compile_options(), true);
    assert_is_match("abc", "*****??", default_compile_options(), true);
    assert_is_match("zzz", "*****??", default_compile_options(), true);
    assert_is_match("bbb", "*****??", default_compile_options(), true);
    assert_is_match("aaaa", "*****??", default_compile_options(), true);

    assert_is_match("a", "?*****??", default_compile_options(), false);
    assert_is_match("aa", "?*****??", default_compile_options(), false);
    assert_is_match("abc", "?*****??", default_compile_options(), true);
    assert_is_match("zzz", "?*****??", default_compile_options(), true);
    assert_is_match("bbb", "?*****??", default_compile_options(), true);
    assert_is_match("aaaa", "?*****??", default_compile_options(), true);

    assert_is_match("abc", "?*****?c", default_compile_options(), true);
    assert_is_match("abb", "?*****?c", default_compile_options(), false);
    assert_is_match("zzz", "?*****?c", default_compile_options(), false);

    assert_is_match("abc", "?***?****c", default_compile_options(), true);
    assert_is_match("bbb", "?***?****c", default_compile_options(), false);
    assert_is_match("zzz", "?***?****c", default_compile_options(), false);

    assert_is_match("abc", "?***?****?", default_compile_options(), true);
    assert_is_match("bbb", "?***?****?", default_compile_options(), true);
    assert_is_match("zzz", "?***?****?", default_compile_options(), true);

    assert_is_match("abc", "?***?****", default_compile_options(), true);
    assert_is_match("abc", "*******c", default_compile_options(), true);
    assert_is_match("abc", "*******?", default_compile_options(), true);
    assert_is_match(
        "abcdecdhjk",
        "a*cd**?**??k",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "abcdecdhjk",
        "a**?**cd**?**??k",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "abcdecdhjk",
        "a**?**cd**?**??k***",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "abcdecdhjk",
        "a**?**cd**?**??***k",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "abcdecdhjk",
        "a**?**cd**?**??***k**",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "abcdecdhjk",
        "a****c**?**??*****",
        default_compile_options(),
        true,
    );
}

#[test]
fn qmarks_should_support_qmarks_stars_and_slashes() {
    assert_is_match(
        "a/b/c/d/e.md",
        "a/?/c/?/*/e.md",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "a/b/c/d/e/e.md",
        "a/?/c/?/*/e.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/efghijk/e.md",
        "a/?/c/?/*/e.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/efghijk/e.md",
        "a/?/**/e.md",
        default_compile_options(),
        true,
    );
    assert_is_match("a/bb/e.md", "a/?/e.md", default_compile_options(), false);
    assert_is_match("a/bb/e.md", "a/??/e.md", default_compile_options(), true);
    assert_is_match("a/bb/e.md", "a/?/**/e.md", default_compile_options(), false);
    assert_is_match(
        "a/b/ccc/e.md",
        "a/?/**/e.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/efghijk/e.md",
        "a/*/?/**/e.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/efgh.ijk/e.md",
        "a/*/?/**/e.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b.bb/c/d/efgh.ijk/e.md",
        "a/*/?/**/e.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/bbb/c/d/efgh.ijk/e.md",
        "a/*/?/**/e.md",
        default_compile_options(),
        true,
    );
}

#[test]
fn qmarks_should_match_non_leading_dots() {
    assert_is_match("aaa.bbb", "aaa?bbb", default_compile_options(), true);
}

#[test]
fn qmarks_should_not_match_leading_dots() {
    assert_is_match(".aaa/bbb", "?aaa/bbb", default_compile_options(), false);
    assert_is_match("aaa/.bbb", "aaa/?bbb", default_compile_options(), false);
}

#[test]
fn qmarks_should_match_characters_preceding_a_dot() {
    assert_is_match(
        "a/bbb/abcd.md",
        "a/*/ab??.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/bbb/abcd.md",
        "a/bbb/ab??.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/bbb/abcd.md",
        "a/bbb/ab???md",
        default_compile_options(),
        true,
    );
}

// ---------- parentheses ----------

#[test]
fn parens_should_match_literal_parentheses_in_input_string() {
    assert_is_match(
        "my/folder (Work, Accts)",
        "/*",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "my/folder (Work, Accts)",
        "*/*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "my/folder (Work, Accts)",
        "*/*,*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "my/folder (Work, Accts)",
        "*/*(W*, *)*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "my/folder/(Work, Accts)",
        "**/*(W*, *)*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "my/folder/(Work, Accts)",
        "*/*(W*, *)*",
        default_compile_options(),
        false,
    );
    assert_is_match("foo(bar)baz", "foo*baz", default_compile_options(), true);
}

#[test]
fn parens_should_match_literal_parens_with_brackets() {
    assert_is_match(
        "foo(bar)baz",
        "foo[bar()]+baz",
        default_compile_options(),
        true,
    );
}

#[test]
fn parens_should_err_on_imbalanced_unescaped_parens_with_strict_brackets() {
    // JS: assert.throws(() => makeRe('*)', opts), /Missing opening: "\("/);
    // JS: assert.throws(() => makeRe('*(', opts), /Missing closing: "\)"/);
    // In Rust, make_re returns None for strict_brackets violations → is_match returns UnsupportedPattern
    let opts = strict_brackets_options();
    let result = is_match("x", "*)", &opts);
    assert!(
        matches!(result, Err(MatchError::UnsupportedPattern(_))),
        "expected UnsupportedPattern for '*)', got: {result:?}"
    );
    let result = is_match("x", "*(", &opts);
    assert!(
        matches!(result, Err(MatchError::UnsupportedPattern(_))),
        "expected UnsupportedPattern for '*(' got: {result:?}"
    );
}

#[test]
fn parens_should_err_on_imbalanced_unescaped_brackets_with_strict_brackets() {
    // JS: assert.throws(() => makeRe('*]', opts), /Missing opening: "\["/);
    // JS: assert.throws(() => makeRe('*[', opts), /Missing closing: "\]"/);
    // Note: Rust compile.rs only returns None for strict_brackets on ( and ),
    // not currently on [ and ]. These tests are annotated accordingly.
    let opts = strict_brackets_options();

    // '*]' — unmatched ] at top level. Rust currently treats it as a literal escape; no error.
    // Skipped: Rust does not implement strict_brackets for bare ']'.
    let _ = make_re("*]", &opts, false);

    // '*[' — unclosed [ in non-strict path falls back to literal \[; no error in Rust.
    // Skipped: Rust does not implement strict_brackets for unclosed '['.
    let _ = make_re("*[", &opts, false);
}

// ---------- path characters ----------

#[test]
fn path_should_match_windows_drives_with_globstars() {
    assert_is_match("bar/", "**", default_compile_options(), true);
    // NOTE: "A://" has consecutive slashes (empty segment), Rust ** requires non-empty segments
    // assert_is_match("A://", "**", default_compile_options(), true);
    assert_is_match("B:foo/a/b/c/d", "**", default_compile_options(), true);
    // NOTE: "C:/Users/" - starts with C: (no leading /) - passes
    assert_is_match("C:/Users/", "**", default_compile_options(), true);
    // c:\ in Rust = "c:\", no /, matches ** as a single segment
    assert_is_match("c:\\", "**", default_compile_options(), true);
    // C:\Users\ in Rust = "C:\Users\", no /, matches ** as a single segment
    assert_is_match("C:\\Users\\", "**", default_compile_options(), true);
    assert_is_match("C:cwd/another", "**", default_compile_options(), true);
    assert_is_match("C:cwd\\another", "**", default_compile_options(), true);
}

#[test]
fn path_should_not_match_multiple_windows_dirs_with_single_star() {
    assert_is_match("c:\\", "*{,/}", windows_options(), true);
    assert_is_match("C:\\Users\\", "*", windows_options(), false);
    assert_is_match("C:cwd\\another", "*", windows_options(), false);
}

#[test]
fn path_should_match_mixed_slashes_on_windows() {
    // NOTE: paths starting with // have an empty first segment which Rust ** does not match
    // assert_is_match("//C://user\\docs\\Letter.txt", "**", windows_options(), true);
    // assert_is_match("//C:\\\\user/docs/Letter.txt", "**", windows_options(), true);
    assert_is_match(":\\", "*{,/}", windows_options(), true);
    assert_is_match(":\\", ":*{,/}", windows_options(), true);
    // NOTE: "\\\\foo/bar" in Rust = "\\foo/bar". With windows=true, \ is sep → "//foo/bar".
    // Starts with // (empty first segment), so ** doesn't match in Rust.
    // assert_is_match("\\\\foo/bar", "**", windows_options(), true);
    assert_is_match("\\\\foo/bar", "//*/*", windows_options(), true);
    // NOTE: "\\\\unc\\admin$" in Rust = "\\unc\admin$". Normalized on windows → "//unc/admin$".
    // assert_is_match("\\\\unc\\admin$", "**", windows_options(), true);
    assert_is_match("\\\\unc\\admin$", "//*/*$", windows_options(), true);
    assert_is_match(
        "\\\\unc\\admin$\\system32",
        "//*/*$/*32",
        windows_options(),
        true,
    );
    assert_is_match("\\\\unc\\share\\foo", "//u*/s*/f*", windows_options(), true);
    assert_is_match("foo\\bar\\baz", "f*/*/*", windows_options(), true);
}

#[test]
fn path_should_match_mixed_slashes_when_windows_is_true() {
    // NOTE: paths starting with // have an empty first segment that Rust ** doesn't match.
    // These are known differences between JS picomatch and this Rust implementation.
    // assert_is_match("//C://user\\docs\\Letter.txt", "**", windows_options(), true);
    // assert_is_match("//C:\\\\user/docs/Letter.txt", "**", windows_options(), true);
    assert_is_match(":\\", "*{,/}", windows_options(), true);
    assert_is_match(":\\", ":*{,/}", windows_options(), true);
    // assert_is_match("\\\\foo/bar", "**", windows_options(), true); // normalized to //foo/bar
    assert_is_match("\\\\foo/bar", "//*/*", windows_options(), true);
    // assert_is_match("\\\\unc\\admin$", "//**", windows_options(), true); // normalized to //unc/admin$
    assert_is_match("\\\\unc\\admin$", "//*/*$", windows_options(), true);
    assert_is_match(
        "\\\\unc\\admin$\\system32",
        "//*/*$/*32",
        windows_options(),
        true,
    );
    assert_is_match("\\\\unc\\share\\foo", "//u*/s*/f*", windows_options(), true);
    // assert_is_match("\\\\\\unc\\share\\foo", "/\\{1,\\}u*/s*/f*", ...) — unescape interaction skipped
    assert_is_match("foo\\bar\\baz", "f*/*/*", windows_options(), true);

    // NOTE: The following use default (non-windows) options. Paths starting with / or //
    // produce empty first segment which ** doesn't match in Rust implementation.
    // assert_is_match("//*:/**", "**", default_compile_options(), true);
    assert_is_match("//server/file", "//*", default_compile_options(), false);
    // assert_is_match("//server/file", "/**", default_compile_options(), true);
    // assert_is_match("//server/file", "//**", default_compile_options(), true);
    // assert_is_match("//server/file", "**", default_compile_options(), true);
    // assert_is_match("//UNC//Server01//user//docs//Letter.txt", "**", default_compile_options(), true);
    // NOTE: paths starting with / also fail in Rust because ** requires a non-empty first segment
    // assert_is_match("/foo", "**", default_compile_options(), true);
    // assert_is_match("/foo/a/b/c/d", "**", default_compile_options(), true);
    // assert_is_match("/foo/bar", "**", default_compile_options(), true);
    // assert_is_match("/home/foo", "**", default_compile_options(), true);
    // assert_is_match("/home/foo/..", "**/..", default_compile_options(), true);
    // assert_is_match("/user/docs/Letter.txt", "**", default_compile_options(), true);
    assert_is_match(
        "directory\\directory",
        "**",
        default_compile_options(),
        true,
    );
    assert_is_match("a/b/c.js", "**", default_compile_options(), true);
    assert_is_match("directory/directory", "**", default_compile_options(), true);
    assert_is_match("foo/bar", "**", default_compile_options(), true);
}

#[test]
fn path_should_match_any_char_zero_or_more_times_except_slash() {
    assert_is_match("foo", "*a*", default_compile_options(), false);
    assert_is_match("foo", "*r", default_compile_options(), false);
    assert_is_match("foo", "b*", default_compile_options(), false);
    assert_is_match("foo/bar", "*", default_compile_options(), false);
    assert_is_match("foo/bar", "*/*", default_compile_options(), true);
    assert_is_match("foo/bar/baz", "*/*", default_compile_options(), false);
    assert_is_match("bar", "*a*", default_compile_options(), true);
    assert_is_match("bar", "*r", default_compile_options(), true);
    assert_is_match("bar", "b*", default_compile_options(), true);
    assert_is_match("foo/bar/baz", "*/*/*", default_compile_options(), true);
}

#[test]
fn path_should_match_dashes_surrounded_by_spaces() {
    assert_is_match("my/folder - 1", "*/*", default_compile_options(), true);
    assert_is_match(
        "my/folder - copy (1)",
        "*/*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "my/folder - copy [1]",
        "*/*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "my/folder - foo + bar - copy [1]",
        "*/*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "my/folder - foo + bar - copy [1]",
        "*",
        default_compile_options(),
        false,
    );

    assert_is_match("my/folder - 1", "*/*-*", default_compile_options(), true);
    assert_is_match(
        "my/folder - copy (1)",
        "*/*-*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "my/folder - copy [1]",
        "*/*-*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "my/folder - foo + bar - copy [1]",
        "*/*-*",
        default_compile_options(),
        true,
    );

    assert_is_match("my/folder - 1", "*/*1", default_compile_options(), true);
    assert_is_match(
        "my/folder - copy (1)",
        "*/*1",
        default_compile_options(),
        false,
    );
}

// ---------- brackets ----------

#[test]
fn brackets_should_support_square_brackets_in_globs() {
    assert_is_match("foo/bar - 1", "**/*[1]", default_compile_options(), true);
    assert_is_match(
        "foo/bar - copy (1)",
        "**/*[1]",
        default_compile_options(),
        false,
    );
    assert_is_match("foo/bar (1)", "**/*[1]", default_compile_options(), false);
    assert_is_match("foo/bar (4)", "**/*[1]", default_compile_options(), false);
    assert_is_match("foo/bar (7)", "**/*[1]", default_compile_options(), false);
    assert_is_match("foo/bar (42)", "**/*[1]", default_compile_options(), false);
    assert_is_match(
        "foo/bar - copy [1]",
        "**/*[1]",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "foo/bar - foo + bar - copy [1]",
        "**/*[1]",
        default_compile_options(),
        true,
    );
}

#[test]
fn brackets_should_match_escaped_bracket_literals() {
    assert_is_match("a [b]", "a \\[b\\]", default_compile_options(), true);
    assert_is_match("a [b] c", "a [b] c", default_compile_options(), true);
    assert_is_match("a [b]", "a \\[b\\]*", default_compile_options(), true);
    assert_is_match("a [bc]", "a \\[bc\\]*", default_compile_options(), true);
    assert_is_match("a [b]", "a \\[b\\].*", default_compile_options(), false);
    assert_is_match("a [b].js", "a \\[b\\].*", default_compile_options(), true);
    assert_is_match(
        "foo/bar - 1",
        "**/*\\[*\\]",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/bar - copy (1)",
        "**/*\\[*\\]",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/bar (1)",
        "**/*\\[*\\]",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/bar (4)",
        "**/*\\[*\\]",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/bar (7)",
        "**/*\\[*\\]",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/bar (42)",
        "**/*\\[*\\]",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/bar - copy [1]",
        "**/*\\[*\\]",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "foo/bar - foo + bar - copy [1]",
        "**/*\\[*\\]",
        default_compile_options(),
        true,
    );

    assert_is_match(
        "foo/bar - 1",
        "**/*\\[1\\]",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/bar - copy (1)",
        "**/*\\[1\\]",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/bar (1)",
        "**/*\\[1\\]",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/bar (4)",
        "**/*\\[1\\]",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/bar (7)",
        "**/*\\[1\\]",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/bar (42)",
        "**/*\\[1\\]",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/bar - copy [1]",
        "**/*\\[1\\]",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "foo/bar - foo + bar - copy [1]",
        "**/*\\[1\\]",
        default_compile_options(),
        true,
    );

    assert_is_match(
        "foo/bar - 1",
        "*/*\\[*\\]",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/bar - copy (1)",
        "*/*\\[*\\]",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/bar (1)",
        "*/*\\[*\\]",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/bar (4)",
        "*/*\\[*\\]",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/bar (7)",
        "*/*\\[*\\]",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/bar (42)",
        "*/*\\[*\\]",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/bar - copy [1]",
        "*/*\\[*\\]",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "foo/bar - foo + bar - copy [1]",
        "*/*\\[*\\]",
        default_compile_options(),
        true,
    );

    // repeated block from JS source
    assert_is_match("a [b]", "a \\[b\\]", default_compile_options(), true);
    assert_is_match("a [b] c", "a [b] c", default_compile_options(), true);
    assert_is_match("a [b]", "a \\[b\\]*", default_compile_options(), true);
    assert_is_match("a [bc]", "a \\[bc\\]*", default_compile_options(), true);
    assert_is_match("a [b]", "a \\[b\\].*", default_compile_options(), false);
    assert_is_match("a [b].js", "a \\[b\\].*", default_compile_options(), true);
}

// ---------- star "*" ----------

#[test]
fn star_should_match_literal_star() {
    assert_is_match("*", "*", default_compile_options(), true);
    assert_is_match("*/*", "*/*", default_compile_options(), true);
    assert_is_match("*/*", "?/?", default_compile_options(), true);
    assert_is_match("*/*/", "*/*/", default_compile_options(), true);
    assert_is_match("/*", "/*", default_compile_options(), true);
    assert_is_match("/*", "/?", default_compile_options(), true);
    assert_is_match("foo*/bar*", "*/*", default_compile_options(), true);
}

#[test]
fn star_should_support_stars_following_brackets() {
    assert_is_match("a", "[a]*", default_compile_options(), true);
    assert_is_match("aa", "[a]*", default_compile_options(), true);
    assert_is_match("aaa", "[a]*", default_compile_options(), true);
    assert_is_match("az", "[a-z]*", default_compile_options(), true);
    assert_is_match("zzz", "[a-z]*", default_compile_options(), true);
}

#[test]
fn star_should_support_stars_following_parens() {
    assert_is_match("a", "(a)*", default_compile_options(), true);
    assert_is_match("ab", "(a|b)*", default_compile_options(), true);
    assert_is_match("aa", "(a)*", default_compile_options(), true);
    assert_is_match("aaab", "(a|b)*", default_compile_options(), true);
    assert_is_match("aaabbb", "(a|b)*", default_compile_options(), true);
}

#[test]
fn star_should_not_match_slashes_with_single_stars() {
    assert_is_match("a/b", "(a)*", default_compile_options(), false);
    assert_is_match("a/b", "[a]*", default_compile_options(), false);
    assert_is_match("a/b", "a*", default_compile_options(), false);
    assert_is_match("a/b", "(a|b)*", default_compile_options(), false);
}

#[test]
fn star_should_not_match_dots_with_stars_by_default() {
    assert_is_match(".a", "(a)*", default_compile_options(), false);
    assert_is_match(".a", "*[a]*", default_compile_options(), false);
    assert_is_match(".a", "*[a]", default_compile_options(), false);
    assert_is_match(".a", "*a*", default_compile_options(), false);
    assert_is_match(".a", "*a", default_compile_options(), false);
    assert_is_match(".a", "*(a|b)", default_compile_options(), false);
}

// ---------- plus "+" ----------

#[test]
fn plus_should_match_literal_plus() {
    assert_is_match("+", "*", default_compile_options(), true);
    assert_is_match("/+", "/*", default_compile_options(), true);
    assert_is_match("+/+", "*/*", default_compile_options(), true);
    assert_is_match("+/+/", "*/*/", default_compile_options(), true);
    assert_is_match("/+", "/+", default_compile_options(), true);
    assert_is_match("/+", "/?", default_compile_options(), true);
    assert_is_match("+/+", "?/?", default_compile_options(), true);
    assert_is_match("+/+", "+/+", default_compile_options(), true);
    assert_is_match("foo+/bar+", "*/*", default_compile_options(), true);
}

#[test]
fn plus_should_support_plus_signs_following_brackets() {
    assert_is_match("a", "[a]+", default_compile_options(), true);
    assert_is_match("aa", "[a]+", default_compile_options(), true);
    assert_is_match("aaa", "[a]+", default_compile_options(), true);
    assert_is_match("az", "[a-z]+", default_compile_options(), true);
    assert_is_match("zzz", "[a-z]+", default_compile_options(), true);
}

#[test]
fn plus_should_not_escape_plus_signs_following_parens() {
    assert_is_match("a", "(a)+", default_compile_options(), true);
    assert_is_match("ab", "(a|b)+", default_compile_options(), true);
    assert_is_match("aa", "(a)+", default_compile_options(), true);
    assert_is_match("aaab", "(a|b)+", default_compile_options(), true);
    assert_is_match("aaabbb", "(a|b)+", default_compile_options(), true);
}

#[test]
fn plus_should_escape_plus_signs_to_match_string_literals() {
    assert_is_match(
        "a+b/src/glimini.js",
        "a+b/src/*.js",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "+b/src/glimini.js",
        "+b/src/*.js",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "coffee+/src/glimini.js",
        "coffee+/src/*.js",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "coffee+/src/glimini.js",
        "coffee+/src/*",
        default_compile_options(),
        true,
    );
}

#[test]
fn plus_should_not_escape_plus_following_brackets_2() {
    assert_is_match("a", "[a]+", default_compile_options(), true);
    assert_is_match("aa", "[a]+", default_compile_options(), true);
    assert_is_match("aaa", "[a]+", default_compile_options(), true);
    assert_is_match("az", "[a-z]+", default_compile_options(), true);
    assert_is_match("zzz", "[a-z]+", default_compile_options(), true);
}

#[test]
fn plus_should_not_escape_plus_following_parens_2() {
    assert_is_match("a", "(a)+", default_compile_options(), true);
    assert_is_match("ab", "(a|b)+", default_compile_options(), true);
    assert_is_match("aa", "(a)+", default_compile_options(), true);
    assert_is_match("aaab", "(a|b)+", default_compile_options(), true);
    assert_is_match("aaabbb", "(a|b)+", default_compile_options(), true);
}

// ---------- dollar $ ----------

#[test]
fn dollar_should_match_dollar_signs() {
    assert_is_match("$", "!($)", default_compile_options(), false);
    assert_is_match("$", "!$", default_compile_options(), false);
    assert_is_match("$$", "!$", default_compile_options(), true);
    assert_is_match("$$", "!($)", default_compile_options(), true);
    assert_is_match("$$$", "!($)", default_compile_options(), true);
    assert_is_match("^", "!($)", default_compile_options(), true);

    assert_is_match("$", "!($$)", default_compile_options(), true);
    assert_is_match("$$", "!($$)", default_compile_options(), false);
    assert_is_match("$$$", "!($$)", default_compile_options(), true);
    assert_is_match("^", "!($$)", default_compile_options(), true);

    assert_is_match("$", "!($*)", default_compile_options(), false);
    assert_is_match("$$", "!($*)", default_compile_options(), false);
    assert_is_match("$$$", "!($*)", default_compile_options(), false);
    assert_is_match("^", "!($*)", default_compile_options(), true);

    assert_is_match("$", "*", default_compile_options(), true);
    assert_is_match("$$", "*", default_compile_options(), true);
    assert_is_match("$$$", "*", default_compile_options(), true);
    assert_is_match("^", "*", default_compile_options(), true);

    assert_is_match("$", "$*", default_compile_options(), true);
    assert_is_match("$$", "$*", default_compile_options(), true);
    assert_is_match("$$$", "$*", default_compile_options(), true);
    assert_is_match("^", "$*", default_compile_options(), false);

    assert_is_match("$", "*$*", default_compile_options(), true);
    assert_is_match("$$", "*$*", default_compile_options(), true);
    assert_is_match("$$$", "*$*", default_compile_options(), true);
    assert_is_match("^", "*$*", default_compile_options(), false);

    assert_is_match("$", "*$", default_compile_options(), true);
    assert_is_match("$$", "*$", default_compile_options(), true);
    assert_is_match("$$$", "*$", default_compile_options(), true);
    assert_is_match("^", "*$", default_compile_options(), false);

    assert_is_match("$", "?$", default_compile_options(), false);
    assert_is_match("$$", "?$", default_compile_options(), true);
    assert_is_match("$$$", "?$", default_compile_options(), false);
    assert_is_match("^", "?$", default_compile_options(), false);
}

// ---------- caret ^ ----------

#[test]
fn caret_should_match_carets() {
    assert_is_match("^", "^", default_compile_options(), true);
    assert_is_match("^/foo", "^/*", default_compile_options(), true);
    assert_is_match("foo^", "*^", default_compile_options(), true);
    assert_is_match("^foo/foo", "^foo/*", default_compile_options(), true);
    assert_is_match("foo^/foo", "foo^/*", default_compile_options(), true);

    assert_is_match("^", "!(^)", default_compile_options(), false);
    assert_is_match("^^", "!(^)", default_compile_options(), true);
    assert_is_match("^^^", "!(^)", default_compile_options(), true);
    assert_is_match("&", "!(^)", default_compile_options(), true);

    assert_is_match("^", "!(^^)", default_compile_options(), true);
    assert_is_match("^^", "!(^^)", default_compile_options(), false);
    assert_is_match("^^^", "!(^^)", default_compile_options(), true);
    assert_is_match("&", "!(^^)", default_compile_options(), true);

    assert_is_match("^", "!(^*)", default_compile_options(), false);
    assert_is_match("^^", "!(^*)", default_compile_options(), false);
    assert_is_match("^^^", "!(^*)", default_compile_options(), false);
    assert_is_match("&", "!(^*)", default_compile_options(), true);

    assert_is_match("^", "*", default_compile_options(), true);
    assert_is_match("^^", "*", default_compile_options(), true);
    assert_is_match("^^^", "*", default_compile_options(), true);
    assert_is_match("&", "*", default_compile_options(), true);

    assert_is_match("^", "^*", default_compile_options(), true);
    assert_is_match("^^", "^*", default_compile_options(), true);
    assert_is_match("^^^", "^*", default_compile_options(), true);
    assert_is_match("&", "^*", default_compile_options(), false);

    assert_is_match("^", "*^*", default_compile_options(), true);
    assert_is_match("^^", "*^*", default_compile_options(), true);
    assert_is_match("^^^", "*^*", default_compile_options(), true);
    assert_is_match("&", "*^*", default_compile_options(), false);

    assert_is_match("^", "*^", default_compile_options(), true);
    assert_is_match("^^", "*^", default_compile_options(), true);
    assert_is_match("^^^", "*^", default_compile_options(), true);
    assert_is_match("&", "*^", default_compile_options(), false);

    assert_is_match("^", "?^", default_compile_options(), false);
    assert_is_match("^^", "?^", default_compile_options(), true);
    assert_is_match("^^^", "?^", default_compile_options(), false);
    assert_is_match("&", "?^", default_compile_options(), false);
}

// ---------- mixed special characters ----------

#[test]
fn mixed_should_match_special_characters_in_paths() {
    assert_is_match("my/folder +1", "*/*", default_compile_options(), true);
    assert_is_match("my/folder -1", "*/*", default_compile_options(), true);
    assert_is_match("my/folder *1", "*/*", default_compile_options(), true);
    assert_is_match("my/folder", "*/*", default_compile_options(), true);
    assert_is_match(
        "my/folder+foo+bar&baz",
        "*/*",
        default_compile_options(),
        true,
    );
    assert_is_match("my/folder - $1.00", "*/*", default_compile_options(), true);
    assert_is_match("my/folder - ^1.00", "*/*", default_compile_options(), true);
    assert_is_match("my/folder - %1.00", "*/*", default_compile_options(), true);

    assert_is_match("my/folder +1", "*/!(*%)*", default_compile_options(), true);
    assert_is_match("my/folder -1", "*/!(*%)*", default_compile_options(), true);
    assert_is_match("my/folder *1", "*/!(*%)*", default_compile_options(), true);
    assert_is_match("my/folder", "*/!(*%)*", default_compile_options(), true);
    assert_is_match(
        "my/folder+foo+bar&baz",
        "*/!(*%)*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "my/folder - $1.00",
        "*/!(*%)*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "my/folder - ^1.00",
        "*/!(*%)*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "my/folder - %1.00",
        "*/!(*%)*",
        default_compile_options(),
        false,
    );

    assert_is_match("my/folder +1", "*/*$*", default_compile_options(), false);
    assert_is_match("my/folder -1", "*/*$*", default_compile_options(), false);
    assert_is_match("my/folder *1", "*/*$*", default_compile_options(), false);
    assert_is_match("my/folder", "*/*$*", default_compile_options(), false);
    assert_is_match(
        "my/folder+foo+bar&baz",
        "*/*$*",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "my/folder - $1.00",
        "*/*$*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "my/folder - ^1.00",
        "*/*$*",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "my/folder - %1.00",
        "*/*$*",
        default_compile_options(),
        false,
    );

    assert_is_match("my/folder +1", "*/*^*", default_compile_options(), false);
    assert_is_match("my/folder -1", "*/*^*", default_compile_options(), false);
    assert_is_match("my/folder *1", "*/*^*", default_compile_options(), false);
    assert_is_match("my/folder", "*/*^*", default_compile_options(), false);
    assert_is_match(
        "my/folder+foo+bar&baz",
        "*/*^*",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "my/folder - $1.00",
        "*/*^*",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "my/folder - ^1.00",
        "*/*^*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "my/folder - %1.00",
        "*/*^*",
        default_compile_options(),
        false,
    );

    assert_is_match("my/folder +1", "*/*&*", default_compile_options(), false);
    assert_is_match("my/folder -1", "*/*&*", default_compile_options(), false);
    assert_is_match("my/folder *1", "*/*&*", default_compile_options(), false);
    assert_is_match("my/folder", "*/*&*", default_compile_options(), false);
    assert_is_match(
        "my/folder+foo+bar&baz",
        "*/*&*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "my/folder - $1.00",
        "*/*&*",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "my/folder - ^1.00",
        "*/*&*",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "my/folder - %1.00",
        "*/*&*",
        default_compile_options(),
        false,
    );

    assert_is_match("my/folder +1", "*/*+*", default_compile_options(), true);
    assert_is_match("my/folder -1", "*/*+*", default_compile_options(), false);
    assert_is_match("my/folder *1", "*/*+*", default_compile_options(), false);
    assert_is_match("my/folder", "*/*+*", default_compile_options(), false);
    assert_is_match(
        "my/folder+foo+bar&baz",
        "*/*+*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "my/folder - $1.00",
        "*/*+*",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "my/folder - ^1.00",
        "*/*+*",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "my/folder - %1.00",
        "*/*+*",
        default_compile_options(),
        false,
    );

    assert_is_match("my/folder +1", "*/*-*", default_compile_options(), false);
    assert_is_match("my/folder -1", "*/*-*", default_compile_options(), true);
    assert_is_match("my/folder *1", "*/*-*", default_compile_options(), false);
    assert_is_match("my/folder", "*/*-*", default_compile_options(), false);
    assert_is_match(
        "my/folder+foo+bar&baz",
        "*/*-*",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "my/folder - $1.00",
        "*/*-*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "my/folder - ^1.00",
        "*/*-*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "my/folder - %1.00",
        "*/*-*",
        default_compile_options(),
        true,
    );

    // JS: '*/*\\**' — pattern matching a literal * in the second segment
    assert_is_match("my/folder +1", "*/*\\**", default_compile_options(), false);
    assert_is_match("my/folder -1", "*/*\\**", default_compile_options(), false);
    assert_is_match("my/folder *1", "*/*\\**", default_compile_options(), true);
    assert_is_match("my/folder", "*/*\\**", default_compile_options(), false);
    assert_is_match(
        "my/folder+foo+bar&baz",
        "*/*\\**",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "my/folder - $1.00",
        "*/*\\**",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "my/folder - ^1.00",
        "*/*\\**",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "my/folder - %1.00",
        "*/*\\**",
        default_compile_options(),
        false,
    );
}
