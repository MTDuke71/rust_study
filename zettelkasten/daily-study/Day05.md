# Day 5 - Option and Result

**Quick Reference Note**
*For full content, see: [[daily_study/rust_learning_week1_notes/Day05]]*

---

## Core Concepts

### Option<T> - Handling Absence
```rust
enum Option<T> {
    Some(T),  // Has a value
    None,     // No value (replaces null)
}
```

**Common Usage:**
```rust
let maybe_number: Option<i32> = Some(42);
let nothing: Option<i32> = None;

// Unwrap (panics on None - use carefully!)
let value = maybe_number.unwrap();

// Safe alternatives
let value = maybe_number.unwrap_or(0);        // Default value
let value = maybe_number.unwrap_or_else(|| calculate_default());
```

### Result<T, E> - Handling Errors
```rust
enum Result<T, E> {
    Ok(T),   // Success with value
    Err(E),  // Error with error info
}
```

**Common Usage:**
```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Pattern matching
match divide(10.0, 2.0) {
    Ok(result) => println!("Result: {}", result),
    Err(error) => println!("Error: {}", error),
}
```

---

## Error Propagation with ?

```rust
fn read_file_contents(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;  // ? returns early on error
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;   // ? returns early on error
    Ok(contents)
}
```

**The ? operator:**
- **If `Ok(value)`**: unwraps the value
- **If `Err(error)`**: returns early with the error
- **Only works in functions returning `Result` or `Option`**

---

## Common Patterns

### Option Patterns
```rust
// Safe unwrapping
let value = option.unwrap_or_default();
let value = option.unwrap_or_else(|| expensive_computation());

// Chaining operations
let result = maybe_number
    .map(|n| n * 2)
    .filter(|n| *n > 10)
    .unwrap_or(0);
```

### Result Patterns
```rust
// Error handling
let result = risky_operation()
    .map_err(|e| format!("Failed: {}", e))
    .and_then(|value| process_value(value));

// Early returns
fn process_data(input: &str) -> Result<String, String> {
    let parsed = parse_input(input)?;  // Returns early on error
    let validated = validate(parsed)?; // Returns early on error
    Ok(transform(validated))
}
```

---

## Quick Rules

- **Use `Option<T>`** for values that might not exist
- **Use `Result<T, E>`** for operations that might fail
- **Use `?` operator** for clean error propagation
- **Never use `.unwrap()`** in production code without good reason
- **Pattern matching** is the idiomatic way to handle both types

---

## Quick Navigation

- **Full Details**: [[daily_study/rust_learning_week1_notes/Day05]]
- **Previous**: [[daily-study/Day04]]
- **Next**: [[zettelkasten/daily-study/Day06]]
- **Week**: [[Week 1 Overview]]
- **MOC**: [[rust-concepts-MOC]]

---

*Tags: #option #result #error-handling #quick-ref*