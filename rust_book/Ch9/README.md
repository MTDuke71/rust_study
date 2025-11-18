# 📖 Chapter 9 - Error Handling

Rust's approach to error handling: reliability, safety, and explicit error management.

---

## 🎯 Chapter Overview

**Core Concepts:**
- **Unrecoverable errors**: Use `panic!` for bugs and impossible states
- **Recoverable errors**: Use `Result<T, E>` for expected failures
- **Error propagation**: The `?` operator for clean error handling
- **Custom error types**: Creating domain-specific error handling
- **Best practices**: When to panic vs when to return errors

**Philosophy:** Rust forces you to acknowledge errors exist and handle them explicitly, making your programs more reliable.

---

## 📚 Quick Navigation

| Section | Directory | Focus |
|---------|-----------|--------|
| **9.1** | `unrecoverable_errors/` | `panic!`, stack unwinding, backtraces |
| **9.2** | `recoverable_errors/` | `Result<T,E>`, `match`, error handling patterns |
| **9.3** | `error_propagation/` | `?` operator, error chaining |
| **Exercises** | `exercises/` | Comprehensive practice problems |

---

## 🚀 **Complete Runnable Examples**

### 1. **Unrecoverable Errors with `panic!`**

When your program encounters an unrecoverable error, use `panic!`:

```rust
fn main() {
    println!("=== Panic! Examples ===\n");
    
    // 1. Explicit panic
    println!("1. Explicit panic with message:");
    // panic!("This is a custom panic message!");
    println!("   (Commented out - would terminate program)\n");
    
    // 2. Index out of bounds panic
    println!("2. Index out of bounds:");
    let v = vec![1, 2, 3];
    // let element = v[99];  // This would panic!
    println!("   Vector: {:?}", v);
    println!("   Accessing v[99] would panic!\n");
    
    // 3. Safe alternative with get()
    println!("3. Safe access with get():");
    match v.get(99) {
        Some(value) => println!("   Found: {}", value),
        None => println!("   Index 99 not found - no panic!"),
    }
    println!();
    
    // 4. Panic with backtrace (set RUST_BACKTRACE=1)
    println!("4. Backtrace information:");
    println!("   Set RUST_BACKTRACE=1 to see call stack on panic");
    println!("   Set RUST_BACKTRACE=full for detailed backtrace\n");
    
    // 5. When to use panic!
    println!("5. When to panic:");
    println!("   ✅ Contract violations (invalid state)");
    println!("   ✅ Programming bugs (logic errors)");
    println!("   ✅ Prototype/example code");
    println!("   ❌ Expected failures (file not found)");
    println!("   ❌ User input validation");
}
```

### 2. **Recoverable Errors with `Result<T, E>`**

Most errors should be recoverable and handled gracefully:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("=== Result<T, E> Examples ===\n");
    
    // 1. Basic Result handling
    println!("1. Basic Result with match:");
    let filename = "hello.txt";
    let f = File::open(filename);
    
    match f {
        Ok(file) => println!("   Successfully opened: {}", filename),
        Err(error) => println!("   Failed to open {}: {:?}", filename, error),
    }
    println!();
    
    // 2. Matching on specific error kinds
    println!("2. Specific error handling:");
    let f = File::open("nonexistent.txt");
    
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("   File not found, creating new one...");
                match File::create("nonexistent.txt") {
                    Ok(fc) => {
                        println!("   Created successfully!");
                        fc
                    }
                    Err(e) => {
                        println!("   Failed to create: {:?}", e);
                        panic!("Problem creating the file: {:?}", e);
                    }
                }
            }
            other_error => {
                println!("   Other error: {:?}", other_error);
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };
    println!();
    
    // 3. Shortcuts: unwrap() and expect()
    println!("3. Shortcuts (use carefully!):");
    // let f = File::open("hello.txt").unwrap();  // Panics on error
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");
    println!("   .unwrap() - panics with default message");
    println!("   .expect(msg) - panics with custom message");
    println!("   Use only when you're certain no error will occur\n");
    
    // 4. Result in functions
    println!("4. Functions returning Result:");
    match divide(10.0, 2.0) {
        Ok(result) => println!("   10.0 ÷ 2.0 = {}", result),
        Err(msg) => println!("   Error: {}", msg),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("   10.0 ÷ 0.0 = {}", result),
        Err(msg) => println!("   Error: {}", msg),
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}
```

### 3. **Error Propagation with `?`**

The `?` operator makes error propagation clean and idiomatic:

```rust
use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("=== Error Propagation Examples ===\n");
    
    // 1. Manual error propagation (verbose)
    println!("1. Manual propagation:");
    match read_username_from_file_manual() {
        Ok(username) => println!("   Username: {}", username),
        Err(e) => println!("   Error reading username: {}", e),
    }
    println!();
    
    // 2. With ? operator (concise)
    println!("2. With ? operator:");
    match read_username_from_file() {
        Ok(username) => println!("   Username: {}", username),
        Err(e) => println!("   Error reading username: {}", e),
    }
    println!();
    
    // 3. Chaining with ?
    println!("3. Chaining operations:");
    match process_config_file() {
        Ok(config) => println!("   Config processed: {} chars", config.len()),
        Err(e) => println!("   Config error: {}", e),
    }
    println!();
    
    // 4. ? with Option
    println!("4. ? with Option:");
    let numbers = vec![1, 2, 3, 4, 5];
    match find_even_number(&numbers) {
        Some(n) => println!("   First even number: {}", n),
        None => println!("   No even numbers found"),
    }
}

