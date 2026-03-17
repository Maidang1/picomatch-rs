mod support;

use support::{assert_is_match, default_compile_options};

#[test]
fn should_support_stars_following_brackets() {
    assert_is_match("a", "[a]*", default_compile_options(), true);
    assert_is_match("aa", "[a]*", default_compile_options(), true);
    assert_is_match("aaa", "[a]*", default_compile_options(), true);
    assert_is_match("az", "[a-z]*", default_compile_options(), true);
    assert_is_match("zzz", "[a-z]*", default_compile_options(), true);
}

#[test]
fn should_match_slashes_defined_in_brackets() {
    assert_is_match("foo/bar", "foo[/]bar", default_compile_options(), true);
    assert_is_match("foo/bar/", "foo[/]bar[/]", default_compile_options(), true);
    assert_is_match(
        "foo/bar/baz",
        "foo[/]bar[/]baz",
        default_compile_options(),
        true,
    );
}

#[test]
fn should_not_match_slashes_following_brackets() {
    assert_is_match("a/b", "[a]*", default_compile_options(), false);
}
