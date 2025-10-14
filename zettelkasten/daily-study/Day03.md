# Day 3 - References & Borrowing

**Quick Reference Note**
*For full content, see: [[daily_study/rust_learning_week1_notes/Day03]]*

---

## Core Concepts

### The Borrowing Rules
1. **Multiple immutable references** OR **one mutable reference** (not both)
2. **References must always be valid** (no dangling references)

### Reference Types

**Immutable Borrow (&T)**
```rust
let s = String::from("hello");
let r1 = &s;  // ✅ OK
let r2 = &s;  // ✅ OK - multiple immutable allowed
println!("{} {}", r1, r2);
```

**Mutable Borrow (&mut T)**
```rust
let mut s = String::from("hello");
let r = &mut s;  // ✅ OK - only one mutable
r.push_str(" world");
// Can't have another reference here
```

**❌ Invalid: Mixed Borrows**
```rust
let mut s = String::from("hello");
let r1 = &s;       // Immutable borrow
let r2 = &mut s;   // ❌ ERROR: Can't have both!
println!("{}", r1);
```

---

## Reference Patterns

### Function Borrowing
```rust
// Immutable borrow - most common
fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope but doesn't drop (not owner)

let s = String::from("hello");
let len = calculate_length(&s);  // s still valid here
println!("{} has length {}", s, len);
```

### Mutable Borrowing
```rust
fn change(s: &mut String) {
    s.push_str(", world");
}

let mut s = String::from("hello");
change(&mut s);
println!("{}", s);  // "hello, world"
```

---

## Quick Rules

- **Use `&`** for immutable borrows (read-only)
- **Use `&mut`** for mutable borrows (can modify)
- **No borrowing** = ownership transfer (move)
- **References don't take ownership** = no drop when scope ends

---

## Quick Navigation

- **Full Details**: [[daily_study/rust_learning_week1_notes/Day03]]
- **Previous**: [[Day 02 - Ownership Basics]]
- **Next**: [[Day 04 - Lifetimes]]
- **Week**: [[Week 1 Overview]]
- **MOC**: [[Rust Concepts MOC]]

---

*Tags: #borrowing #references #mutability #quick-ref*
