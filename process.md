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
- The remaining `29` Node parity failures have not been fixed yet in `napi/src/lib.rs` / Rust core.
- The full Node suite currently aborts before assertions in `test/regex-features.js` because `../lib/utils` no longer exists.

## Latest Verification

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
- Some upstream parity tests currently import historical JS-only helper paths such as `../lib/utils`; those imports will now fail unless equivalent native-backed surfaces are intentionally reintroduced as thin forwards.
- Some upstream JS tests rely on old package entrypoints such as `require('../lib/scan')` and `require('../posix')`; if these entrypoints are retained, they should stay as thin forwarding shims only.
