mod support;

use support::{assert_is_match, default_compile_options};

#[test]
fn options_noglobstar() {
    let mut opts_off = default_compile_options();
    opts_off.noglobstar = false;

    let mut opts_on = default_compile_options();
    opts_on.noglobstar = true;

    // should disable globstar support when options.noglobstar is true
    assert_is_match("a/b/c", "**", opts_off.clone(), true);
    assert_is_match("a/b/c", "**", opts_on.clone(), false);

    assert_is_match("a/b/c", "a/**", opts_off.clone(), true);
    assert_is_match("a/b/c", "a/**", opts_on.clone(), false);
}
