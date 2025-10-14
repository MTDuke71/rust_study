# Day 31 - anyhow and thiserror

**Learning Focus**: Practical error handling crates for production Rust applications

---

## 🎯 Learning Objectives

By the end of this day, you should understand:
- When and why to use `anyhow` vs `thiserror`
- How to create error types with `thiserror`
- How to use `anyhow::Result` for application-level error handling
- Error context and error chains with these crates
- Best practices for choosing between the two approaches
- Integration patterns for complex applications

---

## 📚 Core Concepts

### **anyhow - Application-Level Error Handling**

`anyhow` is designed for application-level error handling where you want to propagate errors with context but don't need strict error type checking.

```rust
use anyhow::{Result, Context, anyhow};

// anyhow::Result is a type alias for Result<T, anyhow::Error>
fn process_file(path: &str) -> Result<String> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path))?;
    
    let processed = content.trim();
    if processed.is_empty() {
        return Err(anyhow!("File is empty: {}", path));
    }
    
    Ok(processed.to_uppercase())
}
```

### **thiserror - Library-Level Error Types**

`thiserror` is designed for libraries where you want to define specific error types that implement `std::error::Error`.

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error at line {line}: {message}")]
    Parse { line: usize, message: String },
    
    #[error("Validation error for field '{field}': {reason}")]
    Validation { field: String, reason: String },
}

fn validate_config(path: &str) -> Result<Config, MyError> {
    let content = std::fs::read_to_string(path)?; // Automatic conversion via #[from]
    
    for (line_num, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            return Err(MyError::Parse {
                line: line_num + 1,
                message: "Empty line not allowed".to_string(),
            });
        }
    }
    
    Ok(Config::default())
}
```

---

## 🔧 Implementation Patterns

### **Pattern 1: anyhow for CLI Applications**

```rust
use anyhow::{Result, Context};
use std::path::PathBuf;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        anyhow::bail!("Usage: {} <input-file>", args[0]);
    }
    
    let input_file = &args[1];
    process_input_file(input_file)?;
    
    Ok(())
}

fn process_input_file(path: &str) -> Result<()> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read input file: {}", path))?;
    
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        anyhow::bail!("Input file is empty: {}", path);
    }
    
    for (line_num, line) in lines.iter().enumerate() {
        process_line(line)
            .with_context(|| format!("Failed to process line {} of {}", line_num + 1, path))?;
    }
    
    println!("Successfully processed {} lines", lines.len());
    Ok(())
}

fn process_line(line: &str) -> Result<()> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid format: expected 'key,value', got '{}'", line);
    }
    
    let key = parts[0].trim();
    let value = parts[1].trim().parse::<i32>()
        .with_context(|| format!("Invalid number: '{}'", parts[1]))?;
    
    println!("Key: {}, Value: {}", key, value);
    Ok(())
}
```

### **Pattern 2: thiserror for Library APIs**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
    
    #[error("Parse error at line {line}: {message}")]
    Parse {
        line: usize,
        message: String,
    },
    
    #[error("Validation error: {field} - {reason}")]
    Validation {
        field: String,
        reason: String,
    },
    
    #[error("Configuration error: {message}")]
    Config { message: String },
}

pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;
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
```

### **Pattern 3: Combining anyhow and thiserror**

```rust
use anyhow::{Result, Context};
use thiserror::Error;

// Library error types using thiserror
#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Connection failed: {message}")]
    ConnectionFailed { message: String },
    
    #[error("Query failed: {sql}")]
    QueryFailed { sql: String, source: Box<dyn std::error::Error + Send + Sync> },
    
    #[error("Transaction failed: {message}")]
    TransactionFailed { message: String },
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Field '{field}' is required")]
    RequiredField { field: String },
    
    #[error("Invalid format for '{field}': {reason}")]
    InvalidFormat { field: String, reason: String },
}

// Application code using anyhow
fn process_user_registration(user_data: &str) -> Result<()> {
    let user = parse_user_data(user_data)
        .context("Failed to parse user registration data")?;
    
    validate_user(&user)
        .context("User validation failed")?;
    
    save_user_to_database(&user)
        .context("Failed to save user to database")?;
    
    println!("User registered successfully: {}", user.name);
    Ok(())
}

fn parse_user_data(data: &str) -> Result<User> {
    let parts: Vec<&str> = data.split(',').collect();
    if parts.len() != 3 {
        anyhow::bail!("Invalid user data format: expected 'name,email,age'");
    }
    
    Ok(User {
        name: parts[0].trim().to_string(),
        email: parts[1].trim().to_string(),
        age: parts[2].trim().parse::<u32>()?,
    })
}

fn validate_user(user: &User) -> Result<(), ValidationError> {
    if user.name.is_empty() {
        return Err(ValidationError::RequiredField {
            field: "name".to_string(),
        });
    }
    
    if !user.email.contains('@') {
        return Err(ValidationError::InvalidFormat {
            field: "email".to_string(),
            reason: "Must contain @ symbol".to_string(),
        });
    }
    
    if user.age < 18 {
        return Err(ValidationError::InvalidFormat {
            field: "age".to_string(),
            reason: "Must be at least 18".to_string(),
        });
    }
    
    Ok(())
}

fn save_user_to_database(user: &User) -> Result<(), DatabaseError> {
    // Simulate database operation
    if user.email.contains("invalid") {
        return Err(DatabaseError::QueryFailed {
            sql: format!("INSERT INTO users VALUES ({}, {}, {})", user.name, user.email, user.age),
            source: Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Database constraint violation"
            )),
        });
    }
    
    Ok(())
}

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: u32,
}
```

