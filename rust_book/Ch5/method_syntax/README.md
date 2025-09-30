# Chapter 5.3 · Method Syntax

> **Learning Context**: Chapter 5.3 introduces method syntax and associated functions, fundamental concepts for implementing Mission5's HashMap with ergonomic APIs and proper Rust idioms.

**Cross-Track Integration:**
- **Mission5 Connection**: Method syntax enables `HashMap::new()` and `map.insert()` ergonomic APIs - see [[Mission5 Overview]]
- **Rust Book**: Part of Chapter 5 Structs sequence building toward object-oriented patterns
- **Daily Study**: Foundation for collection method patterns used in Week 2 studies

**Related Zettelkasten Notes:**
- [[Collections MOC]] - Method patterns across all data structure implementations
- [[zettel-index]] - Main learning hub

## Core Concepts

### Methods vs Associated Functions
```rust
impl Rectangle {
    // Method: takes &self, &mut self, or self as first parameter
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Associated function: no self parameter (like static method)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Usage:
let rect = Rectangle::square(5);  // Associated function with ::
let area = rect.area();           // Method with .
```

### Method Syntax Benefits
- **Ergonomic API**: `rect.area()` vs `area(&rect)`
- **Method Chaining**: `rect.scale(2).rotate(90).draw()`
- **Namespace Organization**: Related functions grouped in `impl` blocks
- **Automatic Referencing**: `rect.area()` automatically borrows `&rect`

## Implementation Patterns

### Basic Method Types
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Immutable borrow method
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Mutable borrow method
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
    
    // Taking ownership method
    fn into_square(self) -> Rectangle {
        let size = std::cmp::max(self.width, self.height);
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

### Associated Functions (Constructors)
```rust
impl Rectangle {
    // Primary constructor
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    
    // Convenience constructors
    fn square(size: u32) -> Self {
        Self::new(size, size)
    }
    
    // Alternative constructor patterns
    fn from_dimensions(dimensions: (u32, u32)) -> Self {
        Self::new(dimensions.0, dimensions.1)
    }
}
```

### Method Chaining Patterns
```rust
impl Rectangle {
    // Return Self for chaining
    fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    
    fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
}

// Usage:
let rect = Rectangle::new(10, 20)
    .with_width(30)
    .with_height(40);
```

## Mission5 Integration: HashMap Method Design

### Ergonomic API Design (REQ-2 & REQ-3)
```rust
// Mission5 HashMap method patterns following Rust std library conventions
impl<K, V> HashMap<K, V> 
where 
    K: Eq + Hash 
{
    // Associated function constructors
    pub fn new() -> Self { /* REQ-1: Generic support */ }
    pub fn with_capacity(capacity: usize) -> Self { /* REQ-5: Performance */ }
    
    // Core methods with proper borrowing
    pub fn insert(&mut self, key: K, value: V) -> Option<V> { /* REQ-2 */ }
    pub fn get(&self, key: &K) -> Option<&V> { /* REQ-3 */ }
    pub fn remove(&mut self, key: &K) -> Option<V> { /* REQ-3 */ }
    
    // Convenience methods 
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
    
    pub fn len(&self) -> usize { /* REQ-4: Size tracking */ }
    pub fn is_empty(&self) -> bool { 
        self.len() == 0 
    }
}

// Usage matches standard library patterns:
let mut map = HashMap::new();           // Associated function
map.insert("key".to_string(), 42);     // Mutable method
let value = map.get("key");             // Immutable method
```

### Method Resolution and Automatic Dereferencing
```rust
// Rust automatically handles borrowing in method calls
let map = HashMap::new();

// These are equivalent:
map.len()        // Automatic borrowing
(&map).len()     // Explicit borrowing
HashMap::len(&map)  // Function syntax

// Rust picks the right method based on receiver type:
// fn method(&self)     - borrows
// fn method(&mut self) - mutably borrows  
// fn method(self)      - takes ownership
```

## Multiple Implementation Blocks

### Organizing Related Functionality
```rust
// Core functionality
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Display functionality
impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    fn description(&self) -> String {
        format!("{}x{} rectangle", self.width, self.height)
    }
}

// Comparison functionality  
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

## Best Practices

### Method Naming Conventions
```rust
impl Collection {
    // Constructors: new, with_*, from_*
    fn new() -> Self { }
    fn with_capacity(cap: usize) -> Self { }
    fn from_iter<I>(iter: I) -> Self { }
    
    // Getters: just the field name (no get_ prefix)
    fn len(&self) -> usize { }
    fn capacity(&self) -> usize { }
    
    // Boolean queries: is_*, has_*, can_*
    fn is_empty(&self) -> bool { }
    fn contains(&self, item: &T) -> bool { }
    
    // Mutating methods: clear action verbs
    fn push(&mut self, item: T) { }
    fn pop(&mut self) -> Option<T> { }
    fn clear(&mut self) { }
}
```

### Ownership Guidelines
```rust
impl Data {
    // Prefer borrowing when possible
    fn analyze(&self) -> Report { }        // ✅ Read-only analysis
    
    // Mutable borrow when modification needed  
    fn update(&mut self) { }               // ✅ In-place update
    
    // Take ownership only when consuming
    fn into_processed(self) -> Processed { } // ✅ Consumes self
}
```

## Advanced Method Patterns

### Generic Methods
```rust
impl<T> MyCollection<T> {
    fn map<U, F>(&self, f: F) -> MyCollection<U> 
    where 
        F: Fn(&T) -> U 
    {
        // Transform elements with function f
    }
    
    fn find<P>(&self, predicate: P) -> Option<&T>
    where 
        P: Fn(&T) -> bool
    {
        // Find first element matching predicate
    }
}
```

### Method Delegation
```rust
impl OuterStruct {
    // Delegate to inner field methods
    fn len(&self) -> usize {
        self.inner_collection.len()
    }
    
    fn is_empty(&self) -> bool {
        self.inner_collection.is_empty()
    }
}
```

## 3-Track Learning Integration

**Mission5 Applications:**
- Constructor patterns: `HashMap::new()`, `HashMap::with_capacity()`
- Method chaining for builder patterns and fluent APIs  
- Proper borrowing semantics for safe concurrent access

**Daily Study Connections:**
- Method syntax enables ergonomic collection APIs from Week 1-2
- Iterator methods (Day 13) built on same method resolution principles
- Error handling methods (upcoming) follow same naming conventions

**Rust Book Progression:**
- Chapter 5.1 (Structs) → 5.2 (Example) → **5.3 (Methods)** → Chapter 6 (Enums)
- Foundation for trait methods (Chapter 10)
- Enables generic programming patterns (Chapter 10)

---
**Zettelkasten Integration:**
*Links: [[Collections MOC]] | [[Mission5 Overview]] | [[zettel-index]]*

*Tags: #method-syntax #structs #api-design #rust-book #ch5 #mission5 #ergonomic-apis #ownership*

## 🚀 **Complete Runnable Example**

```rust
// Copy this entire block to Rust Playground or save as a .rs file

#[derive(Debug, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated functions (constructors)
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    
    fn square(size: u32) -> Self {
        Self::new(size, size)
    }
    
    // Immutable methods
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // Mutable methods
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
    
    // Consuming methods (take ownership)
    fn into_square(self) -> Rectangle {
        let size = std::cmp::max(self.width, self.height);
        Rectangle::square(size)
    }
    
    // Method chaining patterns
    fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    
    fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
}

