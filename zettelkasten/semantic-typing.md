# Semantic Typing - Self-Documenting Types

**Type**: Concept Note  
**Source**: [[rust-for-rustaceans]] Ch3.1 Obvious Interfaces  
**Related**: [[typestate-pattern]] | [[newtype-pattern]] | [[zero-cost-abstractions]]

---

## Overview

**Semantic typing** means using domain-specific types instead of primitive types to make code self-documenting and leverage the type system for compile-time validation.

**Core Principle**: The type name should communicate **meaning and constraints**, not just memory layout.

---

## The Problem with Primitives

### Ambiguous Function Signatures

```rust
// ❌ BAD: What do the bools mean? What's a valid port?
fn connect(encryption: bool, compression: bool, port: u16) -> Result<Connection, Error> {
    // ...
}

// Usage is unclear - which bool is which?
connect(true, false, 8080)?;  // encryption=true, compression=false? Or vice versa?
```

**Problems**:
- **No self-documentation** - must read docs/code to understand parameters
- **Easy to swap arguments** - `connect(false, true, 8080)` compiles but may be wrong intent
- **No validation** - can pass `port: 99999` (invalid port number)
- **IDE can't help** - autocomplete shows generic `bool`, `u16`

---

## The Solution: Semantic Types

### Enums for Boolean-Like States

```rust
#[derive(Debug, Clone, Copy)]
enum Encryption {
    Enabled,
    Disabled,
}

#[derive(Debug, Clone, Copy)]
enum Compression {
    Enabled,
    Disabled,
}

// ✅ GOOD: Intent is crystal clear
fn connect(
    encryption: Encryption,
    compression: Compression,
    port: Port,
) -> Result<Connection, Error> {
    // ...
}

// Usage is self-documenting
connect(
    Encryption::Enabled,
    Compression::Disabled,
    Port::new(8080)?,  // Validated at construction
)?;
```

**Benefits**:
- **Self-documenting** - parameter names match domain concepts
- **Type-safe** - cannot swap `Encryption` and `Compression`
- **Extensible** - can add `Encryption::Tls12`, `Encryption::Tls13` later
- **Pattern matching** - compiler ensures all variants handled

### Newtype for Validated Primitives

```rust
/// A valid network port (1-65535)
#[derive(Debug, Clone, Copy)]
pub struct Port(u16);

impl Port {
    /// Create a new port, validating the value is non-zero
    pub fn new(value: u16) -> Result<Self, PortError> {
        if value == 0 {
            return Err(PortError::InvalidPort);
        }
        Ok(Port(value))
    }
    
    /// Get the underlying port number
    pub fn get(&self) -> u16 {
        self.0
    }
}

#[derive(Debug)]
pub enum PortError {
    InvalidPort,
}
```

**Usage**:

```rust
// ✅ Valid port
let port = Port::new(8080)?;

// ❌ Compile-time error - cannot create Port directly
// let port = Port(0);  // Error: Port::0 is private

// ❌ Runtime error - validation catches invalid value
let invalid = Port::new(0);  // Err(PortError::InvalidPort)
```

**Key Insight**: Validation happens **once at construction**, then type system guarantees validity.

---

## Real-World Examples

### Mission Integration: Coordinates

```rust
// ❌ BAD: Tuple coordinates
fn find_path(start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
    // Easy to swap row/col
}

// ✅ GOOD: Semantic Coord type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        Coord { row, col }
    }
    
    pub fn neighbors(&self) -> [Coord; 4] {
        // Self-documenting: returns Coords, not tuples
        // ...
    }
}

fn find_path(start: Coord, end: Coord) -> Vec<Coord> {
    // Type signature documents domain intent
}
```

**Benefits**:
- **Cannot swap row/col** - different field names
- **Domain methods** - `neighbors()` is natural on Coord
- **Better error messages** - "expected Coord, found (usize, usize)"

### AoC Day 15: Hash Returns u8 Not usize

From today's session - excellent example of semantic typing:

```rust
// ❌ BAD: usize is too generic
fn hash(s: &str) -> usize {
    let mut value = 0_usize;
    for c in s.chars() {
        value = ((value + c as usize) * 17) % 256;
    }
    value
}

// ✅ GOOD: u8 documents mathematical constraint (0-255)
fn hash(s: &str) -> u8 {
    let mut value: u16 = 0;  // Intermediate to prevent overflow
    for c in s.chars() {
        value = ((value + c as u16) * 17) % 256;
    }
    value as u8  // Safe: modulo 256 guarantees result fits in u8
}
```

