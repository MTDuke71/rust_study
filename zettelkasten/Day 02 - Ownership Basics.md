# Day 2 - Ownership Basics

**Quick Reference Note**
*For full content, see: [[daily_study/rust_learning_week1_notes/Day2]]*

---

## Core Concepts

### The Three Ownership Rules
1. Each value in Rust has a single **owner**
2. When the owner goes out of scope, the value is **dropped**
3. There can only be **one owner** at a time

### Move vs Copy Semantics

**Move (Heap Types)**
```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 is now INVALID
```

**Copy (Stack Types)**
```rust
let x = 5;
let y = x;    // x is still VALID (copied)
```

### Types That Implement Copy
- All integer types (`i32`, `u64`, etc.)
- Boolean type (`bool`)
- Floating point types (`f32`, `f64`)
- Character type (`char`)
- Tuples (if all elements implement Copy)

### Types That Don't Implement Copy (Move)
- `String`
- `Vec<T>`
- `HashMap<K, V>`
- Any type with heap allocation

---

## Memory Model

### Stack vs Heap
```
Stack (Copy):        Heap (Move):
┌─────┐             ┌─────────────┐
│  5  │             │ "hello"     │
└─────┘             └─────────────┘
   ↑                     ↑
Copied               Ownership transferred
```

### Function Ownership Transfer
```rust
fn takes_ownership(s: String) {
    println!("{}", s);
}  // s dropped here

let s = String::from("hello");
takes_ownership(s);  // s moved into function
// s is NO LONGER valid here
```

---

## Quick Rules

- **Heap types MOVE** by default (String, Vec, etc.)
- **Stack types COPY** by default (i32, bool, etc.)
- **Use `.clone()`** for explicit deep copy
- **Functions take ownership** unless using references

---

## Quick Navigation

- **Full Details**: [[daily_study/rust_learning_week1_notes/Day2]]
- **Previous**: [[Day 01 - Setup]]
- **Next**: [[Day 03 - Borrowing]]
- **Week**: [[Week 1 Overview]]
- **MOC**: [[Rust Concepts MOC]]

---

*Tags: #ownership #moves #copy #memory-model #quick-ref*
