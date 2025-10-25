// Day 31 Alternative - Using Real anyhow and thiserror Crates
// Demonstrates the power and simplicity of the actual libraries

// To run this example, you need to add dependencies to Cargo.toml:
// [dependencies]
// anyhow = "1.0"
// thiserror = "1.0"
// futures = "0.3"
// tokio = { version = "1.0", features = ["rt", "macros"] }

use anyhow::{Context, Result as AnyhowResult, bail};
use thiserror::Error;
use std::error::Error as StdError;

// Using real thiserror - much simpler with derive macros!
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("IO error")]
    Io(#[from] std::io::Error),  // Automatic From conversion!
    
    #[error("Parse error at line {line}: {message}")]
    Parse { line: usize, message: String },
    
    #[error("Validation error: {field} - {reason}")]
    Validation { field: String, reason: String },
    
    #[error("Configuration error: {message}")]
    Config { message: String },
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Connection failed")]
    ConnectionFailed,
    
    #[error("Query timeout")]
    QueryTimeout,
}

#[derive(Error, Debug)]
pub enum WebError {
    #[error("Invalid request: {message}")]
    InvalidRequest { message: String },
    
    #[error("Database error")]
    Database(#[from] DatabaseError),  // Automatic error chaining!
    
    #[error("Authentication failed: {reason}")]
    Auth { reason: String },
    
    #[error("Rate limit exceeded")]
    RateLimited,
}

#[derive(Error, Debug)]
pub enum ProcessingError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("Invalid file format: expected {expected}, got {actual}")]
    InvalidFormat { expected: String, actual: String },
    
    #[error("Processing failed: {stage}")]
    ProcessingFailed { stage: String },
}

// Data structures
#[derive(Debug)]
#[allow(dead_code)]  // Used in examples but not always accessed
struct Config {
    host: String,
    port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
        }
    }
}

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Debug)]
struct Permissions {
    can_access_api: bool,
}

// Example functions using real anyhow - much cleaner!
fn process_file(path: &str) -> AnyhowResult<String> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path))?;
    
    let processed = content.trim();
    if processed.is_empty() {
        bail!("File is empty: {}", path);  // anyhow's bail! macro
    }
    
    Ok(processed.to_uppercase())
}

fn process_line(line: &str) -> AnyhowResult<()> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 2 {
        bail!("Invalid format: expected 'key,value', got '{}'", line);
    }
    
    let key = parts[0].trim();
    let value: i32 = parts[1].trim().parse()
        .with_context(|| format!("Invalid number: '{}'", parts[1]))?;
    
    println!("Key: {}, Value: {}", key, value);
    Ok(())
}

fn process_user_input(input: &str) -> AnyhowResult<()> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        bail!("Input is empty");
    }
    
    for (line_num, line) in lines.iter().enumerate() {
        process_line(line)
            .with_context(|| format!("Failed to process line {}", line_num + 1))?;
    }
    
    println!("Successfully processed {} lines", lines.len());
    Ok(())
}

// Example functions using real thiserror - automatic implementations!
impl Config {
    pub fn load_from_file(path: &str) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;  // Automatic conversion from io::Error!
        Self::parse(&content)
    }
    
    pub fn parse(content: &str) -> Result<Self, ConfigError> {
        let mut host = None;
        let mut port = None;
        
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some((key, value)) = parse_key_value(line) {
                match key.as_str() {
                    "host" => {
                        if value.is_empty() {
                            return Err(ConfigError::Validation {
                                field: "host".to_string(),
                                reason: "Host cannot be empty".to_string(),
                            });
                        }
                        host = Some(value);
                    }
                    "port" => {
                        port = Some(value.parse::<u16>()
                            .map_err(|_| ConfigError::Parse {
                                line: line_num + 1,
                                message: format!("Invalid port number: {}", value),
                            })?);
                    }
                    _ => {
                        return Err(ConfigError::Parse {
                            line: line_num + 1,
                            message: format!("Unknown configuration key: {}", key),
                        });
                    }
                }
            } else {
                return Err(ConfigError::Parse {
                    line: line_num + 1,
                    message: "Invalid key=value format".to_string(),
                });
            }
        }
        
        Ok(Config {
            host: host.ok_or_else(|| ConfigError::Config {
                message: "Missing required field: host".to_string(),
            })?,
            port: port.unwrap_or(8080),
        })
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

