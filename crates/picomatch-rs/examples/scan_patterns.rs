use picomatch_rs::{scan, ScanOptions};

fn main() {
    // Scan a simple glob pattern
    let options = ScanOptions::default();
    let state = scan("src/**/*.rs", &options);

    println!("Pattern: src/**/*.rs");
    println!("  prefix: {:?}", state.prefix);
    println!("  base: {:?}", state.base);
    println!("  glob: {:?}", state.glob);
    println!("  is_glob: {}", state.is_glob);
    println!("  is_globstar: {}", state.is_globstar);
    println!("  is_extglob: {}", state.is_extglob);

    // Scan with tokens
    println!("\nPattern with tokens: src/**/*.rs");
    let options_with_tokens = ScanOptions {
        tokens: true,
        ..Default::default()
    };
    let state = scan("src/**/*.rs", &options_with_tokens);
    if let Some(tokens) = state.tokens {
        for (i, token) in tokens.iter().enumerate() {
            println!("  token[{}]: value={:?} depth={}", i, token.value, token.depth);
        }
    }

    // Scan with parts
    println!("\nPattern with parts: src/**/*.rs");
    let options_with_parts = ScanOptions {
        parts: true,
        ..Default::default()
    };
    let state = scan("src/**/*.rs", &options_with_parts);
    if let Some(parts) = state.parts {
        println!("  parts: {:?}", parts);
    }
}
