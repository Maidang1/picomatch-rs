mod support;

use picomatch_rs::CompileOptions;

use support::{assert_is_match, assert_match_list, default_compile_options};

#[test]
fn should_not_match_dotfiles_by_default() {
    assert_match_list(&[".dotfile"], "*", default_compile_options(), &[]);
    assert_match_list(&[".dotfile"], "**", default_compile_options(), &[]);
    assert_match_list(
        &["a/b/c/.dotfile.md"],
        "*.md",
        default_compile_options(),
        &[],
    );
    assert_match_list(
        &["a/b", "a/.b", ".a/b", ".a/.b"],
        "**",
        default_compile_options(),
        &["a/b"],
    );
    assert_match_list(&["a/b/c/.dotfile"], "*.*", default_compile_options(), &[]);
}

#[test]
fn should_match_dotfiles_when_a_leading_dot_is_defined_in_the_path() {
    assert_match_list(
        &["a/b/c/.dotfile.md"],
        "**/.*",
        default_compile_options(),
        &["a/b/c/.dotfile.md"],
    );
    assert_match_list(
        &["a/b/c/.dotfile.md"],
        "**/.*.md",
        default_compile_options(),
        &["a/b/c/.dotfile.md"],
    );
}

#[test]
fn should_use_negation_patterns_on_dotfiles() {
    assert_match_list(
        &[".a", ".b", "c", "c.md"],
        "!.*",
        default_compile_options(),
        &["c", "c.md"],
    );
    assert_match_list(
        &[".a", ".b", "c", "c.md"],
        "!.b",
        default_compile_options(),
        &[".a", "c", "c.md"],
    );
}

#[test]
fn should_match_dotfiles_when_there_is_a_leading_dot() {
    let opts = CompileOptions {
        dot: true,
        ..CompileOptions::default()
    };

    assert_match_list(&[".dotfile"], "*", opts.clone(), &[".dotfile"]);
    assert_match_list(&[".dotfile"], "**", opts.clone(), &[".dotfile"]);
    assert_match_list(
        &["a/b", "a/.b", ".a/b", ".a/.b"],
        "**",
        opts.clone(),
        &["a/b", "a/.b", ".a/b", ".a/.b"],
    );
    assert_match_list(
        &["a/b", "a/.b", ".a/.b"],
        "a/{.*,**}",
        opts.clone(),
        &["a/b", "a/.b"],
    );
    assert_match_list(
        &["a/b", "a/.b", ".a/.b"],
        "{.*,**}",
        default_compile_options(),
        &["a/b"],
    );
    assert_match_list(
        &["a/b", "a/.b", ".a/.b"],
        "{.*,**}",
        opts.clone(),
        &["a/b", "a/.b", ".a/.b"],
    );
    assert_match_list(&[".dotfile"], ".dotfile", opts.clone(), &[".dotfile"]);
    assert_match_list(&[".dotfile.md"], ".*.md", opts.clone(), &[".dotfile.md"]);
}

#[test]
fn should_match_dotfiles_when_there_is_not_a_leading_dot() {
    let opts = CompileOptions {
        dot: true,
        ..CompileOptions::default()
    };

    assert_match_list(&[".dotfile"], "*.*", opts.clone(), &[".dotfile"]);
    assert_match_list(
        &[".a", ".b", "c", "c.md"],
        "*.*",
        opts.clone(),
        &[".a", ".b", "c.md"],
    );
    assert_match_list(&[".dotfile"], "*.md", opts.clone(), &[]);
    assert_match_list(&[".verb.txt"], "*.md", opts.clone(), &[]);
    assert_match_list(&["a/b/c/.dotfile"], "*.md", opts.clone(), &[]);
    assert_match_list(&["a/b/c/.dotfile.md"], "*.md", opts.clone(), &[]);
    assert_match_list(
        &["a/b/c/.verb.md"],
        "**/*.md",
        opts.clone(),
        &["a/b/c/.verb.md"],
    );
    assert_match_list(&["foo.md"], "*.md", opts.clone(), &["foo.md"]);
}

#[test]
fn should_use_negation_patterns_on_dotfiles_with_extglob() {
    assert_match_list(
        &[".a", ".b", "c", "c.md"],
        "!.*",
        default_compile_options(),
        &["c", "c.md"],
    );
    assert_match_list(
        &[".a", ".b", "c", "c.md"],
        "!(.*)",
        default_compile_options(),
        &["c", "c.md"],
    );
    assert_match_list(
        &[".a", ".b", "c", "c.md"],
        "!(.*)*",
        default_compile_options(),
        &["c", "c.md"],
    );
    assert_match_list(
        &[".a", ".b", "c", "c.md"],
        "!*.*",
        default_compile_options(),
        &[".a", ".b", "c"],
    );
}