---

## 🎮 Practical Applications

### **Application 1: Web Server Error Handling**

```rust
use anyhow::{Result, Context};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebError {
    #[error("Invalid request: {message}")]
    InvalidRequest { message: String },
    
    #[error("Database error: {source}")]
    Database {
        #[from]
        source: DatabaseError,
    },
    
    #[error("Authentication failed: {reason}")]
    Auth { reason: String },
    
    #[error("Rate limit exceeded")]
    RateLimited,
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Connection failed")]
    ConnectionFailed,
    
    #[error("Query timeout")]
    QueryTimeout,
}

async fn handle_user_request(user_id: u32) -> Result<String> {
    let user = fetch_user_from_database(user_id)
        .context("Failed to fetch user data")?;
    
    let permissions = check_user_permissions(&user)
        .context("Failed to check permissions")?;
    
    if !permissions.can_access_api {
        return Err(anyhow!("User {} does not have API access", user_id));
    }
    
    let response = generate_response(&user)
        .context("Failed to generate response")?;
    
    Ok(response)
}

async fn fetch_user_from_database(user_id: u32) -> Result<User, WebError> {
    // Simulate database call
    if user_id == 0 {
        return Err(WebError::InvalidRequest {
            message: "Invalid user ID".to_string(),
        });
    }
    
    if user_id == 999 {
        return Err(WebError::Database {
            source: DatabaseError::ConnectionFailed,
        });
    }
    
    Ok(User {
        id: user_id,
        name: format!("User {}", user_id),
        email: format!("user{}@example.com", user_id),
    })
}

fn check_user_permissions(user: &User) -> Result<Permissions> {
    // Simulate permission check
    Ok(Permissions {
        can_access_api: !user.email.contains("blocked"),
    })
}

fn generate_response(user: &User) -> Result<String> {
    Ok(format!(r#"{{"id": {}, "name": "{}", "email": "{}"}}"#, 
               user.id, user.name, user.email))
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
```

### **Application 2: File Processing Pipeline**

```rust
use anyhow::{Result, Context};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessingError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("Invalid file format: {expected}, got {actual}")]
    InvalidFormat { expected: String, actual: String },
    
    #[error("Processing failed: {stage}")]
    ProcessingFailed { stage: String },
}

fn process_data_pipeline(input_dir: &str, output_dir: &str) -> Result<()> {
    let files = list_input_files(input_dir)
        .with_context(|| format!("Failed to list files in {}", input_dir))?;
    
    if files.is_empty() {
        anyhow::bail!("No input files found in {}", input_dir);
    }
    
    for file_path in files {
        process_single_file(&file_path, output_dir)
            .with_context(|| format!("Failed to process file: {}", file_path))?;
    }
    
    println!("Successfully processed {} files", files.len());
    Ok(())
}

fn list_input_files(dir: &str) -> Result<Vec<String>> {
    let mut files = Vec::new();
    
    // Simulate directory listing
    if dir == "empty" {
        return Ok(files);
    }
    
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
    
    let output_path = format!("{}/processed_{}", 
                             output_dir, 
                             std::path::Path::new(input_path).file_name().unwrap().to_string_lossy());
    
    std::fs::write(&output_path, processed)
        .with_context(|| format!("Failed to write output file: {}", output_path))?;
    
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
```

---

## 🧪 Best Practices

### **When to Use anyhow**

```rust
// ✅ Use anyhow for:
// - CLI applications
// - Application entry points
// - When you want to add context to errors
// - When you don't need strict error type checking

fn main() -> anyhow::Result<()> {
    let config = load_config("config.toml")
        .context("Failed to load configuration")?;
    
    run_application(&config)?;
    Ok(())
}
```

### **When to Use thiserror**

```rust
// ✅ Use thiserror for:
// - Library APIs
// - When you need specific error types
// - When consumers need to handle specific errors
// - When you want automatic Error trait implementation

#[derive(Error, Debug)]
pub enum MyLibraryError {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Network error: {source}")]
    Network {
        #[from]
        source: std::io::Error,
    },
}
```

### **Combining Both Approaches**

```rust
// ✅ Best practice: Use thiserror in libraries, anyhow in applications

// In your library:
#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("Something went wrong")]
    SomethingWentWrong,
}

// In your application:
use anyhow::Result;

fn main() -> Result<()> {
    library_function()
        .context("Library operation failed")?;
    Ok(())
}
```

---

## 🎯 Key Takeaways

1. **Use `anyhow` for applications** - Great for adding context and simplifying error handling
2. **Use `thiserror` for libraries** - Provides specific error types for consumers
3. **Combine both approaches** - Libraries use `thiserror`, applications use `anyhow`
4. **Add context liberally** - Use `.context()` and `.with_context()` to provide helpful error messages
5. **Use `#[from]` for automatic conversions** - Simplifies error type conversions
6. **Consider error chains** - Both crates support error chaining for debugging

---

## 🔗 Related Concepts

- **[[Custom Error Types]]** - Creating domain-specific error types
- **[[Error Propagation]]** - The ? operator and error forwarding
- **[[Result Combinators]]** - Functional error handling methods
- **[[Error Handling Patterns]]** - When to propagate vs handle
- **[[Error Handling Practice]]** - Building robust error handling systems

---

*Tags: #error-handling #anyhow #thiserror #error-crates #production-rust #error-context #error-chaining*

*Links: [[zettel-index]] | [[Custom Error Types]] | [[Error Propagation]] | [[Result Combinators]] | [[Error Handling Patterns]] | [[Week 5 Overview]]*