// Functions with proper error handling (sync version for compatibility)
fn fetch_user_from_database(user_id: u32) -> Result<User, WebError> {
    // Simulate database call
    
    if user_id == 0 {
        return Err(WebError::InvalidRequest {
            message: "Invalid user ID".to_string(),
        });
    }
    
    if user_id == 999 {
        return Err(WebError::Database(DatabaseError::ConnectionFailed));
    }
    
    Ok(User {
        id: user_id,
        name: format!("User {}", user_id),
        email: format!("user{}@example.com", user_id),
    })
}

fn process_user_request(user_id: u32) -> Result<String, WebError> {
    let user = fetch_user_from_database(user_id)?;
    
    let permissions = check_user_permissions(&user)?;
    if !permissions.can_access_api {
        return Err(WebError::Auth {
            reason: "User does not have API access".to_string(),
        });
    }
    
    generate_response(&user)
}

fn check_user_permissions(user: &User) -> Result<Permissions, WebError> {
    Ok(Permissions {
        can_access_api: !user.email.contains("blocked"),
    })
}

fn generate_response(user: &User) -> Result<String, WebError> {
    Ok(format!(
        r#"{{"id": {}, "name": "{}", "email": "{}"}}"#,
        user.id, user.name, user.email
    ))
}

// File processing with anyhow
fn list_input_files(dir: &str) -> AnyhowResult<Vec<String>> {
    let mut files = Vec::new();
    
    // Simulate directory listing
    if dir == "empty" {
        return Ok(files);
    }
    
    // In real code, you might use std::fs::read_dir
    files.push(format!("{}/data1.csv", dir));
    files.push(format!("{}/data2.csv", dir));
    
    Ok(files)
}

fn process_single_file(input_path: &str, output_dir: &str) -> Result<(), ProcessingError> {
    let content = std::fs::read_to_string(input_path)
        .map_err(|_| ProcessingError::FileNotFound {
            path: input_path.to_string(),
        })?;
    
    if !content.contains("CSV") {
        return Err(ProcessingError::InvalidFormat {
            expected: "CSV".to_string(),
            actual: "Unknown".to_string(),
        });
    }
    
    let processed = transform_data(&content)?;
    
    let output_path = format!(
        "{}/processed_{}",
        output_dir,
        std::path::Path::new(input_path)
            .file_name()
            .unwrap()
            .to_string_lossy()
    );
    
    std::fs::write(&output_path, processed).map_err(|_| ProcessingError::ProcessingFailed {
        stage: "Writing output file".to_string(),
    })?;
    
    println!("Processed: {} -> {}", input_path, output_path);
    Ok(())
}

fn transform_data(content: &str) -> Result<String, ProcessingError> {
    if content.contains("ERROR") {
        return Err(ProcessingError::ProcessingFailed {
            stage: "Data transformation".to_string(),
        });
    }
    
    Ok(content.to_uppercase())
}

// Advanced anyhow features
fn demonstrate_error_chain() -> AnyhowResult<()> {
    std::fs::read_to_string("nonexistent.txt")
        .context("Failed to read config file")
        .context("Application startup failed")?;
    
    Ok(())
}

fn demonstrate_error_downcasting() -> AnyhowResult<()> {
    let result: Result<(), ConfigError> = Err(ConfigError::FileNotFound {
        path: "config.toml".to_string(),
    });
    
    // Convert to anyhow::Error
    result.context("Config loading failed")?;
    
    Ok(())
}

