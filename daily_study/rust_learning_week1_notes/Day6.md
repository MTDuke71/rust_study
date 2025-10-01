# Day 6 · Pattern Matching

## 🔗 Zettelkasten Links
- **Previous**: [[Day 5 - Option and Result]] - Error handling types
- **Next**: [[Day 7 - Week 1 Summary]] - Week foundations review
- **Concept**: [[Rust Concepts MOC]] - Pattern matching system
- **Rust Book**: [[Chapter 6.2 - Match]] - The match control flow operator
- **Rust Book**: [[Chapter 18 - Patterns]] - All pattern forms
- **Week Summary**: [[Day 7 - Week 1 Summary]] - Foundations review

## Overview
Pattern matching is one of Rust's most powerful features, providing a way to destructure and match against the shape of data. It's both a control flow mechanism and a data extraction tool that enforces exhaustiveness and safety at compile time.

## Core Concepts

### 1. `match` Expression - Exhaustive Pattern Matching
- **Exhaustive**: Must cover all possible cases or use a wildcard
- **Expression**: Returns a value (unlike statements)
- **No fallthrough**: Each arm is independent

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) -> String {
    match msg {
        Message::Quit => "Quitting application".to_string(),
        Message::Move { x, y } => format!("Moving to ({}, {})", x, y),
        Message::Write(text) => format!("Writing: {}", text),
        Message::ChangeColor(r, g, b) => format!("Color: rgb({}, {}, {})", r, g, b),
    }
}
```

### 2. `if let` - Single Pattern Matching
Syntactic sugar for matching one pattern and ignoring the rest:

```rust
// Instead of:
match some_option {
    Some(value) => println!("Got: {}", value),
    None => {}
}

// Use if let:
if let Some(value) = some_option {
    println!("Got: {}", value);
}

// Can chain with else
if let Some(value) = some_option {
    println!("Got: {}", value);
} else {
    println!("Nothing found");
}
```

### 3. `while let` - Pattern-Based Loops
Continue looping while a pattern matches:

```rust
let mut stack = vec![1, 2, 3];

// Pop elements while Some
while let Some(top) = stack.pop() {
    println!("Popped: {}", top);
}
```

## Pattern Types and Destructuring

### Literal Patterns
```rust
match x {
    1 => println!("one"),
    2 => println!("two"), 
    3 => println!("three"),
    _ => println!("anything else"),
}
```

### Range Patterns
```rust
match age {
    0..=12 => println!("child"),
    13..=19 => println!("teenager"),
    20..=64 => println!("adult"),
    65.. => println!("senior"),
}
```

### Tuple Destructuring
```rust
let point = (3, 5);
match point {
    (0, 0) => println!("origin"),
    (0, y) => println!("on y-axis at {}", y),
    (x, 0) => println!("on x-axis at {}", x),
    (x, y) => println!("point at ({}, {})", x, y),
}
```

### Struct Destructuring
```rust
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };
match p {
    Point { x: 0, y } => println!("on y-axis at {}", y),
    Point { x, y: 0 } => println!("on x-axis at {}", x),
    Point { x, y } => println!("point at ({}, {})", x, y),
}

// Shorthand when variable names match field names
let Point { x, y } = p;
```

### Array/Slice Patterns
```rust
let arr = [1, 2, 3];
match arr {
    [1, 2, 3] => println!("exact match"),
    [1, rest @ ..] => println!("starts with 1, rest: {:?}", rest),
    [.., 3] => println!("ends with 3"),
    _ => println!("something else"),
}
```

## Advanced Pattern Features

### Guards
Add additional conditions to patterns:

```rust
let num = Some(4);
match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("greater or equal to five: {}", x),
    None => println!("no value"),
}
```

### `@` Bindings
Bind values while pattern matching:

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };
match msg {
    Message::Hello { id: id_variable @ 3..=7 } => {
        println!("Found an id in range: {}", id_variable)
    }
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    }
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    }
}
```

### Ignoring Values
```rust
let (a, _, c) = (1, 2, 3);  // Ignore middle value

match some_tuple {
    (first, ..) => println!("First: {}", first),  // Ignore rest
    (.., last) => println!("Last: {}", last),     // Ignore beginning
}
```

## Real-World Applications

### Error Handling with `Result`
```rust
use std::fs::File;
use std::io::Read;

fn read_file_content(filename: &str) -> Result<String, std::io::Error> {
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            match file.read_to_string(&mut content) {
                Ok(_) => Ok(content),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

// Using the result
match read_file_content("config.txt") {
    Ok(content) => println!("File content: {}", content),
    Err(error) => eprintln!("Error reading file: {}", error),
}
```

