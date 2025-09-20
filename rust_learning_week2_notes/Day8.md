# Day 8 · Vectors (`Vec<T>`)

## Key Points
- `Vec<T>` is a growable, contiguous array.
- Common methods: `push`, `pop`, `len`, `is_empty`, `get`, `iter`.
- Indexing with `[i]` panics if out of bounds; `get(i)` returns `Option<&T>`.
- Vectors own their elements; pushing moves values in, popping moves them out.

## Example
```rust
let mut v = Vec::new();
v.push(1);
v.push(2);
assert_eq!(v.pop(), Some(2));
```

## Takeaway
`Vec<T>` underpins many structures (Stack, Ring Buffer). Mastering it is essential.
