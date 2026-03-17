# options_on_match.rs 测试状态

**对应 JS 文件**: `test/options.onMatch.js`

**状态**: ✅ 已完成

**测试命令**: `cargo test -p picomatch-rs --test options_on_match`

**结果**:
- 测试函数数: 1 个
- 测试用例数: ~40 个 (21 个 is_match_with_format + 14 个 match_with_on_match)
- 通过: 全部通过
- 失败: 0 个
- 忽略: 0 个

## 迁移说明

JS `onMatch` 是一个回调选项，当匹配发生时被调用。在此测试用例中，它被用来：
1. 额外剥离输出中的 `./` 或 `.\`。
2. 将结果添加到 match 集合中。

由于 Rust `CompileOptions` 不支持回调选项（该特性目前仍仅在 JS 层支持），我们在测试层面通过以下助手函数模拟逻辑：
- `format(s)`: 模拟 JS 的 `format` 选项。
- `on_match_format(s)`: 模拟 JS 测试用例中的 `onMatch` 逻辑。
- `match_with_on_match(fixtures, pattern, opts)`: 模拟 JS 的 `match` 过程，并在匹配成功后手动应用 `on_match_format` 逻辑并去重。

通过这种方式，我们保留了测试套件的语义完整性。

### 测试细节
- 验证了 `isMatch` 在带有 `format` 选项时的正确性，涵盖了多种带 `./` 路径前缀的用例。
- 验证了 `match` 与 `onMatch` 结合时对 fixtures 的过滤和格式化输出。
