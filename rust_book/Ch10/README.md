# Chapter 10: Generic Types, Traits, and Lifetimes

## 🔗 Zettelkasten Links
- **Overview**: [[Chapter 10 Overview]]
- **Previous**: [[Chapter 9 Overview]]
- **Next**: [[Chapter 11 Overview]]
- **Missions**: [[Mission3 Overview]] - Trait-based algorithms | [[Mission5 Overview]] - Generic collections
- **Daily Study**: [[Day 15 - Traits]] | [[Day 16 - Generic Types]] | [[Day 17 - Lifetimes]]
- **Book MOC**: [[Rust Book MOC]]

## 📚 Overview

Chapter 10 introduces Rust's powerful system for writing reusable code through generics, traits, and lifetimes. These concepts enable you to write flexible, type-safe code that works with many different types while maintaining Rust's safety guarantees.

---

## 🎯 Key Concepts

### 1. **Generic Data Types**
Generics allow you to write code that works with multiple types without duplicating logic.

```rust
// Generic function
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

// Generic enum
enum Option<T> {
    Some(T),
    None,
}
```

### 2. **Traits: Defining Shared Behavior**
Traits define shared behavior that types can implement, similar to interfaces in other languages.

```rust
// Define a trait
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")  // Default implementation
    }
}

// Implement the trait
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Using trait bounds
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

### 3. **Lifetimes: Validating References**
Lifetimes ensure that references are valid for as long as we need them to be.

```rust
// Lifetime annotation in function
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime annotation in struct
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime elision rules
fn first_word(s: &str) -> &str {  // Compiler infers lifetimes
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

---

## 🔑 Key Takeaways

### Generic Benefits
- **Code Reuse** - Write once, use with many types
- **Type Safety** - Compile-time type checking
- **Performance** - Zero-cost abstractions (monomorphization)

### Trait Benefits
- **Polymorphism** - Different types with same interface
- **Code Organization** - Define behavior separately from data
- **Flexibility** - Default implementations and trait bounds

### Lifetime Benefits
- **Memory Safety** - Prevent dangling references
- **No Garbage Collection** - Explicit lifetime management
- **Zero Runtime Cost** - All checks at compile time

### Best Practices
- **Use generics** when you need to work with multiple types
- **Define traits** for shared behavior across types
- **Lifetime annotations** only when the compiler can't infer them
- **Trait bounds** to constrain generic types to specific behaviors

---

## 🛠️ Common Patterns

### Generic Function with Trait Bounds
```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

### Trait Objects for Dynamic Dispatch
```rust
pub trait Draw {
    fn draw(&self);
}

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
```

### Generic Struct with Lifetime
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

### Multiple Trait Bounds
```rust
use std::fmt::Display;

fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
    println!("Item: {}", item);
}

// Alternative syntax with where clause
fn some_function<T, U>(t: &T, u: &U) -> i32 
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // function body
}
```

### Associated Types in Traits
```rust
pub trait Iterator {
    type Item;  // Associated type

    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // implementation
    }
}
```

---

## 🧠 Mental Model

Think of **generics** as templates that get filled in with specific types at compile time. Just like a cookie cutter can make cookies in different shapes, generics let you write code that works with different types.

**Traits** are like contracts or interfaces. When a type implements a trait, it promises to provide certain functionality. This is similar to how different devices can all "play music" but do it in their own way.

**Lifetimes** are like expiration dates for references. The compiler tracks how long each reference lives and ensures you don't use a reference after it's expired. Think of it like borrowing a book from a library - you can use it, but you must return it before the due date.

Think of the relationship as:
- **Generics** = "This works with any type T"
- **Traits** = "Type T must be able to do X"
- **Lifetimes** = "References to T must be valid for duration 'a"

---

## 📖 Further Reading
- [The Rust Book Chapter 10](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [Rust Reference - Traits](https://doc.rust-lang.org/reference/traits.html)
- [Rust Reference - Lifetimes](https://doc.rust-lang.org/reference/lifetime-elision.html)
- [Rustonomicon - Subtyping and Variance](https://doc.rust-lang.org/nomicon/subtyping.html) (advanced)

---

## 🔗 Related Content

**Missions:**
- [[Mission3 Overview]] - Binary search with trait bounds
- [[Mission5 Overview]] - Generic HashMap implementation

**Daily Study:**
- [[Day 15 - Traits]] - Practical trait implementations
- [[Day 16 - Generic Types]] - Generic programming patterns
- [[Day 17 - Lifetimes]] - Lifetime management exercises

**Next Steps:**
- Complete exercises in `Ch10/generics/`, `Ch10/traits/`, and `Ch10/lifetimes/` directories
- Review [[Chapter 11 Overview]] when ready

---

*This chapter forms the foundation for writing reusable, type-safe Rust code. Essential for understanding Rust's type system and memory safety guarantees.*

*Links: [[Rust Book MOC]] | [[Chapter 9 Overview]] | [[Chapter 11 Overview]]*
*Tags: #rust-book #chapter10 #generics #traits #lifetimes #foundation*
