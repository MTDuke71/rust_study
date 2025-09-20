# Day 4 · Lifetimes Basics

## Key Points
- References must never outlive the values they refer to.
- Compiler infers lifetimes most of the time.
- Explicit lifetimes appear when function returns references from parameters.
- Syntax: `<'a>` means lifetime `'a`.

## Example
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

- `'a` says: the output reference is valid as long as both inputs are.

## Takeaway
Lifetimes prevent dangling references. Most code never needs explicit lifetimes, but understanding them avoids confusion when they appear.
