# Action Items from Repository Review

**Review Date**: 2026-03-21
**Priority Legend**: 🔴 High | 🟡 Medium | 🟢 Low

---

## Immediate Actions (Before Next Release)

### 🔴 1. Improve TypeScript Type Definitions

**File**: `napi/index.d.ts`

**Current Issues**:
- Generic `any` types reduce type safety
- Missing interface definitions for options and return types
- Poor IDE autocomplete experience

**Action Steps**:
1. Define comprehensive TypeScript interfaces:
   ```typescript
   export interface ScanOptions {
     tokens?: boolean;
     parts?: boolean;
   }

   export interface ScanState {
     input: string;
     index: number;
     tokens: ScanToken[];
     parts: string[];
   }

   export interface CompileOptions {
     posix?: boolean;
     windows?: boolean;
     dot?: boolean;
     noextglob?: boolean;
     noglobstar?: boolean;
     nobrace?: boolean;
     // ... 27 more fields
   }
   ```

2. Replace `any` types with specific interfaces
3. Add JSDoc comments for each option
4. Generate types from Rust structs if possible (investigate napi-rs type generation)

**Estimated Effort**: 4 hours
**Impact**: High - Improves developer experience significantly

---

### 🟡 2. Address npm Audit Vulnerabilities

**Current Status**:
```
2 high severity vulnerabilities
- glob@8.1.0 (deprecated, known vulnerabilities)
- inflight@1.0.6 (deprecated, memory leak)
```

**Action Steps**:
1. Run `npm audit` to see full details
2. Update `mocha` to latest version (likely pulls in newer dependencies)
3. Run `npm audit fix` to auto-resolve
4. Verify tests still pass after update
5. Update package-lock.json and commit

**Estimated Effort**: 30 minutes
**Impact**: Medium - Addresses security warnings (dev dependencies only)

**Commands**:
```bash
npm audit
npm update mocha
npm audit fix
npm test
```

---

### 🟢 3. Clean Up Dead Code Warnings

**Current Warnings**:
```
warning: unused import: `default_compile_options`
 --> crates/picomatch-rs/tests/extglobs_bash_generated.rs:5:32

warning: function `posix_options` is never used
 --> crates/picomatch-rs/tests/extglobs_temp.rs:12:4
```

**Action Steps**:
1. Run `cargo clippy --workspace --tests` for full analysis
2. Remove unused imports in test files:
   - `extglobs_bash_generated.rs`
   - `extglobs_temp.rs`
3. Remove or mark unused functions:
   - `posix_options()` in `extglobs_temp.rs`
   - `keep_quotes_opts()` in `negation.rs`
4. Run tests to verify nothing breaks
5. Consider adding `cargo clippy` to CI workflow

**Estimated Effort**: 1 hour
**Impact**: Low - Cleaner code, better maintainability

**Commands**:
```bash
cargo clippy --workspace --tests
cargo test --workspace
```

---

## Short-term Improvements (Next Sprint)

### 🔴 4. Enhance Error Messages for Unsupported Patterns

**Current Behavior**: Invalid patterns silently return `false`

**Problem**: Users cannot distinguish:
- Pattern doesn't match input (expected behavior)
- Pattern is malformed/unsupported (error condition)

**Proposed Solution**:
1. Extend `MatchError` enum in `matcher.rs`:
   ```rust
   pub enum MatchError {
       CompilationFailed(String),
       UnsupportedPattern { pattern: String, reason: String },
       RegexError(fancy_regex::Error),
       MaxLengthExceeded { length: usize, max: usize },
   }
   ```

2. Update `compile_matcher()` to return `Result<Matcher, MatchError>`

3. Update N-API bindings to throw JavaScript errors with descriptive messages

4. Update documentation with error handling examples

**Estimated Effort**: 6 hours
**Impact**: High - Better debugging experience

