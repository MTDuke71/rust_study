# Day 13 · Iterators

## Key Points
- Iterators are lazy, composable pipelines.
- Common adaptors: `map`, `filter`, `enumerate`, `collect`.
- Consuming adaptors: `sum`, `count`, `for_each`.

## Example
```rust
let nums = vec![1, 2, 3, 4];
let doubled: Vec<_> = nums.iter().map(|x| x * 2).collect();
```

- Custom iterators implement `Iterator` trait with `fn next(&mut self)`.

## Takeaway
Iterators are idiomatic Rust. Many AoC puzzles are solved with iterator chains.
