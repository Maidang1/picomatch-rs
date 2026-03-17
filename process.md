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

## Not Done

- The root `test:node` script can still hit sandbox-dependent file-copy behavior in restricted environments because `napi-rs` writes an intermediate artifact under `napi/` before the root sync step.
- Historical status files under `crates/picomatch-rs/tests/status/` are still retained.

## Latest Verification

- `CARGO_TARGET_DIR=/tmp/picomatch-target cargo test --workspace`
  - Result: passed
- `CARGO_TARGET_DIR=/tmp/picomatch-target npm run build`
  - Result: passed
- `npm run mocha -- test/smoke.js`
  - Result: `2 passing`, `0 failing`
- `CARGO_TARGET_DIR=/tmp/picomatch-target cargo test -p picomatch-rs --test posix_classes`
  - Result: `11 passed`, `0 failed`
- Documentation-only update for `AGENTS.md`
  - Result: no tests run
- Repository cleanup pass removing redundant local files
  - Result: no tests run

## Current Risks

- Any freshly built root `.node` artifact is platform-specific and should not be treated as a cross-platform release artifact.
- Some npm build behavior still depends on how `@napi-rs/cli` copies artifacts under restricted sandboxes.


# important

当你开始启动的时候，你需要去读取 process.md 去看之前的完成任务

When you do compression, you need to keep a file (process.md) of work done and work not done, status done and progress done, so that other models can take over and know exactly what you did and didn't do.

补充迁移缺失的那个测试 如果没有实现或者测试用例失败了，不可以不迁移，可以添加注释标注。进行测试用例的时候，需要将测试用例测试的状态写入到 /crates/picomatch-rs/tests/status 文件夹下面,并且标注最后修复时间以及原JS文件位置和输出的Rust文件位置，并且在 process.md 添加简短的测试状态描述。
