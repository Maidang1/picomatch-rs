mod support;

use picomatch_rs::CompileOptions;

use support::{assert_is_match, assert_match_list, default_compile_options};

#[test]
fn matches_question_marks_with_question_marks() {
    assert_match_list(&["?", "??", "???"], "?", default_compile_options(), &["?"]);
    assert_match_list(
        &["?", "??", "???"],
        "??",
        default_compile_options(),
        &["??"],
    );
    assert_match_list(
        &["?", "??", "???"],
        "???",
        default_compile_options(),
        &["???"],
    );
}

#[test]
fn matches_question_marks_and_stars_with_question_marks_and_stars() {
    assert_match_list(
        &["?", "??", "???"],
        "?*",
        default_compile_options(),
        &["?", "??", "???"],
    );
    assert_match_list(
        &["?", "??", "???"],
        "*?",
        default_compile_options(),
        &["?", "??", "???"],
    );
    assert_match_list(
        &["?", "??", "???"],
        "?*?",
        default_compile_options(),
        &["??", "???"],
    );
    assert_match_list(
        &["?*", "?*?", "?*?*?"],
        "?*",
        default_compile_options(),
        &["?*", "?*?", "?*?*?"],
    );
    assert_match_list(
        &["?*", "?*?", "?*?*?"],
        "*?",
        default_compile_options(),
        &["?*", "?*?", "?*?*?"],
    );
    assert_match_list(
        &["?*", "?*?", "?*?*?"],
        "?*?",
        default_compile_options(),
        &["?*", "?*?", "?*?*?"],
    );
}

#[test]
fn supports_consecutive_stars_and_question_marks() {
    assert_match_list(
        &["aaa", "aac", "abc"],
        "a*?c",
        default_compile_options(),
        &["aac", "abc"],
    );
    assert_match_list(
        &["abc", "abb", "acc"],
        "a**?c",
        default_compile_options(),
        &["abc", "acc"],
    );
    assert_match_list(
        &["abc", "aaaabbbbbbccccc"],
        "a*****?c",
        default_compile_options(),
        &["abc", "aaaabbbbbbccccc"],
    );
    assert_match_list(
        &["a", "ab", "abc", "abcd"],
        "*****?",
        default_compile_options(),
        &["a", "ab", "abc", "abcd"],
    );
    assert_match_list(
        &["a", "ab", "abc", "abcd"],
        "*****??",
        default_compile_options(),
        &["ab", "abc", "abcd"],
    );
    assert_match_list(
        &["a", "ab", "abc", "abcd"],
        "?*****??",
        default_compile_options(),
        &["abc", "abcd"],
    );
    assert_match_list(
        &["abc", "abb", "zzz"],
        "?*****?c",
        default_compile_options(),
        &["abc"],
    );
    assert_match_list(
        &["abc", "bbb", "zzz"],
        "?***?****?",
        default_compile_options(),
        &["abc", "bbb", "zzz"],
    );
    assert_match_list(
        &["abc", "bbb", "zzz"],
        "?***?****c",
        default_compile_options(),
        &["abc"],
    );
    assert_match_list(&["abc"], "*******?", default_compile_options(), &["abc"]);
    assert_match_list(&["abc"], "*******c", default_compile_options(), &["abc"]);
    assert_match_list(&["abc"], "?***?****", default_compile_options(), &["abc"]);
    assert_match_list(
        &["abcdecdhjk"],
        "a****c**?**??*****",
        default_compile_options(),
        &["abcdecdhjk"],
    );
    assert_match_list(
        &["abcdecdhjk"],
        "a**?**cd**?**??***k",
        default_compile_options(),
        &["abcdecdhjk"],
    );
    assert_match_list(
        &["abcdecdhjk"],
        "a**?**cd**?**??***k**",
        default_compile_options(),
        &["abcdecdhjk"],
    );
    assert_match_list(
        &["abcdecdhjk"],
        "a**?**cd**?**??k",
        default_compile_options(),
        &["abcdecdhjk"],
    );
    assert_match_list(
        &["abcdecdhjk"],
        "a**?**cd**?**??k***",
        default_compile_options(),
        &["abcdecdhjk"],
    );
    assert_match_list(
        &["abcdecdhjk"],
        "a*cd**?**??k",
        default_compile_options(),
        &["abcdecdhjk"],
    );
}

