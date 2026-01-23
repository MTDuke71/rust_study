//! # Using thiserror and anyhow Crates
//!
//! **Supplemental Example**: Production Error Handling Libraries
//!
//! ## Key Concepts
//! - `thiserror` for library error types (derive macro magic)
//! - `anyhow` for application error handling (context and convenience)
//! - When to use each crate
//! - Integration patterns

use std::io;
use std::num::ParseIntError;
use thiserror::Error;
use anyhow::{Context, Result, anyhow, bail};

// ============================================================================
// PART 1: thiserror for Library Errors
// ============================================================================

/// Library error using thiserror derive macro.
///
/// thiserror automatically implements:
/// - Display trait
/// - Error trait
/// - From conversions
#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("failed to read data from {path}")]
    Read {
        path: String,
        #[source]
        source: io::Error,
    },
    
    #[error("failed to write data to {path}")]
    Write {
        path: String,
        #[source]
        source: io::Error,
    },
    
    #[error("invalid data format: {0}")]
    InvalidFormat(String),
    
    #[error("record not found: {0}")]
    NotFound(String),
    
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Configuration error with detailed variants
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("configuration file not found at {path}")]
    NotFound { path: String },
    
    #[error("failed to parse configuration")]
    Parse {
        #[from]
        source: ParseIntError,
    },
    
    #[error("invalid value for {field}: {value}")]
    InvalidValue { field: String, value: String },
    
    #[error("missing required field: {0}")]
    MissingField(String),
}

// ============================================================================
// PART 2: anyhow for Application Errors
// ============================================================================

/// Application function using anyhow::Result.
///
/// anyhow::Result<T> is an alias for Result<T, anyhow::Error>
/// which can hold any error type.
fn load_user_data(user_id: &str) -> Result<String> {
    let path = format!("users/{}.json", user_id);
    
    // Context adds information to errors
    let data = std::fs::read_to_string(&path)
        .context(format!("failed to read user data from {}", path))?;
    
    // Validate user ID
    let id: i32 = user_id.parse()
        .context("user ID must be a valid number")?;
    
    if id < 0 {
        // bail! macro for early return with error
        bail!("user ID must be non-negative");
    }
    
    Ok(data)
}

/// Demonstrates with_context for lazy evaluation
///
/// **Educational Note**: Shows anyhow's `with_context` for lazy error message
/// generation. Not called in examples but demonstrates the pattern for adding
/// context to errors during propagation.
#[allow(dead_code)] // Educational example - demonstrates context addition pattern
fn process_request(request_id: &str) -> Result<()> {
    let data = std::fs::read_to_string("request.json")
        .with_context(|| format!("failed to read request {}", request_id))?;
    
    if data.is_empty() {
        // anyhow! macro creates ad-hoc errors
        return Err(anyhow!("empty request data for {}", request_id));
    }
    
    Ok(())
}

// ============================================================================
// PART 3: Integration - thiserror + anyhow
// ============================================================================

/// Library function returns specific error type (thiserror)
pub fn library_function(value: i32) -> Result<i32, ConfigError> {
    if value < 0 {
        return Err(ConfigError::InvalidValue {
            field: "value".to_string(),
            value: value.to_string(),
        });
    }
    Ok(value * 2)
}

/// Application function uses anyhow::Result
fn application_function(value: i32) -> Result<i32> {
    // Library error automatically converted to anyhow::Error
    let result = library_function(value)
        .context("failed to process value in library")?;
    
    Ok(result)
}

// ============================================================================
// PART 4: Advanced thiserror Patterns
// ============================================================================

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP {status_code}: {message}")]
    Http {
        status_code: u16,
        message: String,
        #[source]
        source: Option<reqwest::Error>,
    },
    
    #[error("timeout after {duration_ms}ms")]
    Timeout { duration_ms: u64 },
    
    #[error("rate limited (retry after {retry_after}s)")]
    RateLimit { retry_after: u64 },
    
    // Transparent variant forwards Display and Error implementations
    #[error(transparent)]
    Io(#[from] io::Error),
}

// ============================================================================
// PART 5: Error Downcasting with anyhow
// ============================================================================

fn demonstrate_downcasting() -> Result<()> {
    let result: Result<()> = Err(ConfigError::MissingField("port".to_string()).into());
    
    match result {
        Ok(()) => println!("Success"),
        Err(e) => {
            // Downcast to specific error type
            if let Some(cfg_err) = e.downcast_ref::<ConfigError>() {
                println!("Config error: {:?}", cfg_err);
            } else {
                println!("Other error: {}", e);
            }
        }
    }
    
    Ok(())
}

// ============================================================================
// PART 6: Error Context Chains with anyhow
// ============================================================================

fn read_config_file() -> Result<String> {
    std::fs::read_to_string("config.toml")
        .context("failed to read config file")
}

fn parse_port(content: &str) -> Result<u16> {
    content.trim().parse()
        .context("failed to parse port number")
}

fn initialize_app() -> Result<u16> {
    let content = read_config_file()
        .context("initialization failed")?;
    
    let port = parse_port(&content)
        .context("configuration parsing failed")?;
    
    if port < 1024 {
        bail!("port must be >= 1024, got {}", port);
    }
    
    Ok(port)
}

// ============================================================================
// PART 7: Comparison - Manual vs thiserror
// ============================================================================

// Manual implementation (verbose)
#[derive(Debug)]
pub struct ManualError {
    message: String,
}

impl std::fmt::Display for ManualError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ManualError {}

// thiserror implementation (concise)
#[derive(Error, Debug)]
#[error("{message}")]
pub struct ThisError {
    message: String,
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== thiserror and anyhow Crates ===\n");

    // Example 1: thiserror library error
    println!("1. thiserror Library Error:");
    let err = DataStoreError::NotFound("user_123".to_string());
    println!("   {}", err);
    println!();

    // Example 2: anyhow application error
    println!("2. anyhow Application Error:");
    match load_user_data("123") {
        Ok(data) => println!("   Loaded: {}", data),
        Err(e) => {
            println!("   Error: {}", e);
            println!("   Error chain:");
            for (i, cause) in e.chain().enumerate() {
                println!("     {}: {}", i, cause);
            }
        }
    }
    println!();

    // Example 3: Integration
    println!("3. Integration (thiserror + anyhow):");
    match application_function(-5) {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => {
            println!("   Error: {}", e);
            println!("   Debug: {:?}", e);
        }
    }
    println!();

    // Example 4: Error downcasting
    println!("4. Error Downcasting:");
    let _ = demonstrate_downcasting();
    println!();

    // Example 5: Context chains
    println!("5. Context Chains:");
    match initialize_app() {
        Ok(port) => println!("   Port: {}", port),
        Err(e) => {
            println!("   Error: {:?}", e);
            println!("   Chain:");
            for cause in e.chain() {
                println!("     - {}", cause);
            }
        }
    }
    println!();

    println!("Summary:");
    println!("  - Use thiserror for library error types (clean enums)");
    println!("  - Use anyhow for application error handling (context)");
    println!("  - thiserror: explicit, type-safe, exhaustive");
    println!("  - anyhow: flexible, convenient, context-rich");
}

// Note: reqwest::Error used in ApiError example requires adding:
// [dependencies]
// reqwest = "0.11"
// But we'll use a mock for demonstration purposes

#[derive(Debug)]
pub struct MockReqwestError;

impl std::fmt::Display for MockReqwestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mock reqwest error")
    }
}

impl std::error::Error for MockReqwestError {}

mod reqwest {
    pub use super::MockReqwestError as Error;
}
