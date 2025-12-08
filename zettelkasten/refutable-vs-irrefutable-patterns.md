# Refutable vs Irrefutable Patterns

**Tags:** #rust #patterns #match #if-let #while-let #pattern-matching #rust-book-ch19  
**Created:** 2025-12-08  
**Related:** [[rust-book-ch19]], [[pattern-matching]], [[if-let-expressions]], [[while-let-loops]], [[match-expressions]]

---

## 🎯 Core Concept

Rust patterns come in two flavors based on whether they can **fail to match**:

- **Irrefutable**: Patterns that will **always match** for any possible value
- **Refutable**: Patterns that can **fail to match** for some possible values

Understanding this distinction is crucial because **Rust enforces where each type can be used** at compile time.

---

## 📊 Quick Reference

| Context | Accepts | Examples |
|---------|---------|----------|
| `let` bindings | **Irrefutable only** | `let x = 5;` `let (a, b) = tuple;` |
| `function parameters` | **Irrefutable only** | `fn foo(x: i32)` `fn bar((a, b): (i32, i32))` |
| `for` loops | **Irrefutable only** | `for (key, value) in map` |
| `match` arms | **Refutable** (except last arm) | `Some(x) => ...` `Ok(val) => ...` |
| `if let` | **Refutable** | `if let Some(x) = opt` |
| `while let` | **Refutable** | `while let Some(x) = iter.next()` |

---

## 🔒 Irrefutable Patterns

**Definition**: Patterns that match **any value** passed to them - they cannot fail.

### Examples of Irrefutable Patterns

```rust
// 1. Simple variable binding - always succeeds
let x = 5;

// 2. Wildcard - matches anything
let _ = some_value;

// 3. Tuple destructuring - always matches if types align
let (a, b, c) = (1, 2, 3);

// 4. Struct destructuring - always matches
struct Point { x: i32, y: i32 }
let Point { x, y } = point;

// 5. Reference pattern - always matches
let &value = &42;

// 6. Mutable binding - always matches
let mut x = 5;
```

### Where Irrefutable Patterns Are Required

#### `let` statements
```rust
// ✅ Valid - irrefutable pattern
let x = 5;
let (a, b) = (1, 2);
let Point { x, y } = point;

// ❌ Compile error - refutable pattern in let
let Some(x) = some_option; 
// Error: refutable pattern in local binding
```

#### Function parameters
```rust
// ✅ Valid - irrefutable patterns
fn print_coordinates((x, y): (i32, i32)) {
    println!("({}, {})", x, y);
}

fn process_point(Point { x, y }: Point) {
    println!("x: {}, y: {}", x, y);
}

// ❌ Compile error - refutable pattern
fn get_value(Some(x): Option<i32>) -> i32 {
//           ^^^^^^^ Error: refutable pattern in parameter
    x
}
```

#### `for` loops
```rust
let map = HashMap::new();

// ✅ Valid - irrefutable pattern
for (key, value) in &map {
    println!("{}: {}", key, value);
}

// ✅ Valid - enumerate returns (index, item) tuples
for (i, val) in vec.iter().enumerate() {
    println!("[{}] = {}", i, val);
}

// ❌ Won't compile - refutable pattern
for Some(x) in vector_of_options {
//  ^^^^^^^ Error: refutable pattern in for loop
    println!("{}", x);
}
```

---

## ❓ Refutable Patterns

**Definition**: Patterns that can **fail to match** some values.

### Examples of Refutable Patterns

```rust
// 1. Option patterns - might be None
Some(x)
None

// 2. Result patterns - might be Err
Ok(value)
Err(e)

// 3. Specific literal values - might not match
42
"hello"
'a'

// 4. Range patterns - might be outside range
1..=100
'a'..='z'

// 5. Enum variant patterns - might be different variant
Color::Red
Message::Quit

// 6. Multiple patterns with OR - might match none of them
Some(1) | Some(2) | Some(3)
```

### Where Refutable Patterns Are Used

#### `match` expressions
```rust
// ✅ Valid - refutable patterns in match arms
match some_option {
    Some(x) => println!("Got: {}", x),
    None => println!("Nothing"),
}

match result {
    Ok(value) => println!("Success: {}", value),
    Err(e) => eprintln!("Error: {}", e),
}

// Last arm can be irrefutable catchall
match value {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("other"), // irrefutable wildcard
}
```

#### `if let` expressions
```rust
// ✅ Valid - refutable pattern in if let
if let Some(x) = some_option {
    println!("Got: {}", x);
}

if let Ok(value) = file_result {
    process(value);
}

// Can combine with else
if let Some(x) = opt {
    println!("Some: {}", x);
} else {
    println!("None");
}

// ❌ Warning - irrefutable pattern in if let (pointless)
if let x = 5 {
//      ^ Warning: irrefutable pattern, use let instead
    println!("{}", x);
}
```

