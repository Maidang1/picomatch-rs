// #[test]
// fn test_scan() {
//     let scan = Scan::new();
//     let res: ScanState = scan.scan("foo/bar/*/*/*.js".to_string(), ScanOptions::default());
//     let expected_res = ScanState {
//         prefix: "".to_string(),
//         input: "foo/bar/*/*/*.js".to_string(),
//         start: 0,
//         base: "foo/bar".to_string(),
//         glob: "*/*/*.js".to_string(),
//         is_brace: false,
//         is_bracket: false,
//         is_glob: true,
//         is_ext_glob: false,
//         is_glob_star: false,
//         negated: false,
//         negated_extglob: false,
//         max_depth: None,
//         tokens: None,
//     };
//     assert_eq!(res, expected_res);
// }

// #[test]
// fn test_handle_leading() {
//     let scan = Scan::new();
//     let res: ScanState = scan.scan("./foo/bar/*.js".to_string(), ScanOptions::default());
//     let expected_res = ScanState {
//         prefix: "./".to_string(),
//         input: "./foo/bar/*.js".to_string(),
//         start: 2,
//         base: "foo/bar".to_string(),
//         glob: "*.js".to_string(),
//         is_brace: false,
//         is_bracket: false,
//         is_glob: true,
//         is_ext_glob: false,
//         is_glob_star: false,
//         negated: false,
//         negated_extglob: false,
//         max_depth: None,
//         tokens: None,
//     };
//     assert_eq!(res, expected_res);
// }

// #[test]
// fn test_should_detect_braces() {
//     let scan = Scan::new();
//     let res: ScanState = scan.scan("foo/{a,b,c}/*.js".to_string(), ScanOptions::default());
//     let expected_res = ScanState {
//         prefix: "".to_string(),
//         input: "foo/{a,b,c}/*.js".to_string(),
//         start: 0,
//         base: "foo".to_string(),
//         glob: "{a,b,c}/*.js".to_string(),
//         is_brace: true,
//         is_bracket: false,
//         is_glob: true,
//         is_ext_glob: false,
//         is_glob_star: false,
//         negated: false,
//         negated_extglob: false,
//         max_depth: None,
//         tokens: None,
//     };
//     assert_eq!(res, expected_res);
// }

#[test]
fn should_detect_glob_starts() {
    let scan = Scan::new();
    let res: ScanState = scan.scan("./foo/**/*.js".to_string(), ScanOptions {
      scan_to_end: Some(true),
      ..ScanOptions::default()
    });
    let expected_res = ScanState {
        prefix: "./".to_string(),
        input: "./foo/**/*.js".to_string(),
        start: 2,
        base: "foo".to_string(),
        glob: "**/*.js".to_string(),
        is_brace: false,
        is_bracket: false,
        is_glob: true,
        is_ext_glob: false,
        is_glob_star: true,
        negated: false,
        negated_extglob: false,
        max_depth: None,
        tokens: None,
    };
    assert_eq!(res, expected_res);
}
