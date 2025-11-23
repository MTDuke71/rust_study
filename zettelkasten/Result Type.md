# Result Type - Rust's Error Handling Foundation

**`Result<T, E>` is Rust's type-safe approach to error handling**, replacing exceptions and null checks with explicit, composable error management.

## 🎯 The Core Problem: Error Handling

Traditional error handling has fundamental flaws:

```c
// C - Error codes (easy to ignore)
int result = fopen("file.txt", "r");
if (result == NULL) {
    // Error handling is optional - easy to forget!
    return -1;
}

// Java - Exceptions (can be thrown anywhere)
try {
    String content = readFile("file.txt");
    process(content);
} catch (IOException e) {
    // Exception can be thrown from anywhere in call stack
    // No indication in function signature!
}
```

**The Problems**: 
- Error conditions can be ignored
- Function signatures don't indicate error possibilities
- Exceptions can be thrown from anywhere

## 🦀 Rust's Solution: Result<T, E>

```rust
enum Result<T, E> {
    Ok(T),    // Success case - contains the value
    Err(E),   // Error case - contains error information
}
```

This is a **built-in enum** that makes error conditions explicit in the type system.

## 📊 Result vs Traditional Error Handling

| Aspect | C Error Codes | Java Exceptions | Rust Result<T, E> |
|--------|---------------|-----------------|-------------------|
| **Type Safety** | ❌ Easy to ignore | ❌ Not in signature | ✅ Compiler forces handling |
| **Explicit** | ❌ Hidden in return | ❌ Hidden in throws | ✅ Visible in type signature |
| **Composable** | ❌ Manual propagation | ❌ Try-catch blocks | ✅ `?` operator, combinators |
| **Documentation** | ❌ Not enforced | ❌ Optional comments | ✅ Self-documenting |

## 💡 Basic Usage

### **Creating Result Values**

```rust
// Success cases
let success: Result<i32, String> = Ok(42);
let file_result: Result<String, std::io::Error> = std::fs::read_to_string("file.txt");

// Error cases
let error: Result<i32, String> = Err("Something went wrong".to_string());
let parse_error: Result<i32, std::num::ParseIntError> = "not_a_number".parse();
```

### **Pattern Matching**

```rust
fn handle_result(result: Result<i32, String>) {
    match result {
        Ok(value) => println!("Success: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}

// Usage
handle_result(Ok(42));
handle_result(Err("Failed".to_string()));
```

### **Using `unwrap()` and `expect()`**

```rust
// ⚠️ Use with caution - panics on error
let value = Ok(42).unwrap();  // Returns 42
let value = Ok(42).expect("Should be 42");  // Returns 42 with custom message

// These will panic:
// Err("error").unwrap();  // 💥 Panic!
// Err("error").expect("Custom message");  // 💥 Panic with custom message!
```

## 🔧 Common Result Operations

### **The `?` Operator - Error Propagation**

```rust
use std::fs;
use std::io;

fn read_and_process_file(filename: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(filename)?;  // ? propagates error up
    let processed = process_content(&content)?;   // ? propagates error up
    Ok(processed)
}

fn process_content(content: &str) -> Result<String, io::Error> {
    if content.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Empty content"));
    }
    Ok(content.to_uppercase())
}
```

**The `?` operator**:
- Returns early with error if `Result` is `Err`
- Unwraps value if `Result` is `Ok`
- Only works in functions that return `Result`

### **Error Mapping and Transformation**

```rust
// map() - Transform success value
let result: Result<i32, String> = Ok(42);
let doubled = result.map(|x| x * 2);  // Ok(84)

// map_err() - Transform error value
let result: Result<i32, String> = Err("Failed".to_string());
let mapped = result.map_err(|e| format!("Error: {}", e));  // Err("Error: Failed")

// and_then() - Chain operations that return Results
let result: Result<i32, String> = Ok(42);
let chained = result.and_then(|x| {
    if x > 0 {
        Ok(x * 2)
    } else {
        Err("Negative number".to_string())
    }
});
```

### **Combinator Methods**

```rust
// unwrap_or() - Provide default value on error
let result: Result<i32, String> = Err("Failed".to_string());
let value = result.unwrap_or(0);  // Returns 0

// unwrap_or_else() - Provide default via closure
let result: Result<i32, String> = Err("Failed".to_string());
let value = result.unwrap_or_else(|_| {
    println!("Using default value");
    42
});

// unwrap_or_default() - Use Default trait
let result: Result<String, String> = Err("Failed".to_string());
let value = result.unwrap_or_default();  // Returns ""

// or() - Provide alternative Result
let result: Result<i32, String> = Err("Failed".to_string());
let alternative = result.or(Ok(42));  // Ok(42)

// or_else() - Provide alternative via closure
let result: Result<i32, String> = Err("Failed".to_string());
let alternative = result.or_else(|_| Ok(42));  // Ok(42)
```