#### `while let` loops
```rust
// ✅ Valid - refutable pattern in while let
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{}", top);
}

// Iterator pattern
let mut iter = vec.into_iter();
while let Some(val) = iter.next() {
    println!("{}", val);
}

// ❌ Warning - irrefutable pattern in while let (infinite loop!)
while let x = 5 {
//        ^ Warning: irrefutable pattern, use loop instead
    // This would loop forever!
}
```

---

## 🔄 Converting Between Forms

### Making Irrefutable from Refutable: `if let`

When you **want** to use a refutable pattern in a place that requires irrefutable:

```rust
// ❌ Won't compile
let Some(x) = some_option;

// ✅ Solution 1: Use if let
if let Some(x) = some_option {
    println!("Got: {}", x);
}

// ✅ Solution 2: Use match
let x = match some_option {
    Some(val) => val,
    None => panic!("Expected Some!"),
};

// ✅ Solution 3: Use unwrap/expect (if you're sure)
let x = some_option.expect("Expected Some value");
```

### Making Refutable from Irrefutable: Add guard or specific match

When you **try** to use an irrefutable pattern where refutable is expected:

```rust
// ⚠️ Warning - irrefutable pattern in if let
if let x = 5 {
    println!("{}", x);
}

// ✅ Solution: Just use let
let x = 5;
println!("{}", x);

// Or use if with condition instead
if x == 5 {
    println!("It's 5!");
}
```

---

## 💡 Practical Examples

### Example 1: Option Handling Patterns

```rust
fn process_config(config: Option<Config>) {
    // ❌ Won't compile - refutable in let
    // let Some(cfg) = config;
    
    // ✅ Pattern 1: if let for optional processing
    if let Some(cfg) = config {
        println!("Using config: {:?}", cfg);
        apply_config(cfg);
    } else {
        println!("No config provided, using defaults");
    }
    
    // ✅ Pattern 2: match for handling both cases
    match config {
        Some(cfg) => apply_config(cfg),
        None => use_defaults(),
    }
    
    // ✅ Pattern 3: unwrap_or for default value
    let cfg = config.unwrap_or_default();
    apply_config(cfg);
}
```

### Example 2: Result Handling Patterns

```rust
fn read_file(path: &str) -> Result<String, io::Error> {
    // ❌ Won't compile - refutable in let
    // let Ok(contents) = fs::read_to_string(path);
    
    // ✅ Pattern 1: Early return with ?
    let contents = fs::read_to_string(path)?;
    Ok(contents)
    
    // ✅ Pattern 2: match for custom error handling
    match fs::read_to_string(path) {
        Ok(contents) => Ok(contents),
        Err(e) => {
            eprintln!("Failed to read {}: {}", path, e);
            Err(e)
        }
    }
}

fn process_result(result: Result<i32, String>) {
    // ✅ Pattern 3: if let for success case only
    if let Ok(value) = result {
        println!("Success: {}", value);
    }
    
    // ✅ Pattern 4: if let for error case only
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}
```

### Example 3: Iterator Patterns

```rust
fn process_items(items: Vec<String>) {
    // ✅ Irrefutable: for loop always destructures iterator items
    for item in items.iter() {
        println!("{}", item);
    }
    
    // ✅ Refutable: while let with manual iteration
    let mut iter = items.iter();
    while let Some(item) = iter.next() {
        println!("{}", item);
        
        // Can break early based on conditions
        if item == "stop" {
            break;
        }
    }
    
    // ✅ Refutable: pop from vec
    let mut stack = items;
    while let Some(item) = stack.pop() {
        println!("Popped: {}", item);
    }
}
```

### Example 4: Tuple and Struct Destructuring

```rust
struct Point { x: i32, y: i32 }

fn demonstrate_destructuring() {
    // ✅ Irrefutable: tuple destructuring in let
    let tuple = (1, 2, 3);
    let (a, b, c) = tuple;
    println!("{}, {}, {}", a, b, c);
    
    // ✅ Irrefutable: struct destructuring in let
    let point = Point { x: 10, y: 20 };
    let Point { x, y } = point;
    println!("x: {}, y: {}", x, y);
    
    // ✅ Irrefutable: nested destructuring
    let nested = ((1, 2), (3, 4));
    let ((a, b), (c, d)) = nested;
    
    // ✅ Irrefutable: partial destructuring with ..
    let Point { x, .. } = point; // Only extract x
}

fn demonstrate_function_params((x, y): (i32, i32)) {
    // ✅ Irrefutable: function parameter destructuring
    println!("Coordinates: ({}, {})", x, y);
}
```

### Example 5: Match Guard Patterns

```rust
fn categorize_number(num: Option<i32>) {
    match num {
        // Refutable: pattern with guard
        Some(n) if n < 0 => println!("Negative: {}", n),
        Some(n) if n == 0 => println!("Zero"),
        Some(n) if n > 0 => println!("Positive: {}", n),
        
        // Guard makes pattern more specific
        Some(n) if n % 2 == 0 => println!("Even: {}", n),
        Some(n) => println!("Odd: {}", n),
        
        None => println!("No number"),
    }
}
```

---

## 🚨 Common Errors and Solutions

### Error 1: Refutable Pattern in `let`

