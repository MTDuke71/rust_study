# Day 7 · Week 1 Summary & Code Cheat Sheet

## 📋 Week 1 Review: Foundations Mastery

This week covered the **core pillars** of Rust programming that make it unique among systems languages. Every concept builds on ownership and memory safety.

### 🎯 Learning Journey Overview

| Day | Topic | Key Insight |
|-----|-------|-------------|
| **Day 1** | Ownership Basics | Values have exactly one owner, transfers prevent use-after-free |
| **Day 2** | Borrowing Rules | References allow access without ownership transfer |
| **Day 3** | Lifetimes | Compiler ensures references are always valid |
| **Day 4** | Mutability Patterns | Controlled mutation prevents data races |
| **Day 5** | Option & Result | Explicit handling of absence and errors |
| **Day 6** | Pattern Matching | Exhaustive matching ensures all cases handled |

---

## 🔧 Code Cheat Sheet

### **Day 1: Ownership Basics**

```rust
// Move semantics - ownership transfer
let s1 = String::from("hello");
let s2 = s1;  // s1 is now invalid, s2 owns the data
// println!("{}", s1);  // ❌ Compile error

// Stack vs Heap
let x = 5;          // Stack - Copy trait
let y = x;          // Both x and y are valid (copied)
let s = String::new();  // Heap - ownership transfer required

// Functions and ownership
fn take_ownership(s: String) {  // s comes into scope
    println!("{}", s);
} // s goes out of scope and `drop` is called

fn makes_copy(x: i32) {  // i32 implements Copy
    println!("{}", x);
} // x goes out of scope, but nothing special happens
```

### **Day 2: Borrowing Rules**

```rust
// Immutable borrowing
let s = String::from("hello");
let r1 = &s;  // Immutable reference
let r2 = &s;  // Multiple immutable refs OK
println!("{} and {}", r1, r2);

// Mutable borrowing
let mut s = String::from("hello");
let r1 = &mut s;  // Mutable reference
// let r2 = &s;   // ❌ Can't have immutable ref while mutable ref exists
r1.push_str(", world");

// Reference scope rules
let mut s = String::from("hello");
let r1 = &s;      // Immutable reference
let r2 = &s;      // Another immutable reference
println!("{} and {}", r1, r2);  // r1 and r2 go out of scope here
let r3 = &mut s;  // ✅ OK - no other references active
```

### **Day 3: Lifetimes**

```rust
// Lifetime annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct lifetimes
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime elision (compiler infers)
fn first_word(s: &str) -> &str {  // Inferred as: fn first_word<'a>(s: &'a str) -> &'a str
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

### **Day 4: Mutability Patterns**

```rust
// Basic mutability
let mut x = 5;
x = 6;  // ✅ OK

// Mutable references
let mut s = String::from("hello");
let r = &mut s;
r.push_str(", world");

// Interior mutability with Cell/RefCell
use std::cell::{Cell, RefCell};

let x = Cell::new(5);
x.set(10);  // Can mutate even though x is not mut

let y = RefCell::new(5);
*y.borrow_mut() += 1;  // Runtime borrow checking

// Mutable reference patterns
fn modify_string(s: &mut String) {
    s.push_str(" modified");
}

let mut text = String::from("original");
modify_string(&mut text);
```

### **Day 5: Option & Result**

```rust
// Option<T> - handling absence
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some("Alice".to_string())
    } else {
        None
    }
}

// Option methods
let user = find_user(1);
let name = user.unwrap_or("Unknown".to_string());
let name = user.map(|s| s.to_uppercase()).unwrap_or_default();

// Result<T, E> - handling errors
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Error propagation with ?
fn calculate() -> Result<f64, String> {
    let result = divide(10.0, 2.0)?;  // Propagates error if Err
    Ok(result * 2.0)
}

// Result methods
let result = divide(10.0, 2.0)
    .map(|x| x * 2.0)
    .unwrap_or(0.0);
```

### **Day 6: Pattern Matching**

```rust
// match expressions
fn process_option(opt: Option<i32>) -> i32 {
    match opt {
        Some(x) => x * 2,
        None => 0,
    }
}

// match with guards
fn categorize_number(x: i32) -> &'static str {
    match x {
        n if n < 0 => "negative",
        0 => "zero",
        1..=10 => "small positive",
        _ => "large positive",
    }
}

// if let - concise pattern matching
let some_value = Some(5);
if let Some(x) = some_value {
    println!("Got value: {}", x);
}

// while let
let mut stack = vec![1, 2, 3];
while let Some(value) = stack.pop() {
    println!("Popped: {}", value);
}

// Destructuring structs and tuples
struct Point { x: i32, y: i32 }
let p = Point { x: 0, y: 7 };
let Point { x, y } = p;  // Destructure

let tuple = (1, 2, 3);
let (a, b, c) = tuple;  // Destructure tuple
```

---

## 🎯 Key Mental Models

### **Ownership Mental Model**
```
┌─────────────┐    move     ┌─────────────┐
│   Value     │ ────────►   │  New Owner  │
│ (becomes    │             │             │
│  invalid)   │             │   (valid)   │
└─────────────┘             └─────────────┘
```

### **Borrowing Mental Model**
```
┌─────────────┐    &/&mut   ┌─────────────┐
│   Owner     │ ────────►   │  Borrower   │
│  (retains   │             │ (temporary  │
│ ownership)  │             │  access)    │
└─────────────┘             └─────────────┘
```

## 🔥 Common Patterns for Competitive Programming

```rust
// Safe array access with Option
let vec = vec![1, 2, 3];
let item = vec.get(5).unwrap_or(&0);

// Error handling in parsers
fn parse_input(line: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    line.split_whitespace()
        .map(|s| s.parse())
        .collect()
}

// Pattern matching for state machines
enum State {
    Start,
    Processing(i32),
    Done(String),
}

fn process_state(state: State) -> State {
    match state {
        State::Start => State::Processing(0),
        State::Processing(n) if n < 10 => State::Processing(n + 1),
        State::Processing(n) => State::Done(format!("Processed {}", n)),
        State::Done(s) => State::Done(s),
    }
}

// Option chaining for safe operations
let result = Some("42")
    .and_then(|s| s.parse().ok())
    .map(|n: i32| n * 2)
    .filter(|&n| n > 50);
```

## 🧠 Memory: The Big Picture

By Day 7, you've mastered:
- **Memory Safety**: No null pointer dereferences, no use-after-free
- **Thread Safety**: Borrowing rules prevent data races at compile time
- **Zero-Cost Abstractions**: `Option<T>` compiles to the same code as C unions
- **Explicit Control**: Every memory allocation and deallocation is predictable

## 🚀 Ready for Week 2: Collections

Week 1 gave you the **ownership discipline**. Week 2 will show you how Rust's collections (`Vec<T>`, `String`, `HashMap<K,V>`) leverage these ownership rules to provide both safety and performance.

### Week 2 Preview
```rust
// You'll understand why this works:
let mut vec = Vec::new();
vec.push(String::from("hello"));
let item = vec.pop().unwrap();  // Moves ownership back out

// And why this is safe:
let slice = &vec[1..3];  // Borrowing without copying
```

**Mastery Check**: If you can explain why each code example above compiles (or doesn't), you're ready for Week 2! 🎯
