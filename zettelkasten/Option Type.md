# Option Type - Rust's Null Safety Solution

**`Option<T>` is Rust's type-safe replacement for null pointers**, one of the language's most important safety features.

## 🎯 The Core Problem: Null References

Tony Hoare called null references his **"billion-dollar mistake"**. In C, C++, and Java:

```c
// C - null pointer dangers
char* name = NULL;
printf("%s", name);  // 💥 Segmentation fault (undefined behavior)

// Java - NullPointerException
String name = null;
int len = name.length();  // 💥 Runtime crash
```

**The Problem**: The type system doesn't track whether a value can be absent.

## 🦀 Rust's Solution: Option<T>

```rust
enum Option<T> {
    Some(T),    // Value is present
    None,       // Value is absent
}
```

This is a **built-in enum** that makes absence explicit in the type system.

## 📊 Option vs Null Comparison

| Aspect | C/Java Null | Rust Option<T> |
|--------|-------------|----------------|
| **Type Safety** | ❌ Can forget to check | ✅ Compiler forces handling |
| **Explicit** | ❌ Hidden in type | ✅ Visible in type signature |
| **Runtime Crashes** | ❌ Common | ✅ Prevented at compile time |
| **Documentation** | ❌ Not in type | ✅ Self-documenting |

## 💡 Basic Usage

### **Creating Option Values**

```rust
// Explicit creation
let some_number = Some(5);
let some_string = Some(String::from("hello"));
let absent_number: Option<i32> = None;

// From functions
fn find_user(id: u32) -> Option<User> {
    if id == 1 {
        Some(User { name: "Alice".to_string() })
    } else {
        None  // User not found
    }
}
```

### **Consuming Option Values**

#### **Method 1: Pattern Matching** (Most powerful)

```rust
let x = Some(5);

match x {
    Some(value) => println!("Got value: {}", value),
    None => println!("Got nothing"),
}
```

#### **Method 2: if let** (Concise for single case)

```rust
if let Some(value) = x {
    println!("Got value: {}", value);
}
```

#### **Method 3: Unwrap Methods** (Use carefully!)

```rust
// unwrap() - panics on None
let x = Some(5);
let value = x.unwrap();  // 5

// unwrap_or() - provide default
let x: Option<i32> = None;
let value = x.unwrap_or(0);  // 0

// unwrap_or_else() - compute default lazily
let value = x.unwrap_or_else(|| expensive_computation());
```

## 🔧 Common Option Methods

### **Checking Presence**

```rust
let x = Some(5);
assert!(x.is_some());    // true
assert!(!x.is_none());   // false

let y: Option<i32> = None;
assert!(y.is_none());    // true
```

### **Transforming with map()**

```rust
let maybe_number = Some(5);
let maybe_string = maybe_number.map(|n| n.to_string());
// Some("5")

let nothing: Option<i32> = None;
let still_nothing = nothing.map(|n| n * 2);
// None - map does nothing on None
```

### **Chaining with and_then()**

```rust
fn square(x: i32) -> Option<i32> {
    Some(x * x)
}

let result = Some(3)
    .and_then(square)     // Some(9)
    .and_then(square);    // Some(81)

let result = None
    .and_then(square);    // None - short-circuits
```

### **Providing Defaults**

```rust
let x = Some(5);
assert_eq!(x.or(Some(10)), Some(5));  // Use x if present

let y: Option<i32> = None;
assert_eq!(y.or(Some(10)), Some(10)); // Use default if None
```

## 🌍 Real-World Examples

### **Dictionary Lookups**

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Alice", 10);

// get() returns Option<&V>
match scores.get("Alice") {
    Some(&score) => println!("Alice's score: {}", score),
    None => println!("Alice not found"),
}

// Using unwrap_or for defaults
let bob_score = scores.get("Bob").unwrap_or(&0);
```

### **Parsing Input**

```rust
// str::parse returns Result, but demonstrates Option pattern
let maybe_number: Option<i32> = "42".parse().ok();  // Some(42)
let not_number: Option<i32> = "abc".parse().ok();   // None
```

### **Array Indexing (Safe)**

```rust
let numbers = vec![1, 2, 3, 4, 5];

