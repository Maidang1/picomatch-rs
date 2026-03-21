mod support;

use picomatch_rs::{parse, scan, CompileOptions, ScanOptions};

use support::{assert_is_match, assert_is_match_any, default_compile_options};

#[test]
fn validates_empty_patterns() {
    let err = picomatch_rs::is_match("foo", "", &default_compile_options()).unwrap_err();
    assert!(matches!(err, picomatch_rs::MatchError::EmptyPattern));
}

#[test]
fn validates_null_patterns() {
    // JS: isMatch('foo', null) throws with /Expected pattern to be a non-empty string/
    // Rust doesn't have null, but we test with empty string which is the equivalent error case
    let err = picomatch_rs::is_match("foo", "", &default_compile_options()).unwrap_err();
    assert!(matches!(err, picomatch_rs::MatchError::EmptyPattern));
}

#[test]
fn matches_multiple_patterns_any_match() {
    assert_is_match_any(".", &[".", "foo"], default_compile_options(), true);
    assert_is_match_any("a", &["a", "foo"], default_compile_options(), true);
    assert_is_match_any("ab", &["*", "foo", "bar"], default_compile_options(), true);
    assert_is_match_any("ab", &["*b", "foo", "bar"], default_compile_options(), true);
    assert_is_match_any(
        "ab",
        &["./*", "foo", "bar"],
        default_compile_options(),
        true,
    );
    assert_is_match_any("ab", &["a*", "foo", "bar"], default_compile_options(), true);
    assert_is_match_any("ab", &["ab", "foo"], default_compile_options(), true);
}

#[test]
fn matches_multiple_patterns_none_match() {
    assert_is_match_any("/ab", &["/a", "foo"], default_compile_options(), false);
    assert_is_match_any(
        "/ab",
        &["?/?", "foo", "bar"],
        default_compile_options(),
        false,
    );
    assert_is_match_any(
        "/ab",
        &["a/*", "foo", "bar"],
        default_compile_options(),
        false,
    );
    assert_is_match_any("a/b/c", &["a/b", "foo"], default_compile_options(), false);
    assert_is_match_any(
        "ab",
        &["*/*", "foo", "bar"],
        default_compile_options(),
        false,
    );
    assert_is_match_any(
        "ab",
        &["/a", "foo", "bar"],
        default_compile_options(),
        false,
    );
    assert_is_match_any("ab", &["a", "foo"], default_compile_options(), false);
    assert_is_match_any("ab", &["b", "foo"], default_compile_options(), false);
    assert_is_match_any("ab", &["c", "foo", "bar"], default_compile_options(), false);
    assert_is_match_any("abcd", &["ab", "foo"], default_compile_options(), false);
    assert_is_match_any("abcd", &["bc", "foo"], default_compile_options(), false);
    assert_is_match_any("abcd", &["c", "foo"], default_compile_options(), false);
    assert_is_match_any("abcd", &["cd", "foo"], default_compile_options(), false);
    assert_is_match_any("abcd", &["d", "foo"], default_compile_options(), false);
    assert_is_match_any(
        "abcd",
        &["f", "foo", "bar"],
        default_compile_options(),
        false,
    );
    assert_is_match_any(
        "ef",
        &["/*", "foo", "bar"],
        default_compile_options(),
        false,
    );
}

#[test]
fn matches_file_extensions() {
    assert_is_match(".c.md", "*.md", default_compile_options(), false);
    assert_is_match(".c.md", ".c.", default_compile_options(), false);
    assert_is_match(".c.md", ".md", default_compile_options(), false);
    assert_is_match(".md", "*.md", default_compile_options(), false);
    assert_is_match(".md", ".m", default_compile_options(), false);
    assert_is_match("a/b/c.md", "*.md", default_compile_options(), false);
    assert_is_match("a/b/c.md", ".md", default_compile_options(), false);
    assert_is_match("a/b/c.md", "a/*.md", default_compile_options(), false);
    assert_is_match("a/b/c/c.md", "*.md", default_compile_options(), false);
    assert_is_match("a/b/c/c.md", "c.js", default_compile_options(), false);
    assert_is_match(".c.md", ".*.md", default_compile_options(), true);
    assert_is_match(".md", ".md", default_compile_options(), true);
    assert_is_match("a/b/c.js", "a/**/*.*", default_compile_options(), true);
    assert_is_match("a/b/c.md", "**/*.md", default_compile_options(), true);
    assert_is_match("a/b/c.md", "a/*/*.md", default_compile_options(), true);
    assert_is_match("c.md", "*.md", default_compile_options(), true);
}

