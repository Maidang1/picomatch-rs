mod support;

use support::{assert_is_match, assert_match_list, default_compile_options};

#[test]
fn matches_paths_with_no_slashes() {
    // micromatch/#15
    assert_is_match("a.js", "**/*.js", default_compile_options(), true);
    assert_is_match("a.js", "**/a*", default_compile_options(), true);
    assert_is_match("a.js", "**/a*.js", default_compile_options(), true);
    assert_is_match("abc", "**/abc", default_compile_options(), true);
}

#[test]
fn regards_non_exclusive_double_stars_as_single_stars() {
    let fixtures = [
        "a",
        "a/",
        "a/a",
        "a/a/",
        "a/a/a",
        "a/a/a/",
        "a/a/a/a",
        "a/a/a/a/",
        "a/a/a/a/a",
        "a/a/a/a/a/",
        "a/a/b",
        "a/a/b/",
        "a/b",
        "a/b/",
        "a/b/c/.d/e/",
        "a/c",
        "a/c/",
        "a/b",
        "a/x/",
        "b",
        "b/",
        "x/y",
        "x/y/",
        "z/z",
        "z/z/",
    ];

    assert_match_list(
        &fixtures,
        "**a/a/*/",
        default_compile_options(),
        &["a/a/a/", "a/a/b/"],
    );

    assert_is_match("aaa/bba/ccc", "aaa/**ccc", default_compile_options(), false);
    assert_is_match("aaa/bba/ccc", "aaa/**z", default_compile_options(), false);
    assert_is_match(
        "aaa/bba/ccc",
        "aaa/**b**/ccc",
        default_compile_options(),
        true,
    );
    assert_is_match("a/b/c", "**c", default_compile_options(), false);
    assert_is_match("a/b/c", "a/**c", default_compile_options(), false);
    assert_is_match("a/b/c", "a/**z", default_compile_options(), false);
    assert_is_match("a/b/c/b/c", "a/**b**/c", default_compile_options(), false);
    assert_is_match(
        "a/b/c/d/e.js",
        "a/b/c**/*.js",
        default_compile_options(),
        false,
    );
    assert_is_match("a/b/c/b/c", "a/**/b/**/c", default_compile_options(), true);
    assert_is_match("a/aba/c", "a/**b**/c", default_compile_options(), true);
    assert_is_match("a/b/c", "a/**b**/c", default_compile_options(), true);
    assert_is_match(
        "a/b/c/d.js",
        "a/b/c**/*.js",
        default_compile_options(),
        true,
    );
}

#[test]
fn supports_globstars_followed_by_braces() {
    assert_is_match(
        "a/b/c/d/e/z/foo.md",
        "a/**/c/**{,(/z|/x)}/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/e/z/foo.md",
        "a/**{,(/x|/z)}/*.md",
        default_compile_options(),
        true,
    );
}

#[test]
fn supports_globstars_followed_by_braces_with_nested_extglobs() {
    assert_is_match(
        "/x/foo.md",
        "@(/x|/z)/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "/z/foo.md",
        "@(/x|/z)/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/e/z/foo.md",
        "a/**/c/**@(/z|/x)/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/e/z/foo.md",
        "a/**@(/x|/z)/*.md",
        default_compile_options(),
        true,
    );
}

#[test]
fn supports_multiple_globstars_in_one_pattern() {
    assert_is_match(
        "a/b/c/d/e/z/foo.md",
        "a/**/j/**/z/*.md",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "a/b/c/j/e/z/foo.txt",
        "a/**/j/**/z/*.md",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "a/b/c/d/e/j/n/p/o/z/foo.md",
        "a/**/j/**/z/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/e/z/foo.md",
        "a/**/z/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/j/e/z/foo.md",
        "a/**/j/**/z/*.md",
        default_compile_options(),
        true,
    );
}

