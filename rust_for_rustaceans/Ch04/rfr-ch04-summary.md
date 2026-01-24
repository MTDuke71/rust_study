
---
# 📘 Rust for Rustaceans — Chapter 4: Error Handling

**Zettelkasten**: [[rust-for-rustaceans]] — Rustaceans book knowledge hub

---

## 🧩 Chapter Overview
This chapter explores how to represent, handle, and propagate failures in Rust, focusing on the trade-offs between exposing detailed error information and keeping interfaces simple.

---

## 🛠️ Error Representation Patterns

### 1. Enumeration (Domain-Specific Errors)
Use enums when callers need to distinguish between error cases:

```rust
#[derive(Debug)]
enum CopyError {
    Input(std::io::Error),
    Output(std::io::Error),
}
```

**Best Practices:**
- Implement `std::error::Error` (especially `source()` for root cause/backtraces)
- Derive `Display` and `Debug` (Display: one-line, lowercase, no trailing punctuation)
- Ensure `Send`, `Sync`, and `'static` for thread safety and downcasting

### 2. Opaque Errors (Type Erasure)
Use when the specific cause is less important than the fact of failure:

```rust
fn do_work() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // ...
}
```

**Benefits:**
- Simplifies return types
- Composes errors from different sources
- Allows downcasting via `Error::downcast_ref` if `'static`

### 3. Special Error Cases
- **Unit Error**: `Result<T, ()>` — signals failure without info (vs. `Option<T>`: absence of value)
- **Never Type**: `Result<T, !>` — for functions that never return; compiler can optimize error handling away
- **Thread Results**: `std::thread::Result` uses `Box<dyn Any + ...>` since panics can be any type

---

## 🔄 Error Propagation Techniques

### 1. The `?` Operator
- Shorthand for unwrapping `Ok` or returning `Err` early
- Converts error types via the `From` trait

```rust
fn parse_file() -> Result<i32, MyError> {
    let s = std::fs::read_to_string("input.txt")?;
    let n = s.trim().parse()?;
    Ok(n)
}
```

#### 📝 Side Note: From and Into Traits

**Why Both Exist:**
- **Historical**: Before Rust 1.41.0, coherence rules made it difficult to implement both directions. Legacy code and stability guarantees keep both traits.
- **Ergonomics**: Each serves distinct API design roles:
  - `Into` is superior for **trait bounds**: `fn foo(arg: impl Into<Bar>)` 
  - `From` is superior for **explicit calls**: `Ident::from("foo")`

**The Golden Rule:**
> **Implement `From`, use `Into`**

```rust
// ✅ Good: Implement From
impl From<&str> for MyType {
    fn from(s: &str) -> Self { /* ... */ }
}

// ✅ Good: Accept Into in function bounds
fn process(value: impl Into<MyType>) { /* ... */ }

// Automatic: Standard library provides Into via blanket impl
// impl<T, U> Into<U> for T where U: From<T>
```

**The `?` Operator Exception:**
- `?` currently uses `From`, **not** `Into`
- Error types from older libraries implementing only `Into` won't work with `?`
- Compiler handles trait resolution better with `From` in this context
- May eventually upgrade to `Into` as compiler improves

### 2. The `Try` Trait (Unstable)
- Powers the `?` operator
- Abstracts "happy path" vs. "early return" logic
- Enables use with types beyond `Result` (e.g., `Option`, `Poll`)

### 3. Try Blocks (Unstable)
- Scope the `?` operator to a block
- Allows cleanup code to run even if intermediate steps fail

```rust
let result = try {
    step1()?;
    step2()?;
    thing.cleanup();
};
```

---

## 🧠 Insights & Best Practices
- Prefer explicit error types for clarity and composability
- Use crates like `thiserror` for ergonomic error definitions
- Avoid panics in library code; reserve for unrecoverable, programmer errors
- Document error cases in public APIs
- Use `Result<(), E>` for functions that only signal success/failure
- Integrate error handling with logging for production systems

---

## 📚 Further Reading
- [Error Handling - Rust Book](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [thiserror crate](https://docs.rs/thiserror)
- [anyhow crate](https://docs.rs/anyhow)
- [Rust API Guidelines: Error Handling](https://rust-lang.github.io/api-guidelines/interoperability.html#error-handling-c)

---

## 🔗 Related Notes
- [[rust_book/rust-book-ch9-12-review.md]] — Error handling in Rust Book
- [[zettelkasten/error-handling-patterns.md]] — Zettelkasten: Error handling patterns
- [[mission-5]] — Mission 5: HashMap/HashSet error handling

---

*Tags: #rust #error-handling #rust-for-rustaceans #chapter-summary*