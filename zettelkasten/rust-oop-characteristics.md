# 🎯 Rust OOP Characteristics - Comparing Rust to Traditional OOP

*Rust implements OOP concepts through composition and traits rather than classical inheritance, providing memory safety with zero-cost abstractions*

---

## 🎯 **Core Concept**

Object-Oriented Programming (OOP) has no single agreed-upon definition, but commonly includes:
1. **Objects** - Data bundled with behavior
2. **Encapsulation** - Hiding implementation details
3. **Inheritance** - Code reuse through type hierarchies
4. **Polymorphism** - Code that works with multiple types

Rust supports some OOP features but **rejects inheritance** in favor of **composition and traits**, which better align with memory safety and zero-cost abstractions.

---

## 🧠 **Mental Models**

### **The Rust vs Classical OOP Spectrum**

```
Traditional OOP (Java/C++)              Rust's Approach
═══════════════════════════════════════════════════════════════
Class Hierarchies      ──────────────►  Composition + Traits
Runtime Polymorphism   ──────────────►  Static + Dynamic Dispatch
Implementation Reuse   ──────────────►  Default Methods + Delegation
Virtual Methods        ──────────────►  Trait Objects (dyn Trait)
Protected Access       ──────────────►  Module Privacy (pub(crate))
```

### **Integration Analogy (AUTOSAR Context)**

Think of Rust's approach like **AUTOSAR's component model**:
- **Components** (structs) contain data and expose **ports** (trait implementations)
- **Interfaces** (traits) define contracts between components
- **Integration** happens through **composition**, not inheritance
- Components are **validated independently** (like mission libraries)

---

## 🔍 **Detailed Comparison**

### **1. Objects: Data + Behavior ✅**

Both traditional OOP and Rust bundle data with behavior, but with different syntax:

```rust
// Rust: struct + impl = object
pub struct AveragedCollection {
    list: Vec<i32>,      // Private data
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> Self {           // Constructor
        Self { list: vec![], average: 0.0 }
    }
    
    pub fn add(&mut self, value: i32) {    // Method
        self.list.push(value);
        self.update_average();
    }
    
    pub fn average(&self) -> f64 {         // Getter
        self.average
    }
    
    fn update_average(&mut self) {         // Private helper
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

**Key Insight**: `struct` + `impl` in Rust is equivalent to a class in traditional OOP.

---

### **2. Encapsulation ✅**

Rust provides **strong encapsulation** through its module privacy system:

| Feature | Traditional OOP | Rust |
|---------|----------------|------|
| Private by default | No (varies) | **Yes** |
| Public declaration | `public` keyword | `pub` keyword |
| Package-private | `package`, `internal` | `pub(crate)` |
| Protected | `protected` | **Not available** (no inheritance) |

```rust
// lib.rs - Only public API is visible outside module
pub struct Widget {
    // Private fields - implementation detail
    internal_state: Vec<u8>,
    cached_value: Option<i32>,
}

impl Widget {
    // Public constructor - API surface
    pub fn new() -> Self { /* ... */ }
    
    // Public method - API surface
    pub fn process(&mut self) -> i32 { /* ... */ }
    
    // Private helper - implementation detail
    fn calculate_internal(&self) -> i32 { /* ... */ }
}
```

**Rust Advantage**: The `pub` system is explicit and granular (`pub(crate)`, `pub(super)`, `pub(in path)`).

---

### **3. Inheritance ❌ (Rust Uses Alternatives)**

**Traditional OOP Problem**: Inheritance creates tight coupling and fragile base class problems.

**Rust's Solution**: Composition + Traits

#### **Why Rust Rejects Inheritance**

1. **Fragile Base Class Problem** - Changes to parent break children
2. **Diamond Problem** - Multiple inheritance ambiguity  
3. **Deep Hierarchies** - Hard to understand code flow
4. **Inflexibility** - Locked into hierarchy decisions at design time

#### **Rust Alternative: Composition**

```rust
// ❌ NOT POSSIBLE in Rust - No inheritance
// class Dog extends Animal { ... }

// ✅ Rust Way - Composition
struct Animal {
    name: String,
    age: u32,
}

struct Dog {
    animal: Animal,      // Composition - "has-a" not "is-a"
    breed: String,
}

impl Dog {
    fn name(&self) -> &str {
        &self.animal.name   // Delegate to composed type
    }
}
```

#### **Rust Alternative: Default Trait Methods**

```rust
// Trait with default implementation (like interface with default methods)
trait Summary {
    fn summarize_author(&self) -> String;  // Required method
    