// Multiple impl blocks are allowed
impl Rectangle {
    fn description(&self) -> String {
        format!("Rectangle: {}x{} (area: {}, perimeter: {})", 
                self.width, self.height, self.area(), self.perimeter())
    }
}

fn main() {
    println!("=== Method Syntax Demo from Chapter 5.3 ===\n");
    
    // 1. Associated functions (constructors)
    println!("1. Constructor Patterns:");
    let rect1 = Rectangle::new(30, 50);
    let square = Rectangle::square(25);
    println!("   Rectangle: {:?}", rect1);
    println!("   Square: {:?}", square);
    
    // 2. Method calls with automatic borrowing
    println!("\n2. Method Calls:");
    println!("   Rectangle area: {}", rect1.area());
    println!("   Rectangle perimeter: {}", rect1.perimeter());
    println!("   Square area: {}", square.area());
    
    // 3. Method chaining
    println!("\n3. Method Chaining:");
    let chained = Rectangle::new(10, 20)
        .with_width(15)
        .with_height(25);
    println!("   Chained rectangle: {:?}", chained);
    
    // 4. Comparison methods
    println!("\n4. Rectangle Comparison:");
    let small_rect = Rectangle::new(10, 15);
    let large_rect = Rectangle::new(40, 60);
    
    println!("   Can large hold small? {}", large_rect.can_hold(&small_rect));
    println!("   Can small hold large? {}", small_rect.can_hold(&large_rect));
    
    // 5. Mutable methods
    println!("\n5. Mutable Operations:");
    let mut mutable_rect = Rectangle::new(5, 10);
    println!("   Before scaling: {:?}", mutable_rect);
    mutable_rect.scale(3);
    println!("   After scaling by 3: {:?}", mutable_rect);
    
    // 6. Consuming methods (ownership transfer)
    println!("\n6. Consuming Methods:");
    let rect_to_consume = Rectangle::new(20, 30);
    println!("   Original rectangle: {:?}", rect_to_consume);
    let new_square = rect_to_consume.into_square();
    println!("   Converted to square: {:?}", new_square);
    // rect_to_consume is no longer available here
    
    // 7. Multiple impl blocks
    println!("\n7. Multiple Implementation Blocks:");
    let described_rect = Rectangle::new(12, 8);
    println!("   {}", described_rect.description());
    
    // 8. Method resolution demonstration
    println!("\n8. Method Resolution Examples:");
    let demo_rect = Rectangle::square(15);
    
    // These are all equivalent:
    println!("   Method syntax: {}", demo_rect.area());
    println!("   Function syntax: {}", Rectangle::area(&demo_rect));
    // Explicit reference: Rectangle::area(&demo_rect)
    
    println!("\n=== Method syntax provides ergonomic, chainable APIs! ===");
}
```

### **🛠️ How to Run This Code:**

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `ch5_3_demo.rs` and run `rustc ch5_3_demo.rs && ./ch5_3_demo`
3. **In this workspace**: Run the existing `cargo run` command in the `Ch5/method_syntax` directory
4. **Test the example**: `cargo run --example method_syntax_demo` (if added to workspace)