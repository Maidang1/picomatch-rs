# brackets.rs 测试状态

## 对应关系

- 源文件: `test/brackets.js`
- 目标文件: `crates/picomatch-rs/tests/brackets.rs`

## 复核结论

2026-03-15 重新校验后，`brackets` 这组测试已经**完整迁移**，状态文件原结论基本正确。

- JS 顶层测试：`3`
- Rust 顶层测试：`3`
- JS 断言数：`9`
- Rust 断言数：`9`

顶层测试一一对应：

1. `should support stars following brackets` → `should_support_stars_following_brackets`
2. `should match slashes defined in brackets` → `should_match_slashes_defined_in_brackets`
3. `should not match slashes following brackets` → `should_not_match_slashes_following_brackets`

迁移状态：✅ 已完成

## 覆盖范围

- bracket 后接 `*` 的匹配
- bracket 中显式定义 `/` 的匹配
- bracket 后的 `*` 不跨越 `/`

## 验证结果

```bash
cargo test -p picomatch-rs --test brackets
```

- 结果：`3 passed`, `0 failed`

```bash
node - <<'NODE'
const tests = [];
global.describe = (_name, fn) => fn();
global.it = (name, fn) => tests.push({ name, fn });
require('./test/brackets.js');
let passed = 0;
for (const t of tests) {
  t.fn();
  passed++;
}
console.log(`js_harness_passed ${passed}/${tests.length}`);
NODE
```

- 结果：`js_harness_passed 3/3`

## 备注

1. 这组用例不依赖 `node_modules` 中的额外包，因此可以直接用零依赖 harness 复跑原始 JS 文件。
2. 当前 Rust 测试内容与 JS 语义逐条对齐，没有发现缺失项或状态漂移。
