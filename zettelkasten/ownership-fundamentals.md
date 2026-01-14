# Ownership Fundamentals

*The foundational concept that makes Rust unique: memory safety without garbage collection*

**Created**: 2025-11-23  
**Tags**: #ownership #memory-management #rust-fundamentals #borrowing #move-semantics

---

## 🎯 Core Concept

**Ownership** is Rust's central feature that manages memory automatically at compile time through a set of rules enforced by the compiler. It enables **memory safety without runtime overhead** by preventing common bugs like use-after-free, double-free, and data races **at compile time**.

### The Three Golden Rules of Ownership

1. **Each value has a single owner** - Only one variable owns each piece of data
2. **Only one owner at a time** - Ownership can be transferred but never duplicated
3. **Value is dropped when owner goes out of scope** - Automatic cleanup via RAII pattern

---

## 🧠 Mental Models

### Real-World Analogy: Document Ownership

```rust
// Think of data like a physical document with a deed
let document = String::from("Contract");  // You own the document
transfer_ownership(document);             // You gave it away (moved)
// Can't use document anymore - you don't own it!
```

**Real Life**: Like handing someone your only copy of a contract

- ✅ They have it now and can use it
- ❌ You can't use it anymore - you don't have it
- 🔄 They can give it back to you later

### Two Types of Data Behavior

#### 📋 Copy Types (Stack-Only)

Numbers, booleans, chars, tuples of Copy types

```rust
let x = 5;
let y = x;           // Value is COPIED
println!("{}", x);   // ✅ Still works! x is still valid
```

**Mental Model**: Like photocopying - you keep the original, they get a copy

#### 📄 Move Types (Heap-Allocated)  

String, Vec, Box, Rc, custom structs (by default)

```rust
let s1 = String::from("hello");
let s2 = s1;         // Ownership is MOVED
// println!("{}", s1); // ❌ ERROR! s1 is no longer valid
```

**Mental Model**: Like handing over your only copy - they have it, you don't

---

## 🔄 Move Semantics

### Variable Assignment

```rust
// Copy types - value duplicated
let x = 42;
let y = x;  // x copied to y
// Both x and y are valid ✅

// Move types - ownership transferred
let s1 = String::from("hello");
let s2 = s1;  // s1 moved to s2
// s1 is invalid ❌, s2 is valid ✅
```

### Why Moves Happen

**Stack data** (known size): Cheap to copy, so Copy trait is implemented
**Heap data** (variable size): Expensive to copy, so ownership moves instead

```rust
// Under the hood for String:
// Stack: [pointer, length, capacity]  ← This is copied
// Heap: ["hello"]                      ← This is NOT copied, just pointer moved
```

**Preventing Double Free**: Move semantics ensure only ONE pointer can free heap memory

### Function Calls

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);      // s moved into function
    // s is invalid here ❌
    
    let x = 5;
    makes_copy(x);           // x copied into function  
    // x is still valid ✅
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}  // some_string dropped here

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}  // some_integer dropped (no heap cleanup needed)
```

### Getting Ownership Back

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = takes_and_gives_back(s1);  // s1 moved in, new value moved out
    // s1 is invalid ❌, s2 is valid ✅
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // Ownership returned to caller
}
```

---

## 📍 Borrowing - Using Without Owning

### Immutable References (&T)

**Mental Model**: Like reading someone's book over their shoulder - you can look but not take it

```rust
fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);  // Borrow with &
    println!("'{}' has length {}", s, len);  // s still valid ✅
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope but doesn't drop - just a reference
```

**Key Point**: You can have **unlimited immutable references** at the same time

```rust
let s = String::from("hello");
let r1 = &s;  // ✅ First immutable borrow
let r2 = &s;  // ✅ Second immutable borrow  
let r3 = &s;  // ✅ Third immutable borrow
println!("{}, {}, {}", r1, r2, r3);  // All valid!
```

### Mutable References (&mut T)

**Mental Model**: Like borrowing someone's notebook to edit - only one person can edit at a time

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);              // Mutable borrow with &mut
    println!("{}", s);           // s modified: "hello, world"
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

**Key Point**: You can have **only ONE mutable reference** in a scope

```rust
let mut s = String::from("hello");
let r1 = &mut s;  // ✅ First mutable borrow
// let r2 = &mut s;  // ❌ ERROR! Cannot have two mutable borrows
```

### The Borrowing Rules

```
🎯 BORROWING RULES (enforced at compile time):
1. At any time, you can have EITHER:
   - One mutable reference (&mut T)
   - Any number of immutable references (&T)
2. References must always be valid (no dangling pointers)
```

