# 🛠️ Error Handling Patterns

**Comprehensive guide to error handling strategies, patterns, and best practices in Rust**

## 🎯 Overview

Error handling patterns in Rust revolve around the fundamental choice between **recoverable errors** (`Result<T, E>`) and **unrecoverable errors** (panic). This guide covers when to use each approach, common patterns, and best practices for robust error management.

## 🔄 Core Patterns

### **1. Result vs Panic Decision Tree**

```rust
// ✅ Use Result<T, E> for recoverable errors
fn parse_user_input(input: &str) -> Result<u32, ParseIntError> {
    input.parse::<u32>()
}

// ✅ Use panic! for programming errors
fn get_first_element<T>(vec: &Vec<T>) -> &T {
    if vec.is_empty() {
        panic!("Cannot get first element of empty vector");
    }
    &vec[0]
}
```

**Decision Guidelines:**
- **Use `Result`**: User input, file I/O, network operations, data validation
- **Use `panic!`**: Programming bugs, invariant violations, unrecoverable states

### **2. Error Propagation Patterns**

#### **Early Return Pattern**
```rust
fn process_data(data: &str) -> Result<ProcessedData, ProcessingError> {
    let parsed = parse_data(data)?;  // Early return on error
    let validated = validate_data(&parsed)?;  // Early return on error
    let processed = transform_data(validated)?;  // Early return on error
    Ok(processed)
}
```

#### **Error Mapping Pattern**
```rust
fn read_config_file(path: &str) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| ConfigError::FileRead { path: path.to_string(), error: e })?;
    
    let config: Config = toml::from_str(&content)
        .map_err(|e| ConfigError::ParseError { content, error: e })?;
    
    Ok(config)
}
```

### **3. Custom Error Types**

#### **Enum-Based Error Types**
```rust
#[derive(Debug)]
pub enum ProcessingError {
    InvalidInput { field: String, value: String },
    NetworkError { url: String, status: u16 },
    ParseError { line: usize, content: String },
    InternalError(String),
}

impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProcessingError::InvalidInput { field, value } => {
                write!(f, "Invalid input for field '{}': {}", field, value)
            }
            ProcessingError::NetworkError { url, status } => {
                write!(f, "Network error for URL '{}': status {}", url, status)
            }
            ProcessingError::ParseError { line, content } => {
                write!(f, "Parse error at line {}: {}", line, content)
            }
            ProcessingError::InternalError(msg) => {
                write!(f, "Internal error: {}", msg)
            }
        }
    }
}

impl std::error::Error for ProcessingError {}
```

### **4. Error Recovery Patterns**

#### **Default Value Pattern**
```rust
fn get_user_preference(key: &str) -> String {
    match read_config_file(key) {
        Ok(value) => value,
        Err(_) => "default_value".to_string(),  // Recover with default
    }
}
```

#### **Retry Pattern**
```rust
fn fetch_with_retry(url: &str, max_retries: usize) -> Result<String, NetworkError> {
    let mut last_error = None;
    
    for attempt in 0..=max_retries {
        match fetch_url(url) {
            Ok(response) => return Ok(response),
            Err(e) => {
                last_error = Some(e);
                if attempt < max_retries {
                    std::thread::sleep(std::time::Duration::from_millis(100 * (attempt + 1) as u64));
                }
            }
        }
    }
    
    Err(last_error.unwrap())
}
```

### **5. Error Context Patterns**

#### **Error Chaining with `anyhow`**
```rust
use anyhow::{Context, Result};

fn process_user_data(user_id: u32, data: &str) -> Result<ProcessedData> {
    let user = load_user(user_id)
        .with_context(|| format!("Failed to load user {}", user_id))?;
    
    let parsed = parse_data(data)
        .with_context(|| format!("Failed to parse data for user {}", user_id))?;
    
    Ok(ProcessedData::new(user, parsed))
}
```

#### **Error Context with `thiserror`**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to load user {user_id}: {source}")]
    UserLoadError { user_id: u32, source: UserError },
    
    #[error("Failed to parse data for user {user_id}: {source}")]
    DataParseError { user_id: u32, source: ParseError },
}

fn process_user_data(user_id: u32, data: &str) -> Result<ProcessedData, AppError> {
    let user = load_user(user_id)
        .map_err(|e| AppError::UserLoadError { user_id, source: e })?;
    
    let parsed = parse_data(data)
        .map_err(|e| AppError::DataParseError { user_id, source: e })?;
    
    Ok(ProcessedData::new(user, parsed))
}
```

## 🎯 Pattern Selection Guide

### **When to Use Each Pattern**

| Pattern | Use Case | Example |
|---------|----------|---------|
| **Early Return** | Simple error propagation | File processing pipeline |
| **Error Mapping** | Converting between error types | API to domain error conversion |
| **Default Value** | Non-critical failures | Configuration with fallbacks |
| **Retry** | Transient failures | Network requests, database connections |
| **Error Context** | Debugging and logging | Adding context to error messages |
| **Custom Error Types** | Complex error hierarchies | Multi-step validation processes |

### **Performance Considerations**

```rust
// ✅ Good: Error types implement Copy/Clone when possible
#[derive(Debug, Clone, Copy)]
pub enum SimpleError {
    InvalidInput,
    NotFound,
}

