# scan_api.rs 测试覆盖状态

## 测试文件
- 源文件: `test/api.scan.js`
- 目标文件: `crates/picomatch-rs/tests/scan_api.rs`

## 测试覆盖

### 测试函数 (12)

| 测试函数 | 覆盖的 JS 测试语义 |
|---------|------------------|
| `scan_detects_basic_shape_and_flags` | `.scan` 基础测试：prefix/start/base/glob/flags 检测 |
| `scan_strips_glob_magic_to_compute_base_paths` | `.base` glob magic 剥离测试 |
| `scan_respects_escaped_characters_and_plain_paths` | escaped characters 测试 |
| `scan_returns_documented_parts` | parts 分割测试 |
| `scan_matches_glob2base_style_cases` | glob2base 风格测试 |
| `scan_parses_glob_base_examples` | glob base 解析测试 |
| `scan_handles_character_classes_qmarks_non_globs_and_braces` | qmarks / regex classes / braces / non-globs 测试 |
| `scans_basic_glob` | 原 `scan.rs` 内嵌测试：基础 glob 状态断言 |
| `scans_negated_extglob` | 原 `scan.rs` 内嵌测试：negated extglob 状态 |
| `scans_parts_with_prefix` | 原 `scan.rs` 内嵌测试：parts/slashes/prefix 行为 |
| `unescapes_escaped_braces` | 原 `scan.rs` 内嵌测试：`unescape` 下的 escaped braces |
| `preserves_backslashes_inside_literal_brackets` | 原 `scan.rs` 内嵌测试：literal brackets 中保留反斜杠 |

### 覆盖的测试类别

1. **`.scan` 基础测试**
   - prefix/start/base/glob 检测
   - isBrace/isBracket/isGlob/isGlobstar/isExtglob/negated/negatedExtglob flags

2. **`.base` (glob2base) 测试**
   - glob magic 剥离
   - brace 嵌套处理
   - escaped characters

3. **`parts` 测试**
   - pattern 分割
   - slash 位置记录

4. **braces 测试**
   - brace sets: `{a,b}`, `{foo,bar}`
   - brace ranges: `{0..9}`, `{a..c}`
   - brace enclosures with embedded separators: `{,/,bar/baz,qux}`
   - escaped braces: `\{foo,bar\}`, `\{../,./,\{bar,/baz},qux}`
   - complex brace globs: `{a/b/{c,/foo.js}/e.f.g}`

5. **qmarks 测试**
   - question mark 语义
   - 路径边界处理

6. **regex character classes 测试**
   - `[a-c]b*`, `[a-j]*[^c]`
   - unescape 模式下的 bracket 处理

7. **non-glob patterns 测试**
   - 普通文件路径
   - 空白路径

8. **Windows backslash 路径测试**
   - `C:\path\*.js`
   - `C:\\path\\*.js`

9. **negation / extglob 测试**
   - leading `!` negation
   - `!(foo)` negated extglob
   - `@(foo)`, `+(foo)`, `*(foo)`, `?(foo)` extglobs

## 测试结果

```
cargo test -p picomatch-rs --test scan_api
```

- JS 源文件 `test/api.scan.js` 当前共有 40 个 `it(...)` 用例
- 测试数量: 12 个测试函数
- 结果: 全部通过

## 迁移状态

✅ 已完成 - `test/api.scan.js` → `scan_api.rs`

## 审核说明

- 2026-03-15 再次核对：`test/api.scan.js` 的 40 个 JS 用例已经按主题聚合进 `scan_api.rs` 的 7 个 Rust 测试函数中
- 当前未发现缺失、忽略或未标注的 `api.scan.js` 用例
- 原 `scan.rs` 的 5 个内嵌单元测试现已并入同一文件，`scan_internal.rs` 已删除
