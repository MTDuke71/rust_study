# Day 15 - Traits

**Tags:** #daily-study #traits #generics #rust #learning-progression
**Created:** 2025-10-22
**Related:** [[Chapter 10]], [[Generic Programming]], [[Day 16]], [[Closures in Rust]], [[Week 3 Overview]]

## Overview

**Traits** define shared behavior that types can implement. They're Rust's approach to polymorphism and code reuse, similar to interfaces in other languages but more powerful due to their integration with Rust's type system.

## Key Learning Goals

By the end of Day 15, you should understand:
- What traits are and why they're useful
- How to define and implement traits
- Trait bounds and generic constraints
- Common standard library traits
- Trait objects for dynamic dispatch

## Fundamental Concepts

### Defining Traits

```rust
// Define a trait that describes shared behavior
trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation (optional)
    fn summarize_short(&self) -> String {
        format!("(Read more...)")
    }
}
```

### Implementing Traits

```rust
struct Article {
    headline: String,
    content: String,
    author: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
}

// Implement trait for Article
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {} - {}", self.headline, self.author, self.content)
    }
}

// Implement trait for Tweet  
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    
    // Override default implementation
    fn summarize_short(&self) -> String {
        format!("@{}", self.username)
    }
}
```

### Using Traits

```rust
let article = Article {
    headline: "Rust Traits".to_string(),
    content: "Traits enable shared behavior...".to_string(),
    author: "Alice".to_string(),
};

let tweet = Tweet {
    username: "bob_dev".to_string(),
    content: "Learning Rust traits today!".to_string(),
    reply: false,
};

// Call trait methods
println!("{}", article.summarize());
println!("{}", tweet.summarize_short());
```

## Trait Bounds and Generics

### Function Parameters with Traits

```rust
// Accept any type that implements Summary
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Alternative syntax (impl trait)
fn notify_alt(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
fn process<T: Summary + Clone>(item: T) {
    let copy = item.clone();
    println!("{}", copy.summarize());
}

// Where clause for complex bounds
fn complex_function<T, U>(t: &T, u: &U) -> String
where
    T: Summary + Clone,
    U: Summary + std::fmt::Display,
{
    format!("{} and {}", t.summarize(), u)
}
```

### Return Types with Traits

```rust
// Return any type that implements Summary
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: "system".to_string(),
        content: "Daily update".to_string(),
        reply: false,
    }
}

// Note: Cannot return different types in same function
// This won't compile:
/*
fn returns_different(switch: bool) -> impl Summary {
    if switch {
        Article { ... }  // Error: different types
    } else {
        Tweet { ... }
    }
}
*/
```

## Essential Standard Library Traits

### Debug and Display
```rust
#[derive(Debug)]  // Automatic implementation
struct Point {
    x: i32,
    y: i32,
}

// Manual Display implementation
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

let p = Point { x: 1, y: 2 };
println!("{:?}", p);  // Debug output: Point { x: 1, y: 2 }
println!("{}", p);    // Display output: (1, 2)
```

### Clone and Copy
```rust
#[derive(Clone, Copy, Debug)]
struct SmallData(i32);

#[derive(Clone, Debug)]
struct LargeData(Vec<i32>);

let small = SmallData(42);
let small_copy = small;     // Copy (original still valid)
let small_clone = small.clone();  // Also works

let large = LargeData(vec![1, 2, 3]);
// let large_copy = large;  // Error: LargeData doesn't implement Copy
let large_clone = large.clone();  // Clone required for owned types
```

### PartialEq and Eq
```rust
#[derive(Debug, PartialEq, Eq)]
struct Person {
    name: String,
    age: u32,
}

let person1 = Person { name: "Alice".to_string(), age: 30 };
let person2 = Person { name: "Alice".to_string(), age: 30 };

assert_eq!(person1, person2);  // Uses PartialEq
```

### PartialOrd and Ord
```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

let v1 = Version { major: 1, minor: 2, patch: 3 };
let v2 = Version { major: 1, minor: 3, patch: 0 };

assert!(v1 < v2);  // Uses PartialOrd/Ord

let mut versions = vec![v2, v1];
versions.sort();   // Requires Ord
```

## Advanced Trait Patterns

### Trait Objects (Dynamic Dispatch)

```rust
// Store different types that implement the same trait
let items: Vec<Box<dyn Summary>> = vec![
    Box::new(Article { /* ... */ }),
    Box::new(Tweet { /* ... */ }),
];

for item in items {
    println!("{}", item.summarize());  // Runtime polymorphism
}
```

### Associated Types
```rust
trait Iterator {
    type Item;  // Associated type
    
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    current: usize,
    max: usize,
}

impl Iterator for Counter {
    type Item = usize;  // Concrete associated type
    
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
```

