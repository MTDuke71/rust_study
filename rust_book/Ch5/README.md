# Chapter 5: Using Structs to Structure Related Data

This directory demonstrates Rust's **struct** system - how to define custom data types to encapsulate related data and behavior.

## 📦 What Are Structs?

**Structs** are custom data types that let you name and package together multiple related values that make up a meaningful group. They're similar to classes in other languages but without inheritance.

## 🏗️ Chapter 5 Structure

This chapter progresses through three key concepts:

```
Ch5/
├── structs/              # 5.1: Defining and Instantiating Structs
│   ├── Cargo.toml
│   └── src/
│       └── main.rs       # Basic struct syntax and usage
│
├── rectangles/           # 5.2: Example Program Using Structs
│   ├── Cargo.toml
│   └── src/
│       └── main.rs       # Refactoring with structs and debug output
│
├── method_syntax/        # 5.3: Method Syntax and Associated Functions
│   ├── Cargo.toml
│   ├── README.md         # Detailed method syntax guide
│   ├── examples/
│   └── src/
│       └── main.rs       # Methods, &self, and :: syntax
│
└── README.md             # This file
```

## 🔍 Key Concepts Covered

### 5.1: Defining and Instantiating Structs
- **Classic Structs**: Named fields with types
- **Tuple Structs**: Named types without field names
- **Unit-Like Structs**: Zero-size types for type safety
- **Field Init Shorthand**: Ergonomic initialization
- **Struct Update Syntax**: Creating instances from other instances

```rust
// Classic struct
struct User {
    username: String,
    email: String,
    active: bool,
}

// Tuple struct
struct Point(i32, i32, i32);

// Unit-like struct (marker type)
struct AlwaysEqual;
```

### 5.2: Example Program Using Structs
- **Refactoring** primitive types into meaningful structs
- **Derive Traits**: `#[derive(Debug)]` for printable structs
- **Debug Output**: Using `{:?}` and `{:#?}` for struct inspection
- **Real-World Design**: Building a rectangle area calculator

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("rect is {:#?}", rect);  // Pretty-print debug output
}
```

### 5.3: Method Syntax
- **Methods**: Functions that operate on struct instances
- **&self, &mut self, self**: Three ownership patterns
- **Associated Functions**: Like static methods (e.g., `String::new()`)
- **Method Chaining**: Fluent APIs with method calls
- **Multiple impl Blocks**: Organizing implementations

```rust
impl Rectangle {
    // Method: takes &self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method: takes &mut self
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    
    // Method: takes self (consumes instance)
    fn into_square(self) -> Rectangle {
        let size = self.width.max(self.height);
        Rectangle { width: size, height: size }
    }
    
    // Associated function: no self
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

// Usage:
let rect = Rectangle::square(10);      // Associated function with ::
let area = rect.area();                 // Method with .
```

## 🚀 How to Run Examples

### Run the structs example:
```bash
cd structs
cargo run
```

### Run the rectangles example:
```bash
cd rectangles
cargo run
```

### Run the method syntax examples:
```bash
cd method_syntax
cargo run

# Run specific examples:
cargo run --example ownership_patterns
cargo run --example method_chaining
cargo run --example associated_functions
```

## 📖 Learning Objectives

After studying Chapter 5, you should understand:

1. **Struct Definition**: How to create custom types with named fields
2. **Instantiation**: Multiple ways to create struct instances
3. **Ownership in Structs**: How structs own their data
4. **Debug Output**: Using derive macros for printable types
5. **Method Syntax**: The difference between methods and associated functions
6. **&self Patterns**: When to use `&self`, `&mut self`, or `self`
7. **API Design**: Creating ergonomic interfaces with methods

## 🔗 Mission Integration

### **Structs Power These Missions:**

**Mission1 - Stack**
- Struct wrapping `Vec<T>` with custom API
- Methods: `push()`, `pop()`, `peek()`
- Associated function: `Stack::new()`

**Mission2 - Queue** 
- Ring buffer struct with circular indexing
- Methods managing front/back pointers
- Capacity tracking with struct fields

**Mission3 - Binary Search**
- Struct-based search implementations
- Methods for slices and iterators
- Associated functions for entry points

**Mission4 - Linked Lists**
- Complex structs with `Box<Node<T>>`
- Methods navigating ownership chains
- Interior mutability patterns

**Mission5 - HashMap**
- Multi-field struct (buckets, len, hasher)
- Rich method API (`insert`, `get`, `remove`)
- Associated functions (`new`, `with_capacity`)

**All Missions**: Every mission uses structs as the foundation!

## 💡 Real-World Patterns

### Pattern 1: Builder Pattern with Associated Functions
```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    
    fn square(size: u32) -> Self {
        Self::new(size, size)
    }
}
```

### Pattern 2: Method Chaining
```rust
impl Rectangle {
    fn scale(mut self, factor: u32) -> Self {
        self.width *= factor;
        self.height *= factor;
        self
    }
    
