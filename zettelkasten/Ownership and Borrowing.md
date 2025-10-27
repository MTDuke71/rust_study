# Ownership and Borrowing

> **Core Rust Concepts** - Memory safety without garbage collection

## 📚 Overview

Ownership and Borrowing are Rust's fundamental memory safety mechanisms that eliminate data races and memory leaks at compile time, without requiring garbage collection.

## 🎯 Core Principles

### **Ownership Rules**
1. **Each value has one owner**
2. **Only one owner at a time**
3. **Value is dropped when owner goes out of scope**

### **Borrowing Rules**
1. **Multiple immutable borrows OR one mutable borrow**
2. **References must always be valid**
3. **Cannot borrow while value is moved**

## 🔑 Ownership Fundamentals

### **What is Ownership?**
Ownership is Rust's system for managing memory through compile-time checks that ensure memory safety without runtime overhead.

```rust
// Ownership transfer (move)
let s1 = String::from("hello");
let s2 = s1; // s1 is no longer valid
// println!("{}", s1); // ❌ Compile error: value moved

// Copy types don't move
let x = 5;
let y = x; // x is still valid (Copy trait)
println!("{}", x); // ✅ Works fine
```

### **Scope and Drop**
```rust
{
    let s = String::from("hello"); // s comes into scope
    // do stuff with s
} // s goes out of scope, drop() is called automatically
```

### **Move Semantics**
```rust
fn takes_ownership(s: String) { // s takes ownership
    println!("{}", s);
} // s goes out of scope and is dropped

let s = String::from("hello");
takes_ownership(s); // s is moved into the function
// s is no longer valid here
```

## 🔄 Borrowing Fundamentals

### **What is Borrowing?**
Borrowing allows you to reference a value without taking ownership, using references (`&T` and `&mut T`).

```rust
fn calculate_length(s: &String) -> usize { // s is a reference
    s.len()
} // s goes out of scope, but nothing is dropped

let s1 = String::from("hello");
let len = calculate_length(&s1); // s1 is borrowed, not moved
println!("{} has length {}", s1, len); // s1 is still valid
```

### **Mutable References**
```rust
fn change(s: &mut String) {
    s.push_str(", world");
}

let mut s = String::from("hello");
change(&mut s);
println!("{}", s); // "hello, world"
```

### **Borrowing Rules in Action**
```rust
let mut s = String::from("hello");

let r1 = &s; // ✅ OK
let r2 = &s; // ✅ OK - multiple immutable borrows
println!("{} and {}", r1, r2);

let r3 = &mut s; // ❌ Error - cannot borrow as mutable
// while immutable borrows are active
```

## 🧠 Mental Models

### **Ownership as Resource Management**
- **Think of ownership like a key to a house**
- **Only one person can have the key at a time**
- **When you give the key away, you can't use the house anymore**

### **Borrowing as Temporary Access**
- **Think of borrowing like borrowing a book from a library**
- **You can read it, but you don't own it**
- **Multiple people can read the same book (immutable borrows)**
- **Only one person can write in it at a time (mutable borrow)**

> **💡 Deep Dive**: For a comprehensive mental model using the library analogy, see [[Ownership Mental Model - The Library Analogy]]

### **Scope as Lifecycle**
- **Values are born when they come into scope**
- **Values live as long as they're in scope**
- **Values die (are dropped) when they go out of scope**

## 📖 Common Patterns

### **Returning Ownership**
```rust
fn gives_ownership() -> String {
    let s = String::from("hello");
    s // s is returned and ownership is transferred
}

fn takes_and_gives_back(s: String) -> String {
    s // s is returned and ownership is transferred
}
```

### **Borrowing for Read-Only Access**
```rust
fn print_string(s: &String) {
    println!("{}", s);
    // s is borrowed, not owned, so it's not dropped
}
```

### **Mutable Borrowing for Modification**
```rust
fn modify_string(s: &mut String) {
    s.push_str(" modified");
    // s is mutably borrowed, so it can be modified
}
```

