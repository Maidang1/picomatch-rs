mod support;

use support::{assert_is_match, default_compile_options};

#[test]
fn options_noextglob() {
    let mut opts = default_compile_options();
    opts.noextglob = true;

    // should disable extglob support when options.noextglob is true
    // In this case, +(z) is treated literally, but + is escaped in compile_body if it's literal
    // Wait, let's see how it compiles. +(z) -> + followed by (z) as group or literal?
    // In picomatch-rs, when noextglob is true, compile_extglob returns None or
    // it shouldn't even be called.

    assert_is_match("a+z", "a+(z)", opts.clone(), true);
    assert_is_match("az", "a+(z)", opts.clone(), false);
    assert_is_match("azz", "a+(z)", opts.clone(), false);
    assert_is_match("azzz", "a+(z)", opts.clone(), false);
}

#[test]
fn options_noext_alias() {
    // Note: Rust CompileOptions doesn't have a 'noext' field (it's a JS-side alias).
    // So we just test the functional behavior via noextglob.
    let mut opts = default_compile_options();
    opts.noextglob = true;

    assert_is_match("a+z", "a+(z)", opts.clone(), true);
    assert_is_match("az", "a+(z)", opts.clone(), false);
    assert_is_match("azz", "a+(z)", opts.clone(), false);
    assert_is_match("azzz", "a+(z)", opts.clone(), false);
}