## 🎯 Practical Examples

### **File I/O with Result**

```rust
use std::fs;
use std::io;

fn read_config_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn parse_config(content: &str) -> Result<Config, String> {
    if content.is_empty() {
        return Err("Empty config file".to_string());
    }
    
    // Parse logic here...
    Ok(Config::default())
}

fn load_config(path: &str) -> Result<Config, String> {
    let content = read_config_file(path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;
    
    let config = parse_config(&content)?;
    Ok(config)
}

// Usage with error handling
match load_config("config.txt") {
    Ok(config) => println!("Config loaded: {:?}", config),
    Err(error) => eprintln!("Failed to load config: {}", error),
}
```

### **Parsing with Result**

```rust
fn parse_number(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse::<i32>()
}

fn parse_positive_number(input: &str) -> Result<i32, String> {
    let number = input.parse::<i32>()
        .map_err(|e| format!("Invalid number '{}': {}", input, e))?;
    
    if number > 0 {
        Ok(number)
    } else {
        Err(format!("Number must be positive, got: {}", number))
    }
}

// Usage
let result1 = parse_number("42");        // Ok(42)
let result2 = parse_number("abc");       // Err(ParseIntError)
let result3 = parse_positive_number("-5"); // Err("Number must be positive, got: -5")
```

### **Network Operations with Result**

```rust
use std::net::TcpStream;
use std::io::{Read, Write};

fn send_data(host: &str, port: u16, data: &[u8]) -> Result<Vec<u8>, String> {
    let mut stream = TcpStream::connect(format!("{}:{}", host, port))
        .map_err(|e| format!("Failed to connect to {}:{} - {}", host, port, e))?;
    
    stream.write_all(data)
        .map_err(|e| format!("Failed to send data: {}", e))?;
    
    let mut response = Vec::new();
    stream.read_to_end(&mut response)
        .map_err(|e| format!("Failed to read response: {}", e))?;
    
    Ok(response)
}

// Usage
match send_data("example.com", 80, b"GET / HTTP/1.1\r\n\r\n") {
    Ok(response) => println!("Received {} bytes", response.len()),
    Err(error) => eprintln!("Network error: {}", error),
}
```

## 🔄 Converting Between Option and Result

### **Option to Result**

```rust
// Convert Option<T> to Result<T, E>
let some_value = Some(42);
let result = some_value.ok_or("No value present".to_string());  // Ok(42)

let none_value: Option<i32> = None;
let result = none_value.ok_or("No value present".to_string());  // Err("No value present")

// With custom error type
let result = some_value.ok_or_else(|| std::io::Error::new(
    std::io::ErrorKind::NotFound, 
    "Value not found"
));
```

### **Result to Option**

```rust
// Convert Result<T, E> to Option<T>
let ok_result = Ok(42);
let option = ok_result.ok();  // Some(42)

let err_result: Result<i32, String> = Err("Failed".to_string());
let option = err_result.ok();  // None

// Convert Result<T, E> to Option<E>
let err_result: Result<i32, String> = Err("Failed".to_string());
let option = err_result.err();  // Some("Failed")
```

## 🏗️ Custom Error Types

### **Simple Error Types**

```rust
#[derive(Debug)]
pub enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Division by zero"),
            MathError::NegativeSquareRoot => write!(f, "Cannot take square root of negative number"),
            MathError::Overflow => write!(f, "Arithmetic overflow"),
        }
    }
}

impl std::error::Error for MathError {}

// Usage
fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}
```

### **Complex Error Types with Context**

```rust
#[derive(Debug)]
pub enum ProcessingError {
    FileNotFound { path: String },
    ParseError { line: usize, content: String, error: String },
    NetworkError { url: String, status_code: u16 },
    ValidationError { field: String, message: String },
}

impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProcessingError::FileNotFound { path } => {
                write!(f, "File not found: {}", path)
            }
            ProcessingError::ParseError { line, content, error } => {
                write!(f, "Parse error at line {}: {} - {}", line, content, error)
            }
            ProcessingError::NetworkError { url, status_code } => {
                write!(f, "Network error for {}: status {}", url, status_code)
            }
            ProcessingError::ValidationError { field, message } => {
                write!(f, "Validation error for field '{}': {}", field, message)
            }
        }
    }
}

impl std::error::Error for ProcessingError {}
```

## 🔗 Error Propagation Patterns

### **Using `?` for Clean Error Propagation**

```rust
fn process_user_data(user_input: &str) -> Result<ProcessedData, ProcessingError> {
    let parsed = parse_input(user_input)?;  // Propagates ParseError
    let validated = validate_data(&parsed)?;  // Propagates ValidationError
    let enriched = enrich_data(validated)?;  // Propagates NetworkError
    Ok(enriched)
}

fn parse_input(input: &str) -> Result<ParsedData, ProcessingError> {
    // Parsing logic that can return ParseError
    if input.is_empty() {
        return Err(ProcessingError::ParseError {
            line: 1,
            content: input.to_string(),
            error: "Empty input".to_string(),
        });
    }
    Ok(ParsedData::from(input))
}
```

