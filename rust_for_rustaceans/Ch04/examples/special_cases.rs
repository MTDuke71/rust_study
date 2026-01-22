// Uncomment for nightly Rust:
// #![feature(never_type)]
//! # Special Error Cases
//!
//! **Date**: Wednesday (1/21) - Week 4 Day 3
//! **Topic**: Unit Errors, Never Type, and Special Error Patterns
//!
//! ## Key Concepts
//! - Unit error `Result<T, ()>` - failure without information
//! - Difference between `Result<T, ()>` and `Option<T>`
//! - Never type `!` for infallible operations (nightly)
//! - Thread panic results with `Box<dyn Any>`
//! - When to use each pattern

use std::thread;

// ============================================================================
// EXAMPLE 1: Unit Error Type
// ============================================================================

/// Validates input without providing detailed error information.
///
/// `Result<T, ()>` signals that an operation can fail, but doesn't
/// provide details about WHY it failed.
fn validate_positive(n: i32) -> Result<i32, ()> {
    if n > 0 {
        Ok(n)
    } else {
        Err(())
    }
}

/// Validates range membership.
fn in_range(n: i32, min: i32, max: i32) -> Result<i32, ()> {
    if n >= min && n <= max {
        Ok(n)
    } else {
        Err(())
    }
}

// ============================================================================
// EXAMPLE 2: Unit Error vs Option
// ============================================================================

/// Using `Option<T>` - represents absence of value
fn find_even_number(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find(|&&n| n % 2 == 0).copied()
}

/// Using `Result<T, ()>` - represents operation that can fail
fn parse_even_number(s: &str) -> Result<i32, ()> {
    match s.parse::<i32>() {
        Ok(n) if n % 2 == 0 => Ok(n),
        _ => Err(()),
    }
}

// Comparison:
// - `Option::None` = "there is no even number in this list"
// - `Result::Err(())` = "the operation to parse an even number failed"

// ============================================================================
// EXAMPLE 3: Boolean vs Unit Error
// ============================================================================

// ❌ Not idiomatic - returns bool
fn is_valid_password_bool(password: &str) -> bool {
    password.len() >= 8 && password.chars().any(|c| c.is_numeric())
}

// ✅ More idiomatic - returns Result
fn validate_password(password: &str) -> Result<(), ()> {
    if password.len() >= 8 && password.chars().any(|c| c.is_numeric()) {
        Ok(())
    } else {
        Err(())
    }
}

// Even better - return the password itself if valid
fn validate_password_return(password: String) -> Result<String, ()> {
    if password.len() >= 8 && password.chars().any(|c| c.is_numeric()) {
        Ok(password)
    } else {
        Err(())
    }
}

// ============================================================================
// EXAMPLE 4: Thread Panic Results
// ============================================================================

/// Demonstrates `std::thread::Result` which uses `Box<dyn Any + Send>`
/// because panics can be of any type.
fn spawn_thread_example() {
    let handle1 = thread::spawn(|| {
        panic!("panic with &str");
    });

    let handle2 = thread::spawn(|| {
        panic!("{}", 42); // panic with i32 formatted as string
    });

    let handle3 = thread::spawn(|| {
        // Success case
        "completed successfully"
    });

    // Join returns Result<T, Box<dyn Any + Send>>
    println!("Thread 1 result: {:?}", handle1.join());
    println!("Thread 2 result: {:?}", handle2.join());
    
    match handle3.join() {
        Ok(result) => println!("Thread 3 result: {}", result),
        Err(e) => {
            if let Some(s) = e.downcast_ref::<&str>() {
                println!("Thread panicked with: {}", s);
            }
        }
    }
}

// ============================================================================
// EXAMPLE 5: Never Type (Requires Nightly)
// ============================================================================

// Note: This requires #![feature(never_type)]
// Uncomment below and run with `cargo +nightly`
// Output Observed:
// 5. Never Type (Nightly Feature):
// Got: 42
// Infallible result: 42

/*
/// Infallible operation - always succeeds
fn always_succeeds() -> Result<i32, !> {
    Ok(42)
}

/// The compiler knows this never returns an error, enabling optimizations
fn use_infallible() {
    match always_succeeds() {
        Ok(n) => println!("Got: {}", n),
        // No need to handle Err case - it's impossible!
    }
}
*/

// ============================================================================
// EXAMPLE 6: Combining Special Cases
// ============================================================================

/// Chain of validations using unit errors
fn validate_user_input(input: &str) -> Result<String, ()> {
    // Multiple validation steps
    validate_not_empty(input)?;
    validate_length(input, 3, 20)?;
    validate_alphanumeric(input)?;
    
    Ok(input.to_string())
}

fn validate_not_empty(s: &str) -> Result<(), ()> {
    if s.is_empty() {
        Err(())
    } else {
        Ok(())
    }
}

fn validate_length(s: &str, min: usize, max: usize) -> Result<(), ()> {
    let len = s.len();
    if len >= min && len <= max {
        Ok(())
    } else {
        Err(())
    }
}

fn validate_alphanumeric(s: &str) -> Result<(), ()> {
    if s.chars().all(|c| c.is_alphanumeric()) {
        Ok(())
    } else {
        Err(())
    }
}

// ============================================================================
// EXAMPLE 7: When to Use Each Pattern
// ============================================================================

