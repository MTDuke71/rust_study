//! # Opaque Errors - Type Erasure for Simplified Error Handling
//!
//! **Date**: Tuesday (1/20) - Week 4 Day 2
//! **Topic**: Representing Errors with Opaque Types
//!
//! ## Key Concepts
//! - Use `Box<dyn Error>` when specific error type doesn't matter
//! - Simplifies function signatures when composing multiple error types
//! - Enables error downcasting with `'static` bound
//! - Trade-off: simplicity vs. exhaustive error handling

use std::error::Error;
use std::fmt;
use std::fs;
use std::io;

// ============================================================================
// EXAMPLE 1: Basic Opaque Error
// ============================================================================

/// Simple function returning opaque error.
///
/// The caller doesn't need to know about specific io::Error or ParseIntError types.
fn read_number_from_file(path: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let number = content.trim().parse()?;
    Ok(number)
}

// ============================================================================
// EXAMPLE 2: Composing Different Error Sources
// ============================================================================

/// Demonstrates composing multiple error types into one opaque error.
fn process_user_data(user_id: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    // IO error
    let data = fs::read_to_string(format!("users/{}.json", user_id))?;
    
    // Parse error (simulated)
    let id: i32 = user_id.parse()?;
    
    // Custom validation error
    if id < 0 {
        return Err("user ID must be non-negative".into());
    }
    
    Ok(format!("User {}: {}", id, data))
}

// ============================================================================
// EXAMPLE 3: Error Downcasting
// ============================================================================

#[derive(Debug)]
struct CustomError {
    code: i32,
    message: String,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error {}: {}", self.code, self.message)
    }
}

impl Error for CustomError {}

fn operation_with_custom_error() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    Err(Box::new(CustomError {
        code: 42,
        message: "something went wrong".to_string(),
    }))
}

fn handle_error_with_downcast() {
    match operation_with_custom_error() {
        Ok(()) => println!("Success"),
        Err(e) => {
            // Attempt to downcast to specific error type
            if let Some(custom) = e.downcast_ref::<CustomError>() {
                println!("CustomError caught! Code: {}, Message: {}", 
                         custom.code, custom.message);
            } else if let Some(io_err) = e.downcast_ref::<io::Error>() {
                println!("IO Error: {}", io_err);
            } else {
                println!("Unknown error: {}", e);
            }
        }
    }
}

// ============================================================================
// EXAMPLE 4: Opaque Errors in APIs
// ============================================================================

/// Public API that doesn't expose internal error types.
///
/// This provides flexibility to change internal implementation without
/// breaking callers.
pub fn public_api_function(input: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Internal implementation can change without affecting signature
    internal_step1(input)?;
    internal_step2(input)?;
    Ok("Success".to_string())
}

fn internal_step1(s: &str) -> Result<(), io::Error> {
    if s.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "empty input"));
    }
    Ok(())
}

fn internal_step2(s: &str) -> Result<(), std::num::ParseIntError> {
    s.parse::<i32>()?;
    Ok(())
}

// ============================================================================
// EXAMPLE 5: Opaque Errors with Context
// ============================================================================

/// Demonstrates adding context to opaque errors.
fn read_config() -> Result<String, Box<dyn Error>> {
    fs::read_to_string("config.toml")
        .map_err(|e| format!("failed to read config file: {}", e).into())
}

fn parse_config() -> Result<(), Box<dyn Error>> {
    let content = read_config()
        .map_err(|e| format!("configuration error: {}", e))?;
    
    // More processing...
    println!("Config: {}", content);
    Ok(())
}

// ============================================================================
// EXAMPLE 6: Thread-Safe Opaque Errors
// ============================================================================

use std::thread;

/// Demonstrates sending errors across threads.
///
/// Requires `Send + Sync + 'static` bounds.
fn spawn_worker() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let handle = thread::spawn(|| -> Result<(), Box<dyn Error + Send + Sync>> {
        // Simulate some work that might fail
        let result: Result<i32, _> = "not a number".parse();
        result?;
        Ok(())
    });
    
    handle.join().unwrap()
}

// ============================================================================
// COMPARISON: Specific vs Opaque Errors
// ============================================================================

// Specific error type (verbose but exhaustive)
#[derive(Debug)]
enum SpecificError {
    Io(io::Error),
    Parse(std::num::ParseIntError),
    Validation(String),
}

impl fmt::Display for SpecificError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpecificError::Io(e) => write!(f, "io: {}", e),
            SpecificError::Parse(e) => write!(f, "parse: {}", e),
            SpecificError::Validation(s) => write!(f, "validation: {}", s),
        }
    }
}

impl Error for SpecificError {}

fn with_specific_error() -> Result<i32, SpecificError> {
    let data = fs::read_to_string("data.txt")
        .map_err(SpecificError::Io)?;
    let num = data.trim().parse()
        .map_err(SpecificError::Parse)?;
    
    if num < 0 {
        return Err(SpecificError::Validation("must be non-negative".to_string()));
    }
    
    Ok(num)
}

fn with_opaque_error() -> Result<i32, Box<dyn Error>> {
    let data = fs::read_to_string("data.txt")?;
    let num: i32 = data.trim().parse()?;
    
    if num < 0 {
        return Err("must be non-negative".into());
    }
    
    Ok(num)
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== Opaque Errors - Type Erasure ===\n");

    // Example 1: Basic opaque error
    println!("1. Basic Opaque Error:");
    match read_number_from_file("number.txt") {
        Ok(n) => println!("   Read number: {}", n),
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Example 2: Composing errors
    println!("2. Composing Multiple Error Types:");
    match process_user_data("123") {
        Ok(result) => println!("   {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Example 3: Error downcasting
    println!("3. Error Downcasting:");
    handle_error_with_downcast();
    println!();

    // Example 4: Public API
    println!("4. Opaque Errors in Public APIs:");
    match public_api_function("42") {
        Ok(result) => println!("   {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Example 5: Context
    println!("5. Opaque Errors with Context:");
    if let Err(e) = parse_config() {
        println!("   {}", e);
    }
    println!();

    // Example 6: Thread safety
    println!("6. Thread-Safe Opaque Errors:");
    if let Err(e) = spawn_worker() {
        println!("   Worker error: {}", e);
    }
    println!();

    // Comparison
    println!("7. Comparison: Specific vs Opaque:");
    println!("   Specific error type: more verbose, exhaustive matching");
    println!("   Opaque error type: simpler, flexible, but less specific");
}