// Standard indexing panics on out-of-bounds
// let x = numbers[10];  // 💥 Panic!

// get() returns Option - safe!
match numbers.get(10) {
    Some(&value) => println!("Found: {}", value),
    None => println!("Index out of bounds"),
}
```

### **First/Last Elements**

```rust
let numbers = vec![1, 2, 3];
let first = numbers.first();   // Some(&1)
let last = numbers.last();     // Some(&3)

let empty: Vec<i32> = vec![];
let first = empty.first();     // None
```

## 🎄 AoC Applications

### **Finding Elements**

```rust
// AoC: Find character in grid
fn find_char(grid: &[Vec<char>], target: char) -> Option<(usize, usize)> {
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == target {
                return Some((row, col));
            }
        }
    }
    None
}
```

### **Parsing Grid Cells**

```rust
// AoC: Safe grid access
fn get_cell(grid: &[Vec<i32>], row: usize, col: usize) -> Option<i32> {
    grid.get(row)
        .and_then(|r| r.get(col))
        .copied()
}

// Usage
match get_cell(&grid, 5, 10) {
    Some(value) => process(value),
    None => println!("Out of bounds"),
}
```

## 🔗 Relationship to Result<T, E>

`Option<T>` and `Result<T, E>` are closely related:

```rust
// Option: value present or absent
enum Option<T> {
    Some(T),
    None,
}

// Result: success or error (with error information)
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

**When to use which:**
- **`Option<T>`**: Value might be absent (not an error)
- **`Result<T, E>`**: Operation might fail (error needs explanation)

## 🎓 Why Option is Brilliant

1. **Compile-Time Safety**: Cannot use None value without checking
2. **Explicit Intent**: Function signature shows value might be absent
3. **No Runtime Overhead**: Zero-cost abstraction (same as null pointer)
4. **Composability**: Rich API for chaining operations
5. **Self-Documenting**: Type tells you the story

## 🧪 Common Patterns

### **Option to Result Conversion**

```rust
let opt: Option<i32> = Some(5);
let res: Result<i32, &str> = opt.ok_or("value was None");
```

### **Collecting Options**

```rust
let numbers = vec![Some(1), Some(2), None, Some(4)];

// Collect into Option<Vec<_>> - returns None if any element is None
let collected: Option<Vec<i32>> = numbers.into_iter().collect();
// None (because there's a None in the input)

// Filter out Nones
let filtered: Vec<i32> = numbers.into_iter().flatten().collect();
// vec![1, 2, 4]
```

### **The ? Operator** (With Option in functions)

```rust
fn add_two_numbers(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    Some(a? + b?)  // Early return on None
}

assert_eq!(add_two_numbers(Some(3), Some(5)), Some(8));
assert_eq!(add_two_numbers(Some(3), None), None);
```

## 📚 Further Reading

- **Rust Book Chapter 6**: Enums and Pattern Matching
- **Rust Book Chapter 9**: Error Handling (Result type)
- **Rust by Example**: Option and Result patterns
- **API Documentation**: `std::option::Option`

---

## 🔗 Related Concepts

- [[Result Type]] - Error handling with detailed error information
- [[Pattern Matching]] - How to safely extract Option values
- [[RUST_VS_C_ENUMS]] - Why Rust enums are superior to C enums
- [[Error Handling Deep Dive]] - Comprehensive error handling strategies
- [[Day 05 - Option and Result]] - Daily study introduction

---

## 🔗 Navigation

### 📚 Zettelkasten
- **[[zettel-index]]** - Main knowledge base entry point
- **[[Rust Concepts MOC]]** - Core language features
- **[[Collections MOC]]** - HashMap, Vec operations that return Option

### 🎯 Related Learning
- **[[Day 05 - Option and Result]]** - Foundational introduction
- **[[Error Handling Deep Dive]]** - Advanced patterns
- **[[Pattern Matching]]** - Essential for working with Option

---

*Tags: #option #enums #null-safety #type-safety #error-handling #rust-fundamentals #rust-book #ch6*