#[test]
fn should_match_dotfiles_when_options_dot_is_true() {
    let fixtures = ["a/./b", "a/../b", "a/c/b", "a/.d/b"];
    let dot_true = CompileOptions {
        dot: true,
        ..CompileOptions::default()
    };
    let dot_false = CompileOptions {
        dot: false,
        ..CompileOptions::default()
    };

    assert_match_list(&[".dotfile"], "*.*", dot_true.clone(), &[".dotfile"]);
    assert_match_list(&[".dotfile"], "*.md", dot_true.clone(), &[]);
    assert_match_list(&[".dotfile"], ".dotfile", dot_true.clone(), &[".dotfile"]);
    assert_match_list(
        &[".dotfile.md"],
        ".*.md",
        dot_true.clone(),
        &[".dotfile.md"],
    );
    assert_match_list(&[".verb.txt"], "*.md", dot_true.clone(), &[]);
    assert_match_list(&["a/b/c/.dotfile"], "*.md", dot_true.clone(), &[]);
    assert_match_list(
        &["a/b/c/.dotfile.md"],
        "**/*.md",
        dot_true.clone(),
        &["a/b/c/.dotfile.md"],
    );
    assert_match_list(
        &["a/b/c/.dotfile.md"],
        "**/.*",
        dot_false.clone(),
        &["a/b/c/.dotfile.md"],
    );
    assert_match_list(
        &["a/b/c/.dotfile.md"],
        "**/.*.md",
        dot_false.clone(),
        &["a/b/c/.dotfile.md"],
    );
    assert_match_list(&["a/b/c/.dotfile.md"], "*.md", dot_false.clone(), &[]);
    assert_match_list(&["a/b/c/.dotfile.md"], "*.md", dot_true.clone(), &[]);
    assert_match_list(
        &["a/b/c/.verb.md"],
        "**/*.md",
        dot_true.clone(),
        &["a/b/c/.verb.md"],
    );
    assert_match_list(&["d.md"], "*.md", dot_true.clone(), &["d.md"]);
    assert_match_list(&fixtures, "a/*/b", dot_true.clone(), &["a/c/b", "a/.d/b"]);
    assert_match_list(&fixtures, "a/.*/b", default_compile_options(), &["a/.d/b"]);
    assert_match_list(&fixtures, "a/.*/b", dot_true.clone(), &["a/.d/b"]);
}

#[test]
fn should_match_dotfiles_when_options_dot_is_true_is_match() {
    let dot_true = CompileOptions {
        dot: true,
        ..CompileOptions::default()
    };

    assert_is_match(".dot", "**/*dot", dot_true.clone(), true);
    assert_is_match(".dot", "*dot", dot_true.clone(), true);
    assert_is_match(".dot", "?dot", dot_true.clone(), true);
    assert_is_match(".dotfile.js", ".*.js", dot_true.clone(), true);
    assert_is_match("/a/b/.dot", "/**/*dot", dot_true.clone(), true);
    assert_is_match("/a/b/.dot", "**/*dot", dot_true.clone(), true);
    assert_is_match("/a/b/.dot", "**/.[d]ot", dot_true.clone(), true);
    assert_is_match("/a/b/.dot", "**/?dot", dot_true.clone(), true);
    assert_is_match("/a/b/.dot", "/**/.[d]ot", dot_true.clone(), true);
    assert_is_match("/a/b/.dot", "/**/?dot", dot_true.clone(), true);
    assert_is_match("a/b/.dot", "**/*dot", dot_true.clone(), true);
    assert_is_match("a/b/.dot", "**/.[d]ot", dot_true.clone(), true);
    assert_is_match("a/b/.dot", "**/?dot", dot_true.clone(), true);
}

#[test]
fn should_not_match_dotfiles_when_options_dot_is_false() {
    let dot_false = CompileOptions {
        dot: false,
        ..CompileOptions::default()
    };

    assert_is_match("a/b/.dot", "**/*dot", dot_false.clone(), false);
    assert_is_match("a/b/.dot", "**/?dot", dot_false.clone(), false);
}

#[test]
fn should_not_match_dotfiles_when_dot_is_not_defined_and_a_dot_is_not_in_the_glob_pattern() {
    assert_is_match("a/b/.dot", "**/*dot", default_compile_options(), false);
    assert_is_match("a/b/.dot", "**/?dot", default_compile_options(), false);
}

