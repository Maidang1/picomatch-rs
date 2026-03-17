mod support;

use picomatch_rs::CompileOptions;
use support::assert_is_match;

fn nocase_opts() -> CompileOptions {
    CompileOptions {
        nocase: true,
        ..Default::default()
    }
}

fn win_opts() -> CompileOptions {
    CompileOptions {
        windows: true,
        ..Default::default()
    }
}

// In JS, format removes the leading "./" from the input string.
// We apply this transformation manually in the tests since Rust does not
// support passing custom JS functions like `format` in CompileOptions yet.
fn format(s: &str) -> &str {
    s.strip_prefix("./").unwrap_or(s)
}

#[test]
fn test_issue_29() {
    assert_is_match("foo/bar.txt", "foo/**/*.txt", Default::default(), true);
    assert_is_match("n/axios/a.js", "n/!(axios)/**", Default::default(), false);
}

#[test]
fn test_issue_30() {
    assert_is_match(format("foo/bar.js"), "**/foo/**", Default::default(), true);
    assert_is_match(
        format("./foo/bar.js"),
        "./**/foo/**",
        Default::default(),
        true,
    );
    assert_is_match(
        format("./foo/bar.js"),
        "**/foo/**",
        Default::default(),
        true,
    );
    assert_is_match(
        format("./foo/bar.txt"),
        "foo/**/*.txt",
        Default::default(),
        true,
    );

    // assert(makeRe('./foo/**/*.txt').test('foo/bar.txt'));
    assert_is_match("foo/bar.txt", "./foo/**/*.txt", Default::default(), true);

    // assert(!makeRe('./foo/!(bar)/**').test('foo/bar/a.js'));
    assert_is_match("foo/bar/a.js", "./foo/!(bar)/**", Default::default(), false);

    assert_is_match(
        format("foo/bar/a.js"),
        "./foo/!(bar)/**",
        Default::default(),
        false,
    );
}

#[test]
fn test_issue_50() {
    assert_is_match(
        "foo/bar-[ABC].txt",
        "foo/**/*-\\[ABC\\].txt",
        Default::default(),
        true,
    );
    assert_is_match(
        "foo/bar-[ABC].txt",
        "foo/**/*-\\[abc\\].txt",
        Default::default(),
        false,
    );
    assert_is_match(
        "foo/bar-[ABC].txt",
        "foo/**/*-\\[abc\\].txt",
        nocase_opts(),
        true,
    );
}

#[test]
fn test_issue_67() {
    assert_is_match(
        "node_modules/foobar/foo.bar",
        "node_modules/foobar/**/*.bar",
        Default::default(),
        true,
    );
}

#[test]
fn test_issue_75() {
    assert_is_match(
        "foo/baz.qux.js",
        "foo/@(baz.qux).js",
        Default::default(),
        true,
    );
    assert_is_match(
        "foo/baz.qux.js",
        "foo/+(baz.qux).js",
        Default::default(),
        true,
    );
    assert_is_match(
        "foo/baz.qux.js",
        "foo/*(baz.qux).js",
        Default::default(),
        true,
    );
    assert_is_match(
        "foo/baz.qux.js",
        "foo/!(baz.qux).js",
        Default::default(),
        false,
    );
    assert_is_match(
        "foo/bar/baz.qux.js",
        "foo/*/!(baz.qux).js",
        Default::default(),
        false,
    );
    assert_is_match(
        "foo/bar/bazqux.js",
        "**/!(bazqux).js",
        Default::default(),
        false,
    );
    assert_is_match(
        "foo/bar/bazqux.js",
        "**/bar/!(bazqux).js",
        Default::default(),
        false,
    );
    assert_is_match(
        "foo/bar/bazqux.js",
        "foo/**/!(bazqux).js",
        Default::default(),
        false,
    );
    assert_is_match(
        "foo/bar/bazqux.js",
        "foo/**/!(bazqux)*.js",
        Default::default(),
        false,
    );
    assert_is_match(
        "foo/bar/baz.qux.js",
        "foo/**/!(baz.qux)*.js",
        Default::default(),
        false,
    );
    assert_is_match(
        "foo/bar/baz.qux.js",
        "foo/**/!(baz.qux).js",
        Default::default(),
        false,
    );
    assert_is_match("foobar.js", "!(foo)*.js", Default::default(), false);
    assert_is_match("foo.js", "!(foo).js", Default::default(), false);
    assert_is_match("foo.js", "!(foo)*.js", Default::default(), false);
}

#[test]
fn test_issue_78() {
    assert_is_match("a\\b\\c.txt", "a/**/*.txt", win_opts(), true);
    assert_is_match("a/b/c.txt", "a/**/*.txt", win_opts(), true);
}

#[test]
fn test_issue_82() {
    assert_is_match(
        format("./src/test/a.js"),
        "**/test/**",
        Default::default(),
        true,
    );
    assert_is_match("src/test/a.js", "**/test/**", Default::default(), true);
}

#[test]
fn test_issue_83() {
    assert_is_match("foo/bar/a.js", "foo/!(bar)/**", Default::default(), false);
}