### **String Slices (Special Case)**
```rust
let s = String::from("hello world");
let hello = &s[0..5]; // slice is a reference to part of the string
let world = &s[6..11];
// s is still valid, slices are just references
```

## ⚡ Performance Implications

### **Zero-Cost Abstractions**
- **No runtime overhead** for ownership checks
- **Compile-time guarantees** eliminate need for garbage collection
- **Predictable memory usage** without reference counting overhead

### **Memory Safety Benefits**
- **No null pointer dereferences**
- **No use-after-free errors**
- **No data races in multithreaded code**
- **No memory leaks** (with proper resource management)

## 🔗 Related Concepts

### **Core Ownership Concepts**
- **[[Move Semantics]]** - Understanding value movement
- **[[Copy vs Clone]]** - When values are copied vs moved
- **[[Memory Safety]]** - Compile-time safety guarantees

### **Advanced Ownership Patterns**
- **[[Smart Pointers]]** - Rc<T>, RefCell<T>, Box<T>
- **[[Interior Mutability]]** - Mutable data in immutable contexts
- **[[Lifetimes]]** - Ensuring references stay valid

### **Collection Integration**
- **[[Vec]]** - Dynamic arrays and ownership
- **[[HashMap]]** - Key-value storage and borrowing
- **[[String]]** - String ownership and borrowing patterns

## 📚 Integration Points

### **Mission Applications**
- **[[Mission1 Overview]]** - Foundation building with ownership
- **[[Mission2 Overview]]** - Smart pointers and advanced ownership
- **[[Mission4 Overview]]** - Interior mutability patterns

### **Daily Study Progression**
- **[[daily-study/Day02]]** - Ownership basics (previous)
- **[[daily-study/Day03]]** - Borrowing rules (next)
- **[[daily-study/Day04]]** - Lifetimes (advanced)

### **Rust Book Integration**
- **[[Chapter 4 Overview]]** - Ownership and borrowing chapter
- **[[zettelkasten/rust_book/rust-book-ch8]]** - Collections and ownership
- **[[zettelkasten/rust_book/rust-book-ch10]]** - Generics and ownership

## 🧪 Common Pitfalls and Solutions

### **Pitfall 1: Moving Borrowed Values**
```rust
let s = String::from("hello");
let r = &s;
let s2 = s; // ❌ Error: cannot move s while borrowed
```

**Solution**: Use the reference before moving
```rust
let s = String::from("hello");
let r = &s;
println!("{}", r); // Use reference
let s2 = s; // ✅ Now it's OK to move
```

### **Pitfall 2: Dangling References**
```rust
fn dangle() -> &String { // ❌ Error: cannot return reference to local
    let s = String::from("hello");
    &s
}
```

**Solution**: Return ownership instead
```rust
fn no_dangle() -> String { // ✅ Return ownership
    let s = String::from("hello");
    s
}
```

### **Pitfall 3: Borrowing After Move**
```rust
let s = String::from("hello");
let s2 = s;
println!("{}", s); // ❌ Error: s was moved
```

**Solution**: Use references or clone
```rust
let s = String::from("hello");
let s2 = s.clone(); // Clone instead of move
println!("{}", s); // ✅ s is still valid
```

## 💡 Key Takeaways

1. **Ownership is Rust's memory safety system** - no garbage collection needed
2. **Borrowing allows temporary access** without taking ownership
3. **The borrow checker enforces rules at compile time** - no runtime overhead
4. **Move semantics transfer ownership** - values can only have one owner
5. **References must always be valid** - no dangling pointers possible
6. **Scope determines lifetime** - values are dropped when they go out of scope

## 🔄 Next Steps

- **[[daily-study/Day03]]** - Deep dive into borrowing rules
- **[[daily-study/Day04]]** - Lifetimes and reference validity
- **[[Smart Pointer Patterns]]** - Advanced ownership patterns
- **[[Mission1 Overview]]** - Practical ownership applications

---

> **💡 Tip**: Ownership and Borrowing are Rust's superpowers. Once you understand these concepts, you'll have a solid foundation for writing safe, efficient Rust code.
