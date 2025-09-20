# Day 10 · Strings (`String` vs `&str`)

## Key Points
- `&str` = borrowed slice of UTF-8 bytes.
- `String` = owned, growable heap buffer of UTF-8 bytes.
- Indexing by position is tricky: `String` is UTF-8, so `s[0]` doesn’t compile.
- Use `.chars()` or `.char_indices()`.

## Example
```rust
let s = String::from("hi 🚀");
for (i, c) in s.char_indices() {
    println!("{i}: {c}");
}
```

## Takeaway
Work with strings at the `char` or iterator level. Indexing is unsafe in UTF-8 world.
