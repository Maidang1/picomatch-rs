## extglobs_temp Test Status

### 复核结论

- `test/extglobs-temp.js` 共有 **11** 个顶层 `it(...)`。
- `crates/picomatch-rs/tests/extglobs_temp.rs` 将这些矩阵断言拆成了 **1052** 个 Rust `#[test]`。
- 当前状态已经不是“1051 passed + 1 failed/ignored”。
- 现在的准确结果是：**1052 passed, 0 failed, 0 ignored**。

### 本轮修正

之前状态文件提到的 `test_844` 已不应继续标记为失败/忽略项。问题不是 runtime 仍然不支持，而是 `extglobs_temp.rs` 里这一条测试本身的 Rust 字符串多转义了一层：

- 错误的 Rust pattern：`"/dev\\\\/@(tcp|udp)\\\\/*\\\\/*"`
- 对齐原始 JS 后的 pattern：`"/dev\\/@(tcp|udp)\\/*\\/*"`

同时已移除 `#[ignore = "known upstream failure matching bash_windows_options test 8"]`。

### 验证结果

已执行：

```bash
cargo test -p picomatch-rs --test extglobs_temp
node - <<'NODE'
const tests = [];
global.describe = (_name, fn) => fn();
global.it = (name, fn) => tests.push({ name, fn });
require('./test/extglobs-temp.js');
let passed = 0;
for (const t of tests) {
  t.fn();
  passed++;
}
console.log(`js_harness_passed ${passed}/${tests.length}`);
NODE
```

结果：

- Rust: `1052 passed; 0 failed; 0 ignored`
- JS harness: `11/11 passed`

### 当前准确状态

`extglobs_temp` 现在应标记为 **完整迁移且测试正确**。