#[test]
fn matches_dot_files_without_dot_option() {
    assert_is_match(".a", "(a)*", default_compile_options(), false);
    assert_is_match(".a", "*(a|b)", default_compile_options(), false);
    assert_is_match(".a", "*.md", default_compile_options(), false);
    assert_is_match(".a", "*[a]", default_compile_options(), false);
    assert_is_match(".a", "*[a]*", default_compile_options(), false);
    assert_is_match(".a", "*a", default_compile_options(), false);
    assert_is_match(".a", "*a*", default_compile_options(), false);
    assert_is_match(".a.md", "a/b/c/*.md", default_compile_options(), false);
    assert_is_match(".ab", "*.*", default_compile_options(), false);
    assert_is_match(".abc", ".a", default_compile_options(), false);
    assert_is_match(".ba", ".a", default_compile_options(), false);
    assert_is_match(".c.md", "*.md", default_compile_options(), false);
    assert_is_match(".md", "a/b/c/*.md", default_compile_options(), false);
    assert_is_match(".txt", ".md", default_compile_options(), false);
    assert_is_match(".verb.txt", "*.md", default_compile_options(), false);
    assert_is_match("a/.c.md", "*.md", default_compile_options(), false);
    assert_is_match("a/b/d/.md", "a/b/c/*.md", default_compile_options(), false);

    assert_is_match(".a", ".a", default_compile_options(), true);
    assert_is_match(".ab", ".*", default_compile_options(), true);
    assert_is_match(".ab", ".a*", default_compile_options(), true);
    assert_is_match(".b", ".b*", default_compile_options(), true);
    assert_is_match(".md", ".md", default_compile_options(), true);
    assert_is_match("a/.c.md", "a/.c.md", default_compile_options(), true);
    assert_is_match(
        "a/b/c/.xyz.md",
        "a/b/c/.*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d.a.md",
        "a/b/c/*.md",
        default_compile_options(),
        true,
    );
}

#[test]
fn matches_dot_files_with_dot_option() {
    let options = CompileOptions {
        dot: true,
        ..CompileOptions::default()
    };

    assert_is_match("a/b/c/.xyz.md", ".*.md", options.clone(), false);
    assert_is_match(".c.md", "*.md", options.clone(), true);
    assert_is_match(".c.md", ".*", options.clone(), true);
    assert_is_match("a/b/c/.xyz.md", "**/*.md", options.clone(), true);
    assert_is_match("a/b/c/.xyz.md", "**/.*.md", options.clone(), true);
    assert_is_match("a/b/c/.xyz.md", "a/b/c/*.md", options.clone(), true);
    assert_is_match("a/b/c/.xyz.md", "a/b/c/.*.md", options.clone(), true);
}

