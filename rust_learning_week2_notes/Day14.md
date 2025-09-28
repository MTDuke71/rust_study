# Day 14 · Error Handling Patterns (robust error management)

## Core Concepts

### Rust's Error Philosophy
- **No Exceptions**: Rust uses `Result<T, E>` instead of throwing exceptions
- **Explicit Errors**: Errors are part of function signatures - no hidden failures
- **Compile-Time Safety**: Must handle or propagate errors explicitly
- **Zero-Cost**: Error handling compiles to efficient machine code

### Result<T, E> Fundamentals
```rust
// Result is an enum with two variants
enum Result<T, E> {
    Ok(T),    // Success case containing value
    Err(E),   // Error case containing error information
}

// Functions return Result when they can fail
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

## Basic Error Handling

### Handling Results with match
```rust
fn handle_division() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => eprintln!("Error: {}", error),
    }
}

// More concise with if let
fn handle_division_concise() {
    if let Ok(result) = divide(10.0, 2.0) {
        println!("Result: {}", result);
    }
}
```

### The ? Operator (Error Propagation)
```rust
use std::fs::File;
use std::io::{self, Read};

// Without ? operator - verbose
fn read_file_verbose(filename: &str) -> Result<String, io::Error> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(error) => Err(error),
    }
}

// With ? operator - concise
fn read_file_concise(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Even more concise
fn read_file_super_concise(filename: &str) -> Result<String, io::Error> {
    std::fs::read_to_string(filename)
}
```

### Unwrap Methods (Use Carefully!)
```rust
// unwrap() - panics on error (use only when you're sure it won't fail)
let result = divide(10.0, 2.0).unwrap(); // Panics if Err

// expect() - panics with custom message
let result = divide(10.0, 2.0).expect("Division should never fail here");

// unwrap_or() - provides default value
let result = divide(10.0, 0.0).unwrap_or(0.0);

// unwrap_or_else() - provides default via closure
let result = divide(10.0, 0.0).unwrap_or_else(|err| {
    eprintln!("Error occurred: {}", err);
    0.0
});
```

## Custom Error Types

### Simple Custom Errors
```rust
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::NegativeSquareRoot => write!(f, "Cannot take square root of negative number"),
            MathError::Overflow => write!(f, "Mathematical overflow occurred"),
        }
    }
}

impl std::error::Error for MathError {}

fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}
```

### Structured Error Types
```rust
#[derive(Debug)]
struct ParseError {
    line: usize,
    column: usize,
    message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parse error at line {}, column {}: {}", 
               self.line, self.column, self.message)
    }
}

impl std::error::Error for ParseError {}

impl ParseError {
    fn new(line: usize, column: usize, message: impl Into<String>) -> Self {
        ParseError {
            line,
            column,
            message: message.into(),
        }
    }
}
```

## Error Conversion and Chaining

### Using From Trait for Error Conversion
```rust
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(ParseError),
    Math(MathError),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "IO error: {}", err),
            AppError::Parse(err) => write!(f, "Parse error: {}", err),
            AppError::Math(err) => write!(f, "Math error: {}", err),
        }
    }
}

impl std::error::Error for AppError {}

// Automatic conversion from io::Error
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

impl From<ParseError> for AppError {
    fn from(error: ParseError) -> Self {
        AppError::Parse(error)
    }
}

impl From<MathError> for AppError {
    fn from(error: MathError) -> Self {
        AppError::Math(error)
    }
}

// Now ? operator works seamlessly with different error types
fn process_file(filename: &str) -> Result<f64, AppError> {
    let contents = std::fs::read_to_string(filename)?; // io::Error -> AppError
    let number = contents.trim().parse::<f64>()
        .map_err(|_| ParseError::new(1, 0, "Invalid number format"))?; // ParseError -> AppError
    let result = safe_sqrt(number)?; // MathError -> AppError
    Ok(result)
}
```

### Error Chaining with source()
```rust
use std::error::Error;

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            AppError::Parse(err) => Some(err),
            AppError::Math(err) => Some(err),
        }
    }
}

// Print error chain
fn print_error_chain(mut error: &dyn Error) {
    eprintln!("Error: {}", error);
    while let Some(source) = error.source() {
        eprintln!("  Caused by: {}", source);
        error = source;
    }
}
```

## Advanced Error Handling Patterns

### Result Combinators
```rust
// map - transform success value
let result = divide(10.0, 2.0)
    .map(|x| x * 2.0); // Result<f64, String>

// map_err - transform error value
let result = divide(10.0, 0.0)
    .map_err(|_| "Mathematical error occurred".to_string());

