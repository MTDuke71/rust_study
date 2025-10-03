//! # Library Crate Example
//!
//! Demonstrates a library crate without main() function.
//!
//! Run with: `cargo run --bin library_example`

// Library crates don't have main() function
// They provide functionality to other programs

pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}

pub fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b
}

pub struct Calculator {
    value: i32,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { value: 0 }
    }
    
    pub fn add(&mut self, n: i32) {
        self.value += n;
    }
    
    pub fn get_value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("📚 Library Crate Example");
    println!("========================");
    
    println!("This is a library crate because:");
    println!("  - It provides functions and structs");
    println!("  - It can be used by other programs");
    println!("  - It's defined in Cargo.toml as [[bin]]");
    
    // Demonstrate the library functionality
    greet("Rust Developer");
    
    let sum = calculate_sum(5, 3);
    println!("5 + 3 = {}", sum);
    
    let mut calc = Calculator::new();
    calc.add(10);
    calc.add(5);
    println!("Calculator value: {}", calc.get_value());
    
    println!("\nLibrary crates are useful for:");
    println!("  - Reusable code");
    println!("  - API libraries");
    println!("  - Shared functionality");
}
