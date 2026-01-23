//! # Enumeration Errors - Domain-Specific Error Types
//!
//! **Date**: Monday (1/19) - Week 4 Day 1
//! **Topic**: Representing Errors with Enumerations
//!
//! ## Key Concepts
//! - Use enums when callers need to distinguish between error cases
//! - Implement `std::error::Error` trait with `source()` method
//! - Derive `Display` and `Debug` traits
//! - Ensure `Send + Sync + 'static` for thread safety

use std::error::Error;
use std::fmt;
use std::fs;
use std::io;

// ============================================================================
// EXAMPLE 1: File Copy Error (from book)
// ============================================================================

#[derive(Debug)]
enum CopyError {
    Input(io::Error),
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

fn copy_file(src: &str, dst: &str) -> Result<u64, CopyError> {
    let content = fs::read(src).map_err(CopyError::Input)?;
    let len = content.len() as u64;
    fs::write(dst, content).map_err(CopyError::Output)?;
    Ok(len)
}

// ============================================================================
// EXAMPLE 2: Configuration Parser Error
// ============================================================================

/// **Educational Note**: Demonstrates domain-specific configuration error type
/// with multiple variant patterns (tuple, struct, unit). Shows complete error
/// API design even though not all variants are constructed in examples.
#[derive(Debug)]
#[allow(dead_code)] // Educational example - demonstrates complete error type design
enum ConfigError {
    Io(io::Error),
    Parse { line: usize, msg: String },
    MissingField(String),
    InvalidValue { field: String, value: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "io error: {}", e),
            ConfigError::Parse { line, msg } => {
                write!(f, "parse error at line {}: {}", line, msg)
            }
            ConfigError::MissingField(field) => {
                write!(f, "missing required field: {}", field)
            }
            ConfigError::InvalidValue { field, value } => {
                write!(f, "invalid value '{}' for field '{}'", value, field)
            }
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::Io(err)
    }
}

// ============================================================================
// EXAMPLE 3: HTTP Client Error
// ============================================================================

/// **Educational Note**: Demonstrates HTTP client error patterns including
/// timeout (unit variant), request errors (struct variants), and parsing errors.
/// Shows variety of error variant designs for different failure modes.
#[derive(Debug)]
#[allow(dead_code)] // Educational example - demonstrates HTTP error patterns
enum HttpError {
    Network(String),
    Timeout,
    BadRequest { code: u16, message: String },
    ServerError { code: u16, message: String },
    ParseResponse(String),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::Network(msg) => write!(f, "network error: {}", msg),
            HttpError::Timeout => write!(f, "request timed out"),
            HttpError::BadRequest { code, message } => {
                write!(f, "bad request ({}): {}", code, message)
            }
            HttpError::ServerError { code, message } => {
                write!(f, "server error ({}): {}", code, message)
            }
            HttpError::ParseResponse(msg) => write!(f, "failed to parse response: {}", msg),
        }
    }
}

impl Error for HttpError {}

// ============================================================================
// EXAMPLE 4: Database Error with Nested Errors
// ============================================================================

/// **Educational Note**: Demonstrates error nesting with `source` field to
/// preserve underlying error chains. Shows struct variants with named fields
/// for rich error context.
#[derive(Debug)]
#[allow(dead_code)] // Educational example - demonstrates error chaining patterns
enum DbError {
    Connection { host: String, port: u16, source: io::Error },
    Query { sql: String, reason: String },
    NotFound { table: String, id: i64 },
    Constraint { constraint: String, value: String },
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::Connection { host, port, .. } => {
                write!(f, "failed to connect to {}:{}", host, port)
            }
            DbError::Query { sql, reason } => {
                write!(f, "query failed: {} (sql: {})", reason, sql)
            }
            DbError::NotFound { table, id } => {
                write!(f, "record {} not found in table {}", id, table)
            }
            DbError::Constraint { constraint, value } => {
                write!(f, "constraint '{}' violated by value '{}'", constraint, value)
            }
        }
    }
}

impl Error for DbError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DbError::Connection { source, .. } => Some(source),
            _ => None,
        }
    }
}

// ============================================================================
// DEMONSTRATION: Error Chain Traversal
// ============================================================================

fn print_error_chain(e: &dyn Error) {
    println!("Error: {}", e);
    
    let mut current = e.source();
    let mut level = 1;
    
    while let Some(source) = current {
        println!("  {}. Caused by: {}", level, source);
        current = source.source();
        level += 1;
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== Enumeration Errors - Domain-Specific Error Types ===\n");

    // Example 1: File Copy Error
    println!("1. File Copy Error:");
    match copy_file("nonexistent.txt", "output.txt") {
        Ok(bytes) => println!("   Copied {} bytes", bytes),
        Err(e) => {
            println!("   {}", e);
            print_error_chain(&e);
        }
    }
    println!();

    // Example 2: Configuration Error
    println!("2. Configuration Error:");
    let cfg_err = ConfigError::InvalidValue {
        field: "port".to_string(),
        value: "not_a_number".to_string(),
    };
    println!("   {}", cfg_err);
    println!();

    // Example 3: HTTP Error
    println!("3. HTTP Client Error:");
    let http_err = HttpError::ServerError {
        code: 500,
        message: "Internal Server Error".to_string(),
    };
    println!("   {}", http_err);
    println!();

    // Example 4: Database Error with Source
    println!("4. Database Error with Error Chain:");
    let db_err = DbError::Connection {
        host: "localhost".to_string(),
        port: 5432,
        source: io::Error::new(io::ErrorKind::ConnectionRefused, "connection refused"),
    };
    print_error_chain(&db_err);
    println!();

    // Example 5: Pattern Matching on Error Types
    println!("5. Pattern Matching on Errors:");
    let errors: Vec<Box<dyn Error>> = vec![
        Box::new(cfg_err),
        Box::new(http_err),
    ];
    
    for (i, err) in errors.iter().enumerate() {
        println!("   Error {}: {}", i + 1, err);
    }
}
