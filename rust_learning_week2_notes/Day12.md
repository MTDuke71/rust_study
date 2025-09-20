# Day 12 · CLI Tools with Cargo

## Key Points
- `std::env::args()` yields command line arguments.
- Print results with `println!`.
- Use `cargo run -- arg1 arg2` for testing.

## Example
```rust
fn main() {
    for arg in std::env::args().skip(1) {
        println!("Arg: {arg}");
    }
}
```

- Combine with file reading (`std::fs::read_to_string`).

## Takeaway
Small CLI programs are how most AoC solutions are structured.