### State Machine Implementation
```rust
#[derive(Debug)]
enum State {
    Idle,
    Running { task_id: u32 },
    Paused { task_id: u32, remaining: u32 },
    Completed,
}

impl State {
    fn transition(self, event: Event) -> State {
        match (self, event) {
            (State::Idle, Event::Start(id)) => State::Running { task_id: id },
            (State::Running { task_id }, Event::Pause(remaining)) => {
                State::Paused { task_id, remaining }
            }
            (State::Paused { task_id, .. }, Event::Resume) => {
                State::Running { task_id }
            }
            (State::Running { .. }, Event::Complete) => State::Completed,
            (current, _) => {
                println!("Invalid transition from {:?}", current);
                current
            }
        }
    }
}
```

## Connection to Your Workspace Projects

### Mission1 (Stack) - Pattern Matching in Stack Operations
```rust
impl<T> Stack<T> {
    pub fn pop(&mut self) -> Option<T> {
        match self.items.pop() {
            Some(item) => {
                println!("Popped: item from stack");
                Some(item)
            }
            None => {
                println!("Stack is empty");
                None
            }
        }
    }
}
```

### Mission2 (Queue) - Pattern Matching for Queue States
```rust
// Pattern matching for queue capacity checking
match (self.size, self.capacity) {
    (size, cap) if size == cap => Err(EnqueueError::Full(item)),
    _ => {
        // Safe to enqueue
        self.enqueue_internal(item);
        Ok(())
    }
}
```

### Brackets Validation - Complex Pattern Matching
```rust
fn validate_brackets(input: &str) -> ValidationResult {
    let mut stack = Stack::new();
    
    for ch in input.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => match stack.pop() {
                Some('(') => continue,
                Some(_) | None => return ValidationResult::Invalid,
            },
            ']' => match stack.pop() {
                Some('[') => continue,
                Some(_) | None => return ValidationResult::Invalid,
            },
            '}' => match stack.pop() {
                Some('{') => continue,
                Some(_) | None => return ValidationResult::Invalid,
            },
            _ => continue, // Ignore non-bracket characters
        }
    }
    
    match stack.is_empty() {
        true => ValidationResult::Valid,
        false => ValidationResult::Invalid,
    }
}
```

## Best Practices

### 1. Prefer `match` over `if let` for Multiple Cases
```rust
// Good - clear and exhaustive
match result {
    Ok(value) => handle_success(value),
    Err(ParseError::InvalidFormat) => handle_format_error(),
    Err(ParseError::OutOfRange) => handle_range_error(),
}

// Less ideal - multiple if let statements
if let Ok(value) = result {
    handle_success(value);
} else if let Err(ParseError::InvalidFormat) = result {
    handle_format_error();
} // etc...
```

### 2. Use Guards Sparingly
```rust
// Good - simple guard
match x {
    n if n < 0 => println!("negative"),
    n => println!("non-negative: {}", n),
}

// Avoid complex logic in guards - extract to functions instead
match user {
    User { role, .. } if is_admin_with_permissions(&role) => grant_access(),
    _ => deny_access(),
}
```

### 3. Be Explicit About Ignored Fields
```rust
// Good - explicit about ignored fields
match config {
    Config { database_url, .. } => connect_to_database(database_url),
}

// Less clear about what's being ignored
match config {
    Config { database_url, cache_size: _, log_level: _, .. } => {
        connect_to_database(database_url)
    }
}
```

## Performance Considerations

- **Zero-cost abstractions**: Pattern matching compiles to efficient branching code
- **Exhaustiveness checking**: Happens at compile time, no runtime overhead
- **Move semantics**: Be aware of when values are moved vs borrowed in patterns

```rust
// Moves the value
match opt {
    Some(value) => consume(value),  // value is moved
    None => {}
}

// Borrows the value
match &opt {
    Some(value) => use_reference(value),  // value is &T
    None => {}
}
```

## Testing Pattern Matching

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_processing() {
        let msg = Message::Move { x: 10, y: 20 };
        let result = process_message(msg);
        assert_eq!(result, "Moving to (10, 20)");
    }

    #[test]
    fn test_option_handling() {
        let some_val = Some(42);
        let none_val: Option<i32> = None;
        
        match some_val {
            Some(42) => {}, // Expected
            _ => panic!("Should match Some(42)"),
        }
        
        match none_val {
            None => {}, // Expected
            _ => panic!("Should be None"),
        }
    }
}
```

## Takeaways

1. **Exhaustiveness**: The compiler ensures you handle all cases, preventing bugs
2. **Expressiveness**: Complex data structures can be destructured elegantly
3. **Performance**: Zero-cost abstractions with compile-time optimizations
4. **Safety**: No null pointer dereferences or unhandled cases
5. **Readability**: Intent is clear and explicit

Pattern matching is central to idiomatic Rust and appears throughout your workspace projects - from simple Option handling to complex state machines in data structures. Master this concept and you'll write more robust, readable Rust code.

---

*Links: [[Day 5 - Option and Result]] | [[Day 7 - Week 1 Summary]] | [[Rust Concepts MOC]]*
*Tags: #pattern-matching #match #daily-study #rust-book #chapter6 #chapter18 #foundation*
