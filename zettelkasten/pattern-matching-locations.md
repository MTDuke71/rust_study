# Pattern Matching Locations

*Comprehensive guide to all places where patterns can be used in Rust*

## Overview

Rust provides six distinct locations where patterns can be used to destructure and match values. Understanding these locations is crucial for writing idiomatic Rust code.

**Key Insight**: Every `let` statement uses a pattern - even simple `let x = 5` has pattern `x`!

## The Six Pattern Locations

### 1. match Arms (Exhaustive Matching)

The most common and powerful pattern location. `match` expressions require exhaustive handling of all possibilities.

```rust
let point = Point { x: 3, y: 0 };

match point {
    Point { x: 0, y: 0 } => println!("Origin"),
    Point { x, y: 0 } => println!("On x-axis at {}", x),
    Point { x: 0, y } => println!("On y-axis at {}", y),
    Point { x, y } => println!("At ({}, {})", x, y),
}
```

**Characteristics**:
- Exhaustive - must cover all cases or use `_` wildcard
- Type-safe - compiler verifies pattern coverage
- Can use guards: `Some(x) if x > 10 => ...`
- Returns a value from each arm

**When to Use**: When you need to handle multiple variants or want compiler-enforced exhaustiveness.

### 2. if let Expressions (Single Pattern)

Convenient for matching a single pattern without exhaustive matching requirements.

```rust
let config_max = Some(3);

if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
} else {
    println!("No maximum configured");
}
```

**Characteristics**:
- Non-exhaustive - optional `else` clause
- Cleaner than `match` for single pattern
- Can chain with `else if` and `else if let`

**When to Use**: When you only care about one specific pattern and want to ignore other cases concisely.

### 3. while let Conditional Loops

Runs a loop as long as a pattern continues to match.

```rust
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop() {
    println!("Popped: {}", top);
}
// Loop stops when stack.pop() returns None
```

**Characteristics**:
- Loops until pattern fails to match
- Perfect for consuming iterators/collections
- Cleaner than `loop { match ... }`

**When to Use**: Processing items until a source is exhausted or a condition no longer matches.

### 4. for Loops (Iterator Destructuring)

Every `for` loop uses patterns to destructure iterator items.

```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}

// Destructuring tuples/structs
let points = vec![Point { x: 0, y: 0 }, Point { x: 1, y: 1 }];
for Point { x, y } in points {
    println!("Point: ({}, {})", x, y);
}
```

**Characteristics**:
- Pattern appears after `for` keyword
- Destructures each iterator item
- Irrefutable pattern required (must always match)

**When to Use**: Iterating over collections with automatic destructuring of complex types.

### 5. let Statements (Variable Binding)

The most fundamental pattern location - every `let` is a pattern match.

```rust
// Simple binding (pattern: x)
let x = 5;

// Tuple destructuring
let (a, b, c) = (1, 2, 3);

// Nested destructuring
let ((x1, y1), (x2, y2)) = ((1, 2), (3, 4));

// Struct destructuring
let Point { x, y } = Point::new(10, 20);

// Struct shorthand
let Point { x, y } = point;  // Same as { x: x, y: y }
```

**Characteristics**:
- Irrefutable pattern required (must always match)
- Most concise way to destructure
- Can ignore parts with `_`

**When to Use**: Binding variables from tuples, structs, or other complex types.

### 6. Function Parameters (Argument Destructuring)

Function parameters are patterns that destructure arguments.

```rust
// Tuple destructuring in parameter
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

let point = (3, 5);
print_coordinates(&point);

// Struct destructuring in parameter
fn process_point(Point { x, y }: Point) -> i32 {
    x + y
}
```

**Characteristics**:
- Irrefutable patterns only
- Destructures at function call site
- Cleaner than destructuring in function body

**When to Use**: When functions consistently destructure their arguments.

## Pattern Location Comparison

| **Location** | **Exhaustive?** | **Refutability** | **Common Use** |
|--------------|-----------------|------------------|----------------|
| `match` | Yes ✅ | Refutable OK | Multiple variants, compiler verification |
| `if let` | No ❌ | Refutable only | Single pattern of interest |
| `while let` | No ❌ | Refutable only | Loop until pattern fails |
| `for` | N/A | Irrefutable only | Iterator destructuring |
| `let` | N/A | Irrefutable only | Variable binding |
| Function params | N/A | Irrefutable only | Argument destructuring |

## Practical Examples from AoC 2025 Day 7

### Processing Input with Patterns

