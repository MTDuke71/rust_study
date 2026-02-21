# Index Tracking

*Tags: #algorithms #pattern #data-structures #aoc #disambiguation*
*Links: [[zettel-index]] | [[Algorithms MOC]] | [[circular-list]] | [[modular-arithmetic]] | [[Collections MOC]]*

---

## Core Idea

**Index tracking** tags each element with its original position to distinguish duplicates. When a collection contains repeated values, you can't identify elements by value alone — you need a unique identifier. The original index serves as that identifier.

```
Values only:  [2, 1, 2, 3]     ← Which "2" do we mean?
With index:   [(0,2), (1,1), (2,2), (3,3)]  ← (0,2) and (2,2) are distinct
```

**When to use**: Any algorithm that processes elements in a specific order and needs to find/track elements that may have duplicate values.

---

## The Pattern

### Tagging with `enumerate()`

Rust's `enumerate()` pairs each element with its index:

```rust
let values = vec![2, 1, 2, 3];

// Tag with original indices
let tagged: Vec<(usize, i32)> = values.iter().copied().enumerate().collect();
// [(0, 2), (1, 1), (2, 2), (3, 3)]

// Find by original index (not value!)
let pos = tagged.iter().position(|&(i, _)| i == 2).unwrap();
// pos = 2, element is (2, 2) — the SECOND "2", not the first
```

### Strip Indices When Done

After processing, remove the tags to get clean output:

```rust
let result: Vec<i32> = tagged.into_iter().map(|(_, v)| v).collect();
// [2, 1, 2, 3]
```

---

## AoC 2022 Day 20: Full Example

The circular list mixing problem requires processing elements in original order, but the list gets shuffled. Without index tracking, finding "which 2 to move next" is ambiguous:

```rust
fn mix(values: &[i64], rounds: usize) -> Vec<i64> {
    let n = values.len();
    // Tag: each element knows its original position
    let mut seq: Vec<(usize, i64)> = values.iter().copied().enumerate().collect();

    for _ in 0..rounds {
        for orig_idx in 0..n {
            // Find THIS specific element by its original index
            let pos = seq.iter().position(|&(i, _)| i == orig_idx).unwrap();
            let (_, val) = seq[pos];

            if val == 0 { continue; }
            seq.remove(pos);
            let modulus = (n - 1) as i64;
            let new_pos = ((pos as i64 + val) % modulus + modulus) % modulus;
            seq.insert(new_pos as usize, (orig_idx, val));
        }
    }
    // Strip indices for final result
    seq.into_iter().map(|(_, v)| v).collect()
}
```

**Why not use value-based lookup?**
```
Input: [1, 2, -3, 3, -2, 0, 4]   ← all unique, value lookup works
Input: [1, 2, -3, 2, -2, 0, 4]   ← two "2"s! Which one moves first?
```

The actual AoC input has many duplicate values. Index tracking is essential.

---

## Variations

### HashMap Lookup (O(1) Find)

For large collections, linear scan (`position()`) is O(n). A HashMap provides O(1) lookup:

```rust
use std::collections::HashMap;

// Map: original_index → current_position
let mut positions: HashMap<usize, usize> = (0..n).map(|i| (i, i)).collect();

// Find element in O(1)
let current_pos = positions[&orig_idx];

// Update after move
positions.insert(orig_idx, new_pos);
// Must also update positions of shifted elements!
```

Trade-off: O(1) find but maintaining the position map during shifts is complex.

### Stable Sort Preservation

Index tracking preserves original order through sorting:

```rust
let mut indexed: Vec<(usize, i32)> = values.iter().copied().enumerate().collect();
indexed.sort_by_key(|&(_, v)| v);
// Duplicates maintain their relative original order (stable)

// Recover original order:
indexed.sort_by_key(|&(i, _)| i);
```

### Permutation Tracking

Track where each element ends up after a transformation:

```rust
// Before: [A, B, C, D, E]  (indices 0..5)
// After:  [C, A, E, B, D]
// Permutation: [2, 0, 4, 1, 3]  (element at position i came from position perm[i])
```

---

## Related Patterns

### Index Tracking vs. ID Fields

| Approach | When to Use | Example |
|----------|-------------|---------|
| `enumerate()` | Elements are plain values (i32, i64) | AoC Day 20 mixing |
| Struct with `id` field | Elements already have structure | `struct Item { id: usize, value: i64 }` |
| HashMap key | Need O(1) lookup by ID | Large collections with frequent finds |

### Separation of Concerns

Index tracking belongs in the **algorithm**, not the **parser**:

```rust
// Parser returns clean data
fn parse(input: &str) -> Vec<i64> { /* just values */ }

// Algorithm adds indices as needed
fn mix(values: &[i64]) -> Vec<i64> {
    let tagged: Vec<(usize, i64)> = values.iter().copied().enumerate().collect();
    // ...
}
```

Why: The parser shouldn't know which algorithm will consume the data. Different algorithms may need different tagging strategies (or none at all).

---

## AoC Connections

| Problem | Pattern | Notes |
|---------|---------|-------|
| **AoC 2022 Day 20** | Index-tagged circular mixing | Duplicate values in list require original index |
| **AoC 2024 Day 5** | Ordering with position tracking | Track page positions for dependency ordering |
| **General sorting** | Stable sort with indices | Preserve original order for equal elements |

---

## Key Takeaways

1. **`enumerate().collect()`** is the standard Rust pattern for index tagging
2. **Find by index, not value**: `position(|&(i, _)| i == target_idx)` distinguishes duplicates
3. **Strip when done**: `.map(|(_, v)| v)` removes the tracking overhead from results
4. **Keep parsing separate**: Tags are algorithm concerns, not data concerns
5. **O(n) find is fine for moderate n**: At n=5000, linear scan is ~5ms — HashMap overhead isn't worth it

---

*Related: [[circular-list]] | [[modular-arithmetic]] | [[entry-api-hashmap]] | [[Collections MOC]]*
