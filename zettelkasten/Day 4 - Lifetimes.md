# Day 4 - Lifetimes

**Quick Reference Note**
*For full content, see: [[daily_study/rust_learning_week1_notes/Day4]]*

---

## Core Concepts

### What Are Lifetimes?
**Lifetimes** ensure references are valid for as long as they're used.
They prevent **dangling references** (pointers to freed memory).

### Lifetime Syntax
```rust
'a  // Lifetime parameter named 'a
```

### When Lifetimes Are Needed

**Explicit Annotation Required:**
```rust
// Multiple input references
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// 'a means: output lives as long as the shorter input
```

**Automatic Inference (No Annotation):**
```rust
// Single input reference
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap()
}
// Compiler infers: fn first_word<'a>(s: &'a str) -> &'a str
```

---

## Lifetime Elision Rules

The compiler automatically infers lifetimes in these cases:

1. **Each input parameter gets its own lifetime**
   ```rust
   fn foo(x: &i32, y: &i32)
   // Becomes: fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
   ```

2. **Single input → output gets same lifetime**
   ```rust
   fn foo(x: &i32) -> &i32
   // Becomes: fn foo<'a>(x: &'a i32) -> &'a i32
   ```

3. **Method with &self → output gets self's lifetime**
   ```rust
   impl Foo {
       fn method(&self) -> &str
       // Becomes: fn method<'a>(&'a self) -> &'a str
   }
   ```

---

## Common Patterns

### Structs with References
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael...");
    let excerpt = ImportantExcerpt {
        part: &novel[0..10],
    };
    // excerpt cannot outlive novel
}
```

### Multiple Lifetimes
```rust
fn first_word_after<'a, 'b>(
    text: &'a str,
    delimiter: &'b str
) -> &'a str {
    // Output lifetime tied only to text, not delimiter
    match text.find(delimiter) {
        Some(pos) => &text[pos + delimiter.len()..],
        None => text,
    }
}
```

### Static Lifetime
```rust
let s: &'static str = "I live for the entire program";
// String literals have 'static lifetime
```

---

## Quick Rules

- **'a** is a lifetime parameter (like generic T)
- **Output lifetime ≤ shortest input lifetime**
- **Compiler infers** most lifetimes (elision rules)
- **Explicit needed** when ambiguous
- **'static** means "lives entire program"

---

## Visual Model

```
Function:
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str

Caller:
┌─────────────┐
│ string1: 'a │──┐
└─────────────┘  │
                 ├──► longest() ──► &'a str
┌─────────────┐  │                   (valid for 'a)
│ string2: 'b │──┘
└─────────────┘
'a = min('a, 'b)  // Shortest lifetime
```

---

## Common Errors

### ❌ Returning Local Reference
```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // ❌ s dropped, reference invalid
}
```

### ✅ Return Owned Value
```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // ✅ Move ownership out
}
```

---

## Quick Navigation

- **Full Details**: [[daily_study/rust_learning_week1_notes/Day4]]
- **Previous**: [[Day 3 - Borrowing]]
- **Next**: [[Day 5 - Option and Result]]
- **Week**: [[Week 1 Overview]]
- **MOC**: [[Rust Concepts MOC]]
- **Rust Book**: Chapter 10.3

---

*Tags: #lifetimes #references #annotations #compiler #quick-ref*
