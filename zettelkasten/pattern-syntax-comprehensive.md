# Pattern Syntax Comprehensive

**Tags:** #rust #patterns #pattern-matching #destructuring #match-guards #bindings #rust-book-ch19  
**Created:** 2025-12-09  
**Related:** [[rust_book/rust-book-ch19]], [[pattern-matching-locations]], [[refutable-vs-irrefutable-patterns]], [[match-expression-patterns]]

---

## 🎯 Core Concept

**Pattern Syntax** in Rust provides a rich language for matching against the structure of types. Patterns can destructure data, test values, bind variables, and combine multiple conditions - all in a single, expressive syntax.

**Key Insight**: Patterns aren't just for `match` - they're a fundamental feature that makes Rust code more expressive, safer, and more concise throughout the language.

---

## 📋 Complete Pattern Syntax Reference

### **1. Literal Patterns**

Match exact values directly:

```rust
fn match_literals(x: i32) {
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything else"),
    }
}

fn match_chars(c: char) {
    match c {
        'q' => println!("Quit command"),
        'w' => println!("Write command"),
        _ => println!("Unknown command"),
    }
}

fn match_strings(s: &str) {
    match s {
        "start" => println!("Starting..."),
        "stop" => println!("Stopping..."),
        _ => println!("Unknown: {}", s),
    }
}
```

**When to use**: Exact value matching, command parsing, state machines

### **2. Named Variable Patterns**

Bind values to variables:

```rust
fn match_named_variables() {
    let x = Some(5);
    let y = 10;
    
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => {
            // This y SHADOWS the outer y!
            println!("Matched, y = {}", y);  // Prints: y = 5
        }
        _ => println!("Default case"),
    }
    
    println!("Outer y = {}", y);  // Still 10 - not affected by match
}
```

**⚠️ Critical Warning**: Variables in patterns create **new bindings** that shadow outer variables!

**Best practice**: Use different names to avoid confusion:
```rust
match x {
    Some(value) => println!("Got {}", value),  // Clear intent
    _ => println!("None"),
}
```

### **3. Multiple Patterns with `|` (OR)**

Match any of several patterns:

```rust
fn match_multiple_patterns(x: i32) {
    match x {
        1 | 2 => println!("one or two"),
        3 | 4 | 5 => println!("three through five"),
        _ => println!("something else"),
    }
}

fn categorize_digit(c: char) {
    match c {
        '0' | '2' | '4' | '6' | '8' => println!("Even digit"),
        '1' | '3' | '5' | '7' | '9' => println!("Odd digit"),
        _ => println!("Not a digit"),
    }
}
```

**When to use**: Grouping similar cases, reducing code duplication

### **4. Range Patterns with `..=` (Inclusive)**

Match a range of values:

```rust
fn match_ranges(x: i32) {
    match x {
        1..=5 => println!("one through five"),
        6..=10 => println!("six through ten"),
        _ => println!("something else"),
    }
}

fn categorize_char(c: char) {
    match c {
        'a'..='j' => println!("Early ASCII letter"),
        'k'..='z' => println!("Late ASCII letter"),
        'A'..='Z' => println!("Uppercase letter"),
        '0'..='9' => println!("Digit"),
        _ => println!("Other character"),
    }
}
```

**Types that support ranges**:
- Integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- Characters: `char`

**⚠️ Important**: Use `..=` (inclusive) in patterns, not `..` (exclusive is not allowed)

### **5. Destructuring Structs**

Extract fields from structs:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn destructure_structs() {
    let p = Point { x: 0, y: 7 };
    
    // Full destructuring
    let Point { x, y } = p;
    println!("x: {}, y: {}", x, y);
    
    // Shorthand when variable names match field names
    let Point { x, y } = p;
    
    // Different variable names
    let Point { x: a, y: b } = p;
    println!("a: {}, b: {}", a, b);
    
    // Partial destructuring with .. (ignore rest)
    let Point { x, .. } = p;  // Only bind x, ignore y
    
    // Match with literal values
    match p {
        Point { x: 0, y } => println!("On y-axis at {}", y),
        Point { x, y: 0 } => println!("On x-axis at {}", x),
        Point { x, y } => println!("At ({}, {})", x, y),
    }
}

