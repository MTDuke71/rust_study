# 🏗️ Rust Trinity - Struct, Trait, and Impl

**Understanding Rust's three fundamental building blocks: data, behavior contracts, and implementations**

## 🎯 Core Concept

Rust's type system is built on three essential components that work together to create flexible, reusable code:

1. **`struct`** - Data containers (what you have)
2. **`trait`** - Behavior contracts (what you must do)
3. **`impl`** - Implementations (how you do it)

## 📦 Struct - Data Container

### **Purpose: Define What Data a Type Holds**

```rust
// Basic struct
struct Circle {
    radius: f64,
    color: String,
}

struct Square {
    side: f64,
    color: String,
}

struct Rectangle {
    width: f64,
    height: f64,
    color: String,
}
```

### **Key Characteristics:**
- **Data storage** - holds fields/variables
- **Custom types** - define your own data structures
- **Memory layout** - determines how data is stored
- **Ownership** - defines who owns the data

### **Struct Patterns:**
```rust
// Tuple struct
struct Point(f64, f64);

// Unit struct
struct Empty;

// Generic struct
struct Container<T> {
    data: T,
    count: usize,
}
```

## 📋 Trait - Behavior Contract

### **Purpose: Define What Behaviors a Type Must Have**

```rust
trait Drawable {
    fn draw(&self) -> String;
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

trait Cloneable {
    fn clone_self(&self) -> Self;
}

trait Describable {
    fn describe(&self) -> String;
}
```

### **Key Characteristics:**
- **Method signatures** - defines what methods must exist
- **No implementation** - just the contract
- **Multiple implementers** - many types can implement one trait
- **Composable** - types can implement multiple traits

### **Trait Patterns:**
```rust
// Simple trait
trait Draw {
    fn draw(&self);
}

// Trait with default implementation
trait Draw {
    fn draw(&self);
    fn draw_with_color(&self) {  // Default implementation
        println!("Drawing with default color");
        self.draw();
    }
}

// Trait with associated types
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

## 🔧 Impl - Implementation

### **Purpose: Define How a Type Implements Required Behavior**

```rust
impl Drawable for Circle {
    fn draw(&self) -> String {
        format!("Circle with radius {} and color {}", self.radius, self.color)
    }
    
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Drawable for Square {
    fn draw(&self) -> String {
        format!("Square with side {} and color {}", self.side, self.color)
    }
    
    fn area(&self) -> f64 {
        self.side * self.side
    }
    
    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }
}
```

### **Key Characteristics:**
- **Method bodies** - actual implementation code
- **Type-specific** - each type implements traits differently
- **Required** - must implement all trait methods
- **Flexible** - can add additional methods beyond trait requirements

## 🔄 The Complete Pattern

### **Step 1: Define the Data (Struct)**
```rust
struct Circle { radius: f64, color: String }
struct Square { side: f64, color: String }
struct Rectangle { width: f64, height: f64, color: String }
```

### **Step 2: Define the Behavior Contract (Trait)**
```rust
trait Drawable {
    fn draw(&self) -> String;
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}
```

### **Step 3: Implement the Behavior (Impl)**
```rust
impl Drawable for Circle { /* Circle-specific implementation */ }
impl Drawable for Square { /* Square-specific implementation */ }
impl Drawable for Rectangle { /* Rectangle-specific implementation */ }
```

### **Step 4: Use with Polymorphism**
```rust
let shapes: Vec<Box<dyn Drawable>> = vec![
    Box::new(Circle { radius: 5.0, color: "red".to_string() }),
    Box::new(Square { side: 4.0, color: "blue".to_string() }),
    Box::new(Rectangle { width: 3.0, height: 6.0, color: "green".to_string() }),
];

for shape in shapes {
    println!("{}", shape.draw());
    println!("Area: {}", shape.area());
    println!("Perimeter: {}", shape.perimeter());
}
```

## 🎯 Real-World Analogy

### **Think of it like a Restaurant:**

**`struct`** = **Menu Items** (what ingredients you have)
```rust
struct Pizza { dough: String, cheese: String, toppings: Vec<String> }
struct Salad { greens: String, dressing: String, extras: Vec<String> }
```

**`trait`** = **Cooking Standards** (what every dish must do)
```rust
trait Cookable {
    fn cook(&self) -> String;
    fn prep_time(&self) -> u32;
    fn calories(&self) -> u32;
}
```

**`impl`** = **Actual Recipes** (how you make each dish)
```rust
impl Cookable for Pizza {
    fn cook(&self) -> String { "Bake at 450°F for 15 minutes".to_string() }
    fn prep_time(&self) -> u32 { 20 }
    fn calories(&self) -> u32 { 800 }
}

impl Cookable for Salad {
    fn cook(&self) -> String { "Toss ingredients together".to_string() }
    fn prep_time(&self) -> u32 { 5 }
    fn calories(&self) -> u32 { 200 }
}
```

## 🔗 Multiple Traits per Type

### **One Struct, Multiple Behaviors**
```rust
struct Circle { radius: f64, color: String }

// Implement multiple traits for the same struct
impl Drawable for Circle { /* ... */ }
impl Clone for Circle { /* ... */ }
impl Debug for Circle { /* ... */ }
impl Describable for Circle { /* ... */ }
```

### **One Trait, Multiple Types**
```rust
trait Drawable { fn draw(&self) -> String; }

