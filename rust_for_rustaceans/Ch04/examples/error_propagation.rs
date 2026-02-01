//! # Error Propagation - The `?` Operator and Error Chains
//!
//! **Date**: Thursday (1/22) - Week 4 Day 4
//! **Topic**: Propagating Errors with `?` Operator and `From` Trait
//!
//! ## Key Concepts
//! - The `?` operator for early return on errors
//! - Automatic error conversion via `From` trait
//! - Error chains and context preservation
//! - The unstable `Try` trait (conceptual understanding)
//! - Try blocks for scoped error handling (unstable)

use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;

// ============================================================================
// EXAMPLE 1: Basic `?` Operator
// ============================================================================

/// Manual error propagation (verbose)
///
/// **Educational Note**: Demonstrates verbose manual error propagation pattern
/// for comparison with `?` operator. Not called in examples but illustrates
/// the pattern that `?` operator replaces.
#[allow(dead_code)] // Educational example - shows manual propagation pattern
fn read_number_manual(path: &str) -> Result<i32, io::Error> {
    let content = fs::read_to_string(path)?;

    match content.trim().parse() {
        Ok(n) => Ok(n),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
}

/// Using `?` operator (concise)
fn read_number_with_try(path: &str) -> Result<i32, io::Error> {
    let content = fs::read_to_string(path)?;
    content
        .trim()
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

// ============================================================================
// EXAMPLE 2: Error Conversion with `From` Trait
// ============================================================================

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(ParseIntError),
    Validation(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "io error: {}", e),
            AppError::Parse(e) => write!(f, "parse error: {}", e),
            AppError::Validation(s) => write!(f, "validation error: {}", s),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            AppError::Parse(e) => Some(e),
            AppError::Validation(_) => None,
        }
    }
}

// Automatic conversion from io::Error
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

// Automatic conversion from ParseIntError
impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::Parse(err)
    }
}

/// The `?` operator automatically converts errors using `From`
fn process_file(path: &str) -> Result<i32, AppError> {
    // io::Error automatically converted to AppError::Io
    let content = fs::read_to_string(path)?;

    // ParseIntError automatically converted to AppError::Parse
    let number: i32 = content.trim().parse()?;

    // Manual error creation for validation
    if number < 0 {
        return Err(AppError::Validation(
            "number must be non-negative".to_string(),
        ));
    }

    Ok(number)
}

// ============================================================================
// EXAMPLE 3: Error Chains and Context
// ============================================================================

#[derive(Debug)]
struct ConfigError {
    context: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.context)
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}

impl ConfigError {
    fn new(context: impl Into<String>) -> Self {
        ConfigError {
            context: context.into(),
            source: None,
        }
    }

    fn with_source(context: impl Into<String>, source: impl Error + Send + Sync + 'static) -> Self {
        ConfigError {
            context: context.into(),
            source: Some(Box::new(source)),
        }
    }
}

fn load_config() -> Result<String, ConfigError> {
    fs::read_to_string("config.toml")
        .map_err(|e| ConfigError::with_source("failed to load config file", e))
}

fn parse_config(content: &str) -> Result<i32, ConfigError> {
    content
        .trim()
        .parse()
        .map_err(|e| ConfigError::with_source("failed to parse port number", e))
}

fn initialize_server() -> Result<i32, ConfigError> {
    let content = load_config()?;
    let port = parse_config(&content)?;

    if port < 1024 {
        return Err(ConfigError::new("port must be >= 1024"));
    }

    Ok(port)
}

// ============================================================================
// EXAMPLE 4: Multiple Error Types with map_err
// ============================================================================

fn read_config_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("failed to read {}: {}", path, e))
}

fn parse_port(s: &str) -> Result<u16, String> {
    s.parse().map_err(|e| format!("invalid port number: {}", e))
}

fn setup_server(config_path: &str) -> Result<u16, String> {
    let content = read_config_file(config_path)?;
    let port = parse_port(content.trim())?;
    Ok(port)
}

// ============================================================================
// EXAMPLE 5: Chaining Operations
// ============================================================================

