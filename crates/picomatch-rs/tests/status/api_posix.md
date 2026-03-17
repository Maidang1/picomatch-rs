# api_posix.rs Status

## Source
- JS Test: `test/api.posix.js`

## Test Cases
1. `should_use_posix_paths_only_by_default` - Verifies POSIX paths work, Windows paths fail by default
2. `should_still_be_manually_configurable_to_accept_non_posix_paths` - Verifies Windows paths work when `windows: true`

## Status
- ✅ All tests passing (2/2)
