//! # Binary Crate Example
//!
//! Demonstrates a binary crate with main() function.
//!
//! Run with: `cargo run --bin binary_example`

fn main() {
    println!("🦀 Binary Crate Example");
    println!("======================");
    
    println!("This is a binary crate because:");
    println!("  - It has a main() function");
    println!("  - It can be executed directly");
    println!("  - It's defined in Cargo.toml as [[bin]]");
    
    println!("\nBinary crates are useful for:");
    println!("  - Command-line tools");
    println!("  - Server applications");
    println!("  - Any executable program");
}
