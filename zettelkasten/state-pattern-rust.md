# State Pattern in Rust

**Two approaches to implementing state machines: OOP trait objects vs Type States**

> **Key Insight**: Rust offers a unique choice - traditional OOP state pattern for runtime flexibility, or type states for compile-time safety. The "Rusty" approach often favors type states.

---

## 🎯 **The Problem: Modeling State Machines**

Many domains require objects that behave differently based on their current state:
- **Blog posts**: Draft → PendingReview → Published
- **TCP connections**: Closed → Listen → Established → Closed
- **Payment processing**: Pending → Authorized → Captured → Refunded
- **User verification**: Unverified → EmailVerified → FullyVerified

Traditional OOP uses the **State Pattern** with polymorphic state objects. Rust offers an alternative: **Type States**.

---

## 🔄 **Approach 1: OOP State Pattern (Runtime)**

Uses trait objects for runtime state transitions.

### **Implementation**

```rust
/// State trait - common interface for all states
pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str { "" }
    fn can_add_text(&self) -> bool { false }
}

/// Concrete states
pub struct Draft {}
pub struct PendingReview {}
pub struct Published {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self  // Can't approve a draft
    }
    
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self  // Already in draft
    }
    
    fn can_add_text(&self) -> bool { true }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self  // Already in review
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> { self }
    fn approve(self: Box<Self>) -> Box<dyn State> { self }
    fn reject(self: Box<Self>) -> Box<dyn State> { self }
    
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

/// Context object holds current state
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    
    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().map(|s| s.can_add_text()).unwrap_or(false) {
            self.content.push_str(text);
        }
    }
    
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
    
    pub fn content(&self) -> &str {
        self.state.as_ref()
            .map(|s| s.content(self))
            .unwrap_or("")
    }
}
```

### **Trade-offs**

| Aspect | Evaluation |
|--------|------------|
| **Flexibility** | ✅ States can be changed at runtime |
| **Extensibility** | ✅ Add new states without modifying existing code |
| **Collection storage** | ✅ Can store mixed states in `Vec<Box<dyn State>>` |
| **Error detection** | ⚠️ Invalid transitions fail silently (return self) |
| **Performance** | ⚠️ Box allocation + vtable lookup overhead |
| **Type safety** | ❌ Calling wrong method just does nothing |

---

## 📐 **Approach 2: Type States (Compile-Time)**

Encode states as distinct types - invalid transitions won't compile.

### **Implementation**

```rust
/// Each state is a separate type
pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

pub struct Post {
    content: String,
}

/// Factory function
impl Post {
    pub fn new() -> DraftPost {
        DraftPost { content: String::new() }
    }
    
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// DraftPost methods - only drafts can be edited
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    
    /// Consumes DraftPost, returns PendingReviewPost
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { content: self.content }
    }
}

/// PendingReviewPost methods - can approve or reject
impl PendingReviewPost {
    /// Consumes self, returns Published Post
    pub fn approve(self) -> Post {
        Post { content: self.content }
    }
    
    /// Consumes self, returns back to DraftPost
    pub fn reject(self) -> DraftPost {
        DraftPost { content: self.content }
    }
}

// Usage:
fn workflow() {
    let mut post = Post::new();       // DraftPost
    post.add_text("Hello world");
    
    let post = post.request_review(); // PendingReviewPost
    // post.add_text("more");          // ❌ WON'T COMPILE - no such method
    
    let post = post.approve();        // Post
    println!("{}", post.content());
}
```

### **Compile-Time Safety**

```rust
// These are COMPILE ERRORS, not runtime failures:

let draft = Post::new();
draft.approve();         // ❌ Error: no method `approve` for DraftPost
draft.content();         // ❌ Error: no method `content` for DraftPost

let pending = draft.request_review();
pending.add_text("x");   // ❌ Error: no method `add_text` for PendingReviewPost

let published = pending.approve();
published.add_text("x"); // ❌ Error: no method `add_text` for Post
published.approve();     // ❌ Error: no method `approve` for Post
```

### **Trade-offs**

