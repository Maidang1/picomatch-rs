# braces.rs 测试状态

## 对应关系

- 源文件: `test/braces.js`
- 目标文件: `crates/picomatch-rs/tests/braces.rs`

## 复核结论

2026-03-15 重新校验后，原状态文件“16 个测试全部通过”的结论**不完整**：

- `test/braces.js` 实际有 **17** 个顶层 `it(...)` 测试
- 旧版 `braces.rs` 只有 **16** 个 Rust `#[test]`
- 缺失项是最后一个 JS 用例：
  - `should match special chars and expand ranges in parentheses`

现已补齐该测试，当前状态为：

- JS 顶层测试：`17/17`
- Rust 顶层测试：`17/17`
- 迁移状态：✅ 已完成

## 补齐的缺失测试

新增 Rust 测试：

- `should_match_special_chars_and_expand_ranges_in_parentheses`

迁移说明：

- 原 JS 用例依赖函数型 `expandRange` 回调和 `fill-range({ toRegex: true })`
- Rust 原生测试层不暴露 callback 版 `expandRange`
- 本仓库现有迁移约定已在 `options_expand_range.rs` 中采用“等价语义迁移”
- 因此这里直接使用 Rust 内建 brace range 展开验证相同匹配行为，而不是模拟 JS callback

这组断言覆盖了：

- `*/* {4..10}`
- `*/* \({4..10}\)`
- `*/* \({4..43}\)`
- `*/* \[{0..5}\]`

以及对应输入：

- `foo/bar - 1`
- `foo/bar - copy (1)`
- `foo/bar (1)`
- `foo/bar (4)`
- `foo/bar (7)`
- `foo/bar (42)`
- `foo/bar - copy [1]`
- `foo/bar - foo + bar - copy [1]`

## 验证结果

```bash
cargo test -p picomatch-rs --test braces
```

- 结果：`17 passed`, `0 failed`

```bash
node - <<'NODE'
const Module = require('module');
const origLoad = Module._load;
function fillRange(a, b, opts = {}) {
  if (!opts.toRegex) throw new Error('mock fill-range only supports toRegex');
  const isNum = /^-?\d+$/.test(String(a)) && /^-?\d+$/.test(String(b));
  if (isNum) {
    const vals = [];
    for (let i = Number(a); i <= Number(b); i++) vals.push(String(i));
    return vals.length === 1 ? vals[0] : `(?:${vals.join('|')})`;
  }
  return `[${String(a)}-${String(b)}]`;
}
Module._load = function(request, parent, isMain) {
  if (request === 'fill-range') return fillRange;
  return origLoad.apply(this, arguments);
};
const tests = [];
global.describe = (_name, fn) => fn();
global.it = (name, fn) => tests.push({ name, fn });
require('./test/braces.js');
let passed = 0;
for (const t of tests) {
  t.fn();
  passed++;
}
console.log(`js_harness_passed ${passed}/${tests.length}`);
NODE
```

- 结果：`js_harness_passed 17/17`
- 说明：当前工作树缺少 `node_modules`，因此使用了带 `fill-range` mock 的零依赖 harness 来验证原始 JS 测试文件

## 备注

1. `assert.deepStrictEqual(match(...))` 这类 JS 列表断言在 Rust 中继续使用 `assert_match_list(...)` 聚合验证，因此原始断言行数与 Rust 断言调用数不必逐行相等。
2. `expandRange` callback 本身仍是 JS 侧能力；Rust 这里验证的是与原 JS 用例等价的匹配语义。