#[test]
fn matches_backslashes_with_question_marks_when_not_on_windows() {
    if cfg!(windows) {
        return;
    }

    assert_is_match(r"aaa\\bbb", "aaa?bbb", default_compile_options(), false);
    assert_is_match(r"aaa\\bbb", "aaa??bbb", default_compile_options(), true);
    assert_is_match(r"aaa\bbb", "aaa?bbb", default_compile_options(), true);
}

#[test]
fn matches_one_character_per_question_mark() {
    let fixtures = ["a", "aa", "ab", "aaa", "abcdefg"];
    assert_match_list(&fixtures, "?", default_compile_options(), &["a"]);
    assert_match_list(&fixtures, "??", default_compile_options(), &["aa", "ab"]);
    assert_match_list(&fixtures, "???", default_compile_options(), &["aaa"]);
    assert_match_list(
        &["a/", "/a/", "/a/b/", "/a/b/c/", "/a/b/c/d/"],
        "??",
        default_compile_options(),
        &[],
    );
    assert_match_list(
        &["a/b/c.md"],
        "a/?/c.md",
        default_compile_options(),
        &["a/b/c.md"],
    );
    assert_match_list(&["a/bb/c.md"], "a/?/c.md", default_compile_options(), &[]);
    assert_match_list(
        &["a/bb/c.md"],
        "a/??/c.md",
        default_compile_options(),
        &["a/bb/c.md"],
    );
    assert_match_list(&["a/bbb/c.md"], "a/??/c.md", default_compile_options(), &[]);
    assert_match_list(
        &["a/bbb/c.md"],
        "a/???/c.md",
        default_compile_options(),
        &["a/bbb/c.md"],
    );
    assert_match_list(
        &["a/bbbb/c.md"],
        "a/????/c.md",
        default_compile_options(),
        &["a/bbbb/c.md"],
    );
}

#[test]
fn does_not_match_slashes_with_question_marks() {
    let fixtures = ["//", "a/", "/a", "/a/", "aa", "/aa", "a/a", "aaa", "/aaa"];
    assert_match_list(&fixtures, "/?", default_compile_options(), &["/a"]);
    assert_match_list(&fixtures, "/??", default_compile_options(), &["/aa"]);
    assert_match_list(&fixtures, "/???", default_compile_options(), &["/aaa"]);
    assert_match_list(&fixtures, "/?/", default_compile_options(), &["/a/"]);
    assert_match_list(&fixtures, "??", default_compile_options(), &["aa"]);
    assert_match_list(&fixtures, "?/?", default_compile_options(), &["a/a"]);
    assert_match_list(&fixtures, "???", default_compile_options(), &["aaa"]);
    assert_match_list(&fixtures, "a?a", default_compile_options(), &["aaa"]);
    assert_match_list(&fixtures, "aa?", default_compile_options(), &["aaa"]);
    assert_match_list(&fixtures, "?aa", default_compile_options(), &["aaa"]);
}

#[test]
fn supports_question_marks_and_stars_between_slashes() {
    assert_match_list(
        &["a/b.bb/c/d/efgh.ijk/e"],
        "a/*/?/**/e",
        default_compile_options(),
        &["a/b.bb/c/d/efgh.ijk/e"],
    );
    assert_match_list(
        &["a/b/c/d/e"],
        "a/?/c/?/*/e",
        default_compile_options(),
        &[],
    );
    assert_match_list(
        &["a/b/c/d/e/e"],
        "a/?/c/?/*/e",
        default_compile_options(),
        &["a/b/c/d/e/e"],
    );
    assert_match_list(
        &["a/b/c/d/efgh.ijk/e"],
        "a/*/?/**/e",
        default_compile_options(),
        &["a/b/c/d/efgh.ijk/e"],
    );
    assert_match_list(
        &["a/b/c/d/efghijk/e"],
        "a/*/?/**/e",
        default_compile_options(),
        &["a/b/c/d/efghijk/e"],
    );
    assert_match_list(
        &["a/b/c/d/efghijk/e"],
        "a/?/**/e",
        default_compile_options(),
        &["a/b/c/d/efghijk/e"],
    );
    assert_match_list(
        &["a/b/c/d/efghijk/e"],
        "a/?/c/?/*/e",
        default_compile_options(),
        &["a/b/c/d/efghijk/e"],
    );
    assert_match_list(&["a/bb/e"], "a/?/**/e", default_compile_options(), &[]);
    assert_match_list(&["a/bb/e"], "a/?/e", default_compile_options(), &[]);
    assert_match_list(
        &["a/bbb/c/d/efgh.ijk/e"],
        "a/*/?/**/e",
        default_compile_options(),
        &["a/bbb/c/d/efgh.ijk/e"],
    );
}

