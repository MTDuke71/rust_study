# Chapter 18: Object-Oriented Programming Features of Rust

## 🔗 Zettelkasten Links
- **Overview**: [[zettelkasten/rust_book/rust-book-ch18]]
- **Previous**: [[zettelkasten/rust_book/rust-book-ch17]]
- **Next**: [[zettelkasten/rust_book/rust-book-ch19]]
- **Missions**: [[mission-5]] - Encapsulation with HashMap | [[mission-8]] - Trait objects for polymorphism
- **Daily Study**: [[daily-study/Day25]] | [[daily-study/Day26]]
- **Book MOC**: [[rust-book]]

## 📚 Overview

Chapter 18 explores how Rust implements Object-Oriented Programming (OOP) concepts. While Rust isn't a traditional OOP language, it provides powerful alternatives to achieve similar goals through its unique ownership system, traits, and type system.

**Official Reference**: https://doc.rust-lang.org/book/ch18-00-oop.html

---

## 🎯 Learning Objectives

By completing this chapter, you will understand:

1. **Characteristics of OOP** - What makes a language object-oriented and how Rust fits
2. **Encapsulation** - Using structs and modules to hide implementation details
3. **Inheritance via Trait Default Methods** - Code reuse through trait defaults
4. **Polymorphism with Trait Objects** - Dynamic dispatch using `dyn Trait`
5. **State Pattern** - Implementing OOP design patterns in Rust
6. **Type System Alternatives** - Encoding state in the type system vs runtime checks

**Integration Points**: This chapter connects to:
- **[[mission-5]]** - HashMap encapsulation patterns
- **[[mission-8]]** - Graph trait abstractions
- **[[daily-study/Day25]]** - Design patterns in Rust
- **[[zettelkasten/trait-objects-deep-dive]]** - Advanced trait object patterns

---

## 🎯 Chapter Concepts

### 18.1. Characteristics of Object-Oriented Languages

**Official Definition**: OOP languages typically share certain characteristics: objects (data + methods), encapsulation (hide implementation), and inheritance (code reuse and polymorphism).

**Practical Understanding**: Rust provides:
- **Objects**: Structs with data + `impl` blocks with methods
- **Encapsulation**: `pub` keyword controls visibility
- **No classical inheritance**: But has trait default methods and composition
- **Polymorphism**: Through trait objects and generics

**Key Examples**:

```rust
// Object with data and methods
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> Self {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    // Private method - implementation detail
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

**Common Mistakes**:
- **Mistake 1**: Expecting classical inheritance - Rust uses composition and traits instead
- **Mistake 2**: Making all fields public - defeats encapsulation benefits
- **Mistake 3**: Not using modules for larger encapsulation boundaries

**Integration**: Encapsulation pattern used in [[mission-5]] HashMap implementation.

---

### 18.2. Using Trait Objects That Allow for Values of Different Types

**Official Definition**: Trait objects allow you to store values of different types in the same collection by using dynamic dispatch through the `dyn Trait` syntax.

**Practical Understanding**: When you need runtime polymorphism (different types implementing the same trait), use trait objects. This enables:
- Heterogeneous collections
- Plugin systems
- GUI component hierarchies
- Strategy pattern implementations

**Key Examples**:

```rust
// Define a trait for drawing
pub trait Draw {
    fn draw(&self);
}

// Different types implementing Draw
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button: {} ({}x{})", self.label, self.width, self.height);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing select box ({}x{}) with {} options", 
                 self.width, self.height, self.options.len());
    }
}

// Screen holds heterogeneous components using trait objects
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Usage
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

**Trait Objects vs Generics**:

```rust
// Generic version - compile-time monomorphization
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
// Problem: Can only hold ONE type of component!

// Trait object version - runtime polymorphism
pub struct ScreenDynamic {
    pub components: Vec<Box<dyn Draw>>,
}
// Solution: Can hold MULTIPLE types of components
```

**Common Mistakes**:
- **Mistake 1**: Using trait objects when generics would work - trait objects have runtime cost
- **Mistake 2**: Forgetting object safety requirements - traits with generic methods can't be trait objects
- **Mistake 3**: Not using `Box<dyn Trait>` or `&dyn Trait` - trait objects need indirection

**Integration**: Trait objects enable [[mission-8]] polymorphic graph implementations.

---

### 18.3. Implementing an Object-Oriented Design Pattern

**Official Definition**: The State pattern is an OOP design pattern where an object's behavior changes based on its internal state. The state is encapsulated in separate state objects.

**Practical Understanding**: Rust can implement the State pattern in two ways:
1. **Traditional OOP approach**: State objects with trait objects (runtime checks)
2. **Type system approach**: States encoded as types (compile-time checks)

