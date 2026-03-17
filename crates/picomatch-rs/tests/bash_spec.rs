mod support;

use picomatch_rs::CompileOptions;

use support::assert_is_match;

fn bash_options() -> CompileOptions {
    CompileOptions {
        bash: true,
        ..CompileOptions::default()
    }
}

#[test]
fn dotglob_tests() {
    let opts = bash_options();

    assert_is_match("a/b/.x", "**/.x/**", opts.clone(), true);
    assert_is_match(".x", "**/.x/**", opts.clone(), true);
    assert_is_match(".x/", "**/.x/**", opts.clone(), true);
    assert_is_match(".x/a", "**/.x/**", opts.clone(), true);
    assert_is_match(".x/a/b", "**/.x/**", opts.clone(), true);
    assert_is_match(".x/.x", "**/.x/**", opts.clone(), true);
    assert_is_match("a/.x", "**/.x/**", opts.clone(), true);
    assert_is_match("a/b/.x/c", "**/.x/**", opts.clone(), true);
    assert_is_match("a/b/.x/c/d", "**/.x/**", opts.clone(), true);
    assert_is_match("a/b/.x/c/d/e", "**/.x/**", opts.clone(), true);
    assert_is_match("a/b/.x/", "**/.x/**", opts.clone(), true);
    assert_is_match("a/.x/b", "**/.x/**", opts.clone(), true);
    assert_is_match("a/.x/b/.x/c", "**/.x/**", opts.clone(), false);

    assert_is_match(".bashrc", "?bashrc", opts.clone(), false);

    assert_is_match(".bar.baz/", ".*.*", opts.clone(), true);
    assert_is_match(".bar.baz/", ".*.*/", opts.clone(), true);
    assert_is_match(".bar.baz", ".*.*", opts.clone(), true);
}

