//! Migration of `test/options.expandRange.js` to Rust.
//!
//! # Notes on `expandRange` option
//!
//! The JS `expandRange` option accepts a user-supplied callback `(a, b) => string`
//! that overrides how brace ranges like `{a..c}` or `{1..100}` are compiled to
//! a regex fragment. Rust does not expose such a callback because the native
//! compiler already has built-in alpha/numeric range expansion.
//!
//! Migration strategy:
//! - Test 1 & 2: `{a..c}` with custom `(a, b) => \`([${a}-${b}])\``
//!   produces the fragment `([a-c])`. The native Rust expansion produces
//!   `[a-c]`, which is semantically equivalent (matches the same characters).
//!   → Migrated directly using the default `CompileOptions`.
//!
//! - Test 3: `{1..100}` with `fill-range { toRegex: true }` produces a
//!   large alternation regex covering 1–100. The native Rust expansion
//!   emits `(?:1|2|...|100)`, which is semantically equivalent.
//!   → Migrated directly using the default `CompileOptions`.

mod support;

use support::{assert_is_match, default_compile_options};

/// Corresponds to:
///   it('should support a custom function for expanding ranges in brace patterns', ...)
///
/// JS (original):
///   assert(isMatch('a/c', 'a/{a..c}', { expandRange: (a, b) => `([${a}-${b}])` }));
///   assert(!isMatch('a/z', 'a/{a..c}', { expandRange: (a, b) => `([${a}-${b}])` }));
///   assert(isMatch('a/99', 'a/{1..100}', { expandRange(a, b) { return `(${fill(a, b, { toRegex: true })})`; } }));
///
/// Note: `expandRange` is a JS-only callback option. The Rust native compiler
/// has equivalent built-in alpha and numeric range expansion, so the tests are
/// migrated without the callback — the matching behaviour is identical.
#[test]
fn should_support_expanding_ranges_in_brace_patterns() {
    let opts = default_compile_options();

    // {a..c} natively expands to [a-c] — matches 'a', 'b', 'c'
    assert_is_match("a/c", "a/{a..c}", opts.clone(), true);

    // 'z' is outside [a-c], so should not match
    assert_is_match("a/z", "a/{a..c}", opts.clone(), false);

    // {1..100} natively expands to (?:1|2|...|100) — matches '99'
    assert_is_match("a/99", "a/{1..100}", opts.clone(), true);
}
