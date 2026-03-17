# dots_invalid.rs 测试状态

## 对应关系

- 源文件: `test/dots-invalid.js`
- 目标文件: `crates/picomatch-rs/tests/dots_invalid.rs`

## 复核结论

2026-03-15 重新校验后，`dots_invalid` 这组测试已经**完整迁移**，当前状态正确。

- JS 顶层测试：`152`
- Rust `#[test]` 数：`152`
- JS 原始断言行数：`1009`
- Rust `assert_is_match(...)` 调用数：`152`

说明：

- `test/dots-invalid.js` 的大量断言是“同一语义组内的矩阵重复”
- Rust 迁移方式是将每个 JS `it(...)` 压成一个 Rust `#[test]`，在测试体内批量遍历 pattern / input 组合
- 因此 Rust 断言调用数远小于 JS 断言行数，但顶层测试语义与执行结果已完整对齐

## 覆盖范围

- `..`（double dots）在以下选项组合下的非法匹配保护：
  - 默认选项
  - `{ dot: true }`
  - `{ strictSlashes: true }`
  - `{ dot: true, strictSlashes: true }`
- `.`（single dots）在同样四组选项下的非法匹配保护
- leading / nested / trailing 三类位置
- `*`、`.*`、`*.`、`**`、`**/.**`、`**/**.`、`**/**.**/**` 等多种 pattern 形态

迁移状态：✅ 已完成

## 验证结果

```bash
cargo test -p picomatch-rs --test dots_invalid
```

- 结果：`152 passed`, `0 failed`

```bash
node - <<'NODE'
const tests = [];
global.describe = (_name, fn) => fn();
global.it = (name, fn) => tests.push({ name, fn });
require('./test/dots-invalid.js');
let passed = 0;
for (const t of tests) {
  t.fn();
  passed++;
}
console.log(`js_harness_passed ${passed}/${tests.length}`);
NODE
```

- 结果：`js_harness_passed 152/152`

## 备注

1. 如果用简单的 `fn ...()` 正则去数 Rust 文件里的函数，会得到 `158`，因为其中包含少量模块内 helper 函数；真正的 `#[test]` 数量是 `152`，与 JS 顶层 `it(...)` 完全一致。
2. 当前没有发现缺失项、错误期待值或状态漂移。
