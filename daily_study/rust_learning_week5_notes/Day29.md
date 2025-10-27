# Day 29 - Custom Error Types

**Learning Focus**: Implementing `std::error::Error` trait for custom error types

---

## 🎯 Learning Objectives

By the end of this day, you should understand:
- How to create custom error types that implement `std::error::Error`
- The relationship between `Error`, `Display`, and `Debug` traits
- When and why to use custom error types vs built-in errors
- Error context and error chaining patterns
- Best practices for error type design

---

## 📚 Core Concepts

### **The Error Trait Hierarchy**

```rust
use std::error::Error;
use std::fmt;

// Custom error types must implement:
// 1. Display (for user-facing error messages)
// 2. Debug (for debugging output)
// 3. Error (for error trait objects)
// 4. 'static lifetime (for error trait objects)
```

### **Basic Custom Error Type**

```rust
#[derive(Debug)]
struct ParseError {
    message: String,
    line: usize,
    column: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error at line {}, column {}: {}", 
               self.line, self.column, self.message)
    }
}

impl Error for ParseError {}
```

### **Error with Source Information**

```rust
#[derive(Debug)]
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
```

---

## 🔧 Implementation Patterns

### **Pattern 1: Simple Domain Errors**

```rust
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
```

### **Pattern 2: Error with Context**

```rust
#[derive(Debug)]
pub struct ValidationError {
    field: String,
    value: String,
    reason: String,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, value: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            value: value.into(),
            reason: reason.into(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Validation failed for field '{}' with value '{}': {}", 
               self.field, self.value, self.reason)
    }
}

impl Error for ValidationError {}
```

### **Pattern 3: Hierarchical Errors**

```rust
#[derive(Debug)]
pub enum NetworkError {
    ConnectionFailed { url: String, source: std::io::Error },
    Timeout,
    InvalidResponse { status: u16, body: String },
    ParsingFailed { source: serde_json::Error },
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkError::ConnectionFailed { url, .. } => {
                write!(f, "Failed to connect to {}", url)
            }
            NetworkError::Timeout => {
                write!(f, "Request timed out")
            }
            NetworkError::InvalidResponse { status, .. } => {
                write!(f, "Invalid response with status {}", status)
            }
            NetworkError::ParsingFailed { .. } => {
                write!(f, "Failed to parse response")
            }
        }
    }
}

impl Error for NetworkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NetworkError::ConnectionFailed { source, .. } => Some(source),
            NetworkError::ParsingFailed { source } => Some(source),
            _ => None,
        }
    }
}
```

---

## 🎮 Practical Applications

### **Application 1: Configuration Parser**

```rust
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound { path: String },
    ParseError { line: usize, message: String },
    InvalidFormat { key: String, expected: String, found: String },
    IoError { source: std::io::Error },
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
            ConfigError::InvalidFormat { key, expected, found } => {
                write!(f, "Invalid format for '{}': expected {}, found {}", 
                       key, expected, found)
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

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, ConfigError> {
    let path_str = path.as_ref().to_string_lossy().to_string();
    
    let content = fs::read_to_string(&path)
        .map_err(|e| ConfigError::IoError { source: e })?;
    
    parse_config(&content, &path_str)
}

fn parse_config(content: &str, path: &str) -> Result<Config, ConfigError> {
    let mut config = Config::new();
    
    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = parse_key_value(line) {
            match key.as_str() {
                "port" => {
                    let port = value.parse::<u16>()
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

fn parse_key_value(line: &str) -> Option<(String, String)> {
    if let Some(pos) = line.find('=') {
        let key = line[..pos].trim().to_string();
        let value = line[pos + 1..].trim().to_string();
        Some((key, value))
    } else {
        None
    }
}

#[derive(Debug)]
pub struct Config {
    port: Option<u16>,
    host: Option<String>,
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
```

### **Application 2: Database Operations**

