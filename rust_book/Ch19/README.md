# Chapter 19: Patterns and Matching

## 🔗 Zettelkasten Links
- **Overview**: [[zettelkasten/rust_book/rust-book-ch19]]
- **Previous**: [[zettelkasten/rust_book/rust-book-ch18]]
- **Next**: [[zettelkasten/rust_book/rust-book-ch20]]
- **Missions**: [[mission-3]] - Binary search variants with pattern matching | [[mission-8]] - Graph algorithms with destructuring
- **Daily Study**: [[daily-study/Day27]] | [[daily-study/Day28]]
- **Book MOC**: [[rust-book]]

## 📚 Overview

Chapter 19 explores Rust's powerful pattern matching system. Patterns are a special syntax for matching against the structure of types, making your code more expressive and safer. This chapter covers where patterns can be used, the concept of refutability, and comprehensive pattern syntax.

**Official Reference**: https://doc.rust-lang.org/book/ch19-00-patterns.html

---

## 🎯 Learning Objectives

By completing this chapter, you will understand:

1. **Pattern Locations** - All the places in Rust where patterns can be used (match, if let, while let, for, function parameters)
2. **Refutability** - The difference between refutable and irrefutable patterns and when to use each
3. **Pattern Syntax** - Comprehensive syntax including literals, destructuring, wildcards, guards, and @ bindings
4. **Practical Applications** - Using patterns to write cleaner, safer code in real-world scenarios
5. **Advanced Techniques** - Combining multiple pattern features for expressive data handling

**Integration Points**: This chapter connects to:
- **[[mission-3]]** - Pattern matching in binary search edge cases
- **[[mission-8]]** - Destructuring graph traversal results
- **[[zettelkasten/pattern-matching-locations]]** - Comprehensive pattern usage guide
- **[[zettelkasten/refutable-vs-irrefutable-patterns]]** - Refutability deep dive
- **[[zettelkasten/pattern-syntax-comprehensive]]** - Complete pattern syntax reference

---

## 🎯 Chapter Concepts

### 19.1. All the Places Patterns Can Be Used

**Official Definition**: Patterns are used throughout Rust in `match` expressions, `if let`, `while let`, `for` loops, function parameters, and `let` statements.

**Practical Understanding**: Patterns aren't just for `match` - they're a fundamental part of Rust's syntax that makes code more expressive and safe in many contexts.

**Key Examples**:

```rust
// 1. match arms - most common pattern usage
fn describe_point(point: (i32, i32)) -> String {
    match point {
        (0, 0) => "origin".to_string(),
        (x, 0) => format!("on x-axis at {}", x),
        (0, y) => format!("on y-axis at {}", y),
        (x, y) => format!("at ({}, {})", x, y),
    }
}

// 2. if let - conditional destructuring
fn process_optional(value: Option<i32>) {
    if let Some(x) = value {
        println!("Got value: {}", x);
    } else {
        println!("Got None");
    }
}

// 3. while let - loop while pattern matches
fn drain_values(stack: &mut Vec<i32>) {
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}

// 4. for loops - iterate with destructuring
fn iterate_coordinates(points: Vec<(i32, i32)>) {
    for (x, y) in points {
        println!("Point: ({}, {})", x, y);
    }
}

// 5. let statements - always use patterns!
fn demonstrate_let_patterns() {
    let x = 5;  // Simple pattern
    let (a, b, c) = (1, 2, 3);  // Tuple destructuring
    let Point { x, y } = Point { x: 10, y: 20 };  // Struct destructuring
}

// 6. Function parameters - patterns in signatures
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
```

**Common Mistakes**:
- **Mistake 1**: Not recognizing that `let x = 5` uses a pattern
- **Mistake 2**: Using `match` when `if let` would be clearer for single case
- **Mistake 3**: Forgetting that function parameters are patterns

**Integration**: Pattern locations used extensively in [[mission-8]] for graph algorithm results.

---

### 19.2. Refutability: Whether a Pattern Might Fail to Match

**Official Definition**: Patterns come in two forms: **refutable** (can fail to match) and **irrefutable** (will always match). Function parameters, `let` statements, and `for` loops accept only irrefutable patterns. `match` arms (except the last) require refutable patterns.

**Practical Understanding**: 
- **Irrefutable**: Always matches, can't fail (e.g., `let x = 5`, `let (a, b) = tuple`)
- **Refutable**: Might fail to match (e.g., `if let Some(x) = value`, `match` arms)

**Key Examples**:

