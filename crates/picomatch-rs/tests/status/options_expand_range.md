# options_expand_range.rs 测试状态

**对应 JS 文件**: `test/options.expandRange.js`

**状态**: ✅ 已完成

**测试命令**: `cargo test -p picomatch-rs --test options_expand_range`

**结果**:
- 测试函数数: 1 个
- 测试用例数: 3 个（assert_is_match 调用）
- 通过: 3 个
- 失败: 0 个
- 忽略: 0 个

## 迁移说明

JS 原始测试使用了 `expandRange` 回调选项（接受 `(a, b) => string` 函数），
用于自定义 `{a..c}` / `{1..100}` 等 brace range 的 regex 编译输出。
Rust 原生编译器不暴露该回调，但已内置等价的字母/数字 range 展开逻辑。

### 用例对应关系

| JS 用例 | 测试输入 | 测试 pattern | JS expandRange 输出 | Rust 原生输出 | 是否等价 |
|---------|---------|-------------|-------------------|--------------|---------|
| assert(isMatch(...)) | `a/c` | `a/{a..c}` | `([a-c])` | `[a-c]` | ✅ |
| assert(!isMatch(...)) | `a/z` | `a/{a..c}` | `([a-c])` | `[a-c]` | ✅ |
| assert(isMatch(...)) | `a/99` | `a/{1..100}` | fill-range toRegex | `(?:1\|2\|...\|100)` | ✅ |

- 测试 1 & 2：`{a..c}` 的自定义 expandRange 生成 `([a-c])`，Rust 原生生成 `[a-c]`，匹配字符集相同。
- 测试 3：`{1..100}` 的 fill-range toRegex 生成大型数字交替 regex，Rust 原生生成 `(?:1|2|...|100)`，均可匹配 `99`。

所有测试用例均按等价语义直译，无需注释或跳过。
