mod support;
use picomatch_rs::CompileOptions;
use support::assert_is_match;

fn dot_opts() -> CompileOptions {
    CompileOptions {
        dot: true,
        ..Default::default()
    }
}

#[test]
fn test_issue_8() {
    // should match with braces (see picomatch/issues#8)
    assert_is_match(
        "directory/.test.txt",
        "{file.txt,directory/**/*}",
        dot_opts(),
        true,
    );
    assert_is_match(
        "directory/test.txt",
        "{file.txt,directory/**/*}",
        dot_opts(),
        true,
    );
    assert_is_match(
        "directory/.test.txt",
        "{file.txt,directory/**/*}",
        Default::default(),
        false,
    );
    assert_is_match(
        "directory/test.txt",
        "{file.txt,directory/**/*}",
        Default::default(),
        true,
    );
}

#[test]
fn test_issue_127_japanese() {
    // should match Japanese characters (see micromatch/issues#127)
    assert_is_match("フォルダ/aaa.js", "フ*/**/*", Default::default(), true);
    assert_is_match("フォルダ/aaa.js", "フォ*/**/*", Default::default(), true);
    assert_is_match("フォルダ/aaa.js", "フォル*/**/*", Default::default(), true);
    assert_is_match("フォルダ/aaa.js", "フ*ル*/**/*", Default::default(), true);
    assert_is_match("フォルダ/aaa.js", "フォルダ/**/*", Default::default(), true);
}

#[test]
fn test_issue_15() {
    // micromatch issue#15
    assert_is_match("a/b-c/d/e/z.js", "a/b-*/**/z.js", Default::default(), true);
    assert_is_match("z.js", "z*", Default::default(), true);
    assert_is_match("z.js", "**/z*", Default::default(), true);
    assert_is_match("z.js", "**/z*.js", Default::default(), true);
    assert_is_match("z.js", "**/*.js", Default::default(), true);
    assert_is_match("foo", "**/foo", Default::default(), true);
}

#[test]
fn test_issue_23() {
    // micromatch issue#23
    assert_is_match("zzjs", "z*.js", Default::default(), false);
    assert_is_match("zzjs", "*z.js", Default::default(), false);
}

#[test]
fn test_issue_24() {
    // micromatch issue#24
    assert_is_match("a/b/c/d/", "a/b/**/f", Default::default(), false);
    assert_is_match("a", "a/**", Default::default(), true);
    assert_is_match("a", "**", Default::default(), true);
    assert_is_match("a/", "**", Default::default(), true);
    assert_is_match("a/b/c/d", "**", Default::default(), true);
    assert_is_match("a/b/c/d/", "**", Default::default(), true);
    assert_is_match("a/b/c/d/", "**/**", Default::default(), true);
    assert_is_match("a/b/c/d/", "**/b/**", Default::default(), true);
    assert_is_match("a/b/c/d/", "a/b/**", Default::default(), true);
    assert_is_match("a/b/c/d/", "a/b/**/", Default::default(), true);
    assert_is_match("a/b/c/d/e.f", "a/b/**/**/*.*", Default::default(), true);
    assert_is_match("a/b/c/d/e.f", "a/b/**/*.*", Default::default(), true);
    assert_is_match("a/b/c/d/g/e.f", "a/b/**/d/**/*.*", Default::default(), true);
    assert_is_match(
        "a/b/c/d/g/g/e.f",
        "a/b/**/d/**/*.*",
        Default::default(),
        true,
    );
}

#[test]
fn test_issue_58() {
    // micromatch issue#58 - only match nested dirs when `**` is the only thing in a segment
    assert_is_match("a/b/c", "a/b**", Default::default(), false);
    assert_is_match("a/c/b", "a/**b", Default::default(), false);
}

#[test]
fn test_issue_79() {
    // micromatch issue#79
    assert_is_match("a/foo.js", "**/foo.js", Default::default(), true);
    assert_is_match("foo.js", "**/foo.js", Default::default(), true);
    assert_is_match("a/foo.js", "**/foo.js", dot_opts(), true);
    assert_is_match("foo.js", "**/foo.js", dot_opts(), true);
}
