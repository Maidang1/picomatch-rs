# dotfiles.rs 测试状态

## 对应关系

- 源文件: `test/dotfiles.js`
- 目标文件: `crates/picomatch-rs/tests/dotfiles.rs`

## 复核结论

2026-03-15 重新校验后，`dotfiles` 这组测试已经**完整迁移**，状态文件原结论基本正确。

- JS 顶层测试：`23`
- Rust 顶层测试：`23`
- JS 原始断言行数：`210`
- Rust 断言调用数：`123`

说明：

- JS 中有大量重复/批量断言
- Rust 侧将这类矩阵场景折叠为 `assert_match_list(...)`、`for` 循环和共享 fixture
- 因此断言调用行数不与 JS 一一相等，但顶层测试语义已完整覆盖

顶层测试一一对应，包含：

- 默认不匹配 dotfiles
- leading dot / explicit dot pattern
- `dot` 选项开关
- negation 与 extglob negation
- single star / globstar 对 dotfiles 的处理
- `.` / `..` 的显式与非显式匹配
- root path 上 `**/` 前缀的 leading dot 行为
- bracket 中显式 `.` 的匹配

迁移状态：✅ 已完成

## 验证结果

```bash
cargo test -p picomatch-rs --test dotfiles
```

- 结果：`23 passed`, `0 failed`

```bash
node - <<'NODE'
const tests = [];
global.describe = (_name, fn) => fn();
global.it = (name, fn) => tests.push({ name, fn });
require('./test/dotfiles.js');
let passed = 0;
for (const t of tests) {
  t.fn();
  passed++;
}
console.log(`js_harness_passed ${passed}/${tests.length}`);
NODE
```

- 结果：`js_harness_passed 23/23`

## 备注

1. `test/dotfiles.js` 里的两个 `async` 用例本身没有异步断言逻辑，零依赖 harness 可直接同步执行。
2. Rust 侧保留了原 JS 的重复语义，但通过 `for` 循环和聚合 matcher 断言减少了样板代码，所以不要用“断言行数更少”误判为漏迁。
3. 当前没有发现缺失项、错误期待值或状态漂移。