#[test]
fn glob_tests() {
    let opts = bash_options();

    assert_is_match("a/b/.x", "**/.x/**", opts.clone(), true);
    assert_is_match(".x", "**/.x/**", opts.clone(), true);
    assert_is_match(".x/", "**/.x/**", opts.clone(), true);
    assert_is_match(".x/a", "**/.x/**", opts.clone(), true);
    assert_is_match(".x/a/b", "**/.x/**", opts.clone(), true);
    assert_is_match(".x/.x", "**/.x/**", opts.clone(), true);
    assert_is_match("a/.x", "**/.x/**", opts.clone(), true);
    assert_is_match("a/b/.x/c", "**/.x/**", opts.clone(), true);
    assert_is_match("a/b/.x/c/d", "**/.x/**", opts.clone(), true);
    assert_is_match("a/b/.x/c/d/e", "**/.x/**", opts.clone(), true);
    assert_is_match("a/b/.x/", "**/.x/**", opts.clone(), true);
    assert_is_match("a/.x/b", "**/.x/**", opts.clone(), true);
    assert_is_match("a/.x/b/.x/c", "**/.x/**", opts.clone(), false);

    assert_is_match("a/c/b", "a/*/b", opts.clone(), true);
    assert_is_match("a/.d/b", "a/*/b", opts.clone(), false);
    assert_is_match("a/./b", "a/*/b", opts.clone(), false);
    assert_is_match("a/../b", "a/*/b", opts.clone(), false);

    assert_is_match("ab", "ab**", opts.clone(), true);
    assert_is_match("abcdef", "ab**", opts.clone(), true);
    assert_is_match("abef", "ab**", opts.clone(), true);
    assert_is_match("abcfef", "ab**", opts.clone(), true);

    assert_is_match("ab", "ab***ef", opts.clone(), false);
    assert_is_match("abcdef", "ab***ef", opts.clone(), true);
    assert_is_match("abef", "ab***ef", opts.clone(), true);
    assert_is_match("abcfef", "ab***ef", opts.clone(), true);

    assert_is_match(".bashrc", "?bashrc", opts.clone(), false);

    assert_is_match("abbc", "ab?bc", opts.clone(), false);
    assert_is_match("abc", "ab?bc", opts.clone(), false);

    assert_is_match("a.a", "[a-d]*.[a-b]", opts.clone(), true);
    assert_is_match("a.b", "[a-d]*.[a-b]", opts.clone(), true);
    assert_is_match("c.a", "[a-d]*.[a-b]", opts.clone(), true);
    assert_is_match("a.a.a", "[a-d]*.[a-b]", opts.clone(), true);
    assert_is_match("a.a.a", "[a-d]*.[a-b]*.[a-b]", opts.clone(), true);

    assert_is_match("a.a", "*.[a-b]", opts.clone(), true);
    assert_is_match("a.b", "*.[a-b]", opts.clone(), true);
    assert_is_match("a.a.a", "*.[a-b]", opts.clone(), true);
    assert_is_match("c.a", "*.[a-b]", opts.clone(), true);
    assert_is_match("d.a.d", "*.[a-b]", opts.clone(), false);
    assert_is_match("a.bb", "*.[a-b]", opts.clone(), false);
    assert_is_match("a.ccc", "*.[a-b]", opts.clone(), false);
    assert_is_match("c.ccc", "*.[a-b]", opts.clone(), false);

    assert_is_match("a.a", "*.[a-b]*", opts.clone(), true);
    assert_is_match("a.b", "*.[a-b]*", opts.clone(), true);
    assert_is_match("a.a.a", "*.[a-b]*", opts.clone(), true);
    assert_is_match("c.a", "*.[a-b]*", opts.clone(), true);
    assert_is_match("d.a.d", "*.[a-b]*", opts.clone(), true);
    assert_is_match("d.a.d", "*.[a-b]*.[a-b]*", opts.clone(), false);
    assert_is_match("d.a.d", "*.[a-d]*.[a-d]*", opts.clone(), true);
    assert_is_match("a.bb", "*.[a-b]*", opts.clone(), true);
    assert_is_match("a.ccc", "*.[a-b]*", opts.clone(), false);
    assert_is_match("c.ccc", "*.[a-b]*", opts.clone(), false);

    assert_is_match("a.a", "*[a-b].[a-b]*", opts.clone(), true);
    assert_is_match("a.b", "*[a-b].[a-b]*", opts.clone(), true);
    assert_is_match("a.a.a", "*[a-b].[a-b]*", opts.clone(), true);
    assert_is_match("c.a", "*[a-b].[a-b]*", opts.clone(), false);
    assert_is_match("d.a.d", "*[a-b].[a-b]*", opts.clone(), false);
    assert_is_match("a.bb", "*[a-b].[a-b]*", opts.clone(), true);
    assert_is_match("a.ccc", "*[a-b].[a-b]*", opts.clone(), false);
    assert_is_match("c.ccc", "*[a-b].[a-b]*", opts.clone(), false);

    assert_is_match("abd", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("abe", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("bb", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("bcd", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("ca", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("cb", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("dd", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("de", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("bdir/", "[a-y]*[^c]", opts.clone(), true);

    assert_is_match("abd", "**/*", opts.clone(), true);
}

#[test]
fn globstar_tests() {
    let opts = bash_options();

    assert_is_match("a.js", "**/*.js", opts.clone(), true);
    assert_is_match("a/a.js", "**/*.js", opts.clone(), true);
    assert_is_match("a/a/b.js", "**/*.js", opts.clone(), true);

    assert_is_match("a/b/z.js", "a/b/**/*.js", opts.clone(), true);
    assert_is_match("a/b/c/z.js", "a/b/**/*.js", opts.clone(), true);

    assert_is_match("foo.md", "**/*.md", opts.clone(), true);
    assert_is_match("foo/bar.md", "**/*.md", opts.clone(), true);

    assert_is_match("foo/bar", "foo/**/bar", opts.clone(), true);
    assert_is_match("foo/bar", "foo/**bar", opts.clone(), true);

    assert_is_match("ab/a/d", "**/*", opts.clone(), true);
    assert_is_match("ab/b", "**/*", opts.clone(), true);
    assert_is_match("a/b/c/d/a.js", "**/*", opts.clone(), true);
    assert_is_match("a/b/c.js", "**/*", opts.clone(), true);
    assert_is_match("a/b/c.txt", "**/*", opts.clone(), true);
    assert_is_match("a/b/.js/c.txt", "**/*", opts.clone(), true);
    assert_is_match("a.js", "**/*", opts.clone(), true);
    assert_is_match("za.js", "**/*", opts.clone(), true);
    assert_is_match("ab", "**/*", opts.clone(), true);
    assert_is_match("a.b", "**/*", opts.clone(), true);

    assert_is_match("foo/", "foo/**/", opts.clone(), true);
    assert_is_match("foo/bar", "foo/**/", opts.clone(), false);
    assert_is_match("foo/bazbar", "foo/**/", opts.clone(), false);
    assert_is_match("foo/barbar", "foo/**/", opts.clone(), false);
    assert_is_match("foo/bar/baz/qux", "foo/**/", opts.clone(), false);
    assert_is_match("foo/bar/baz/qux/", "foo/**/", opts.clone(), true);
}
