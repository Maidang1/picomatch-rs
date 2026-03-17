# options_noglobstar.rs 测试状态

**对应 JS 文件**: `test/options.noglobstar.js`

**状态**: ✅ 已完成

**测试命令**: `cargo test -p picomatch-rs --test options_noglobstar`

**结果**:
- 测试函数数: 1 个
- 测试用例数: 4 个
- 通过: 全部通过
- 失败: 0 个

## 迁移说明

迁移了 `test/options.noglobstar.js` 中的测试用例。

### 特点

- **`options.noglobstar`**: 验证了当此选项为 `true` 时，`**` 将不再具有匹配多个目录层级的特殊语义（即不再作为 globstar 处理）。
- JS 原测试文件中的 description 标注为 "should disable extglob support"，显然是笔误，已在 Rust 测试中更正为 globstar。