impl Drawable for Circle { /* ... */ }
impl Drawable for Square { /* ... */ }
impl Drawable for Triangle { /* ... */ }
impl Drawable for Hexagon { /* ... */ }
```

## 🎯 Key Differences

| **Aspect** | **`struct`** | **`trait`** | **`impl`** |
|------------|--------------|-------------|------------|
| **Purpose** | Define data | Define behavior contract | Implement behavior |
| **Contains** | Fields/variables | Method signatures | Method bodies |
| **When** | Define once | Define once | Implement for each type |
| **Reusability** | One per type | Many types can implement | One per type-trait pair |
| **Example** | `struct Circle { radius: f64 }` | `trait Drawable { fn draw(&self); }` | `impl Drawable for Circle { ... }` |

## 🚀 Advanced Patterns

### **Generic Structs with Traits**
```rust
struct Container<T: Drawable> {
    item: T,
    id: usize,
}

impl<T: Drawable> Container<T> {
    fn new(item: T, id: usize) -> Self {
        Container { item, id }
    }
    
    fn draw_item(&self) -> String {
        format!("Container {}: {}", self.id, self.item.draw())
    }
}
```

### **Trait Bounds**
```rust
fn process_drawable<T: Drawable>(item: T) -> String {
    format!("Processing: {}", item.draw())
}

// Or with where clause
fn process_drawable<T>(item: T) -> String 
where 
    T: Drawable,
{
    format!("Processing: {}", item.draw())
}
```

### **Trait Objects**
```rust
// Dynamic dispatch with trait objects
let shapes: Vec<Box<dyn Drawable>> = vec![
    Box::new(Circle { radius: 5.0, color: "red".to_string() }),
    Box::new(Square { side: 4.0, color: "blue".to_string() }),
];

for shape in shapes {
    println!("{}", shape.draw());  // Dynamic dispatch
}
```

## 🧪 Practice Exercises

### **Exercise 1: Vehicle System**
```rust
// Define vehicle data
struct Car { model: String, year: u32 }
struct Truck { model: String, year: u32, cargo_capacity: f64 }

// Define vehicle behavior
trait Vehicle {
    fn start(&self) -> String;
    fn stop(&self) -> String;
    fn info(&self) -> String;
}

// Implement behavior for each vehicle type
impl Vehicle for Car { /* ... */ }
impl Vehicle for Truck { /* ... */ }
```

### **Exercise 2: Animal System**
```rust
// Define animal data
struct Dog { name: String, breed: String }
struct Cat { name: String, color: String }

// Define animal behavior
trait Animal {
    fn make_sound(&self) -> String;
    fn move_around(&self) -> String;
}

// Implement behavior
impl Animal for Dog { /* ... */ }
impl Animal for Cat { /* ... */ }
```

## 📚 Key Takeaways

1. **`struct`** defines **what data** a type holds
2. **`trait`** defines **what behaviors** a type must have
3. **`impl`** defines **how** a type implements those behaviors
4. **Separation of concerns** - data and behavior are separate
5. **Polymorphism** - multiple types can implement the same trait
6. **Composability** - one type can implement multiple traits
7. **Flexibility** - easy to add new types or new behaviors

## 🔄 OOP Equivalent Concepts

### **Rust vs Object-Oriented Programming**

| **Rust** | **OOP Equivalent** | **Purpose** |
|----------|-------------------|-------------|
| `struct` | **Class** | Define data structure |
| `trait` | **Interface** | Define behavior contract |
| `impl` | **Class Implementation** | Implement interface methods |

### **Key Differences from OOP:**

**Rust's Separation of Concerns:**
- **Data** (struct) and **behavior** (trait) are **separate**
- **Multiple traits** can be implemented for one struct
- **One trait** can be implemented by **multiple structs**
- **Composition over inheritance**

**OOP's Unified Approach:**
- **Data** and **behavior** are **combined** in classes
- **Inheritance** for code reuse
- **Single inheritance** (usually) with interfaces for multiple behaviors

### **Polymorphism Comparison:**

**Rust Trait Objects:**
```rust
let shapes: Vec<Box<dyn Drawable>> = vec![
    Box::new(Circle { radius: 5.0 }),
    Box::new(Square { side: 4.0 }),
];

for shape in shapes {
    println!("{}", shape.draw());  // Dynamic dispatch
}
```

**OOP Equivalent (Java):**
```java
List<Drawable> shapes = Arrays.asList(
    new Circle(5.0),
    new Square(4.0)
);

for (Drawable shape : shapes) {
    System.out.println(shape.draw());  // Dynamic dispatch
}
```

### **Rust's Advantages:**
- **Zero-cost abstractions** - compile-time polymorphism
- **Memory safety** - no null pointers, no data races
- **Flexible composition** - multiple traits per struct
- **Better performance** - compile-time optimizations

## 🔗 Related Concepts

### **Ownership & Memory**
- [[Day 02 - Ownership Basics]] - How structs own their data
- [[Box Smart Pointer Patterns]] - Heap allocation for trait objects

### **Advanced Type System**
- [[Day 15 - Traits Fundamentals]] - Deep dive into traits
- [[Day 16 - Generic Types]] - Generic structs and traits
- [[Day 18 - Advanced Traits]] - Associated types and defaults
- [[Day 19 - Trait Objects]] - Dynamic dispatch patterns

### **Collections & Data Structures**
- [[Collections MOC]] - How structs and traits work in collections
- [[Generic Programming]] - Reusable code patterns

---

*Tags: #struct #trait #impl #type-system #polymorphism #data-structures #behavior-contracts #rust-trinity*
*Links: [[Day 15 - Traits Fundamentals]] | [[Day 16 - Generic Types]] | [[Box Smart Pointer Patterns]] | [[Rust Concepts MOC]] | [[rust_book/Ch5/method_syntax/README]]*
