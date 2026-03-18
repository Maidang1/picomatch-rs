# posix_classes.rs 测试状态

## 对应 JS 测试
`test/posix-classes.js`

## 来源与落点
- 上游原始 JS 源文件（迁移来源记录）：`/Users/felixwliu/codes/open-source/picomatch-js/test/posix-classes.js`
- 当前仓库迁移副本：`/Users/bytedance/codes/open-source/picomatch-rs-v1/test/posix-classes.js`
- Rust 测试文件：`/Users/bytedance/codes/open-source/picomatch-rs-v1/crates/picomatch-rs/tests/posix_classes.rs`
- Rust 实现文件：`/Users/bytedance/codes/open-source/picomatch-rs-v1/crates/picomatch-rs/src/compile.rs`

## 最后修复时间
- 2026-03-18

## 当前状态
- done
- `test/posix-classes.js` 与 `crates/picomatch-rs/tests/posix_classes.rs` 当前都已通过。

## 记录修正
- 将“原 JS 文件位置”从当前仓库的迁移副本改回上游迁移来源，避免把镜像文件误记为原始来源。
- 明确补充当前仓库内的迁移副本路径，方便本地对照。
- 将 Rust 测试文件与 Rust 实现文件分别单列，避免之前的“Output Rust file”描述过于笼统。

## 本次修复
- `CompileOptions.posix` 下的 `[:punct:]` 现在保留 JS 兼容的 `parse().output` 形式：`@[\\]^`
- 仅在生成可执行 regex source 时，对 `[:punct:]` 片段做额外转义，避免 `fancy_regex` 将内部 `[` 判为非法字符类
- 这样同时满足了：
  - Node `parse()` 输出断言
  - Rust / Node 运行期 regex 编译
  - 畸形输入 `[abc[:punct:][0-9]` 的兼容行为

## 最近验证

```bash
pnpm run test
```

- 结果：通过
- Rust: `crates/picomatch-rs/tests/posix_classes.rs` 11/11 通过
- Node: `test/posix-classes.js` 全部通过
