//! # Rust for Rustaceans - Chapter 4: Error Handling
//!
//! This module demonstrates core error handling concepts from Chapter 4.
//!
//! ## Topics Covered
//! - Error representation patterns (enums, opaque, special cases)
//! - Error propagation with the `?` operator
//! - The `Try` trait (unstable)
//! - Best practices for error types

use std::error::Error;
use std::fmt;
use std::io;

// ============================================================================
// 1. ENUMERATION ERRORS - Domain-Specific Error Types
// ============================================================================

/// Domain-specific error for a file copy operation.
///
/// This demonstrates the enumeration pattern where callers need to
/// distinguish between different error cases.
///
/// **Educational Note**: This enum demonstrates error pattern design from
/// "Rust for Rustaceans" Ch4. Variants may not all be constructed in examples,
/// but illustrate complete error type design patterns.
#[derive(Debug)]
#[allow(dead_code)] // Educational example - demonstrates error pattern design
enum CopyError {
    /// Error occurred while reading input file
    Input(io::Error),
    /// Error occurred while writing output file
    Output(io::Error),
}

impl fmt::Display for CopyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CopyError::Input(e) => write!(f, "failed to read input: {}", e),
            CopyError::Output(e) => write!(f, "failed to write output: {}", e),
        }
    }
}

impl Error for CopyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CopyError::Input(e) | CopyError::Output(e) => Some(e),
        }
    }
}

// ============================================================================
// 2. OPAQUE ERRORS - Type Erasure
// ============================================================================

/// Demonstrates opaque error handling using trait objects.
///
/// This pattern is useful when the specific error type doesn't matter
/// to the caller, only that an error occurred.
fn process_data() -> Result<String, Box<dyn Error + Send + Sync + 'static>> {
    // Different error types can be combined
    let data = std::fs::read_to_string("data.txt")?;
    let parsed: i32 = data.trim().parse()?;
    Ok(format!("Parsed: {}", parsed))
}

// ============================================================================
// 3. SPECIAL ERROR CASES
// ============================================================================

/// Unit error type - signals failure without information.
///
/// Different from `Option<T>` which signals absence of value.
/// `Result<T, ()>` signals an operation that can fail.
fn validate_positive(n: i32) -> Result<i32, ()> {
    if n > 0 {
        Ok(n)
    } else {
        Err(())
    }
}

// Note: Never type `!` examples require nightly Rust
// #![feature(never_type)]
// fn infallible() -> Result<i32, !> {
//     Ok(42)
// }

// ============================================================================
// 4. ERROR PROPAGATION WITH `?` OPERATOR
// ============================================================================

#[derive(Debug)]
struct ParseError {
    line: usize,
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parse error at line {}: {}", self.line, self.message)
    }
}

impl Error for ParseError {}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        ParseError {
            line: 0,
            message: format!("io error: {}", err),
        }
    }
}

impl From<std::num::ParseIntError> for ParseError {
    fn from(err: std::num::ParseIntError) -> Self {
        ParseError {
            line: 0,
            message: format!("parse int error: {}", err),
        }
    }
}

/// Demonstrates error propagation with `?` operator.
///
/// The `?` operator automatically converts errors using the `From` trait.
fn parse_config_file(path: &str) -> Result<Vec<i32>, ParseError> {
    // `?` converts io::Error to ParseError via From trait
    let content = std::fs::read_to_string(path)?;
    
    let mut numbers = Vec::new();
    for (line_num, line) in content.lines().enumerate() {
        // `?` converts ParseIntError to ParseError via From trait
        match line.trim().parse::<i32>() {
            Ok(n) => numbers.push(n),
            Err(e) => {
                return Err(ParseError {
                    line: line_num + 1,
                    message: e.to_string(),
                });
            }
        }
    }
    
    Ok(numbers)
}

// ============================================================================
// 5. COMPREHENSIVE ERROR TYPE
// ============================================================================

/// A well-designed error type following best practices:
/// - Implements `Error`, `Display`, and `Debug`
/// - Provides `source()` for error chain traversal
/// - Is `Send + Sync + 'static` for thread safety and downcasting
///
/// **Educational Note**: Demonstrates comprehensive error type design with
/// multiple variant patterns. Not all variants constructed in examples, but
/// shows complete real-world error API design.
#[derive(Debug)]
#[allow(dead_code)] // Educational example - shows complete error type design
enum DatabaseError {
    Connection(String),
    Query(String),
    NotFound { table: String, id: i32 },
    Duplicate { table: String, key: String },
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::Connection(msg) => write!(f, "connection failed: {}", msg),
            DatabaseError::Query(msg) => write!(f, "query failed: {}", msg),
            DatabaseError::NotFound { table, id } => {
                write!(f, "record not found in table '{}' with id {}", table, id)
            }
            DatabaseError::Duplicate { table, key } => {
                write!(f, "duplicate key '{}' in table '{}'", key, table)
            }
        }
    }
}

impl Error for DatabaseError {}

// ============================================================================
// MAIN DEMONSTRATION
// ============================================================================

fn main() {
    println!("=== Rust for Rustaceans - Chapter 4: Error Handling ===\n");

    // 1. Enumeration Errors
    println!("1. Enumeration Errors:");
    let copy_err = CopyError::Input(io::Error::new(io::ErrorKind::NotFound, "file.txt"));
    println!("   Error: {}", copy_err);
    println!("   Source: {:?}", copy_err.source());
    println!();

    // 2. Opaque Errors
    println!("2. Opaque Errors:");
    match process_data() {
        Ok(result) => println!("   Success: {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // 3. Special Cases
    println!("3. Special Error Cases:");
    match validate_positive(42) {
        Ok(n) => println!("   Valid: {}", n),
        Err(()) => println!("   Invalid number"),
    }
    match validate_positive(-5) {
        Ok(n) => println!("   Valid: {}", n),
        Err(()) => println!("   Invalid number (negative)"),
    }
    println!();

    // 4. Error Propagation
    println!("4. Error Propagation:");
    match parse_config_file("config.txt") {
        Ok(numbers) => println!("   Parsed numbers: {:?}", numbers),
        Err(e) => println!("   Parse error: {}", e),
    }
    println!();

    // 5. Comprehensive Error Type
    println!("5. Comprehensive Error Type:");
    let db_err = DatabaseError::NotFound {
        table: "users".to_string(),
        id: 42,
    };
    println!("   Error: {}", db_err);
    println!("   Debug: {:?}", db_err);
    println!();

    // 6. Error Downcasting (requires 'static)
    println!("6. Error Downcasting:");
    let boxed_err: Box<dyn Error + Send + Sync + 'static> = Box::new(db_err);
    if let Some(db_err) = boxed_err.downcast_ref::<DatabaseError>() {
        println!("   Successfully downcast to DatabaseError: {:?}", db_err);
    }
}
