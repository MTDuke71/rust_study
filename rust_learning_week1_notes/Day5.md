# Day 5 · Option and Result

## Key Points
- `Option<T>`: either `Some(T)` or `None`. Replaces null.
- `Result<T, E>`: either `Ok(T)` or `Err(E)`. For error handling.
- Pattern matching with `match`:
  ```rust
  match maybe_val {
      Some(x) => println!("{}", x),
      None => println!("empty"),
  }
  ```
- `?` operator propagates errors in functions that return `Result`.

## Example
```rust
fn read_file() -> std::io::Result<String> {
    let mut s = String::new();
    std::fs::File::open("foo.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

## Takeaway
Option/Result make absence/errors explicit and type-checked. No hidden exceptions.