#[test]
fn matches_file_extensions() {
    assert_match_list(
        &[".md", "a.md", "a/b/c.md", ".txt"],
        "**/*.md",
        default_compile_options(),
        &["a.md", "a/b/c.md"],
    );
    assert_match_list(
        &[".md/.md", ".md", "a/.md", "a/b/.md"],
        "**/.md",
        default_compile_options(),
        &[".md", "a/.md", "a/b/.md"],
    );
    assert_match_list(
        &[".md/.md", ".md/foo/.md", ".md", "a/.md", "a/b/.md"],
        ".md/**/.md",
        default_compile_options(),
        &[".md/.md", ".md/foo/.md"],
    );
}

#[test]
fn respects_trailing_slashes_on_patterns() {
    let fixtures = [
        "a",
        "a/",
        "a/a",
        "a/a/",
        "a/a/a",
        "a/a/a/",
        "a/a/a/a",
        "a/a/a/a/",
        "a/a/a/a/a",
        "a/a/a/a/a/",
        "a/a/b",
        "a/a/b/",
        "a/b",
        "a/b/",
        "a/b/c/.d/e/",
        "a/c",
        "a/c/",
        "a/b",
        "a/x/",
        "b",
        "b/",
        "x/y",
        "x/y/",
        "z/z",
        "z/z/",
    ];

    assert_match_list(
        &fixtures,
        "**/*/a/",
        default_compile_options(),
        &["a/a/", "a/a/a/", "a/a/a/a/", "a/a/a/a/a/"],
    );
    assert_match_list(
        &fixtures,
        "**/*/a/*/",
        default_compile_options(),
        &["a/a/a/", "a/a/a/a/", "a/a/a/a/a/", "a/a/b/"],
    );
    assert_match_list(&fixtures, "**/*/x/", default_compile_options(), &["a/x/"]);
    assert_match_list(
        &fixtures,
        "**/*/*/*/*/",
        default_compile_options(),
        &["a/a/a/a/", "a/a/a/a/a/"],
    );
    assert_match_list(
        &fixtures,
        "**/*/*/*/*/*/",
        default_compile_options(),
        &["a/a/a/a/a/"],
    );
    assert_match_list(
        &fixtures,
        "*a/a/*/",
        default_compile_options(),
        &["a/a/a/", "a/a/b/"],
    );
    assert_match_list(
        &fixtures,
        "**a/a/*/",
        default_compile_options(),
        &["a/a/a/", "a/a/b/"],
    );
    assert_match_list(
        &fixtures,
        "**/a/*/*/",
        default_compile_options(),
        &["a/a/a/", "a/a/a/a/", "a/a/a/a/a/", "a/a/b/"],
    );
    assert_match_list(
        &fixtures,
        "**/a/*/*/*/",
        default_compile_options(),
        &["a/a/a/a/", "a/a/a/a/a/"],
    );
    assert_match_list(
        &fixtures,
        "**/a/*/*/*/*/",
        default_compile_options(),
        &["a/a/a/a/a/"],
    );
    assert_match_list(
        &fixtures,
        "**/a/*/a/",
        default_compile_options(),
        &["a/a/a/", "a/a/a/a/", "a/a/a/a/a/"],
    );
    assert_match_list(
        &fixtures,
        "**/a/*/b/",
        default_compile_options(),
        &["a/a/b/"],
    );
}

#[test]
fn matches_literal_globstars_when_stars_are_escaped() {
    let fixtures = [".md", "**a.md", "**.md", ".md", "**"];
    assert_match_list(
        &fixtures,
        "\\*\\**.md",
        default_compile_options(),
        &["**a.md", "**.md"],
    );
    assert_match_list(
        &fixtures,
        "\\*\\*.md",
        default_compile_options(),
        &["**.md"],
    );
}