**Why this is better**:
- **Type documents invariant** - result is always 0-255
- **Explicit casts at usage** - `boxes[hash(label) as usize]` makes indexing intentional
- **Compiler catches mistakes** - cannot accidentally use hash result in arithmetic expecting larger range
- **Self-documenting** - reader knows hash space is 256 buckets from return type

---

## Pattern: The Newtype Pattern

Wrap primitive types in single-field structs to add meaning:

```rust
struct UserId(u64);
struct OrderId(u64);
struct Timestamp(i64);

// ✅ Type-safe - cannot mix up IDs
fn get_user_orders(user_id: UserId) -> Vec<OrderId> {
    // ...
}

// ❌ Compile error - wrong type
let user = UserId(42);
let order = OrderId(100);
// get_user_orders(order);  // Error: expected UserId, found OrderId
```

**Zero-cost abstraction**: The wrapper is optimized away at runtime.

```rust
assert_eq!(
    std::mem::size_of::<UserId>(),
    std::mem::size_of::<u64>()
);
```

---

## Common Semantic Type Patterns

### 1. Units of Measurement

```rust
struct Meters(f64);
struct Kilometers(f64);
struct Miles(f64);

impl Meters {
    fn to_kilometers(&self) -> Kilometers {
        Kilometers(self.0 / 1000.0)
    }
}

// Prevents unit confusion bugs (e.g., Mars Climate Orbiter)
```

### 2. Validated Strings

```rust
/// An email address that has been validated
pub struct Email(String);

impl Email {
    pub fn parse(s: &str) -> Result<Self, EmailError> {
        // Validation logic
        if s.contains('@') {
            Ok(Email(s.to_string()))
        } else {
            Err(EmailError::MissingAtSign)
        }
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Once you have Email, it's guaranteed valid
fn send_email(to: Email, subject: &str, body: &str) {
    // No need to re-validate
}
```

### 3. Non-Empty Collections

```rust
/// A vector guaranteed to have at least one element
pub struct NonEmpty<T> {
    head: T,
    tail: Vec<T>,
}

impl<T> NonEmpty<T> {
    pub fn new(head: T) -> Self {
        NonEmpty { head, tail: vec![] }
    }
    
    pub fn push(&mut self, item: T) {
        self.tail.push(item);
    }
    
    /// Always safe - guaranteed to have at least one element
    pub fn first(&self) -> &T {
        &self.head
    }
}

// No need for Option<&T> or panic on empty - type guarantees non-empty
```

### 4. Positive Integers

```rust
/// A positive (non-zero) integer
#[derive(Debug, Clone, Copy)]
pub struct Positive(u32);

impl Positive {
    pub fn new(value: u32) -> Option<Self> {
        if value > 0 {
            Some(Positive(value))
        } else {
            None
        }
    }
    
    pub fn get(&self) -> u32 {
        self.0
    }
}

// Safe division - no risk of divide by zero
fn divide(numerator: u32, denominator: Positive) -> u32 {
    numerator / denominator.get()  // Safe - denominator is positive
}
```

---

## When to Use Semantic Types

### ✅ Use Semantic Types When:
- **Domain concept is important** - `UserId` is not just "a number"
- **Primitive is ambiguous** - `bool enable_x` is unclear, `Feature::Enabled` is obvious
- **Validation is needed** - `Email`, `Port`, `Positive`
- **Preventing mix-ups** - `Meters` vs `Kilometers`
- **Type has domain operations** - `Coord::neighbors()`, `Port::is_privileged()`

### ❌ Use Primitives When:
- **Truly generic data** - counts, indices in local scope
- **Over-engineering risk** - wrapping `i32` as `Count` for a 10-line function
- **Standard library patterns** - `Option<T>`, `Result<T, E>` already semantic

---

## Comparison with Typestate Pattern

| Semantic Typing | Typestate Pattern |
|----------------|-------------------|
| **What**: Meaningful names for values | **What**: Encode states as types |
| **Example**: `Port(u16)` vs `u16` | **Example**: `Connection<Connected>` vs `Connection<Disconnected>` |
| **Goal**: Self-documenting data | **Goal**: Compile-time state machine |
| **Benefit**: Clarity, validation | **Benefit**: Impossible invalid transitions |
| **Complexity**: Simple wrapper | **Complexity**: Generic type parameters |

**They complement each other**:

```rust
struct Port(u16);  // Semantic type

struct Connection<State> {  // Typestate pattern
    port: Port,  // Using semantic type inside
    state: PhantomData<State>,
}
```

