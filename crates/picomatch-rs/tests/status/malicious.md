# malicious.rs 测试状态

## 测试结果
- ✅ 全部通过

## 统计信息
- 测试用例数: 2
- 通过: 2
- 忽略: 0
- 失败: 0

## 迁移情况与差异说明
从 `test/malicious.js` 迁移了防备正则拒绝服务的测试，考虑到语言运行时的机制差异，做出以下调整：
- ** maxLength 限制**: N-API 层已经按 JS 语义默认限制到 65536 字节，并支持 `maxLength` 自定义上限。
- ** long backslash sequences**: `test_long_escape_sequences` 已恢复为正常测试，不再 `#[ignore]`。
- ** invalid patterns fail closed**: Rust 核心仍会把部分不支持的畸形模式视为 `UnsupportedPattern`；Node N-API parity 路径则在 `isMatch` 上返回 `false`，与 JS 用例保持一致。
- 迁移了原生的 `constructor`、`__proto__` 关键词属性作为普通字符串输入匹配校验能够正确通过。

## 最近修复
- 在 `crates/picomatch-rs/src/compile.rs` 中加入了长反斜杠序列压缩逻辑，避免超长 `\\\\...\\\\A` 模式在编译期展开失控
- 在 `napi/src/lib.rs` 中将 unsupported pattern 的 `isMatch` 行为调整为 fail-closed `false`

## 验证命令

```bash
cargo test -p picomatch-rs --test malicious
npm run mocha -- test/malicious.js
```