```rust
#[derive(Debug)]
pub enum DatabaseError {
    ConnectionFailed { message: String },
    QueryFailed { sql: String, source: Box<dyn Error + Send + Sync> },
    NotFound { table: String, id: String },
    ConstraintViolation { constraint: String, message: String },
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::ConnectionFailed { message } => {
                write!(f, "Database connection failed: {}", message)
            }
            DatabaseError::QueryFailed { sql, .. } => {
                write!(f, "Query failed: {}", sql)
            }
            DatabaseError::NotFound { table, id } => {
                write!(f, "Record not found in table '{}' with id '{}'", table, id)
            }
            DatabaseError::ConstraintViolation { constraint, message } => {
                write!(f, "Constraint violation '{}': {}", constraint, message)
            }
        }
    }
}

impl Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DatabaseError::QueryFailed { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

pub struct Database;

impl Database {
    pub fn connect(url: &str) -> Result<Connection, DatabaseError> {
        if url.is_empty() {
            return Err(DatabaseError::ConnectionFailed {
                message: "Empty connection URL".to_string(),
            });
        }
        
        // Simulate connection
        Ok(Connection { url: url.to_string() })
    }
}

pub struct Connection {
    url: String,
}

impl Connection {
    pub fn query<T>(&self, sql: &str) -> Result<Vec<T>, DatabaseError> {
        if sql.trim().is_empty() {
            return Err(DatabaseError::QueryFailed {
                sql: sql.to_string(),
                source: Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Empty SQL query",
                )),
            });
        }
        
        // Simulate query execution
        if sql.contains("SELECT") {
            Ok(vec![]) // Empty result set
        } else {
            Err(DatabaseError::QueryFailed {
                sql: sql.to_string(),
                source: Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Query execution failed",
                )),
            })
        }
    }
}
```

---

## 🧪 Error Handling Best Practices

### **Best Practice 1: Error Context**

```rust
// ❌ BAD: Losing context
fn process_file(filename: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(filename)
}

// ✅ GOOD: Preserving context
fn process_file(filename: &str) -> Result<String, FileOperationError> {
    std::fs::read_to_string(filename)
        .map_err(|source| FileOperationError::IoError { source })
}
```

### **Best Practice 2: Error Chaining**

```rust
// ✅ GOOD: Proper error chaining
fn load_and_parse_config(path: &str) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(path)
        .map_err(|source| ConfigError::IoError { source })?;
    
    parse_config(&content, path)
}
```

### **Best Practice 3: Error Conversion**

```rust
impl From<std::io::Error> for ConfigError {
    fn from(source: std::io::Error) -> Self {
        ConfigError::IoError { source }
    }
}

// Now we can use ? operator directly
fn load_config_simple(path: &str) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(path)?; // Automatically converts
    parse_config(&content, path)
}
```

---

## 🎯 Key Takeaways

1. **Custom errors provide better context** than generic error types
2. **Always implement Display, Debug, and Error traits** for custom errors
3. **Use source() method** to chain errors and preserve error context
4. **Implement From conversions** to enable automatic error conversion with `?`
5. **Design error types** to match your domain and use cases
6. **Consider error hierarchies** for complex applications with multiple error sources

---

## 🔗 Related Concepts

- **[[Error Handling Patterns]]** - Core error handling patterns
- **[[Result Type]]** - The Result<T, E> type for error handling
- **[[Option Type]]** - The Option<T> type for nullable values
- **[[anyhow and thiserror]]** - Advanced error handling crates
- **[[Error Propagation]]** - The ? operator and error forwarding
- **[[../../zettelkasten/daily-study/Day29]]** - Zettelkasten redirect page

---

## 🔗 Navigation

**Previous**: [[Day28]] | **Next**: [[Day30]]

**Week Overview**: [[README|Week 5 Overview]]

**Zettelkasten**: [[../../zettelkasten/daily-study/Day29|Day29 (Zettelkasten)]]

---

*Tags: #error-handling #custom-errors #std-error #error-trait #error-chaining #error-context*

*Links: [[zettel-index]] | [[Error Handling Patterns]] | [[Result Type]] | [[anyhow and thiserror]] | [[Error Propagation]] | [[Week 5 Overview]]*
