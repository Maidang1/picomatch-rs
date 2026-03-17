# options_format.rs 测试状态

**对应 JS 文件**: `test/options.format.js`

**状态**: ✅ 已完成

**测试命令**: `cargo test -p picomatch-rs --test options_format`

**结果**:
- 测试函数数: 2 个
- 测试用例数: ~40 个（isMatch 断言 + match list 过滤断言）
- 通过: 全部通过
- 失败: 0 个
- 忽略: 0 个

## 迁移说明

JS `format` option 接受一个回调 `(str) => string`，在匹配前对输入字符串进行变换。
本测试文件中的 format 函数为：
```js
str => str.replace(/\\/g, '/').replace(/^\.\//, '')
```
即：将反斜杠替换为正斜杠，并去除前导 `./`。

Rust 不暴露 `format` callback，但通过在调用 `is_match` 前手动对输入应用相同变换，
实现了完全等价的测试语义。

### 测试组

| 测试函数 | 说明 | 用例数 |
|---------|------|-------|
| `should_match_the_string_returned_by_options_format` | isMatch 断言（含负面 & 正面用例） | ~25 |
| `should_filter_fixtures_with_format` | match(fixtures, pattern) 列表过滤断言 | ~13 |

### 注意事项
- JS `match()` helper 使用 `Set` 去重，Rust 迁移版本也做了相同处理。
- `strictSlashes: true` 对应 Rust `strict_slashes: true`。
- `{ ...opts, windows: false }` 形式在 Rust 中通过单独构造 `CompileOptions` 实现。
