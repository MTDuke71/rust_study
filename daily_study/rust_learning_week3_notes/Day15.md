# Day 15 · Traits Fundamentals (defining and implementing)

> **Learning Context**: Day 15 begins Week 3's deep dive into Rust's trait system, the foundation for generic programming and code reuse that enables Mission5's flexible HashMap implementation.

**Cross-Track Integration:**
- **Mission Focus**: Traits enable Mission5's `Eq + Hash` constraints and generic implementation - see [[mission-5]]
- **Daily Study**: Week 3 opener - transitions from collections to type system mastery
- **Rust Book**: Builds on Chapter 10 Generic Types, Traits, and Lifetimes

**Related Zettelkasten Notes:**
- [[Collections MOC]] - Trait usage patterns across data structures
- [[zettel-index]] - Main learning hub

## Core Concepts

### What Are Traits?
- **Interface Definition**: Traits define shared behavior that types can implement
- **Code Reuse**: Write generic code that works with any type implementing a trait
- **Compile-Time Polymorphism**: Zero-cost abstractions with static dispatch
- **Safety**: Type system ensures trait methods are available before use

### Trait Definition Syntax
```rust
// Define what behavior is possible
trait Drawable {
    // Required method - implementors must provide
    fn draw(&self);
    
    // Default implementation - implementors can override
    fn description(&self) -> String {
        String::from("A drawable object")
    }
    
    // Associated function (no self)
    fn type_name() -> &'static str {
        "Unknown Drawable"
    }
}
```

### Implementing Traits
```rust
struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
    
    // Override default implementation
    fn description(&self) -> String {
        format!("Circle with radius {}", self.radius)
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing rectangle {}x{}", self.width, self.height);
    }
    
    fn description(&self) -> String {
        format!("Rectangle {}x{}", self.width, self.height)
    }
}
```

## Standard Library Traits

### Debug and Display
```rust
use std::fmt;

#[derive(Debug)] // Automatic Debug implementation
struct Point {
    x: i32,
    y: i32,
}

// Manual Display implementation
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

let p = Point { x: 3, y: 4 };
println!("Debug: {:?}", p);      // Debug: Point { x: 3, y: 4 }
println!("Display: {}", p);      // Display: (3, 4)
```

### Clone and Copy
```rust
// Clone: explicit duplication
#[derive(Clone, Debug)]
struct ExpensiveData {
    data: Vec<i32>,
}

// Copy: implicit duplication (only for types that implement Clone)
#[derive(Clone, Copy, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

let expensive = ExpensiveData { data: vec![1, 2, 3] };
let cloned = expensive.clone(); // Explicit clone() call

let coord = Coordinate { x: 1, y: 2 };
let copied = coord; // Automatic copy (no clone() needed)
// coord still usable here because it implements Copy
```

### Eq and PartialEq
```rust
#[derive(Debug, PartialEq, Eq)]
struct Student {
    id: u32,
    name: String,
}

let student1 = Student { id: 1, name: "Alice".to_string() };
let student2 = Student { id: 1, name: "Alice".to_string() };

assert_eq!(student1, student2); // Uses PartialEq

// Custom equality implementation
impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id // Only compare IDs
    }
}
```

## Mission5 Integration: Trait Constraints for HashMap

### Hash and Eq Constraints (REQ-1 Generic Support)
```rust
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// Mission5 HashMap requires K: Eq + Hash
#[derive(Debug)]
struct UserId(u32);

// Implement required traits for HashMap key
impl PartialEq for UserId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for UserId {} // Marker trait

impl Hash for UserId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

// Now UserId can be used as HashMap key
let mut user_data: HashMap<UserId, String> = HashMap::new();
user_data.insert(UserId(123), "Alice".to_string());
user_data.insert(UserId(456), "Bob".to_string());

println!("User 123: {:?}", user_data.get(&UserId(123)));
```

### Generic Trait Bounds
```rust
// Mission5 pattern: generic functions with trait bounds
fn process_keys<K>(keys: Vec<K>) -> usize 
where 
    K: Hash + Eq + std::fmt::Debug
{
    // Can use Hash, Eq, and Debug methods on K
    println!("Processing keys: {:?}", keys);
    keys.len()
}

// Works with any type implementing the required traits
let string_keys = vec!["key1".to_string(), "key2".to_string()];
let user_keys = vec![UserId(1), UserId(2), UserId(3)];

let count1 = process_keys(string_keys);
let count2 = process_keys(user_keys);
```

## Advanced Trait Patterns

