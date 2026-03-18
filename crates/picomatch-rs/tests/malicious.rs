mod support;

use picomatch_rs::{is_match, CompileOptions, MatchError};
use support::assert_is_match;

fn repeat(ch: &str, n: usize) -> String {
    ch.repeat(n)
}

#[test]
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

    let invalid_pattern = format!("[!({}A", rep_65500);
    let invalid_result = is_match("A", &invalid_pattern, &CompileOptions::default());
    assert!(
        matches!(
            invalid_result,
            Ok(false) | Err(MatchError::UnsupportedPattern(_))
        ),
        "expected invalid pattern to fail closed, got: {invalid_result:?}"
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
