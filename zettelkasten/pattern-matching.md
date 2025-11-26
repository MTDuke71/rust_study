# Pattern Matching in Rust

*Tags: #pattern-matching #rust-concepts #tutorial #learning-track #competitive-programming*  
*Links: [[rust-book-ch6]] | [[Option Type]] | [[Text Parsing Patterns]] | [[AoC Patterns MOC]]*

---

## 🎯 Overview

Pattern matching is one of Rust's most powerful features, enabling you to destructure and match complex data structures elegantly and safely. It goes far beyond simple `switch` statements—it's a fundamental way to analyze and destructure values.

---

## 📚 Core Concepts

### **Basic Pattern Matching with `match`**

```rust
// Simple value matching
let number = 5;
match number {
    1 => println!("One"),
    2 | 3 => println!("Two or Three"),  // OR pattern
    4..=10 => println!("Between 4 and 10"),
    _ => println!("Something else"),  // Wildcard
}
```

### **Destructuring Enums**

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Pattern match on enum variants
let result: Result<i32, String> = Ok(42);
match result {
    Ok(value) => println!("Success: {}", value),
    Err(msg) => println!("Error: {}", msg),
}
```

### **Destructuring Structs**

```rust
struct Point { x: i32, y: i32 }

let point = Point { x: 0, y: 7 };

// Match on struct fields
match point {
    Point { x: 0, y } => println!("On y-axis at {}", y),
    Point { x, y: 0 } => println!("On x-axis at {}", x),
    Point { x, y } => println!("({}, {})", x, y),
}

// Or use shorthand with same field names
match point {
    Point { x, y } => println!("({}, {})", x, y),
}
```

### **Destructuring Tuples**

```rust
let tuple = (1, 2, 3);

match tuple {
    (0, y, z) => println!("First is zero: ({}, {})", y, z),
    (x, 0, z) => println!("Second is zero: ({}, {})", x, z),
    (x, y, z) => println!("All values: ({}, {}, {})", x, y, z),
}
```

---

## 🔍 Pattern Types

### **1. Literal Patterns**

```rust
match x {
    1 => { },           // Exact value
    "hello" => { },     // String literal
    true => { },        // Boolean
}
```

### **2. Range Patterns**

```rust
match x {
    1..=5 => { },       // Inclusive range
    6..10 => { },       // Exclusive range
    _ => { },
}
```

### **3. Named Variable Patterns**

```rust
match x {
    value => println!("Got {}", value),  // Binds value to `value`
}
```

### **4. Wildcard Pattern**

```rust
match x {
    1 => { },
    _ => { },  // Matches anything, doesn't bind
}
```

### **5. `ref` and `ref mut` Patterns**

```rust
let value = String::from("hello");
match value {
    ref s => println!("Got reference: {}", s),  // Borrow instead of move
}

// Or with mutable reference
match value {
    ref mut s => s.push_str(" world"),
}
```

### **6. Binding with `@`**

```rust
match x {
    num @ 1..=5 => println!("Small number: {}", num),
    num @ 6..=10 => println!("Medium number: {}", num),
    _ => { },
}
```

### **7. Guard Clauses**

```rust
match tuple {
    (x, y) if x > y => println!("{} > {}", x, y),
    (x, y) if x < y => println!("{} < {}", x, y),
    (x, y) => println!("{} == {}", x, y),
}
```

---

## 🎨 Common Patterns

### **Pattern 1: Option Handling**

```rust
let maybe_number: Option<i32> = Some(5);

match maybe_number {
    Some(n) => println!("Number: {}", n),
    None => println!("No number"),
}

// More concise using if let
if let Some(n) = maybe_number {
    println!("Number: {}", n);
}
```

### **Pattern 2: Result Handling**

```rust
let result: Result<String, String> = Ok("success".to_string());

match result {
    Ok(msg) => println!("Success: {}", msg),
    Err(e) => println!("Error: {}", e),
}

// Or use match expression value
let message = match result {
    Ok(msg) => msg,
    Err(e) => format!("Failed: {}", e),
};
```

### **Pattern 3: Nested Patterns**

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text: {}", text),
        Message::ChangeColor(r, g, b) => println!("RGB: {}, {}, {}", r, g, b),
    }
}
```

### **Pattern 4: Exhaustiveness Checking**

```rust
// ✅ CORRECT - Rust compiler ensures all cases handled
match option {
    Some(x) => { },
    None => { },
}

// ❌ COMPILER ERROR - Missing Some case
match option {
    None => { },
    // Missing Some case!
}

// ✅ CORRECT - Use wildcard to catch remaining
match option {
    Some(x) if x > 5 => { },
    _ => { },  // Catches Some(x) where x <= 5 and None
}
```

---

## 💡 Real-World Examples

### **Example 1: Bracket Validation**

```rust
// From Brackets_Basic project
fn validate_brackets(s: &str) -> Result<(), BracketError> {
    let mut stack = Vec::new();
    
    for (i, ch) in s.chars().enumerate() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' | ']' | '}' => {
                // Pattern match to validate pairing
                match (stack.pop(), ch) {
                    (Some('('), ')') | 
                    (Some('['), ']') | 
                    (Some('{'), '}') => { },
                    (None, _) => return Err(BracketError::UnexpectedClosing { position: i }),
                    _ => return Err(BracketError::MismatchedPair { position: i }),
                }
            },
            _ => { },  // Ignore non-bracket characters
        }
    }
    
    if stack.is_empty() {
        Ok(())
    } else {
        Err(BracketError::UnclosedOpenings { count: stack.len() })
    }
}
```