#[test]
fn single_dots_not_matched_by_default() {
    // Single dots
    assert_is_match(".a/a", "**", default_compile_options(), false);
    assert_is_match("a/.a", "**", default_compile_options(), false);
    assert_is_match(".a/a", "**/", default_compile_options(), false);
    assert_is_match("a/.a", "**/", default_compile_options(), false);
    assert_is_match(".a/a", "**/**", default_compile_options(), false);
    assert_is_match("a/.a", "**/**", default_compile_options(), false);
    assert_is_match(".a/a", "**/**/*", default_compile_options(), false);
    assert_is_match("a/.a", "**/**/*", default_compile_options(), false);
    assert_is_match(".a/a", "**/**/x", default_compile_options(), false);
    assert_is_match("a/.a", "**/**/x", default_compile_options(), false);
    assert_is_match(".a/a", "**/x", default_compile_options(), false);
    assert_is_match("a/.a", "**/x", default_compile_options(), false);
    assert_is_match(".a/a", "**/x/*", default_compile_options(), false);
    assert_is_match("a/.a", "**/x/*", default_compile_options(), false);
    assert_is_match(".a/a", "**/x/**", default_compile_options(), false);
    assert_is_match("a/.a", "**/x/**", default_compile_options(), false);
    assert_is_match(".a/a", "**/x/*/*", default_compile_options(), false);
    assert_is_match("a/.a", "**/x/*/*", default_compile_options(), false);
    assert_is_match(".a/a", "*/x/**", default_compile_options(), false);
    assert_is_match("a/.a", "*/x/**", default_compile_options(), false);
    assert_is_match(".a/a", "a/**", default_compile_options(), false);
    assert_is_match("a/.a", "a/**", default_compile_options(), false);
    assert_is_match(".a/a", "a/**/*", default_compile_options(), false);
    assert_is_match("a/.a", "a/**/*", default_compile_options(), false);
    assert_is_match(".a/a", "a/**/**/*", default_compile_options(), false);
    assert_is_match("a/.a", "a/**/**/*", default_compile_options(), false);
    assert_is_match(".a/a", "b/**", default_compile_options(), false);
    assert_is_match("a/.a", "b/**", default_compile_options(), false);
}