#[test]
fn escapes_plus_signs() {
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
fn matches_non_globs() {
    assert_is_match(".", ".", default_compile_options(), true);
    assert_is_match("/a", "/a", default_compile_options(), true);
    assert_is_match("/ab", "/a", default_compile_options(), false);
    assert_is_match("a", "a", default_compile_options(), true);
    assert_is_match("ab", "/a", default_compile_options(), false);
    assert_is_match("ab", "a", default_compile_options(), false);
    assert_is_match("ab", "ab", default_compile_options(), true);
    assert_is_match("abcd", "cd", default_compile_options(), false);
    assert_is_match("abcd", "bc", default_compile_options(), false);
    assert_is_match("abcd", "ab", default_compile_options(), false);
}

#[test]
fn matches_file_names() {
    assert_is_match("a.b", "a.b", default_compile_options(), true);
    assert_is_match("a.b", "*.b", default_compile_options(), true);
    assert_is_match("a.b", "a.*", default_compile_options(), true);
    assert_is_match("a.b", "*.*", default_compile_options(), true);
    assert_is_match("a-b.c-d", "a*.c*", default_compile_options(), true);
    assert_is_match("a-b.c-d", "*b.*d", default_compile_options(), true);
    assert_is_match("a-b.c-d", "*.*", default_compile_options(), true);
    assert_is_match("a-b.c-d", "*.*-*", default_compile_options(), true);
    assert_is_match("a-b.c-d", "*-*.*-*", default_compile_options(), true);
    assert_is_match("a-b.c-d", "*.c-*", default_compile_options(), true);
    assert_is_match("a-b.c-d", "*.*-d", default_compile_options(), true);
    assert_is_match("a-b.c-d", "a-*.*-d", default_compile_options(), true);
    assert_is_match("a-b.c-d", "*-b.c-*", default_compile_options(), true);
    assert_is_match("a-b.c-d", "*-b*c-*", default_compile_options(), true);
    assert_is_match("a-b.c-d", "*-bc-*", default_compile_options(), false);
}

#[test]
fn matches_common_glob_patterns() {
    assert_is_match("/ab", "./*/", default_compile_options(), false);
    assert_is_match("/ef", "*", default_compile_options(), false);
    assert_is_match("ab", "./*/", default_compile_options(), false);
    assert_is_match("ef", "/*", default_compile_options(), false);
    assert_is_match("/ab", "/*", default_compile_options(), true);
    assert_is_match("/cd", "/*", default_compile_options(), true);
    assert_is_match("ab", "*", default_compile_options(), true);
    assert_is_match("ab", "./*", default_compile_options(), true);
    assert_is_match("ab", "ab", default_compile_options(), true);
    assert_is_match("ab/", "./*/", default_compile_options(), true);
}

#[test]
fn matches_wildcards() {
    assert_is_match("a/b/c/z.js", "*.js", default_compile_options(), false);
    assert_is_match("a/b/z.js", "*.js", default_compile_options(), false);
    assert_is_match("a/z.js", "*.js", default_compile_options(), false);
    assert_is_match("z.js", "*.js", default_compile_options(), true);

    assert_is_match("z.js", "z*.js", default_compile_options(), true);
    assert_is_match("a/z.js", "a/z*.js", default_compile_options(), true);
    assert_is_match("a/z.js", "*/z*.js", default_compile_options(), true);
}

#[test]
fn matches_globstars() {
    assert_is_match("a/b/c/z.js", "**/*.js", default_compile_options(), true);
    assert_is_match("a/b/z.js", "**/*.js", default_compile_options(), true);
    assert_is_match("a/z.js", "**/*.js", default_compile_options(), true);
    assert_is_match(
        "a/b/c/d/e/z.js",
        "a/b/**/*.js",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/z.js",
        "a/b/**/*.js",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/z.js",
        "a/b/c/**/*.js",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/z.js",
        "a/b/c**/*.js",
        default_compile_options(),
        true,
    );
    assert_is_match("a/b/c/z.js", "a/b/**/*.js", default_compile_options(), true);
    assert_is_match("a/b/z.js", "a/b/**/*.js", default_compile_options(), true);

    assert_is_match("a/z.js", "a/b/**/*.js", default_compile_options(), false);
    assert_is_match("z.js", "a/b/**/*.js", default_compile_options(), false);

    assert_is_match("z.js", "z*", default_compile_options(), true);
    assert_is_match("z.js", "**/z*", default_compile_options(), true);
    assert_is_match("z.js", "**/z*.js", default_compile_options(), true);
    assert_is_match("z.js", "**/*.js", default_compile_options(), true);
    assert_is_match("foo", "**/foo", default_compile_options(), true);
}

#[test]
fn issue_23() {
    assert_is_match("zzjs", "z*.js", default_compile_options(), false);
    assert_is_match("zzjs", "*z.js", default_compile_options(), false);
}

#[test]
fn issue_24_matches_zero_or_more_directories() {
    assert_is_match("a/b/c/d/", "a/b/**/f", default_compile_options(), false);
    assert_is_match("a", "a/**", default_compile_options(), true);
    assert_is_match("a", "**", default_compile_options(), true);
    assert_is_match("a/", "**", default_compile_options(), true);
    assert_is_match(
        "a/b-c/d/e/z.js",
        "a/b-*/**/z.js",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b-c/z.js",
        "a/b-*/**/z.js",
        default_compile_options(),
        true,
    );
    assert_is_match("a/b/c/d", "**", default_compile_options(), true);
    assert_is_match("a/b/c/d/", "**", default_compile_options(), true);
    assert_is_match("a/b/c/d/", "**/**", default_compile_options(), true);
    assert_is_match("a/b/c/d/", "**/b/**", default_compile_options(), true);
    assert_is_match("a/b/c/d/", "a/b/**", default_compile_options(), true);
    assert_is_match("a/b/c/d/", "a/b/**/", default_compile_options(), true);
    assert_is_match("a/b/c/d/", "a/b/**/c/**/", default_compile_options(), true);
    assert_is_match(
        "a/b/c/d/",
        "a/b/**/c/**/d/",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/e.f",
        "a/b/**/**/*.*",
        default_compile_options(),
        true,
    );
    assert_is_match("a/b/c/d/e.f", "a/b/**/*.*", default_compile_options(), true);
    assert_is_match(
        "a/b/c/d/e.f",
        "a/b/**/c/**/d/*.*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/e.f",
        "a/b/**/d/**/*.*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/g/e.f",
        "a/b/**/d/**/*.*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/g/g/e.f",
        "a/b/**/d/**/*.*",
        default_compile_options(),
        true,
    );
}

#[test]
fn matches_slashes() {
    assert_is_match("bar/baz/foo", "*/foo", default_compile_options(), false);
    assert_is_match("deep/foo/bar", "**/bar/*", default_compile_options(), false);
    assert_is_match(
        "deep/foo/bar/baz/x",
        "*/bar/**",
        default_compile_options(),
        false,
    );
    assert_is_match("foo/bar", "foo?bar", default_compile_options(), false);
    assert_is_match("foo/bar/baz", "**/bar*", default_compile_options(), false);
    assert_is_match("foo/bar/baz", "**/bar**", default_compile_options(), false);
    assert_is_match("foo/baz/bar", "foo**bar", default_compile_options(), false);
    assert_is_match("foo/baz/bar", "foo*bar", default_compile_options(), false);
    assert_is_match(
        "deep/foo/bar/baz",
        "**/bar/*/",
        default_compile_options(),
        false,
    );

    let strict_options = CompileOptions {
        strict_slashes: true,
        ..CompileOptions::default()
    };
    assert_is_match("deep/foo/bar/baz/", "**/bar/*", strict_options, false);

    assert_is_match(
        "deep/foo/bar/baz/",
        "**/bar/*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "deep/foo/bar/baz",
        "**/bar/*",
        default_compile_options(),
        true,
    );
    assert_is_match("foo", "foo/**", default_compile_options(), true);
    assert_is_match(
        "deep/foo/bar/baz/",
        "**/bar/*{,/}",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/j/c/z/x.md",
        "a/**/j/**/z/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/j/z/x.md",
        "a/**/j/**/z/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match("bar/baz/foo", "**/foo", default_compile_options(), true);
    assert_is_match(
        "deep/foo/bar/",
        "**/bar/**",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "deep/foo/bar/baz",
        "**/bar/*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "deep/foo/bar/baz/",
        "**/bar/*/",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "deep/foo/bar/baz/",
        "**/bar/**",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "deep/foo/bar/baz/x",
        "**/bar/*/*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "foo/b/a/z/bar",
        "foo/**/**/bar",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "foo/b/a/z/bar",
        "foo/**/bar",
        default_compile_options(),
        true,
    );
    assert_is_match("foo/bar", "foo/**/**/bar", default_compile_options(), true);
    assert_is_match("foo/bar", "foo/**/bar", default_compile_options(), true);
    assert_is_match("foo/bar", "foo[/]bar", default_compile_options(), true);
    assert_is_match("foo/bar/baz/x", "*/bar/**", default_compile_options(), true);
    assert_is_match(
        "foo/baz/bar",
        "foo/**/**/bar",
        default_compile_options(),
        true,
    );
    assert_is_match("foo/baz/bar", "foo/**/bar", default_compile_options(), true);
    assert_is_match("foobazbar", "foo**bar", default_compile_options(), true);
    assert_is_match("XXX/foo", "**/foo", default_compile_options(), true);

    assert_is_match(
        "foo//baz.md",
        "foo//baz.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "foo//baz.md",
        "foo//*baz.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "foo//baz.md",
        "foo{/,//}baz.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "foo/baz.md",
        "foo{/,//}baz.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "foo//baz.md",
        "foo/+baz.md",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo//baz.md",
        "foo//+baz.md",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo//baz.md",
        "foo/baz.md",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "foo/baz.md",
        "foo//baz.md",
        default_compile_options(),
        false,
    );
}

#[test]
fn question_marks_should_not_match_slashes() {
    assert_is_match("aaa/bbb", "aaa?bbb", default_compile_options(), false);
}

#[test]
fn should_not_match_dotfiles_without_dot_option() {
    assert_is_match(".c.md", "*.md", default_compile_options(), false);
    assert_is_match("a/.c.md", "*.md", default_compile_options(), false);
    assert_is_match("a/.c.md", "a/.c.md", default_compile_options(), true);
    assert_is_match(".a", "*.md", default_compile_options(), false);
    assert_is_match(".verb.txt", "*.md", default_compile_options(), false);
    assert_is_match(
        "a/b/c/.xyz.md",
        "a/b/c/.*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(".md", ".md", default_compile_options(), true);
    assert_is_match(".txt", ".md", default_compile_options(), false);
    assert_is_match(".md", ".md", default_compile_options(), true);
    assert_is_match(".a", ".a", default_compile_options(), true);
    assert_is_match(".b", ".b*", default_compile_options(), true);
    assert_is_match(".ab", ".a*", default_compile_options(), true);
    assert_is_match(".ab", ".*", default_compile_options(), true);
    assert_is_match(".ab", "*.*", default_compile_options(), false);
    assert_is_match(".md", "a/b/c/*.md", default_compile_options(), false);
    assert_is_match(".a.md", "a/b/c/*.md", default_compile_options(), false);
    assert_is_match(
        "a/b/c/d.a.md",
        "a/b/c/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match("a/b/d/.md", "a/b/c/*.md", default_compile_options(), false);
}

#[test]
fn should_match_dotfiles_with_dot_option() {
    let options = CompileOptions {
        dot: true,
        ..CompileOptions::default()
    };

    assert_is_match(".c.md", "*.md", options.clone(), true);
    assert_is_match(".c.md", ".*", options.clone(), true);
    assert_is_match("a/b/c/.xyz.md", "a/b/c/*.md", options.clone(), true);
    assert_is_match("a/b/c/.xyz.md", "a/b/c/.*.md", options.clone(), true);
}

#[test]
fn test_parse_tokens_for_fastpath_pattern() {
    let state = parse("a*.txt", &CompileOptions::default()).expect("parse state");
    let tokens = state.tokens.expect("parse tokens");
    let actual = tokens
        .iter()
        .map(|token| (token.kind.as_str(), token.value.as_str()))
        .collect::<Vec<_>>();

    assert_eq!(
        actual,
        vec![("bos", ""), ("text", "a"), ("star", "*"), ("text", ".txt"),]
    );
}

#[test]
fn test_parse_tokens_for_brace_pattern() {
    let state = parse("{a,b}*", &CompileOptions::default()).expect("parse state");
    let tokens = state.tokens.expect("parse tokens");
    let actual = tokens
        .iter()
        .map(|token| (token.kind.as_str(), token.value.as_str()))
        .collect::<Vec<_>>();

    assert_eq!(
        actual,
        vec![
            ("bos", ""),
            ("brace", "{"),
            ("text", "a"),
            ("comma", ","),
            ("text", "b"),
            ("brace", "}"),
            ("star", "*"),
            ("maybe_slash", ""),
        ]
    );
}

#[test]
fn test_parse_tokens_with_output_field() {
    let state = parse("foo.(m|c|)js", &CompileOptions::default()).expect("parse state");
    let tokens = state.tokens.expect("parse tokens");
    let actual = tokens
        .iter()
        .map(|token| {
            (
                token.kind.as_str(),
                TokenSnapshot {
                    output: token.output.as_deref(),
                    value: token.value.as_str(),
                },
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        actual,
        vec![
            (
                "bos",
                TokenSnapshot {
                    output: Some(""),
                    value: "",
                },
            ),
            (
                "text",
                TokenSnapshot {
                    output: Some("foo."),
                    value: "foo.",
                },
            ),
            (
                "paren",
                TokenSnapshot {
                    output: None,
                    value: "(",
                },
            ),
            (
                "text",
                TokenSnapshot {
                    output: Some("m|c|"),
                    value: "m|c|",
                },
            ),
            (
                "paren",
                TokenSnapshot {
                    output: Some(")"),
                    value: ")",
                },
            ),
            (
                "text",
                TokenSnapshot {
                    output: None,
                    value: "js",
                },
            ),
        ]
    );
}

#[test]
fn test_negated_extglob() {
    let state = scan("!(abc)", &ScanOptions::default());
    assert!(state.negated_extglob);

    let state = scan("!(abc)**", &ScanOptions::default());
    assert!(state.negated_extglob);

    let state = scan("!(abc)/**", &ScanOptions::default());
    assert!(state.negated_extglob);

    let state = scan("(!(abc))", &ScanOptions::default());
    assert!(!state.negated_extglob);

    let state = scan("**!(abc)", &ScanOptions::default());
    assert!(!state.negated_extglob);
}

#[derive(Debug, PartialEq, Eq)]
struct TokenSnapshot<'a> {
    output: Option<&'a str>,
    value: &'a str,
}