// and_then - chain operations that can fail
let result = divide(10.0, 2.0)
    .and_then(|x| safe_sqrt(x))
    .and_then(|x| safe_divide(x, 2.0));

// or_else - provide alternative on error
let result = divide(10.0, 0.0)
    .or_else(|_| divide(5.0, 1.0));
```

### Working with Multiple Results
```rust
// Collect Results - fails if any operation fails
let numbers = vec!["1", "2", "3", "not_a_number"];
let parsed: Result<Vec<i32>, _> = numbers
    .iter()
    .map(|s| s.parse::<i32>())
    .collect();

// Partition into successes and failures
let results: Vec<Result<i32, _>> = numbers
    .iter()
    .map(|s| s.parse::<i32>())
    .collect();

let (successes, failures): (Vec<_>, Vec<_>) = results
    .into_iter()
    .partition(Result::is_ok);

let successes: Vec<i32> = successes.into_iter().map(Result::unwrap).collect();
let failures: Vec<_> = failures.into_iter().map(Result::unwrap_err).collect();
```

### Early Return Pattern
```rust
fn complex_operation(input: &str) -> Result<String, AppError> {
    // Early validation
    if input.is_empty() {
        return Err(AppError::Parse(ParseError::new(0, 0, "Empty input")));
    }
    
    // Chain operations with ?
    let processed = input.trim();
    let number = processed.parse::<f64>()
        .map_err(|_| ParseError::new(1, 0, "Invalid number"))?;
    
    let sqrt_result = safe_sqrt(number)?;
    let final_result = safe_divide(sqrt_result, 2.0)?;
    
    Ok(format!("Result: {:.2}", final_result))
}
```

## Option<T> Error Handling

### Option as Simple Error Type
```rust
fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some("Alice".to_string()),
        2 => Some("Bob".to_string()),
        _ => None,
    }
}

// Convert Option to Result
fn find_user_strict(id: u32) -> Result<String, String> {
    find_user(id).ok_or_else(|| format!("User {} not found", id))
}

// Option combinators
let user = find_user(1)
    .map(|name| name.to_uppercase())
    .filter(|name| name.len() > 3)
    .unwrap_or_else(|| "Unknown".to_string());
```

### Working with Option and Result Together
```rust
fn parse_and_find(input: &str) -> Result<String, AppError> {
    let id = input.parse::<u32>()
        .map_err(|_| ParseError::new(1, 0, "Invalid ID format"))?;
    
    let user = find_user(id)
        .ok_or_else(|| ParseError::new(1, 0, format!("User {} not found", id)))?;
    
    Ok(user)
}
```

## Error Handling in Different Contexts

### Library vs Application Error Handling
```rust
// Library code - specific error types
pub enum DatabaseError {
    ConnectionFailed(String),
    QueryFailed(String),
    RecordNotFound(u64),
}

pub fn get_user(id: u64) -> Result<User, DatabaseError> {
    // Implementation details...
    if id == 0 {
        Err(DatabaseError::RecordNotFound(id))
    } else {
        Ok(User { id, name: "Test".to_string() })
    }
}

// Application code - convert to app-specific errors
#[derive(Debug)]
pub enum AppError {
    Database(DatabaseError),
    Config(String),
    Network(String),
}

impl From<DatabaseError> for AppError {
    fn from(err: DatabaseError) -> Self {
        AppError::Database(err)
    }
}
```

### AoC Error Handling Patterns
```rust
use std::str::FromStr;

#[derive(Debug)]
pub enum AocError {
    ParseError(String),
    InvalidInput(String),
    SolutionNotFound,
}

impl std::fmt::Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AocError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AocError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AocError::SolutionNotFound => write!(f, "No solution found"),
        }
    }
}

impl std::error::Error for AocError {}

// Generic parsing helper
fn parse_lines<T>(input: &str) -> Result<Vec<T>, AocError>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.parse::<T>().map_err(|e| {
                AocError::ParseError(format!("Line {}: {:?}", i + 1, e))
            })
        })
        .collect()
}

// Coordinate parsing
fn parse_coordinate(s: &str) -> Result<(i32, i32), AocError> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err(AocError::InvalidInput(
            format!("Expected 'x,y' format, got '{}'", s)
        ));
    }
    
    let x = parts[0].trim().parse::<i32>()
        .map_err(|_| AocError::ParseError(format!("Invalid x coordinate: '{}'", parts[0])))?;
    let y = parts[1].trim().parse::<i32>()
        .map_err(|_| AocError::ParseError(format!("Invalid y coordinate: '{}'", parts[1])))?;
    
    Ok((x, y))
}
```

## Third-Party Error Handling Crates

### anyhow - For Applications
```rust
use anyhow::{Context, Result};

