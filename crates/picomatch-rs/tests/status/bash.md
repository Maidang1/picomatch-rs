# bash.rs 测试覆盖状态

## 测试文件

- 源文件: `test/bash.js`
- 目标文件: `crates/picomatch-rs/tests/bash.rs`

## 测试覆盖

### 顶层测试函数 (12/12)

`crates/picomatch-rs/tests/bash.rs` 已保留并对齐 `test/bash.js` 的 12 个顶层 `it(...)` 用例。

### 断言覆盖状态

- JS 原始断言数：`645`
- JS 去重后的唯一断言语义：`623`
- Rust 已覆盖的唯一断言语义：`623`
- Rust 额外显式断言：`1`
  - `assert_is_match("a*", "[a-y]*[^c]", opts.clone(), true);`
  - 说明：这条不是 `test/bash.js` 的原始断言，但它记录了当前 JS/Rust 在默认选项下的一致行为，避免后续状态再次漂移

### 已补齐的缺失迁移

1. `should_match_escaped_quotes`
   - 补齐 `foo/\"**\"/bar`、`foo/\"*\"/bar`、`foo/"*"/bar` 的完整负例矩阵
2. `should_support_character_classes`
   - 补齐 `[a-y]*[^c]` 的默认 / `bash: true` / `regex: true` 断言
   - 补齐 `a[b]c`、`a["b"]c`、`a[\\\\b]c`、`a[\\b]c`、`a[b-d]c`、`a?c` 的完整样例矩阵
   - 补齐 `[^a-c]*` 的遗漏负例 `a/*`
3. `should_match_escaped_characters`
   - 补齐非 Windows 条件分支：`\\*` 与 `[A-Z]+/\\\\`

### 仍保留的结构差异

Rust 侧为了可读性没有强制复制 JS 里的重复断言行，因此“原始行数”仍不与 `test/bash.js` 完全相同；但唯一断言语义现已完全覆盖。

### 覆盖的测试类别

1. **Regular Globbing**
   - `a*` 匹配 `a`, `ab`, `abc`
   - `\a*` 转义模式
2. **Directory Matching**
   - `b*/` 尾随斜杠匹配目录
3. **Escaped Characters as Literals**
   - `\^` 匹配 `^`
   - `\*` 匹配 `*`
   - `a\*` 匹配 `a*`
4. **Quoted Characters**
   - 双引号 `"***"` 匹配 `***`
   - 单引号 `'***'` 匹配 `'***'`
   - `"*"*` 内部通配符
5. **Escaped Quotes**
   - `\"**\"` 匹配 `**`
   - `foo/\"*\"/bar` 匹配 `foo/"*"/bar`
6. **Character Classes**
   - 范围: `[a-c]`, `[a-y]`, `[a-z]`
   - 否定: `[^c]`, `[^a-c]`
   - 特殊字符: `[X-]`
7. **Basic Wildmatch Brackets**
   - `a[]-]b` 匹配 `a-b`, `a]b`
   - `[ten]` 字符集
   - `[\\]a\\-]` 转义
8. **Extended Slash-Matching**
   - `foo[/]` 斜杠在括号内
9. **Escaped Characters**
   - `\[ab]` 字面量括号
10. **Star Consolidation**
    - 多余星号合并: `**`, `***`, `*****?`

## 测试结果

```bash
cargo test -p picomatch-rs --test bash
```

- 结果：`12 passed`, `0 failed`

```bash
node - <<'NODE'
const tests = [];
global.describe = (_name, fn) => fn();
global.it = (name, fn) => tests.push({ name, fn });
require('./test/bash.js');
let passed = 0;
for (const t of tests) {
  t.fn();
  passed++;
}
console.log(`js_harness_passed ${passed}/${tests.length}`);
NODE
```

- 结果：`js_harness_passed 12/12`

## 迁移状态

✅ 已完成（按唯一断言语义完全迁移）- `test/bash.js` → `bash.rs`

## 备注

1. 2026-03-15 重新复核后，确认原状态文件“已完成”结论过于乐观：当时 `bash.rs` 尚缺 `escaped quotes`、`character classes`、非 Windows `escaped characters` 的一批断言。现已补齐并通过复测。
2. 2026-03-15 额外保留了 `a*` 对 `[a-y]*[^c]` 的默认选项显式断言：
   - 当前 JS 行为: `true`
   - 当前 Rust 行为: `true`
   - 该断言用于固定当前实现行为，即使它不是原始 `test/bash.js` 里的独立断言