#[test]
fn double_dots_not_matched_by_default() {
    // Double dots
    assert_is_match("a/../a", "**", default_compile_options(), false);
    assert_is_match("ab/../ac", "**", default_compile_options(), false);
    assert_is_match("../a", "**", default_compile_options(), false);
    assert_is_match("../../b", "**", default_compile_options(), false);
    assert_is_match("../c", "**", default_compile_options(), false);
    assert_is_match("../c/d", "**", default_compile_options(), false);
    assert_is_match("a/../a", "**/", default_compile_options(), false);
    assert_is_match("ab/../ac", "**/", default_compile_options(), false);
    assert_is_match("../a", "**/", default_compile_options(), false);
    assert_is_match("../../b", "**/", default_compile_options(), false);
    assert_is_match("../c", "**/", default_compile_options(), false);
    assert_is_match("../c/d", "**/", default_compile_options(), false);
    assert_is_match("a/../a", "**/**", default_compile_options(), false);
    assert_is_match("ab/../ac", "**/**", default_compile_options(), false);
    assert_is_match("../a", "**/**", default_compile_options(), false);
    assert_is_match("../../b", "**/**", default_compile_options(), false);
    assert_is_match("../c", "**/**", default_compile_options(), false);
    assert_is_match("../c/d", "**/**", default_compile_options(), false);
    assert_is_match("a/../a", "**/**/*", default_compile_options(), false);
    assert_is_match("ab/../ac", "**/**/*", default_compile_options(), false);
    assert_is_match("../a", "**/**/*", default_compile_options(), false);
    assert_is_match("../../b", "**/**/*", default_compile_options(), false);
    assert_is_match("../c", "**/**/*", default_compile_options(), false);
    assert_is_match("../c/d", "**/**/*", default_compile_options(), false);
    assert_is_match("a/../a", "**/**/x", default_compile_options(), false);
    assert_is_match("ab/../ac", "**/**/x", default_compile_options(), false);
    assert_is_match("../a", "**/**/x", default_compile_options(), false);
    assert_is_match("../../b", "**/**/x", default_compile_options(), false);
    assert_is_match("../c", "**/**/x", default_compile_options(), false);
    assert_is_match("../c/d", "**/**/x", default_compile_options(), false);
    assert_is_match("a/../a", "**/x", default_compile_options(), false);
    assert_is_match("ab/../ac", "**/x", default_compile_options(), false);
    assert_is_match("../a", "**/x", default_compile_options(), false);
    assert_is_match("../../b", "**/x", default_compile_options(), false);
    assert_is_match("../c", "**/x", default_compile_options(), false);
    assert_is_match("../c/d", "**/x", default_compile_options(), false);
    assert_is_match("a/../a", "**/x/*", default_compile_options(), false);
    assert_is_match("ab/../ac", "**/x/*", default_compile_options(), false);
    assert_is_match("../a", "**/x/*", default_compile_options(), false);
    assert_is_match("../../b", "**/x/*", default_compile_options(), false);
    assert_is_match("../c", "**/x/*", default_compile_options(), false);
    assert_is_match("../c/d", "**/x/*", default_compile_options(), false);
    assert_is_match("a/../a", "**/x/**", default_compile_options(), false);
    assert_is_match("ab/../ac", "**/x/**", default_compile_options(), false);
    assert_is_match("../a", "**/x/**", default_compile_options(), false);
    assert_is_match("../../b", "**/x/**", default_compile_options(), false);
    assert_is_match("../c", "**/x/**", default_compile_options(), false);
    assert_is_match("../c/d", "**/x/**", default_compile_options(), false);
    assert_is_match("a/../a", "**/x/*/*", default_compile_options(), false);
    assert_is_match("ab/../ac", "**/x/*/*", default_compile_options(), false);
    assert_is_match("../a", "**/x/*/*", default_compile_options(), false);
    assert_is_match("../../b", "**/x/*/*", default_compile_options(), false);
    assert_is_match("../c", "**/x/*/*", default_compile_options(), false);
    assert_is_match("../c/d", "**/x/*/*", default_compile_options(), false);
    assert_is_match("a/../a", "*/x/**", default_compile_options(), false);
    assert_is_match("ab/../ac", "*/x/**", default_compile_options(), false);
    assert_is_match("../a", "*/x/**", default_compile_options(), false);
    assert_is_match("../../b", "*/x/**", default_compile_options(), false);
    assert_is_match("../c", "*/x/**", default_compile_options(), false);
    assert_is_match("../c/d", "*/x/**", default_compile_options(), false);
    assert_is_match("a/../a", "a/**", default_compile_options(), false);
    assert_is_match("ab/../ac", "a/**", default_compile_options(), false);
    assert_is_match("../a", "a/**", default_compile_options(), false);
    assert_is_match("../../b", "a/**", default_compile_options(), false);
    assert_is_match("../c", "a/**", default_compile_options(), false);
    assert_is_match("../c/d", "a/**", default_compile_options(), false);
    assert_is_match("a/../a", "a/**/*", default_compile_options(), false);
    assert_is_match("ab/../ac", "a/**/*", default_compile_options(), false);
    assert_is_match("../a", "a/**/*", default_compile_options(), false);
    assert_is_match("../../b", "a/**/*", default_compile_options(), false);
    assert_is_match("../c", "a/**/*", default_compile_options(), false);
    assert_is_match("../c/d", "a/**/*", default_compile_options(), false);
    assert_is_match("a/../a", "a/**/**/*", default_compile_options(), false);
    assert_is_match("ab/../ac", "a/**/**/*", default_compile_options(), false);
    assert_is_match("../a", "a/**/**/*", default_compile_options(), false);
    assert_is_match("../../b", "a/**/**/*", default_compile_options(), false);
    assert_is_match("../c", "a/**/**/*", default_compile_options(), false);
    assert_is_match("../c/d", "a/**/**/*", default_compile_options(), false);
    assert_is_match("a/../a", "b/**", default_compile_options(), false);
    assert_is_match("ab/../ac", "b/**", default_compile_options(), false);
    assert_is_match("../a", "b/**", default_compile_options(), false);
    assert_is_match("../../b", "b/**", default_compile_options(), false);
    assert_is_match("../c", "b/**", default_compile_options(), false);
    assert_is_match("../c/d", "b/**", default_compile_options(), false);
}

