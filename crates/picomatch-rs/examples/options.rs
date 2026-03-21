use picomatch_rs::{is_match, CompileOptions};

fn main() {
    // Case insensitive matching
    let options_case_insensitive = CompileOptions {
        flags: "i".to_string(),
        ..Default::default()
    };

    println!("Case insensitive matching:");
    println!("  'A/B/C.MD' matches 'a/b/*.md': {}",
        is_match("A/B/C.MD", "a/b/*.md", &options_case_insensitive).unwrap());

    // Windows-style paths
    let options_windows = CompileOptions {
        windows: true,
        ..Default::default()
    };

    println!("\nWindows path matching:");
    println!("  'aaa\\\\bbb' matches 'aaa/bbb': {}",
        is_match("aaa\\bbb", "aaa/bbb", &options_windows).unwrap());
    println!("  'aaa/bbb' matches 'aaa/bbb': {}",
        is_match("aaa/bbb", "aaa/bbb", &options_windows).unwrap());

    // Dot files (files starting with .)
    let options_dot = CompileOptions {
        dot: true,
        ..Default::default()
    };

    println!("\nDot file matching:");
    println!("  '.gitignore' matches '.*': {}",
        is_match(".gitignore", ".*", &options_dot).unwrap());

    // Unescape option
    let options_unescape = CompileOptions {
        unescape: true,
        ..Default::default()
    };

    println!("\nUnescape matching:");
    println!("  'file.txt' matches 'file\\\\.txt': {}",
        is_match("file.txt", "file\\.txt", &options_unescape).unwrap());
}