### Supertraits
```rust
// OutlinePrint requires Display to be implemented
trait OutlinePrint: std::fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();  // Uses Display
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

## Practical Examples

### Configuration Trait
```rust
trait Config {
    fn get_database_url(&self) -> String;
    fn get_port(&self) -> u16;
    fn is_debug(&self) -> bool { false }  // Default
}

struct ProductionConfig;
struct DevelopmentConfig;

impl Config for ProductionConfig {
    fn get_database_url(&self) -> String {
        "postgres://prod.db.com/app".to_string()
    }
    
    fn get_port(&self) -> u16 {
        80
    }
}

impl Config for DevelopmentConfig {
    fn get_database_url(&self) -> String {
        "postgres://localhost/app_dev".to_string()
    }
    
    fn get_port(&self) -> u16 {
        3000
    }
    
    fn is_debug(&self) -> bool {
        true
    }
}

fn start_server<C: Config>(config: C) {
    println!("Starting server on port {}", config.get_port());
    println!("Database: {}", config.get_database_url());
    if config.is_debug() {
        println!("Debug mode enabled");
    }
}
```

### Calculation Trait for Different Number Types
```rust
trait Calculate {
    fn add(&self, other: &Self) -> Self;
    fn multiply(&self, other: &Self) -> Self;
}

impl Calculate for i32 {
    fn add(&self, other: &Self) -> Self {
        self + other
    }
    
    fn multiply(&self, other: &Self) -> Self {
        self * other
    }
}

impl Calculate for f64 {
    fn add(&self, other: &Self) -> Self {
        self + other
    }
    
    fn multiply(&self, other: &Self) -> Self {
        self * other
    }
}

fn compute_result<T: Calculate + Copy>(a: T, b: T, c: T) -> T {
    a.add(&b).multiply(&c)  // (a + b) * c
}
```

## Common Derivable Traits

```rust
// Automatically implement common traits
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Student {
    name: String,
    id: u32,
    grade: char,
}

// Now Student can be:
let student = Student { /* ... */ };
println!("{:?}", student);        // Debug
let copy = student.clone();       // Clone
let is_same = student == copy;    // PartialEq
// Used in HashMap/HashSet       // Hash
// Sorted in collections         // Ord
```

## Integration with Collections

```rust
use std::collections::HashMap;

// Using traits with collections
let mut students: HashMap<u32, Student> = HashMap::new();
students.insert(123, student);  // Requires Hash + Eq for key type

let mut grades: Vec<Student> = vec![/* ... */];
grades.sort();  // Requires Ord

grades.dedup();  // Requires PartialEq
```

## Practice Exercises for Day 15

### Exercise 1: Shape Trait
```rust
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    
    fn describe(&self) -> String {
        format!("Area: {:.2}, Perimeter: {:.2}", self.area(), self.perimeter())
    }
}

// TODO: Implement for Circle, Rectangle, Triangle
struct Circle { radius: f64 }
struct Rectangle { width: f64, height: f64 }
```

### Exercise 2: Comparable Values
```rust
trait Comparable {
    fn is_greater_than(&self, other: &Self) -> bool;
    fn is_less_than(&self, other: &Self) -> bool;
    fn is_equal_to(&self, other: &Self) -> bool;
}

// TODO: Implement for Temperature struct
struct Temperature { celsius: f64 }
```

### Exercise 3: Data Processing Pipeline
```rust
trait Processor<T> {
    fn process(&self, input: T) -> T;
}

// TODO: Create different processors and chain them
struct Doubler;
struct Incrementer;
```

## Connection to Previous Days

- **Day 14:** Built on generic programming concepts
- **Collections:** Traits enable generic collection operations  
- **Error Handling:** Result and Option implement many useful traits
- **Ownership:** Clone and Copy traits manage memory behavior

## Looking Ahead to Day 16

Day 16 will explore:
- **Lifetimes** and their relationship to traits
- **Advanced trait bounds** with lifetime parameters
- **Trait objects** and object safety rules
- **Associated types vs generic parameters**

## Key Takeaways

1. **Traits define shared behavior** across different types
2. **Trait bounds** enable generic functions with constraints
3. **Standard library traits** provide fundamental functionality
4. **Derive macros** automatically implement common traits
5. **Trait objects** enable runtime polymorphism

## Study Tips

- Practice implementing traits for your own types
- Explore standard library trait implementations
- Understand when to use `impl Trait` vs trait objects
- Learn the derivable traits and when to use them
- Connect traits to generic programming concepts

## Related Concepts

- [[Generic Programming]] - Traits provide bounds for generics
- [[Chapter 10]] - Full coverage of traits, generics, and lifetimes
- [[Closures in Rust]] - Closure traits (Fn, FnMut, FnOnce)
- [[Collections MOC]] - Traits enable generic collection behavior
- [[Day 16]] - Next step: lifetimes and advanced traits

---

*Day 15: Master traits as Rust's powerful abstraction mechanism for shared behavior and polymorphism.*