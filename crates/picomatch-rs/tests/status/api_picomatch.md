# api_picomatch.rs 测试状态

## 对应 JS 测试
`test/api.picomatch.js`

## 来源与落点
- JS 源文件：`/Users/felixwliu/codes/open-source/picomatch/test/api.picomatch.js`
- Native / 包入口实现：
  - `/Users/felixwliu/codes/open-source/picomatch/napi/src/lib.rs`
  - `/Users/felixwliu/codes/open-source/picomatch/index.js`
  - `/Users/felixwliu/codes/open-source/picomatch/posix.js`
  - `/Users/felixwliu/codes/open-source/picomatch/lib/picomatch.js`

## 最后修复时间
- 2026-03-18

## 测试用例数量
23 个（23 个通过，0 个 ignored）

## 测试覆盖范围
- validation - 无效参数验证
- multiple patterns - 多模式任意匹配
- file extensions - 文件扩展名匹配
- dot files - 点文件匹配（带/不带 dot 选项）
- escapes plus signs - 转义加号
- matches non-globs - 非 glob 模式
- matches file names - 文件名匹配
- matches common glob patterns - 常见 glob 模式
- matches wildcards - 通配符匹配
- matches globstars - globstar 匹配
- issue #23 - 边界情况
- issue #24 - 零或多目录匹配
- matches slashes - 斜杠匹配
- question marks not match slashes - 问号不匹配斜杠
- parse tokens - 解析 tokens
- state negatedExtglob - 否定扩展 glob 状态
- **issue#125, issue#100** - token output/value 字段（已实现并对齐 JS）

## 测试结果
- Node 原始 JS 用例：24 个通过，0 个失败
- Rust 对应迁移用例：23 个通过
- 0 个 ignored

## 备注
- Rust 侧已为 `parse()` 补充 `tokens` 与 `ParseToken.output`，`api_picomatch.rs` 现在直接校验 `parse().tokens`
- `test_parse_tokens_with_output_field` 已取消 `#[ignore]`，对应 JS `pictomatch issue#125, issue#100`
- 2026-03-18 修复了 3 个 Node parity 问题：
  - 顶层导出恢复为可调用 `picomatch(...)`
  - `matcher.state` 重新暴露 `negatedExtglob`
  - 空 pattern / `null` pattern 的报错文案对齐为 `Expected pattern to be a non-empty string`

## 验证命令

```bash
cargo test -p picomatch-rs --lib
cargo test -p picomatch-rs --test api_picomatch -- --nocapture
npm run mocha -- test/api.picomatch.js
```
