//! # Binary Crate Example: Using the Library
//!
//! This demonstrates a **binary crate** that uses the library crate.
//! Binary crates have a main() function and can be executed directly.
//!
//! Run with: `cargo run --bin library_example`

// Import functions and types from our library crate
use crates::{calculate_sum, greet, math, Calculator};

fn main() {
    println!("🦀 Binary Crate Example: Using the Library");
    println!("==========================================");

    println!("This is a binary crate because:");
    println!("  - It has a main() function");
    println!("  - It can be executed directly");
    println!("  - It uses the library crate as a dependency");
    println!();

    // Use the library's greeting function
    greet("Rust Developer");

    // Use the library's calculation function
    let sum = calculate_sum(5, 3);
    println!("5 + 3 = {}", sum);

    // Use the library's Calculator struct
    let mut calc = Calculator::new();
    calc.add(10);
    calc.add(5);
    println!("Calculator value: {}", calc.get_value());

    // Use the library's math module
    println!("Factorial of 5: {}", math::factorial(5));
    println!("Is 17 prime? {}", math::is_prime(17));

    println!("\nBinary crates are useful for:");
    println!("  - Command-line tools");
    println!("  - Server applications");
    println!("  - Any executable program");
    println!("  - Testing library functionality");
}
