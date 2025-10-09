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

// Mutable borrow
fn append_text(s: &mut String) {
    s.push_str("!");
}

// Usage
let mut text = String::from("hello");
let len = calculate_length(&text);     // Borrow immutably
append_text(&mut text);                // Borrow mutably
```

### Borrow Scope Rules
```rust
let mut s = String::from("hello");

{
    let r1 = &s;      // Immutable borrow starts
    println!("{}", r1);
}                     // r1 scope ends here

let r2 = &mut s;      // ✅ OK - no active borrows
r2.push_str(" world");
```

---

## Common Patterns

### Read Without Taking Ownership
```rust
fn get_length(text: &String) -> usize {
    text.len()  // Can read but not modify
}

let s = String::from("hello");
let len = get_length(&s);
println!("{}", s);  // s still valid
```

### Modify Without Taking Ownership
```rust
fn make_uppercase(text: &mut String) {
    *text = text.to_uppercase();
}

let mut s = String::from("hello");
make_uppercase(&mut s);
println!("{}", s);  // "HELLO"
```

---

## Quick Rules

- **&T** = Immutable reference (many allowed)
- **&mut T** = Mutable reference (only ONE)
- **Can't mix** immutable and mutable borrows
- **References don't own** - no drop when scope ends
- **Prefer &str over &String** for function parameters

---

## Visual Memory Model

```
Owner:              Borrower:
┌─────────────┐    ┌─────────┐
│   String    │◄───│  &s     │
│  "hello"    │    │ (ref)   │
└─────────────┘    └─────────┘
     owns            borrows
```

---

## Quick Navigation

- **Full Details**: [[daily_study/rust_learning_week1_notes/Day03|Day03]]
- **Previous**: [[Day 02 - Ownership Basics]]
- **Next**: [[Day 04 - Lifetimes]]
- **Week**: [[Week 1 Overview]]
- **MOC**: [[Rust Concepts MOC]]

---

*Tags: #borrowing #references #mutable #immutable #quick-ref*
