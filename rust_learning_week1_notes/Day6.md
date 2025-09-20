# Day 6 · Pattern Matching

## Key Points
- `match` is exhaustive: must cover all cases.
- `if let` is shorthand for matching one pattern.
- `while let` loops until pattern fails.
- Useful with `Option` and `Result`.

## Example
```rust
let v = Some(42);
if let Some(x) = v {
    println!("x = {}", x);
}
```

- Destructuring patterns with tuples, structs, enums.

## Takeaway
Pattern matching is powerful, expressive, and central to Rust style.
