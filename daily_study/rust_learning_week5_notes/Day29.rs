// Day 29 - Custom Error Types
// Runnable examples demonstrating custom error type implementation

#![allow(dead_code, unused_imports)]

use std::error::Error;
use std::fmt;

// Example 1: Basic Custom Error Type
#[derive(Debug)]
struct ParseError {
    message: String,
    line: usize,
    column: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parse error at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl Error for ParseError {}

// Example 2: Error with Source Information
#[derive(Debug)]
#[allow(dead_code)] // Demo types for learning purposes
enum FileOperationError {
    NotFound { path: String },
    PermissionDenied { path: String },
    IoError { source: std::io::Error },
}

impl fmt::Display for FileOperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileOperationError::NotFound { path } => {
                write!(f, "File not found: {}", path)
            }
            FileOperationError::PermissionDenied { path } => {
                write!(f, "Permission denied: {}", path)
            }
            FileOperationError::IoError { source } => {
                write!(f, "IO error: {}", source)
            }
        }
    }
}

impl Error for FileOperationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FileOperationError::IoError { source } => Some(source),
            _ => None,
        }
    }
}

// Example 3: Calculator Error Type
#[derive(Debug)]
pub enum CalculatorError {
    DivisionByZero,
    InvalidOperation(String),
    Overflow,
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculatorError::DivisionByZero => {
                write!(f, "Cannot divide by zero")
            }
            CalculatorError::InvalidOperation(op) => {
                write!(f, "Invalid operation: {}", op)
            }
            CalculatorError::Overflow => {
                write!(f, "Arithmetic overflow")
            }
        }
    }
}

impl Error for CalculatorError {}

// Example 4: Validation Error
#[derive(Debug)]
pub struct ValidationError {
    field: String,
    value: String,
    reason: String,
}

impl ValidationError {
    pub fn new(
        field: impl Into<String>,
        value: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            value: value.into(),
            reason: reason.into(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Validation failed for field '{}' with value '{}': {}",
            self.field, self.value, self.reason
        )
    }
}

impl Error for ValidationError {}

// Example 5: Configuration Error
#[derive(Debug)]
pub enum ConfigError {
    FileNotFound {
        path: String,
    },
    ParseError {
        line: usize,
        message: String,
    },
    InvalidFormat {
        key: String,
        expected: String,
        found: String,
    },
    IoError {
        source: std::io::Error,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileNotFound { path } => {
                write!(f, "Configuration file not found: {}", path)
            }
            ConfigError::ParseError { line, message } => {
                write!(f, "Parse error at line {}: {}", line, message)
            }
            ConfigError::InvalidFormat {
                key,
                expected,
                found,
            } => {
                write!(
                    f,
                    "Invalid format for '{}': expected {}, found {}",
                    key, expected, found
                )
            }
            ConfigError::IoError { source } => {
                write!(f, "IO error: {}", source)
            }
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::IoError { source } => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(source: std::io::Error) -> Self {
        ConfigError::IoError { source }
    }
}

#[derive(Debug)]
pub struct Config {
    port: Option<u16>,
    host: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            port: None,
            host: None,
        }
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = Some(port);
    }

    pub fn set_host(&mut self, host: String) {
        self.host = Some(host);
    }

    pub fn port(&self) -> Option<u16> {
        self.port
    }

    pub fn host(&self) -> Option<&str> {
        self.host.as_deref()
    }
}

fn parse_key_value(line: &str) -> Option<(String, String)> {
    if let Some(pos) = line.find('=') {
        let key = line[..pos].trim().to_string();
        let value = line[pos + 1..].trim().to_string();
        Some((key, value))
    } else {
        None
    }
}

fn parse_config(content: &str, _path: &str) -> Result<Config, ConfigError> {
    let mut config = Config::new();

    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = parse_key_value(line) {
            match key.as_str() {
                "port" => {
                    let port = value
                        .parse::<u16>()
                        .map_err(|_| ConfigError::InvalidFormat {
                            key: "port".to_string(),
                            expected: "u16".to_string(),
                            found: value.clone(),
                        })?;
                    config.set_port(port);
                }
                "host" => {
                    config.set_host(value);
                }
                _ => {
                    return Err(ConfigError::ParseError {
                        line: line_num + 1,
                        message: format!("Unknown configuration key: {}", key),
                    });
                }
            }
        } else {
            return Err(ConfigError::ParseError {
                line: line_num + 1,
                message: "Invalid key=value format".to_string(),
            });
        }
    }

    Ok(config)
}

// Example Calculator Implementation
fn divide(a: i32, b: i32) -> Result<f64, CalculatorError> {
    if b == 0 {
        return Err(CalculatorError::DivisionByZero);
    }

    let result = a as f64 / b as f64;
    if result.is_infinite() {
        return Err(CalculatorError::Overflow);
    }

    Ok(result)
}

fn validate_age(age: i32) -> Result<i32, ValidationError> {
    if age < 0 {
        return Err(ValidationError::new(
            "age",
            age.to_string(),
            "Age cannot be negative",
        ));
    }
    if age > 150 {
        return Err(ValidationError::new(
            "age",
            age.to_string(),
            "Age cannot exceed 150",
        ));
    }
    Ok(age)
}

fn main() {
    println!("=== Day 29: Custom Error Types Examples ===\n");

    // Example 1: Basic Parse Error
    println!("1. Basic Parse Error:");
    let parse_err = ParseError {
        message: "Unexpected token".to_string(),
        line: 10,
        column: 5,
    };
    println!("Error: {}", parse_err);
    println!("Debug: {:?}\n", parse_err);

    // Example 2: Calculator Errors
    println!("2. Calculator Error Handling:");
    match divide(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(i32::MAX, 1) {
        Ok(result) => println!("MAX / 1 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Example 3: Validation Errors
    println!("3. Validation Error Handling:");
    for age in [-5, 25, 200] {
        match validate_age(age) {
            Ok(valid_age) => println!("Age {} is valid", valid_age),
            Err(e) => println!("Validation failed: {}", e),
        }
    }
    println!();

    // Example 4: Configuration Parsing
    println!("4. Configuration Parsing:");
    let config_content = r#"
# Server configuration
host=localhost
port=8080
invalid_key=should_fail
"#;

    match parse_config(config_content, "config.txt") {
        Ok(config) => {
            println!("Configuration loaded successfully:");
            println!("  Host: {:?}", config.host());
            println!("  Port: {:?}", config.port());
        }
        Err(e) => println!("Configuration error: {}", e),
    }
    println!();

    // Example 5: Error Chaining
    println!("5. Error Chaining Example:");
    let file_operation_error = FileOperationError::IoError {
        source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
    };

    println!("File operation error: {}", file_operation_error);
    if let Some(source) = file_operation_error.source() {
        println!("Source error: {}", source);
    }
    println!();

    println!("=== End of Day 29 Examples ===");
}
