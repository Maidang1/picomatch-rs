use picomatch_rs::{scan, ScanOptions, ScanState};

#[derive(Clone, Copy)]
struct ExpectedScan<'a> {
    input: &'a str,
    prefix: &'a str,
    start: usize,
    base: &'a str,
    glob: &'a str,
    is_brace: bool,
    is_bracket: bool,
    is_glob: bool,
    is_globstar: bool,
    is_extglob: bool,
    negated: bool,
    negated_extglob: bool,
}

fn assert_scan(pattern: &str, options: ScanOptions, expected: ExpectedScan<'_>) {
    let state = scan(pattern, &options);
    assert_scan_state(&state, expected);
}

fn assert_scan_state(actual: &ScanState, expected: ExpectedScan<'_>) {
    assert_eq!(actual.input, expected.input);
    assert_eq!(actual.prefix, expected.prefix);
    assert_eq!(actual.start, expected.start);
    assert_eq!(actual.base, expected.base);
    assert_eq!(actual.glob, expected.glob);
    assert_eq!(actual.is_brace, expected.is_brace);
    assert_eq!(actual.is_bracket, expected.is_bracket);
    assert_eq!(actual.is_glob, expected.is_glob);
    assert_eq!(actual.is_globstar, expected.is_globstar);
    assert_eq!(actual.is_extglob, expected.is_extglob);
    assert_eq!(actual.negated, expected.negated);
    assert_eq!(actual.negated_extglob, expected.negated_extglob);
}

fn assert_base(pattern: &str, expected: &str) {
    assert_eq!(
        scan(pattern, &ScanOptions::default()).base,
        expected,
        "{pattern}"
    );
}

fn assert_base_with(pattern: &str, options: ScanOptions, expected: &str) {
    assert_eq!(scan(pattern, &options).base, expected, "{pattern}");
}

fn assert_both(pattern: &str, expected: (&str, &str)) {
    let state = scan(pattern, &ScanOptions::default());
    assert_eq!(
        (state.base.as_str(), state.glob.as_str()),
        expected,
        "{pattern}"
    );
}

fn assert_parts(pattern: &str, expected: &[&str]) {
    let state = scan(
        pattern,
        &ScanOptions {
            parts: true,
            ..ScanOptions::default()
        },
    );

    let parts = state.parts.expect("expected parts to be present");
    assert_eq!(parts, expected, "{pattern}");
}