### Trait Bounds in Implementation
```rust
// Conditional implementation based on trait bounds
struct Container<T> {
    value: T,
}

// Only implement Clone if T implements Clone
impl<T: Clone> Clone for Container<T> {
    fn clone(&self) -> Self {
        Container {
            value: self.value.clone(),
        }
    }
}

// Only implement Debug if T implements Debug
impl<T: std::fmt::Debug> std::fmt::Debug for Container<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Container")
            .field("value", &self.value)
            .finish()
    }
}
```

### Multiple Trait Bounds
```rust
// Multiple bounds with + syntax
fn compare_and_print<T>(a: &T, b: &T) -> bool
where 
    T: PartialEq + std::fmt::Debug + Clone
{
    println!("Comparing {:?} and {:?}", a, b);
    a == b
}

// Alternative syntax
fn alternative_syntax<T: PartialEq + std::fmt::Debug>(a: &T, b: &T) -> bool {
    a == b
}
```

### Orphan Rule and Newtype Pattern
```rust
// Can't implement foreign trait for foreign type
// impl Display for Vec<i32> {} // ❌ Compilation error

// Solution: Newtype pattern
struct IntList(Vec<i32>);

impl std::fmt::Display for IntList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", 
            self.0.iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

let list = IntList(vec![1, 2, 3, 4]);
println!("{}", list); // [1, 2, 3, 4]
```

## Derivable Traits

### Common Derive Macros
```rust
// Most common derivable traits
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GameScore {
    player_id: u32,
    score: i32,
    level: u8,
}

// Copy requires Clone
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

// Ord requires PartialOrd, Eq, PartialEq
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Priority {
    urgency: u8,
    importance: u8,
}
```

## Real-World Applications

### Iterator Pattern with Traits
```rust
// Custom iterator implementing Iterator trait
struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

// Now can use with iterator methods
let counter = Counter::new(5);
let doubled: Vec<usize> = counter.map(|x| x * 2).collect();
println!("Doubled: {:?}", doubled); // [0, 2, 4, 6, 8]
```

### Trait Objects Preview (More in Day 19)
```rust
// Basic trait object usage
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, &self.content[0..50])
    }
}

// Vector of different types implementing Summary
let summaries: Vec<Box<dyn Summary>> = vec![
    Box::new(Article {
        title: "Rust Traits".to_string(),
        content: "Traits enable powerful generic programming...".to_string(),
    }),
];

for item in summaries {
    println!("{}", item.summarize());
}
```

## Best Practices

### Trait Design Principles
```rust
// ✅ Good: Small, focused traits
trait Readable {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error>;
}

trait Writable {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error>;
}

// ✅ Good: Combine small traits when needed
trait ReadWrite: Readable + Writable {}

// ❌ Avoid: Large, unfocused traits
trait Everything {
    fn read(&self) -> String;
    fn write(&self, data: String);
    fn process(&self);
    fn validate(&self) -> bool;
    // ... many more methods
}
```

### When to Use Traits
```rust
// ✅ Use traits for:
// 1. Shared behavior across different types
trait Drawable {
    fn draw(&self);
}

// 2. Generic constraints
fn sort_and_display<T: Ord + std::fmt::Debug>(mut items: Vec<T>) {
    items.sort();
    for item in items {
        println!("{:?}", item);
    }
}

// 3. Extension methods (but be careful of orphan rule)
trait StringExtensions {
    fn word_count(&self) -> usize;
}

impl StringExtensions for String {
    fn word_count(&self) -> usize {
        self.split_whitespace().count()
    }
}
```

## Learning Progression Summary

From Day 15, you should understand:
1. **Trait Definition**: Creating contracts that types can implement
2. **Standard Traits**: Debug, Display, Clone, Copy, Eq, Hash patterns
3. **Generic Constraints**: Using traits to constrain type parameters
4. **Derivable Traits**: Automatic implementations with `#[derive]`
5. **Mission5 Integration**: How traits enable generic HashMap implementation
6. **Best Practices**: Small, focused traits vs large, unfocused ones

**3-Track Learning Integration:**
- **Mission5**: Traits enable generic HashMap with `K: Eq + Hash` constraints
- **Week 3 Foundation**: Essential for Days 16-21 (Generics, Lifetimes, Advanced Traits)
- **Rust Book Ch10**: Practical application of theoretical trait concepts

**Cross-References:**
- [[Collections MOC]] - See "Trait Requirements" sections across data structures
- [[mission-5]] - REQ-1 generic support relies on trait system
- [[HashMap Internals]] - Hash and Eq traits essential for key storage

**Next**: Day 16 will cover **Generic Types** - parameterized types for flexible, reusable code!