| Aspect | Evaluation |
|--------|------------|
| **Safety** | ✅ Invalid transitions are compile errors |
| **Performance** | ✅ Zero runtime overhead (no Box, no vtable) |
| **Documentation** | ✅ Types self-document valid operations |
| **Extensibility** | ⚠️ Adding states requires modifying transition code |
| **Collection storage** | ❌ Cannot store mixed states together |
| **Runtime decisions** | ❌ State must be known at compile time |

---

## ⚖️ **Comparison Summary**

| Criterion | OOP State Pattern | Type States |
|-----------|-------------------|-------------|
| **Invalid transition** | Silent no-op or runtime error | Compile error |
| **Performance** | Box + vtable overhead | Zero-cost |
| **Mixed collections** | ✅ `Vec<Box<dyn State>>` | ❌ Not possible |
| **Add new states** | Easy (new struct + impl) | Modify multiple impls |
| **Runtime flexibility** | ✅ Change states dynamically | ❌ Static only |
| **Code complexity** | More boilerplate (trait impls) | Simpler per-state impls |
| **Rust idiom** | OOP-familiar | "Rusty" approach |

---

## 🔧 **Advanced: Two-Approval Requirement**

Type states can encode complex business rules:

```rust
pub struct PendingReviewPost {
    content: String,
    approvals: u32,
}

impl PendingReviewPost {
    /// Returns Ok(Post) if 2+ approvals, Err(self) otherwise
    pub fn approve(mut self) -> Result<Post, PendingReviewPost> {
        self.approvals += 1;
        
        if self.approvals >= 2 {
            Ok(Post { content: self.content })
        } else {
            Err(self)  // Still needs more approvals
        }
    }
}

// Usage:
let pending = draft.request_review();
let pending = pending.approve().unwrap_err();  // First approval
let published = pending.approve().unwrap();    // Second approval
```

---

## 🎯 **When to Use Each**

### **Use OOP State Pattern when:**

- States need to be stored in heterogeneous collections
- State transitions depend on runtime data
- Plugin/extensibility is important
- Porting from OOP codebase

### **Use Type States when:**

- Invalid transitions should be compile errors
- Performance is critical (no allocation overhead)
- State flow is well-defined and static
- You want self-documenting APIs
- Building domain-specific type-safe APIs

---

## 📚 **Real-World Applications**

### **Type State Favorites**

- **HTTP Request Builders**: `NoMethod → HasMethod → HasUrl → Ready`
- **Connection Handles**: `Disconnected → Connected → Authenticated`
- **File Handles**: `Unopened → Open → Closed`
- **Transaction Processing**: `Pending → Authorized → Captured`

### **OOP State Pattern Favorites**

- **Game AI**: Different behavior modes stored in collections
- **UI Components**: Widget states that change based on user input
- **Protocol Handlers**: States determined by incoming messages
- **Plugin Systems**: Extensible state machines

---

## 🔗 **Related Concepts**

### **Chapter 18 Content**
- [[rust-book-ch18]] - OOP features in Rust
- [[trait-objects-polymorphism]] - Foundation for OOP state pattern

### **Patterns**
- [[Generic Programming]] - PhantomData for type state markers
- [[API Design Patterns]] - Type-state builder pattern
- [[mission-composition-patterns]] - Composing validated components

### **Exercises**
- [[../../rust_book/Ch18/src/exercises.rs]] - Exercise 6 (User Registration) and Exercise 7 (HTTP Builder) implement both approaches

### **Performance**
- [[Performance Optimization Guide]] - Static vs dynamic dispatch trade-offs

---

## 📖 **Code Examples**

| Example | Location | Description |
|---------|----------|-------------|
| OOP State Pattern | `rust_book/Ch18/examples/ch18_3_state_pattern.rs` | Traditional approach with trait objects |
| Type States | `rust_book/Ch18/examples/ch18_3_type_states.rs` | Compile-time state validation |
| User Registration | `rust_book/Ch18/src/exercises.rs` (Ex 6) | Both approaches side-by-side |
| HTTP Builder | `rust_book/Ch18/src/exercises.rs` (Ex 7) | Type-safe builder pattern |

---

*Tags: #state-pattern #type-states #oop #design-patterns #compile-time-safety #trait-objects #ch18*

*Links: [[rust-book-ch18]] | [[trait-objects-polymorphism]] | [[Generic Programming]] | [[API Design Patterns]] | [[Performance Optimization Guide]]*