// Manual error propagation (don't do this)
fn read_username_from_file_manual() -> Result<String, io::Error> {
    let f = File::open("username.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s.trim().to_string()),
        Err(e) => Err(e),
    }
}

// With ? operator (idiomatic)
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s.trim().to_string())
}

// Even more concise
fn process_config_file() -> Result<String, io::Error> {
    std::fs::read_to_string("config.txt")
}

// ? with Option
fn find_even_number(numbers: &[i32]) -> Option<i32> {
    let first = numbers.get(0)?;  // Returns None if empty
    if first % 2 == 0 {
        Some(*first)
    } else {
        numbers.iter().find(|&&n| n % 2 == 0).copied()
    }
}
```

### 4. **Custom Error Types**

Creating your own error types for better error handling:

```rust
use std::fmt;
use std::error::Error;
use std::num::ParseIntError;

fn main() {
    println!("=== Custom Error Types ===\n");
    
    // 1. Simple custom error
    println!("1. Simple custom error:");
    match validate_age("25") {
        Ok(age) => println!("   Valid age: {}", age),
        Err(e) => println!("   Error: {}", e),
    }
    
    match validate_age("150") {
        Ok(age) => println!("   Valid age: {}", age),
        Err(e) => println!("   Error: {}", e),
    }
    println!();
    
    // 2. Enum-based errors
    println!("2. Enum-based errors:");
    let test_cases = vec!["25", "abc", "-5", "150"];
    
    for case in test_cases {
        match parse_and_validate_age(case) {
            Ok(age) => println!("   '{}' -> Valid age: {}", case, age),
            Err(e) => println!("   '{}' -> Error: {} (kind: {:?})", case, e, e),
        }
    }
    println!();
    
    // 3. Error chaining
    println!("3. Error chaining:");
    match process_user_input("abc") {
        Ok(result) => println!("   Processed: {}", result),
        Err(e) => {
            println!("   Error: {}", e);
            
            // Walk the error chain
            let mut source = e.source();
            while let Some(err) = source {
                println!("   Caused by: {}", err);
                source = err.source();
            }
        }
    }
}

// Simple custom error
#[derive(Debug)]
struct AgeError {
    message: String,
}

impl fmt::Display for AgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Age validation error: {}", self.message)
    }
}

impl Error for AgeError {}

fn validate_age(input: &str) -> Result<u8, AgeError> {
    let age: u8 = input.parse().map_err(|_| AgeError {
        message: "Age must be a valid number".to_string(),
    })?;
    
    if age > 120 {
        Err(AgeError {
            message: "Age cannot be greater than 120".to_string(),
        })
    } else {
        Ok(age)
    }
}