**Files to modify**:
- `crates/picomatch-rs/src/matcher.rs`
- `crates/picomatch-rs/src/compile.rs`
- `napi/src/lib.rs`
- `napi/index.d.ts`

---

### 🟡 5. Standardize Test Status Files

**Issue**: Mixed English/Chinese in `crates/picomatch-rs/tests/status/`

**Action Steps**:
1. Choose English as standard language (better for international collaboration)
2. Create status file template in AGENTS.md:
   ```markdown
   # Test Suite: [name]

   **Source**: [upstream reference]
   **Status**: [passing/failing]
   **Last Updated**: YYYY-MM-DD

   ## Results
   - Total tests: X
   - Passing: Y
   - Failing: Z

   ## Notes
   [Any specific observations]
   ```

3. Update all 32 status files to match template
4. Translate Chinese status files to English

**Estimated Effort**: 3 hours
**Impact**: Medium - Better maintainability for international contributors

---

### 🟡 6. Add Windows Path Edge Case Tests

**Goal**: Ensure cross-platform reliability

**Action Steps**:
1. Create `crates/picomatch-rs/tests/windows_paths.rs`
2. Add test cases for:
   - Mixed separators: `path\\to/file.txt`
   - UNC paths: `\\\\server\\share\\file.txt`
   - Drive letters: `C:\\Users\\file.txt`
   - Trailing slashes: `path\\to\\dir\\`
   - Forward slashes on Windows: `C:/Users/file.txt`
3. Test with `windows: true` option
4. Document Windows-specific behavior in CLAUDE.md
5. Update status file

**Estimated Effort**: 4 hours
**Impact**: Medium - Prevents cross-platform bugs

**Reference**: Node.js `path.basename()` behavior

---

## Long-term Enhancements (Next Quarter)

### 🟡 7. Document fancy-regex Dependency Choice

**Goal**: Explain why `fancy-regex` instead of standard `regex` crate

**Action Steps**:
1. Research lookahead/lookbehind usage in pattern compilation
2. Add rustdoc comment in `matcher.rs`:
   ```rust
   /// Uses fancy-regex instead of regex crate because:
   /// - Extglob patterns require lookahead assertions
   /// - Negation patterns need lookbehind
   /// - POSIX bracket classes need complex character ranges
   ///
   /// Trade-off: fancy-regex is slower but more feature-complete
   ```

3. Document in CLAUDE.md under "Important Implementation Notes"
4. Note any known limitations or quirks
5. Add link to fancy-regex documentation

**Estimated Effort**: 2 hours
**Impact**: Medium - Better understanding for maintainers

---

### 🟢 8. Add Inline Documentation for Complex Logic

**Files needing more comments**:
- `compile.rs` - `compile_bracket()` function (~200 lines)
- `compile.rs` - POSIX class expansion logic
- `scan.rs` - Token state machine transitions

**Action Steps**:
1. Add rustdoc comments to public functions
2. Add inline comments for complex algorithms:
   ```rust
   // POSIX [:punct:] expands to \[\] instead of [] to avoid
   // nested-bracket parse errors in fancy_regex (see issue #X)
   if posix_class == "punct" {
       output.push_str(r"\[\]");
   }
   ```

3. Link to relevant test cases in comments
4. Explain non-obvious design decisions

**Estimated Effort**: 4 hours
**Impact**: Low - Better code readability

---

### 🟢 9. Create API Reference Documentation

**Goal**: Comprehensive user-facing documentation

**Action Steps**:
1. Create `docs/API.md` with:
   - All exported functions
   - Parameter descriptions
   - Return types
   - Usage examples
   - Common patterns

2. Document CompileOptions fields:
   ```markdown
   ### CompileOptions

   | Option | Type | Default | Description |
   |--------|------|---------|-------------|
   | posix | boolean | true | Enable POSIX character classes |
   | windows | boolean | false | Use Windows path separators |
   | dot | boolean | false | Match dotfiles |
   ...
   ```

3. Add migration guide from picomatch-js
4. Include performance tips
5. Link from README.md