// Use `Option<T>` when:
fn first_element<T>(slice: &[T]) -> Option<&T> {
    slice.first() // Absence of value is natural
}

// Use `Result<T, ()>` when:
fn parse_binary(s: &str) -> Result<u32, ()> {
    // Operation can fail, but error details don't matter
    u32::from_str_radix(s, 2).map_err(|_| ())
}

// Use specific error type when:
use std::num::ParseIntError;
fn parse_with_error(s: &str) -> Result<i32, ParseIntError> {
    // Caller needs error details
    s.parse()
}

// Use `Result<(), E>` for side-effect operations:
use std::fs;
use std::io;
fn write_log(message: &str) -> Result<(), io::Error> {
    // No meaningful value to return on success
    fs::write("log.txt", message)
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== Special Error Cases ===\n");

    // Example 1: Unit error
    println!("1. Unit Error Type:");
    match validate_positive(42) {
        Ok(n) => println!("   Valid: {}", n),
        Err(()) => println!("   Invalid"),
    }
    match validate_positive(-5) {
        Ok(n) => println!("   Valid: {}", n),
        Err(()) => println!("   Invalid (negative)"),
    }
    match in_range(50, 0, 100) {
        Ok(n) => println!("   In range: {}", n),
        Err(()) => println!("   Out of range"),
    }
    println!();

    // Example 2: Option vs Result
    println!("2. Option vs Result<T, ()>:");
    let numbers = [1, 3, 5, 7];
    match find_even_number(&numbers) {
        Some(n) => println!("   Found even: {}", n),
        None => println!("   No even numbers found"),
    }
    match parse_even_number("42") {
        Ok(n) => println!("   Parsed even: {}", n),
        Err(()) => println!("   Failed to parse even number"),
    }
    match parse_even_number("43") {
        Ok(n) => println!("   Parsed even: {}", n),
        Err(()) => println!("   Failed to parse even number (43 is odd)"),
    }
    println!();

    // Example 3: Password validation
    println!("3. Boolean vs Result (Invalid Password):");
    let password = "pass123";
    println!("   Boolean approach: {}", is_valid_password_bool(password));
    match validate_password(password) {
        Ok(()) => println!("   Result approach: valid"),
        Err(()) => println!("   Result approach: invalid"),
    }
    match validate_password_return(password.to_string()) {
        Ok(validated) => println!("   Validated password: {}", validated),
        Err(()) => println!("   Invalid password"),
    }
    println!();

    // Example 3a: Password validation with valid password
    println!("3a. Boolean vs Result (Valid Password):");
    let valid_password = "password123";
    println!("   Boolean approach: {}", is_valid_password_bool(valid_password));
    match validate_password(valid_password) {
        Ok(()) => println!("   Result approach: valid"),
        Err(()) => println!("   Result approach: invalid"),
    }
    match validate_password_return(valid_password.to_string()) {
        Ok(validated) => println!("   Validated password: {}", validated),
        Err(()) => println!("   Invalid password"),
    }
    println!();

    // Example 4: Thread panics
    println!("4. Thread Panic Results:");
    spawn_thread_example();
    println!();

    // Example 5: Never type (infallible operations)
    // Output
    // 5. Never Type (Nightly Feature):
    // Got: 42
    // Infallible result: 42
    // Uncomment below and run with `cargo +nightly`
    
    // println!("5. Never Type (Nightly Feature):");
    // use_infallible();
    // match always_succeeds() {
    //    Ok(n) => println!("   Infallible result: {}", n),
    // Compiler knows Err is impossible - no need for Err arm!
    // }
    // println!();

    // Example 6: Chained validations
    println!("6. Chained Validations:");
    match validate_user_input("user123") {
        Ok(validated) => println!("   Valid input: {}", validated),
        Err(()) => println!("   Invalid input"),
    }
    match validate_user_input("ab") {
        Ok(validated) => println!("   Valid input: {}", validated),
        Err(()) => println!("   Invalid input (too short)"),
    }
    match validate_user_input("user@123") {
        Ok(validated) => println!("   Valid input: {}", validated),
        Err(()) => println!("   Invalid input (not alphanumeric)"),
    }
    println!();

    // Example 7: Using different pattern functions
    println!("7. Different Pattern Usage:");
    let numbers = [1, 2, 3, 4, 5];
    match first_element(&numbers) {
        Some(&n) => println!("   First element: {}", n),
        None => println!("   Empty slice"),
    }
    match parse_binary("1010") {
        Ok(n) => println!("   Binary parsed: {}", n),
        Err(()) => println!("   Failed to parse binary"),
    }
    match parse_with_error("123") {
        Ok(n) => println!("   Parsed with error type: {}", n),
        Err(e) => println!("   Parse error: {}", e),
    }
    match parse_with_error("not_a_number") {
        Ok(n) => println!("   Parsed: {}", n),
        Err(e) => println!("   Parse error with details: {}", e),
    }
    match write_log("Example log message") {
        Ok(()) => println!("   Log written successfully"),
        Err(e) => println!("   Failed to write log: {}", e),
    }
    println!();

    println!("Summary:");
    println!("  - Use Option<T> when there might not be a value");
    println!("  - Use Result<T, ()> when operation can fail without details");
    println!("  - Use Result<(), E> for side-effect operations");
    println!("  - Use specific error types when caller needs details");
}