```rust
// Irrefutable patterns - always match
fn irrefutable_examples() {
    let x = 5;  // Always matches
    let (a, b) = (1, 2);  // Always matches (tuple has 2 elements)
    
    // Function parameters must be irrefutable
    fn takes_tuple((x, y): (i32, i32)) {  // Always matches
        println!("{}, {}", x, y);
    }
    
    // for loops use irrefutable patterns
    for (key, value) in vec![("a", 1), ("b", 2)] {
        println!("{}: {}", key, value);
    }
}

// Refutable patterns - might not match
fn refutable_examples() {
    let numbers = vec![1, 2, 3];
    
    // if let with refutable pattern
    if let Some(first) = numbers.first() {
        println!("First: {}", first);
    }
    
    // while let with refutable pattern
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    
    // match arms (except exhaustive catch-all)
    match numbers.get(0) {
        Some(n) => println!("Found: {}", n),  // Refutable
        None => println!("Not found"),        // Makes match exhaustive
    }
}

// Compiler errors - wrong refutability
fn refutability_errors() {
    // ❌ ERROR: refutable pattern in irrefutable position
    // let Some(x) = some_option_value;
    
    // ✅ CORRECT: Use if let for refutable patterns
    let some_option_value = Some(5);
    if let Some(x) = some_option_value {
        println!("{}", x);
    }
    
    // ❌ ERROR: irrefutable pattern in match arm
    // match some_value {
    //     x => println!("{}", x),  // This is irrefutable, should be in if let
    // }
    
    // ✅ CORRECT: Use if let for single irrefutable case
    let some_value = 5;
    if some_value > 0 {
        println!("Positive: {}", some_value);
    }
}
```

**Common Mistakes**:
- **Mistake 1**: Using `let Some(x) = value` - compiler error, use `if let`
- **Mistake 2**: All match arms being irrefutable - use `if let` instead
- **Mistake 3**: Not understanding why compiler rejects certain patterns

**Integration**: Refutability concepts critical for [[mission-3]] Option handling in search algorithms.

---

### 19.3. Pattern Syntax

**Official Definition**: Patterns can match literals, destructure arrays/tuples/enums/structs, use wildcards, add match guards, and create @ bindings.

**Practical Understanding**: Rust's pattern syntax is incredibly expressive, allowing you to:
- Match specific values directly
- Destructure complex data structures
- Ignore parts you don't care about
- Add conditional logic with guards
- Bind matched values to variables

**Key Examples**:

```rust
// 1. Matching literals
fn match_literals(x: i32) {
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything else"),
    }
}

// 2. Matching named variables
fn match_named_variables() {
    let x = Some(5);
    let y = 10;
    
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {}", y),  // Shadows outer y
        _ => println!("Default case, x = {:?}", x),
    }
    
    println!("Outer y = {}", y);  // Still 10
}

// 3. Multiple patterns with |
fn match_multiple_patterns(x: i32) {
    match x {
        1 | 2 => println!("one or two"),
        3 | 4 | 5 => println!("three through five"),
        _ => println!("something else"),
    }
}

// 4. Matching ranges with ..=
fn match_ranges(x: char) {
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        '0'..='9' => println!("digit"),
        _ => println!("something else"),
    }
}

// 5. Destructuring structs
struct Point {
    x: i32,
    y: i32,
}

fn destructure_structs() {
    let p = Point { x: 0, y: 7 };
    
    // Full destructuring
    let Point { x, y } = p;
    println!("x: {}, y: {}", x, y);
    
    // Partial destructuring
    let Point { x, .. } = p;  // Ignore y
    
    // Destructuring with match
    match p {
        Point { x: 0, y } => println!("On y-axis at {}", y),
        Point { x, y: 0 } => println!("On x-axis at {}", x),
        Point { x, y } => println!("At ({}, {})", x, y),
    }
}

// 6. Destructuring enums
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destructure_enums(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text: {}", text),
        Message::ChangeColor(r, g, b) => println!("RGB: ({}, {}, {})", r, g, b),
    }
}

// 7. Destructuring nested structures
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum NestedMessage {
    ChangeColor(Color),
}

fn destructure_nested() {
    let msg = NestedMessage::ChangeColor(Color::Hsv(0, 160, 255));
    
    match msg {
        NestedMessage::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("RGB: ({}, {}, {})", r, g, b);
        }
        NestedMessage::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("HSV: ({}, {}, {})", h, s, v);
        }
    }
}

// 8. Destructuring arrays and tuples
fn destructure_arrays_tuples() {
    // Tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    
    // Arrays with .. rest pattern
    let numbers = [1, 2, 3, 4, 5];
    match numbers {
        [first, .., last] => println!("First: {}, Last: {}", first, last),
    }
}

// 9. Ignoring values with _ and ..
fn ignore_values() {
    // Ignore entire value
    fn foo(_: i32, y: i32) {
        println!("y: {}", y);
    }
    
    // Ignore parts of a value
    let (x, _, z) = (1, 2, 3);
    
    // Ignore remaining parts
    let Point { x, .. } = Point { x: 1, y: 2 };
    
    // Ignore with underscore prefix (unused variable warning suppression)
    let _unused_variable = 5;
}

// 10. Match guards - extra if condition
fn match_with_guards(num: Option<i32>) {
    match num {
        Some(x) if x % 2 == 0 => println!("Even number: {}", x),
        Some(x) => println!("Odd number: {}", x),
        None => println!("None"),
    }
}

// 11. @ bindings - bind and test simultaneously
fn at_bindings() {
    enum Message2 {
        Hello { id: i32 },
    }
    
    let msg = Message2::Hello { id: 5 };
    
    match msg {
        Message2::Hello {
            id: id_variable @ 3..=7,  // Bind to id_variable AND test range
        } => println!("Found id in range: {}", id_variable),
        Message2::Hello { id: 10..=12 } => {
            println!("Found id in another range (but can't use value)");
        }
        Message2::Hello { id } => println!("Found some other id: {}", id),
    }
}
```

