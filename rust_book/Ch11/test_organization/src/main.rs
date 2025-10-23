//! # Chapter 11.3: Test Organization
//!
//! Demonstrates how to organize tests effectively, including unit tests,
//! integration tests, and testing private functions.
//!
//! Run with: `cargo test`

use test_organization::{add_two, Calculator, Stack};

pub fn main() {
    println!("🗂️ Chapter 11.3: Test Organization");
    println!("==================================");

    example1_unit_tests();
    example2_testing_private_functions();
    example3_integration_tests();
    example4_test_modules();

    println!("✅ All examples completed!");
    println!("📖 Check the tests/ directory for integration tests");
    println!("📖 Run `cargo test` to see all test types in action");
}

fn example1_unit_tests() {
    println!("🧪 Example 1: Unit Tests");
    println!("========================");

    println!("Unit tests are in the same file as the code they test.");
    println!("They use #[cfg(test)] to compile only during testing.");

    println!();
}

fn example2_testing_private_functions() {
    println!("🔒 Example 2: Testing Private Functions");
    println!("=======================================");

    println!("Rust allows testing private functions because tests are");
    println!("part of the same module and can access private items.");

    let result = add_two(5);
    println!("add_two(5) = {}", result);

    println!();
}

fn example3_integration_tests() {
    println!("🔗 Example 3: Integration Tests");
    println!("===============================");

    println!("Integration tests are in the tests/ directory.");
    println!("They test the public API as external code would use it.");

    println!();
}

fn example4_test_modules() {
    println!("📦 Example 4: Test Module Organization");
    println!("======================================");

    println!("Tests can be organized into submodules for better structure.");
    println!("This is especially useful for large test suites.");

    // Demonstrate using the public API
    let mut calc = Calculator::new();
    calc.add(10.0).multiply(2.0);
    println!(
        "Calculator example: 10 + 0, then * 2 = {}",
        calc.get_value()
    );

    let mut stack = Stack::new();
    stack.push("first");
    stack.push("second");
    println!(
        "Stack example: pushed 'first' and 'second', peek = {:?}",
        stack.peek()
    );

    println!();
}
