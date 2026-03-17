## extglobs-minimatch 迁移状态复核

已重新对照 `test/extglobs-minimatch.js` 与 `crates/picomatch-rs/tests/extglobs_minimatch.rs` 校验，并修复了此前注释跳过的 8 条已知差异。

### 复核结论

- `test/extglobs-minimatch.js` 共有 **642** 条 `assert(...)` 用例。
- `crates/picomatch-rs/tests/extglobs_minimatch.rs` 仍然通过 **1** 个批量 `#[test]` 承载这些场景。
- 现在 **642/642** 条 JS 用例都已作为可执行 Rust 断言覆盖，不再保留注释跳过项。

### 本轮修复的两个根因

1. **空字符串误匹配**
   - Rust 之前会让 `*(...)` / `?(...)` 这类可空模式错误匹配 `""`。
   - 现已在 matcher 层对齐 JS：非空 glob 不再匹配空输入。

2. **Windows 下反斜杠语义不一致**
   - Rust 之前会把 Windows 模式下的 `\` 过度当作路径分隔符，导致 `a\\z`、`a\\(b`、`*\\;...` 等 pattern 与 JS 不一致。
   - 现已按 JS 行为区分：
     - `\/` 继续作为路径分隔符处理；
     - `\(`、`\*`、`\;`、`\?` 等作为字面量转义；
     - `\z` 这类“反斜杠 + 普通字符”保留字面反斜杠语义。

### 已补齐的原 8 条差异

以下场景现在都已经恢复成正式断言并通过：

1. `isMatch('', '*(0|1|3|5|7|9)', { windows: true }) === false`
2. `isMatch('a\\(b', 'a(*b', { windows: true }) === false`
3. `isMatch('a\\(b', 'a(b', { windows: true }) === false`
4. `isMatch('a\\z', 'a\\\\z', { windows: true }) === false`
5. `isMatch('a\\\\z', 'a\\\\z', { windows: true }) === true`
6. `isMatch('a\\b', 'a/b', { windows: true }) === true`
7. `isMatch('a\\z', 'a\\\\z', { windows: false }) === true`
8. `isMatch('a\\z', 'a\\z', { windows: true }) === true`

### 验证结果

已执行：

```bash
cargo test -p picomatch-rs --test extglobs_minimatch
cargo test -p picomatch-rs --test slashes_windows
cargo test -p picomatch-rs --test extglobs_bash_generated
node - <<'NODE'
const tests = [];
global.describe = (_name, fn) => fn();
global.it = (name, fn) => tests.push({ name, fn });
require('./test/extglobs-minimatch.js');
let passed = 0;
for (const t of tests) {
  t.fn();
  passed++;
}
console.log(`js_harness_passed ${passed}/${tests.length}`);
NODE
```

结果：

- Rust `extglobs_minimatch`: `1 passed`
- Rust `slashes_windows`: `12 passed`
- Rust `extglobs_bash_generated`: `649 passed`
- JS harness `extglobs-minimatch.js`: `642/642 passed`

### 当前准确状态

`extglobs_minimatch` 现在已经可以标记为 **完整迁移且测试正确**，不再是“634 条已执行 + 8 条注释跳过”。