    fn summarize(&self) -> String {        // Default implementation
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct NewsArticle { author: String, /* ... */ }

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
    // Uses default summarize() - code reuse without inheritance!
}
```

---

### **4. Polymorphism ✅ (Via Trait Objects)**

Rust supports polymorphism through **trait objects** (`dyn Trait`):

```rust
// Define shared behavior through a trait
pub trait Draw {
    fn draw(&self);
}

// Screen holds heterogeneous collection of drawable things
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,  // Trait object polymorphism
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();  // Polymorphic call - resolved at runtime
        }
    }
}

// Different types implementing the same trait
struct Button { label: String }
struct SelectBox { options: Vec<String> }

impl Draw for Button {
    fn draw(&self) { println!("Drawing button: {}", self.label); }
}

impl Draw for SelectBox {
    fn draw(&self) { println!("Drawing select box with {} options", self.options.len()); }
}

// Usage - heterogeneous collection
let screen = Screen {
    components: vec![
        Box::new(Button { label: "OK".to_string() }),
        Box::new(SelectBox { options: vec!["Yes".into(), "No".into()] }),
    ],
};
screen.run();  // Each calls its own draw() implementation
```

---

## ⚖️ **Trade-offs: Static vs Dynamic Dispatch**

| Aspect | Generics (Static) | Trait Objects (Dynamic) |
|--------|-------------------|-------------------------|
| **Dispatch** | Compile-time | Runtime (vtable) |
| **Performance** | Faster (inlined) | Slight overhead |
| **Binary size** | Larger (monomorphization) | Smaller |
| **Flexibility** | Homogeneous only | Heterogeneous |
| **Type info** | Preserved | Erased |

### **When to Use Each**

```rust
// Static dispatch - when all types known at compile time
fn process_all<T: Draw>(items: &[T]) {
    for item in items {
        item.draw();  // Compile-time dispatch, inlined
    }
}

// Dynamic dispatch - when types vary at runtime
fn process_dynamic(items: &[Box<dyn Draw>]) {
    for item in items {
        item.draw();  // Runtime vtable lookup
    }
}
```

**Decision Rule**: Use generics by default; use trait objects when you need heterogeneous collections or runtime flexibility.

---

## 🎭 **Object Safety Rules**

Not all traits can be used as trait objects. A trait is **object-safe** if:

1. **Return type isn't `Self`** (can't know concrete type at runtime)
2. **No generic type parameters** on methods (can't monomorphize)

```rust
// ✅ Object-safe - Can use as `dyn Draw`
trait Draw {
    fn draw(&self);
}

// ❌ NOT object-safe - Returns Self
trait Clone {
    fn clone(&self) -> Self;  // Self not allowed
}

// ❌ NOT object-safe - Generic method
trait Processor {
    fn process<T>(&self, item: T);  // Generic not allowed
}
```

---

## 💡 **Key Takeaways**

1. **Rust supports objects and encapsulation** fully through `struct` + `impl` + privacy
2. **No inheritance** - Use composition and default trait methods instead
3. **Polymorphism via trait objects** (`Box<dyn Trait>`) enables heterogeneous collections
4. **Prefer generics** for performance; use trait objects for flexibility
5. **Object safety rules** determine which traits can become trait objects
6. **Composition > Inheritance** aligns with modern software design principles

---

## 🔗 **Integration Points**

### **Builds On**
- [[Traits]] - Foundation for polymorphism in Rust
- [[ownership-fundamentals]] - Memory model that makes composition efficient
- [[Generic Programming]] - Static dispatch and monomorphization

### **Enables**
- [[Trait Objects]] - Deep dive into dynamic dispatch mechanics
- [[state-pattern-rust]] - OOP design pattern adapted to Rust (planned)
- [[trait-objects-polymorphism]] - Advanced trait object patterns (planned)

### **Related Concepts**
- [[Rust Trinity - Struct Trait Impl]] - The three building blocks of Rust types
- [[rust_book/rust-book-ch10]] - Generics, traits, and lifetimes
- [[API Design Patterns]] - Public interface design using these concepts

### **Chapter Reference**
- [[rust_book/Ch18/README|Chapter 18 README]] - Complete OOP chapter with examples

---

*Tags: #concept #rust-book #oop #traits #polymorphism #intermediate*

*Links: [[zettel-index]] | [[rust-concepts-MOC]] | [[Trait Objects]] | [[Traits]] | [[rust_book/Ch18/README]] | [[async-trait-objects]]*