### Why These Rules Matter

**Prevents Data Races** at compile time:

- Multiple readers OR one writer (never both simultaneously)
- Eliminates entire classes of bugs that plague other languages

```rust
let mut s = String::from("hello");

let r1 = &s;      // ✅ Immutable borrow
let r2 = &s;      // ✅ Another immutable borrow
// let r3 = &mut s;  // ❌ Cannot borrow mutably while immutable borrows exist

println!("{} {}", r1, r2);
// r1 and r2 are no longer used after this point

let r3 = &mut s;  // ✅ Mutable borrow OK now - immutable borrows are done
r3.push_str(" world");
```

---

## 🎓 Common Patterns

### Pattern 1: Temporary Borrowing

```rust
fn main() {
    let mut data = vec![1, 2, 3];
    process(&data);         // Borrow temporarily
    modify(&mut data);      // Borrow mutably
    println!("{:?}", data); // Still own it
}

fn process(v: &Vec<i32>) {
    println!("Length: {}", v.len());
}

fn modify(v: &mut Vec<i32>) {
    v.push(4);
}
```

### Pattern 2: Ownership Transfer Chain

```rust
fn main() {
    let data = create_data();     // Receive ownership
    let result = process(data);   // Transfer ownership
    consume(result);              // Transfer ownership again
}  // result dropped here

fn create_data() -> String {
    String::from("hello")
}

fn process(s: String) -> String {
    format!("{} world", s)
}

fn consume(s: String) {
    println!("{}", s);
}
```

### Pattern 3: Clone When Needed

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Explicit deep copy
    
    // Both valid ✅
    println!("s1 = {}, s2 = {}", s1, s2);
}
```

**When to Clone**:

- ✅ When you need independent copies
- ✅ When borrowing won't work (async, threads)
- ⚠️ Be aware: Clone can be expensive for large data

---

## 🔍 Stack vs Heap

### Stack Allocation

**Characteristics**:

- ✅ Fast allocation/deallocation (just move stack pointer)
- ✅ Deterministic size known at compile time
- ✅ Automatic cleanup (LIFO order)
- ❌ Limited size (~8MB typical)
- ❌ Can't grow dynamically

**Data on Stack**: Primitives (i32, f64, bool), arrays, tuples of stack types

```rust
let x = 5;              // On stack
let arr = [1, 2, 3];    // On stack (fixed size)
let tuple = (1, 2.0);   // On stack
```

### Heap Allocation

**Characteristics**:

- ✅ Dynamic size, can grow
- ✅ Large amount of memory available
- ❌ Slower allocation (requires finding free space)
- ❌ Requires explicit management (ownership in Rust)

**Data on Heap**: String, Vec, Box, Rc, custom structs with heap data

```rust
let s = String::from("hello");  // String data on heap
let v = vec![1, 2, 3];          // Vec data on heap
let b = Box::new(5);            // Boxed value on heap
```

### Ownership Manages Heap Memory

```rust
{
    let s = String::from("hello");
    // s owns heap-allocated string data
}  // s goes out of scope, Rust calls drop(), heap memory freed
```

**Key Insight**: Ownership rules primarily exist to manage heap memory safely

---

## 💡 Why Ownership Matters

### Memory Safety Without Garbage Collection

**Problems Ownership Prevents**:

1. **Use-after-free**: Can't use moved values
2. **Double-free**: Only one owner can drop data
3. **Memory leaks**: Drop called automatically  
4. **Data races**: Borrowing rules prevent simultaneous access
5. **Null pointer dereferencing**: Compiler enforces validity

### Compile-Time Guarantees

```rust
// This won't compile - Rust catches the error before running:
let s = String::from("hello");
let s2 = s;
println!("{}", s);  // ❌ Compile error: value used after move
```

**Zero Runtime Overhead**: All ownership checks happen at compile time

- No garbage collection pauses
- No reference counting overhead (except Rc/Arc when explicitly needed)
- Performance comparable to C/C++

---

## 🎯 Learning Progression

### Week 1: Basic Understanding

- Focus: Recognize when values move vs copy
- Practice: Simple examples with String and primitives
- Goal: Understand compiler error messages

### Week 2: Borrowing Basics  

- Focus: Using references to avoid moves
- Practice: Function parameters with `&T` and `&mut T`
- Goal: Predict when borrows are valid

### Week 3: Ownership Patterns

- Focus: Common patterns (temporary borrowing, ownership chains)
- Practice: Refactoring code to satisfy borrow checker
- Goal: Write idiomatic Rust code

### Don't Worry Yet About

- ⏸️ Lifetimes (advanced topic)
- ⏸️ Complex smart pointer patterns
- ⏸️ Async ownership challenges
- ⏸️ FFI and raw pointers

---

## 🐛 Common Errors and Solutions

### Error: "value borrowed here after move"

```rust
// ❌ Problem:
let s = String::from("hello");
let s2 = s;
println!("{}", s);  // ERROR: s moved to s2

