# String Manipulation

*Created: 2025-11-08*
*Tags: #rust-strings #text-processing #performance #unicode #memory-management*

## Overview

String manipulation in Rust involves working with UTF-8 encoded text through multiple string types, each optimized for different use cases. Rust's string handling balances **memory safety**, **Unicode correctness**, and **performance**.

## Core String Types

### String vs &str

```rust
// Owned, growable UTF-8 string
let mut owned = String::from("Hello");
owned.push_str(", World!");

// String slice - borrowed view of UTF-8 text
let slice: &str = "Hello, World!";
let borrowed: &str = &owned; // Borrow from String
```

### Key Differences

- **String**: Owned, heap-allocated, growable
- **&str**: Borrowed, immutable view, can point to stack/heap/binary

## Common Operations

### Creation and Conversion

```rust
// Various creation methods
let s1 = String::new();
let s2 = String::from("literal");
let s3 = "literal".to_string();
let s4 = "literal".to_owned();

// From other types
let s5 = format!("Number: {}", 42);
let s6 = 42_i32.to_string();
```

### Modification

```rust
let mut s = String::new();
s.push('H');           // Single character
s.push_str("ello");    // String slice
s += " World";         // Operator overloading
s = s + "!";           // Takes ownership, returns new String
```

### Inspection and Search

```rust
let text = "Hello, 世界";

// Length and capacity
println!("Length: {} bytes", text.len());          // 13 bytes (UTF-8)
println!("Chars: {}", text.chars().count());       // 8 characters

// Search operations
println!("Contains 'World': {}", text.contains("世界"));
println!("Starts with 'Hello': {}", text.starts_with("Hello"));
println!("Position: {:?}", text.find("世"));       // Some(7)
```

## AoC String Patterns

### Day 11: Password Generation

```rust
// Increment string like number
fn increment_password(password: &mut String) {
    let bytes = unsafe { password.as_bytes_mut() };
    for i in (0..bytes.len()).rev() {
        if bytes[i] == b'z' {
            bytes[i] = b'a';
        } else {
            bytes[i] += 1;
            break;
        }
    }
}

// Character validation patterns
fn has_straight(s: &str) -> bool {
    s.chars()
        .collect::<Vec<_>>()
        .windows(3)
        .any(|w| w[0] as u8 + 1 == w[1] as u8 && w[1] as u8 + 1 == w[2] as u8)
}
```

### Day 12: JSON Parsing

```rust
// Recursive string processing
fn sum_numbers(json: &str) -> i32 {
    let mut sum = 0;
    let mut current_number = String::new();
    let mut in_number = false;
    
    for ch in json.chars() {
        match ch {
            '0'..='9' | '-' => {
                current_number.push(ch);
                in_number = true;
            }
            _ if in_number => {
                sum += current_number.parse::<i32>().unwrap_or(0);
                current_number.clear();
                in_number = false;
            }
            _ => {}
        }
    }
    sum
}
```

### Day 1: String Traversal

```rust
// Character-by-character processing
fn solve_elevator(instructions: &str) -> (i32, Option<usize>) {
    let mut floor = 0;
    let mut basement_pos = None;
    
    for (i, ch) in instructions.chars().enumerate() {
        match ch {
            '(' => floor += 1,
            ')' => {
                floor -= 1;
                if floor == -1 && basement_pos.is_none() {
                    basement_pos = Some(i + 1);
                }
            }
            _ => {}
        }
    }
    (floor, basement_pos)
}
```

## Performance Considerations

### Memory Allocation

```rust
// Efficient: pre-allocate capacity
let mut result = String::with_capacity(1000);
for i in 0..100 {
    result.push_str(&format!("Item {}, ", i));
}

// Inefficient: multiple reallocations
let mut result = String::new();
for i in 0..100 {
    result = result + &format!("Item {}, ", i); // New allocation each time
}
```

### String Building Patterns

```rust
// collect() method - efficient
let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
let result: String = numbers
    .iter()
    .map(|n| n.to_string())
    .collect::<Vec<_>>()
    .join(", ");

// format! macro - convenient
let formatted = format!("Numbers: {}", result);

// Direct buffer manipulation - fastest
let mut buffer = String::with_capacity(estimated_size);
for (i, &num) in numbers.iter().enumerate() {
    if i > 0 { buffer.push_str(", "); }
    buffer.push_str(&num.to_string());
}
```

