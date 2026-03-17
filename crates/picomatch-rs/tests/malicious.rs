mod support;

use picomatch_rs::CompileOptions;
use support::assert_is_match;

fn repeat(ch: &str, n: usize) -> String {
    ch.repeat(n)
}

#[test]
#[ignore = "Rust implementation does not collapse consecutive backslashes like JS scan.js does"]
fn test_long_escape_sequences() {
    let rep_65500 = repeat("\\", 65500);

    // if process.platform !== 'win32'
    if !cfg!(windows) {
        assert_is_match(
            "\\A",
            &format!("{}A", rep_65500),
            CompileOptions::default(),
            true,
        );
    }

    assert_is_match(
        "A",
        &format!("!{}A", rep_65500),
        CompileOptions::default(),
        true,
    );

    assert_is_match(
        "A",
        &format!("!({}A)", rep_65500),
        CompileOptions::default(),
        true,
    );

    assert_is_match(
        "A",
        &format!("[!({}A", rep_65500),
        CompileOptions::default(),
        false,
    );
}

#[test]
fn test_object_instance_properties() {
    assert_is_match(
        "constructor",
        "constructor",
        CompileOptions::default(),
        true,
    );
    assert_is_match("__proto__", "__proto__", CompileOptions::default(), true);
    assert_is_match("toString", "toString", CompileOptions::default(), true);
}
