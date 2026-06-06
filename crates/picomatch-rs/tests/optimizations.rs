mod support;

use picomatch_rs::{CompileOptions, scan, is_match};

#[test]
fn test_basename_behavior() {
    // Test basename behavior through the public API
    let options = CompileOptions { match_base: true, ..CompileOptions::default() };
    
    // Test that match_base option correctly uses basename
    assert!(is_match("a/b/c.md", "c.md", &options).unwrap());
    assert!(is_match("x/y/z.txt", "z.txt", &options).unwrap());
    assert!(!is_match("a/b/c.md", "b.md", &options).unwrap());
}

#[test]
fn test_dot_segment_handling() {
    // Test that dot segments are handled correctly
    let options = CompileOptions::default();
    
    // Test paths with dot segments
    assert!(is_match("./test", "./test", &options).unwrap());
    assert!(is_match("test/./sub", "test/./sub", &options).unwrap());
    assert!(is_match("../test", "../test", &options).unwrap());
}

#[test]
fn test_remove_backslashes() {
    use picomatch_rs::utils::remove_backslashes;
    
    // Test case 1: Regular backslashes
    assert_eq!(remove_backslashes("a\\b\\c"), "abc");
    
    // Test case 2: Backslashes inside brackets (should be preserved)
    assert_eq!(remove_backslashes("[a\\b]"), "[a\\b]");
    
    // Test case 3: Backslash before bracket (should be removed)
    assert_eq!(remove_backslashes("\\[a]"), "[a]");
    
    // Test case 4: Mixed backslashes and forward slashes
    assert_eq!(remove_backslashes("a\\b/c\\d"), "ab/cd");
    
    // Test case 5: No backslashes
    assert_eq!(remove_backslashes("abc"), "abc");
    
    // Test case 6: Consecutive backslashes
    assert_eq!(remove_backslashes("a\\\\b"), "a\\b");
    
    // Test case 7: Trailing backslash
    assert_eq!(remove_backslashes("abc\\"), "abc");
}

#[test]
fn test_input_view_optimization() {
    // Test that scan function works correctly with the optimized InputView
    let result = scan("test/path/*", &Default::default());
    assert_eq!(result.base, "test/path");
    assert_eq!(result.glob, "*");
    assert!(result.is_glob);
    
    let result = scan("test/[a-z]/*", &Default::default());
    assert_eq!(result.base, "test");
    assert_eq!(result.glob, "[a-z]/*");
    assert!(result.is_glob);
    assert!(result.is_bracket);
}
