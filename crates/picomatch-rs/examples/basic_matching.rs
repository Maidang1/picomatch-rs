use picomatch_rs::{is_match, CompileOptions};

fn main() {
    let options = CompileOptions::default();

    // Basic glob matching
    println!("Matching '**/*.rs':");
    println!("  src/lib.rs: {}", is_match("src/lib.rs", "**/*.rs", &options).unwrap());
    println!("  README.md: {}", is_match("README.md", "**/*.rs", &options).unwrap());

    // File extensions
    println!("\nMatching '*.txt':");
    println!("  readme.txt: {}", is_match("readme.txt", "*.txt", &options).unwrap());
    println!("  doc.md: {}", is_match("doc.md", "*.txt", &options).unwrap());

    // Single character wildcard
    println!("\nMatching 'file?.txt':");
    println!("  file1.txt: {}", is_match("file1.txt", "file?.txt", &options).unwrap());
    println!("  file12.txt: {}", is_match("file12.txt", "file?.txt", &options).unwrap());

    // Character classes
    println!("\nMatching '[abc]*.txt':");
    println!("  a.txt: {}", is_match("a.txt", "[abc]*.txt", &options).unwrap());
    println!("  d.txt: {}", is_match("d.txt", "[abc]*.txt", &options).unwrap());
}