// More complex example
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

fn process_color(color: Color) {
    match color {
        Color { r: 255, g: 0, b: 0, .. } => println!("Bright red"),
        Color { a: 0, .. } => println!("Fully transparent"),
        Color { r, g, b, a: 255 } => println!("Opaque RGB({}, {}, {})", r, g, b),
        _ => println!("Some other color"),
    }
}
```

**Common patterns**:
- `{ x, y }` - Bind all fields with same names
- `{ x: a, y: b }` - Bind with different names
- `{ x, .. }` - Ignore remaining fields
- `{ x: 0, y }` - Match exact value for x, bind y

### **6. Destructuring Enums**

Extract data from enum variants:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destructure_enums(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quit - no data to destructure");
        }
        Message::Move { x, y } => {
            println!("Move to ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to RGB({}, {}, {})", r, g, b);
        }
    }
}

// With if let for single case
fn process_write_message(msg: Message) {
    if let Message::Write(text) = msg {
        println!("Got text: {}", text);
    }
}
```

**Enum destructuring patterns**:
- Unit variants: `Message::Quit` (no data)
- Struct-like variants: `Message::Move { x, y }`
- Tuple-like variants: `Message::Write(text)`

### **7. Destructuring Nested Structures**

Extract data from deeply nested types:

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum NestedMessage {
    ChangeColor(Color),
    Move { x: i32, y: i32 },
}

fn destructure_nested(msg: NestedMessage) {
    match msg {
        // Match nested enum variant
        NestedMessage::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change to RGB: ({}, {}, {})", r, g, b);
        }
        NestedMessage::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change to HSV: ({}, {}, {})", h, s, v);
        }
        NestedMessage::Move { x, y } => {
            println!("Move to ({}, {})", x, y);
        }
    }
}

// Even more complex nesting
struct Location {
    position: Point,
    metadata: Metadata,
}

struct Metadata {
    timestamp: u64,
    user_id: Option<u32>,
}

fn process_location(loc: Location) {
    match loc {
        Location {
            position: Point { x: 0, y: 0 },
            metadata: Metadata { user_id: Some(id), .. },
        } => {
            println!("User {} at origin", id);
        }
        Location {
            position: Point { x, y },
            ..
        } => {
            println!("Position: ({}, {})", x, y);
        }
    }
}
```

**Key technique**: Nest patterns to match structure exactly

### **8. Destructuring Tuples**

Extract elements from tuples:

```rust
fn destructure_tuples() {
    let triple = (1, 2, 3);
    
    // Full destructuring
    let (a, b, c) = triple;
    
    // Partial destructuring with _
    let (first, _, third) = triple;
    
    // Match on tuple values
    match triple {
        (0, y, z) => println!("First is 0, y={}, z={}", y, z),
        (1, ..) => println!("First is 1, don't care about rest"),
        (.., 3) => println!("Last is 3, don't care about rest"),
        (x, y, z) => println!("({}, {}, {})", x, y, z),
    }
}

// Complex nested tuple destructuring
fn process_complex_tuple(data: ((i32, i32), Point)) {
    let ((feet, inches), Point { x, y }) = data;
    println!("Feet: {}, Inches: {}, Point: ({}, {})", feet, inches, x, y);
}
```

### **9. Destructuring Arrays and Slices**

Extract elements from fixed-size arrays:

```rust
fn destructure_arrays() {
    let numbers = [1, 2, 3, 4, 5];
    
    match numbers {
        // Match exact array
        [1, 2, 3, 4, 5] => println!("Exact match"),
        
        // First and rest
        [first, ..] => println!("First: {}", first),
        
        // First and last
        [first, .., last] => println!("First: {}, Last: {}", first, last),
        
        // First two
        [a, b, ..] => println!("First two: {}, {}", a, b),
        
        // Last two
        [.., a, b] => println!("Last two: {}, {}", a, b),
        
        // Middle ignored
        [first, .., last] => println!("Ends: {}, {}", first, last),
    }
}

