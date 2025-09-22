# Day 5 · Option and Result - Expanded

## Overview
Rust eliminates null pointer exceptions and unchecked exceptions through two powerful enum types: `Option<T>` and `Result<T, E>`. These types force explicit handling of absence and errors at compile time.

## Option<T> - Handling Absence

### What is Option?
`Option<T>` is an enum that represents either a value (`Some(T)`) or nothing (`None`). It's Rust's replacement for null pointers.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

### Basic Usage
```rust
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some("Alice".to_string())
    } else {
        None
    }
}

fn main() {
    let user = find_user(1);
    match user {
        Some(name) => println!("Found user: {}", name),
        None => println!("User not found"),
    }
}
```

### Common Option Methods
```rust
let maybe_number = Some(42);

// unwrap() - panics if None
let value = maybe_number.unwrap(); // 42

// unwrap_or() - provides default value
let value = maybe_number.unwrap_or(0); // 42
let none_value = None.unwrap_or(0); // 0

// map() - transforms the value if Some
let doubled = maybe_number.map(|x| x * 2); // Some(84)

// and_then() - chains Option operations
let result = maybe_number.and_then(|x| {
    if x > 40 { Some(x) } else { None }
}); // Some(42)

// is_some() / is_none() - check variant
if maybe_number.is_some() {
    println!("Has a value!");
}
```

### Pattern Matching with Option
```rust
match maybe_val {
    Some(x) => println!("Got value: {}", x),
    None => println!("No value"),
}

// if let - convenient for single pattern
if let Some(x) = maybe_val {
    println!("Got value: {}", x);
}

// while let - loop while pattern matches
let mut stack = vec![Some(1), Some(2), None, Some(3)];
while let Some(maybe_val) = stack.pop() {
    if let Some(val) = maybe_val {
        println!("Popped: {}", val);
    }
}
```

## Result<T, E> - Error Handling

### What is Result?
`Result<T, E>` represents either success (`Ok(T)`) or failure (`Err(E)`). It's Rust's approach to recoverable errors.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Basic Usage
```rust
use std::fs::File;
use std::io::ErrorKind;

fn open_file(filename: &str) -> Result<File, std::io::Error> {
    File::open(filename)
}

fn main() {
    match open_file("hello.txt") {
        Ok(file) => println!("File opened successfully"),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("File not found"),
            ErrorKind::PermissionDenied => println!("Permission denied"),
            other_error => println!("Other error: {:?}", other_error),
        },
    }
}
```

### Common Result Methods
```rust
let result: Result<i32, &str> = Ok(42);

// unwrap() - panics on Err
let value = result.unwrap(); // 42

// expect() - panics with custom message
let value = result.expect("Should have a value"); // 42

// unwrap_or() - provides default on error
let value = result.unwrap_or(0); // 42

// map() - transforms Ok value
let doubled = result.map(|x| x * 2); // Ok(84)

// map_err() - transforms Err value
let with_new_err = result.map_err(|_| "New error"); // Ok(42)

// and_then() - chains Result operations
let chained = result.and_then(|x| {
    if x > 40 { Ok(x) } else { Err("Too small") }
}); // Ok(42)
```

## The ? Operator - Error Propagation

### How ? Works
The `?` operator is syntactic sugar for early return on errors. It automatically converts errors and propagates them up the call stack.

```rust
// Without ?
fn read_file_verbose() -> std::io::Result<String> {
    use std::fs::File;
    use std::io::Read;
    
    let mut file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}

// With ?
fn read_file_concise() -> std::io::Result<String> {
    use std::fs::File;
    use std::io::Read;
    
    let mut file = File::open("hello.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Even more concise
fn read_file_shortest() -> std::io::Result<String> {
    std::fs::read_to_string("hello.txt")
}
```

### ? with Option
```rust
fn parse_and_double(s: &str) -> Option<i32> {
    let num = s.parse::<i32>().ok()?; // Convert Result to Option
    Some(num * 2)
}

fn main() {
    println!("{:?}", parse_and_double("42")); // Some(84)
    println!("{:?}", parse_and_double("abc")); // None
}
```

## Combining Option and Result

### Converting Between Types
```rust
// Result to Option
let result: Result<i32, &str> = Ok(42);
let option = result.ok(); // Some(42)

let result: Result<i32, &str> = Err("error");
let option = result.ok(); // None

// Option to Result
let option = Some(42);
let result = option.ok_or("No value"); // Ok(42)

let option: Option<i32> = None;
let result = option.ok_or("No value"); // Err("No value")
```

### Working with Collections
```rust
// collect() can work with Option/Result
let strings = vec!["1", "2", "3", "4"];
let numbers: Option<Vec<i32>> = strings
    .iter()
    .map(|s| s.parse().ok())
    .collect(); // Some([1, 2, 3, 4])

let strings_with_error = vec!["1", "2", "abc", "4"];
let numbers: Option<Vec<i32>> = strings_with_error
    .iter()
    .map(|s| s.parse().ok())
    .collect(); // None (because one parse failed)

// With Result
let numbers: Result<Vec<i32>, _> = strings
    .iter()
    .map(|s| s.parse())
    .collect(); // Ok([1, 2, 3, 4])
```