// ✅ Good: Use Box<dyn Error> for complex error hierarchies
pub type AppResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

// ✅ Good: Avoid string allocations in hot paths
#[derive(Debug)]
pub enum FastError {
    InvalidInput,  // No string allocation
    NetworkError(u16),  // Just status code
}
```

## 🧪 Testing Error Patterns

### **Testing Error Conditions**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_input_error() {
        let result = parse_data("invalid");
        assert!(result.is_err());
        
        match result.unwrap_err() {
            ProcessingError::InvalidInput { field, value } => {
                assert_eq!(field, "data");
                assert_eq!(value, "invalid");
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_error_recovery() {
        let result = get_user_preference("missing_key");
        assert_eq!(result, "default_value");
    }
}
```

## 🔗 Integration Patterns

### **With Collections**
```rust
fn process_items(items: Vec<&str>) -> Result<Vec<ProcessedItem>, ProcessingError> {
    items
        .iter()
        .map(|item| process_single_item(item))
        .collect()  // Collects Result<ProcessedItem, Error> into Result<Vec<ProcessedItem>, Error>
}
```

### **With Iterators**
```rust
fn find_valid_items(items: Vec<&str>) -> Vec<ProcessedItem> {
    items
        .iter()
        .filter_map(|item| process_single_item(item).ok())  // Filter out errors
        .collect()
}
```

### **With Option**
```rust
fn safe_division(a: i32, b: i32) -> Result<f64, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivideByZero)
    } else {
        Ok(a as f64 / b as f64)
    }
}
```

## 📚 Related Concepts

### **Daily Study Integration**
- **[[daily-study/Day05]]** - Basic error handling with Option and Result
- **[[daily-study/Day30]]** - Error propagation with the ? operator
- **[[daily-study/Day31]]** - anyhow and thiserror libraries
- **[[daily-study/Day32]]** - Result combinators and functional patterns
- **[[daily-study/Day33]]** - Panic recovery and unwrap patterns
- **[[daily-study/Day34]]** - Advanced error handling patterns
- **[[daily-study/Day35]]** - Robust parsing with error recovery

### **Mission Applications**
- **[[mission-1]]** - Stack operations with bounds checking
- **[[mission-4]]** - Interior mutability error patterns
- **[[mission-5]]** - HashMap operations with key validation
- **[[mission-6]]** - Grid bounds checking and coordinate validation

### **Rust Book Integration**
- **[[../rust_book/Ch9/README]]** - Complete error handling chapter
- **Chapter 9.1**: Unrecoverable Errors with panic!
- **Chapter 9.2**: Recoverable Errors with Result
- **Chapter 9.3**: To panic! or Not to panic!

## 🎯 Best Practices

### **DO:**
- Use `Result<T, E>` for recoverable errors
- Provide meaningful error messages with context
- Implement `Display` and `Error` traits for custom errors
- Use `?` operator for error propagation
- Consider using `anyhow` for application errors
- Consider using `thiserror` for library errors

### **DON'T:**
- Use `unwrap()` in production code without good reason
- Panic on user input errors
- Ignore errors with `let _ = result;`
- Use `String` errors in performance-critical code
- Chain too many `?` operators without context

## 🔍 Common Anti-Patterns

### **❌ Panic on User Input**
```rust
// Bad: Panics on invalid user input
let age: u32 = user_input.parse().unwrap();

// Good: Handle the error gracefully
let age: u32 = match user_input.parse() {
    Ok(age) => age,
    Err(_) => {
        println!("Invalid age, using default value 18");
        18
    }
};
```

### **❌ Silent Error Ignoring**
```rust
// Bad: Silently ignoring errors
let _ = write_to_file(data);

// Good: Handle the error appropriately
if let Err(e) = write_to_file(data) {
    eprintln!("Failed to write file: {}", e);
    return Err(e.into());
}
```

### **❌ Generic String Errors**
```rust
// Bad: Not very helpful
fn process_data(data: &str) -> Result<Data, String> {
    if data.is_empty() {
        return Err("Error".to_string());
    }
    // ...
}

// Good: Specific error types with context
fn process_data(data: &str) -> Result<Data, ProcessingError> {
    if data.is_empty() {
        return Err(ProcessingError::InvalidInput {
            field: "data".to_string(),
            value: "empty string".to_string(),
        });
    }
    // ...
}
```

---

*Tags: #error-handling #result #panic #patterns #best-practices #anyhow #thiserror #recovery #propagation*
*Links: [[zettel-index]] | [[rust-book-ch9-12-review]] | [[Error Handling Deep Dive]] | [[daily-study/Day05]] | [[daily-study/Day30]] | [[daily-study/Day31]] | [[daily-study/Day32]] | [[daily-study/Day33]] | [[daily-study/Day34]] | [[daily-study/Day35]] | [[rust-concepts-MOC]] | [[../rust_book/Ch9/README]]*