#[test]
fn matches_basic_globstar_patterns() {
    // should match
    assert_is_match("a", "**/a/*", default_compile_options(), false);
    assert_is_match("a", "**/a/*/*", default_compile_options(), false);
    assert_is_match("a", "*/a/**", default_compile_options(), false);
    assert_is_match("a", "a/**/*", default_compile_options(), false);
    assert_is_match("a", "a/**/**/*", default_compile_options(), false);
    assert_is_match("a/b", "**/", default_compile_options(), false);
    assert_is_match("a/b", "**/b/*", default_compile_options(), false);
    assert_is_match("a/b", "**/b/*/*", default_compile_options(), false);
    assert_is_match("a/b", "b/**", default_compile_options(), false);
    assert_is_match("a/b/c", "**/", default_compile_options(), false);
    assert_is_match("a/b/c", "**/**/b", default_compile_options(), false);
    assert_is_match("a/b/c", "**/b", default_compile_options(), false);
    assert_is_match("a/b/c", "**/b/*/*", default_compile_options(), false);
    assert_is_match("a/b/c", "b/**", default_compile_options(), false);
    assert_is_match("a/b/c/d", "**/", default_compile_options(), false);
    assert_is_match("a/b/c/d", "**/d/*", default_compile_options(), false);
    assert_is_match("a/b/c/d", "b/**", default_compile_options(), false);

    assert_is_match("a", "**", default_compile_options(), true);
    assert_is_match("a", "**/**", default_compile_options(), true);
    assert_is_match("a", "**/**/*", default_compile_options(), true);
    assert_is_match("a", "**/**/a", default_compile_options(), true);
    assert_is_match("a", "**/a", default_compile_options(), true);
    assert_is_match("a", "**/a/**", default_compile_options(), true);
    assert_is_match("a", "a/**", default_compile_options(), true);
    assert_is_match("a/b", "**", default_compile_options(), true);
    assert_is_match("a/b", "**/**", default_compile_options(), true);
    assert_is_match("a/b", "**/**/*", default_compile_options(), true);
    assert_is_match("a/b", "**/**/b", default_compile_options(), true);
    assert_is_match("a/b", "**/b", default_compile_options(), true);
    assert_is_match("a/b", "**/b/**", default_compile_options(), true);
    assert_is_match("a/b", "*/b/**", default_compile_options(), true);
    assert_is_match("a/b", "a/**", default_compile_options(), true);
    assert_is_match("a/b", "a/**/*", default_compile_options(), true);
    assert_is_match("a/b", "a/**/**/*", default_compile_options(), true);
    assert_is_match("a/b/c", "**", default_compile_options(), true);
    assert_is_match("a/b/c", "**/**", default_compile_options(), true);
    assert_is_match("a/b/c", "**/**/*", default_compile_options(), true);
    assert_is_match("a/b/c", "**/b/*", default_compile_options(), true);
    assert_is_match("a/b/c", "**/b/**", default_compile_options(), true);
    assert_is_match("a/b/c", "*/b/**", default_compile_options(), true);
    assert_is_match("a/b/c", "a/**", default_compile_options(), true);
    assert_is_match("a/b/c", "a/**/*", default_compile_options(), true);
    assert_is_match("a/b/c", "a/**/**/*", default_compile_options(), true);
    assert_is_match("a/b/c/d", "**", default_compile_options(), true);
    assert_is_match("a/b/c/d", "**/**", default_compile_options(), true);
    assert_is_match("a/b/c/d", "**/**/*", default_compile_options(), true);
    assert_is_match("a/b/c/d", "**/**/d", default_compile_options(), true);
    assert_is_match("a/b/c/d", "**/b/**", default_compile_options(), true);
    assert_is_match("a/b/c/d", "**/b/*/*", default_compile_options(), true);
    assert_is_match("a/b/c/d", "**/d", default_compile_options(), true);
    assert_is_match("a/b/c/d", "*/b/**", default_compile_options(), true);
    assert_is_match("a/b/c/d", "a/**", default_compile_options(), true);
    assert_is_match("a/b/c/d", "a/**/*", default_compile_options(), true);
    assert_is_match("a/b/c/d", "a/**/**/*", default_compile_options(), true);
}

