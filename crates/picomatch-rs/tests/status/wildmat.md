# wildmat.rs — Migration Status

Source: `test/wildmat.js`
Rust test: `tests/wildmat.rs`
Status: **2/2 tests passing** ✅

## Sections migrated

| JS `it(...)` description | Rust test function | Status |
|---|---|---|
| Basic wildmat features | `basic_wildmat_features` | ✅ |
| should support recursion | `should_support_recursion` | ✅ |

## Coverage

All 44 JS assertions fully migrated (12 basic + 32 recursion).
No known differences or skipped assertions.