    fn double(self) -> Self {
        self.scale(2)
    }
}

// Usage:
let rect = Rectangle::new(10, 20)
    .scale(2)
    .double();
```

### Pattern 3: Getter Methods
```rust
impl Rectangle {
    fn width(&self) -> u32 {
        self.width
    }
    
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}
```

## 🎓 Advanced Concepts (Preview)

Chapter 5 sets the stage for:
- **Traits** (Chapter 10): Adding behavior to structs
- **Generics** (Chapter 10): Making structs work with any type
- **Lifetimes** (Chapter 10): Managing references in structs
- **Error Handling** (Chapter 9): Result types using enums and structs

## 📊 Progression Path

```
Ch5 Structs → Ch6 Enums → Ch7 Modules → Ch8 Collections
     ↓           ↓             ↓              ↓
  Custom     Pattern      Organization   Real Data
   Types     Matching     & Privacy    Structures
```

## 🔗 Related Concepts

### **Zettelkasten Deep Dives**
- [[Struct Fundamentals]] - Core struct concepts and patterns
- [[API Design Patterns]] - Method naming and interface design
- [[Ownership Methods]] - Understanding &self patterns
- [[Collections MOC]] - Structs in data structure implementations

### **Mission Applications**
- [[Mission1 Overview]] - Stack struct with Vec wrapper
- [[Mission2 Overview]] - Ring buffer struct design
- [[Mission5 Overview]] - HashMap struct with complex internals

### **Rust Book Connections**
- [[Chapter 6 - Enums]] - Complementary to structs for sum types
- [[Chapter 10 - Generics]] - Making structs type-flexible
- [[../Ch7/crates/README|Chapter 7 - Packages and Crates]] - Organizing struct-based code

## 💡 Key Takeaways

1. **Structs = Custom Types**: Bundle related data into meaningful units
2. **Methods = Behavior**: Add functionality to structs with impl blocks
3. **&self Patterns**: Choose ownership based on operation needs
4. **Associated Functions**: Create constructors and utilities
5. **Derive Macros**: Get functionality for free with `#[derive(...)]`
6. **Ergonomic APIs**: Use method syntax for clean, readable code

**Chapter 5 Philosophy:**
> "Structs are the foundation of Rust's type system. Master them, and you can model any domain effectively with type safety and zero-cost abstractions." 🦀

---

## 🧪 Practice Exercises

1. Create a `Circle` struct and implement `area()` and `circumference()` methods
2. Build a `Person` struct with methods for full name formatting
3. Implement a `Temperature` struct with conversion methods (Celsius ↔ Fahrenheit)
4. Design a `BankAccount` struct with deposit/withdraw methods
5. Create a `Point3D` struct with distance calculation methods

---

*Tags: #rust-book #chapter5 #structs #methods #impl #api-design #custom-types #ownership*

*Links: [[../../zettelkasten/zettel-index|Zettelkasten Index]] | [[../../zettelkasten/Rust Concepts MOC|Rust Concepts]] | [[../../zettelkasten/Missions Overview|Missions Overview]] | [[../../zettelkasten/Mission1 Overview|Mission1]] | [[../../zettelkasten/Mission5 Overview|Mission5]]*