```rust
// Using match for multi-variant processing
fn process_instruction(line: &str) -> Action {
    match line.split_whitespace().collect::<Vec<_>>().as_slice() {
        ["move", amount, "from", src, "to", dst] => {
            Action::Move {
                count: amount.parse().unwrap(),
                from: src.parse().unwrap(),
                to: dst.parse().unwrap(),
            }
        }
        _ => Action::Invalid,
    }
}

// Using if let for optional values
fn parse_number(s: &str) -> Option<i32> {
    if let Ok(num) = s.parse::<i32>() {
        Some(num)
    } else {
        None
    }
}

// Using while let to process lines
let mut lines = input.lines();
while let Some(line) = lines.next() {
    if line.is_empty() { break; }
    // Process line
}

// Using for with destructuring
for (index, value) in data.iter().enumerate() {
    // Both index and value available
}

// Using let for tuple destructuring
let (part1, part2) = input.split_once("\n\n").unwrap();
```

## Mission Integration

### Mission 6 (Grid) - Pattern Destructuring

```rust
use mission6::Grid;

// Destructuring grid coordinates in for loop
for Point { x, y } in grid.neighbors(pos) {
    // Direct access to x, y coordinates
}

// Match on grid cell states
match grid.get(pos) {
    Some(&Cell::Wall) => continue,
    Some(&Cell::Empty) => explore(pos),
    None => break,
}
```

### Mission 8 (Graph) - State Matching

```rust
// Match on traversal states
match visit_status {
    VisitState::Unvisited => {
        queue.push_back(node);
    }
    VisitState::Visited(distance) => {
        if new_distance < distance {
            update_distance(node, new_distance);
        }
    }
}

// While let for BFS queue processing
while let Some(current) = queue.pop_front() {
    for neighbor in graph.neighbors(current) {
        // Process neighbor
    }
}
```

### Mission 10 (Union-Find) - Pattern Guards

```rust
// Match with guards for set operations
match (find(x), find(y)) {
    (root_x, root_y) if root_x == root_y => {
        // Already in same set
        false
    }
    (root_x, root_y) => {
        union(root_x, root_y);
        true
    }
}
```

## Best Practices

### ✅ Do

1. **Use `match` for exhaustiveness**: Compiler catches missing cases
2. **Use `if let` for single patterns**: Cleaner than `match` with wildcard
3. **Destructure in `for` loops**: `for (k, v) in map` instead of `for item in map`
4. **Destructure in `let`**: `let (x, y) = point;` instead of `let x = point.0;`
5. **Use `while let` for consuming**: Better than `loop { match ... }`

### ❌ Avoid

1. **Nested `if let`**: Use `match` instead for multiple patterns
2. **Destructuring in function body**: Use parameter patterns when consistent
3. **Ignoring compiler warnings**: Refutability errors indicate wrong location
4. **Overusing wildcards in `match`**: Defeats exhaustiveness checking

## Common Patterns by Problem Type

### Parsing Input
- **`if let` / `match`**: Parse different input formats
- **`let` destructuring**: Split input into components
- **`for` with enumerate**: Process indexed data

### State Machines
- **`match`**: Exhaustive state transitions
- **Guards**: Conditional state changes

### Collection Processing
- **`while let`**: Drain/consume collections
- **`for` with destructuring**: Process key-value pairs, tuples

### Error Handling
- **`if let Ok(...)`**: Handle success case
- **`match Result`**: Handle both success and error

## Related Concepts

- [[rust_book/rust-book-ch6|Ch6 - Enums and Pattern Matching]] - Introduction to `match`
- [[rust_book/rust-book-ch19|Ch19 - Patterns and Matching]] - Comprehensive pattern guide
- [[refutable-vs-irrefutable-patterns|Refutability]] - Which patterns work where
- [[pattern-syntax-comprehensive|Pattern Syntax]] - All pattern syntax types
- [[mission-6|Mission 6 - Grid]] - Grid coordinate destructuring
- [[mission-8|Mission 8 - Graphs]] - Graph traversal patterns

## References

- Rust Book Chapter 19.1: "All the Places Patterns Can Be Used"
- [[rust_book/rust-book-ch19|Rust Book Ch19]]
- Example: `rust_book/Ch19/examples/ch19_1_pattern_locations.rs`

---

*Tags: #rust #patterns #matching #destructuring #control-flow*

*Created: 2025-12-07*  
*Context: Rust Book Ch19.1 study + AoC 2025 Day 7*
