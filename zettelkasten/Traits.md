# 🎭 Traits in Rust

**Shared behavior and interface definitions through Rust's trait system**

## 🎯 Core Concept

**Traits** define shared behavior that types can implement. They are similar to interfaces in other languages but more powerful, enabling polymorphism, operator overloading, and compile-time abstraction without runtime overhead.

## 📋 Basic Trait Definition

### **Simple Trait**

```rust
// Define a trait
trait Drawable {
    fn draw(&self);
    
    // Default implementation
    fn display(&self) {
        println!("Displaying item");
        self.draw();
    }
}

// Implement trait for a type
struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}

// Use the trait
let circle = Circle { radius: 5.0 };
circle.draw();
circle.display(); // Uses default implementation
```

## 🔧 Trait Bounds and Generic Constraints

### **Constraining Generic Types**

```rust
// Function with trait bound
fn print_if_displayable<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}

// Multiple trait bounds
fn process<T: Clone + std::fmt::Debug>(item: T) {
    let copy = item.clone();
    println!("{:?}", copy);
}

// Where clause syntax (more readable)
fn complex_function<T, U>(t: T, u: U)
where
    T: Clone + std::fmt::Debug,
    U: std::fmt::Display,
{
    println!("{:?} and {}", t, u);
}
```

### **Trait Bounds in Structs**

```rust
struct Container<T: Clone> {
    value: T,
}

impl<T: Clone> Container<T> {
    fn duplicate(&self) -> T {
        self.value.clone()
    }
}
```

## 🌟 Common Standard Library Traits

### **Derivable Traits**

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

// Automatically implements:
// - Debug: {:?} formatting
// - Clone: .clone() method
// - PartialEq: == and != operators
// - Eq: full equivalence relation
// - Hash: use in HashMap/HashSet
```

### **Display and Debug**

```rust
use std::fmt;

struct Person {
    name: String,
    age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (age {})", self.name, self.age)
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Person")
            .field("name", &self.name)
            .field("age", &self.age)
            .finish()
    }
}
```

### **Operator Overloading**

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 3, y: 4 };
let p3 = p1 + p2; // Uses Add trait
println!("{:?}", p3); // Point { x: 4, y: 6 }
```

## 🔁 Trait Objects and Dynamic Dispatch

### **Static vs Dynamic Dispatch**

```rust
// Static dispatch (compile-time)
fn process_static<T: Drawable>(item: &T) {
    item.draw();
}

// Dynamic dispatch (runtime)
fn process_dynamic(item: &dyn Drawable) {
    item.draw();
}

// Trait object in collection
let drawables: Vec<Box<dyn Drawable>> = vec![
    Box::new(Circle { radius: 5.0 }),
    Box::new(Rectangle { width: 10.0, height: 20.0 }),
];

for drawable in &drawables {
    drawable.draw();
}
```

## 🎯 Associated Types

### **Defining Associated Types**

```rust
trait Iterator {
    type Item; // Associated type
    
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
    max: u32,
}

impl Iterator for Counter {
    type Item = u32; // Concrete type
    
    fn next(&mut self) -> Option<u32> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

## 📊 Supertraits

### **Trait Inheritance**

```rust
// Subtrait requires supertrait
trait Printable: std::fmt::Display {
    fn print(&self) {
        println!("{}", self);
    }
}

// Must implement Display before Printable
impl std::fmt::Display for MyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MyType")
    }
}

impl Printable for MyType {}
```

## 🛠️ Trait Implementation Patterns

### **Extension Traits**

```rust
// Add methods to existing types
trait VecExt<T> {
    fn first_or_default(&self, default: T) -> T;
}

impl<T: Clone> VecExt<T> for Vec<T> {
    fn first_or_default(&self, default: T) -> T {
        self.first().cloned().unwrap_or(default)
    }
}

// Usage
let v = vec![1, 2, 3];
let first = v.first_or_default(0); // 1
```

### **Blanket Implementations**

```rust
// Implement for all types that satisfy a constraint
trait MyTrait {
    fn do_something(&self);
}

impl<T: std::fmt::Display> MyTrait for T {
    fn do_something(&self) {
        println!("Doing something with: {}", self);
    }
}