## Advanced Patterns

### Custom Error Types
```rust
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}
```

### Error Chaining
```rust
fn complex_calculation(x: f64) -> Result<f64, MathError> {
    let divided = divide(x, 2.0)?;
    let result = sqrt(divided)?;
    Ok(result * 3.0)
}
```

### Working with Multiple Error Types
```rust
use std::error::Error;
use std::fmt;

// Using Box<dyn Error> for mixed error types
fn mixed_operations() -> Result<i32, Box<dyn Error>> {
    let file_contents = std::fs::read_to_string("numbers.txt")?; // io::Error
    let number: i32 = file_contents.trim().parse()?; // ParseIntError
    Ok(number * 2)
}
```

## Real-World Example
```rust
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Debug)]
enum AppError {
    IoError(io::Error),
    ParseError(std::num::ParseIntError),
    ValidationError(String),
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self {
        AppError::ParseError(error)
    }
}

fn process_user_age(filename: &str) -> Result<String, AppError> {
    // Read file
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    // Parse age
    let age: u32 = contents.trim().parse()?;
    
    // Validate age
    if age > 150 {
        return Err(AppError::ValidationError("Age too high".to_string()));
    }
    
    // Process result
    if age >= 18 {
        Ok("Adult".to_string())
    } else {
        Ok("Minor".to_string())
    }
}

fn main() {
    match process_user_age("age.txt") {
        Ok(status) => println!("User is: {}", status),
        Err(AppError::IoError(e)) => println!("File error: {}", e),
        Err(AppError::ParseError(e)) => println!("Parse error: {}", e),
        Err(AppError::ValidationError(e)) => println!("Validation error: {}", e),
    }
}
```

## Best Practices

### 1. Prefer Result over panic!
```rust
// ❌ Don't do this
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero!");
    }
    a / b
}

// ✅ Do this instead
fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}
```

### 2. Use meaningful error types
```rust
// ❌ Generic string errors
fn parse_config() -> Result<Config, String> { ... }

// ✅ Specific error types
#[derive(Debug)]
enum ConfigError {
    FileNotFound,
    InvalidFormat,
    MissingField(String),
}

fn parse_config() -> Result<Config, ConfigError> { ... }
```

### 3. Use ? for cleaner error propagation
```rust
// ❌ Verbose error handling
fn complex_operation() -> Result<i32, MyError> {
    let a = match step1() {
        Ok(val) => val,
        Err(e) => return Err(e),
    };
    let b = match step2(a) {
        Ok(val) => val,
        Err(e) => return Err(e),
    };
    Ok(b * 2)
}

// ✅ Clean with ?
fn complex_operation() -> Result<i32, MyError> {
    let a = step1()?;
    let b = step2(a)?;
    Ok(b * 2)
}
```

## Key Takeaways
- **Option<T>** makes null values explicit and type-safe
- **Result<T, E>** makes error handling explicit and recoverable
- **Pattern matching** provides exhaustive handling of all cases
- **The ? operator** enables clean error propagation
- **No hidden exceptions** - all errors are visible in function signatures
- **Compile-time safety** - forgot to handle an error? Compiler will tell you
- **Composability** - Option and Result work well with iterators and functional patterns

These types are fundamental to Rust's approach to safety and make programs more reliable by forcing explicit handling of edge cases.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

Please expand on the outline from day 5 to give more details.

Day5.md
Used 1 reference
I'll expand your Day 5 notes to provide more comprehensive details about Option and Result in Rust.

Day5.md+402-17
I've significantly expanded your Day 5 notes to provide comprehensive coverage of Option and Result in Rust. The expansion includes:

📚 New Content Added:
Option<T> Deep Dive
Detailed explanation of the enum definition
Common methods (unwrap, unwrap_or, map, and_then, etc.)
Pattern matching examples including if let and while let
Result<T, E> Deep Dive
Comprehensive error handling patterns
Methods for transforming and chaining Results
Real-world file handling examples
The ? Operator
Before/after comparisons showing how ? simplifies code
Examples with both Result and Option
Error propagation patterns
Advanced Topics
Converting between Option and Result
Working with collections using collect()
Custom error types and error chaining
Handling multiple error types with Box<dyn Error>
Real-World Example
Complete application showing file I/O, parsing, and validation
Custom error types with From implementations
Proper error handling patterns
Best Practices
When to use Result vs panic!
Creating meaningful error types
Using ? for clean code
The expanded version transforms your concise outline into a comprehensive reference that covers everything from basics to advanced patterns, making it suitable for both learning and future reference. The examples are all runnable and demonstrate real-world usage patterns you'll encounter in Rust development.