#[test]
fn scan_detects_basic_shape_and_flags() {
    let plain_cases = [
        ("foo/bar", ("foo/bar", "")),
        ("foo/@bar", ("foo/@bar", "")),
        (r"foo/@bar\+", (r"foo/@bar\+", "")),
        ("foo/bar+", ("foo/bar+", "")),
        ("foo/bar*", ("foo", "bar*")),
    ];

    for (pattern, expected) in plain_cases {
        assert_both(pattern, expected);
    }

    assert_scan(
        "./foo/bar/*.js",
        ScanOptions::default(),
        ExpectedScan {
            input: "./foo/bar/*.js",
            prefix: "./",
            start: 2,
            base: "foo/bar",
            glob: "*.js",
            is_brace: false,
            is_bracket: false,
            is_glob: true,
            is_globstar: false,
            is_extglob: false,
            negated: false,
            negated_extglob: false,
        },
    );

    assert_scan(
        "foo/{a,b,c}/*.js",
        ScanOptions {
            scan_to_end: true,
            ..ScanOptions::default()
        },
        ExpectedScan {
            input: "foo/{a,b,c}/*.js",
            prefix: "",
            start: 0,
            base: "foo",
            glob: "{a,b,c}/*.js",
            is_brace: true,
            is_bracket: false,
            is_glob: true,
            is_globstar: false,
            is_extglob: false,
            negated: false,
            negated_extglob: false,
        },
    );

    assert_scan(
        "./foo/**/*.js",
        ScanOptions {
            scan_to_end: true,
            ..ScanOptions::default()
        },
        ExpectedScan {
            input: "./foo/**/*.js",
            prefix: "./",
            start: 2,
            base: "foo",
            glob: "**/*.js",
            is_brace: false,
            is_bracket: false,
            is_glob: true,
            is_globstar: true,
            is_extglob: false,
            negated: false,
            negated_extglob: false,
        },
    );

    let extglob_cases = [
        (
            "./foo/@(foo)/*.js",
            ExpectedScan {
                input: "./foo/@(foo)/*.js",
                prefix: "./",
                start: 2,
                base: "foo",
                glob: "@(foo)/*.js",
                is_brace: false,
                is_bracket: false,
                is_glob: true,
                is_globstar: false,
                is_extglob: true,
                negated: false,
                negated_extglob: false,
            },
        ),
        (
            "!foo/bar/*.js",
            ExpectedScan {
                input: "!foo/bar/*.js",
                prefix: "!",
                start: 1,
                base: "foo/bar",
                glob: "*.js",
                is_brace: false,
                is_bracket: false,
                is_glob: true,
                is_globstar: false,
                is_extglob: false,
                negated: true,
                negated_extglob: false,
            },
        ),
        (
            "!(foo)*",
            ExpectedScan {
                input: "!(foo)*",
                prefix: "",
                start: 0,
                base: "",
                glob: "!(foo)*",
                is_brace: false,
                is_bracket: false,
                is_glob: true,
                is_globstar: false,
                is_extglob: true,
                negated: false,
                negated_extglob: true,
            },
        ),
        (
            "!(foo)",
            ExpectedScan {
                input: "!(foo)",
                prefix: "",
                start: 0,
                base: "",
                glob: "!(foo)",
                is_brace: false,
                is_bracket: false,
                is_glob: true,
                is_globstar: false,
                is_extglob: true,
                negated: false,
                negated_extglob: true,
            },
        ),
        (
            "test/!(foo)/*",
            ExpectedScan {
                input: "test/!(foo)/*",
                prefix: "",
                start: 0,
                base: "test",
                glob: "!(foo)/*",
                is_brace: false,
                is_bracket: false,
                is_glob: true,
                is_globstar: false,
                is_extglob: true,
                negated: false,
                negated_extglob: false,
            },
        ),
        (
            "./!foo/bar/*.js",
            ExpectedScan {
                input: "./!foo/bar/*.js",
                prefix: "./!",
                start: 3,
                base: "foo/bar",
                glob: "*.js",
                is_brace: false,
                is_bracket: false,
                is_glob: true,
                is_globstar: false,
                is_extglob: false,
                negated: true,
                negated_extglob: false,
            },
        ),
        (
            "!./foo/bar/*.js",
            ExpectedScan {
                input: "!./foo/bar/*.js",
                prefix: "!./",
                start: 3,
                base: "foo/bar",
                glob: "*.js",
                is_brace: false,
                is_bracket: false,
                is_glob: true,
                is_globstar: false,
                is_extglob: false,
                negated: true,
                negated_extglob: false,
            },
        ),
    ];

    for (pattern, expected) in extglob_cases {
        assert_scan(pattern, ScanOptions::default(), expected);
    }

    let state = scan(
        "./foo/@(bar)/**/*.js",
        &ScanOptions {
            parts: true,
            ..ScanOptions::default()
        },
    );

    assert_scan_state(
        &state,
        ExpectedScan {
            input: "./foo/@(bar)/**/*.js",
            prefix: "./",
            start: 2,
            base: "foo",
            glob: "@(bar)/**/*.js",
            is_brace: false,
            is_bracket: false,
            is_glob: true,
            is_globstar: true,
            is_extglob: true,
            negated: false,
            negated_extglob: false,
        },
    );
    assert_eq!(state.slashes.expect("slashes"), vec![1, 5, 12, 15]);
    assert_eq!(
        state.parts.expect("parts"),
        vec!["foo", "@(bar)", "**", "*.js"]
    );
}