#[test]
fn matches_nested_directories() {
    assert_is_match("a/b", "*/*", default_compile_options(), true);
    assert_is_match(
        "a/b/c/xyz.md",
        "a/b/c/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/bb.bb/c/xyz.md",
        "a/*/c/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/bb/c/xyz.md",
        "a/*/c/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/bbbb/c/xyz.md",
        "a/*/c/*.md",
        default_compile_options(),
        true,
    );

    assert_is_match("a/b/c", "**/*", default_compile_options(), true);
    assert_is_match("a/b/c", "**/**", default_compile_options(), true);
    assert_is_match("a/b/c", "*/**", default_compile_options(), true);
    assert_is_match(
        "a/b/c/d/e/j/n/p/o/z/c.md",
        "a/**/j/**/z/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/e/z/c.md",
        "a/**/z/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/bb.bb/aa/b.b/aa/c/xyz.md",
        "a/**/c/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/bb.bb/aa/bb/aa/c/xyz.md",
        "a/**/c/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/j/e/z/c.txt",
        "a/**/j/**/z/*.md",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "a/b/c/xyz.md",
        "a/b/**/c{d,e}/**/xyz.md",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "a/b/d/xyz.md",
        "a/b/**/c{d,e}/**/xyz.md",
        default_compile_options(),
        false,
    );
    assert_is_match("a/b", "a/**/", default_compile_options(), false);
    assert_is_match("a/b/.js/c.txt", "**/*", default_compile_options(), false);
    assert_is_match("a/b/c/d", "a/**/", default_compile_options(), false);
    assert_is_match("a/bb", "a/**/", default_compile_options(), false);
    assert_is_match("a/cb", "a/**/", default_compile_options(), false);
    assert_is_match("/a/b", "/**", default_compile_options(), true);
    assert_is_match("a.b", "**/*", default_compile_options(), true);
    assert_is_match("a.js", "**/*", default_compile_options(), true);
    assert_is_match("a.js", "**/*.js", default_compile_options(), true);
    assert_is_match("a/", "a/**/", default_compile_options(), true);
    assert_is_match("a/a.js", "**/*.js", default_compile_options(), true);
    assert_is_match("a/a/b.js", "**/*.js", default_compile_options(), true);
    assert_is_match("a/b", "a/**/b", default_compile_options(), true);
    assert_is_match("a/b", "a/**b", default_compile_options(), true);
    assert_is_match("a/b.md", "**/*.md", default_compile_options(), true);
    assert_is_match("a/b/c.js", "**/*", default_compile_options(), true);
    assert_is_match("a/b/c.txt", "**/*", default_compile_options(), true);
    assert_is_match("a/b/c/d/", "a/**/", default_compile_options(), true);
    assert_is_match("a/b/c/d/a.js", "**/*", default_compile_options(), true);
    assert_is_match("a/b/c/z.js", "a/b/**/*.js", default_compile_options(), true);
    assert_is_match("a/b/z.js", "a/b/**/*.js", default_compile_options(), true);
    assert_is_match("ab", "**/*", default_compile_options(), true);
    assert_is_match("ab/c", "**/*", default_compile_options(), true);
    assert_is_match("ab/c/d", "**/*", default_compile_options(), true);
    assert_is_match("abc.js", "**/*", default_compile_options(), true);
}

