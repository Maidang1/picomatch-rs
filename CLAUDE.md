# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Test Commands

```sh
npm run build       # build release addon and sync .node artifact
npm run build:debug # build debug addon for local debugging
npm run test:rust   # run Rust workspace tests
npm run test:node   # build addon and run Node test suite
npm test            # run both Rust and Node verification
cargo fmt --all     # format Rust code before landing changes
```

Run a single Rust test:
```sh
cargo test -p picomatch-rs --test <test_name>
```

## Architecture

This is a Rust-first `napi-rs` workspace that implements glob matching (like bash globs) for Node.js.

### Workspace Structure

- `crates/picomatch-rs/` — Rust matcher core (publishable to crates.io)
- `napi/` — Node-API binding crate that exposes the Rust core to Node.js
- Root package — minimal npm wrapper that loads the compiled addon

### Rust Core Modules (`crates/picomatch-rs/src/`)

- `compile.rs` — Pattern parsing and regex generation (ParseState, ParseToken, CompileOptions, regex_output_for_engine)
- `matcher.rs` — Matcher struct using `fancy-regex`; exposes is_match, compile_matcher, MatchError
- `scan.rs` — Token scanner (ScanState, ScanToken, ScanOptions, scan function)
- `constants.rs` — Character constants (CHAR_ASTERISK, etc.)
- `utils.rs` — Helpers: is_path_separator, remove_backslashes, etc.

### N-API Layer (`napi/src/lib.rs`)

Exposes native functions to Node: scan, parse, compileRe, makeRe, toRegex, test, matchBase, isMatch, compileMatcher, NativeMatcher.

### Key CompileOptions (serde camelCase for N-API)

- `posix: true` (default) — POSIX character class support
- `windows: false` — Windows path separator handling
- `strict_brackets: false` — validation of balanced parens/brackets
- `strict_slashes: false` — slash count validation
- `unescape: false` — unescape backslashes in output
- `keep_quotes: bool` — preserve double-quote literals in regex output
- `max_length: Option<usize>` — bounds-check pattern length

## Testing Conventions

- Rust integration tests under `crates/picomatch-rs/tests/` are the primary compatibility suite
- Status files in `crates/picomatch-rs/tests/status/` track compatibility suite results (update after fixing tests)
- Node tests under `test/` are smoke and parity coverage
- When fixing a compatibility test, update the matching status file with source location, result, and date

## Important Implementation Notes

- POSIX `[:punct:]` expansion emits `\[\]` to avoid nested-bracket parse errors in fancy_regex
- Long backslash runs are collapsed during compilation to avoid pathological regex behavior
- Invalid/unsupported patterns fail closed (return false) rather than throwing hard errors
- Globstar `**` standalone terminal uses `slash+` internally with optional leading `(?:slash+)?` prefix at index 0
- Trailing wildcard slash uses `(?:slash+)?` rather than `slash?`

## Release

Pushing a tag like `v0.1.0` triggers `.github/workflows/release.yml` which:
- Validates version parity across package.json, napi/Cargo.toml, crates/picomatch-rs/Cargo.toml
- Builds multi-platform .node artifacts
- Publishes to npm (@maidang1/picomatch-rs) and crates.io