// Enum-based error types
#[derive(Debug, Clone)]
enum ValidationError {
    InvalidFormat,
    NegativeAge,
    TooOld,
    TooYoung,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::InvalidFormat => write!(f, "Invalid number format"),
            ValidationError::NegativeAge => write!(f, "Age cannot be negative"),
            ValidationError::TooOld => write!(f, "Age too old (max 120)"),
            ValidationError::TooYoung => write!(f, "Age too young (min 0)"),
        }
    }
}

impl Error for ValidationError {}

fn parse_and_validate_age(input: &str) -> Result<u8, ValidationError> {
    let age: i32 = input.parse().map_err(|_| ValidationError::InvalidFormat)?;
    
    if age < 0 {
        Err(ValidationError::NegativeAge)
    } else if age > 120 {
        Err(ValidationError::TooOld)
    } else {
        Ok(age as u8)
    }
}

// Error with source chain
#[derive(Debug)]
struct ProcessingError {
    kind: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Processing error: {}", self.kind)
    }
}

impl Error for ProcessingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

fn process_user_input(input: &str) -> Result<u32, ProcessingError> {
    let number: u32 = input.parse().map_err(|e: ParseIntError| ProcessingError {
        kind: "Failed to parse input as number".to_string(),
        source: Some(Box::new(e)),
    })?;
    
    if number == 0 {
        return Err(ProcessingError {
            kind: "Number cannot be zero".to_string(),
            source: None,
        });
    }
    
    Ok(number * 2)
}
```

### **🛠️ How to Run This Code:**
1. **Online**: Copy individual examples to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `ch9_demo.rs` and run `rustc ch9_demo.rs && ./ch9_demo`
3. **In this workspace**: Navigate to specific project directories and use `cargo run`

---

## 🧠 **Key Concepts**

### **When to Panic vs Return Result**

**Use `panic!` when:**
- Programming bugs (array out of bounds)
- Contract violations (invariants broken)
- Prototype code
- Test code
- Situations that should "never" happen

**Use `Result` when:**
- File operations (file might not exist)
- Network operations (connection might fail)  
- User input validation (users make mistakes)
- Parsing operations (data might be malformed)
- Any expected failure scenario

### **Error Handling Patterns**

```rust
// Pattern 1: Convert errors with map_err()
fn read_number() -> Result<i32, String> {
    "42".parse().map_err(|e| format!("Parse error: {}", e))
}

// Pattern 2: Early return with ?
fn process_file() -> Result<String, std::io::Error> {
    let contents = std::fs::read_to_string("file.txt")?;
    Ok(contents.to_uppercase())
}

// Pattern 3: Default values with unwrap_or()
fn get_config_value() -> String {
    std::env::var("CONFIG").unwrap_or_else(|_| "default".to_string())
}

// Pattern 4: Chaining with and_then()
fn validate_and_process(input: &str) -> Result<u32, String> {
    input.parse::<u32>()
        .map_err(|_| "Invalid number".to_string())
        .and_then(|n| {
            if n > 0 {
                Ok(n * 2)
            } else {
                Err("Number must be positive".to_string())
            }
        })
}
```

### **Error Propagation Chain**

The `?` operator:
1. **If `Ok(T)`**: unwraps the value
2. **If `Err(E)`**: returns early with the error
3. **Auto-converts** error types (via `From` trait)
4. **Works with** `Option` (converts `None` to early return)

---

## 🎯 **Best Practices**

### 1. **Design Error Types Thoughtfully**
```rust
// Good: Specific error variants
enum FileError {
    NotFound(String),
    PermissionDenied(String),
    InvalidFormat { line: usize, column: usize },
}

// Less ideal: Generic error
struct GenericError(String);
```

### 2. **Provide Context in Errors**
```rust
// Good: Contextual information
Err(format!("Failed to parse line {} in file {}: {}", line_num, filename, err))

// Less helpful: Bare error
Err("Parse error".to_string())
```

### 3. **Use Type System for Prevention**
```rust
// Good: Impossible states unrepresentable
struct NonEmptyVec<T> {
    first: T,
    rest: Vec<T>,
}

