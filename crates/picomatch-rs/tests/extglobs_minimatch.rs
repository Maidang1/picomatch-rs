mod support;

use picomatch_rs::CompileOptions;

use support::{assert_is_match, default_compile_options};

#[macro_export]
macro_rules! assert_is_match {
    ($input:expr, $pattern:expr, $opts:expr, $expected:expr) => {
        assert_is_match($input, $pattern, $opts, $expected)
    };
}

fn windows_opts() -> CompileOptions {
    let mut opts = default_compile_options();
    opts.windows = true;
    opts
}

fn posix_opts() -> CompileOptions {
    let mut opts = default_compile_options();
    opts.windows = false;
    opts
}

#[test]
fn test_extglobs_minimatch_basic() {
    let opts = windows_opts();

    assert_is_match("", "*(0|1|3|5|7|9)", opts.clone(), false);

    assert_is_match!("*(a|b[)", "*(a|b\\[)", opts.clone(), false);
    assert_is_match!("*(a|b[)", "\\*\\(a\\|b\\[\\)", opts.clone(), true);
    assert_is_match!("***", "\\*\\*\\*", opts.clone(), true);
    assert_is_match!(
        "-adobe-courier-bold-o-normal--12-120-75-75-/-70-iso8859-1",
        "-*-*-*-*-*-*-12-*-*-*-m-*-*-*",
        opts.clone(),
        false
    );
    assert_is_match!(
        "-adobe-courier-bold-o-normal--12-120-75-75-m-70-iso8859-1",
        "-*-*-*-*-*-*-12-*-*-*-m-*-*-*",
        opts.clone(),
        true
    );
    assert_is_match!(
        "-adobe-courier-bold-o-normal--12-120-75-75-X-70-iso8859-1",
        "-*-*-*-*-*-*-12-*-*-*-m-*-*-*",
        opts.clone(),
        false
    );
    assert_is_match!(
        "/dev/udp/129.22.8.102/45",
        "/dev\\/@(tcp|udp)\\/*\\/*",
        opts.clone(),
        true
    );
    assert_is_match!("/x/y/z", "/x/y/z", opts.clone(), true);
    assert_is_match!("0377", "+([0-7])", opts.clone(), true);
    assert_is_match!("07", "+([0-7])", opts.clone(), true);
    assert_is_match!("09", "+([0-7])", opts.clone(), false);
    assert_is_match!("1", "0|[1-9]*([0-9])", opts.clone(), true);
    assert_is_match!("12", "0|[1-9]*([0-9])", opts.clone(), true);
    assert_is_match!("123abc", "(a+|b)*", opts.clone(), false);
    assert_is_match!("123abc", "(a+|b)+", opts.clone(), false);
    assert_is_match!("123abc", "*?(a)bc", opts.clone(), true);
    assert_is_match!("123abc", "a(b*(foo|bar))d", opts.clone(), false);
    assert_is_match!("123abc", "ab*(e|f)", opts.clone(), false);
    assert_is_match!("123abc", "ab**", opts.clone(), false);
    assert_is_match!("123abc", "ab**(e|f)", opts.clone(), false);
    assert_is_match!("123abc", "ab**(e|f)g", opts.clone(), false);
    assert_is_match!("123abc", "ab***ef", opts.clone(), false);
    assert_is_match!("123abc", "ab*+(e|f)", opts.clone(), false);
    assert_is_match!("123abc", "ab*d+(e|f)", opts.clone(), false);
    assert_is_match!("123abc", "ab?*(e|f)", opts.clone(), false);
    assert_is_match!("12abc", "0|[1-9]*([0-9])", opts.clone(), false);
    assert_is_match!("137577991", "*(0|1|3|5|7|9)", opts.clone(), true);
    assert_is_match!("2468", "*(0|1|3|5|7|9)", opts.clone(), false);
    assert_is_match!("?a?b", "\\??\\?b", opts.clone(), true);
    assert_is_match!("\\a\\b\\c", "abc", opts.clone(), false);
    assert_is_match!("a", "!(*.a|*.b|*.c)", opts.clone(), true);
    assert_is_match!("a", "!(a)", opts.clone(), false);
    assert_is_match!("a", "!(a)*", opts.clone(), false);
    assert_is_match!("a", "(a)", opts.clone(), true);
    assert_is_match!("a", "(b)", opts.clone(), false);
    assert_is_match!("a", "*(a)", opts.clone(), true);
    assert_is_match!("a", "+(a)", opts.clone(), true);
    assert_is_match!("a", "?", opts.clone(), true);
    assert_is_match!("a", "?(a|b)", opts.clone(), true);
    assert_is_match!("a", "??", opts.clone(), false);
    assert_is_match!("a", "a!(b)*", opts.clone(), true);
    assert_is_match!("a", "a?(a|b)", opts.clone(), true);
    assert_is_match!("a", "a?(x)", opts.clone(), true);
    assert_is_match!("a", "a??b", opts.clone(), false);
    assert_is_match!("a", "b?(a|b)", opts.clone(), false);
    assert_is_match!("a((((b", "a(*b", opts.clone(), true);
    assert_is_match!("a((((b", "a(b", opts.clone(), false);
    assert_is_match!("a((((b", "a\\(b", opts.clone(), false);
    assert_is_match!("a((b", "a(*b", opts.clone(), true);
    assert_is_match!("a((b", "a(b", opts.clone(), false);
    assert_is_match!("a((b", "a\\(b", opts.clone(), false);
    assert_is_match!("a(b", "a(*b", opts.clone(), true);
    assert_is_match!("a(b", "a(b", opts.clone(), true);
    assert_is_match!("a(b", "a\\(b", opts.clone(), true);
    assert_is_match!("a.", "!(*.a|*.b|*.c)", opts.clone(), true);
    assert_is_match!("a.", "*!(.a|.b|.c)", opts.clone(), true);
    assert_is_match!("a.", "*.!(a)", opts.clone(), true);
    assert_is_match!("a.", "*.!(a|b|c)", opts.clone(), true);
    assert_is_match!("a.", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), false);
    assert_is_match!("a.", "*.+(b|d)", opts.clone(), false);
    assert_is_match!("a.a", "!(*.[a-b]*)", opts.clone(), false);
    assert_is_match!("a.a", "!(*.a|*.b|*.c)", opts.clone(), false);
    assert_is_match!("a.a", "!(*[a-b].[a-b]*)", opts.clone(), false);
    assert_is_match!("a.a", "!*.(a|b)", opts.clone(), false);
    assert_is_match!("a.a", "!*.(a|b)*", opts.clone(), false);
    assert_is_match!("a.a", "(a|d).(a|b)*", opts.clone(), true);
    assert_is_match!("a.a", "(b|a).(a)", opts.clone(), true);
    assert_is_match!("a.a", "*!(.a|.b|.c)", opts.clone(), true);
    assert_is_match!("a.a", "*.!(a)", opts.clone(), false);
    assert_is_match!("a.a", "*.!(a|b|c)", opts.clone(), false);
    assert_is_match!("a.a", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), true);
    assert_is_match!("a.a", "*.+(b|d)", opts.clone(), false);
    assert_is_match!("a.a", "@(b|a).@(a)", opts.clone(), true);
    assert_is_match!("a.a.a", "!(*.[a-b]*)", opts.clone(), false);
    assert_is_match!("a.a.a", "!(*[a-b].[a-b]*)", opts.clone(), false);
    assert_is_match!("a.a.a", "!*.(a|b)", opts.clone(), false);
    assert_is_match!("a.a.a", "!*.(a|b)*", opts.clone(), false);
    assert_is_match!("a.a.a", "*.!(a)", opts.clone(), true);
    assert_is_match!("a.a.a", "*.+(b|d)", opts.clone(), false);
    assert_is_match!("a.aa.a", "(b|a).(a)", opts.clone(), false);
    assert_is_match!("a.aa.a", "@(b|a).@(a)", opts.clone(), false);
    assert_is_match!("a.abcd", "!(*.a|*.b|*.c)", opts.clone(), true);
    assert_is_match!("a.abcd", "!(*.a|*.b|*.c)*", opts.clone(), false);
    assert_is_match!("a.abcd", "*!(*.a|*.b|*.c)*", opts.clone(), true);
    assert_is_match!("a.abcd", "*!(.a|.b|.c)", opts.clone(), true);
    assert_is_match!("a.abcd", "*.!(a|b|c)", opts.clone(), true);
    assert_is_match!("a.abcd", "*.!(a|b|c)*", opts.clone(), false);
    assert_is_match!("a.abcd", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), true);
    assert_is_match!("a.b", "!(*.*)", opts.clone(), false);
    assert_is_match!("a.b", "!(*.[a-b]*)", opts.clone(), false);
    assert_is_match!("a.b", "!(*.a|*.b|*.c)", opts.clone(), false);
    assert_is_match!("a.b", "!(*[a-b].[a-b]*)", opts.clone(), false);
    assert_is_match!("a.b", "!*.(a|b)", opts.clone(), false);
    assert_is_match!("a.b", "!*.(a|b)*", opts.clone(), false);
    assert_is_match!("a.b", "(a|d).(a|b)*", opts.clone(), true);
    assert_is_match!("a.b", "*!(.a|.b|.c)", opts.clone(), true);
    assert_is_match!("a.b", "*.!(a)", opts.clone(), true);
    assert_is_match!("a.b", "*.!(a|b|c)", opts.clone(), false);
    assert_is_match!("a.b", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), true);
    assert_is_match!("a.b", "*.+(b|d)", opts.clone(), true);
    assert_is_match!("a.bb", "!(*.[a-b]*)", opts.clone(), false);
    assert_is_match!("a.bb", "!(*[a-b].[a-b]*)", opts.clone(), false);
    assert_is_match!("a.bb", "!*.(a|b)", opts.clone(), true);
    assert_is_match!("a.bb", "!*.(a|b)*", opts.clone(), false);
    assert_is_match!("a.bb", "!*.*(a|b)", opts.clone(), false);
    assert_is_match!("a.bb", "(a|d).(a|b)*", opts.clone(), true);
    assert_is_match!("a.bb", "(b|a).(a)", opts.clone(), false);
    assert_is_match!("a.bb", "*.+(b|d)", opts.clone(), true);
    assert_is_match!("a.bb", "@(b|a).@(a)", opts.clone(), false);
    assert_is_match!("a.c", "!(*.a|*.b|*.c)", opts.clone(), false);
    assert_is_match!("a.c", "*!(.a|.b|.c)", opts.clone(), true);
    assert_is_match!("a.c", "*.!(a|b|c)", opts.clone(), false);
    assert_is_match!("a.c", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), false);
    assert_is_match!("a.c.d", "!(*.a|*.b|*.c)", opts.clone(), true);
    assert_is_match!("a.c.d", "*!(.a|.b|.c)", opts.clone(), true);
    assert_is_match!("a.c.d", "*.!(a|b|c)", opts.clone(), true);
    assert_is_match!("a.c.d", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), false);
    assert_is_match!("a.ccc", "!(*.[a-b]*)", opts.clone(), true);
    assert_is_match!("a.ccc", "!(*[a-b].[a-b]*)", opts.clone(), true);
    assert_is_match!("a.ccc", "!*.(a|b)", opts.clone(), true);
    assert_is_match!("a.ccc", "!*.(a|b)*", opts.clone(), true);
    assert_is_match!("a.ccc", "*.+(b|d)", opts.clone(), false);
    assert_is_match!("a.js", "!(*.js)", opts.clone(), false);
    assert_is_match!("a.js", "*!(.js)", opts.clone(), true);
    assert_is_match!("a.js", "*.!(js)", opts.clone(), false);
    assert_is_match!("a.js", "a.!(js)", opts.clone(), false);
    assert_is_match!("a.js", "a.!(js)*", opts.clone(), false);
    assert_is_match!("a.js.js", "!(*.js)", opts.clone(), false);
    assert_is_match!("a.js.js", "*!(.js)", opts.clone(), true);
    assert_is_match!("a.js.js", "*.!(js)", opts.clone(), true);
    assert_is_match!("a.js.js", "*.*(js).js", opts.clone(), true);
    assert_is_match!("a.md", "!(*.js)", opts.clone(), true);
    assert_is_match!("a.md", "*!(.js)", opts.clone(), true);
    assert_is_match!("a.md", "*.!(js)", opts.clone(), true);
    assert_is_match!("a.md", "a.!(js)", opts.clone(), true);
    assert_is_match!("a.md", "a.!(js)*", opts.clone(), true);
    assert_is_match!("a.md.js", "*.*(js).js", opts.clone(), false);
    assert_is_match!("a.txt", "a.!(js)", opts.clone(), true);
    assert_is_match!("a.txt", "a.!(js)*", opts.clone(), true);
    assert_is_match!("a/!(z)", "a/!(z)", opts.clone(), true);
    assert_is_match!("a/b", "a/!(z)", opts.clone(), true);
    assert_is_match!("a/b/c.txt", "*/b/!(*).txt", opts.clone(), false);
    assert_is_match!("a/b/c.txt", "*/b/!(c).txt", opts.clone(), false);
    assert_is_match!("a/b/c.txt", "*/b/!(cc).txt", opts.clone(), true);
    assert_is_match!("a/b/cc.txt", "*/b/!(*).txt", opts.clone(), false);
    assert_is_match!("a/b/cc.txt", "*/b/!(c).txt", opts.clone(), false);
    assert_is_match!("a/b/cc.txt", "*/b/!(cc).txt", opts.clone(), false);
    assert_is_match!("a/dir/foo.txt", "*/dir/**/!(bar).txt", opts.clone(), true);
    assert_is_match!("a/z", "a/!(z)", opts.clone(), false);
    assert_is_match!("a\\(b", "a(*b", opts.clone(), false);
    assert_is_match!("a\\(b", "a(b", opts.clone(), false);
    assert_is_match!("a\\z", "a\\\\z", opts.clone(), false);
    assert_is_match!("a\\\\z", "a\\\\z", opts.clone(), true);
    assert_is_match!("a\\b", "a/b", opts.clone(), true);
    assert_is_match!("a\\z", "a\\\\z", posix_opts(), true);
    assert_is_match!("a\\z", "a\\z", opts.clone(), true);
    assert_is_match!("aa", "!(a!(b))", opts.clone(), false);
    assert_is_match!("aa", "!(a)", opts.clone(), true);
    assert_is_match!("aa", "!(a)*", opts.clone(), false);
    assert_is_match!("aa", "?", opts.clone(), false);
    assert_is_match!("aa", "@(a)b", opts.clone(), false);
    assert_is_match!("aa", "a!(b)*", opts.clone(), true);
    assert_is_match!("aa", "a??b", opts.clone(), false);
    assert_is_match!("aa.aa", "(b|a).(a)", opts.clone(), false);
    assert_is_match!("aa.aa", "@(b|a).@(a)", opts.clone(), false);
    assert_is_match!("aaa", "!(a)*", opts.clone(), false);
    assert_is_match!("aaa", "a!(b)*", opts.clone(), true);
    assert_is_match!("aaaaaaabababab", "*ab", opts.clone(), true);
    assert_is_match!("aaac", "*(@(a))a@(c)", opts.clone(), true);
    assert_is_match!("aaaz", "[a*(]*z", opts.clone(), true);
    assert_is_match!("aab", "!(a)*", opts.clone(), false);
    assert_is_match!("aab", "?", opts.clone(), false);
    assert_is_match!("aab", "??", opts.clone(), false);
    assert_is_match!("aab", "@(c)b", opts.clone(), false);
    assert_is_match!("aab", "a!(b)*", opts.clone(), true);
    assert_is_match!("aab", "a??b", opts.clone(), false);
    assert_is_match!("aac", "*(@(a))a@(c)", opts.clone(), true);
    assert_is_match!("aac", "*(@(a))b@(c)", opts.clone(), false);
    assert_is_match!("aax", "a!(a*|b)", opts.clone(), false);
    assert_is_match!("aax", "a!(x*|b)", opts.clone(), true);
    assert_is_match!("aax", "a?(a*|b)", opts.clone(), true);
    assert_is_match!("aaz", "[a*(]*z", opts.clone(), true);
    assert_is_match!("ab", "!(*.*)", opts.clone(), true);
    assert_is_match!("ab", "!(a!(b))", opts.clone(), true);
    assert_is_match!("ab", "!(a)*", opts.clone(), false);
    assert_is_match!("ab", "(a+|b)*", opts.clone(), true);
    assert_is_match!("ab", "(a+|b)+", opts.clone(), true);
    assert_is_match!("ab", "*?(a)bc", opts.clone(), false);
    assert_is_match!("ab", "a!(*(b|B))", opts.clone(), false);
    assert_is_match!("ab", "a!(@(b|B))", opts.clone(), false);
    assert_is_match!("aB", "a!(@(b|B))", opts.clone(), false);
    assert_is_match!("ab", "a!(b)*", opts.clone(), false);
    assert_is_match!("ab", "a(*b", opts.clone(), false);
    assert_is_match!("ab", "a(b", opts.clone(), false);
    assert_is_match!("ab", "a(b*(foo|bar))d", opts.clone(), false);
    assert_is_match!("ab", "a/b", opts.clone(), false);
    assert_is_match!("ab", "a\\(b", opts.clone(), false);
    assert_is_match!("ab", "ab*(e|f)", opts.clone(), true);
    assert_is_match!("ab", "ab**", opts.clone(), true);
    assert_is_match!("ab", "ab**(e|f)", opts.clone(), true);
    assert_is_match!("ab", "ab**(e|f)g", opts.clone(), false);
    assert_is_match!("ab", "ab***ef", opts.clone(), false);
    assert_is_match!("ab", "ab*+(e|f)", opts.clone(), false);
    assert_is_match!("ab", "ab*d+(e|f)", opts.clone(), false);
    assert_is_match!("ab", "ab?*(e|f)", opts.clone(), false);
    assert_is_match!("ab/cXd/efXg/hi", "**/*X*/**/*i", opts.clone(), true);
    assert_is_match!("ab/cXd/efXg/hi", "*/*X*/*/*i", opts.clone(), true);
    assert_is_match!("ab/cXd/efXg/hi", "*X*i", opts.clone(), false);
    assert_is_match!("ab/cXd/efXg/hi", "*Xg*i", opts.clone(), false);
    assert_is_match!("ab]", "a!(@(b|B))", opts.clone(), true);
    assert_is_match!("abab", "(a+|b)*", opts.clone(), true);
    assert_is_match!("abab", "(a+|b)+", opts.clone(), true);
    assert_is_match!("abab", "*?(a)bc", opts.clone(), false);
    assert_is_match!("abab", "a(b*(foo|bar))d", opts.clone(), false);
    assert_is_match!("abab", "ab*(e|f)", opts.clone(), false);
    assert_is_match!("abab", "ab**", opts.clone(), true);
    assert_is_match!("abab", "ab**(e|f)", opts.clone(), true);
    assert_is_match!("abab", "ab**(e|f)g", opts.clone(), false);
    assert_is_match!("abab", "ab***ef", opts.clone(), false);
    assert_is_match!("abab", "ab*+(e|f)", opts.clone(), false);
    assert_is_match!("abab", "ab*d+(e|f)", opts.clone(), false);
    assert_is_match!("abab", "ab?*(e|f)", opts.clone(), false);
    assert_is_match!("abb", "!(*.*)", opts.clone(), true);
    assert_is_match!("abb", "!(a)*", opts.clone(), false);
    assert_is_match!("abb", "a!(b)*", opts.clone(), false);
    assert_is_match!("abbcd", "@(ab|a*(b))*(c)d", opts.clone(), true);
    assert_is_match!("abc", "\\a\\b\\c", opts.clone(), false);
    assert_is_match!("aBc", "a!(@(b|B))", opts.clone(), true);
    assert_is_match!("abcd", "?@(a|b)*@(c)d", opts.clone(), true);
    assert_is_match!("abcd", "@(ab|a*@(b))*(c)d", opts.clone(), true);
    assert_is_match!(
        "abcd/abcdefg/abcdefghijk/abcdefghijklmnop.txt",
        "**/*a*b*g*n*t",
        opts.clone(),
        true
    );
    assert_is_match!(
        "abcd/abcdefg/abcdefghijk/abcdefghijklmnop.txtz",
        "**/*a*b*g*n*t",
        opts.clone(),
        false
    );
    assert_is_match!("abcdef", "(a+|b)*", opts.clone(), true);
    assert_is_match!("abcdef", "(a+|b)+", opts.clone(), false);
    assert_is_match!("abcdef", "*?(a)bc", opts.clone(), false);
    assert_is_match!("abcdef", "a(b*(foo|bar))d", opts.clone(), false);
    assert_is_match!("abcdef", "ab*(e|f)", opts.clone(), false);
    assert_is_match!("abcdef", "ab**", opts.clone(), true);
    assert_is_match!("abcdef", "ab**(e|f)", opts.clone(), true);
    assert_is_match!("abcdef", "ab**(e|f)g", opts.clone(), false);
    assert_is_match!("abcdef", "ab***ef", opts.clone(), true);
    assert_is_match!("abcdef", "ab*+(e|f)", opts.clone(), true);
    assert_is_match!("abcdef", "ab*d+(e|f)", opts.clone(), true);
    assert_is_match!("abcdef", "ab?*(e|f)", opts.clone(), false);
    assert_is_match!("abcfef", "(a+|b)*", opts.clone(), true);
    assert_is_match!("abcfef", "(a+|b)+", opts.clone(), false);
    assert_is_match!("abcfef", "*?(a)bc", opts.clone(), false);
    assert_is_match!("abcfef", "a(b*(foo|bar))d", opts.clone(), false);
    assert_is_match!("abcfef", "ab*(e|f)", opts.clone(), false);
    assert_is_match!("abcfef", "ab**", opts.clone(), true);
    assert_is_match!("abcfef", "ab**(e|f)", opts.clone(), true);
    assert_is_match!("abcfef", "ab**(e|f)g", opts.clone(), false);
    assert_is_match!("abcfef", "ab***ef", opts.clone(), true);
    assert_is_match!("abcfef", "ab*+(e|f)", opts.clone(), true);
    assert_is_match!("abcfef", "ab*d+(e|f)", opts.clone(), false);
    assert_is_match!("abcfef", "ab?*(e|f)", opts.clone(), true);
    assert_is_match!("abcfefg", "(a+|b)*", opts.clone(), true);
    assert_is_match!("abcfefg", "(a+|b)+", opts.clone(), false);
    assert_is_match!("abcfefg", "*?(a)bc", opts.clone(), false);
    assert_is_match!("abcfefg", "a(b*(foo|bar))d", opts.clone(), false);
    assert_is_match!("abcfefg", "ab*(e|f)", opts.clone(), false);
    assert_is_match!("abcfefg", "ab**", opts.clone(), true);
    assert_is_match!("abcfefg", "ab**(e|f)", opts.clone(), true);
    assert_is_match!("abcfefg", "ab**(e|f)g", opts.clone(), true);
    assert_is_match!("abcfefg", "ab***ef", opts.clone(), false);
    assert_is_match!("abcfefg", "ab*+(e|f)", opts.clone(), false);
    assert_is_match!("abcfefg", "ab*d+(e|f)", opts.clone(), false);
    assert_is_match!("abcfefg", "ab?*(e|f)", opts.clone(), false);
    assert_is_match!("abcx", "!([[*])*", opts.clone(), true);
    assert_is_match!("abcx", "+(a|b\\[)*", opts.clone(), true);
    assert_is_match!("abcx", "[a*(]*z", opts.clone(), false);
    assert_is_match!("abcXdefXghi", "*X*i", opts.clone(), true);
    assert_is_match!("abcz", "!([[*])*", opts.clone(), true);
    assert_is_match!("abcz", "+(a|b\\[)*", opts.clone(), true);
    assert_is_match!("abcz", "[a*(]*z", opts.clone(), true);
    assert_is_match!("abd", "(a+|b)*", opts.clone(), true);
    assert_is_match!("abd", "(a+|b)+", opts.clone(), false);
    assert_is_match!("abd", "*?(a)bc", opts.clone(), false);
    assert_is_match!("abd", "a!(*(b|B))", opts.clone(), true);
    assert_is_match!("abd", "a!(@(b|B))", opts.clone(), true);
    assert_is_match!("abd", "a!(@(b|B))d", opts.clone(), false);
    assert_is_match!("abd", "a(b*(foo|bar))d", opts.clone(), true);
    assert_is_match!("abd", "a+(b|c)d", opts.clone(), true);
    assert_is_match!("abd", "a[b*(foo|bar)]d", opts.clone(), true);
    assert_is_match!("abd", "ab*(e|f)", opts.clone(), false);
    assert_is_match!("abd", "ab**", opts.clone(), true);
    assert_is_match!("abd", "ab**(e|f)", opts.clone(), true);
    assert_is_match!("abd", "ab**(e|f)g", opts.clone(), false);
    assert_is_match!("abd", "ab***ef", opts.clone(), false);
    assert_is_match!("abd", "ab*+(e|f)", opts.clone(), false);
    assert_is_match!("abd", "ab*d+(e|f)", opts.clone(), false);
    assert_is_match!("abd", "ab?*(e|f)", opts.clone(), true);
    assert_is_match!("abef", "(a+|b)*", opts.clone(), true);
    assert_is_match!("abef", "(a+|b)+", opts.clone(), false);
    assert_is_match!("abef", "*(a+|b)", opts.clone(), false);
    assert_is_match!("abef", "*?(a)bc", opts.clone(), false);
    assert_is_match!("abef", "a(b*(foo|bar))d", opts.clone(), false);
    assert_is_match!("abef", "ab*(e|f)", opts.clone(), true);
    assert_is_match!("abef", "ab**", opts.clone(), true);
    assert_is_match!("abef", "ab**(e|f)", opts.clone(), true);
    assert_is_match!("abef", "ab**(e|f)g", opts.clone(), false);
    assert_is_match!("abef", "ab***ef", opts.clone(), true);
    assert_is_match!("abef", "ab*+(e|f)", opts.clone(), true);
    assert_is_match!("abef", "ab*d+(e|f)", opts.clone(), false);
    assert_is_match!("abef", "ab?*(e|f)", opts.clone(), true);
    assert_is_match!("abz", "a!(*)", opts.clone(), false);
    assert_is_match!("abz", "a!(z)", opts.clone(), true);
    assert_is_match!("abz", "a*!(z)", opts.clone(), true);
    assert_is_match!("abz", "a*(z)", opts.clone(), false);
    assert_is_match!("abz", "a**(z)", opts.clone(), true);
    assert_is_match!("abz", "a*@(z)", opts.clone(), true);
    assert_is_match!("abz", "a+(z)", opts.clone(), false);
    assert_is_match!("abz", "a?(z)", opts.clone(), false);
    assert_is_match!("abz", "a@(z)", opts.clone(), false);
    assert_is_match!("ac", "!(a)*", opts.clone(), false);
    assert_is_match!("ac", "*(@(a))a@(c)", opts.clone(), true);
    assert_is_match!("ac", "a!(*(b|B))", opts.clone(), true);
    assert_is_match!("ac", "a!(@(b|B))", opts.clone(), true);
    assert_is_match!("ac", "a!(b)*", opts.clone(), true);
    assert_is_match!("accdef", "(a+|b)*", opts.clone(), true);
    assert_is_match!("accdef", "(a+|b)+", opts.clone(), false);
    assert_is_match!("accdef", "*?(a)bc", opts.clone(), false);
    assert_is_match!("accdef", "a(b*(foo|bar))d", opts.clone(), false);
    assert_is_match!("accdef", "ab*(e|f)", opts.clone(), false);
    assert_is_match!("accdef", "ab**", opts.clone(), false);
    assert_is_match!("accdef", "ab**(e|f)", opts.clone(), false);
    assert_is_match!("accdef", "ab**(e|f)g", opts.clone(), false);
    assert_is_match!("accdef", "ab***ef", opts.clone(), false);
    assert_is_match!("accdef", "ab*+(e|f)", opts.clone(), false);
    assert_is_match!("accdef", "ab*d+(e|f)", opts.clone(), false);
    assert_is_match!("accdef", "ab?*(e|f)", opts.clone(), false);
    assert_is_match!("acd", "(a+|b)*", opts.clone(), true);
    assert_is_match!("acd", "(a+|b)+", opts.clone(), false);
    assert_is_match!("acd", "*?(a)bc", opts.clone(), false);
    assert_is_match!("acd", "@(ab|a*(b))*(c)d", opts.clone(), true);
    assert_is_match!("acd", "a!(*(b|B))", opts.clone(), true);
    assert_is_match!("acd", "a!(@(b|B))", opts.clone(), true);
    assert_is_match!("acd", "a!(@(b|B))d", opts.clone(), true);
    assert_is_match!("acd", "a(b*(foo|bar))d", opts.clone(), false);
    assert_is_match!("acd", "a+(b|c)d", opts.clone(), true);
    assert_is_match!("acd", "a[b*(foo|bar)]d", opts.clone(), false);
    assert_is_match!("acd", "ab*(e|f)", opts.clone(), false);
    assert_is_match!("acd", "ab**", opts.clone(), false);
    assert_is_match!("acd", "ab**(e|f)", opts.clone(), false);
    assert_is_match!("acd", "ab**(e|f)g", opts.clone(), false);
    assert_is_match!("acd", "ab***ef", opts.clone(), false);
    assert_is_match!("acd", "ab*+(e|f)", opts.clone(), false);
    assert_is_match!("acd", "ab*d+(e|f)", opts.clone(), false);
    assert_is_match!("acd", "ab?*(e|f)", opts.clone(), false);
    assert_is_match!("axz", "a+(z)", opts.clone(), false);
    assert_is_match!("az", "a!(*)", opts.clone(), false);
    assert_is_match!("az", "a!(z)", opts.clone(), false);
    assert_is_match!("az", "a*!(z)", opts.clone(), true);
    assert_is_match!("az", "a*(z)", opts.clone(), true);
    assert_is_match!("az", "a**(z)", opts.clone(), true);
    assert_is_match!("az", "a*@(z)", opts.clone(), true);
    assert_is_match!("az", "a+(z)", opts.clone(), true);
    assert_is_match!("az", "a?(z)", opts.clone(), true);
    assert_is_match!("az", "a@(z)", opts.clone(), true);
    assert_is_match!("az", "a\\\\z", opts.clone(), false);
    assert_is_match!("az", "a\\\\z", opts.clone(), false);
    assert_is_match!("b", "!(a)*", opts.clone(), true);
    assert_is_match!("b", "(a+|b)*", opts.clone(), true);
    assert_is_match!("b", "a!(b)*", opts.clone(), false);
    assert_is_match!("b.a", "(b|a).(a)", opts.clone(), true);
    assert_is_match!("b.a", "@(b|a).@(a)", opts.clone(), true);
    assert_is_match!("b/a", "!(b/a)", opts.clone(), false);
    assert_is_match!("b/b", "!(b/a)", opts.clone(), true);
    assert_is_match!("b/c", "!(b/a)", opts.clone(), true);
    assert_is_match!("b/c", "b/!(c)", opts.clone(), false);
    assert_is_match!("b/c", "b/!(cc)", opts.clone(), true);
    assert_is_match!("b/c.txt", "b/!(c).txt", opts.clone(), false);
    assert_is_match!("b/c.txt", "b/!(cc).txt", opts.clone(), true);
    assert_is_match!("b/cc", "b/!(c)", opts.clone(), true);
    assert_is_match!("b/cc", "b/!(cc)", opts.clone(), false);
    assert_is_match!("b/cc.txt", "b/!(c).txt", opts.clone(), false);
    assert_is_match!("b/cc.txt", "b/!(cc).txt", opts.clone(), false);
    assert_is_match!("b/ccc", "b/!(c)", opts.clone(), true);
    assert_is_match!("ba", "!(a!(b))", opts.clone(), true);
    assert_is_match!("ba", "b?(a|b)", opts.clone(), true);
    assert_is_match!("baaac", "*(@(a))a@(c)", opts.clone(), false);
    assert_is_match!("bar", "!(foo)", opts.clone(), true);
    assert_is_match!("bar", "!(foo)*", opts.clone(), true);
    assert_is_match!("bar", "!(foo)b*", opts.clone(), true);
    assert_is_match!("bar", "*(!(foo))", opts.clone(), true);
    assert_is_match!("baz", "!(foo)*", opts.clone(), true);
    assert_is_match!("baz", "!(foo)b*", opts.clone(), true);
    assert_is_match!("baz", "*(!(foo))", opts.clone(), true);
    assert_is_match!("bb", "!(a!(b))", opts.clone(), true);
    assert_is_match!("bb", "!(a)*", opts.clone(), true);
    assert_is_match!("bb", "a!(b)*", opts.clone(), false);
    assert_is_match!("bb", "a?(a|b)", opts.clone(), false);
    assert_is_match!("bbc", "!([[*])*", opts.clone(), true);
    assert_is_match!("bbc", "+(a|b\\[)*", opts.clone(), false);
    assert_is_match!("bbc", "[a*(]*z", opts.clone(), false);
    assert_is_match!("bz", "a+(z)", opts.clone(), false);
    assert_is_match!("c", "*(@(a))a@(c)", opts.clone(), false);
    assert_is_match!("c.a", "!(*.[a-b]*)", opts.clone(), false);
    assert_is_match!("c.a", "!(*[a-b].[a-b]*)", opts.clone(), true);
    assert_is_match!("c.a", "!*.(a|b)", opts.clone(), false);
    assert_is_match!("c.a", "!*.(a|b)*", opts.clone(), false);
    assert_is_match!("c.a", "(b|a).(a)", opts.clone(), false);
    assert_is_match!("c.a", "*.!(a)", opts.clone(), false);
    assert_is_match!("c.a", "*.+(b|d)", opts.clone(), false);
    assert_is_match!("c.a", "@(b|a).@(a)", opts.clone(), false);
    assert_is_match!("c.c", "!(*.a|*.b|*.c)", opts.clone(), false);
    assert_is_match!("c.c", "*!(.a|.b|.c)", opts.clone(), true);
    assert_is_match!("c.c", "*.!(a|b|c)", opts.clone(), false);
    assert_is_match!("c.c", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), false);
    assert_is_match!("c.ccc", "!(*.[a-b]*)", opts.clone(), true);
    assert_is_match!("c.ccc", "!(*[a-b].[a-b]*)", opts.clone(), true);
    assert_is_match!("c.js", "!(*.js)", opts.clone(), false);
    assert_is_match!("c.js", "*!(.js)", opts.clone(), true);
    assert_is_match!("c.js", "*.!(js)", opts.clone(), false);
    assert_is_match!("c/a/v", "c/!(z)/v", opts.clone(), true);
    assert_is_match!("c/a/v", "c/*(z)/v", opts.clone(), false);
    assert_is_match!("c/a/v", "c/+(z)/v", opts.clone(), false);
    assert_is_match!("c/a/v", "c/@(z)/v", opts.clone(), false);
    assert_is_match!("c/z/v", "*(z)", opts.clone(), false);
    assert_is_match!("c/z/v", "+(z)", opts.clone(), false);
    assert_is_match!("c/z/v", "?(z)", opts.clone(), false);
    assert_is_match!("c/z/v", "c/!(z)/v", opts.clone(), false);
    assert_is_match!("c/z/v", "c/*(z)/v", opts.clone(), true);
    assert_is_match!("c/z/v", "c/+(z)/v", opts.clone(), true);
    assert_is_match!("c/z/v", "c/@(z)/v", opts.clone(), true);
    assert_is_match!("c/z/v", "c/z/v", opts.clone(), true);
    assert_is_match!("cc.a", "(b|a).(a)", opts.clone(), false);
    assert_is_match!("cc.a", "@(b|a).@(a)", opts.clone(), false);
    assert_is_match!("ccc", "!(a)*", opts.clone(), true);
    assert_is_match!("ccc", "a!(b)*", opts.clone(), false);
    assert_is_match!("cow", "!(*.*)", opts.clone(), true);
    assert_is_match!("cow", "!(*.*).", opts.clone(), false);
    assert_is_match!("cow", ".!(*.*)", opts.clone(), false);
    assert_is_match!("cz", "a!(*)", opts.clone(), false);
    assert_is_match!("cz", "a!(z)", opts.clone(), false);
    assert_is_match!("cz", "a*!(z)", opts.clone(), false);
    assert_is_match!("cz", "a*(z)", opts.clone(), false);
    assert_is_match!("cz", "a**(z)", opts.clone(), false);
    assert_is_match!("cz", "a*@(z)", opts.clone(), false);
    assert_is_match!("cz", "a+(z)", opts.clone(), false);
    assert_is_match!("cz", "a?(z)", opts.clone(), false);
    assert_is_match!("cz", "a@(z)", opts.clone(), false);
    assert_is_match!("d.a.d", "!(*.[a-b]*)", opts.clone(), false);
    assert_is_match!("d.a.d", "!(*[a-b].[a-b]*)", opts.clone(), true);
    assert_is_match!("d.a.d", "!*.(a|b)*", opts.clone(), false);
    assert_is_match!("d.a.d", "!*.*(a|b)", opts.clone(), true);
    assert_is_match!("d.a.d", "!*.{a,b}*", opts.clone(), false);
    assert_is_match!("d.a.d", "*.!(a)", opts.clone(), true);
    assert_is_match!("d.a.d", "*.+(b|d)", opts.clone(), true);
    assert_is_match!("d.d", "!(*.a|*.b|*.c)", opts.clone(), true);
    assert_is_match!("d.d", "*!(.a|.b|.c)", opts.clone(), true);
    assert_is_match!("d.d", "*.!(a|b|c)", opts.clone(), true);
    assert_is_match!("d.d", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), false);
    assert_is_match!("d.js.d", "!(*.js)", opts.clone(), true);
    assert_is_match!("d.js.d", "*!(.js)", opts.clone(), true);
    assert_is_match!("d.js.d", "*.!(js)", opts.clone(), true);
    assert_is_match!("dd.aa.d", "(b|a).(a)", opts.clone(), false);
    assert_is_match!("dd.aa.d", "@(b|a).@(a)", opts.clone(), false);
    assert_is_match!("def", "()ef", opts.clone(), false);
    assert_is_match!("e.e", "!(*.a|*.b|*.c)", opts.clone(), true);
    assert_is_match!("e.e", "*!(.a|.b|.c)", opts.clone(), true);
    assert_is_match!("e.e", "*.!(a|b|c)", opts.clone(), true);
    assert_is_match!("e.e", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), false);
    assert_is_match!("ef", "()ef", opts.clone(), true);
    assert_is_match!("effgz", "@(b+(c)d|e*(f)g?|?(h)i@(j|k))", opts.clone(), true);
    assert_is_match!("efgz", "@(b+(c)d|e*(f)g?|?(h)i@(j|k))", opts.clone(), true);
    assert_is_match!("egz", "@(b+(c)d|e*(f)g?|?(h)i@(j|k))", opts.clone(), true);
    assert_is_match!("egz", "@(b+(c)d|e+(f)g?|?(h)i@(j|k))", opts.clone(), false);
    assert_is_match!(
        "egzefffgzbcdij",
        "*(b+(c)d|e*(f)g?|?(h)i@(j|k))",
        opts.clone(),
        true
    );
    assert_is_match!("f", "!(f!(o))", opts.clone(), false);
    assert_is_match!("f", "!(f(o))", opts.clone(), true);
    assert_is_match!("f", "!(f)", opts.clone(), false);
    assert_is_match!("f", "*(!(f))", opts.clone(), false);
    assert_is_match!("f", "+(!(f))", opts.clone(), false);
    assert_is_match!("f.a", "!(*.a|*.b|*.c)", opts.clone(), false);
    assert_is_match!("f.a", "*!(.a|.b|.c)", opts.clone(), true);
    assert_is_match!("f.a", "*.!(a|b|c)", opts.clone(), false);
    assert_is_match!("f.f", "!(*.a|*.b|*.c)", opts.clone(), true);
    assert_is_match!("f.f", "*!(.a|.b|.c)", opts.clone(), true);
    assert_is_match!("f.f", "*.!(a|b|c)", opts.clone(), true);
    assert_is_match!("f.f", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), false);
    assert_is_match!("fa", "!(f!(o))", opts.clone(), false);
    assert_is_match!("fa", "!(f(o))", opts.clone(), true);
    assert_is_match!("fb", "!(f!(o))", opts.clone(), false);
    assert_is_match!("fb", "!(f(o))", opts.clone(), true);
    assert_is_match!("fff", "!(f)", opts.clone(), true);
    assert_is_match!("fff", "*(!(f))", opts.clone(), true);
    assert_is_match!("fff", "+(!(f))", opts.clone(), true);
    assert_is_match!(
        "fffooofoooooffoofffooofff",
        "*(*(f)*(o))",
        opts.clone(),
        true
    );
    assert_is_match!("ffo", "*(f*(o))", opts.clone(), true);
    assert_is_match!("file.C", "*.c?(c)", opts.clone(), false);
    assert_is_match!("file.c", "*.c?(c)", opts.clone(), true);
    assert_is_match!("file.cc", "*.c?(c)", opts.clone(), true);
    assert_is_match!("file.ccc", "*.c?(c)", opts.clone(), false);
    assert_is_match!("fo", "!(f!(o))", opts.clone(), true);
    assert_is_match!("fo", "!(f(o))", opts.clone(), false);
    assert_is_match!("fofo", "*(f*(o))", opts.clone(), true);
    assert_is_match!("fofoofoofofoo", "*(fo|foo)", opts.clone(), true);
    assert_is_match!("fofoofoofofoo", "*(fo|foo)", opts.clone(), true);
    assert_is_match!("foo", "!(!(foo))", opts.clone(), true);
    assert_is_match!("foo", "!(f)", opts.clone(), true);
    assert_is_match!("foo", "!(foo)", opts.clone(), false);
    assert_is_match!("foo", "!(foo)*", opts.clone(), false);
    assert_is_match!("foo", "!(foo)*", opts.clone(), false);
    assert_is_match!("foo", "!(foo)+", opts.clone(), false);
    assert_is_match!("foo", "!(foo)b*", opts.clone(), false);
    assert_is_match!("foo", "!(x)", opts.clone(), true);
    assert_is_match!("foo", "!(x)*", opts.clone(), true);
    assert_is_match!("foo", "*", opts.clone(), true);
    assert_is_match!("foo", "*(!(f))", opts.clone(), true);
    assert_is_match!("foo", "*(!(foo))", opts.clone(), false);
    assert_is_match!("foo", "*(@(a))a@(c)", opts.clone(), false);
    assert_is_match!("foo", "*(@(foo))", opts.clone(), true);
    assert_is_match!("foo", "*(a|b\\[)", opts.clone(), false);
    assert_is_match!("foo", "*(a|b\\[)|f*", opts.clone(), true);
    assert_is_match!("foo", "@(*(a|b\\[)|f*)", opts.clone(), true);
    assert_is_match!("foo", "*/*/*", opts.clone(), false);
    assert_is_match!("foo", "*f", opts.clone(), false);
    assert_is_match!("foo", "*foo*", opts.clone(), true);
    assert_is_match!("foo", "+(!(f))", opts.clone(), true);
    assert_is_match!("foo", "??", opts.clone(), false);
    assert_is_match!("foo", "???", opts.clone(), true);
    assert_is_match!("foo", "bar", opts.clone(), false);
    assert_is_match!("foo", "f*", opts.clone(), true);
    assert_is_match!("foo", "fo", opts.clone(), false);
    assert_is_match!("foo", "foo", opts.clone(), true);
    assert_is_match!("foo", "{*(a|b\\[),f*}", opts.clone(), true);
    assert_is_match!("foo*", "foo\\*", opts.clone(), true);
    assert_is_match!("foo*bar", "foo\\*bar", opts.clone(), true);
    assert_is_match!("foo.js", "!(foo).js", opts.clone(), false);
    assert_is_match!("foo.js.js", "*.!(js)", opts.clone(), true);
    assert_is_match!("foo.js.js", "*.!(js)*", opts.clone(), false);
    assert_is_match!("foo.js.js", "*.!(js)*.!(js)", opts.clone(), false);
    assert_is_match!("foo.js.js", "*.!(js)+", opts.clone(), false);
    assert_is_match!("foo.txt", "**/!(bar).txt", opts.clone(), true);
    assert_is_match!("foo/bar", "*/*/*", opts.clone(), false);
    assert_is_match!("foo/bar", "foo/!(foo)", opts.clone(), true);
    assert_is_match!("foo/bar", "foo/*", opts.clone(), true);
    assert_is_match!("foo/bar", "foo/bar", opts.clone(), true);
    assert_is_match!("foo/bar", "foo?bar", opts.clone(), false);
    assert_is_match!("foo/bar", "foo[/]bar", opts.clone(), true);
    assert_is_match!(
        "foo/bar/baz.jsx",
        "foo/bar/**/*.+(js|jsx)",
        opts.clone(),
        true
    );
    assert_is_match!("foo/bar/baz.jsx", "foo/bar/*.+(js|jsx)", opts.clone(), true);
    assert_is_match!("foo/bb/aa/rr", "**/**/**", opts.clone(), true);
    assert_is_match!("foo/bb/aa/rr", "*/*/*", opts.clone(), false);
    assert_is_match!("foo/bba/arr", "*/*/*", opts.clone(), true);
    assert_is_match!("foo/bba/arr", "foo*", opts.clone(), false);
    assert_is_match!("foo/bba/arr", "foo**", opts.clone(), false);
    assert_is_match!("foo/bba/arr", "foo/*", opts.clone(), false);
    assert_is_match!("foo/bba/arr", "foo/**", opts.clone(), true);
    assert_is_match!("foo/bba/arr", "foo/**arr", opts.clone(), false);
    assert_is_match!("foo/bba/arr", "foo/**z", opts.clone(), false);
    assert_is_match!("foo/bba/arr", "foo/*arr", opts.clone(), false);
    assert_is_match!("foo/bba/arr", "foo/*z", opts.clone(), false);
    assert_is_match!("foob", "!(foo)b*", opts.clone(), false);
    assert_is_match!("foob", "(foo)bb", opts.clone(), false);
    assert_is_match!("foobar", "!(foo)", opts.clone(), true);
    assert_is_match!("foobar", "!(foo)*", opts.clone(), false);
    assert_is_match!("foobar", "!(foo)b*", opts.clone(), false);
    assert_is_match!("foobar", "*(!(foo))", opts.clone(), true);
    assert_is_match!("foobar", "*ob*a*r*", opts.clone(), true);
    assert_is_match!("foobar", "foo\\*bar", opts.clone(), false);
    assert_is_match!("foobb", "!(foo)b*", opts.clone(), false);
    assert_is_match!("foobb", "(foo)bb", opts.clone(), true);
    assert_is_match!("(foo)bb", "\\(foo\\)bb", opts.clone(), true);
    assert_is_match!("foofoofo", "@(foo|f|fo)*(f|of+(o))", opts.clone(), true);
    assert_is_match!("foofoofo", "@(foo|f|fo)*(f|of+(o))", opts.clone(), true);
    assert_is_match!("fooofoofofooo", "*(f*(o))", opts.clone(), true);
    assert_is_match!("foooofo", "*(f*(o))", opts.clone(), true);
    assert_is_match!("foooofof", "*(f*(o))", opts.clone(), true);
    assert_is_match!("foooofof", "*(f+(o))", opts.clone(), false);
    assert_is_match!("foooofofx", "*(f*(o))", opts.clone(), false);
    assert_is_match!("foooxfooxfoxfooox", "*(f*(o)x)", opts.clone(), true);
    assert_is_match!("foooxfooxfxfooox", "*(f*(o)x)", opts.clone(), true);
    assert_is_match!("foooxfooxofoxfooox", "*(f*(o)x)", opts.clone(), false);
    assert_is_match!("foot", "@(!(z*)|*x)", opts.clone(), true);
    assert_is_match!("foox", "@(!(z*)|*x)", opts.clone(), true);
    assert_is_match!("fz", "*(z)", opts.clone(), false);
    assert_is_match!("fz", "+(z)", opts.clone(), false);
    assert_is_match!("fz", "?(z)", opts.clone(), false);
    assert_is_match!("moo.cow", "!(moo).!(cow)", opts.clone(), false);
    assert_is_match!("moo.cow", "!(*).!*", opts.clone(), false);
    assert_is_match!("mad.moo.cow", "!(*.*).!(*.*)", opts.clone(), false);
    assert_is_match!("mad.moo.cow", ".!(*.*)", opts.clone(), false);
    assert_is_match!(
        "Makefile",
        "!(*.c|*.h|Makefile.in|config*|README)",
        opts.clone(),
        true
    );
    assert_is_match!(
        "Makefile.in",
        "!(*.c|*.h|Makefile.in|config*|README)",
        opts.clone(),
        false
    );
    assert_is_match!("moo", "!(*.*)", opts.clone(), true);
    assert_is_match!("moo", "!(*.*).", opts.clone(), false);
    assert_is_match!("moo", ".!(*.*)", opts.clone(), false);
    assert_is_match!("moo.cow", "!(*.*)", opts.clone(), false);
    assert_is_match!("moo.cow", "!(*.*).", opts.clone(), false);
    assert_is_match!("moo.cow", ".!(*.*)", opts.clone(), false);
    assert_is_match!("mucca.pazza", "mu!(*(c))?.pa!(*(z))?", opts.clone(), false);
    assert_is_match!("ofoofo", "*(of+(o))", opts.clone(), true);
    assert_is_match!("ofoofo", "*(of+(o)|f)", opts.clone(), true);
    assert_is_match!("ofooofoofofooo", "*(f*(o))", opts.clone(), false);
    assert_is_match!("ofoooxoofxo", "*(*(of*(o)x)o)", opts.clone(), true);
    assert_is_match!(
        "ofoooxoofxoofoooxoofxo",
        "*(*(of*(o)x)o)",
        opts.clone(),
        true
    );
    assert_is_match!(
        "ofoooxoofxoofoooxoofxofo",
        "*(*(of*(o)x)o)",
        opts.clone(),
        false
    );
    assert_is_match!(
        "ofoooxoofxoofoooxoofxoo",
        "*(*(of*(o)x)o)",
        opts.clone(),
        true
    );
    assert_is_match!(
        "ofoooxoofxoofoooxoofxooofxofxo",
        "*(*(of*(o)x)o)",
        opts.clone(),
        true
    );
    assert_is_match!("ofxoofxo", "*(*(of*(o)x)o)", opts.clone(), true);
    assert_is_match!("oofooofo", "*(of|oof+(o))", opts.clone(), true);
    assert_is_match!("ooo", "!(f)", opts.clone(), true);
    assert_is_match!("ooo", "*(!(f))", opts.clone(), true);
    assert_is_match!("ooo", "+(!(f))", opts.clone(), true);
    assert_is_match!("oxfoxfox", "*(oxf+(ox))", opts.clone(), false);
    assert_is_match!("oxfoxoxfox", "*(oxf+(ox))", opts.clone(), true);
    assert_is_match!("para", "para*([0-9])", opts.clone(), true);
    assert_is_match!("para", "para+([0-9])", opts.clone(), false);
    assert_is_match!("para.38", "para!(*.[00-09])", opts.clone(), true);
    assert_is_match!("para.graph", "para!(*.[0-9])", opts.clone(), true);
    assert_is_match!("para13829383746592", "para*([0-9])", opts.clone(), true);
    assert_is_match!("para381", "para?([345]|99)1", opts.clone(), false);
    assert_is_match!("para39", "para!(*.[0-9])", opts.clone(), true);
    assert_is_match!("para987346523", "para+([0-9])", opts.clone(), true);
    assert_is_match!("para991", "para?([345]|99)1", opts.clone(), true);
    assert_is_match!("paragraph", "para!(*.[0-9])", opts.clone(), true);
    assert_is_match!("paragraph", "para*([0-9])", opts.clone(), false);
    assert_is_match!("paragraph", "para@(chute|graph)", opts.clone(), true);
    assert_is_match!("paramour", "para@(chute|graph)", opts.clone(), false);
    assert_is_match!(
        "parse.y",
        "!(*.c|*.h|Makefile.in|config*|README)",
        opts.clone(),
        true
    );
    assert_is_match!(
        "shell.c",
        "!(*.c|*.h|Makefile.in|config*|README)",
        opts.clone(),
        false
    );
    assert_is_match!("VMS.FILE;", "*\\;[1-9]*([0-9])", opts.clone(), false);
    assert_is_match!("VMS.FILE;0", "*\\;[1-9]*([0-9])", opts.clone(), false);
    assert_is_match!("VMS.FILE;1", "*\\;[1-9]*([0-9])", opts.clone(), true);
    assert_is_match!("VMS.FILE;1", "*;[1-9]*([0-9])", opts.clone(), true);
    assert_is_match!("VMS.FILE;139", "*\\;[1-9]*([0-9])", opts.clone(), true);
    assert_is_match!("VMS.FILE;1N", "*\\;[1-9]*([0-9])", opts.clone(), false);
    assert_is_match!("xfoooofof", "*(f*(o))", opts.clone(), false);
    // NOTE: This test uses windows: false (posix_opts)
    assert_is_match!(
        "XXX/adobe/courier/bold/o/normal//12/120/75/75/m/70/iso8859/1",
        "XXX/*/*/*/*/*/*/12/*/*/*/m/*/*/*",
        posix_opts(),
        true
    );
    // This test uses windows: true (windows_opts)
    assert_is_match!(
        "XXX/adobe/courier/bold/o/normal//12/120/75/75/X/70/iso8859/1",
        "XXX/*/*/*/*/*/*/12/*/*/*/m/*/*/*",
        windows_opts(),
        false
    );
    assert_is_match!("z", "*(z)", opts.clone(), true);
    assert_is_match!("z", "+(z)", opts.clone(), true);
    assert_is_match!("z", "?(z)", opts.clone(), true);
    assert_is_match!("zf", "*(z)", opts.clone(), false);
    assert_is_match!("zf", "+(z)", opts.clone(), false);
    assert_is_match!("zf", "?(z)", opts.clone(), false);
    assert_is_match!("zoot", "@(!(z*)|*x)", opts.clone(), false);
    assert_is_match!("zoox", "@(!(z*)|*x)", opts.clone(), true);
    assert_is_match!("zz", "(a+|b)*", opts.clone(), false);
}