---

## Performance Characteristics

### Zero-Cost Abstraction

```rust
struct Meters(f64);

// Compiled code is identical to using f64 directly
let distance = Meters(100.0);
let doubled = Meters(distance.0 * 2.0);

// Assembly: Same as f64 operations
// No runtime overhead for the wrapper
```

**Proof**:

```rust
assert_eq!(
    std::mem::size_of::<Meters>(),
    std::mem::size_of::<f64>()
);

assert_eq!(
    std::mem::align_of::<Meters>(),
    std::mem::align_of::<f64>()
);
```

### Validation Cost

Validation happens **once at construction**, then type guarantees validity:

```rust
// Pay validation cost once
let port = Port::new(8080)?;

// Use many times - no re-validation needed
for _ in 0..1000 {
    connect(port);  // No overhead
}
```

---

## Design Guidelines

### 1. Make Construction Validate

```rust
impl Port {
    /// Validates port is non-zero
    pub fn new(value: u16) -> Result<Self, PortError> {
        // Validation here
    }
}

// Don't expose the inner value for direct construction
// Don't impl Default if it could be invalid
```

### 2. Provide Safe Accessors

```rust
impl Port {
    /// Get the port number
    pub fn get(&self) -> u16 {
        self.0
    }
}

// Not: pub struct Port(pub u16);  // Breaks validation invariant
```

### 3. Implement Domain Operations

```rust
impl Port {
    /// Check if this is a privileged port (< 1024)
    pub fn is_privileged(&self) -> bool {
        self.0 < 1024
    }
}

// Methods that make sense for the domain concept
```

### 4. Consider derive Macros

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Port(u16);

// Common traits make semantic types ergonomic
```

---

## Mission Examples

### Mission 6: `Grid<T>` Uses Coord

```rust
// Semantic coordinate type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl<T> Grid<T> {
    // ✅ Self-documenting signature
    pub fn get(&self, coord: Coord) -> Option<&T> {
        self.data.get(coord.row)?.get(coord.col)
    }
    
    // ❌ Would be ambiguous
    // pub fn get(&self, row: usize, col: usize) -> Option<&T>
}
```

### Mission 10: UnionFind Size Wrapper

```rust
/// A valid UnionFind size (positive integer)
pub struct Size(usize);

impl Size {
    pub fn new(value: usize) -> Result<Self, SizeError> {
        if value == 0 {
            return Err(SizeError::ZeroSize);
        }
        Ok(Size(value))
    }
}

impl UnionFind {
    // Guaranteed non-zero size
    pub fn new(size: Size) -> Self {
        // No need to validate - type guarantees it
    }
}
```

---

## Common Mistakes

### 1. Making Fields Public

```rust
// ❌ BAD: Can bypass validation
pub struct Port(pub u16);

let invalid = Port(0);  // Oops, created invalid port

// ✅ GOOD: Private field, controlled access
pub struct Port(u16);
```

### 2. No Validation in Constructor

```rust
// ❌ BAD: Constructor doesn't validate
impl Email {
    pub fn new(s: String) -> Self {
        Email(s)  // What if s is not an email?
    }
}

// ✅ GOOD: Validate at construction
impl Email {
    pub fn parse(s: &str) -> Result<Self, EmailError> {
        // Validation logic
    }
}
```

### 3. Over-Engineering Simple Cases

```rust
// ❌ BAD: Overkill for local scope
fn count_items(items: &[Item]) -> Count {
    let mut count = Count(0);
    for _ in items {
        count.0 += 1;
    }
    count
}

// ✅ GOOD: Just use usize for simple counts
fn count_items(items: &[Item]) -> usize {
    items.len()
}
```

---

## Key Takeaways

1. **Semantic Types = Self-Documentation** - Type names communicate meaning
2. **Validate Once, Use Safely** - Construction validates, type guarantees
3. **Zero-Cost Abstraction** - Wrappers optimized away at runtime
4. **Prevent Mix-Ups** - `UserId` can't be swapped with `OrderId`
5. **Domain Operations** - Methods natural to the concept
6. **Complement Typestate** - Use together for powerful APIs
7. **Integrator Mindset** - Types are contracts between components

---

*Links*: [[rust-for-rustaceans]] | [[typestate-pattern]] | [[newtype-pattern]] | [[zero-cost-abstractions]] | [[phantom-data]] | [[api-design]]

*Tags*: #rust #semantic-typing #newtype #api-design #type-safety #self-documenting #domain-modeling #zero-cost