### **Example 2: AoC Day 12 - JSON Parsing**

```rust
// Conditional parsing based on JSON value type
fn sum_numbers(value: &JsonValue) -> i64 {
    match value {
        JsonValue::Number(n) => *n,
        JsonValue::String(_) => 0,
        JsonValue::Object(map) => {
            map.values()
                .map(|v| sum_numbers(v))
                .sum()
        },
        JsonValue::Array(arr) => {
            arr.iter()
                .map(|v| sum_numbers(v))
                .sum()
        },
    }
}
```

### **Example 3: State Machine**

```rust
enum State {
    Idle,
    Processing(i32),
    Complete(String),
    Error(String),
}

fn handle_state(state: State) {
    match state {
        State::Idle => println!("Waiting..."),
        State::Processing(id) => println!("Processing {}", id),
        State::Complete(msg) => println!("Done: {}", msg),
        State::Error(e) => println!("Error: {}", e),
    }
}
```

---

## 🔧 Pattern Matching vs Traditional Conditionals

### **Pattern Matching (Preferred in Rust)**

```rust
// ✅ Clear, safe, exhaustive
match value {
    Some(x) => { },
    None => { },
}
```

### **Traditional Conditionals (Less Idiomatic)**

```rust
// ⚠️ Less safe, not exhaustive checking
if let Some(x) = value {
    // ...
} else {
    // ...
}
```

---

## ⚡ Advanced Techniques

### **1. Match Expression as Value**

```rust
let number = 13;
let description = match number {
    1..=5 => "small",
    6..=10 => "medium",
    _ => "large",
};
```

### **2. Combining with Loops**

```rust
for item in items {
    match item {
        State::Process(data) => process(data),
        State::Skip => continue,
        State::Stop => break,
    }
}
```

### **3. Partial Patterns with `if let`**

```rust
// When you only care about one case
if let Some(x) = value {
    println!("Got: {}", x);
}

// Or destructure in loop
if let Ok(msg) = receiver.recv() {
    println!("Message: {}", msg);
}
```

### **4. Multiple Patterns with `|`**

```rust
match ch {
    'a' | 'e' | 'i' | 'o' | 'u' => println!("Vowel"),
    _ => println!("Consonant"),
}
```

---

## 📋 Pattern Matching Checklist

Before writing match expressions:

- [ ] **Exhaustiveness**: Does my match cover all possibilities?
- [ ] **Guard Clauses**: Do I need `if` conditions inside patterns?
- [ ] **Simplicity**: Could `if let` be clearer for single case?
- [ ] **Binding**: Am I capturing the values I need?
- [ ] **Fallback**: Is my `_` pattern appropriate?
- [ ] **Efficiency**: Am I avoiding unnecessary clones?

---

## 🚀 Performance Characteristics

- **No runtime overhead** - Pattern matching compiles to efficient machine code
- **Branch prediction** - CPU can predict matches effectively
- **Compile-time checking** - Exhaustiveness verified at compile time
- **Zero-cost abstractions** - Patterns are as fast as manual if/else chains

---

## 📖 Related Topics

- **[[Option Type]]** - Using pattern matching with Option
- **[[Text Parsing Patterns]]** - Parsing techniques using patterns
- **[[AoC Patterns MOC]]** - Pattern matching in competitive programming
- **[[rust-book-ch6]]** - Official Rust Book chapter on enums and pattern matching

---

## 🎓 Learning Resources

### **From This Workspace**

1. **Daily Study**: `daily_study/` - Pattern matching exercises
2. **AoC Examples**: `advent_of_code/aoc2015/Problem_Statements/` - Real-world usage
3. **Brackets Project**: `advanced_examples/Brackets_Basic/` - Practical pattern matching
4. **Mission 3+**: `missions/Mission3/` - Data structure pattern matching

### **Interactive Exercises**

```rust
// Try this in Rust Playground
fn classify(n: i32) {
    match n {
        0 => println!("Zero"),
        1..=10 => println!("Small"),
        11..=100 => println!("Medium"),
        101..=1000 => println!("Large"),
        _ => println!("Very large"),
    }
}

fn main() {
    for &n in &[0, 5, 50, 500, 5000] {
        classify(n);
    }
}
```

---

## ⚠️ Common Mistakes

| ❌ Wrong | ✅ Correct | Issue |
|---------|-----------|-------|
| Missing patterns | Use `_` for rest | Compiler error if not exhaustive |
| `if let` everywhere | Use `match` for multiple | Less clear code |
| Complex guards | Break into nested matches | Readability suffers |
| Cloning in patterns | Use `ref` patterns | Unnecessary copies |
| Ignoring `_` warnings | Explicitly use `_` | May hide issues |

---

## 🔗 Cross-References

**In this workspace**:

- Implementation: `advanced_examples/Brackets_Basic/src/lib.rs` (bracket matching)
- Advanced: `advanced_examples/Brackets_Ext/` (configurable patterns)
- Real-world: `advent_of_code/aoc2015/Problem_Statements/day12` (JSON parsing)
- Mission: `missions/Mission3/README.md` (detailed coverage)

---

**Status**: ✅ Active  
**Last Updated**: October 17, 2025  
**Topics Covered**: Basic, Intermediate, Advanced pattern matching  
**Difficulty**: Beginner → Advanced
