# regex_features.rs 测试状态

## 对应 JS 测试
`test/regex-features.js`

## 测试用例数量
JS 文件共 26 个 `it(...)` 块；Rust 集成测试 25 个 + 1 个内部单元测试

## 测试覆盖范围
- word boundaries (`\b`)
- regex lookarounds (`(?<=...)`、`(?<!...)`)
- regex backreferences (`\1`)
- character classes
- negated character classes
- character class ranges
- regex capture groups
- regex non-capture groups
- regex quantifiers
- extglobs with regex quantifiers
- basename paths（`src/matcher.rs::tests::should_basename_paths`，因 `basename` 为私有函数）

## 测试结果
全部通过 ✅（25 集成测试 + 1 内部单元测试）

## 备注
- `nobracket` 选项测试已覆盖
- `unescape` 选项测试已覆盖
- `should basename paths` 迁移至 `src/matcher.rs` 内部测试模块（`basename` 是私有函数，无法从集成测试访问）
