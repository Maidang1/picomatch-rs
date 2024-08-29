#[test]
fn test_scan() {
    let scan = Scan::new();
    let res: ScanState = scan.scan("foo/bar/*/*/*.js".to_string(), ScanOptions::default());
    let expected_res = ScanState {
        prefix: "".to_string(),
        input: "foo/bar/*/*/*.js".to_string(),
        start: 0,
        base: "foo/bar".to_string(),
        glob: "*/*/*.js".to_string(),
        is_brace: false,
        is_bracket: false,
        is_glob: true,
        is_ext_glob: false,
        is_glob_star: false,
        negated: false,
        negated_extglob: false,
        max_depth: None,
        tokens: None,
        ..ScanState::default()
    };
    assert_eq!(res, expected_res);
}

#[test]
fn test_handle_leading() {
    let scan = Scan::new();
    let res: ScanState = scan.scan("./foo/bar/*.js".to_string(), ScanOptions::default());
    let expected_res = ScanState {
        prefix: "./".to_string(),
        input: "./foo/bar/*.js".to_string(),
        start: 2,
        base: "foo/bar".to_string(),
        glob: "*.js".to_string(),
        is_brace: false,
        is_bracket: false,
        is_glob: true,
        is_ext_glob: false,
        is_glob_star: false,
        negated: false,
        negated_extglob: false,
        max_depth: None,
        tokens: None,
        ..ScanState::default()
    };
    assert_eq!(res, expected_res);
}

#[test]
fn test_should_detect_braces() {
    let scan = Scan::new();
    let res: ScanState = scan.scan("foo/{a,b,c}/*.js".to_string(), ScanOptions::default());
    let expected_res = ScanState {
        prefix: "".to_string(),
        input: "foo/{a,b,c}/*.js".to_string(),
        start: 0,
        base: "foo".to_string(),
        glob: "{a,b,c}/*.js".to_string(),
        is_brace: true,
        is_bracket: false,
        is_glob: true,
        is_ext_glob: false,
        is_glob_star: false,
        negated: false,
        negated_extglob: false,
        max_depth: None,
        tokens: None,
        ..ScanState::default()
    };
    assert_eq!(res, expected_res);
}

#[test]
fn should_detect_glob_starts() {
    let scan = Scan::new();
    let res: ScanState = scan.scan(
        "./foo/**/*.js".to_string(),
        ScanOptions {
            scan_to_end: Some(true),
            ..ScanOptions::default()
        },
    );
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
        ..ScanState::default()
    };
    assert_eq!(res, expected_res);
}

#[test]
fn should_detect_extglobs() {
    let scan = Scan::new();
    let res: ScanState = scan.scan(
        "./foo/@(foo)/*.js".to_string(),
        ScanOptions {
            ..ScanOptions::default()
        },
    );
    let expected_res = ScanState {
        prefix: "./".to_string(),
        input: "./foo/@(foo)/*.js".to_string(),
        start: 2,
        base: "foo".to_string(),
        glob: "@(foo)/*.js".to_string(),
        is_brace: false,
        is_bracket: false,
        is_glob: true,
        is_ext_glob: true,
        is_glob_star: false,
        negated: false,
        negated_extglob: false,
        max_depth: None,
        tokens: None,
        ..ScanState::default()
    };
    assert_eq!(res, expected_res);
}

#[test]
fn should_detect_extglobs_and_globstars() {
    let scan = Scan::new();
    let res: ScanState = scan.scan(
        "./foo/@(bar)/**/*.js".to_string(),
        ScanOptions {
            parts: Some(true),
            ..ScanOptions::default()
        },
    );
    let expected_res = ScanState {
        prefix: "./".to_string(),
        input: "./foo/@(bar)/**/*.js".to_string(),
        start: 2,
        base: "foo".to_string(),
        glob: "@(bar)/**/*.js".to_string(),
        is_brace: false,
        is_bracket: false,
        is_glob: true,
        is_ext_glob: true,
        is_glob_star: true,
        negated: false,
        negated_extglob: false,
        slashes: Some(vec![1, 5, 12, 15]),
        parts:Some(vec!["foo".to_string(), "@(bar)".to_string(), "**".to_string(), "*.js".to_string()]),
        max_depth: None,
        tokens: None,
        ..ScanState::default()
    };
    assert_eq!(res, expected_res);
}
