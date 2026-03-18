# options.rs 测试状态

**对应 JS 文件**: `test/options.js`

**状态**: ✅ 已完成

**测试命令**: `cargo test -p picomatch-rs --test options`

**结果**:
- 测试函数数: 9 个
- 测试用例数: 约 80 个
- 通过: 全部通过
- 失败: 0 个

## 迁移说明

对 `test/options.js` 进行了 Rust 直译，涵盖了 `matchBase`, `flags`, `nocase`, `noextglob`, `unescape`, `nonegate`, `windows` 等选项。

### 实现差异与注释

1. **`options.unescape`**:
   - `windows: true` 且 `unescape: true` 时，模式 `\\a\\b\\c` 现在同时兼容：
     - `abc`
     - `/a/b/c`
   - Rust `options.rs` 内部辅助函数已改成与 JS `test/support/match.js` 一样做去重，避免 `\\a\\b\\c` fixture 归一化后重复返回 `/a/b/c`。

2. **`options.format`**:
   - JS 中的 `format` 回调选项在 Rust 中通过手动处理输入字符串（如 `strip_prefix("./")` 和 `replace('\\', "/")`）来模拟，以达到等效的测试效果。
   - 在 `options_windows_strip_leading_dot_slash` 测试中体现了这一点。

### 测试组

- `options_match_base`: 验证 `matchBase` 对 basename 的匹配支持。
- `options_flags`: 验证 `flags: 'i'` 的大小写不敏感支持。
- `options_nocase`: 验证 `nocase: true` 选项。
- `options_noextglob`: 验证禁用 extglob 后的行为，包括将特殊字符视为字面量。
- `options_unescape`: 验证反斜杠转义逻辑。
- `options_nonegate`: 验证 `nonegate` 禁用取反逻辑。
- `options_windows`: 验证 Windows 路径处理（反斜杠转正斜杠）。
- `options_windows_strip_leading_dot_slash`: 验证处理前导 `./` 的逻辑（模拟 `format` 选项）。
- `windows_paths`: 验证路径分隔符转换。