**Key Examples - Traditional State Pattern**:

```rust
// State trait
pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// Draft state
pub struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// PendingReview state
pub struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// Published state
pub struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

// Post with state
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}
```

**Type System Alternative**:

```rust
// Encode states as types
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

// Usage - compile-time state enforcement!
fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();
    // post.add_text() won't compile - wrong type!

    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
```

**Trade-offs**:

| **Approach** | **Pros** | **Cons** |
|--------------|----------|----------|
| **OOP State Pattern** | Flexible, runtime state changes, familiar to OOP developers | Runtime overhead, boilerplate, less type safety |
| **Type System States** | Compile-time safety, zero cost, invalid states impossible | Less flexible, must consume values for transitions |

**Common Mistakes**:
- **Mistake 1**: Always using OOP patterns - consider type system alternatives
- **Mistake 2**: Not leveraging Rust's type system for compile-time guarantees
- **Mistake 3**: Translating OOP designs directly without considering Rust idioms

**Integration**: State pattern examples inform [[mission-9]] state machine implementations.

---

## 🧪 **Exercises and Practice**

### **Basic Exercises**

Run examples with: `cargo run --example [example_name]`

1. **Exercise 1**: Encapsulation Fundamentals
   - **Goal**: Create encapsulated type with private fields
   - **Task**: Implement `AveragedCollection` with add/remove/average
   - **Validation**: Can't access internal state directly

2. **Exercise 2**: Trait Objects
   - **Goal**: Use trait objects for polymorphism
   - **Task**: Create GUI system with heterogeneous components
   - **Validation**: Different types in same collection

3. **Exercise 3**: OOP State Pattern
   - **Goal**: Implement traditional state pattern
   - **Task**: Build blog post workflow (Draft → Review → Published)
   - **Validation**: State transitions work correctly

4. **Exercise 4**: Type System States
   - **Goal**: Encode states in type system
   - **Task**: Reimplement blog post with type-based states
   - **Validation**: Invalid transitions don't compile

5. **Exercise 5**: Comparing Approaches
   - **Goal**: Understand trade-offs between OOP and type system
   - **Task**: Implement same logic both ways
   - **Validation**: Observe compile-time vs runtime differences

### **Integration Exercises**

Run tests with: `cargo test`

1. **Integration 1**: Apply to [[mission-5]] requirements
   - Encapsulate HashMap implementation details
   - Hide internal resizing logic

2. **Integration 2**: Use in [[mission-8]] graph algorithms
   - Trait objects for polymorphic graph types
   - Multiple graph implementations with same interface

---

## 🔗 **Cross-References**

### **Prerequisites**
- **[[rust_book/Ch10]]**: Traits and generics required for understanding trait objects
- **[[rust_book/Ch15]]**: Box<T> and trait objects often used together
- **[[daily-study/Day15]]**: Object-oriented concepts foundation

### **Applications**
- **[[mission-5]]**: Uses encapsulation for HashMap internals
- **[[mission-8]]**: Trait objects for graph polymorphism
- **[[advanced_examples/plugin-system]]**: Real-world trait object usage

### **Reinforcement**
- **[[zettelkasten/trait-objects-deep-dive]]**: Advanced trait object patterns
- **[[zettelkasten/state-pattern-rust]]**: State pattern variations
- **[[tutorials/Mission8_tut]]**: Polymorphic trait implementations

---

## 📊 **Chapter Summary**

### **Key Takeaways**

1. **Rust supports OOP concepts** through structs, traits, and encapsulation
2. **Encapsulation** is achieved with `pub` keyword and module boundaries
3. **Trait objects** (`Box<dyn Trait>`) enable runtime polymorphism
4. **State pattern** can be implemented traditionally or with type system
5. **Type system approach** often provides better compile-time guarantees
6. **Trade-offs exist** between OOP patterns and Rust idioms

### **OOP in Rust Decision Matrix**

```
Need runtime polymorphism?
├─ Yes, heterogeneous collection → Trait Objects (Box<dyn Trait>)
├─ No, single type at compile time → Generics (T: Trait)
└─ State management needed?
   ├─ Complex runtime transitions → OOP State Pattern
   └─ Simple compile-time safety → Type System States
```

### **Object Safety Requirements**

For a trait to be object-safe (usable as `dyn Trait`):
- No generic methods
- No `Self` in return types (except receivers)
- No associated const or types (except with bounds)

---

*Run all examples*: `cargo run --example ch18_[section]`  
*Run all tests*: `cargo test`  
*Check quality*: `cargo clippy -- -D warnings`
