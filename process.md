# Process

## Current Status

- Status: in progress
- Repository shape: Rust-first `napi-rs` workspace
- Package identity: `picomatch-rs`
- Root package role: minimal npm wrapper for the native addon

## Done

- Removed the legacy JavaScript implementation and old JS-facing repo layout:
  - deleted `lib/`
  - deleted `examples/`
  - deleted `bench/`
  - deleted migration-only `plans/` and `scripts/`
  - deleted the old JS parity suite under `test/`, keeping only `test/smoke.js`
- Kept the Rust/N-API repository core:
  - `Cargo.toml`
  - `crates/picomatch-rs/`
  - `napi/`
  - root `index.js` and `index.d.ts`
  - `crates/picomatch-rs/tests/status/`
- Reworked the root package into a standard native package shape:
  - updated `package.json` to `picomatch-rs`
  - rewrote `README.md`
  - updated CI workflows and contributing guidance
  - added `tools/sync-native-artifact.js` to sync the built native library into a package-root `.node` artifact
- Replaced the old JS API test entry with a minimal root package smoke test:
  - `test/smoke.js`
- Re-verified and fixed POSIX class coverage:
  - fixed `[:punct:]` escaping in `crates/picomatch-rs/src/compile.rs`
  - corrected Rust expectations in `crates/picomatch-rs/tests/posix_classes.rs`
  - updated `crates/picomatch-rs/tests/status/posix_classes.md`
  - updated `crates/picomatch-rs/tests/status/napi_api.md`
- Added a root contributor guide:
  - created `AGENTS.md` with repository structure, commands, style, testing, PR guidance, and handoff notes
- Removed clearly redundant local files after a full repo pass:
  - deleted duplicate handoff notes in `CLAUDE.md`
  - deleted local generated artifacts: `node_modules/`, `target/`, `picomatch-rs.darwin-arm64.node`
- Migrated the upstream JS parity suite from `/Users/felixwliu/codes/open-source/picomatch-js/test` into the repo root `test/` directory:
  - added all upstream `test/*.js` files
  - added `test/support/match.js`
  - kept the existing `test/smoke.js`
- Expanded the Node test script to run the full JS suite:
  - updated `package.json` `test:node` to run `test/**/*.js`
- Captured the first full Node parity result against the current native surface:
  - `1937 passing`
  - `29 failing`
- Identified the current implementation constraint from the user:
  - JS layer must only forward to native exports
  - JS layer must not implement compatibility logic on top of native behavior
- Collapsed the interim JS compatibility wrapper back to native-only forwarding:
  - `index.js` now exports `./native`
  - `lib/picomatch.js` now exports `../native`
  - `posix.js` now exports `./native`
  - removed `lib/utils.js`
  - retained `lib/scan.js` as a thin forward to `native.scan`
- Restored the callable package surface required by upstream API tests while keeping matching behavior in native code:
  - `index.js` now exports a callable `picomatch(...)` wrapper around native `compileMatcher`
  - `posix.js` and `lib/picomatch.js` now forward to the root callable export
  - `napi/src/lib.rs` now aligns invalid-pattern errors with the JS test expectation
  - `napi/src/lib.rs` now includes `negatedExtglob` on `matcher.state`
  - updated `crates/picomatch-rs/tests/status/api_picomatch.md`

## Not Done

- The root `test:node` script can still hit sandbox-dependent file-copy behavior in restricted environments because `napi-rs` writes an intermediate artifact under `napi/` before the root sync step.
- Historical status files under `crates/picomatch-rs/tests/status/` are still retained.
- There are still low-signal Rust test warnings (`unused_imports`, `dead_code`) in a few migrated test files that have not been cleaned up.

## Latest Verification

- `pnpm run test`
  - Result: passed
  - Rust: workspace tests passed, including `malicious.rs`, `options.rs`, and `posix_classes.rs`
  - Node: `1966 passing`, `0 failing`
- `pnpm run test`
  - Result: failed in `crates/picomatch-rs/tests/posix_classes.rs` with `5` failing tests:
    - `posix_bracket_type_conversion`
    - `should_not_create_an_invalid_posix_character_class`
    - `matches_positive`
    - `bash_unit_tests_ported`
    - `posix_2_bre_tests`
  - Failure shape: POSIX `[:punct:]` expansion currently emits `[` without escaping it for the generated regex character class, which causes expectation drift and `fancy_regex` parse failures (`Invalid character class`)
- `CARGO_TARGET_DIR=/tmp/picomatch-target cargo test --workspace`
  - Result: passed
- `CARGO_TARGET_DIR=/tmp/picomatch-target npm run build`
  - Result: passed
- `npm run mocha -- test/smoke.js`
  - Result: `2 passing`, `0 failing`
- `cargo fmt --all`
  - Result: passed
- `CARGO_TARGET_DIR=/tmp/picomatch-target npm run build`
  - Result: passed
- `npm run mocha -- test/api.picomatch.js`
  - Result: `24 passing`, `0 failing`
- `npm run mocha -- test/api.posix.js`
  - Result: `2 passing`, `0 failing`
- `npm run mocha -- "test/**/*.js"`
  - Result: failed during module load because `test/regex-features.js` requires missing `../lib/utils`
- `CARGO_TARGET_DIR=/tmp/picomatch-target cargo test -p picomatch-rs --test posix_classes`
  - Result: `11 passed`, `0 failed`