### **Error Context with `map_err()`**

```rust
use std::fs;

fn read_and_process_file(filename: &str) -> Result<String, String> {
    let content = fs::read_to_string(filename)
        .map_err(|e| format!("Failed to read file '{}': {}", filename, e))?;
    
    let processed = process_content(&content)
        .map_err(|e| format!("Failed to process file '{}': {}", filename, e))?;
    
    Ok(processed)
}
```

## 🎯 Best Practices

### **DO:**
- **Use `Result<T, E>` for recoverable errors**
- **Make error types descriptive and actionable**
- **Use `?` operator for clean error propagation**
- **Implement `Display` and `Error` traits for custom errors**
- **Use `map_err()` to add context to errors**

### **DON'T:**
- **Don't use `unwrap()` in production code without good reason**
- **Don't ignore errors - handle them explicitly**
- **Don't use overly generic error types**
- **Don't panic for recoverable conditions**

### **When to Use Result vs Option:**

```rust
// Use Option<T> when absence is normal
fn find_user_by_id(id: u32) -> Option<User> {
    // User might not exist - this is normal
}

// Use Result<T, E> when absence indicates an error
fn load_user_by_id(id: u32) -> Result<User, DatabaseError> {
    // Database error indicates a problem that should be handled
}
```

## 🔗 Related Concepts

### **Error Handling Integration**
- **[[Error Handling Patterns]]** - Comprehensive error handling strategies
- **[[Option Type]]** - Handling absence vs errors
- **[[anyhow and thiserror]]** - Advanced error handling crates
- **[[Error Propagation]]** - Patterns for error flow

### **Mission Applications**
- **[[mission-2]]** - Queue operations with Result error handling
- **[[mission-4]]** - Linked list operations with borrow errors
- **[[mission-5]]** - HashMap operations with key errors
- **[[mission-6]]** - Grid operations with bounds errors

### **Daily Study Integration**
- **[[daily-study/Day05]]** - Option and Result introduction
- **[[daily-study/Day29]]** - Error handling patterns
- **[[daily-study/Day30]]** - Custom error types
- **[[Week 5 Overview]]** - Advanced error handling

## 📊 Performance Considerations

### **Result vs Exception Performance**

| Aspect | Exceptions (C++) | Result<T, E> (Rust) |
|--------|------------------|---------------------|
| **Success Path** | Zero overhead | Zero overhead |
| **Error Path** | Expensive stack unwinding | Simple enum variant |
| **Memory** | Dynamic allocation | Stack allocation |
| **Predictability** | Unpredictable | Predictable |

### **Zero-Cost Abstractions**

```rust
// This Result has zero runtime overhead
fn might_fail() -> Result<i32, String> {
    Ok(42)  // Just a tagged union in memory
}

// The compiler optimizes away the Result wrapper in many cases
let value = might_fail().unwrap();  // Compiles to direct value access
```

## 🧪 Testing Result Types

### **Testing Success Cases**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_operation() {
        let result = divide(10, 2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5);
    }

    #[test]
    fn test_result_matching() {
        let result = parse_number("42");
        match result {
            Ok(value) => assert_eq!(value, 42),
            Err(_) => panic!("Expected Ok(42)"),
        }
    }
}
```

### **Testing Error Cases**

```rust
#[test]
fn test_division_by_zero() {
    let result = divide(10, 0);
    assert!(result.is_err());
    
    match result {
        Err(MathError::DivisionByZero) => {}, // Expected
        _ => panic!("Expected DivisionByZero error"),
    }
}

#[test]
fn test_error_message() {
    let result = parse_positive_number("-5");
    assert!(result.is_err());
    
    let error_message = result.unwrap_err();
    assert!(error_message.contains("positive"));
}
```

### **Property-Based Testing**

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_division_properties(a in -1000i32..1000, b in -1000i32..1000) {
        let result = divide(a, b);
        
        if b == 0 {
            assert!(result.is_err());
        } else {
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), a / b);
        }
    }
}
```

---

*Tags: #result-type #error-handling #type-safety #rust-enum #composable-errors*
*Links: [[zettel-index]] | [[rust-book-ch9-12-review]] | [[Error Handling Patterns]] | [[Option Type]] | [[anyhow and thiserror]] | [[Error Propagation]] | [[daily-study/Day05]] | [[daily-study/Day29]] | [[daily-study/Day30]] | [[Week 5 Overview]] | [[mission-2]] | [[mission-4]] | [[mission-5]] | [[mission-6]]*
