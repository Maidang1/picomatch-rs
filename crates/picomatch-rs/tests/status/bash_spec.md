# bash_spec.rs 测试状态

## 对应测试文件

- 源文件：`test/bash.spec.js`
- 目标文件：`crates/picomatch-rs/tests/bash_spec.rs`

## 迁移完整性

- 已逐项核对：`test/bash.spec.js` 的 111 条 `assert(isMatch(...))` / `assert(!isMatch(...))` 断言都已迁入 `bash_spec.rs`
- 覆盖分组：
  - `dotglob`
  - `glob`
  - `globstar`

## 最新验证

```bash
cargo test -p picomatch-rs --test bash_spec
```

- 结果：`3 passed`, `0 failed`
- 通过：`dotglob_tests`、`glob_tests`、`globstar_tests`

## 修复分析

- 根因在 `crates/picomatch-rs/src/compile.rs` 对 bash 模式的 `/**` 尾部 globstar 编译过宽/过窄不一致：
  - 旧行为会错误拒绝 `.x/.x`
  - 在放宽后又会错误接受 `a/.x/b/.x/c`
- 已改为对“前面已有字面量段、后面是结尾 `/**`”的 bash 场景使用与 JS 一致的 tempered tail 语义，因此两条关键断言都已对齐：
  - `.x/.x` vs `**/.x/**` => `true`
  - `a/.x/b/.x/c` vs `**/.x/**` => `false`

## 备注

- 旧记录里“源文件是 `test/bash.js`”和“测试文件内部存在矛盾断言”的结论都已失效。
- 当前问题已经修复；失败原因不是迁移缺失，而是 Rust 对 bash globstar 尾部语义的实现曾与 JS 不一致。
- 状态更新时间：2026-03-15