#[test]
fn scan_strips_glob_magic_to_compute_base_paths() {
    assert_base("./(a|b)", "");

    let cases = [
        (".", "."),
        (".*", ""),
        ("/.*", "/"),
        ("/.*/", "/"),
        ("a/.*/b", "a"),
        ("a*/.*/b", ""),
        ("*/a/b/c", ""),
        ("*", ""),
        ("*/", ""),
        ("*/*", ""),
        ("*/*/", ""),
        ("**", ""),
        ("**/", ""),
        ("**/*", ""),
        ("**/*/", ""),
        ("/*.js", "/"),
        ("*.js", ""),
        ("**/*.js", ""),
        ("/root/path/to/*.js", "/root/path/to"),
        ("[a-z]", ""),
        ("chapter/foo [bar]/", "chapter"),
        ("path/!/foo", "path/!/foo"),
        ("path/!/foo/", "path/!/foo/"),
        ("path/!subdir/foo.js", "path/!subdir/foo.js"),
        ("path/**/*", "path"),
        ("path/**/subdir/foo.*", "path"),
        ("path/*/foo", "path"),
        ("path/*/foo/", "path"),
        ("path/+/foo", "path/+/foo"),
        ("path/+/foo/", "path/+/foo/"),
        ("path/?/foo", "path"),
        ("path/?/foo/", "path"),
        ("path/@/foo", "path/@/foo"),
        ("path/@/foo/", "path/@/foo/"),
        ("path/[a-z]", "path"),
        ("path/subdir/**/foo.js", "path/subdir"),
        ("path/to/*.js", "path/to"),
    ];

    for (pattern, expected) in cases {
        assert_base(pattern, expected);
    }
}

#[test]
fn scan_respects_escaped_characters_and_plain_paths() {
    let escaped_cases = [
        (r"path/\*\*/subdir/foo.*", r"path/\*\*/subdir"),
        (r"path/\[\*\]/subdir/foo.*", r"path/\[\*\]/subdir"),
        (r"path/\[foo bar\]/subdir/foo.*", r"path/\[foo bar\]/subdir"),
        (r"path/\[bar]/", r"path/\[bar]/"),
        (r"path/\[bar]", r"path/\[bar]"),
        ("[bar]", ""),
        ("[bar]/", ""),
        (r"./\[bar]", r"\[bar]"),
        (r"\[bar]/", r"\[bar]/"),
        (r"\[bar\]/", r"\[bar\]/"),
        (r"[bar\]/", r"[bar\]/"),
        (r"path/foo \[bar]/", r"path/foo \[bar]/"),
        (r"\[bar]", r"\[bar]"),
        (r"[bar\]", r"[bar\]"),
    ];

    for (pattern, expected) in escaped_cases {
        assert_base(pattern, expected);
    }

    for pattern in ["path", "path/foo", "path/foo/", "path/foo/bar.js"] {
        assert_base(pattern, pattern);
    }

    assert_scan(
        "./foo/bar/*.js",
        ScanOptions {
            noext: true,
            ..ScanOptions::default()
        },
        ExpectedScan {
            input: "./foo/bar/*.js",
            prefix: "./",
            start: 2,
            base: "foo/bar/*.js",
            glob: "",
            is_brace: false,
            is_bracket: false,
            is_glob: false,
            is_globstar: false,
            is_extglob: false,
            negated: false,
            negated_extglob: false,
        },
    );

    assert_scan(
        "!foo/bar/*.js",
        ScanOptions {
            nonegate: true,
            ..ScanOptions::default()
        },
        ExpectedScan {
            input: "!foo/bar/*.js",
            prefix: "",
            start: 0,
            base: "!foo/bar",
            glob: "*.js",
            is_brace: false,
            is_bracket: false,
            is_glob: true,
            is_globstar: false,
            is_extglob: false,
            negated: false,
            negated_extglob: false,
        },
    );
}