// ✅ Solution 1: Clone
let s = String::from("hello");
let s2 = s.clone();
println!("{}", s);  // OK: s still valid

// ✅ Solution 2: Borrow instead
let s = String::from("hello");
let s2 = &s;
println!("{}", s);  // OK: s only borrowed
```

### Error: "cannot borrow as mutable more than once"

```rust
// ❌ Problem:
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;  // ERROR: second mutable borrow

// ✅ Solution: Use borrows sequentially
let mut s = String::from("hello");
{
    let r1 = &mut s;
    r1.push_str(" world");
}  // r1 goes out of scope
let r2 = &mut s;  // OK: r1 no longer exists
```

### Error: "cannot borrow as mutable because it is also borrowed as immutable"

```rust
// ❌ Problem:
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s;  // ERROR: already immutably borrowed

// ✅ Solution: Finish with immutable borrows first
let mut s = String::from("hello");
let r1 = &s;
println!("{}", r1);  // r1 last used here
let r2 = &mut s;     // OK: r1 no longer active
```

---

## 🔗 Related Concepts

### Foundation for Advanced Topics

- **[[borrowing-rules]]** - Detailed borrowing and lifetime rules
- **[[lifetimes]]** - Ensuring references remain valid
- **[[move-semantics]]** - Deep dive into move operations

### Smart Pointers (Build on Ownership)

- **[[box-in-aoc-problems]]** - Single ownership on heap
- **[[rc-shared-ownership]]** - Multiple ownership via reference counting
- **[[refcell-interior-mutability]]** - Runtime-checked mutable borrowing
- **[[Smart Pointers MOC]]** - Complete smart pointer ecosystem

### Memory Management

- **[[Memory Management]]** - Overall memory management strategies
- **[[drop-trait]]** - Automatic cleanup and RAII
- **[[copy-trait]]** - Types that can be duplicated
- **[[clone-trait]]** - Explicit deep copying

### Practical Applications

- **[[mission-1]]** - Stack implementation demonstrating ownership
- **[[mission-4]]** - LinkedList with complex ownership patterns
- **[[rust-threading-basics]]** - Ownership in concurrent contexts
- **[[async-await-basics]]** - Ownership in async code

### Learning Resources

- **[[../../missions/Mission1/QUICK_REFERENCE]]** - 2-minute ownership overview
- **[[../../missions/Mission1/SIMPLE_GUIDE]]** - Detailed learning guide with exercises
- **[[../../rust_book/Ch4/README]]** - Rust Book Chapter 4 notes
- **[[Ownership and Borrowing]]** - Comprehensive ownership guide

---

## 📚 Further Reading

### Mission Documentation

- **Mission 1**: Basic ownership with Stack implementation
- **Mission 2**: Ownership in Queue and RingBuffer
- **Mission 4**: Complex ownership with LinkedList and smart pointers

### Rust Book Integration

- **Chapter 4**: Understanding Ownership
- **Chapter 8**: Collections and ownership
- **Chapter 15**: Smart Pointers

### Daily Study Connections

- **[[daily-study/Day01]]** - Collections and ownership basics
- **[[daily-study/Day12]]** - Ord trait and ownership semantics

---

## 🎯 Key Takeaways

1. **Ownership = Memory Safety** - Prevents entire classes of bugs at compile time
2. **Three Rules** - One owner, one at a time, drop when out of scope
3. **Move vs Copy** - Stack data copies, heap data moves
4. **Borrowing** - Use without owning via references (&T, &mut T)
5. **Compile-Time Checked** - Zero runtime overhead, maximum safety
6. **Foundation** - All advanced Rust features build on ownership

---

*Remember: Ownership is the most important concept in Rust. Take your time, build intuition through practice, and don't get discouraged by compiler errors - they're teaching you to write safer code!*

---

*Navigation: [[zettel-index]] | [[rust-concepts-MOC]] | [[Memory Management]] | [[Smart Pointers MOC]] | [[cow-borrowed-vs-owned]]*

*Tags: #ownership #fundamentals #memory-safety #borrowing #move-semantics #rust-core-concepts*