**Estimated Effort**: 8 hours
**Impact**: Low - Better onboarding for new users

---

### 🟢 10. Enrich Package Metadata

**Goal**: Improve discoverability

**Action Steps**:

1. Update `crates/picomatch-rs/Cargo.toml`:
   ```toml
   [package]
   keywords = ["glob", "matcher", "wildcard", "picomatch", "pattern", "napi"]
   categories = ["filesystem", "text-processing", "parser-implementations"]

   [badges]
   maintenance = { status = "actively-developed" }
   ```

2. Update `package.json`:
   ```json
   "keywords": [
     "glob", "rust", "napi-rs", "node-api",
     "pattern", "matcher", "wildcard", "picomatch"
   ]
   ```

3. Add badges to README.md:
   - Crates.io version
   - npm version
   - CI status
   - Test coverage (if available)
   - License

4. Add links:
   - npm package → crates.io
   - crates.io docs → GitHub

**Estimated Effort**: 1 hour
**Impact**: Trivial - Slightly better discoverability

---

## Optional Performance Optimizations

### 🟢 11. Profile and Optimize Hot Paths (if needed)

**Prerequisites**: Benchmark to confirm actual performance issues

**Potential Areas**:
1. Pattern caching at N-API layer (LRU cache for repeated patterns)
2. String interning for common patterns
3. Lazy regex compilation

**Action Steps** (only if profiling shows need):
1. Create benchmark suite:
   ```bash
   cargo bench
   ```

2. Profile with `cargo flamegraph`
3. Identify bottlenecks
4. Implement targeted optimizations
5. Verify with benchmarks
6. Document performance characteristics

**Estimated Effort**: 8-16 hours (if pursued)
**Impact**: Unknown - depends on profiling results

---

## CI/CD Enhancements

### 🟢 12. Add Cargo Clippy to CI

**Goal**: Catch lints automatically

**Action Steps**:
1. Update `.github/workflows/test.yml`:
   ```yaml
   - name: Run clippy
     run: cargo clippy --workspace --all-targets -- -D warnings
   ```

2. Fix any existing clippy warnings first
3. Add to test workflow before running tests

**Estimated Effort**: 30 minutes
**Impact**: Low - Prevents future lint regressions

---

### 🟢 13. Add Code Coverage Tracking

**Goal**: Visibility into test coverage

**Action Steps**:
1. Add `cargo-tarpaulin` or `cargo-llvm-cov` to CI
2. Upload coverage to Codecov or Coveralls
3. Add coverage badge to README.md
4. Set minimum coverage threshold (e.g., 80%)

**Estimated Effort**: 2 hours
**Impact**: Low - Better visibility, but coverage already high

---

## Summary by Priority

### High Priority (Must Do)
- 🔴 Improve TypeScript type definitions (4 hours)
- 🔴 Enhance error messages (6 hours)

**Total**: 10 hours

### Medium Priority (Should Do)
- 🟡 Address npm audit vulnerabilities (30 min)
- 🟡 Standardize test status files (3 hours)
- 🟡 Add Windows path tests (4 hours)
- 🟡 Document fancy-regex choice (2 hours)

**Total**: 9.5 hours

### Low Priority (Nice to Have)
- 🟢 Clean up dead code warnings (1 hour)
- 🟢 Add inline documentation (4 hours)
- 🟢 Create API reference (8 hours)
- 🟢 Enrich package metadata (1 hour)
- 🟢 Add clippy to CI (30 min)
- 🟢 Add code coverage (2 hours)

**Total**: 16.5 hours

---

## Next Steps

1. **Review this action plan** with maintainers
2. **Prioritize items** based on project roadmap
3. **Create GitHub issues** for each action item
4. **Assign owners** and deadlines
5. **Track progress** in project board

**Estimated Total Effort**: 36 hours for all items
**Recommended First Sprint**: Items 1-3 (immediate actions) = 5.5 hours
