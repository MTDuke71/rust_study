# Day 9 · HashMap & HashSet

## Key Points
- `HashMap<K,V>` stores key-value pairs. Requires `K: Eq + Hash`.
- `HashSet<T>` is just `HashMap<T, ()>` internally.
- Common ops: `insert`, `get`, `contains_key`, `entry`.

## Example
```rust
use std::collections::HashMap;
let mut counts = HashMap::new();
for word in ["a", "b", "a"] {
    *counts.entry(word).or_insert(0) += 1;
}
assert_eq!(counts["a"], 2);
```

## Takeaway
Maps/sets appear in almost every AoC puzzle: counting, state tracking, deduplication.
