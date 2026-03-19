# picomatch-rs

Native glob matching for Node.js, implemented in Rust and exposed through `napi-rs`.

The package root loads the compiled addon and re-exports the native API. The Rust core lives in `crates/picomatch-rs/`, and the Node binding lives in `napi/`.

## Install

```sh
npm install picomatch-rs
```

## Usage

### Node.js

`require('picomatch-rs')` returns a callable matcher factory and also exposes the native helpers as named exports.

```js
const picomatch = require('picomatch-rs');

const isJsFile = picomatch('**/*.js');

console.log(isJsFile('src/index.js'));
console.log(picomatch.isMatch('src/lib.rs', '**/*.rs'));
console.log(picomatch.scan('src/**/*.rs'));
```

Available exports:

- default export: callable matcher factory
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

Legacy entrypoints such as `./lib/picomatch`, `./posix`, and `./lib/scan` are thin compatibility shims.

### Rust

```rust
use picomatch_rs::{is_match, CompileOptions};

let matched = is_match("src/lib.rs", "**/*.rs", CompileOptions::default()).unwrap();
assert!(matched);
```

## Repository Layout

```text
.
├── crates/picomatch-rs/   # Rust matcher core and Rust tests
├── napi/                  # Node-API binding crate
├── index.js               # package entrypoint
├── index.d.ts             # root TypeScript declarations
├── native.js              # native addon loader
├── lib/                   # compatibility shims
└── test/                  # Node smoke and parity tests
```

## Development

```sh
npm run build       # build the release addon and sync the .node artifact
npm run build:debug # build a debug addon for local debugging
npm run test:rust   # run the Rust workspace tests
npm run test:node   # build the addon and run the Node tests
npm test            # run Rust and Node verification
```

## Notes

- `index.js` stays intentionally small: it loads the native addon and provides the callable package surface.
- `index.d.ts` re-exports the generated declarations from `napi/index.d.ts`.
- The repository no longer uses the old JavaScript implementation path for core matching behavior.

## License

MIT