---
**Zettelkasten Integration:**
*Links: [[Collections MOC]] | [[mission-5]] | [[zettel-index]]*

*Tags: #traits #fundamentals #generic-programming #interfaces #mission5 #daily-study #week3 #type-system*

## 🚀 **Complete Runnable Example**

```rust
// Copy this entire block to Rust Playground or save as a .rs file
use std::fmt;
use std::collections::HashMap;

// 1. Basic trait definition and implementation
trait Describable {
    fn describe(&self) -> String;
    
    // Default implementation
    fn category(&self) -> String {
        "Unknown".to_string()
    }
}

#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

#[derive(Debug, Clone)]
struct Movie {
    title: String,
    director: String,
    duration_minutes: u32,
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("'{}' by {} ({} pages)", self.title, self.author, self.pages)
    }
    
    fn category(&self) -> String {
        "Literature".to_string()
    }
}

impl Describable for Movie {
    fn describe(&self) -> String {
        format!("'{}' directed by {} ({} min)", self.title, self.director, self.duration_minutes)
    }
    
    fn category(&self) -> String {
        "Cinema".to_string()
    }
}

// 2. Custom type for HashMap key (Mission5 pattern)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct ProductId(u32);

impl fmt::Display for ProductId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Product#{}", self.0)
    }
}

// 3. Generic function with trait bounds
fn print_description<T: Describable + fmt::Debug>(item: &T) {
    println!("Item: {:?}", item);
    println!("Description: {}", item.describe());
    println!("Category: {}", item.category());
    println!();
}

// 4. Function requiring multiple trait bounds
fn store_in_map<K, V>(map: &mut HashMap<K, V>, key: K, value: V) -> Option<V>
where 
    K: std::hash::Hash + Eq + fmt::Debug,
    V: fmt::Debug,
{
    println!("Storing key: {:?}", key);
    map.insert(key, value)
}

fn main() {
    println!("=== Traits Fundamentals Demo from Day 15 ===\n");
    
    // 1. Basic trait usage
    println!("1. Basic Trait Implementation:");
    let book = Book {
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik".to_string(),
        pages: 552,
    };
    
    let movie = Movie {
        title: "The Matrix".to_string(),
        director: "The Wachowskis".to_string(),
        duration_minutes: 136,
    };
    
    print_description(&book);
    print_description(&movie);
    
    // 2. Standard library traits (Clone, Debug, etc.)
    println!("2. Standard Library Traits:");
    let original_book = book.clone(); // Clone trait
    println!("Original: {:?}", book);        // Debug trait
    println!("Cloned: {:?}", original_book);
    
    // 3. HashMap with custom key type (Mission5 pattern)
    println!("\n3. Custom Types as HashMap Keys (Mission5 Pattern):");
    let mut inventory: HashMap<ProductId, String> = HashMap::new();
    
    let product1 = ProductId(12345);
    let product2 = ProductId(67890);
    
    store_in_map(&mut inventory, product1.clone(), "Laptop".to_string());
    store_in_map(&mut inventory, product2.clone(), "Mouse".to_string());
    
    println!("Inventory:");
    for (id, name) in &inventory {
        println!("  {}: {}", id, name);
    }
    
    // 4. Derivable traits demonstration
    println!("\n4. Derivable Traits:");
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Coordinate {
        x: i32,
        y: i32,
    }
    
    let pos1 = Coordinate { x: 10, y: 20 };
    let pos2 = pos1.clone();
    let pos3 = Coordinate { x: 10, y: 20 };
    
    println!("Position 1: {:?}", pos1);
    println!("Position 2: {:?}", pos2);
    println!("pos1 == pos3: {}", pos1 == pos3);  // PartialEq
    
    // 5. Multiple trait bounds in action
    println!("\n5. Generic Functions with Trait Bounds:");
    
    fn compare_and_show<T>(a: &T, b: &T) -> bool
    where 
        T: PartialEq + fmt::Debug + Clone,
    {
        println!("Comparing {:?} and {:?}", a, b);
        a == b
    }
    
    let result1 = compare_and_show(&pos1, &pos3);
    let result2 = compare_and_show(&"hello", &"world");
    
    println!("Coordinates equal: {}", result1);
    println!("Strings equal: {}", result2);
    
    println!("\n=== Traits enable generic programming and code reuse! ===");
}
```

### **🛠️ How to Run This Code:**

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day15_demo.rs` and run `rustc day15_demo.rs && ./day15_demo`
3. **In this workspace**: `.\scripts\run_md.bat daily_study\rust_learning_week3_notes\Day15.md`
4. **As Cargo example**: `cargo run --example day15_traits_demo` (if you add it to Mission5_tut)
