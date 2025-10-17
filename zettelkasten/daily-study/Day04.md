# Day 4 - Lifetimes

**Quick Reference Note**
*For full content, see: [[daily_study/rust_learning_week1_notes/Day04]]*

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

2. **Single input reference returns same lifetime**
   ```rust
   fn first_word(s: &str) -> &str
   // Becomes: fn first_word<'a>(s: &'a str) -> &'a str
   ```

3. **Multiple inputs with output: all must match**
   ```rust
   fn longest(x: &str, y: &str) -> &str  // ❌ Ambiguous!
   ```

---

## Struct Lifetimes

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,  // Reference must live at least as long as struct
}

fn main() {
    let novel = String::from("Call me Ishmael...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    // i is valid as long as first_sentence is valid
}
```

---

## Quick Rules

- **Lifetimes ensure reference validity**
- **Most lifetime annotations are inferred**
- **Explicit annotations needed for ambiguous cases**
- **Structs with references need lifetime parameters**

---

## Quick Navigation

- **Full Details**: [[daily_study/rust_learning_week1_notes/Day04]]
- **Previous**: [[daily-study/Day03]]
- **Next**: [[daily-study/Day05]]
- **Week**: [[Week 1 Overview]]
- **MOC**: [[Rust Concepts MOC]]

---

*Tags: #lifetimes #references #memory-safety #quick-ref*