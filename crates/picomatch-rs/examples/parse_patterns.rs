use picomatch_rs::{parse, make_re, CompileOptions};

fn main() {
    let options = CompileOptions::default();

    // Parse a pattern to see its structure
    println!("Parsing pattern: 'src/**/*.rs'");
    if let Some(state) = parse("src/**/*.rs", &options) {
        println!("  input: {}", state.input);
        println!("  output: {}", state.output);
        println!("  negated: {}", state.negated);
        if let Some(tokens) = &state.tokens {
            println!("  tokens:");
            for (i, token) in tokens.iter().enumerate() {
                println!("    [{}] {} => {:?}",
                    i, token.kind, token.output);
            }
        }
    }

    // Generate regex from pattern
    println!("\nGenerating regex for: '*.txt'");
    if let Some(descriptor) = make_re("*.txt", &options, false) {
        println!("  source: {}", descriptor.source);
        println!("  flags: {:?}", descriptor.flags);
        println!("  output: {}", descriptor.output);
    }

    // Parse brace expansion
    println!("\nParsing pattern with braces: 'file{{a,b}}.txt'");
    if let Some(state) = parse("file{a,b}.txt", &options) {
        println!("  output: {}", state.output);
    }

    // Parse character class
    println!("\nParsing pattern with character class: '[abc]*.txt'");
    if let Some(state) = parse("[abc]*.txt", &options) {
        println!("  output: {}", state.output);
    }
}