#[test]
fn micromatch_issue_63_dots() {
    let dot_true = CompileOptions {
        dot: true,
        ..CompileOptions::default()
    };

    assert_is_match(
        "/aaa/.git/foo",
        "/aaa/**/*",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "/aaa/bbb/.git",
        "/aaa/bbb/*",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "/aaa/bbb/.git",
        "/aaa/bbb/**",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "/aaa/bbb/ccc/.git",
        "/aaa/bbb/**",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "aaa/bbb/.git",
        "aaa/bbb/**",
        default_compile_options(),
        false,
    );
    assert_is_match("/aaa/bbb/", "/aaa/bbb/**", default_compile_options(), true);
    assert_is_match(
        "/aaa/bbb/foo",
        "/aaa/bbb/**",
        default_compile_options(),
        true,
    );

    assert_is_match("/aaa/.git/foo", "/aaa/**/*", dot_true.clone(), true);
    assert_is_match("/aaa/bbb/.git", "/aaa/bbb/*", dot_true.clone(), true);
    assert_is_match("/aaa/bbb/.git", "/aaa/bbb/**", dot_true.clone(), true);
    assert_is_match("/aaa/bbb/ccc/.git", "/aaa/bbb/**", dot_true.clone(), true);
    assert_is_match("aaa/bbb/.git", "aaa/bbb/**", dot_true.clone(), true);
}

#[test]
fn should_not_match_dotfiles_with_single_stars_by_default() {
    assert_is_match("foo", "*", default_compile_options(), true);
    assert_is_match("foo/bar", "*/*", default_compile_options(), true);
    assert_is_match(".foo", "*", default_compile_options(), false);
    assert_is_match(".foo/bar", "*/*", default_compile_options(), false);
    assert_is_match(".foo/.bar", "*/*", default_compile_options(), false);
    assert_is_match("foo/.bar", "*/*", default_compile_options(), false);
    assert_is_match("foo/.bar/baz", "*/*/*", default_compile_options(), false);
}

#[test]
fn should_work_with_dots_in_the_path() {
    let dot_true = CompileOptions {
        dot: true,
        ..CompileOptions::default()
    };

    assert_is_match("../test.js", "../*.js", default_compile_options(), true);
    assert_is_match("../.test.js", "../*.js", dot_true.clone(), true);
    assert_is_match("../.test.js", "../*.js", default_compile_options(), false);
}

#[test]
fn should_not_match_dotfiles_with_globstar_by_default() {
    assert_is_match(".foo", "**/**", default_compile_options(), false);
    assert_is_match(".foo", "**", default_compile_options(), false);
    assert_is_match(".foo", "**/*", default_compile_options(), false);
    assert_is_match("bar/.foo", "**/*", default_compile_options(), false);
    assert_is_match(".bar", "**/*", default_compile_options(), false);
    assert_is_match("foo/.bar", "**/*", default_compile_options(), false);
    assert_is_match("foo/.bar", "**/*a*", default_compile_options(), false);
}

#[test]
fn should_match_dotfiles_when_a_leading_dot_is_in_the_pattern() {
    assert_is_match("foo", "**/.*a*", default_compile_options(), false);
    assert_is_match(".bar", "**/.*a*", default_compile_options(), true);
    assert_is_match("foo/.bar", "**/.*a*", default_compile_options(), true);
    assert_is_match(".foo", "**/.*", default_compile_options(), true);

    assert_is_match("foo", ".*a*", default_compile_options(), false);
    assert_is_match(".bar", ".*a*", default_compile_options(), true);
    assert_is_match("bar", ".*a*", default_compile_options(), false);

    assert_is_match("foo", ".b*", default_compile_options(), false);
    assert_is_match(".bar", ".b*", default_compile_options(), true);
    assert_is_match("bar", ".b*", default_compile_options(), false);

    assert_is_match("foo", ".*r", default_compile_options(), false);
    assert_is_match(".bar", ".*r", default_compile_options(), true);
    assert_is_match("bar", ".*r", default_compile_options(), false);
}