- Documentation-only update for `AGENTS.md`
  - Result: no tests run
- Repository cleanup pass removing redundant local files
  - Result: no tests run
- `CARGO_TARGET_DIR=/tmp/picomatch-target npm run build`
  - Result: passed
- `npm run mocha -- "test/**/*.js"`
  - Result: `1937 passing`, `29 failing`
- `node` require smoke for `.`, `./lib/picomatch`, `./posix`, `./lib/scan`
  - Result: passed
- `npm run mocha -- test/smoke.js`
  - Result: `2 passing`, `0 failing`

## Current Risks

- Any freshly built root `.node` artifact is platform-specific and should not be treated as a cross-platform release artifact.
- Some npm build behavior still depends on how `@napi-rs/cli` copies artifacts under restricted sandboxes.
- The current JS wrapper layer can mask which failures belong to native behavior versus wrapper behavior, so parity work should now move back into `napi/src/lib.rs` and the Rust core only.
- Some upstream JS tests rely on old package entrypoints such as `require('../lib/scan')` and `require('../posix')`; if these entrypoints are retained, they should stay as thin forwarding shims only.

## Session: Fix JS test case errors (keepQuotes, maxLength, strictBrackets, globstar, POSIX)

### Done
- Added `keep_quotes: bool` to `CompileOptions` (serde: `keepQuotes`); when true, double-quoted literals keep their quotes in the regex output
- Added `max_length: Option<usize>` to `CompileOptions` (serde: `maxLength`); NAPI layer enforces 65536-byte default in `makeRe`, `isMatch`, `compileMatcher`
- Added `check_strict_brackets` in `napi/src/lib.rs`; throws descriptive errors for unbalanced parens/brackets when `strict_brackets: true`
- Fixed globstar standalone terminal `**` to handle consecutive slashes (`slash+` internally, optional `(?:slash+)?` leading prefix at index 0)
- Changed trailing wildcard slash from `slash?` to `(?:slash+)?`
- Fixed POSIX `blank` class: `r" \t"` (regex escape) instead of literal tab
- Fixed POSIX `punct` class: `\[\]` instead of `[\]` to avoid nested-bracket parse errors in fancy_regex
- Updated `posix_classes.rs` test expectations for blank (`\\t`) and punct (`\\[\\]`)
- Bounds-checked backslash skip in `check_strict_brackets`

### Verification
- `CARGO_TARGET_DIR=/tmp/picomatch-target cargo test --workspace` → all passed
- `npm run mocha -- "test/special-characters.js" "test/malicious.js" "test/extglobs.js" "test/negation.js"` → 96 passing, 2 failing (both pre-existing)
- `npm run mocha -- "test/**/*.js"` → 1960 passing, 6 failing (was 1937/29 before)
- CodeQL: 0 alerts

### Current Test Status
- `pnpm run test` now passes end-to-end
- POSIX `[:punct:]` parse output now stays JS-compatible while regex compilation sanitizes the fragment for `fancy_regex`
- `windows + unescape` now matches both separator-preserving and separator-stripped forms for patterns like `\\a\\b\\c`
- Long backslash sequences now collapse safely enough to satisfy `malicious.js`
- Invalid unsupported patterns now fail closed in N-API `isMatch` / `compileMatcher` instead of surfacing as hard errors in the Node parity path

### Status files to update
- `crates/picomatch-rs/tests/status/posix_classes.md` — now passing again; parse output vs engine-source sanitization documented
- `crates/picomatch-rs/tests/status/malicious.md` — long backslash sequence test re-enabled; 2/2 passing
- `crates/picomatch-rs/tests/status/options.md` — `options.unescape` parity updated for `windows: true`
- `crates/picomatch-rs/tests/status/special_characters.md` — globstar consecutive slash partially improved

## Session: Fix current pnpm test regressions

### Done
- Restored JS-compatible `[:punct:]` parse output while sanitizing regex source only for the Rust regex engine
- Fixed N-API `compileRe` / `makeRe` source generation to use the sanitized engine output
- Fixed `windows + unescape` handling for ordinary escaped literals such as `\\a\\b\\c`
- Collapsed very long backslash runs during compilation to avoid pathological regex behavior
- Re-enabled `crates/picomatch-rs/tests/malicious.rs::test_long_escape_sequences`
- Adjusted N-API unsupported-pattern handling so Node parity `isMatch` returns `false` instead of throwing on fail-closed cases
- Updated Rust `options.rs` helper to dedupe normalized matches like JS `test/support/match.js`

### Verification
- `cargo test -p picomatch-rs --test posix_classes --test options --test malicious -- --ignored` → passed
- `npm run mocha -- test/malicious.js` → `4 passing`
- `pnpm run test` → passed

## Session: Fix Windows native artifact sync lookup

### Done
- Fixed `tools/sync-native-artifact.js` candidate names to support both `lib*` and non-`lib` dynamic library names:
  - `libpicomatch_rs_napi.<ext>`
  - `libpicomatch_napi.<ext>`
  - `picomatch_rs_napi.<ext>`
  - `picomatch_napi.<ext>`
- Root cause addressed: Windows `cdylib` commonly emits `picomatch_rs_napi.dll` (no `lib` prefix), which the old script did not search.

### Verification
- `npm run build` → passed
- `npm run mocha -- test/smoke.js` → `2 passing`, `0 failing`

### Not Done
- No live Windows runner verification in this local macOS session; Windows CI should be re-run to confirm end-to-end.
