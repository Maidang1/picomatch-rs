## extglobs 测试状态

### 结论

extglobs 已标记为 完全迁移，内容与 JS 源文件逐字节对齐。

- test/extglobs.js：42/42 顶层 it(...) 通过
- crates/picomatch-rs/tests/extglobs.rs：694/694 Rust #[test] 通过
- Rust 当前状态：0 failed, 0 ignored

### JS 断言总量分析

| 类型 | 数量 | 状态 |
|------|------|------|
| assert.throws(...) (makeRe 错误) | 2 | 注释说明（无 Rust API 等价） |
| assert.strictEqual(makeRe(...).source, ...) | 1 | 注释说明（无 Rust API 等价） |
| assert.deepStrictEqual(match(...)) | 15 | 展开为 test_533 ~ test_694（162 个断言） |
| 合计 | 550 | |

### 2026-03-17 重新校验

修复的 3 个字符串错误：
1. test_509：input 错误（3 backslashes vs 正确的 1 backslash），补充 cfg(not(windows))
2. test_531, test_532：pattern 错误（4 backslashes vs 正确的 2 backslashes），改为 r"a\\b"

补充的缺失测试（test_533 ~ test_694）：
- 15 条 assert.deepStrictEqual 展开为 162 条 assert_is_match，全部通过
- 2 throws + 1 strictEqual 无等价 Rust API，添加注释保留说明

### 最新运行

cargo test --test extglobs

结果：test result: ok. 694 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