#[test]
fn matches_no_more_than_one_character_between_slashes() {
    let fixtures = ["a/a", "a/a/a", "a/aa/a", "a/aaa/a", "a/aaaa/a", "a/aaaaa/a"];
    assert_match_list(&fixtures, "?/?", default_compile_options(), &["a/a"]);
    assert_match_list(
        &fixtures,
        "?/???/?",
        default_compile_options(),
        &["a/aaa/a"],
    );
    assert_match_list(
        &fixtures,
        "?/????/?",
        default_compile_options(),
        &["a/aaaa/a"],
    );
    assert_match_list(
        &fixtures,
        "?/?????/?",
        default_compile_options(),
        &["a/aaaaa/a"],
    );
    assert_match_list(&fixtures, "a/?", default_compile_options(), &["a/a"]);
    assert_match_list(&fixtures, "a/?/a", default_compile_options(), &["a/a/a"]);
    assert_match_list(&fixtures, "a/??/a", default_compile_options(), &["a/aa/a"]);
    assert_match_list(
        &fixtures,
        "a/???/a",
        default_compile_options(),
        &["a/aaa/a"],
    );
    assert_match_list(
        &fixtures,
        "a/????/a",
        default_compile_options(),
        &["a/aaaa/a"],
    );
    assert_match_list(
        &fixtures,
        "a/????a/a",
        default_compile_options(),
        &["a/aaaaa/a"],
    );
}

#[test]
fn does_not_match_non_leading_dots_with_question_marks() {
    let fixtures = [
        ".", ".a", "a", "aa", "a.a", "aa.a", "aaa", "aaa.a", "aaaa.a", "aaaaa",
    ];
    assert_match_list(&fixtures, "?", default_compile_options(), &["a"]);
    assert_match_list(&fixtures, ".?", default_compile_options(), &[".a"]);
    assert_match_list(&fixtures, "?a", default_compile_options(), &["aa"]);
    assert_match_list(&fixtures, "??", default_compile_options(), &["aa"]);
    assert_match_list(&fixtures, "?a?", default_compile_options(), &["aaa"]);
    assert_match_list(
        &fixtures,
        "aaa?a",
        default_compile_options(),
        &["aaa.a", "aaaaa"],
    );
    assert_match_list(
        &fixtures,
        "a?a?a",
        default_compile_options(),
        &["aaa.a", "aaaaa"],
    );
    assert_match_list(
        &fixtures,
        "a???a",
        default_compile_options(),
        &["aaa.a", "aaaaa"],
    );
    assert_match_list(&fixtures, "a?????", default_compile_options(), &["aaaa.a"]);
}

#[test]
fn matches_non_leading_dots_with_question_marks_when_dot_is_true() {
    let fixtures = [
        ".", ".a", "a", "aa", "a.a", "aa.a", ".aa", "aaa.a", "aaaa.a", "aaaaa",
    ];
    let options = CompileOptions {
        dot: true,
        ..CompileOptions::default()
    };

    assert_match_list(&fixtures, "?", options.clone(), &[".", "a"]);
    assert_match_list(&fixtures, ".?", options.clone(), &[".a"]);
    assert_match_list(&fixtures, "?a", options.clone(), &[".a", "aa"]);
    assert_match_list(&fixtures, "??", options.clone(), &[".a", "aa"]);
    assert_match_list(&fixtures, "?a?", options, &[".aa"]);
}
