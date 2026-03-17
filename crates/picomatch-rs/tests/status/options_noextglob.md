# options_noextglob.rs 测试状态

**对应 JS 文件**: `test/options.noextglob.js`

**状态**: ✅ 已完成

**测试命令**: `cargo test -p picomatch-rs --test options_noextglob`

**结果**:
- 测试函数数: 2 个
- 测试用例数: 8 个
- 通过: 全部通过
- 失败: 0 个

## 迁移说明

迁移了 `test/options.noextglob.js` 中的所有测试用例。

### 特点

- **`options.noextglob`**: 验证了当此选项为 `true` 时，extglob 语法（如 `+(z)`）被视为普通字符串，其中 `+` 会被转义或作为字面量处理。
- **`options.noext` 别名**: 在 Rust 的 `CompileOptions` 中并没有 `noext` 字段（它是 JS 层的 alias），因此测试中通过 `noextglob` 字段来验证功能一致性。
