# Day 6 - Pattern Matching

**Quick Reference Note**
*For full content, see: [[daily_study/rust_learning_week1_notes/Day6]]*

---

## Core Concepts

### match Expression

**Exhaustive matching** - must cover all cases:
```rust
match value {
    pattern1 => expression1,
    pattern2 => expression2,
    _ => default_expression,  // Wildcard for "everything else"
}
```

### Basic Patterns

**Literal Matching:**
```rust
match x {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("other"),
}
```

**Range Matching:**
```rust
match age {
    0..=12 => println!("child"),
    13..=19 => println!("teenager"),
    20..=64 => println!("adult"),
    65.. => println!("senior"),
}
```

**Enum Matching:**
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

match msg {
    Message::Quit => process_quit(),
    Message::Move { x, y } => process_move(x, y),
    Message::Write(text) => process_write(&text),
}
```

---

## Destructuring

### Tuples
```rust
let point = (3, 5);
match point {
    (0, 0) => println!("origin"),
    (0, y) => println!("y-axis at {}", y),
    (x, 0) => println!("x-axis at {}", x),
    (x, y) => println!("point ({}, {})", x, y),
}
```

### Structs
```rust
struct Point { x: i32, y: i32 }

let p = Point { x: 0, y: 7 };
match p {
    Point { x: 0, y } => println!("on y-axis: {}", y),
    Point { x, y: 0 } => println!("on x-axis: {}", x),
    Point { x, y } => println!("({}, {})", x, y),
}
```

### Option and Result
```rust
match maybe_value {
    Some(x) => println!("got {}", x),
    None => println!("nothing"),
}

match operation_result {
    Ok(val) => println!("success: {}", val),
    Err(e) => eprintln!("error: {}", e),
}
```

---

## Advanced Patterns

### Guards (Conditions)
```rust
match num {
    x if x < 0 => println!("negative: {}", x),
    x if x % 2 == 0 => println!("even: {}", x),
    x => println!("odd positive: {}", x),
}
```

### @ Bindings (Bind & Test)
```rust
match msg {
    Message::Hello { id: id_var @ 3..=7 } => {
        println!("Found id in range: {}", id_var)
    }
    Message::Hello { id } => {
        println!("Other id: {}", id)
    }
}
```

### Ignoring Values
```rust
// Ignore specific values
let (x, _, z) = (1, 2, 3);  // Ignore middle

// Ignore rest
match tuple {
    (first, ..) => println!("First: {}", first),
}
```

---

## Shorthand Syntax

### if let (Single Pattern)
```rust
// Instead of:
match maybe_value {
    Some(x) => println!("{}", x),
    None => {}
}

// Use:
if let Some(x) = maybe_value {
    println!("{}", x);
}
```

### while let (Loop Pattern)
```rust
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

### let else (Refutable Pattern)
```rust
let Some(value) = maybe_value else {
    return; // Handle None case
};
// Use value here
```

---

## Common Patterns

### Multiple Patterns (OR)
```rust
match character {
    'a' | 'e' | 'i' | 'o' | 'u' => println!("vowel"),
    _ => println!("consonant"),
}
```

### Nested Patterns
```rust
match nested_option {
    Some(Some(x)) => println!("nested value: {}", x),
    Some(None) => println!("inner none"),
    None => println!("outer none"),
}
```

### Reference Patterns
```rust
let x = &Some(5);
match x {
    Some(val) => println!("{}", val),  // val is &i32
    None => println!("none"),
}

// Or dereference
match *x {
    Some(val) => println!("{}", val),  // val is i32
    None => println!("none"),
}
```

---

## Quick Rules

- **match** is an expression (returns a value)
- **Must be exhaustive** (cover all cases or use _)
- **Use if let** for single pattern
- **Use while let** for pattern-based loops
- **Guards add conditions** with if
- **@ binds and tests** simultaneously

---

## Pattern Types Quick Reference

| Pattern | Example | Use Case |
|---------|---------|----------|
| Literal | `1 => ...` | Match exact value |
| Range | `0..=10 => ...` | Match range |
| Wildcard | `_ => ...` | Match anything |
| Variable | `x => ...` | Bind to variable |
| Enum | `Some(x) => ...` | Match enum variant |
| Tuple | `(x, y) => ...` | Destructure tuple |
| Struct | `Point { x, y } => ...` | Destructure struct |
| Reference | `&val => ...` | Match reference |
| Guard | `x if x > 10 => ...` | Add condition |
| Or | `'a' \| 'e' => ...` | Multiple patterns |
| Binding | `val @ 1..=5 => ...` | Bind and test |

---

## Quick Navigation

- **Full Details**: [[daily_study/rust_learning_week1_notes/Day6]]
- **Previous**: [[Day 5 - Option and Result]]
- **Next**: [[Day 7 - Week 1 Summary]]
- **Week**: [[Week 1 Overview]]
- **MOC**: [[Rust Concepts MOC]]
- **Rust Book**: Chapter 6.2, Chapter 18

---

*Tags: #pattern-matching #match #destructuring #if-let #quick-ref*
