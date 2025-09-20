# Day 11 · Custom Errors

## Key Points
- Define errors as `enum`:
```rust
#[derive(Debug)]
enum ParseError {
    Empty,
    BadNumber(std::num::ParseIntError),
}
```
- Implement `From` for conversion.
- Functions return `Result<T, E>` with custom error.

## Example
```rust
fn parse_number(s: &str) -> Result<i32, ParseError> {
    if s.trim().is_empty() {
        return Err(ParseError::Empty);
    }
    s.trim().parse::<i32>().map_err(ParseError::BadNumber)
}
```

## Takeaway
Custom error types make failures explicit and composable in AoC parsers.
