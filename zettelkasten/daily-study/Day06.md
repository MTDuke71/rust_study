# Day 6 - Pattern Matching

**Quick Reference Note**
*For full content, see: [[daily_study/rust_learning_week1_notes/Day06]]*

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
    Message::Quit => println!("Goodbye"),
    Message::Move { x, y } => println!("Move to {}, {}", x, y),
    Message::Write(text) => println!("Write: {}", text),
}
```

---

## Destructuring Patterns

### Tuple Destructuring
```rust
let point = (3, 5);
match point {
    (0, 0) => println!("Origin"),
    (0, y) => println!("On y-axis at {}", y),
    (x, 0) => println!("On x-axis at {}", x),
    (x, y) => println!("Point at ({}, {})", x, y),
}
```

### Struct Destructuring
```rust
struct Point { x: i32, y: i32 }

match Point { x: 0, y: 7 } {
    Point { x: 0, y } => println!("On y-axis at {}", y),
    Point { x, y: 0 } => println!("On x-axis at {}", x),
    Point { x, y } => println!("Point at ({}, {})", x, y),
}
```

---

## Advanced Patterns

### Guards (Conditions)
```rust
match number {
    x if x < 0 => println!("Negative"),
    x if x == 0 => println!("Zero"),
    x => println!("Positive: {}", x),
}
```

### Multiple Values
```rust
match (x, y) {
    (0, 0) => println!("Origin"),
    (0, _) => println!("On y-axis"),
    (_, 0) => println!("On x-axis"),
    _ => println!("Somewhere else"),
}
```

### Binding with @
```rust
match age {
    n @ 0..=12 => println!("Child aged {}", n),
    n @ 13..=19 => println!("Teenager aged {}", n),
    n => println!("Adult aged {}", n),
}
```

---

## if let and while let

### if let (Single Pattern)
```rust
let config_max = Some(3u8);

// Instead of:
match config_max {
    Some(max) => println!("Max: {}", max),
    _ => (),
}

// Use:
if let Some(max) = config_max {
    println!("Max: {}", max);
}
```

### while let (Pattern Loop)
```rust
let mut stack = Vec::new();
stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

---

## Quick Rules

- **match must be exhaustive** - cover all possible cases
- **Use `_`** for wildcard/default cases
- **Use `if let`** for single pattern matching
- **Use `while let`** for pattern-based loops
- **Guards (`if` conditions)** add extra logic to patterns
- **@ binding** captures matched values with names

---

## Quick Navigation

- **Full Details**: [[daily_study/rust_learning_week1_notes/Day06]]
- **Previous**: [[daily-study/Day05]]
- **Next**: [[daily-study/Day07]]
- **Week**: [[Week 1 Overview]]
- **MOC**: [[Rust Concepts MOC]]

---

*Tags: #pattern-matching #match #destructuring #quick-ref*