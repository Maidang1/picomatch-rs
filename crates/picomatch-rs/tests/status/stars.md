# stars.rs — Migration Status

Source: `test/stars.js`
Rust test: `tests/stars.rs`
Status: **17/17 tests passing** ✅

## Sections migrated

| JS `it(...)` description | Rust test function | Status |
|---|---|---|
| issue related: respect dots in glob pattern | `respects_dots_defined_in_glob_pattern` | ✅ |
| should match anything except slashes and leading dots | `single_stars_match_anything_except_slashes_and_leading_dots` | ✅ |
| should match spaces | `single_stars_match_spaces` | ✅ |
| should support multiple non-consecutive stars in a path segment | `supports_multiple_non_consecutive_stars_in_a_path_segment` | ✅ |
| should support multiple stars in a segment | `supports_multiple_stars_in_a_segment` | ✅ |
| should return true when one of the given patterns matches | `returns_true_when_one_of_the_given_patterns_matches_the_string` | ✅ |
| should return false when the path does not match the pattern | `returns_false_when_the_path_does_not_match_the_pattern` | ✅ |
| should match a path segment for each single star | `matches_a_path_segment_for_each_single_star` | ✅ |
| should support single globs (*) | `supports_single_globs` | ✅ |
| should only match a single folder per star when globstars are used | `only_matches_a_single_folder_per_star_when_globstars_are_used` | ✅ |
| should not match a trailing slash when a star is last char | `does_not_match_a_trailing_slash_when_a_star_is_last_char` | ✅ |
| should work with file extensions | `works_with_file_extensions` | ✅ |
| should not match slashes when globstars are not exclusive | `does_not_match_slashes_when_globstars_are_not_exclusive_in_a_path_segment` | ✅ |
| should match slashes when defined in braces | `matches_slashes_when_defined_in_braces` | ✅ |
| should correctly match slashes | `correctly_matches_slashes` | ✅ |
| should ignore leading "./" when defined on pattern | `ignores_leading_dot_slash_when_defined_on_pattern` | ✅ |
| should optionally match trailing slashes with braces | `optionally_matches_trailing_slashes_with_braces` | ✅ |

## Notes

- All 263 JS single-pattern assertions are covered. Some are structured as `for input in [...]` loops
  with the pattern extracted outside the loop, so a naive text search may appear to miss them —
  they are fully tested.
- `{ strictSlashes: true }` option tests handled via dedicated `CompileOptions { strict_slashes: true, .. }`.
- Array-pattern JS calls (`isMatch(str, ['pat'])`) mapped to `assert_is_match_any`.
