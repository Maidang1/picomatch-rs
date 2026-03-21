# picomatch-rs

Glob matching library for Rust, inspired by picomatch.

## Install

```sh
cargo add picomatch-rs
```

## Usage

```rust
use picomatch_rs::{is_match, CompileOptions};

let matched = is_match("src/lib.rs", "**/*.rs", CompileOptions::default()).unwrap();
assert!(matched);
```

## Repository Layout

```text
.
├── crates/picomatch-rs/   # Rust matcher core and tests
├── napi/                  # Node-API binding crate (not published)
└── test/                  # Node smoke tests
```

## Development

```sh
npm run build       # build the release addon and sync the .node artifact
npm run build:debug # build a debug addon for local debugging
npm run test:rust   # run the Rust workspace tests
npm run test:node   # build the addon and run the Node tests
npm test            # run Rust and Node verification
```

## Release

Pushing a tag like `v0.1.0` triggers `.github/workflows/release.yml`.

The release workflow:

- verifies that the tag and `crates/picomatch-rs/Cargo.toml` use the same version
- publishes `crates/picomatch-rs` to crates.io

Required GitHub repository secret:

- `CARGO_REGISTRY_TOKEN`

## Test
via napi-rs, 100% test cases passed 
<img width="1628" height="890" alt="image" src="https://github.com/user-attachments/assets/bed7296b-139f-4d7f-9f39-b34697cea5c5" />


## License

MIT
