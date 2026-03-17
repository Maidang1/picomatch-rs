# posix_classes

- Last fixed: 2026-03-18
- Status: done
- Original JS file: [test/posix-classes.js](/Users/felixwliu/codes/open-source/picomatch/test/posix-classes.js)
- Output Rust file:
  - [crates/picomatch-rs/tests/posix_classes.rs](/Users/felixwliu/codes/open-source/picomatch/crates/picomatch-rs/tests/posix_classes.rs)
  - [crates/picomatch-rs/src/compile.rs](/Users/felixwliu/codes/open-source/picomatch/crates/picomatch-rs/src/compile.rs)

## Notes

- Re-verified the migrated Rust test against the original JS source on 2026-03-18.
- Fixed `[:punct:]` escaping in `compile.rs` so `fancy-regex` accepts the generated character class.
- Corrected two Rust expectations that had drifted from the original JS file:
  - `[:space:]` conversion must keep the JS-style escaped output including `\r`
  - malformed `[abc[:punct:][0-9]` is expected to match `!` in the original JS test suite

## Verification

- `CARGO_TARGET_DIR=/tmp/picomatch-target cargo test -p picomatch-rs --test posix_classes`
  - Result: `11 passed`, `0 failed`