#[test]
fn should_not_match_a_dot_when_the_dot_is_not_explicitly_defined() {
    for (input, pattern) in [
        (".dot", "**/*dot"),
        (".dot", "**/?dot"),
        (".dot", "*/*dot"),
        (".dot", "*/?dot"),
        (".dot", "*dot"),
        (".dot", "/*dot"),
        (".dot", "/?dot"),
        ("/.dot", "**/*dot"),
        ("/.dot", "**/?dot"),
        ("/.dot", "*/*dot"),
        ("/.dot", "*/?dot"),
        ("/.dot", "/*dot"),
        ("/.dot", "/?dot"),
        ("abc/.dot", "*/*dot"),
        ("abc/.dot", "*/?dot"),
        ("abc/.dot", "abc/*dot"),
        ("abc/abc/.dot", "**/*dot"),
        ("abc/abc/.dot", "**/?dot"),
    ] {
        assert_is_match(input, pattern, default_compile_options(), false);
    }
}

#[test]
fn should_not_match_leading_dots_with_question_marks() {
    assert_is_match(".dot", "?dot", default_compile_options(), false);
    assert_is_match("/.dot", "/?dot", default_compile_options(), false);
    assert_is_match("abc/.dot", "abc/?dot", default_compile_options(), false);
}

#[test]
fn should_match_double_dots_when_defined_in_pattern() {
    for (input, pattern) in [
        ("../../b", "**/../*"),
        ("../../b", "*/../*"),
        ("../../b", "../*"),
        ("../abc", "*/../*"),
        ("../c/d", "**/../*"),
        ("../c/d", "*/../*"),
        ("../c/d", "../*"),
        ("abc", "**/../*"),
        ("abc", "*/../*"),
        ("abc", "../*"),
        ("abc/../abc", "../*"),
        ("abc/../", "**/../*"),
    ] {
        assert_is_match(input, pattern, default_compile_options(), false);
    }

    for (input, pattern) in [
        ("..", ".."),
        ("../b", "../*"),
        ("../../b", "../../*"),
        ("../../..", "../../.."),
        ("../abc", "**/../*"),
        ("../abc", "../*"),
        ("abc/../abc", "**/../*"),
        ("abc/../abc", "*/../*"),
    ] {
        assert_is_match(input, pattern, default_compile_options(), true);
    }
}

#[test]
fn should_not_match_double_dots_when_not_defined_in_pattern() {
    for (input, pattern) in [
        ("../abc", "**/*"),
        ("../abc", "**/**/**"),
        ("../abc", "**/**/abc"),
        ("../abc", "**/**/abc/**"),
        ("../abc", "**/*/*"),
        ("../abc", "**/abc/**"),
        ("../abc", "*/*"),
        ("../abc", "*/abc/**"),
        ("abc/../abc", "**/*"),
        ("abc/../abc", "**/*/*"),
        ("abc/../abc", "**/*/abc"),
        ("abc/../abc", "*/**/*"),
        ("abc/../abc", "*/*/*"),
        ("abc/../abc", "abc/**/*"),
        ("abc/../abc", "**/**/*"),
        ("abc/../abc", "**/*/*"),
        ("abc/../abc", "*/**/*"),
        ("abc/../abc", "*/*/*"),
    ] {
        assert_is_match(input, pattern, default_compile_options(), false);
    }

    let dot_true = CompileOptions {
        dot: true,
        ..CompileOptions::default()
    };

    for (input, pattern) in [
        ("../abc", "**/**/**"),
        ("../abc", "**/**/abc"),
        ("../abc", "**/**/abc/**"),
        ("../abc", "**/abc/**"),
        ("../abc", "*/abc/**"),
        ("../abc", "**/*/*"),
        ("../abc", "*/*"),
        ("abc/../abc", "**/*/*"),
        ("abc/../abc", "*/*/*"),
        ("abc/../abc", "**/*/*"),
        ("abc/../abc", "*/*/*"),
        ("abc/..", "**/*"),
        ("abc/..", "*/*"),
        ("abc/abc/..", "*/**/*"),
    ] {
        assert_is_match(input, pattern, dot_true.clone(), false);
    }

    for (input, pattern) in [
        ("abc/../abc", "abc/**/*"),
        ("abc/../abc", "abc/**/*"),
        ("abc/../abc", "abc/**/*/*"),
        ("abc/../abc", "abc/*/*/*"),
        ("abc/../abc", "abc/**/*/*"),
        ("abc/../abc", "abc/*/*/*"),
        ("abc/..", "abc/**/*"),
        ("abc/..", "abc/*/*"),
        ("abc/abc/..", "abc/*/**/*"),
    ] {
        assert_is_match(input, pattern, default_compile_options(), false);
    }

    for (input, pattern) in [
        ("abc/../abc", "abc/**/*"),
        ("abc/../abc", "abc/**/*/*"),
        ("abc/../abc", "abc/**/*/*"),
        ("abc/../abc", "abc/*/*/*"),
        ("abc/../abc", "abc/**/*/*"),
        ("abc/../abc", "abc/*/*/*"),
        ("abc/..", "abc/**/*"),
        ("abc/..", "abc/*/*"),
        ("abc/abc/..", "abc/*/**/*"),
    ] {
        assert_is_match(input, pattern, dot_true.clone(), false);
    }

    let strict_slashes = CompileOptions {
        strict_slashes: true,
        ..CompileOptions::default()
    };

    for (input, pattern) in [
        ("abc/../abc", "abc/**/*"),
        ("abc/../abc", "abc/**/*/*"),
        ("abc/../abc", "abc/**/*/*"),
        ("abc/../abc", "abc/*/*/*"),
        ("abc/../abc", "abc/**/*/*"),
        ("abc/../abc", "abc/*/*/*"),
        ("abc/..", "abc/**/*"),
        ("abc/..", "abc/*/*"),
        ("abc/abc/..", "abc/*/**/*"),
    ] {
        assert_is_match(input, pattern, strict_slashes.clone(), false);
    }
}

