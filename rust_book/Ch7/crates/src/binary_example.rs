//! # Binary Crate Example: Standalone Program
//!
//! This demonstrates a **binary crate** that doesn't use any external libraries.
//! It's a standalone executable program.
//!
//! Run with: `cargo run --bin binary_example`

fn main() {
    println!("🦀 Binary Crate Example: Standalone Program");
    println!("===========================================");

    println!("This is a binary crate because:");
    println!("  - It has a main() function");
    println!("  - It can be executed directly");
    println!("  - It's defined in Cargo.toml as [[bin]]");
    println!("  - It doesn't depend on external libraries");
    println!();

    // Demonstrate some basic functionality
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    let average = sum as f64 / numbers.len() as f64;

    println!("Numbers: {:?}", numbers);
    println!("Sum: {}", sum);
    println!("Average: {:.2}", average);

    println!("\nBinary crates are useful for:");
    println!("  - Command-line tools");
    println!("  - Server applications");
    println!("  - Any executable program");
    println!("  - Standalone utilities");
}