#[test]
fn scan_returns_documented_parts() {
    let cases: &[(&str, &[&str])] = &[
        ("./foo", &["foo"]),
        ("../foo", &["..", "foo"]),
        ("foo/bar", &["foo", "bar"]),
        ("foo/*", &["foo", "*"]),
        ("foo/**", &["foo", "**"]),
        ("foo/**/*", &["foo", "**", "*"]),
        ("フォルダ/**/*", &["フォルダ", "**", "*"]),
        ("foo/!(abc)", &["foo", "!(abc)"]),
        ("c/!(z)/v", &["c", "!(z)", "v"]),
        ("c/@(z)/v", &["c", "@(z)", "v"]),
        ("foo/(bar|baz)", &["foo", "(bar|baz)"]),
        ("foo/(bar|baz)*", &["foo", "(bar|baz)*"]),
        ("**/*(W*, *)*", &["**", "*(W*, *)*"]),
        ("a/**@(/x|/z)/*.md", &["a", "**@(/x|/z)", "*.md"]),
        ("foo/(bar|baz)/*.js", &["foo", "(bar|baz)", "*.js"]),
        (
            "XXX/*/*/12/*/*/m/*/*",
            &["XXX", "*", "*", "12", "*", "*", "m", "*", "*"],
        ),
        (r#"foo/\"**\"/bar"#, &["foo", r#"\"**\""#, "bar"]),
        ("[0-9]/[0-9]", &["[0-9]", "[0-9]"]),
        ("foo/[0-9]/[0-9]", &["foo", "[0-9]", "[0-9]"]),
        ("foo[0-9]/bar[0-9]", &["foo[0-9]", "bar[0-9]"]),
    ];

    for (pattern, expected) in cases {
        assert_parts(pattern, expected);
    }
}

#[test]
fn scan_matches_glob2base_style_cases() {
    let default_cases = [
        ("js/*.js", "js"),
        ("js/**/test/*.js", "js"),
        ("js/test/wow.js", "js/test/wow.js"),
        ("js/t[a-z]st}/*.js", "js"),
        ("js/t+(wo|est)/*.js", "js"),
        ("(a|b)", ""),
        ("foo/(a|b)", "foo"),
        ("/(a|b)", "/"),
        ("a/(b c)", "a"),
        ("foo/(b c)/baz", "foo"),
        ("a/(b c)/", "a"),
        ("a/(b c)/d", "a"),
        (r"a/\(b c)", r"a/\(b c)"),
        (r"a/\+\(b c)/foo", r"a/\+\(b c)/foo"),
        ("js/t(wo|est)/*.js", "js"),
        ("js/t/(wo|est)/*.js", "js/t"),
        ("path/(foo bar)/subdir/foo.*", "path"),
        ("path/(foo/bar|baz)", "path"),
        ("path/(foo/bar|baz)/", "path"),
        ("path/(to|from)", "path"),
        (r"path/\(foo/bar|baz)/", r"path/\(foo/bar|baz)/"),
        (r"path/\*(a|b)", "path"),
        (r"path/\*(a|b)/subdir/foo.*", "path"),
        (r"path/\*/(a|b)/subdir/foo.*", r"path/\*"),
        (r"path/\*\(a\|b\)/subdir/foo.*", r"path/\*\(a\|b\)/subdir"),
    ];

    for (pattern, expected) in default_cases {
        assert_base(pattern, expected);
    }

    let noparen = ScanOptions {
        noparen: true,
        ..ScanOptions::default()
    };
    let noparen_cases = [
        ("a/(b c)", "a/(b c)"),
        ("a/(b c)/", "a/(b c)/"),
        ("a/(b c)/d", "a/(b c)/d"),
        ("foo/(b c)/baz", "foo/(b c)/baz"),
        ("path/(foo bar)/subdir/foo.*", "path/(foo bar)/subdir"),
    ];

    for (pattern, expected) in noparen_cases {
        assert_base_with(pattern, noparen.clone(), expected);
    }

    let windows_cases = [
        (r"C:\path\*.js", r"C:\path\*.js"),
        (r"C:\\path\\*.js", ""),
        (r"C:\\path\*.js", r"C:\\path\*.js"),
    ];

    for (pattern, expected) in windows_cases {
        assert_base(pattern, expected);
    }
}

#[test]
fn scan_parses_glob_base_examples() {
    let cases = [
        ("!foo", ("foo", "")),
        ("*", ("", "*")),
        ("**", ("", "**")),
        ("**/*.md", ("", "**/*.md")),
        ("**/*.min.js", ("", "**/*.min.js")),
        ("**/*foo.js", ("", "**/*foo.js")),
        ("**/.*", ("", "**/.*")),
        ("**/d", ("", "**/d")),
        ("*.*", ("", "*.*")),
        ("*.js", ("", "*.js")),
        ("*.md", ("", "*.md")),
        ("*.min.js", ("", "*.min.js")),
        ("*/*", ("", "*/*")),
        ("*/*/*/*", ("", "*/*/*/*")),
        ("*/*/*/e", ("", "*/*/*/e")),
        ("*/b/*/e", ("", "*/b/*/e")),
        ("*b", ("", "*b")),
        (".*", ("", ".*")),
        ("a/**/j/**/z/*.md", ("a", "**/j/**/z/*.md")),
        ("a/**/z/*.md", ("a", "**/z/*.md")),
        (
            "node_modules/*-glob/**/*.js",
            ("node_modules", "*-glob/**/*.js"),
        ),
        ("{a/b/{c,/foo.js}/e.f.g}", ("", "{a/b/{c,/foo.js}/e.f.g}")),
        (".a*", ("", ".a*")),
        (".b*", ("", ".b*")),
        ("/*", ("/", "*")),
        ("a/***", ("a", "***")),
        ("a/**/b/*.{foo,bar}", ("a", "**/b/*.{foo,bar}")),
        ("a/**/c/*", ("a", "**/c/*")),
        ("a/**/c/*.md", ("a", "**/c/*.md")),
        ("a/**/e", ("a", "**/e")),
        ("a/**c*", ("a", "**c*")),
        ("a/**c/*", ("a", "**c/*")),
        ("a/*/*/e", ("a", "*/*/e")),
        ("a/*/c/*.md", ("a", "*/c/*.md")),
        ("a/b/**/c{d,e}/**/xyz.md", ("a/b", "**/c{d,e}/**/xyz.md")),
        ("a/b/**/e", ("a/b", "**/e")),
        ("a/b/*.{foo,bar}", ("a/b", "*.{foo,bar}")),
        ("a/b/*/e", ("a/b", "*/e")),
        ("a/b/.git/", ("a/b/.git/", "")),
        ("a/b/.git/**", ("a/b/.git", "**")),
        ("a/b/.{foo,bar}", ("a/b", ".{foo,bar}")),
        ("a/b/c/*", ("a/b/c", "*")),
        ("a/b/c/**/*.min.js", ("a/b/c", "**/*.min.js")),
        ("a/b/c/*.md", ("a/b/c", "*.md")),
        ("a/b/c/.*.md", ("a/b/c", ".*.md")),
        (
            "a/b/{c,.gitignore,{a,b}}/{a,b}/abc.foo.js",
            ("a/b", "{c,.gitignore,{a,b}}/{a,b}/abc.foo.js"),
        ),
        ("a/b/{c,/.gitignore}", ("a/b", "{c,/.gitignore}")),
        ("a/b/{c,d}/", ("a/b", "{c,d}/")),
        ("a/b/{c,d}/e/f.g", ("a/b", "{c,d}/e/f.g")),
        ("b/*/*/*", ("b", "*/*/*")),
        (".md", (".md", "")),
        ("!*.min.js", ("", "*.min.js")),
        ("!foo/*.js", ("foo", "*.js")),
        ("!foo/(a|b).min.js", ("foo", "(a|b).min.js")),
        ("!foo/[a-b].min.js", ("foo", "[a-b].min.js")),
        ("!foo/{a,b}.min.js", ("foo", "{a,b}.min.js")),
        ("a/b/c/!foo", ("a/b/c/!foo", "")),
        ("/a/b/!(a|b)/e.f.g/", ("/a/b", "!(a|b)/e.f.g/")),
        ("/a/b/@(a|b)/e.f.g/", ("/a/b", "@(a|b)/e.f.g/")),
        ("@(a|b)/e.f.g/", ("", "@(a|b)/e.f.g/")),
    ];

    for (pattern, expected) in cases {
        assert_both(pattern, expected);
    }

    for pattern in [
        "path/!(to|from)",
        "path/*(to|from)",
        "path/+(to|from)",
        "path/?(to|from)",
        "path/@(to|from)",
    ] {
        assert_base(pattern, "path");
    }
}

#[test]
fn scan_handles_character_classes_qmarks_non_globs_and_braces() {
    let unescape = ScanOptions {
        unescape: true,
        ..ScanOptions::default()
    };

    let regex_cases = [
        ("[a-c]b*", ("", "[a-c]b*")),
        ("[a-j]*[^c]", ("", "[a-j]*[^c]")),
        ("[a-j]*[^c]b/c", ("", "[a-j]*[^c]b/c")),
        ("[a-j]*[^c]bc", ("", "[a-j]*[^c]bc")),
        ("[ab][ab]", ("", "[ab][ab]")),
        ("foo/[a-b].min.js", ("foo", "[a-b].min.js")),
    ];

    for (pattern, expected) in regex_cases {
        assert_both(pattern, expected);
    }

    assert_base_with(r"path/foo[a\/]/", unescape.clone(), "path");
    assert_base_with(r"path/foo\[a\/]/", unescape.clone(), r"path/foo[a\/]/");
    assert_base_with(r"foo[a\/]", unescape.clone(), "");
    assert_base_with(r"foo\[a\/]", unescape.clone(), r"foo[a\/]");

    let qmark_cases = [
        ("?", ("", "?")),
        ("?/?", ("", "?/?")),
        ("??", ("", "??")),
        ("???", ("", "???")),
        ("?a", ("", "?a")),
        ("?b", ("", "?b")),
        ("a?b", ("", "a?b")),
        ("a/?/c.js", ("a", "?/c.js")),
        ("a/?/c.md", ("a", "?/c.md")),
        ("a/?/c/?/*/f.js", ("a", "?/c/?/*/f.js")),
        ("a/?/c/?/*/f.md", ("a", "?/c/?/*/f.md")),
        ("a/?/c/?/e.js", ("a", "?/c/?/e.js")),
        ("a/?/c/?/e.md", ("a", "?/c/?/e.md")),
        ("a/?/c/???/e.js", ("a", "?/c/???/e.js")),
        ("a/?/c/???/e.md", ("a", "?/c/???/e.md")),
        ("a/??/c.js", ("a", "??/c.js")),
        ("a/??/c.md", ("a", "??/c.md")),
        ("a/???/c.js", ("a", "???/c.js")),
        ("a/???/c.md", ("a", "???/c.md")),
        ("a/????/c.js", ("a", "????/c.js")),
    ];

    for (pattern, expected) in qmark_cases {
        assert_both(pattern, expected);
    }

    for pattern in [
        "",
        ".",
        "a",
        ".a",
        "/a",
        "a/",
        "/a/",
        "/a/b/c",
        "/a/b/c/",
        "a/b/c/",
        "a.min.js",
        "a/.x.md",
        "a/b/.gitignore",
        "a/b/c/d.md",
        "a/b/c/d.e.f/g.min.js",
        "a/b/.git",
        "a/b/.git/",
        "a/b/c",
        "a/b/c.d/e.md",
        "a/b/c.md",
        "a/b/c.min.js",
        "a/b/git/",
        "aa",
        "ab",
        "bb",
        "c.md",
        "foo",
    ] {
        assert_both(pattern, (pattern, ""));
    }

    let brace_base_cases = [
        ("path/{to,from}", "path"),
        ("path/{foo,bar}/", "path"),
        ("js/{src,test}/*.js", "js"),
        ("{a,b}", ""),
        ("/{a,b}", "/"),
        ("/{a,b}/", "/"),
        ("js/test{0..9}/*.js", "js"),
        ("one/{foo,bar}/**/{baz,qux}/*.txt", "one"),
        ("two/baz/**/{abc,xyz}/*.js", "two/baz"),
        ("foo/{bar,baz}/**/aaa/{bbb,ccc}", "foo"),
    ];

    for (pattern, expected) in brace_base_cases {
        assert_base(pattern, expected);
    }

    let unescape_brace_cases = [
        (r"path/\{,/,bar/baz,qux}/", "path/{,/,bar/baz,qux}/"),
        (r"path/\{,/,bar/baz,qux\}/", "path/{,/,bar/baz,qux}/"),
        (r"/\{,/,bar/baz,qux}/", "/{,/,bar/baz,qux}/"),
        (r"\{,/,bar/baz,qux\}", "{,/,bar/baz,qux}"),
        (r"\{,/,bar/baz,qux}/", "{,/,bar/baz,qux}/"),
        (r"\{../,./,\{bar,/baz},qux}", "{../,./,{bar,/baz},qux}"),
        (r"\{../,./,\{bar,/baz},qux}/", "{../,./,{bar,/baz},qux}/"),
        (r"path/\{,/,bar/{baz,qux}}/", "path/{,/,bar/{baz,qux}}/"),
        (
            r"path/\{../,./,\{bar,/baz},qux}/",
            "path/{../,./,{bar,/baz},qux}/",
        ),
        (
            r"path/\{../,./,{bar,/baz},qux}/",
            "path/{../,./,{bar,/baz},qux}/",
        ),
        (r"\{foo,bar\}", "{foo,bar}"),
        (r"\{foo,bar\}/", "{foo,bar}/"),
        (r"\{foo,bar}/", "{foo,bar}/"),
        (r"path/\{foo,bar}/", "path/{foo,bar}/"),
    ];

    for (pattern, expected) in unescape_brace_cases {
        assert_base_with(pattern, unescape.clone(), expected);
    }

    assert_base_with("path/{,/,bar/baz,qux}/", unescape.clone(), "path");
    assert_base_with("/{,/,bar/baz,qux}/", unescape.clone(), "/");
    assert_base_with("{,/,bar/baz,qux}", unescape.clone(), "");
    assert_base_with("path/{,/,bar/\\{baz,qux}}/", unescape, "path");

    let brace_both_cases = [
        ("/a/b/{c,/foo.js}/e.f.g/", ("/a/b", "{c,/foo.js}/e.f.g/")),
        (
            "{a/b/c.js,/a/b/{c,/foo.js}/e.f.g/}",
            ("", "{a/b/c.js,/a/b/{c,/foo.js}/e.f.g/}"),
        ),
        ("/a/b/{c,d}/", ("/a/b", "{c,d}/")),
        ("/a/b/{c,d}/*.js", ("/a/b", "{c,d}/*.js")),
        ("/a/b/{c,d}/*.min.js", ("/a/b", "{c,d}/*.min.js")),
        ("/a/b/{c,d}/e.f.g/", ("/a/b", "{c,d}/e.f.g/")),
        ("{.,*}", ("", "{.,*}")),
        ("a/b/.{c,.gitignore}", ("a/b", ".{c,.gitignore}")),
        ("a/b/.{c,/.gitignore}", ("a/b", ".{c,/.gitignore}")),
        ("a/b/.{foo,bar}", ("a/b", ".{foo,bar}")),
        ("a/b/{c,.gitignore}", ("a/b", "{c,.gitignore}")),
        ("a/b/{c,/.gitignore}", ("a/b", "{c,/.gitignore}")),
        ("a/b/{c,/gitignore}", ("a/b", "{c,/gitignore}")),
        ("a/b/{c,d}", ("a/b", "{c,d}")),
        ("a/b/{c,./d}/e/f.g", ("a/b", "{c,./d}/e/f.g")),
        ("a/b/{c,./d}/e/f.min.g", ("a/b", "{c,./d}/e/f.min.g")),
        (
            "a/b/{c,.gitignore,{a,./b}}/{a,b}/abc.foo.js",
            ("a/b", "{c,.gitignore,{a,./b}}/{a,b}/abc.foo.js"),
        ),
        (
            "a/b/{c,.gitignore,{a,b}}/{a,b}/*.foo.js",
            ("a/b", "{c,.gitignore,{a,b}}/{a,b}/*.foo.js"),
        ),
        (
            "a/b/{c,.gitignore,{a,b}}/{a,b}/abc.foo.js",
            ("a/b", "{c,.gitignore,{a,b}}/{a,b}/abc.foo.js"),
        ),
        ("a/b/{c,/d}/e/f.g", ("a/b", "{c,/d}/e/f.g")),
        ("a/b/{c,/d}/e/f.min.g", ("a/b", "{c,/d}/e/f.min.g")),
        ("a/b/{c,d}/", ("a/b", "{c,d}/")),
        ("a/b/{c,d}/*.js", ("a/b", "{c,d}/*.js")),
        ("a/b/{c,d}/*.min.js", ("a/b", "{c,d}/*.min.js")),
        ("a/b/{c,d}/e.f.g/", ("a/b", "{c,d}/e.f.g/")),
        ("a/b/{c,d}/e/f.g", ("a/b", "{c,d}/e/f.g")),
        ("a/b/{c,d}/e/f.min.g", ("a/b", "{c,d}/e/f.min.g")),
        ("foo/{a,b}.min.js", ("foo", "{a,b}.min.js")),
    ];

    for (pattern, expected) in brace_both_cases {
        assert_both(pattern, expected);
    }
}

#[test]
fn scans_basic_glob() {
    let state = scan("./foo/bar/*.js", &ScanOptions::default());

    assert_eq!(state.prefix, "./");
    assert_eq!(state.start, 2);
    assert_eq!(state.base, "foo/bar");
    assert_eq!(state.glob, "*.js");
    assert!(state.is_glob);
    assert!(!state.is_extglob);
}

#[test]
fn scans_negated_extglob() {
    let state = scan("!(foo)*", &ScanOptions::default());

    assert_eq!(state.base, "");
    assert_eq!(state.glob, "!(foo)*");
    assert!(state.is_glob);
    assert!(state.is_extglob);
    assert!(state.negated_extglob);
}

#[test]
fn scans_parts_with_prefix() {
    let state = scan(
        "./foo/@(bar)/**/*.js",
        &ScanOptions {
            parts: true,
            ..ScanOptions::default()
        },
    );

    assert_eq!(state.base, "foo");
    assert_eq!(state.glob, "@(bar)/**/*.js");
    assert_eq!(
        state.parts,
        Some(vec![
            "foo".to_string(),
            "@(bar)".to_string(),
            "**".to_string(),
            "*.js".to_string()
        ])
    );
    assert_eq!(state.slashes.as_deref(), Some(&[1, 5, 12, 15][..]));
}

#[test]
fn unescapes_escaped_braces() {
    let state = scan(
        "path/\\{foo,bar}/",
        &ScanOptions {
            unescape: true,
            ..ScanOptions::default()
        },
    );

    assert_eq!(state.base, "path/{foo,bar}/");
    assert_eq!(state.glob, "");
    assert!(!state.is_glob);
}

#[test]
fn preserves_backslashes_inside_literal_brackets() {
    let state = scan(
        "path/foo\\[a\\\\/]/",
        &ScanOptions {
            unescape: true,
            ..ScanOptions::default()
        },
    );

    assert_eq!(state.base, "path/foo[a\\\\/]/");
    assert_eq!(state.glob, "");
    assert!(!state.is_glob);
}