// Instead of: Runtime checks everywhere
fn process_items(items: Vec<String>) -> Result<String, String> {
    if items.is_empty() {
        return Err("Items cannot be empty".to_string());
    }
    // process items...
}
```

---

## 🔗 **Related Chapters**
- **Chapter 6**: Enums and Pattern Matching (`Result` and `Option` types)
- **Chapter 10**: Generic Types (error type parameters)
- **Chapter 13**: Functional Features (`map`, `and_then`, closures with errors)
- **Chapter 15**: Smart Pointers (error handling in complex data structures)

---

## 📝 **Learning Notes**

### **Section Progress Tracker**

| Section | Status | Key Concepts Mastered |
|---------|--------|----------------------|
| **9.1 Unrecoverable Errors** | ✅ Complete | `panic!`, backtrace, safe alternatives |
| **9.2 Recoverable Errors** | ✅ Complete | `Result<T,E>`, custom errors, combinators |
| **9.3 Error Propagation** | ✅ Complete | `?` operator, error chaining, conversions |
| **Exercises** | ✅ Complete | Real-world patterns, retry logic, validation |

### **Practical Applications in This Workspace**
- **Mission Projects**: Apply error handling to data structure implementations
- **AoC Solutions**: Robust parsing and validation for competitive programming
- **Daily Study**: Error patterns reinforce Week 2 concepts
- **Tutorial System**: Error handling appears in all MissionX_tut projects

---

## 🎓 **Key Takeaways**

### **Design Philosophy**
- **Explicit > Implicit**: Rust forces you to handle errors consciously
- **Type Safety**: `Result<T, E>` and `Option<T>` prevent null pointer exceptions
- **Zero-cost Abstractions**: Error handling doesn't impact performance
- **Ergonomic**: The `?` operator makes error propagation clean and readable

### **When to Use Each Pattern**

| Pattern | Use Case | Example |
|---------|----------|---------|
| **`panic!`** | Programming bugs, impossible states | Array bounds violations, contract violations |
| **`Result<T, E>`** | Expected failures, recoverable errors | File I/O, network operations, user input |
| **`Option<T>`** | Values that might not exist | Array access, HashMap lookups, parsing |
| **`?` operator** | Error propagation in function chains | File processing pipelines, validation chains |

### **Advanced Patterns Learned**
- **Custom Error Types**: Enum-based errors with `Display` and `Error` traits
- **Error Conversion**: `From` trait for automatic error type conversion
- **Error Chaining**: Using `source()` to preserve error context
- **Recovery Patterns**: Fallbacks, retries, and error collection
- **Mixed Types**: Converting between `Result` and `Option` as needed

---

## 🔗 **Integration Points**

### **Cross-Chapter Connections**
- **Chapter 6**: Error types use enums and pattern matching extensively
- **Chapter 8**: Collections often return `Option` for safe access patterns  
- **Chapter 10**: Generic error types `Result<T, E>` and trait bounds
- **Chapter 13**: Functional error handling with `map`, `and_then`, closures
- **Chapter 15**: Smart pointers and error handling in complex data structures

### **Workspace Integration** 
- **Mission1-7**: Add proper error handling to all data structure operations
- **Daily Study**: Week 2 error concepts now have complete implementation examples
- **Competitive Programming**: Robust parsing patterns for AoC and contests
- **Tutorial System**: Error handling appears in progressive learning examples

---

## � **Next Steps**

1. **Practice with Exercises**: Complete all 6 exercise categories for mastery
2. **Apply to Missions**: Refactor existing Mission projects to use proper error handling
3. **Advanced Patterns**: Study `thiserror` and `anyhow` crates for production code
4. **Integration**: Use error patterns in upcoming chapters (generics, traits, lifetimes)

---

*Chapter 9 Complete: Error handling is now a core competency for reliable Rust programming!*

*Tags: #rust-book #chapter9 #error-handling #result #panic #option #complete*
*Links: [[../Ch8/README]] | [[../Ch10/README]] | [[Mission5]] | [[daily-study/Day05]] | [[zettel-index]] | [[Error Handling Deep Dive]] | [[Week 5 Overview]] | [[daily-study/Day29]] | [[daily-study/Day30]] | [[daily-study/Day31]] | [[rust-concepts-MOC]]*