#[test]
fn should_not_match_single_exclusive_dots_when_not_defined_in_pattern() {
    for (input, pattern) in [
        (".", "**"),
        ("abc/./abc", "**"),
        ("abc/abc/.", "**"),
        ("abc/abc/./abc", "**"),
    ] {
        assert_is_match(input, pattern, default_compile_options(), false);
    }

    let dot_true = CompileOptions {
        dot: true,
        ..CompileOptions::default()
    };

    for (input, pattern) in [
        (".", "**"),
        ("..", "**"),
        ("../", "**"),
        ("/../", "**"),
        ("/..", "**"),
        ("abc/./abc", "**"),
        ("abc/abc/.", "**"),
        ("abc/abc/./abc", "**"),
    ] {
        assert_is_match(input, pattern, dot_true.clone(), false);
    }
}

#[test]
fn should_match_leading_dots_in_root_path_when_glob_is_prefixed_with_doublestar() {
    assert_is_match(".abc/.abc", "**/.abc/**", default_compile_options(), false);

    for (input, pattern) in [
        (".abc", "**/.abc/**"),
        (".abc/", "**/.abc/**"),
        (".abc/abc", "**/.abc/**"),
        (".abc/abc/b", "**/.abc/**"),
        ("abc/.abc/b", "**/.abc/**"),
        ("abc/abc/.abc", "**/.abc"),
        ("abc/abc/.abc", "**/.abc/**"),
        ("abc/abc/.abc/", "**/.abc/**"),
        ("abc/abc/.abc/abc", "**/.abc/**"),
        ("abc/abc/.abc/c/d", "**/.abc/**"),
        ("abc/abc/.abc/c/d/e", "**/.abc/**"),
    ] {
        assert_is_match(input, pattern, default_compile_options(), true);
    }
}

#[test]
fn should_match_a_dot_when_the_dot_is_explicitly_defined() {
    let strict_slashes = CompileOptions {
        strict_slashes: true,
        ..CompileOptions::default()
    };

    for (input, pattern) in [
        ("/.dot", "**/.dot*"),
        ("aaa/bbb/.dot", "**/.dot*"),
        ("aaa/.dot", "*/.dot*"),
        (".aaa.bbb", ".*.*"),
        (".aaa.bbb", ".*.*"),
        (".aaa.bbb/", ".*.*/"),
        (".aaa.bbb/", ".*.*{,/}"),
        (".aaa.bbb", ".*.bbb"),
        (".dotfile.js", ".*.js"),
        (".dot", ".*ot"),
        (".dot.bbb.ccc", ".*ot.*.*"),
        (".dot", ".d?t"),
        (".dot", ".dot*"),
        ("/.dot", "/.dot*"),
    ] {
        assert_is_match(input, pattern, default_compile_options(), true);
    }

    assert_is_match(".aaa.bbb/", ".*.*", strict_slashes.clone(), false);
    assert_is_match(".aaa.bbb", ".*.*/", default_compile_options(), false);
}

#[test]
fn should_match_dots_defined_in_brackets() {
    for (input, pattern) in [
        ("/.dot", "**/.[d]ot"),
        ("aaa/.dot", "**/.[d]ot"),
        ("aaa/bbb/.dot", "**/.[d]ot"),
        ("aaa/.dot", "*/.[d]ot"),
        (".dot", ".[d]ot"),
        ("/.dot", "/.[d]ot"),
    ] {
        assert_is_match(input, pattern, default_compile_options(), true);
    }
}
