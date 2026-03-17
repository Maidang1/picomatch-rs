# Repository Guidelines

## Project Structure & Module Organization

This repository is a Rust-first workspace with a thin Node-API wrapper. The Rust core lives in `crates/picomatch-rs/`, including the matcher implementation under `src/` and the main regression suite under `tests/`. The native Node binding crate lives in `napi/`. The package root contains the npm manifest, the loader in `index.js`, the public typings in `index.d.ts`, build helpers in `tools/`, and a small smoke test in `test/smoke.js`.

## Build, Test, and Development Commands

- `npm run build`: build the release addon for the current platform and sync the `.node` artifact to the package root.
- `npm run build:debug`: build the debug-profile addon for local debugging.
- `npm run test:rust`: run the Cargo workspace tests.
- `npm run test:node`: build the addon and run the Node smoke test.
- `npm test`: run the full Rust and Node verification path.

Use repo-root paths when working locally, for example `crates/picomatch-rs/tests/posix_classes.rs`.

## Coding Style & Naming Conventions

Follow `.editorconfig`: UTF-8, LF endings, spaces, and 2-space indentation for JavaScript and Markdown. Keep root-level JavaScript minimal and loader-focused; core matching behavior belongs in Rust or the `napi` crate. Prefer descriptive lowercase filenames. Run standard Rust formatting before landing Rust changes.

## Testing Guidelines

Rust integration tests in `crates/picomatch-rs/tests/` are the primary compatibility suite. The root `test/` directory is only for addon loading and API smoke coverage. When migrating or fixing a compatibility test, update the matching note in `crates/picomatch-rs/tests/status/` with source locations, result, and last fix date, then add a short status line to `process.md`.

## Commit & Pull Request Guidelines

Keep commit subjects short and imperative. Existing history uses plain subjects and prefixes such as `feat:` and `refactor:`. Pull requests should summarize structural changes, list the exact commands run, and call out packaging, native artifact, or CI behavior changes.

## Agent-Specific Notes

Read `process.md` before starting work. After finishing, record what changed, what remains open, and any verification you ran so the next contributor can pick up without re-discovery.


# important

当你开始启动的时候，你需要去读取 process.md 去看之前的完成任务

When you do compression, you need to keep a file (process.md) of work done and work not done, status done and progress done, so that other models can take over and know exactly what you did and didn't do.

补充迁移缺失的那个测试 如果没有实现或者测试用例失败了，不可以不迁移，可以添加注释标注。进行测试用例的时候，需要将测试用例测试的状态写入到 /crates/picomatch-rs/tests/status 文件夹下面,并且标注最后修复时间以及原JS文件位置和输出的Rust文件位置，并且在 process.md 添加简短的测试状态描述。
