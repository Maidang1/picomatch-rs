# picomatch-rs

Rust-first glob matching with a `napi-rs` powered Node-API package.

This repository now keeps only the native implementation path:

- `crates/picomatch-rs`: core Rust library and Rust integration tests
- `napi/`: Node-API binding crate built with `napi-rs`
- package root: minimal npm wrapper that loads the compiled addon

## Install

```sh
npm install picomatch-rs
```

## Workspace Layout

```text
.
├── Cargo.toml
├── crates/
│   └── picomatch-rs/
├── napi/
├── index.js
├── index.d.ts
└── test/
    └── smoke.js
```

## Node Usage

```js
const native = require('picomatch-rs');

console.log(native.scan('src/**/*.rs'));
console.log(native.isMatch('src/lib.rs', '**/*.rs'));

const matcher = native.compileMatcher('**/*.rs');
console.log(matcher.test('crates/picomatch-rs/src/lib.rs'));
```

The package exports the native runtime surface from `napi/src/lib.rs`:

- `scan`
- `parse`
- `compileRe`
- `makeRe`
- `toRegex`
- `test`
- `matchBase`
- `isMatch`
- `compileMatcher`
- `NativeMatcher`

## Rust Usage

```rust
use picomatch_rs::{is_match, CompileOptions};

let matched = is_match("src/lib.rs", "**/*.rs", CompileOptions::default()).unwrap();
assert!(matched);
```

## Development

Build the Node addon:

```sh
npm run build
```

Run Rust tests:

```sh
npm run test:rust
```

Run the Node smoke test:

```sh
npm run test:node
```

Run the full verification path:

```sh
npm test
```

## Notes

- The root `index.js` is intentionally a thin native-loader shim.
- Generated TypeScript declarations live under `napi/index.d.ts` and are re-exported from the package root.
- Legacy JavaScript matcher sources, JS benchmark fixtures, and JS parity suites have been removed from the main development path.

## License

MIT
