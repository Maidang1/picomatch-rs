mod support;

use support::{assert_is_match, default_compile_options};

#[test]
fn should_use_posix_paths_only_by_default() {
    let options = default_compile_options();
    assert_is_match("a/b", "a/**", options.clone(), true);
    assert_is_match("a\\b", "a/**", options, false);
}

#[test]
fn should_still_be_manually_configurable_to_accept_non_posix_paths() {
    let mut options = default_compile_options();
    options.windows = true;
    assert_is_match("a\\b", "a/**", options.clone(), true);
    assert_is_match("a/b", "a/**", options, true);
}
