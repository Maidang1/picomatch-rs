mod support;

use support::{assert_is_match, default_compile_options};

#[test]
fn stars_following_parens() {
    let opts = default_compile_options();

    // should support stars following parens
    assert_is_match("a", "(a)*", opts.clone(), true);
    assert_is_match("az", "(a)*", opts.clone(), true);
    assert_is_match("zz", "(a)*", opts.clone(), false);
    assert_is_match("ab", "(a|b)*", opts.clone(), true);
    assert_is_match("abc", "(a|b)*", opts.clone(), true);
    assert_is_match("aa", "(a)*", opts.clone(), true);
    assert_is_match("aaab", "(a|b)*", opts.clone(), true);
    assert_is_match("aaabbb", "(a|b)*", opts.clone(), true);
}

#[test]
fn slashes_with_single_stars() {
    let opts = default_compile_options();

    // should not match slashes with single stars
    assert_is_match("a/b", "(a)*", opts.clone(), false);
    assert_is_match("a/b", "(a|b)*", opts.clone(), false);
}
