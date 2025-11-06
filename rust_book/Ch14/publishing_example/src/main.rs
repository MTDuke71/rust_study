//! Binary example for the publishing-example crate
//!
//! This demonstrates how a library crate can also provide
//! a binary interface.

use publishing_example::Calculator;

fn main() {
    println!("Publishing Example - Calculator Demo");
    println!("====================================");
    
    let calc = Calculator::new();
    
    println!("5 + 3 = {}", calc.add(5, 3));
    println!("10 - 4 = {}", calc.subtract(10, 4));
    println!("6 * 7 = {}", calc.multiply(6, 7));
    
    match calc.divide(20, 4) {
        Some(result) => println!("20 / 4 = {}", result),
        None => println!("Cannot divide by zero!"),
    }
    
    match calc.divide(10, 0) {
        Some(result) => println!("10 / 0 = {}", result),
        None => println!("Cannot divide by zero!"),
    }
}