// Slices with pattern matching
fn process_slice(data: &[i32]) {
    match data {
        [] => println!("Empty slice"),
        [single] => println!("One element: {}", single),
        [first, second] => println!("Two elements: {}, {}", first, second),
        [first, .., last] => println!("First: {}, Last: {}", first, last),
    }
}
```

**⚠️ Limitation**: Full slice patterns work only with fixed-length slices in some contexts. Use `if let` for runtime length checking.

### **10. Ignoring Values with `_` and `..`**

Skip binding parts of values:

```rust
// Ignore entire value
fn foo(_: i32, y: i32) {
    println!("This function only uses y = {}", y);
}

// Ignore parts of tuple
fn ignore_tuple_parts() {
    let (x, _, z) = (1, 2, 3);  // Ignore middle element
    println!("x: {}, z: {}", x, z);
}

// Ignore struct fields
fn ignore_struct_fields(point: Point) {
    let Point { x, .. } = point;  // Only care about x
    println!("x: {}", x);
}

// Ignore with prefix (suppress unused warning)
fn ignore_unused() {
    let _unused_variable = 5;  // Won't warn even if unused
    let _x = calculate_something();  // Intentionally unused
}

// .. ignores remaining parts
fn ignore_remaining() {
    let numbers = (1, 2, 3, 4, 5);
    let (first, ..) = numbers;  // Take first, ignore rest
    let (.., last) = numbers;   // Take last, ignore rest
    let (first, .., last) = numbers;  // Take ends, ignore middle
}
```

**Difference**:
- `_` - Ignores a **single** value (doesn't bind)
- `..` - Ignores **remaining** values (rest pattern)
- `_variable` - Binds but suppresses unused warning

### **11. Match Guards (Extra `if` Conditions)**

Add additional conditions to patterns:

```rust
fn match_with_guards(num: Option<i32>) {
    match num {
        Some(x) if x % 2 == 0 => {
            println!("Even number: {}", x);
        }
        Some(x) if x % 2 != 0 => {
            println!("Odd number: {}", x);
        }
        Some(x) => {
            // This is unreachable in this example,
            // but shows you can have default Some case
            println!("Number: {}", x);
        }
        None => {
            println!("No number");
        }
    }
}

// Guards with multiple patterns
fn categorize_number(x: i32) {
    match x {
        n if n < 0 => println!("Negative: {}", n),
        0 => println!("Zero"),
        n if n % 2 == 0 => println!("Positive even: {}", n),
        n => println!("Positive odd: {}", n),
    }
}

// Guards with OR patterns - applies to ALL patterns!
fn test_guard_with_or(x: i32, y: bool) {
    match x {
        // Guard applies to BOTH 4 and 5!
        4 | 5 if y => println!("4 or 5 and y is true"),
        4 | 5 => println!("4 or 5 and y is false"),
        _ => println!("Not 4 or 5"),
    }
}

// Complex guard conditions
fn process_point(point: Point) {
    match point {
        Point { x, y } if x == y => {
            println!("On diagonal at {}", x);
        }
        Point { x, y } if x > y => {
            println!("Above diagonal at ({}, {})", x, y);
        }
        Point { x, y } => {
            println!("Below diagonal at ({}, {})", x, y);
        }
    }
}
```

**⚠️ Critical with `|` patterns**: The guard applies to **all** alternatives in the `|` pattern!

```rust
match x {
    1 | 2 if condition => { }  // Means: (1 if condition) OR (2 if condition)
    // NOT: 1 OR (2 if condition)
}
```

### **12. `@` Bindings (Bind AND Test)**

Bind a value to a variable while also testing it:

```rust
enum Message {
    Hello { id: i32 },
}