**Common Mistakes**:
- **Mistake 1**: Forgetting `..` to ignore remaining struct fields
- **Mistake 2**: Not using `@ bindings` when you need both test and value
- **Mistake 3**: Confusing `..` (rest) with `_` (ignore single)
- **Mistake 4**: Forgetting that match guards apply to the entire pattern with `|`

**Integration**: Comprehensive pattern syntax used throughout [[mission-8]] graph traversal and [[zettelkasten/pattern-syntax-comprehensive]].

---

## 🧪 Exercises and Practice

### **Exercise 1: Pattern Location Mastery**
Create a function that demonstrates all six pattern locations with a single data structure.

**Goal**: Practice using patterns in match, if let, while let, for, let, and function parameters.

```rust
// See examples/ch19_1_pattern_locations.rs
cargo run --example ch19_1_pattern_locations
```

### **Exercise 2: Refutability Understanding**
Write code that correctly uses refutable and irrefutable patterns, demonstrating compiler errors if used incorrectly.

**Goal**: Understand when to use each pattern type and why.

```rust
// See examples/ch19_2_refutability.rs
cargo run --example ch19_2_refutability
```

### **Exercise 3: Pattern Syntax Showcase**
Implement a parser for a simple message format using all pattern syntax features.

**Goal**: Combine literals, destructuring, guards, ranges, and @ bindings.

```rust
// See examples/ch19_3_pattern_syntax.rs
cargo run --example ch19_3_pattern_syntax
```

---

## 🔗 Cross-References

### **Prerequisites**
- **[[rust_book/Ch6]]**: Enums and pattern matching basics
- **[[rust_book/Ch8]]**: Collections that pattern matching works with
- **[[rust_book/Ch9]]**: Result and Option that use patterns

### **Applications**
- **[[mission-3]]**: Binary search variants with pattern matching edge cases
- **[[mission-8]]**: Graph traversal with destructuring and match expressions
- **[[advanced_examples/Brackets_Basic]]**: State validation with patterns

### **Reinforcement**
- **[[zettelkasten/pattern-matching-locations]]**: Complete guide to pattern locations
- **[[zettelkasten/refutable-vs-irrefutable-patterns]]**: Deep dive on refutability
- **[[zettelkasten/pattern-syntax-comprehensive]]**: Comprehensive syntax reference
- **[[daily-study/Day27]]**: Pattern matching in practice
- **[[daily-study/Day28]]**: Advanced destructuring techniques

---

## 📚 Examples

Run examples to see concepts in action:

```bash
# Pattern locations demonstration
cargo run --example ch19_1_pattern_locations

# Refutability examples
cargo run --example ch19_2_refutability

# Pattern syntax showcase
cargo run --example ch19_3_pattern_syntax

# Run all tests
cargo test
```

---

## 🎯 Integration with Learning System

### **Mission Integration Matrix**

| Chapter Concept | Mission Application | Specific Usage | Validation |
|-----------------|-------------------|----------------|------------|
| Pattern Matching | [[mission-3]] REQ-2 | Binary search edge case handling | `test_edge_cases_with_patterns` |
| Destructuring | [[mission-8]] REQ-1 | Graph node unpacking | `test_node_destructuring` |
| Match Guards | [[mission-8]] REQ-3 | Conditional traversal | `test_conditional_traversal` |

### **AoC Applications**
- **2024 Day 4**: Grid pattern matching with destructuring
- **2024 Day 5**: Rule parsing with enum patterns
- **2025 Day 7-9**: Pattern matching for puzzle solutions

### **Zettelkasten Integration**
Create these concept pages:
- **[[pattern-matching-locations]]**: Sunday, December 7
- **[[refutable-vs-irrefutable-patterns]]**: Monday, December 8
- **[[pattern-syntax-comprehensive]]**: Tuesday, December 9

---

## ✅ Chapter Completion Checklist

- [ ] Read all three sections (19.1, 19.2, 19.3)
- [ ] Run all examples and understand output
- [ ] Complete all exercises in `src/exercises.rs`
- [ ] Create zettelkasten pages for each section
- [ ] Solve corresponding AoC problems (Days 7-9)
- [ ] All tests passing: `cargo test`
- [ ] All clippy checks passing: `cargo clippy -- -D warnings`
- [ ] Chapter summary created: `CHAPTER_COMPLETE.md`

**Creation Documentation**: See [[CREATION_SUMMARY]] for complete package setup details, quality verification, and testing infrastructure.

---

*Run all examples*: `cargo run --example [name]`  
*Run all tests*: `cargo test`  
*Check quality*: `cargo clippy -- -D warnings`

*Tags: #rust-book #ch19 #patterns #matching #destructuring #refutability*

*Links: [[../../zettelkasten/rust_book/rust-book-ch19]] | [[../../zettelkasten/zettel-index]]*
