# Day 5 - Option and Result

**Quick Reference Note**
*For full content, see: [[daily_study/rust_learning_week1_notes/Day5]]*

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
    Err(e) => eprintln!("Error: {}", e),
}
```

---

## The ? Operator (Error Propagation)

**Before ?:**
```rust
fn process() -> Result<i32, Error> {
    let val = match parse_number() {
        Ok(v) => v,
        Err(e) => return Err(e),  // Early return
    };
    Ok(val * 2)
}
```

**With ?:**
```rust
fn process() -> Result<i32, Error> {
    let val = parse_number()?;  // Returns Err early if error
    Ok(val * 2)
}
```

**? with Option:**
```rust
fn get_first_char(text: &str) -> Option<char> {
    text.chars().next()  // Returns None if empty
}

fn process(text: &str) -> Option<char> {
    let first = get_first_char(text)?;  // Returns None early
    Some(first.to_uppercase().next()?)
}
```

---

## Common Methods

### Option Methods
```rust
let x = Some(5);

x.is_some()           // true
x.is_none()           // false
x.unwrap()            // 5 (panics on None!)
x.unwrap_or(0)        // 5 (default if None)
x.map(|v| v * 2)      // Some(10)
x.and_then(|v| Some(v + 1))  // Some(6) - chain Options
```

### Result Methods
```rust
let r: Result<i32, &str> = Ok(42);

r.is_ok()             // true
r.is_err()            // false
r.unwrap()            // 42 (panics on Err!)
r.unwrap_or(0)        // 42 (default if Err)
r.map(|v| v * 2)      // Ok(84)
r.map_err(|e| format!("Error: {}", e))  // Transform error
```

---

## Conversion Between Types

```rust
// Result → Option
let result: Result<i32, String> = Ok(42);
let option = result.ok();  // Some(42)

let result: Result<i32, String> = Err("error".into());
let option = result.ok();  // None

// Option → Result
let option = Some(42);
let result = option.ok_or("No value");  // Ok(42)

let option: Option<i32> = None;
let result = option.ok_or("No value");  // Err("No value")
```

---

## Pattern Matching

### Option
```rust
match maybe_value {
    Some(x) => println!("Got: {}", x),
    None => println!("Nothing"),
}

// if let shorthand
if let Some(x) = maybe_value {
    println!("Got: {}", x);
}
```

### Result
```rust
match operation() {
    Ok(value) => println!("Success: {}", value),
    Err(error) => eprintln!("Failed: {}", error),
}

// if let shorthand
if let Ok(value) = operation() {
    println!("Success: {}", value);
}
```

---

## Quick Rules

- **Use Option<T>** for nullable values (no null pointers!)
- **Use Result<T, E>** for operations that can fail
- **Use ? operator** for clean error propagation
- **Never unwrap()** in production - use safer alternatives
- **Pattern match** for exhaustive handling

---

## Common Patterns

### Entry API with Option
```rust
let mut cache: HashMap<String, Data> = HashMap::new();

cache.entry("key".to_string())
    .or_insert_with(|| expensive_computation());
```

### Chaining Operations
```rust
let result = parse_input(text)
    .map(|data| process(data))
    .and_then(|processed| validate(processed))
    .unwrap_or_default();
```

---

## Quick Navigation

- **Full Details**: [[daily_study/rust_learning_week1_notes/Day5]]
- **Previous**: [[Day 4 - Lifetimes]]
- **Next**: [[Day 6 - Pattern Matching]]
- **Week**: [[Week 1 Overview]]
- **MOC**: [[Rust Concepts MOC]]
- **Rust Book**: Chapter 6 (Option), Chapter 9 (Result)

---

*Tags: #option #result #error-handling #question-mark-operator #quick-ref*
