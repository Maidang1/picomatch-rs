use picomatch_rs::{compile_matcher, is_match_any, CompileOptions};

fn main() {
    let options = CompileOptions::default();

    // Compile a matcher once, reuse for multiple matches
    println!("Compiling matcher for: '**/*.rs'");
    let matcher = compile_matcher("**/*.rs", &options).expect("Failed to compile");

    let test_paths = [
        "src/lib.rs",
        "src/main.rs",
        "README.md",
        "tests/test.rs",
        "docs/readme.txt",
    ];

    for path in test_paths {
        match matcher.is_match(path) {
            Ok(result) => println!("  {} => {}", path, result),
            Err(e) => println!("  {} => Error: {:?}", path, e),
        }
    }

    // Match against multiple patterns
    println!("\nMatching 'file.rs' against any of ['*.rs', '*.js', '*.txt']:");
    let result = is_match_any("file.rs", ["*.rs", "*.js", "*.txt"], &options)
        .expect("Failed to match");
    println!("  result: {}", result);

    println!("\nMatching 'file.py' against any of ['*.rs', '*.js', '*.txt']:");
    let result = is_match_any("file.py", ["*.rs", "*.js", "*.txt"], &options)
        .expect("Failed to match");
    println!("  result: {}", result);
}