fn read_config(filename: &str) -> Result<Config> {
    let contents = std::fs::read_to_string(filename)
        .with_context(|| format!("Failed to read config file '{}'", filename))?;
    
    let config: Config = serde_json::from_str(&contents)
        .with_context(|| "Failed to parse config as JSON")?;
    
    Ok(config)
}

// Usage
fn main() -> Result<()> {
    let config = read_config("config.json")?;
    // ... use config
    Ok(())
}
```

### thiserror - For Libraries
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    
    #[error("JSON parse error")]
    Json(#[from] serde_json::Error),
    
    #[error("Invalid configuration: {message}")]
    Invalid { message: String },
    
    #[error("Missing required field: {field}")]
    MissingField { field: String },
}
```

## Testing Error Conditions

### Testing Error Cases
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_division_by_zero() {
        let result = safe_divide(10.0, 0.0);
        assert!(result.is_err());
        
        if let Err(MathError::DivisionByZero) = result {
            // Expected error type
        } else {
            panic!("Expected DivisionByZero error");
        }
    }
    
    #[test]
    fn test_error_message() {
        let result = safe_divide(10.0, 0.0);
        assert_eq!(result.unwrap_err().to_string(), "Cannot divide by zero");
    }
    
    #[test]
    fn test_error_chain() {
        let result = process_file("nonexistent.txt");
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(matches!(error, AppError::Io(_)));
        assert!(error.source().is_some());
    }
}
```

## Best Practices

### When to Use Each Error Handling Approach

```rust
// ✅ Use Result<T, E> when:
// - Operation can fail in expected ways
// - Caller should handle the error
// - Error information is important

// ✅ Use Option<T> when:
// - Absence of value is not an error
// - Simple present/absent semantics

// ✅ Use panic! when:
// - Unrecoverable errors (programming bugs)
// - Contract violations
// - Development-time debugging

// ✅ Use ? operator when:
// - Propagating errors up the call stack
// - Error types are compatible (via From trait)

// ✅ Use unwrap() when:
// - You're absolutely certain operation won't fail
// - In tests or examples
// - Quick prototyping (but never in production)
```

### Error Design Guidelines
1. **Be Specific**: Create meaningful error types, not just `String`
2. **Be Consistent**: Use consistent error types within your API
3. **Be Helpful**: Include context and actionable information
4. **Be Composable**: Implement `From` for error conversion
5. **Be Testable**: Make errors easy to test and match against

## Learning Progression Summary

From Day 14, you should understand:
1. **Result<T, E>**: Rust's primary error handling mechanism
2. **Error Propagation**: Using `?` operator for clean error handling
3. **Custom Errors**: Creating domain-specific error types
4. **Error Conversion**: Using `From` trait for seamless error transformation
5. **Combinators**: Functional error handling with `map`, `and_then`, etc.
6. **Best Practices**: When to use Result vs Option vs panic
7. **Testing**: How to test error conditions effectively
8. **Real-World Patterns**: AoC parsing, file processing, data validation

**Week 2 Complete!** You now have a solid foundation in Rust's collections, iterators, and error handling - essential tools for any Rust developer!

## 🚀 **Complete Runnable Example**

```rust
// Copy this entire block to Rust Playground or save as a .rs file
use std::collections::HashMap;
use std::fmt;