// Main function 
fn main() -> AnyhowResult<()> {
    println!("=== Day 31 Alternative: Real anyhow and thiserror ===\n");
    
    // Example 1: anyhow with cleaner syntax
    println!("1. Real anyhow Error Handling:");
    let input_data = "key1,100\nkey2,200\nkey3,invalid";
    match process_user_input(input_data) {
        Ok(()) => println!("Processing completed successfully"),
        Err(e) => {
            println!("Processing failed: {}", e);
            
            // anyhow provides excellent error chain reporting
            for cause in e.chain() {
                println!("  Caused by: {}", cause);
            }
        }
    }
    println!();
    
    // Example 2: thiserror with automatic implementations
    println!("2. Real thiserror Error Handling:");
    let config_content = "host=example.com\nport=9090\ninvalid_key=should_fail";
    match Config::parse(config_content) {
        Ok(config) => println!("Config parsed: {:?}", config),
        Err(e) => {
            println!("Config error: {}", e);
            
            // thiserror provides automatic source chaining
            let mut source = e.source();
            while let Some(err) = source {
                println!("  Caused by: {}", err);
                source = err.source();
            }
        }
    }
    println!();
    
    // Example 3: Web server error handling
    println!("3. Web Server Error Handling:");
    match process_user_request(123) {
        Ok(response) => println!("User response: {}", response),
        Err(e) => println!("Request failed: {}", e),
    }
    
    match process_user_request(999) {
        Ok(response) => println!("User response: {}", response),
        Err(e) => println!("Request failed: {}", e),
    }
    println!();
    
    // Example 4: File processing with better error context
    println!("4. File Processing with Enhanced Error Context:");
    let files = list_input_files("data")?;
    
    for file in files {
        if let Err(e) = process_single_file(&file, "output") {
            println!("File processing failed: {}", e);
        }
    }
    println!();
    
    // Example 5: Advanced anyhow features
    println!("5. Advanced Error Features:");
    
    // Error chain demonstration
    if let Err(e) = demonstrate_error_chain() {
        println!("Error chain example:");
        for (i, cause) in e.chain().enumerate() {
            println!("  {}: {}", i, cause);
        }
    }
    
    // Example 6: Additional anyhow features demonstration
    println!("6. Additional anyhow Features:");
    
    // Demonstrate process_file function
    match process_file("Cargo.toml") {
        Ok(content) => println!("File processed successfully (length: {})", content.len()),
        Err(e) => println!("File processing failed: {}", e),
    }
    
    // Demonstrate Config::load_from_file
    match Config::load_from_file("nonexistent_config.toml") {
        Ok(config) => println!("Config loaded: {:?}", config),
        Err(e) => println!("Config loading failed: {}", e),
    }
    
    // Demonstrate error downcasting
    if let Err(e) = demonstrate_error_downcasting() {
        println!("Error downcasting example: {}", e);
    }
    
    println!("\n=== Key Benefits of Real Libraries ===");
    println!("✅ anyhow benefits:");
    println!("   • bail!() macro for easy error creation");
    println!("   • Automatic error chain reporting");
    println!("   • Context trait for adding error context");
    println!("   • Excellent debugging with backtrace support");
    println!("   • Rich error chain iteration and analysis");
    
    println!("\n✅ thiserror benefits:");
    println!("   • #[derive(Error)] eliminates boilerplate");
    println!("   • #[from] for automatic error conversion");
    println!("   • #[error(\"...\")] for custom error messages");
    println!("   • Automatic Display and Error trait implementations");
    println!("   • Professional error types for library development");
    
    println!("\n=== Code Quality Improvements ===");
    println!("📈 Compared to manual implementation:");
    println!("   • ~80% less boilerplate code");
    println!("   • Better error chain reporting");
    println!("   • More maintainable error handling");
    println!("   • Industry-standard error patterns");
    
    println!("\n=== End of Real Library Examples ===");
    Ok(())
}

// Cargo.toml would need:
/*
[package]
name = "day31_alt"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0"
thiserror = "1.0"
futures = "0.3"
tokio = { version = "1.0", features = ["full"] }
*/