#[derive(Debug)]
struct DataProcessor {
    data: Vec<i32>,
}

impl DataProcessor {
    fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        let data: Result<Vec<i32>, _> = content.lines().map(|line| line.trim().parse()).collect();

        Ok(DataProcessor { data: data? })
    }

    fn validate(&self) -> Result<(), Box<dyn Error>> {
        if self.data.is_empty() {
            return Err("data cannot be empty".into());
        }

        if self.data.iter().any(|&n| n < 0) {
            return Err("all numbers must be non-negative".into());
        }

        Ok(())
    }

    fn process(&self) -> Result<i32, Box<dyn Error>> {
        self.validate()?;
        Ok(self.data.iter().sum())
    }
}

// ============================================================================
// EXAMPLE 6: Result Extension Methods
// ============================================================================

trait ResultExt<T, E> {
    /// Add context to an error
    fn context(self, context: &str) -> Result<T, String>
    where
        E: fmt::Display;

    /// Transform error with a function
    fn map_err_ctx<F>(self, f: F) -> Result<T, String>
    where
        E: fmt::Display,
        F: FnOnce(E) -> String;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn context(self, context: &str) -> Result<T, String>
    where
        E: fmt::Display,
    {
        self.map_err(|e| format!("{}: {}", context, e))
    }

    fn map_err_ctx<F>(self, f: F) -> Result<T, String>
    where
        E: fmt::Display,
        F: FnOnce(E) -> String,
    {
        self.map_err(f)
    }
}

fn use_result_ext() -> Result<i32, String> {
    let content = fs::read_to_string("data.txt").context("failed to read data file")?;

    let number: i32 = content
        .trim()
        .parse()
        .map_err_ctx(|e| format!("invalid number format: {}", e))?;

    Ok(number)
}

// ============================================================================
// EXAMPLE 7: Traversing Error Chains
// ============================================================================

fn print_error_chain(err: &dyn Error) {
    eprintln!("Error: {}", err);

    let mut current = err.source();
    let mut depth = 1;

    while let Some(source) = current {
        eprintln!("  {}: Caused by: {}", depth, source);
        current = source.source();
        depth += 1;
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== Error Propagation ===\n");

    // Example 1: Basic ? operator
    println!("1. Basic ? Operator:");
    match read_number_with_try("number.txt") {
        Ok(n) => println!("   Read: {}", n),
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Example 2: Automatic conversion
    println!("2. Automatic Error Conversion:");
    match process_file("data.txt") {
        Ok(n) => println!("   Processed: {}", n),
        Err(e) => {
            println!("   Error: {}", e);
            if let Some(source) = e.source() {
                println!("   Caused by: {}", source);
            }
        }
    }
    println!();

    // Example 3: Error chains
    println!("3. Error Chains and Context:");
    match initialize_server() {
        Ok(port) => println!("   Server port: {}", port),
        Err(e) => print_error_chain(&e),
    }
    println!();

    // Example 4: map_err
    println!("4. Error Transformation with map_err:");
    match setup_server("config.toml") {
        Ok(port) => println!("   Port: {}", port),
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Example 5: Chaining operations
    println!("5. Chaining Operations:");
    match DataProcessor::load("data.txt").and_then(|p| p.process()) {
        Ok(sum) => println!("   Sum: {}", sum),
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Example 6: Result extensions
    println!("6. Result Extension Methods:");
    match use_result_ext() {
        Ok(n) => println!("   Number: {}", n),
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Example 7: Traversing error chains
    println!("7. Traversing Error Chains:");
    // Create a nested error to demonstrate traversal
    match initialize_server() {
        Ok(port) => println!("   Server initialized on port: {}", port),
        Err(e) => {
            println!("   Error chain:");
            print_error_chain(&e);
        }
    }
    println!();

    println!("Summary:");
    println!("  - Use `?` for concise error propagation");
    println!("  - Implement `From` for automatic error conversion");
    println!("  - Use `map_err` to add context to errors");
    println!("  - Preserve error chains with `source()` method");
    println!("  - Traverse error chains to diagnose root causes");
}