fn at_bindings_basic() {
    let msg = Message::Hello { id: 5 };
    
    match msg {
        // Bind to id_variable AND test that it's in range 3..=7
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found id in range: {}", id_variable);
        }
        // Test range but DON'T bind (can't use the value)
        Message::Hello { id: 10..=12 } => {
            println!("Found id in another range");
            // Can't use id here!
        }
        // Bind without testing
        Message::Hello { id } => {
            println!("Found some other id: {}", id);
        }
    }
}

// More complex @ usage
fn process_value(value: Option<i32>) {
    match value {
        // Bind to n AND test it's positive
        Some(n @ 1..) => {
            println!("Positive number: {}", n);
        }
        // Bind to n AND test it's negative
        Some(n @ ..=-1) => {
            println!("Negative number: {}", n);
        }
        Some(0) => {
            println!("Zero");
        }
        None => {
            println!("None");
        }
    }
}

// @ with nested patterns
struct Container {
    value: Result<i32, String>,
}

fn process_container(container: Container) {
    match container {
        // Bind the Ok value AND test it's in range
        Container {
            value: Ok(n @ 10..=20),
        } => {
            println!("Valid value in range: {}", n);
        }
        // Bind the entire Result for further processing
        Container {
            value: result @ Ok(_),
        } => {
            println!("Some Ok value: {:?}", result);
        }
        Container {
            value: Err(e),
        } => {
            println!("Error: {}", e);
        }
    }
}
```

**When to use `@`**:
- ✅ Need the value AND need to test it matches a pattern
- ✅ Matching ranges but need the actual value
- ✅ Complex nested patterns where you want intermediate binding

**When NOT needed**:
- ❌ Simple binding: `Some(x)` is clearer than `Some(x @ _)`
- ❌ Just testing: `Some(3..=7)` if you don't need the value

---

## 🎯 Pattern Combination Examples

### **Example 1: Comprehensive Message Parser**

```rust
#[derive(Debug)]
enum Command {
    Move { x: i32, y: i32, fast: bool },
    Write { text: String, urgent: bool },
    ChangeColor { r: u8, g: u8, b: u8 },
    Quit,
}

fn process_command(cmd: Command) {
    match cmd {
        // Urgent write with short text
        Command::Write {
            text: msg @ _,
            urgent: true,
        } if msg.len() < 10 => {
            println!("URGENT SHORT: {}", msg);
        }
        
        // Urgent write with long text
        Command::Write {
            text,
            urgent: true,
        } => {
            println!("URGENT: {}", text);
        }
        
        // Normal write
        Command::Write { text, urgent: false } => {
            println!("Normal: {}", text);
        }
        
        // Fast move to origin
        Command::Move { x: 0, y: 0, fast: true } => {
            println!("Fast return to origin");
        }
        
        // Fast move elsewhere
        Command::Move { x, y, fast: true } => {
            println!("Fast move to ({}, {})", x, y);
        }
        
        // Slow move
        Command::Move { x, y, fast: false } => {
            println!("Slow move to ({}, {})", x, y);
        }
        
        // Grayscale (r == g == b)
        Command::ChangeColor { r, g, b } if r == g && g == b => {
            println!("Grayscale: {}", r);
        }
        
        // Red, green, or blue
        Command::ChangeColor { r: 255, g: 0, b: 0 }
        | Command::ChangeColor { r: 0, g: 255, b: 0 }
        | Command::ChangeColor { r: 0, g: 0, b: 255 } => {
            println!("Primary color");
        }
        
        // Other colors
        Command::ChangeColor { r, g, b } => {
            println!("RGB({}, {}, {})", r, g, b);
        }
        
        Command::Quit => {
            println!("Quitting");
        }
    }
}
```

### **Example 2: AoC-Style Problem Solving**

```rust
#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn process_move(current: Coord, dir: Direction, distance: i64) -> Coord {
    match (current, dir, distance) {
        // No movement
        (pos, _, 0) => pos,
        
        // Cardinal directions with @ binding for distance
        (Coord { x, y }, Direction::North, d @ 1..) => {
            Coord { x, y: y + d }
        }
        (Coord { x, y }, Direction::South, d @ 1..) => {
            Coord { x, y: y - d }
        }
        (Coord { x, y }, Direction::East, d @ 1..) => {
            Coord { x: x + d, y }
        }
        (Coord { x, y }, Direction::West, d @ 1..) => {
            Coord { x: x - d, y }
        }
        
        // Negative distance (shouldn't happen, but handle it)
        (pos, _, _) => {
            eprintln!("Invalid negative distance");
            pos
        }
    }
}
```

### **Example 3: State Machine with Guards**

```rust
#[derive(Debug, PartialEq)]
enum State {
    Idle,
    Running { tasks: usize },
    Paused { tasks: usize },
    Shutdown,
}