## Unicode Handling

### Character vs Byte Operations

```rust
let s = "Hello, 世界!";

// Correct: character iteration
for ch in s.chars() {
    println!("Char: {}", ch);
}

// Dangerous: byte iteration for non-ASCII
for byte in s.bytes() {
    println!("Byte: {}", byte);
}

// Safe byte manipulation
let mut bytes = s.as_bytes().to_vec();
// Modify bytes safely...
let result = String::from_utf8(bytes)?;
```

### Slicing Safely

```rust
// Dangerous: may panic on UTF-8 boundaries
// let slice = &s[0..5]; // Could panic

// Safe: character-based slicing
let safe_slice: String = s.chars().take(5).collect();

// Byte-boundary safe slicing
if s.is_char_boundary(5) {
    let slice = &s[0..5];
}
```

## Mission Integration

### Mission 1: Stack<String>

- String ownership in stack operations
- Move semantics with string data

### Mission 5: HashMap<String, V>

- String keys and hash computation
- Borrowing vs owning string keys

### Mission 9: JSON Processing

- String parsing and validation
- Memory-efficient string building

## Common Patterns

### String Interning

```rust
use std::collections::HashMap;

struct StringInterner {
    strings: HashMap<String, usize>,
    values: Vec<String>,
}

impl StringInterner {
    fn intern(&mut self, s: &str) -> usize {
        if let Some(&id) = self.strings.get(s) {
            id
        } else {
            let id = self.values.len();
            self.values.push(s.to_string());
            self.strings.insert(s.to_string(), id);
            id
        }
    }
}
```

### String Pool Pattern

```rust
// Avoid repeated allocations
fn process_words(text: &str) -> HashMap<String, usize> {
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        // Use to_lowercase() once, store owned String
        let key = word.to_lowercase();
        *word_count.entry(key).or_insert(0) += 1;
    }
    word_count
}
```

## Error Handling

### String Validation

```rust
use std::str::FromStr;

#[derive(Debug)]
struct ValidatedString(String);

impl FromStr for ValidatedString {
    type Err = &'static str;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 100 {
            Err("String too long")
        } else if s.is_empty() {
            Err("String is empty")
        } else {
            Ok(ValidatedString(s.to_string()))
        }
    }
}
```

### UTF-8 Conversion

```rust
// Safe UTF-8 handling
fn process_bytes(bytes: Vec<u8>) -> Result<String, String> {
    String::from_utf8(bytes)
        .map_err(|e| format!("Invalid UTF-8: {}", e))
}

// Lossy conversion for resilience
fn process_bytes_lossy(bytes: Vec<u8>) -> String {
    String::from_utf8_lossy(&bytes).into_owned()
}
```

## Integration with Other Concepts

- **[[Memory Safety]]**: Preventing buffer overflows and invalid UTF-8
- **[[Performance Patterns]]**: Efficient string processing techniques
- **[[ownership]]**: String ownership and borrowing rules
- **[[Performance Benchmarking]]**: Measuring string operation costs
- **[[zero-cost-abstractions]]**: Iterator patterns over strings

## Daily Study Applications

### Week 2: Collections and String Processing

- String as a collection of UTF-8 bytes
- Efficient string building patterns

### Week 3: Advanced String Patterns

- Regular expressions and pattern matching
- Custom string types and validation

### Week 5: Error Handling with Strings

- String parsing and error propagation
- Custom error types with string descriptions

## Further Reading

- [[Rust Book]] Chapter 8: Collections - Strings
- [[Performance Patterns]]: String optimization techniques
- [[ownership]]: String ownership semantics
- [[Memory Safety]]: Safe string manipulation

---

*String Manipulation Links:*

- [[Performance Benchmarking]] - String operation measurement
- [[zero-cost-abstractions]] - Iterator patterns over strings
- [[Memory Safety]] - Safe string operations
- [[ownership]] - String ownership rules
- [[Performance Patterns]] - String optimization
- [[Rust Book]] - String documentation
- [[mission-5]] - HashMap with string keys
- [[mission-9]] - JSON string processing
