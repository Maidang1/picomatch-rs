# Contributing to picomatch-rs

Thanks for contributing.

This repository is now organized as a Rust workspace with a `napi-rs` binding crate and a minimal npm package wrapper. Contributions should preserve that split:

- core glob behavior belongs in `crates/picomatch-rs`
- Node-API glue belongs in `napi/`
- root JavaScript should stay as a thin addon loader and smoke-test layer

## Before opening a PR

- run `npm run test:rust`
- run `npm run test:node`
- update `process.md` if the change affects migration status or test coverage bookkeeping
- update `crates/picomatch-rs/tests/status/` when a migrated test file changes status

## Issues and PRs

When reporting a bug or opening a PR, include:

- the exact command you ran
- the platform and Node version when the issue is Node-facing
- the affected Rust crate or N-API surface
- any mismatch between Rust tests and the Node smoke layer