fn transition(current: State, event: &str) -> State {
    match (current, event) {
        // From Idle
        (State::Idle, "start") => State::Running { tasks: 0 },
        (State::Idle, "shutdown") => State::Shutdown,
        
        // From Running - add task
        (State::Running { tasks }, "add") => {
            State::Running { tasks: tasks + 1 }
        }
        
        // From Running - pause only if tasks exist
        (State::Running { tasks: n @ 1.. }, "pause") => {
            State::Paused { tasks: n }
        }
        
        // From Running - can't pause with no tasks
        (State::Running { tasks: 0 }, "pause") => {
            println!("Can't pause with no tasks");
            State::Running { tasks: 0 }
        }
        
        // From Paused
        (State::Paused { tasks }, "resume") => {
            State::Running { tasks }
        }
        
        // Shutdown from any state
        (_, "shutdown") => State::Shutdown,
        
        // Invalid transitions stay in current state
        (state, event) => {
            println!("Invalid event '{}' for state {:?}", event, state);
            state
        }
    }
}
```

---

## 🎓 Common Patterns and Idioms

### **Pattern 1: Option Unwrapping with Default**

```rust
fn get_value_or_default(opt: Option<i32>) -> i32 {
    match opt {
        Some(value) => value,
        None => 0,
    }
}

