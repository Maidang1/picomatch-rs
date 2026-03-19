# Repository Guidelines

## Purpose

This repository packages a Rust glob matcher as a native Node addon.

- Rust core: `crates/picomatch-rs/`
- Node binding: `napi/`
- Package entrypoint and loader: `index.js` and `native.js`
- Public TypeScript declarations: `index.d.ts`
- Compatibility shims: `lib/`
- Smoke and parity tests: `test/`

## Before You Start

- Read `process.md` first.
- Keep changes small and explicit.
- Do not rewrite user-owned work or unrelated files.

## Build And Test

- `npm run build`: build the release addon and sync the package-root `.node` artifact.
- `npm run build:debug`: build a debug addon for local debugging.
- `npm run test:rust`: run the Rust workspace tests.
- `npm run test:node`: build the addon and run the Node test suite.
- `npm test`: run Rust and Node verification.

Use repo-root paths when referring to files, for example `crates/picomatch-rs/tests/posix_classes.rs`.

## Coding Conventions

- Follow `.editorconfig`: UTF-8, LF line endings, spaces, and 2-space indentation for JavaScript and Markdown.
- Keep root-level JavaScript loader-focused.
- Keep APIs small and behavior explicit.
- Prefer descriptive lowercase filenames.
- Run `cargo fmt --all` before landing Rust changes.

## Testing Notes

- Rust integration tests under `crates/picomatch-rs/tests/` are the primary compatibility suite.
- `test/` is for addon loading and Node smoke/parity coverage.
- When migrating or fixing a compatibility test, update the matching note in `crates/picomatch-rs/tests/status/` with source location, result, and last fix date.
- Add a short status line to `process.md` after test work.

## Handoff

- Summarize what changed, what remains open, and which verification commands ran.
- Keep commit subjects short and imperative.
- Mention packaging, native artifact, or CI behavior changes in PR notes when relevant.