#[test]
fn does_not_match_dotfiles_by_default() {
    assert_is_match("a/.b", "a/**/z/*.md", default_compile_options(), false);
    assert_is_match("a/b/z/.a", "a/**/z/*.a", default_compile_options(), false);
    assert_is_match("a/b/z/.a", "a/*/z/*.a", default_compile_options(), false);
    assert_is_match("a/b/z/.a", "b/a", default_compile_options(), false);
    assert_is_match(
        "a/foo/z/.b",
        "a/**/z/*.md",
        default_compile_options(),
        false,
    );
}

#[test]
fn matches_leading_dots_when_defined_in_pattern() {
    let fixtures = [".gitignore", "a/b/z/.dotfile", "a/b/z/.dotfile.md"];

    assert_is_match(
        ".gitignore",
        "a/**/z/*.md",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "a/b/z/.dotfile",
        "a/**/z/*.md",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "a/b/z/.dotfile.md",
        "**/c/.*.md",
        default_compile_options(),
        false,
    );
    assert_is_match("a/.b", "a/.*", default_compile_options(), true);
    assert_is_match("a/b/z/.a", "a/*/z/.a", default_compile_options(), true);
    assert_is_match(
        "a/b/z/.dotfile.md",
        "**/.*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/z/.dotfile.md",
        "a/**/z/.*.md",
        default_compile_options(),
        true,
    );

    assert_match_list(
        &[".md", "a.md", "a/b/c.md", ".txt"],
        "**/*.md",
        default_compile_options(),
        &["a.md", "a/b/c.md"],
    );
    assert_match_list(
        &[".md/.md", ".md", "a/.md", "a/b/.md"],
        "**/.md",
        default_compile_options(),
        &[".md", "a/.md", "a/b/.md"],
    );
    assert_match_list(
        &[".md/.md", ".md/foo/.md", ".md", "a/.md", "a/b/.md"],
        ".md/**/.md",
        default_compile_options(),
        &[".md/.md", ".md/foo/.md"],
    );
    assert_match_list(
        &fixtures,
        "a/**/z/.*.md",
        default_compile_options(),
        &["a/b/z/.dotfile.md"],
    );
}

#[test]
fn matches_globstar_basic_patterns() {
    // micromatch/#24
    assert_is_match(
        "foo/bar/baz/one/image.png",
        "foo/bar/**/one/**/*.*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "foo/bar/baz/one/two/image.png",
        "foo/bar/**/one/**/*.*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "foo/bar/baz/one/two/three/image.png",
        "foo/bar/**/one/**/*.*",
        default_compile_options(),
        true,
    );
    assert_is_match("a/b/c/d/", "a/b/**/f", default_compile_options(), false);
    assert_is_match("a", "a/**", default_compile_options(), true);
    assert_is_match("a", "**", default_compile_options(), true);
    assert_is_match("a", "a{,/**}", default_compile_options(), true);
    assert_is_match("a/", "**", default_compile_options(), true);
    assert_is_match("a/", "a/**", default_compile_options(), true);
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

    assert_is_match(
        "a/b-c/z.js",
        "a/b-*/**/z.js",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b-c/d/e/z.js",
        "a/b-*/**/z.js",
        default_compile_options(),
        true,
    );
}

#[test]
fn matches_globstars() {
    assert_is_match("a/b/c/d.js", "**/*.js", default_compile_options(), true);
    assert_is_match("a/b/c.js", "**/*.js", default_compile_options(), true);
    assert_is_match("a/b.js", "**/*.js", default_compile_options(), true);
    assert_is_match(
        "a/b/c/d/e/f.js",
        "a/b/**/*.js",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/e.js",
        "a/b/**/*.js",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d.js",
        "a/b/c/**/*.js",
        default_compile_options(),
        true,
    );
    assert_is_match("a/b/c/d.js", "a/b/**/*.js", default_compile_options(), true);
    assert_is_match("a/b/d.js", "a/b/**/*.js", default_compile_options(), true);

    assert_is_match("a/d.js", "a/b/**/*.js", default_compile_options(), false);
    assert_is_match("d.js", "a/b/**/*.js", default_compile_options(), false);
}