// Or with if let for side effects
fn process_if_some(opt: Option<String>) {
    if let Some(text) = opt {
        println!("Got: {}", text);
    }
}
```

### **Pattern 2: Result Error Handling**

```rust
fn process_result(result: Result<i32, String>) {
    match result {
        Ok(value) if value > 0 => println!("Positive: {}", value),
        Ok(value) => println!("Non-positive: {}", value),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### **Pattern 3: Multi-Level Option/Result**

```rust
fn process_nested(data: Option<Result<i32, String>>) {
    match data {
        Some(Ok(value)) => println!("Success: {}", value),
        Some(Err(e)) => println!("Error: {}", e),
        None => println!("No data"),
    }
}
```

### **Pattern 4: Iterator with Destructuring**

```rust
fn process_coordinates(coords: Vec<(i32, i32)>) {
    for (x, y) in coords {
        println!("Point: ({}, {})", x, y);
    }
}

// With enumerate
fn process_with_index(values: Vec<i32>) {
    for (i, value) in values.iter().enumerate() {
        println!("Index {}: {}", i, value);
    }
}
```

### **Pattern 5: Function Parameter Destructuring**

```rust
// Destructure in function signature
fn print_point(&(x, y): &(i32, i32)) {
    println!("({}, {})", x, y);
}

// With structs
fn distance_from_origin(&Point { x, y }: &Point) -> f64 {
    ((x * x + y * y) as f64).sqrt()
}
```

---

## 🚨 Common Pitfalls and Solutions

### **Pitfall 1: Variable Shadowing in Match**

```rust
// ❌ CONFUSING: Shadows outer variable
let y = 10;
match x {
    Some(y) => println!("y = {}", y),  // This y shadows outer y!
    None => {}
}

// ✅ CLEAR: Use different name
let y = 10;
match x {
    Some(value) => println!("value = {}", value),
    None => {}
}
println!("y still = {}", y);
```

### **Pitfall 2: Match Guard with OR Patterns**

```rust
// ❌ WRONG EXPECTATION: Guard applies to ALL patterns in |
match x {
    4 | 5 if y => {}  // Means: (4 if y) OR (5 if y)
}

// If you want: 4 OR (5 if y), you need separate arms:
match x {
    4 => {}
    5 if y => {}
    _ => {}
}
```

### **Pitfall 3: Forgetting `..` for Struct Rest**

```rust
struct LargeStruct {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
}

// ❌ TEDIOUS: Must list all fields
let LargeStruct { a, b, c, d, e } = value;

// ✅ BETTER: Use .. for rest
let LargeStruct { a, .. } = value;
```

### **Pitfall 4: @ Binding Overuse**

```rust
// ❌ UNNECESSARY: Simple binding doesn't need @
match value {
    Some(x @ _) => println!("{}", x),  // Just use Some(x)
    None => {}
}

// ✅ USE @ WHEN: Testing AND binding
match value {
    Some(x @ 1..=10) => println!("In range: {}", x),  // Good use of @
    _ => {}
}
```

---

## 📊 Pattern Decision Tree

```
Need to match a value?
├─ Single exact value? → Use literal pattern: `42`, `"hello"`
├─ Multiple exact values? → Use `|`: `1 | 2 | 3`
├─ Range of values? → Use `..=`: `1..=10`, `'a'..='z'`
└─ Complex structure?
    ├─ Struct? → Destructure: `Point { x, y }`
    ├─ Enum? → Match variant: `Some(value)`
    ├─ Tuple? → Destructure: `(a, b, c)`
    └─ Array? → Destructure: `[first, .., last]`

Need additional condition?
└─ Use match guard: `if x > 0`

Need to bind AND test?
└─ Use @ binding: `value @ 1..=10`

Want to ignore parts?
├─ Single value? → Use `_`
├─ Remaining values? → Use `..`
└─ Suppress unused warning? → Prefix with `_`: `_x`
```

---

## 🔗 Related Concepts

**Fundamentals**:
- [[pattern-matching-locations]] - Where patterns can be used
- [[refutable-vs-irrefutable-patterns]] - Pattern exhaustiveness
- [[match-expression-patterns]] - Match arms and exhaustiveness

**Applications**:
- [[rust_book/rust-book-ch6]] - Enums and pattern matching basics
- [[rust_book/rust-book-ch9]] - Result and Option patterns
- [[mission-8]] - Graph algorithms with destructuring

**Advanced**:
- [[trait-patterns]] - Using patterns with trait implementations
- [[macro-patterns]] - Macros and pattern matching
- [[error-handling-patterns]] - Result/Option pattern idioms

---

## 💡 Best Practices

1. **Prefer exhaustive matching** - Let compiler help you catch missing cases
2. **Use meaningful names** - Avoid shadowing, choose clear variable names
3. **Keep patterns simple** - Complex nested patterns can be split into multiple match levels
4. **Use `if let` for single cases** - Don't use `match` when `if let` is clearer
5. **Document guard logic** - Complex guards deserve comments
6. **@ bindings sparingly** - Only when you truly need both test and bind
7. **Destructure in loops** - Use pattern destructuring in `for` loops for clarity

---

*Pattern syntax is one of Rust's most powerful features, enabling expressive, type-safe code that clearly expresses intent. Master patterns to write more idiomatic, maintainable Rust code. From simple literal matching to complex nested destructuring with guards and @ bindings, patterns make Rust code both safe and elegant.*

**Links:** [[rust_book/rust-book-ch19]] | [[pattern-matching-locations]] | [[refutable-vs-irrefutable-patterns]] | [[match-expression-patterns]] | [[mission-8]] | [[rust_book/rust-book-ch6]]