```rust
// ❌ Error
let Some(x) = some_option;
// Error: refutable pattern in local binding: `None` not covered

// ✅ Solution 1: Use if let
if let Some(x) = some_option {
    println!("{}", x);
}

// ✅ Solution 2: Use match with panic for None
let x = match some_option {
    Some(x) => x,
    None => panic!("Expected Some!"),
};

// ✅ Solution 3: Provide default with unwrap_or
let x = some_option.unwrap_or(0);
```

### Error 2: Irrefutable Pattern in `if let`

```rust
// ⚠️ Warning (but compiles)
if let x = 5 {
    println!("{}", x);
}
// Warning: irrefutable `if let` pattern

// ✅ Solution: Use let instead
let x = 5;
println!("{}", x);
```

### Error 3: Refutable Pattern in Function Parameter

```rust
// ❌ Error
fn process_some(Some(x): Option<i32>) {
//              ^^^^^^^ Error: refutable pattern in parameter
    println!("{}", x);
}

// ✅ Solution 1: Accept Option, use if let inside
fn process_some(opt: Option<i32>) {
    if let Some(x) = opt {
        println!("{}", x);
    }
}

// ✅ Solution 2: Accept Option, use match
fn process_some(opt: Option<i32>) {
    match opt {
        Some(x) => println!("{}", x),
        None => println!("No value"),
    }
}

// ✅ Solution 3: Different API - take unwrapped value directly
fn process_some(x: i32) {
    println!("{}", x);
}
// Caller does: if let Some(x) = opt { process_some(x); }
```

---

## 🎯 Decision Tree: Which Pattern Form?

```
Need to handle a value?
│
├─ Value ALWAYS has same shape? (tuple, struct, primitive)
│  └─ Use IRREFUTABLE pattern in `let`
│     Example: let (x, y) = point;
│
└─ Value MIGHT fail to match? (Option, Result, enum, literal)
   │
   ├─ Need to handle both success and failure?
   │  └─ Use `match` expression
   │     Example: match opt { Some(x) => ..., None => ... }
   │
   ├─ Only care about success case?
   │  └─ Use `if let` expression
   │     Example: if let Some(x) = opt { ... }
   │
   ├─ Loop while pattern matches?
   │  └─ Use `while let` loop
   │     Example: while let Some(x) = iter.next() { ... }
   │
   └─ Want to panic/unwrap on failure?
      └─ Use `let` with unwrap/expect
         Example: let x = opt.expect("msg");
```

---

## 📚 Real-World AoC Example: Day 7 Part 2

From AoC 2025 Day 7 - demonstrating pattern matching from Ch19.1:

```rust
// Irrefutable: let destructuring of tuple
let (grid, start_pos) = parse_manifold(input)?;

// Refutable: if let for Option handling (cache lookup)
if let Some(&cached) = memo.get(&pos) {
    return cached; // Early return with cached value
}

// Refutable: if let for bounds checking
if let Some(next_pos) = grid.try_move_in_direction(pos, Direction::South) {
    // Process valid move
    process_next_position(next_pos);
}

// Refutable: while let for queue processing
while let Some((pos, dir)) = queue.pop_front() {
    // Process beam until queue empty
    handle_beam(pos, dir);
}

// Refutable: match for cell type branching
match cell {
    Cell::Splitter => {
        // Create left and right beams
        split_beam(pos);
    }
    Cell::Empty => {
        // Continue south
        continue_beam(pos);
    }
}
```

---

## 🎓 Key Takeaways

1. **Irrefutable patterns** = Always match → Use in `let`, function params, `for` loops
2. **Refutable patterns** = Can fail → Use in `match`, `if let`, `while let`
3. **Rust enforces this at compile time** → Prevents runtime pattern match failures in irrefutable contexts
4. **`if let` and `while let`** exist specifically to handle refutable patterns outside of match
5. **Match arms can be refutable** except the final catchall arm should be irrefutable
6. **Function parameters must be irrefutable** → Forces callers to handle optionality
7. **Compiler warnings help** → "irrefutable pattern in if let" means use `let` instead

---

## 🔗 Related Concepts

- **Pattern matching syntax**: [[pattern-matching]]
- **if let expressions**: [[if-let-expressions]]
- **while let loops**: [[while-let-loops]]
- **match expressions**: [[match-expressions]]
- **Option and Result**: [[option-type]], [[result-type]]
- **Destructuring**: [[destructuring-patterns]]
- **Match guards**: [[match-guards]]

---

## *Links:*

**Rust Book:** [[rust-book-ch19]] | [[rust-book-ch18-pattern-syntax]]

**Pattern Types:** [[pattern-matching]] | [[if-let-expressions]] | [[while-let-loops]] | [[match-expressions]]

**Data Types:** [[option-type]] | [[result-type]] | [[enum-patterns]]

**Applications:** [[aoc-day-07]] | [[error-handling-patterns]]

**Advanced:** [[match-guards]] | [[destructuring-patterns]] | [[pattern-binding-modes]]

---

*Tags: #rust #patterns #refutable #irrefutable #match #if-let #while-let #rust-book-ch19*