#[test]
fn supports_globstars_basic() {
    assert_is_match("a", "a/**/*", default_compile_options(), false);
    assert_is_match("a", "a/**/**/*", default_compile_options(), false);
    assert_is_match("a", "a/**/**/**/*", default_compile_options(), false);
    assert_is_match("a/", "**/a", default_compile_options(), false);
    assert_is_match("a/", "a/**/*", default_compile_options(), false);
    assert_is_match("a/", "a/**/**/*", default_compile_options(), false);
    assert_is_match("a/", "a/**/**/**/*", default_compile_options(), false);
    assert_is_match("a/b", "**/a", default_compile_options(), false);
    assert_is_match(
        "a/b/c/j/e/z/c.txt",
        "a/**/j/**/z/*.md",
        default_compile_options(),
        false,
    );
    assert_is_match("a/bb", "a/**/b", default_compile_options(), false);
    assert_is_match("a/c", "**/a", default_compile_options(), false);
    assert_is_match("a/b", "**/a", default_compile_options(), false);
    assert_is_match("a/x/y", "**/a", default_compile_options(), false);
    assert_is_match("a/b/c/d", "**/a", default_compile_options(), false);

    assert_is_match("a", "**", default_compile_options(), true);
    assert_is_match("a", "**/a", default_compile_options(), true);
    assert_is_match("a", "a/**", default_compile_options(), true);
    assert_is_match("a/", "**", default_compile_options(), true);
    assert_is_match("a/", "**/a/**", default_compile_options(), true);
    assert_is_match("a/", "a/**", default_compile_options(), true);
    assert_is_match("a/", "a/**/**", default_compile_options(), true);
    assert_is_match("a/a", "**/a", default_compile_options(), true);
    assert_is_match("a/b", "**", default_compile_options(), true);
    assert_is_match("a/b", "*/*", default_compile_options(), true);
    assert_is_match("a/b", "a/**", default_compile_options(), true);
    assert_is_match("a/b", "a/**/*", default_compile_options(), true);
    assert_is_match("a/b", "a/**/**/*", default_compile_options(), true);
    assert_is_match("a/b", "a/**/**/**/*", default_compile_options(), true);
    assert_is_match("a/b", "a/**/b", default_compile_options(), true);
    assert_is_match("a/b/c", "**", default_compile_options(), true);
    assert_is_match("a/b/c", "**/*", default_compile_options(), true);
    assert_is_match("a/b/c", "**/**", default_compile_options(), true);
    assert_is_match("a/b/c", "*/**", default_compile_options(), true);
    assert_is_match("a/b/c", "a/**", default_compile_options(), true);
    assert_is_match("a/b/c", "a/**/*", default_compile_options(), true);
    assert_is_match("a/b/c", "a/**/**/*", default_compile_options(), true);
    assert_is_match("a/b/c", "a/**/**/**/*", default_compile_options(), true);
    assert_is_match("a/b/c/d", "**", default_compile_options(), true);
    assert_is_match("a/b/c/d", "a/**", default_compile_options(), true);
    assert_is_match("a/b/c/d", "a/**/*", default_compile_options(), true);
    assert_is_match("a/b/c/d", "a/**/**/*", default_compile_options(), true);
    assert_is_match("a/b/c/d", "a/**/**/**/*", default_compile_options(), true);
    assert_is_match(
        "a/b/c/d.e",
        "a/b/**/c/**/*.*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/e/f/g.md",
        "a/**/f/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/d/e/f/g/h/i/j/k/l.md",
        "a/**/f/**/k/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/b/c/def.md",
        "a/b/c/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/bb.bb/c/ddd.md",
        "a/*/c/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/bb.bb/cc/d.d/ee/f/ggg.md",
        "a/**/f/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/bb.bb/cc/dd/ee/f/ggg.md",
        "a/**/f/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/bb/c/ddd.md",
        "a/*/c/*.md",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "a/bbbb/c/ddd.md",
        "a/*/c/*.md",
        default_compile_options(),
        true,
    );
}
