mod support;

use picomatch_rs::CompileOptions;

use support::{assert_is_match, default_compile_options};

/// Double dots tests - no options
mod double_dots_no_options {
    use super::*;

    #[test]
    fn should_not_match_leading_double_dots_with_single_star() {
        for (input, pattern) in [
            ("../abc", "*/*"),
            ("../abc", "*/abc"),
            ("../abc", "*/abc/*"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_dot_single_star() {
        for (input, pattern) in [
            ("../abc", ".*/*"),
            ("../abc", ".*/abc"),
            ("../abc", "*./*"),
            ("../abc", "*./abc"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_globstar() {
        for (input, pattern) in [
            ("../abc", "**"),
            ("../abc", "**/**"),
            ("../abc", "**/**/**"),
            ("../abc", "**/abc"),
            ("../abc", "**/abc/**"),
            ("../abc", "abc/**"),
            ("../abc", "abc/**/**"),
            ("../abc", "abc/**/**/**"),
            ("../abc", "**/abc"),
            ("../abc", "**/abc/**"),
            ("../abc", "**/abc/**/**"),
            ("../abc", "**/**/abc/**"),
            ("../abc", "**/**/abc/**/**"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("../abc", ".**"),
            ("../abc", ".**/**"),
            ("../abc", ".**/abc"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [("../abc", "*.*/**"), ("../abc", "*.*/abc")] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_globstar_dot() {
        for (input, pattern) in [("../abc", "**./**"), ("../abc", "**./abc")] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_star() {
        for (input, pattern) in [
            ("/../abc", "*/*"),
            ("/../abc", "/*/*"),
            ("/../abc", "*/*/*"),
            ("abc/../abc", "*/*/*"),
            ("abc/../abc/abc", "*/*/*/*"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_dot_star() {
        for (input, pattern) in [
            ("/../abc", "*/.*/*"),
            ("/../abc", "/.*/*"),
            ("/../abc", "*/*.*/*"),
            ("/../abc", "/*.*/*"),
            ("/../abc", "*/*./*"),
            ("/../abc", "/*./*"),
            ("abc/../abc", "*/.*/*"),
            ("abc/../abc", "*/*.*/*"),
            ("abc/../abc", "*/*./*"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_globstar() {
        for (input, pattern) in [
            ("/../abc", "**"),
            ("/../abc", "**/**"),
            ("/../abc", "/**/**"),
            ("/../abc", "**/**/**"),
            ("abc/../abc", "**/**/**"),
            ("abc/../abc/abc", "**/**/**/**"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("/../abc", "**/.**/**"),
            ("/../abc", "/.**/**"),
            ("abc/../abc", "**/.**/**"),
            ("abc/../abc", "/.**/**"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_globstar_dot() {
        for (input, pattern) in [
            ("/../abc", "**/**./**"),
            ("/../abc", "/**./**"),
            ("abc/../abc", "**/**./**"),
            ("abc/../abc", "/**./**"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [
            ("/../abc", "**/**.**/**"),
            ("/../abc", "**/*.*/**"),
            ("/../abc", "/**.**/**"),
            ("/../abc", "/*.*/**"),
            ("abc/../abc", "**/**.**/**"),
            ("abc/../abc", "**/*.*/**"),
            ("abc/../abc", "/**.**/**"),
            ("abc/../abc", "/*.*/**"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_single_star() {
        for (input, pattern) in [
            ("abc/..", "*/*"),
            ("abc/..", "*/*/"),
            ("abc/..", "*/*/*"),
            ("abc/../", "*/*"),
            ("abc/../", "*/*/"),
            ("abc/../", "*/*/*"),
            ("abc/../abc/../", "*/*/*/*"),
            ("abc/../abc/../", "*/*/*/*/"),
            ("abc/../abc/abc/../", "*/*/*/*/*"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_dot_star() {
        for (input, pattern) in [
            ("abc/..", "*/.*"),
            ("abc/..", "*/.*/"),
            ("abc/..", "*/.*/*"),
            ("abc/../", "*/.*"),
            ("abc/../", "*/.*/"),
            ("abc/../", "*/.*/*"),
            ("abc/../abc/../", "*/.*/*/.*"),
            ("abc/../abc/../", "*/.*/*/.*/"),
            ("abc/../abc/abc/../", "*/.*/*/.*/*"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_star_dot() {
        for (input, pattern) in [
            ("abc/..", "*/*."),
            ("abc/..", "*/*./"),
            ("abc/..", "*/*./*"),
            ("abc/../", "*/*."),
            ("abc/../", "*/*./"),
            ("abc/../", "*/*./*"),
            ("abc/../abc/../", "*/*./*/*."),
            ("abc/../abc/../", "*/*./*/*./"),
            ("abc/../abc/abc/../", "*/*./*/*./*"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_globstar() {
        for (input, pattern) in [
            ("abc/..", "**/**"),
            ("abc/..", "**/**/"),
            ("abc/..", "**/**/**"),
            ("abc/../", "**/**"),
            ("abc/../", "**/**/"),
            ("abc/../", "**/**/**"),
            ("abc/../abc/../", "**/**/**/**"),
            ("abc/../abc/../", "**/**/**/**/"),
            ("abc/../abc/abc/../", "**/**/**/**/**"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("abc/..", "**/.**"),
            ("abc/..", "**/.**/"),
            ("abc/..", "**/.**/**"),
            ("abc/../", "**/.**"),
            ("abc/../", "**/.**/"),
            ("abc/../", "**/.**/**"),
            ("abc/../abc/../", "**/.**/**/.**"),
            ("abc/../abc/../", "**/.**/**/.**/"),
            ("abc/../abc/abc/../", "**/.**/**/.**/**"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [
            ("abc/..", "**/**.**"),
            ("abc/..", "**/**.**/"),
            ("abc/..", "**/**.**/**"),
            ("abc/../", "**/**.**"),
            ("abc/../", "**/**.**/"),
            ("abc/../", "**/**.**/**"),
            ("abc/../abc/../", "**/**.**/**/**.**"),
            ("abc/../abc/../", "**/**.**/**/**.**/"),
            ("abc/../abc/abc/../", "**/**.**/**/.**/**"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_globstar_dot() {
        for (input, pattern) in [
            ("abc/..", "**/**."),
            ("abc/..", "**/**./"),
            ("abc/..", "**/**./**"),
            ("abc/../", "**/**."),
            ("abc/../", "**/**./"),
            ("abc/../", "**/**./**"),
            ("abc/../abc/../", "**/**./**/**."),
            ("abc/../abc/../", "**/**./**/**./"),
            ("abc/../abc/abc/../", "**/**./**/**./**"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }
}

/// Double dots tests - dot: true
mod double_dots_dot_true {
    use super::*;

    fn opts() -> CompileOptions {
        CompileOptions {
            dot: true,
            ..CompileOptions::default()
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_single_star() {
        for (input, pattern) in [
            ("../abc", "*/*"),
            ("../abc", "*/abc"),
            ("../abc", "*/abc/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_dot_single_star() {
        for (input, pattern) in [
            ("../abc", ".*/*"),
            ("../abc", ".*/abc"),
            ("../abc", "*./*"),
            ("../abc", "*./abc"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_globstar() {
        for (input, pattern) in [
            ("../abc", "**"),
            ("../abc", "**/**"),
            ("../abc", "**/**/**"),
            ("../abc", "**/abc"),
            ("../abc", "**/abc/**"),
            ("../abc", "abc/**"),
            ("../abc", "abc/**/**"),
            ("../abc", "abc/**/**/**"),
            ("../abc", "**/abc"),
            ("../abc", "**/abc/**"),
            ("../abc", "**/abc/**/**"),
            ("../abc", "**/**/abc/**"),
            ("../abc", "**/**/abc/**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("../abc", ".**"),
            ("../abc", ".**/**"),
            ("../abc", ".**/abc"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [("../abc", "*.*/**"), ("../abc", "*.*/abc")] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_globstar_dot() {
        for (input, pattern) in [("../abc", "**./**"), ("../abc", "**./abc")] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_star() {
        for (input, pattern) in [
            ("/../abc", "*/*"),
            ("/../abc", "/*/*"),
            ("/../abc", "*/*/*"),
            ("abc/../abc", "*/*/*"),
            ("abc/../abc/abc", "*/*/*/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_dot_star() {
        for (input, pattern) in [
            ("/../abc", "*/.*/*"),
            ("/../abc", "/.*/*"),
            ("/../abc", "*/*.*/*"),
            ("/../abc", "/*.*/*"),
            ("/../abc", "*/*./*"),
            ("/../abc", "/*./*"),
            ("abc/../abc", "*/.*/*"),
            ("abc/../abc", "*/*.*/*"),
            ("abc/../abc", "*/*./*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_globstar() {
        for (input, pattern) in [
            ("/../abc", "**"),
            ("/../abc", "**/**"),
            ("/../abc", "/**/**"),
            ("/../abc", "**/**/**"),
            ("abc/../abc", "**/**/**"),
            ("abc/../abc/abc", "**/**/**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("/../abc", "**/.**/**"),
            ("/../abc", "/.**/**"),
            ("abc/../abc", "**/.**/**"),
            ("abc/../abc", "/.**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_globstar_dot() {
        for (input, pattern) in [
            ("/../abc", "**/**./**"),
            ("/../abc", "/**./**"),
            ("abc/../abc", "**/**./**"),
            ("abc/../abc", "/**./**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [
            ("/../abc", "**/**.**/**"),
            ("/../abc", "**/*.*/**"),
            ("/../abc", "/**.**/**"),
            ("/../abc", "/*.*/**"),
            ("abc/../abc", "**/**.**/**"),
            ("abc/../abc", "**/*.*/**"),
            ("abc/../abc", "/**.**/**"),
            ("abc/../abc", "/*.*/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_single_star() {
        for (input, pattern) in [
            ("abc/..", "*/*"),
            ("abc/..", "*/*/"),
            ("abc/..", "*/*/*"),
            ("abc/../", "*/*"),
            ("abc/../", "*/*/"),
            ("abc/../", "*/*/*"),
            ("abc/../abc/../", "*/*/*/*"),
            ("abc/../abc/../", "*/*/*/*/"),
            ("abc/../abc/abc/../", "*/*/*/*/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_dot_star() {
        for (input, pattern) in [
            ("abc/..", "*/.*"),
            ("abc/..", "*/.*/"),
            ("abc/..", "*/.*/*"),
            ("abc/../", "*/.*"),
            ("abc/../", "*/.*/"),
            ("abc/../", "*/.*/*"),
            ("abc/../abc/../", "*/.*/*/.*"),
            ("abc/../abc/../", "*/.*/*/.*/"),
            ("abc/../abc/abc/../", "*/.*/*/.*/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_star_dot() {
        for (input, pattern) in [
            ("abc/..", "*/*."),
            ("abc/..", "*/*./"),
            ("abc/..", "*/*./*"),
            ("abc/../", "*/*."),
            ("abc/../", "*/*./"),
            ("abc/../", "*/*./*"),
            ("abc/../abc/../", "*/*./*/*."),
            ("abc/../abc/../", "*/*./*/*./"),
            ("abc/../abc/abc/../", "*/*./*/*./*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_globstar() {
        for (input, pattern) in [
            ("abc/..", "**/**"),
            ("abc/..", "**/**/"),
            ("abc/..", "**/**/**"),
            ("abc/../", "**/**"),
            ("abc/../", "**/**/"),
            ("abc/../", "**/**/**"),
            ("abc/../abc/../", "**/**/**/**"),
            ("abc/../abc/../", "**/**/**/**/"),
            ("abc/../abc/abc/../", "**/**/**/**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("abc/..", "**/.**"),
            ("abc/..", "**/.**/"),
            ("abc/..", "**/.**/**"),
            ("abc/../", "**/.**"),
            ("abc/../", "**/.**/"),
            ("abc/../", "**/.**/**"),
            ("abc/../abc/../", "**/.**/**/.**"),
            ("abc/../abc/../", "**/.**/**/.**/"),
            ("abc/../abc/abc/../", "**/.**/**/.**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [
            ("abc/..", "**/**.**"),
            ("abc/..", "**/**.**/"),
            ("abc/..", "**/**.**/**"),
            ("abc/../", "**/**.**"),
            ("abc/../", "**/**.**/"),
            ("abc/../", "**/**.**/**"),
            ("abc/../abc/../", "**/**.**/**/**.**"),
            ("abc/../abc/../", "**/**.**/**/**.**/"),
            ("abc/../abc/abc/../", "**/**.**/**/.**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_globstar_dot() {
        for (input, pattern) in [
            ("abc/..", "**/**."),
            ("abc/..", "**/**./"),
            ("abc/..", "**/**./**"),
            ("abc/../", "**/**."),
            ("abc/../", "**/**./"),
            ("abc/../", "**/**./**"),
            ("abc/../abc/../", "**/**./**/**."),
            ("abc/../abc/../", "**/**./**/**./"),
            ("abc/../abc/abc/../", "**/**./**/**./**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }
}

/// Double dots tests - strictSlashes: true
mod double_dots_strict_slashes {
    use super::*;

    fn opts() -> CompileOptions {
        CompileOptions {
            strict_slashes: true,
            ..CompileOptions::default()
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_single_star() {
        for (input, pattern) in [
            ("../abc", "*/*"),
            ("../abc", "*/abc"),
            ("../abc", "*/abc/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_dot_single_star() {
        for (input, pattern) in [
            ("../abc", ".*/*"),
            ("../abc", ".*/abc"),
            ("../abc", "*./*"),
            ("../abc", "*./abc"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_globstar() {
        for (input, pattern) in [
            ("../abc", "**"),
            ("../abc", "**/**"),
            ("../abc", "**/**/**"),
            ("../abc", "**/abc"),
            ("../abc", "**/abc/**"),
            ("../abc", "abc/**"),
            ("../abc", "abc/**/**"),
            ("../abc", "abc/**/**/**"),
            ("../abc", "**/abc"),
            ("../abc", "**/abc/**"),
            ("../abc", "**/abc/**/**"),
            ("../abc", "**/**/abc/**"),
            ("../abc", "**/**/abc/**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("../abc", ".**"),
            ("../abc", ".**/**"),
            ("../abc", ".**/abc"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [("../abc", "*.*/**"), ("../abc", "*.*/abc")] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_globstar_dot() {
        for (input, pattern) in [("../abc", "**./**"), ("../abc", "**./abc")] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_star() {
        for (input, pattern) in [
            ("/../abc", "*/*"),
            ("/../abc", "/*/*"),
            ("/../abc", "*/*/*"),
            ("abc/../abc", "*/*/*"),
            ("abc/../abc/abc", "*/*/*/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_dot_star() {
        for (input, pattern) in [
            ("/../abc", "*/.*/*"),
            ("/../abc", "/.*/*"),
            ("/../abc", "*/*.*/*"),
            ("/../abc", "/*.*/*"),
            ("/../abc", "*/*./*"),
            ("/../abc", "/*./*"),
            ("abc/../abc", "*/.*/*"),
            ("abc/../abc", "*/*.*/*"),
            ("abc/../abc", "*/*./*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_globstar() {
        for (input, pattern) in [
            ("/../abc", "**"),
            ("/../abc", "**/**"),
            ("/../abc", "/**/**"),
            ("/../abc", "**/**/**"),
            ("abc/../abc", "**/**/**"),
            ("abc/../abc/abc", "**/**/**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("/../abc", "**/.**/**"),
            ("/../abc", "/.**/**"),
            ("abc/../abc", "**/.**/**"),
            ("abc/../abc", "/.**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_globstar_dot() {
        for (input, pattern) in [
            ("/../abc", "**/**./**"),
            ("/../abc", "/**./**"),
            ("abc/../abc", "**/**./**"),
            ("abc/../abc", "/**./**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [
            ("/../abc", "**/**.**/**"),
            ("/../abc", "**/*.*/**"),
            ("/../abc", "/**.**/**"),
            ("/../abc", "/*.*/**"),
            ("abc/../abc", "**/**.**/**"),
            ("abc/../abc", "**/*.*/**"),
            ("abc/../abc", "/**.**/**"),
            ("abc/../abc", "/*.*/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_single_star() {
        for (input, pattern) in [
            ("abc/..", "*/*"),
            ("abc/..", "*/*/"),
            ("abc/..", "*/*/*"),
            ("abc/../", "*/*"),
            ("abc/../", "*/*/"),
            ("abc/../", "*/*/*"),
            ("abc/../abc/../", "*/*/*/*"),
            ("abc/../abc/../", "*/*/*/*/"),
            ("abc/../abc/abc/../", "*/*/*/*/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_dot_star() {
        for (input, pattern) in [
            ("abc/..", "*/.*"),
            ("abc/..", "*/.*/"),
            ("abc/..", "*/.*/*"),
            ("abc/../", "*/.*"),
            ("abc/../", "*/.*/"),
            ("abc/../", "*/.*/*"),
            ("abc/../abc/../", "*/.*/*/.*"),
            ("abc/../abc/../", "*/.*/*/.*/"),
            ("abc/../abc/abc/../", "*/.*/*/.*/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_star_dot() {
        for (input, pattern) in [
            ("abc/..", "*/*."),
            ("abc/..", "*/*./"),
            ("abc/..", "*/*./*"),
            ("abc/../", "*/*."),
            ("abc/../", "*/*./"),
            ("abc/../", "*/*./*"),
            ("abc/../abc/../", "*/*./*/*."),
            ("abc/../abc/../", "*/*./*/*./"),
            ("abc/../abc/abc/../", "*/*./*/*./*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_globstar() {
        for (input, pattern) in [
            ("abc/..", "**/**"),
            ("abc/..", "**/**/"),
            ("abc/..", "**/**/**"),
            ("abc/../", "**/**"),
            ("abc/../", "**/**/"),
            ("abc/../", "**/**/**"),
            ("abc/../abc/../", "**/**/**/**"),
            ("abc/../abc/../", "**/**/**/**/"),
            ("abc/../abc/abc/../", "**/**/**/**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("abc/..", "**/.**"),
            ("abc/..", "**/.**/"),
            ("abc/..", "**/.**/**"),
            ("abc/../", "**/.**"),
            ("abc/../", "**/.**/"),
            ("abc/../", "**/.**/**"),
            ("abc/../abc/../", "**/.**/**/.**"),
            ("abc/../abc/../", "**/.**/**/.**/"),
            ("abc/../abc/abc/../", "**/.**/**/.**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [
            ("abc/..", "**/**.**"),
            ("abc/..", "**/**.**/"),
            ("abc/..", "**/**.**/**"),
            ("abc/../", "**/**.**"),
            ("abc/../", "**/**.**/"),
            ("abc/../", "**/**.**/**"),
            ("abc/../abc/../", "**/**.**/**/**.**"),
            ("abc/../abc/../", "**/**.**/**/**.**/"),
            ("abc/../abc/abc/../", "**/**.**/**/.**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_globstar_dot() {
        for (input, pattern) in [
            ("abc/..", "**/**."),
            ("abc/..", "**/**./"),
            ("abc/..", "**/**./**"),
            ("abc/../", "**/**."),
            ("abc/../", "**/**./"),
            ("abc/../", "**/**./**"),
            ("abc/../abc/../", "**/**./**/**."),
            ("abc/../abc/../", "**/**./**/**./"),
            ("abc/../abc/abc/../", "**/**./**/**./**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }
}

/// Double dots tests - dot: true + strictSlashes: true
mod double_dots_dot_and_strict {
    use super::*;

    fn opts() -> CompileOptions {
        CompileOptions {
            dot: true,
            strict_slashes: true,
            ..CompileOptions::default()
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_single_star() {
        for (input, pattern) in [
            ("../abc", "*/*"),
            ("../abc", "*/abc"),
            ("../abc", "*/abc/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_dot_single_star() {
        for (input, pattern) in [
            ("../abc", ".*/*"),
            ("../abc", ".*/abc"),
            ("../abc", "*./*"),
            ("../abc", "*./abc"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_globstar() {
        for (input, pattern) in [
            ("../abc", "**"),
            ("../abc", "**/**"),
            ("../abc", "**/**/**"),
            ("../abc", "**/abc"),
            ("../abc", "**/abc/**"),
            ("../abc", "abc/**"),
            ("../abc", "abc/**/**"),
            ("../abc", "abc/**/**/**"),
            ("../abc", "**/abc"),
            ("../abc", "**/abc/**"),
            ("../abc", "**/abc/**/**"),
            ("../abc", "**/**/abc/**"),
            ("../abc", "**/**/abc/**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("../abc", ".**"),
            ("../abc", ".**/**"),
            ("../abc", ".**/abc"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [("../abc", "*.*/**"), ("../abc", "*.*/abc")] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_double_dots_with_globstar_dot() {
        for (input, pattern) in [("../abc", "**./**"), ("../abc", "**./abc")] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_star() {
        for (input, pattern) in [
            ("/../abc", "*/*"),
            ("/../abc", "/*/*"),
            ("/../abc", "*/*/*"),
            ("abc/../abc", "*/*/*"),
            ("abc/../abc/abc", "*/*/*/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_dot_star() {
        for (input, pattern) in [
            ("/../abc", "*/.*/*"),
            ("/../abc", "/.*/*"),
            ("/../abc", "*/*.*/*"),
            ("/../abc", "/*.*/*"),
            ("/../abc", "*/*./*"),
            ("/../abc", "/*./*"),
            ("abc/../abc", "*/.*/*"),
            ("abc/../abc", "*/*.*/*"),
            ("abc/../abc", "*/*./*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_globstar() {
        for (input, pattern) in [
            ("/../abc", "**"),
            ("/../abc", "**/**"),
            ("/../abc", "/**/**"),
            ("/../abc", "**/**/**"),
            ("abc/../abc", "**/**/**"),
            ("abc/../abc/abc", "**/**/**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("/../abc", "**/.**/**"),
            ("/../abc", "/.**/**"),
            ("abc/../abc", "**/.**/**"),
            ("abc/../abc", "/.**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_globstar_dot() {
        for (input, pattern) in [
            ("/../abc", "**/**./**"),
            ("/../abc", "/**./**"),
            ("abc/../abc", "**/**./**"),
            ("abc/../abc", "/**./**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_double_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [
            ("/../abc", "**/**.**/**"),
            ("/../abc", "**/*.*/**"),
            ("/../abc", "/**.**/**"),
            ("/../abc", "/*.*/**"),
            ("abc/../abc", "**/**.**/**"),
            ("abc/../abc", "**/*.*/**"),
            ("abc/../abc", "/**.**/**"),
            ("abc/../abc", "/*.*/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_single_star() {
        for (input, pattern) in [
            ("abc/..", "*/*"),
            ("abc/..", "*/*/"),
            ("abc/..", "*/*/*"),
            ("abc/../", "*/*"),
            ("abc/../", "*/*/"),
            ("abc/../", "*/*/*"),
            ("abc/../abc/../", "*/*/*/*"),
            ("abc/../abc/../", "*/*/*/*/"),
            ("abc/../abc/abc/../", "*/*/*/*/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_dot_star() {
        for (input, pattern) in [
            ("abc/..", "*/.*"),
            ("abc/..", "*/.*/"),
            ("abc/..", "*/.*/*"),
            ("abc/../", "*/.*"),
            ("abc/../", "*/.*/"),
            ("abc/../", "*/.*/*"),
            ("abc/../abc/../", "*/.*/*/.*"),
            ("abc/../abc/../", "*/.*/*/.*/"),
            ("abc/../abc/abc/../", "*/.*/*/.*/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_star_dot() {
        for (input, pattern) in [
            ("abc/..", "*/*."),
            ("abc/..", "*/*./"),
            ("abc/..", "*/*./*"),
            ("abc/../", "*/*."),
            ("abc/../", "*/*./"),
            ("abc/../", "*/*./*"),
            ("abc/../abc/../", "*/*./*/*."),
            ("abc/../abc/../", "*/*./*/*./"),
            ("abc/../abc/abc/../", "*/*./*/*./*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_globstar() {
        for (input, pattern) in [
            ("abc/..", "**/**"),
            ("abc/..", "**/**/"),
            ("abc/..", "**/**/**"),
            ("abc/../", "**/**"),
            ("abc/../", "**/**/"),
            ("abc/../", "**/**/**"),
            ("abc/../abc/../", "**/**/**/**"),
            ("abc/../abc/../", "**/**/**/**/"),
            ("abc/../abc/abc/../", "**/**/**/**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("abc/..", "**/.**"),
            ("abc/..", "**/.**/"),
            ("abc/..", "**/.**/**"),
            ("abc/../", "**/.**"),
            ("abc/../", "**/.**/"),
            ("abc/../", "**/.**/**"),
            ("abc/../abc/../", "**/.**/**/.**"),
            ("abc/../abc/../", "**/.**/**/.**/"),
            ("abc/../abc/abc/../", "**/.**/**/.**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [
            ("abc/..", "**/**.**"),
            ("abc/..", "**/**.**/"),
            ("abc/..", "**/**.**/**"),
            ("abc/../", "**/**.**"),
            ("abc/../", "**/**.**/"),
            ("abc/../", "**/**.**/**"),
            ("abc/../abc/../", "**/**.**/**/**.**"),
            ("abc/../abc/../", "**/**.**/**/**.**/"),
            ("abc/../abc/abc/../", "**/**.**/**/.**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_double_dots_with_globstar_dot() {
        for (input, pattern) in [
            ("abc/..", "**/**."),
            ("abc/..", "**/**./"),
            ("abc/..", "**/**./**"),
            ("abc/../", "**/**."),
            ("abc/../", "**/**./"),
            ("abc/../", "**/**./**"),
            ("abc/../abc/../", "**/**./**/**."),
            ("abc/../abc/../", "**/**./**/**./"),
            ("abc/../abc/abc/../", "**/**./**/**./**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }
}

/// Single dots tests - no options
mod single_dots_no_options {
    use super::*;

    #[test]
    fn should_not_match_leading_single_dots_with_single_star() {
        for (input, pattern) in [
            ("./abc", "*"),
            ("./abc", "*/*"),
            ("./abc", "*/abc"),
            ("./abc", "*/abc/*"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_dot_single_star() {
        for (input, pattern) in [
            ("./abc", ".*/*"),
            ("./abc", ".*/abc"),
            ("./abc", "*./*"),
            ("./abc", "*./abc"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_globstar() {
        for (input, pattern) in [
            ("./abc", "**"),
            ("./abc", "**/**"),
            ("./abc", "**/**/**"),
            ("./abc", "**/abc"),
            ("./abc", "**/abc/**"),
            ("./abc", "abc/**"),
            ("./abc", "abc/**/**"),
            ("./abc", "abc/**/**/**"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_dot_globstar() {
        for (input, pattern) in [("./abc", ".**"), ("./abc", ".**/**"), ("./abc", ".**/abc")] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [("./abc", "*.*/**"), ("./abc", "*.*/abc")] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_globstar_dot() {
        for (input, pattern) in [("./abc", "**./**"), ("./abc", "**./abc")] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_star() {
        for (input, pattern) in [
            ("/./abc", "*/*"),
            ("/./abc", "/*/*"),
            ("/./abc", "*/*/*"),
            ("abc/./abc", "*/*/*"),
            ("abc/./abc/abc", "*/*/*/*"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_dot_star() {
        for (input, pattern) in [
            ("/./abc", "*/.*/*"),
            ("/./abc", "/.*/*"),
            ("/./abc", "*/*.*/*"),
            ("/./abc", "/*.*/*"),
            ("/./abc", "*/*./*"),
            ("/./abc", "/*./*"),
            ("abc/./abc", "*/.*/*"),
            ("abc/./abc", "*/*.*/*"),
            ("abc/./abc", "*/*./*"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_globstar() {
        for (input, pattern) in [
            ("/./abc", "**"),
            ("/./abc", "**/**"),
            ("/./abc", "/**/**"),
            ("/./abc", "**/**/**"),
            ("abc/./abc", "**/**/**"),
            ("abc/./abc/abc", "**/**/**/**"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("/./abc", "**/.**/**"),
            ("/./abc", "/.**/**"),
            ("abc/./abc", "**/.**/**"),
            ("abc/./abc", "/.**/**"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_globstar_dot() {
        for (input, pattern) in [
            ("/./abc", "**/**./**"),
            ("/./abc", "/**./**"),
            ("abc/./abc", "**/**./**"),
            ("abc/./abc", "/**./**"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [
            ("/./abc", "**/**.**/**"),
            ("/./abc", "**/*.*/**"),
            ("/./abc", "/**.**/**"),
            ("/./abc", "/*.*/**"),
            ("abc/./abc", "**/**.**/**"),
            ("abc/./abc", "**/*.*/**"),
            ("abc/./abc", "/**.**/**"),
            ("abc/./abc", "/*.*/**"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_single_star() {
        for (input, pattern) in [
            ("abc/.", "*/*"),
            ("abc/.", "*/*/"),
            ("abc/.", "*/*/*"),
            ("abc/./", "*/*"),
            ("abc/./", "*/*/"),
            ("abc/./", "*/*/*"),
            ("abc/./abc/./", "*/*/*/*"),
            ("abc/./abc/./", "*/*/*/*/"),
            ("abc/./abc/abc/./", "*/*/*/*/*"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_dot_star() {
        for (input, pattern) in [
            ("abc/.", "*/.*"),
            ("abc/.", "*/.*/"),
            ("abc/.", "*/.*/*"),
            ("abc/./", "*/.*"),
            ("abc/./", "*/.*/"),
            ("abc/./", "*/.*/*"),
            ("abc/./abc/./", "*/.*/*/.*"),
            ("abc/./abc/./", "*/.*/*/.*/"),
            ("abc/./abc/abc/./", "*/.*/*/.*/*"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_star_dot() {
        for (input, pattern) in [
            ("abc/.", "*/*."),
            ("abc/.", "*/*./"),
            ("abc/.", "*/*./*"),
            ("abc/./", "*/*."),
            ("abc/./", "*/*./"),
            ("abc/./", "*/*./*"),
            ("abc/./abc/./", "*/*./*/*."),
            ("abc/./abc/./", "*/*./*/*./"),
            ("abc/./abc/abc/./", "*/*./*/*./*"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_globstar() {
        for (input, pattern) in [
            ("abc/.", "**/**"),
            ("abc/.", "**/**/"),
            ("abc/.", "**/**/**"),
            ("abc/./", "**/**"),
            ("abc/./", "**/**/"),
            ("abc/./", "**/**/**"),
            ("abc/./abc/./", "**/**/**/**"),
            ("abc/./abc/./", "**/**/**/**/"),
            ("abc/./abc/abc/./", "**/**/**/**/**"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("abc/.", "**/.**"),
            ("abc/.", "**/.**/"),
            ("abc/.", "**/.**/**"),
            ("abc/./", "**/.**"),
            ("abc/./", "**/.**/"),
            ("abc/./", "**/.**/**"),
            ("abc/./abc/./", "**/.**/**/.**"),
            ("abc/./abc/./", "**/.**/**/.**/"),
            ("abc/./abc/abc/./", "**/.**/**/.**/**"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [
            ("abc/.", "**/**.**"),
            ("abc/.", "**/**.**/"),
            ("abc/.", "**/**.**/**"),
            ("abc/./", "**/**.**"),
            ("abc/./", "**/**.**/"),
            ("abc/./", "**/**.**/**"),
            ("abc/./abc/./", "**/**.**/**/**.**"),
            ("abc/./abc/./", "**/**.**/**/**.**/"),
            ("abc/./abc/abc/./", "**/**.**/**/.**/**"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_globstar_dot() {
        for (input, pattern) in [
            ("abc/.", "**/**."),
            ("abc/.", "**/**./"),
            ("abc/.", "**/**./**"),
            ("abc/./", "**/**."),
            ("abc/./", "**/**./"),
            ("abc/./", "**/**./**"),
            ("abc/./abc/./", "**/**./**/**."),
            ("abc/./abc/./", "**/**./**/**./"),
            ("abc/./abc/abc/./", "**/**./**/**./**"),
        ] {
            assert_is_match(input, pattern, default_compile_options(), false);
        }
    }
}

/// Single dots tests - dot: true
mod single_dots_dot_true {
    use super::*;

    fn opts() -> CompileOptions {
        CompileOptions {
            dot: true,
            ..CompileOptions::default()
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_single_star() {
        for (input, pattern) in [("./abc", "*/*"), ("./abc", "*/abc"), ("./abc", "*/abc/*")] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_dot_single_star() {
        for (input, pattern) in [
            ("./abc", ".*/*"),
            ("./abc", ".*/abc"),
            ("./abc", "*./*"),
            ("./abc", "*./abc"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_globstar() {
        for (input, pattern) in [
            ("./abc", "**"),
            ("./abc", "**/**"),
            ("./abc", "**/**/**"),
            ("./abc", "**/abc"),
            ("./abc", "**/abc/**"),
            ("./abc", "abc/**"),
            ("./abc", "abc/**/**"),
            ("./abc", "abc/**/**/**"),
            ("./abc", "**/abc"),
            ("./abc", "**/abc/**"),
            ("./abc", "**/abc/**/**"),
            ("./abc", "**/**/abc/**"),
            ("./abc", "**/**/abc/**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_dot_globstar() {
        for (input, pattern) in [("./abc", ".**"), ("./abc", ".**/**"), ("./abc", ".**/abc")] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [("./abc", "*.*/**"), ("./abc", "*.*/abc")] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_globstar_dot() {
        for (input, pattern) in [("./abc", "**./**"), ("./abc", "**./abc")] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_star() {
        for (input, pattern) in [
            ("/./abc", "*/*"),
            ("/./abc", "/*/*"),
            ("/./abc", "*/*/*"),
            ("abc/./abc", "*/*/*"),
            ("abc/./abc/abc", "*/*/*/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_dot_star() {
        for (input, pattern) in [
            ("/./abc", "*/.*/*"),
            ("/./abc", "/.*/*"),
            ("/./abc", "*/*.*/*"),
            ("/./abc", "/*.*/*"),
            ("/./abc", "*/*./*"),
            ("/./abc", "/*./*"),
            ("abc/./abc", "*/.*/*"),
            ("abc/./abc", "*/*.*/*"),
            ("abc/./abc", "*/*./*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_globstar() {
        for (input, pattern) in [
            ("/./abc", "**"),
            ("/./abc", "**/**"),
            ("/./abc", "/**/**"),
            ("/./abc", "**/**/**"),
            ("abc/./abc", "**/**/**"),
            ("abc/./abc/abc", "**/**/**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("/./abc", "**/.**/**"),
            ("/./abc", "/.**/**"),
            ("abc/./abc", "**/.**/**"),
            ("abc/./abc", "/.**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_globstar_dot() {
        for (input, pattern) in [
            ("/./abc", "**/**./**"),
            ("/./abc", "/**./**"),
            ("abc/./abc", "**/**./**"),
            ("abc/./abc", "/**./**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [
            ("/./abc", "**/**.**/**"),
            ("/./abc", "**/*.*/**"),
            ("/./abc", "/**.**/**"),
            ("/./abc", "/*.*/**"),
            ("abc/./abc", "**/**.**/**"),
            ("abc/./abc", "**/*.*/**"),
            ("abc/./abc", "/**.**/**"),
            ("abc/./abc", "/*.*/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_single_star() {
        for (input, pattern) in [
            ("abc/.", "*/*"),
            ("abc/.", "*/*/"),
            ("abc/.", "*/*/*"),
            ("abc/./", "*/*"),
            ("abc/./", "*/*/"),
            ("abc/./", "*/*/*"),
            ("abc/./abc/./", "*/*/*/*"),
            ("abc/./abc/./", "*/*/*/*/"),
            ("abc/./abc/abc/./", "*/*/*/*/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_dot_star() {
        for (input, pattern) in [
            ("abc/.", "*/.*"),
            ("abc/.", "*/.*/"),
            ("abc/.", "*/.*/*"),
            ("abc/./", "*/.*"),
            ("abc/./", "*/.*/"),
            ("abc/./", "*/.*/*"),
            ("abc/./abc/./", "*/.*/*/.*"),
            ("abc/./abc/./", "*/.*/*/.*/"),
            ("abc/./abc/abc/./", "*/.*/*/.*/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_star_dot() {
        for (input, pattern) in [
            ("abc/.", "*/*."),
            ("abc/.", "*/*./"),
            ("abc/.", "*/*./*"),
            ("abc/./", "*/*."),
            ("abc/./", "*/*./"),
            ("abc/./", "*/*./*"),
            ("abc/./abc/./", "*/*./*/*."),
            ("abc/./abc/./", "*/*./*/*./"),
            ("abc/./abc/abc/./", "*/*./*/*./*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_globstar() {
        for (input, pattern) in [
            ("abc/.", "**/**"),
            ("abc/.", "**/**/"),
            ("abc/.", "**/**/**"),
            ("abc/./", "**/**"),
            ("abc/./", "**/**/"),
            ("abc/./", "**/**/**"),
            ("abc/./abc/./", "**/**/**/**"),
            ("abc/./abc/./", "**/**/**/**/"),
            ("abc/./abc/abc/./", "**/**/**/**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("abc/.", "**/.**"),
            ("abc/.", "**/.**/"),
            ("abc/.", "**/.**/**"),
            ("abc/./", "**/.**"),
            ("abc/./", "**/.**/"),
            ("abc/./", "**/.**/**"),
            ("abc/./abc/./", "**/.**/**/.**"),
            ("abc/./abc/./", "**/.**/**/.**/"),
            ("abc/./abc/abc/./", "**/.**/**/.**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [
            ("abc/.", "**/**.**"),
            ("abc/.", "**/**.**/"),
            ("abc/.", "**/**.**/**"),
            ("abc/./", "**/**.**"),
            ("abc/./", "**/**.**/"),
            ("abc/./", "**/**.**/**"),
            ("abc/./abc/./", "**/**.**/**/**.**"),
            ("abc/./abc/./", "**/**.**/**/**.**/"),
            ("abc/./abc/abc/./", "**/**.**/**/.**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_globstar_dot() {
        for (input, pattern) in [
            ("abc/.", "**/**."),
            ("abc/.", "**/**./"),
            ("abc/.", "**/**./**"),
            ("abc/./", "**/**."),
            ("abc/./", "**/**./"),
            ("abc/./", "**/**./**"),
            ("abc/./abc/./", "**/**./**/**."),
            ("abc/./abc/./", "**/**./**/**./"),
            ("abc/./abc/abc/./", "**/**./**/**./**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }
}

/// Single dots tests - strictSlashes: true
mod single_dots_strict_slashes {
    use super::*;

    fn opts() -> CompileOptions {
        CompileOptions {
            strict_slashes: true,
            ..CompileOptions::default()
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_single_star() {
        for (input, pattern) in [("./abc", "*/*"), ("./abc", "*/abc"), ("./abc", "*/abc/*")] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_dot_single_star() {
        for (input, pattern) in [
            ("./abc", ".*/*"),
            ("./abc", ".*/abc"),
            ("./abc", "*./*"),
            ("./abc", "*./abc"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_globstar() {
        for (input, pattern) in [
            ("./abc", "**"),
            ("./abc", "**/**"),
            ("./abc", "**/**/**"),
            ("./abc", "**/abc"),
            ("./abc", "**/abc/**"),
            ("./abc", "abc/**"),
            ("./abc", "abc/**/**"),
            ("./abc", "abc/**/**/**"),
            ("./abc", "**/abc"),
            ("./abc", "**/abc/**"),
            ("./abc", "**/abc/**/**"),
            ("./abc", "**/**/abc/**"),
            ("./abc", "**/**/abc/**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_dot_globstar() {
        for (input, pattern) in [("./abc", ".**"), ("./abc", ".**/**"), ("./abc", ".**/abc")] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [("./abc", "*.*/**"), ("./abc", "*.*/abc")] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_globstar_dot() {
        for (input, pattern) in [("./abc", "**./**"), ("./abc", "**./abc")] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_star() {
        for (input, pattern) in [
            ("/./abc", "*/*"),
            ("/./abc", "/*/*"),
            ("/./abc", "*/*/*"),
            ("abc/./abc", "*/*/*"),
            ("abc/./abc/abc", "*/*/*/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_dot_star() {
        for (input, pattern) in [
            ("/./abc", "*/.*/*"),
            ("/./abc", "/.*/*"),
            ("/./abc", "*/*.*/*"),
            ("/./abc", "/*.*/*"),
            ("/./abc", "*/*./*"),
            ("/./abc", "/*./*"),
            ("abc/./abc", "*/.*/*"),
            ("abc/./abc", "*/*.*/*"),
            ("abc/./abc", "*/*./*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_globstar() {
        for (input, pattern) in [
            ("/./abc", "**"),
            ("/./abc", "**/**"),
            ("/./abc", "/**/**"),
            ("/./abc", "**/**/**"),
            ("abc/./abc", "**/**/**"),
            ("abc/./abc/abc", "**/**/**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("/./abc", "**/.**/**"),
            ("/./abc", "/.**/**"),
            ("abc/./abc", "**/.**/**"),
            ("abc/./abc", "/.**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_globstar_dot() {
        for (input, pattern) in [
            ("/./abc", "**/**./**"),
            ("/./abc", "/**./**"),
            ("abc/./abc", "**/**./**"),
            ("abc/./abc", "/**./**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [
            ("/./abc", "**/**.**/**"),
            ("/./abc", "**/*.*/**"),
            ("/./abc", "/**.**/**"),
            ("/./abc", "/*.*/**"),
            ("abc/./abc", "**/**.**/**"),
            ("abc/./abc", "**/*.*/**"),
            ("abc/./abc", "/**.**/**"),
            ("abc/./abc", "/*.*/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_single_star() {
        for (input, pattern) in [
            ("abc/.", "*/*"),
            ("abc/.", "*/*/"),
            ("abc/.", "*/*/*"),
            ("abc/./", "*/*"),
            ("abc/./", "*/*/"),
            ("abc/./", "*/*/*"),
            ("abc/./abc/./", "*/*/*/*"),
            ("abc/./abc/./", "*/*/*/*/"),
            ("abc/./abc/abc/./", "*/*/*/*/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_dot_star() {
        for (input, pattern) in [
            ("abc/.", "*/.*"),
            ("abc/.", "*/.*/"),
            ("abc/.", "*/.*/*"),
            ("abc/./", "*/.*"),
            ("abc/./", "*/.*/"),
            ("abc/./", "*/.*/*"),
            ("abc/./abc/./", "*/.*/*/.*"),
            ("abc/./abc/./", "*/.*/*/.*/"),
            ("abc/./abc/abc/./", "*/.*/*/.*/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_star_dot() {
        for (input, pattern) in [
            ("abc/.", "*/*."),
            ("abc/.", "*/*./"),
            ("abc/.", "*/*./*"),
            ("abc/./", "*/*."),
            ("abc/./", "*/*./"),
            ("abc/./", "*/*./*"),
            ("abc/./abc/./", "*/*./*/*."),
            ("abc/./abc/./", "*/*./*/*./"),
            ("abc/./abc/abc/./", "*/*./*/*./*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_globstar() {
        for (input, pattern) in [
            ("abc/.", "**/**"),
            ("abc/.", "**/**/"),
            ("abc/.", "**/**/**"),
            ("abc/./", "**/**"),
            ("abc/./", "**/**/"),
            ("abc/./", "**/**/**"),
            ("abc/./abc/./", "**/**/**/**"),
            ("abc/./abc/./", "**/**/**/**/"),
            ("abc/./abc/abc/./", "**/**/**/**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("abc/.", "**/.**"),
            ("abc/.", "**/.**/"),
            ("abc/.", "**/.**/**"),
            ("abc/./", "**/.**"),
            ("abc/./", "**/.**/"),
            ("abc/./", "**/.**/**"),
            ("abc/./abc/./", "**/.**/**/.**"),
            ("abc/./abc/./", "**/.**/**/.**/"),
            ("abc/./abc/abc/./", "**/.**/**/.**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [
            ("abc/.", "**/**.**"),
            ("abc/.", "**/**.**/"),
            ("abc/.", "**/**.**/**"),
            ("abc/./", "**/**.**"),
            ("abc/./", "**/**.**/"),
            ("abc/./", "**/**.**/**"),
            ("abc/./abc/./", "**/**.**/**/**.**"),
            ("abc/./abc/./", "**/**.**/**/**.**/"),
            ("abc/./abc/abc/./", "**/**.**/**/.**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_globstar_dot() {
        for (input, pattern) in [
            ("abc/.", "**/**."),
            ("abc/.", "**/**./"),
            ("abc/.", "**/**./**"),
            ("abc/./", "**/**."),
            ("abc/./", "**/**./"),
            ("abc/./", "**/**./**"),
            ("abc/./abc/./", "**/**./**/**."),
            ("abc/./abc/./", "**/**./**/**./"),
            ("abc/./abc/abc/./", "**/**./**/**./**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }
}

/// Single dots tests - dot: true + strictSlashes: true
mod single_dots_dot_and_strict {
    use super::*;

    fn opts() -> CompileOptions {
        CompileOptions {
            dot: true,
            strict_slashes: true,
            ..CompileOptions::default()
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_single_star() {
        for (input, pattern) in [("./abc", "*/*"), ("./abc", "*/abc"), ("./abc", "*/abc/*")] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_dot_single_star() {
        for (input, pattern) in [
            ("./abc", ".*/*"),
            ("./abc", ".*/abc"),
            ("./abc", "*./*"),
            ("./abc", "*./abc"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_globstar() {
        for (input, pattern) in [
            ("./abc", "**"),
            ("./abc", "**/**"),
            ("./abc", "**/**/**"),
            ("./abc", "**/abc"),
            ("./abc", "**/abc/**"),
            ("./abc", "abc/**"),
            ("./abc", "abc/**/**"),
            ("./abc", "abc/**/**/**"),
            ("./abc", "**/abc"),
            ("./abc", "**/abc/**"),
            ("./abc", "**/abc/**/**"),
            ("./abc", "**/**/abc/**"),
            ("./abc", "**/**/abc/**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_dot_globstar() {
        for (input, pattern) in [("./abc", ".**"), ("./abc", ".**/**"), ("./abc", ".**/abc")] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [("./abc", "*.*/**"), ("./abc", "*.*/abc")] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_leading_single_dots_with_globstar_dot() {
        for (input, pattern) in [("./abc", "**./**"), ("./abc", "**./abc")] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_star() {
        for (input, pattern) in [
            ("/./abc", "*/*"),
            ("/./abc", "/*/*"),
            ("/./abc", "*/*/*"),
            ("abc/./abc", "*/*/*"),
            ("abc/./abc/abc", "*/*/*/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_dot_star() {
        for (input, pattern) in [
            ("/./abc", "*/.*/*"),
            ("/./abc", "/.*/*"),
            ("/./abc", "*/*.*/*"),
            ("/./abc", "/*.*/*"),
            ("/./abc", "*/*./*"),
            ("/./abc", "/*./*"),
            ("abc/./abc", "*/.*/*"),
            ("abc/./abc", "*/*.*/*"),
            ("abc/./abc", "*/*./*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_globstar() {
        for (input, pattern) in [
            ("/./abc", "**"),
            ("/./abc", "**/**"),
            ("/./abc", "/**/**"),
            ("/./abc", "**/**/**"),
            ("abc/./abc", "**/**/**"),
            ("abc/./abc/abc", "**/**/**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("/./abc", "**/.**/**"),
            ("/./abc", "/.**/**"),
            ("abc/./abc", "**/.**/**"),
            ("abc/./abc", "/.**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_globstar_dot() {
        for (input, pattern) in [
            ("/./abc", "**/**./**"),
            ("/./abc", "/**./**"),
            ("abc/./abc", "**/**./**"),
            ("abc/./abc", "/**./**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_nested_single_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [
            ("/./abc", "**/**.**/**"),
            ("/./abc", "**/*.*/**"),
            ("/./abc", "/**.**/**"),
            ("/./abc", "/*.*/**"),
            ("abc/./abc", "**/**.**/**"),
            ("abc/./abc", "**/*.*/**"),
            ("abc/./abc", "/**.**/**"),
            ("abc/./abc", "/*.*/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_single_star() {
        for (input, pattern) in [
            ("abc/.", "*/*"),
            ("abc/.", "*/*/"),
            ("abc/.", "*/*/*"),
            ("abc/./", "*/*"),
            ("abc/./", "*/*/"),
            ("abc/./", "*/*/*"),
            ("abc/./abc/./", "*/*/*/*"),
            ("abc/./abc/./", "*/*/*/*/"),
            ("abc/./abc/abc/./", "*/*/*/*/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_dot_star() {
        for (input, pattern) in [
            ("abc/.", "*/.*"),
            ("abc/.", "*/.*/"),
            ("abc/.", "*/.*/*"),
            ("abc/./", "*/.*"),
            ("abc/./", "*/.*/"),
            ("abc/./", "*/.*/*"),
            ("abc/./abc/./", "*/.*/*/.*"),
            ("abc/./abc/./", "*/.*/*/.*/"),
            ("abc/./abc/abc/./", "*/.*/*/.*/*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_star_dot() {
        for (input, pattern) in [
            ("abc/.", "*/*."),
            ("abc/.", "*/*./"),
            ("abc/.", "*/*./*"),
            ("abc/./", "*/*."),
            ("abc/./", "*/*./"),
            ("abc/./", "*/*./*"),
            ("abc/./abc/./", "*/*./*/*."),
            ("abc/./abc/./", "*/*./*/*./"),
            ("abc/./abc/abc/./", "*/*./*/*./*"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_globstar() {
        for (input, pattern) in [
            ("abc/.", "**/**"),
            ("abc/.", "**/**/"),
            ("abc/.", "**/**/**"),
            ("abc/./", "**/**"),
            ("abc/./", "**/**/"),
            ("abc/./", "**/**/**"),
            ("abc/./abc/./", "**/**/**/**"),
            ("abc/./abc/./", "**/**/**/**/"),
            ("abc/./abc/abc/./", "**/**/**/**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_dot_globstar() {
        for (input, pattern) in [
            ("abc/.", "**/.**"),
            ("abc/.", "**/.**/"),
            ("abc/.", "**/.**/**"),
            ("abc/./", "**/.**"),
            ("abc/./", "**/.**/"),
            ("abc/./", "**/.**/**"),
            ("abc/./abc/./", "**/.**/**/.**"),
            ("abc/./abc/./", "**/.**/**/.**/"),
            ("abc/./abc/abc/./", "**/.**/**/.**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_globstar_dot_globstar() {
        for (input, pattern) in [
            ("abc/.", "**/**.**"),
            ("abc/.", "**/**.**/"),
            ("abc/.", "**/**.**/**"),
            ("abc/./", "**/**.**"),
            ("abc/./", "**/**.**/"),
            ("abc/./", "**/**.**/**"),
            ("abc/./abc/./", "**/**.**/**/**.**"),
            ("abc/./abc/./", "**/**.**/**/**.**/"),
            ("abc/./abc/abc/./", "**/**.**/**/.**/**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }

    #[test]
    fn should_not_match_trailing_single_dots_with_globstar_dot() {
        for (input, pattern) in [
            ("abc/.", "**/**."),
            ("abc/.", "**/**./"),
            ("abc/.", "**/**./**"),
            ("abc/./", "**/**."),
            ("abc/./", "**/**./"),
            ("abc/./", "**/**./**"),
            ("abc/./abc/./", "**/**./**/**."),
            ("abc/./abc/./", "**/**./**/**./"),
            ("abc/./abc/abc/./", "**/**./**/**./**"),
        ] {
            assert_is_match(input, pattern, opts(), false);
        }
    }
}
