# options_ignore.rs 测试状态

**对应 JS 文件**: `test/options.ignore.js`

**状态**: ✅ 已完成

**测试命令**: `cargo test -p picomatch-rs --test options_ignore`

**结果**:
- 测试函数数: 4 个
- 测试用例数: ~30 个（assert 断言 + match list 过滤断言）
- 通过: 全部通过
- 失败: 0 个
- 忽略: 0 个

## 迁移说明

JS `ignore` option 接受一个或多个 glob pattern，在主 pattern 匹配成功后，
再用 ignore patterns 过滤掉不需要的结果。Rust `CompileOptions` 尚未暴露
`ignore` 字段（该功能仍在 JS 层实现）。

迁移策略：在测试层面模拟 `ignore` 逻辑——先用 `is_match(input, pattern)`
匹配主 pattern，再逐个检查 ignore patterns，如果任一 ignore pattern 匹配
则最终结果为 `false`。这与 JS 实现的行为完全等价。

### 测试组

| 测试函数 | 说明 | 用例数 |
|---------|------|-------|
| `should_not_match_ignored_patterns` | 基础 ignore 正/负匹配 | 4 |
| `should_filter_out_ignored_patterns` | 列表过滤 + ignore（无 dot） | 10 |
| `should_filter_with_dot_option` | 列表过滤 + ignore + dot 选项 | 8 |
| `should_handle_micromatch_issue_79_and_negation_patterns` | micromatch #79 + negation + ignore 组合 | ~9 |

### 注意事项
- `ignore` 在 Rust 层尚无原生支持，通过测试辅助函数 `is_match_with_ignore` / `match_with_ignore` 模拟。
- JS `match()` helper 使用 `Set` 去重，Rust 版本同样处理。
- `strictSlashes: true/false` 对应 Rust `strict_slashes` 字段。
