# minimatch.rs 测试状态

## 测试结果
- ✅ 全部通过

## 统计信息
- 测试函数数: 8
- 测试用例数: 29
- 通过: 29
- 失败: 0
- 忽略: 0

## 迁移说明
从 `test/minimatch.js` 迁移了共 8 个 Minimatch parity 测试（基于 issue 列表：29, 30, 50, 67, 75, 78, 82, 83）。
- JavaScript 中的 `{ format }` 函数配置，在 Rust 测试中以手动在传给 `assert_is_match` 之前 stripping 前缀 `./` 实现（`strip_prefix("./")`）。
- 测试通过结果一致，所有 pattern 的匹配表现已全量覆盖。