// Now all Display types have MyTrait
42.do_something();
"hello".do_something();
```

## 🎓 Mission Applications

### **Mission 1 - Stack Traits**

```rust
trait Stack<T> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
}
```

### **Mission 3 - Binary Search Trait Bounds**

```rust
fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    // Requires Ord trait for comparisons
}
```

### **Mission 5 - HashMap Trait Requirements**

```rust
use std::hash::Hash;

struct HashMap<K: Hash + Eq, V> {
    // K must implement Hash and Eq traits
}
```

## 🔗 Key Trait Categories

### **Marker Traits**

- `Send` - Type can be transferred across thread boundaries
- `Sync` - Type can be referenced from multiple threads
- `Copy` - Type can be copied with simple bitwise copy
- `Sized` - Type has known size at compile time

### **Conversion Traits**

- `From<T>` / `Into<T>` - Type conversions
- `TryFrom<T>` / `TryInto<T>` - Fallible conversions
- `AsRef<T>` / `AsMut<T>` - Reference conversions

### **Comparison Traits**

- `PartialEq` - Partial equivalence relation
- `Eq` - Full equivalence relation
- `PartialOrd` - Partial ordering
- `Ord` - Total ordering

### **Operation Traits**

- `Add`, `Sub`, `Mul`, `Div` - Arithmetic operators
- `Index`, `IndexMut` - Array indexing
- `Deref`, `DerefMut` - Smart pointer dereferencing

## 💡 Best Practices

### **1. Prefer Trait Bounds Over Trait Objects**

```rust
// Good: Static dispatch, better performance
fn process<T: Display>(item: T) { }

// Use only when heterogeneous types needed
fn process_dynamic(item: Box<dyn Display>) { }
```

### **2. Use Derive When Possible**

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point { x: i32, y: i32 }
```

### **3. Consider Default Implementations**

```rust
trait MyTrait {
    fn required(&self);
    
    fn optional(&self) {
        println!("Default implementation");
    }
}
```

### **4. Use Where Clauses for Complex Bounds**

```rust
fn complex<T, U>(t: T, u: U)
where
    T: Clone + Debug + Display,
    U: Iterator<Item = T>,
{
    // More readable than inline bounds
}
```

## 🔍 Common Patterns

### **Builder Pattern with Traits**

```rust
trait Builder {
    type Output;
    fn build(self) -> Self::Output;
}

struct ConfigBuilder { /* ... */ }
impl Builder for ConfigBuilder {
    type Output = Config;
    fn build(self) -> Config { /* ... */ }
}
```

### **Newtype Pattern**

```rust
struct UserId(u64);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "User #{}", self.0)
    }
}
```

## 🐛 Common Pitfalls

### **Orphan Rule**

```rust
// Can't implement external trait for external type
// impl Display for Vec<i32> { } // ERROR!

// Solution: Use newtype pattern
struct MyVec(Vec<i32>);
impl Display for MyVec { /* OK */ }
```

### **Trait Object Safety**

```rust
// Not object-safe (generic method)
trait NotObjectSafe {
    fn generic<T>(&self, item: T); // ERROR in trait object
}

// Object-safe version
trait ObjectSafe {
    fn method(&self, item: i32); // OK
}
```

## 📚 Related Concepts

- [[Generic Programming]] - Using traits with generics
- [[zettelkasten/rust_book/rust-book-ch10]] - Rust Book Chapter 10: Traits and Generics
- [[Ownership and Borrowing]] - Trait implementations and ownership
- [[Error Handling Patterns]] - Result and Option traits
- [[rust-concepts-MOC]] - Core language features overview
- [[rust-oop-characteristics]] - **NEW**: Comparing Rust to traditional OOP (Ch 18)

---

## 🎯 Key Takeaways

1. **Traits define shared behavior** that types can implement
2. **Trait bounds constrain generics** to types with specific capabilities
3. **Static dispatch is preferred** for performance (monomorphization)
4. **Trait objects enable dynamic dispatch** for heterogeneous collections
5. **Associated types simplify complex trait designs**
6. **Derivable traits reduce boilerplate** code
7. **Marker traits enable compiler optimizations** and safety guarantees

---

*Tags: #rust #traits #polymorphism #generics #interfaces #type-system*
*Links: [[rust-book-ch9-12-review]] | [[Generic Programming]] | [[zettelkasten/rust_book/rust-book-ch10]] | [[rust-concepts-MOC]]*