fn main() {
    println!("=== Error Handling Patterns Demo from Day 14 ===\n");
    
    // 1. Basic Result usage
    println!("1. Basic Result Usage:");
    match divide(10.0, 2.0) {
        Ok(result) => println!("   10.0 / 2.0 = {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    
    // 2. Error propagation with ? operator
    println!("\n2. Error Propagation with ? Operator:");
    match process_numbers() {
        Ok(results) => println!("   Results: {:?}", results),
        Err(e) => println!("   Error: {}", e),
    }
    
    // 3. Custom error types
    println!("\n3. Custom Error Types:");
    match parse_user_data("alice,25,engineer") {
        Ok(user) => println!("   User: {} ({}, {})", user.name, user.age, user.role),
        Err(e) => println!("   Parse error: {}", e),
    }
    
    match parse_user_data("bob,invalid,manager") {
        Ok(user) => println!("   User: {} ({}, {})", user.name, user.age, user.role),
        Err(e) => println!("   Parse error: {}", e),
    }
    
    // 4. Error conversion and chaining
    println!("\n4. Error Conversion and From Trait:");
    match load_and_parse_config() {
        Ok(config) => println!("   Config loaded: {:?}", config),
        Err(e) => println!("   Config error: {}", e),
    }
    
    // 5. Result combinators
    println!("\n5. Result Combinators:");
    let numbers = vec!["42", "13", "invalid", "99"];
    let parsed_sum: Result<i32, _> = numbers
        .iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .map(|nums| nums.iter().sum());
    
    match parsed_sum {
        Ok(sum) => println!("   Sum of valid numbers: {}", sum),
        Err(e) => println!("   Parsing error: {}", e),
    }
    
    // 6. Practical AoC-style error handling
    println!("\n6. AoC-Style Data Processing:");
    let input_lines = vec![
        "move 3 from 1 to 2",
        "move 1 from 2 to 3",
        "invalid instruction",
        "move 2 from 3 to 1",
    ];
    
    let (valid_moves, errors): (Vec<_>, Vec<_>) = input_lines
        .iter()
        .map(|line| parse_move_instruction(line))
        .partition(Result::is_ok);
    
    let moves: Vec<_> = valid_moves.into_iter().map(Result::unwrap).collect();
    let parse_errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    
    println!("   Valid moves: {:?}", moves);
    println!("   Parse errors: {:?}", parse_errors);
    
    // 7. Option and Result interoperability
    println!("\n7. Option ↔ Result Conversion:");
    let maybe_number = Some(42);
    let result_number: Result<i32, &str> = maybe_number.ok_or("No number found");
    println!("   Option to Result: {:?}", result_number);
    
    let result_value: Result<String, &str> = Ok("success".to_string());
    let option_value: Option<String> = result_value.ok();
    println!("   Result to Option: {:?}", option_value);
}

// Helper functions for the examples

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn process_numbers() -> Result<Vec<f64>, String> {
    let a = divide(20.0, 4.0)?;  // ? propagates error if Err
    let b = divide(15.0, 3.0)?;
    let c = divide(8.0, 2.0)?;
    Ok(vec![a, b, c])
}

// Custom error type for user parsing
#[derive(Debug)]
enum UserParseError {
    InvalidFormat,
    InvalidAge(std::num::ParseIntError),
    EmptyField,
}

impl fmt::Display for UserParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserParseError::InvalidFormat => write!(f, "Invalid format: expected 'name,age,role'"),
            UserParseError::InvalidAge(e) => write!(f, "Invalid age: {}", e),
            UserParseError::EmptyField => write!(f, "Empty field found"),
        }
    }
}

impl From<std::num::ParseIntError> for UserParseError {
    fn from(error: std::num::ParseIntError) -> Self {
        UserParseError::InvalidAge(error)
    }
}

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    role: String,
}

fn parse_user_data(data: &str) -> Result<User, UserParseError> {
    let parts: Vec<&str> = data.split(',').collect();
    
    if parts.len() != 3 {
        return Err(UserParseError::InvalidFormat);
    }
    
    let name = parts[0];
    if name.is_empty() {
        return Err(UserParseError::EmptyField);
    }
    
    let age: u32 = parts[1].parse()?; // Uses From trait conversion
    
    let role = parts[2];
    if role.is_empty() {
        return Err(UserParseError::EmptyField);
    }
    
    Ok(User {
        name: name.to_string(),
        age,
        role: role.to_string(),
    })
}

fn load_and_parse_config() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    // Simulated config loading that could have different error types
    let config_data = "key1=value1\nkey2=value2\ninvalid_line";
    
    let mut config = HashMap::new();
    for line in config_data.lines() {
        if line.contains('=') {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                config.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
    }
    
    if config.is_empty() {
        Err("No valid config entries found".into())
    } else {
        Ok(config)
    }
}

#[derive(Debug, PartialEq)]
struct Move {
    count: u32,
    from: u32,
    to: u32,
}

fn parse_move_instruction(line: &str) -> Result<Move, String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    if parts.len() != 6 || parts[0] != "move" || parts[2] != "from" || parts[4] != "to" {
        return Err(format!("Invalid move format: {}", line));
    }
    
    let count = parts[1].parse().map_err(|_| format!("Invalid count: {}", parts[1]))?;
    let from = parts[3].parse().map_err(|_| format!("Invalid from: {}", parts[3]))?;
    let to = parts[5].parse().map_err(|_| format!("Invalid to: {}", parts[5]))?;
    
    Ok(Move { count, from, to })
}
```

### **🛠️ How to Run This Code:**

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day14_demo.rs` and run `rustc day14_demo.rs && ./day14_demo`
3. **In this workspace**: `.\run_md.bat rust_learning_week2_notes\Day14.md`
4. **As Cargo example**: `cargo run --example day14_error_handling_demo` (if you add it to Mission5_tut)