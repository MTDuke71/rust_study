# Circular List

*Tags: #algorithms #data-structures #pattern #aoc #circular #modular-arithmetic*
*Links: [[zettel-index]] | [[Algorithms MOC]] | [[modular-arithmetic]] | [[index-tracking]] | [[Collections MOC]]*

---

## Core Idea

A **circular list** is a sequence where the last element wraps around to the first. There is no "beginning" or "end" — any element can be the starting point. Positions are defined relative to other elements, not absolute indices.

```
Linear:    [A, B, C, D, E]     ← A is first, E is last
Circular:  ... → A → B → C → D → E → A → B → ...   ← no start/end
```

**Key property**: Position arithmetic uses modular indexing — `index % n` wraps around naturally.

---

## Fundamental Operations

### Circular Indexing

Access elements at any offset, wrapping around:

```rust
let items = vec![10, 20, 30, 40, 50]; // n = 5
let n = items.len();

// Forward wrap
items[7 % n]  // 7 % 5 = 2 → 30
items[12 % n] // 12 % 5 = 2 → 30

// Offset from a position
let start = 3;
items[(start + 2) % n] // (3 + 2) % 5 = 0 → 10 (wrapped!)
```

### Circular Extraction

Extract values at fixed offsets from a known position:

```rust
// AoC 2022 Day 20: Find values at positions 1000, 2000, 3000 after 0
let zero_pos = mixed.iter().position(|&v| v == 0).unwrap();
let n = mixed.len();
let sum: i64 = [1000, 2000, 3000]
    .iter()
    .map(|offset| mixed[(zero_pos + offset) % n])
    .sum();
```

### Circular Distance

The shortest distance between two positions in a circular list:

```rust
fn circular_distance(a: usize, b: usize, n: usize) -> usize {
    let forward = (b + n - a) % n;
    let backward = (a + n - b) % n;
    forward.min(backward)
}
```

---

## The Remove-and-Reinsert Pattern

A common operation: remove an element, compute a new position, and reinsert it. This is the core of AoC 2022 Day 20's "mixing" algorithm.

### Critical Subtlety: Modulus is `n-1`, Not `n`

After removing an element, the list has `n-1` positions. The modulus must reflect the reduced list size:

```rust
let n = seq.len();        // original size
seq.remove(pos);          // now seq has n-1 elements

// CORRECT: mod (n-1) because element is removed
let modulus = (n - 1) as i64;
let new_pos = ((pos as i64 + val) % modulus + modulus) % modulus;
seq.insert(new_pos as usize, element);
```

### Why `n-1`?

Consider 7 elements in a circle. Remove one — now 6 gaps exist for reinsertion:

```
Before: [A, B, C, D, E, F, G]    (7 elements, 7 circular positions)
Remove C: [A, B, _, D, E, F, G]  → [A, B, D, E, F, G]  (6 elements, 6 gaps)

If C moves by 7 (full circle), it should return to same spot:
  mod 6: (2 + 7) % 6 = 3 ✓  (inserts between D and E = original spot)
  mod 7: (2 + 7) % 7 = 2 ✓  (works here, but fails at boundaries)
```

The `n-1` modulus is correct because a full cycle of `n-1` moves brings the element back to where it was in the remaining `n-1`-element list.

---

## Implementation Strategies

### Vec-Based (Simple, Cache-Friendly)

Best for small-to-medium lists (~5,000 elements). `Vec::remove` and `Vec::insert` are O(n) but cache-friendly:

```rust
fn mix(values: &[i64], rounds: usize) -> Vec<i64> {
    let n = values.len();
    let mut seq: Vec<(usize, i64)> = values.iter().copied().enumerate().collect();

    for _ in 0..rounds {
        for orig_idx in 0..n {
            let pos = seq.iter().position(|&(i, _)| i == orig_idx).unwrap();
            let (_, val) = seq[pos];
            if val == 0 { continue; }

            seq.remove(pos);
            let modulus = (n - 1) as i64;
            let new_pos = ((pos as i64 + val) % modulus + modulus) % modulus;
            seq.insert(new_pos as usize, (orig_idx, val));
        }
    }
    seq.into_iter().map(|(_, v)| v).collect()
}
```

**Complexity**: O(n²) per round — n elements × O(n) find + O(n) shift each.

### VecDeque-Based (Rotation)

For problems where you rotate the list rather than moving individual elements:

```rust
use std::collections::VecDeque;

let mut deque: VecDeque<i32> = (0..10).collect();
deque.rotate_left(3);  // [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]
deque.rotate_right(1); // [2, 3, 4, 5, 6, 7, 8, 9, 0, 1]
```

### Linked List (Theoretical O(1) Move, Poor Cache)

A doubly-linked list with an index lookup array gives O(1) moves:

```
Index Array: orig_idx → node pointer
Move: Unlink node → Traverse k positions → Relink

Find: O(1) via index array
Move: O(k) traversal (but k reduced by mod n-1)
```

In practice, cache misses make this slower than Vec for n ≤ ~10,000. Contiguous memory in Vec allows CPU prefetching that dominates pointer-chasing.

---

## AoC Connections

| Problem | Pattern | Notes |
|---------|---------|-------|
| **AoC 2022 Day 20** | Remove-reinsert mixing | Core circular list problem, mod n-1 |
| **AoC 2015 Day 19** | Circular buffer | Josephus-style elimination |
| **AoC 2018 Day 9** | Marble circle | Insert/remove in circular sequence |

---

## Common Pitfalls

1. **Off-by-one with modulus**: Use `n-1` when element is removed, `n` when it's still in the list
2. **Negative modulus in Rust**: `%` preserves sign → use `((x % m) + m) % m` pattern (see [[modular-arithmetic]])
3. **Duplicate values**: Can't find elements by value alone in a circular list with duplicates → use [[index-tracking]]
4. **"Full circle = no movement" assumption**: A full circle is `n-1` moves (not `n`) when element is temporarily removed

---

## Key Takeaways

1. **Circular = modular indexing**: `(pos + offset) % n` handles all wrap-around
2. **Remove-reinsert uses mod `n-1`**: The temporarily-reduced list has fewer gaps
3. **Vec beats linked list for moderate n**: Cache locality wins up to ~10K elements
4. **Double-modulus for negative values**: `((x % m) + m) % m` normalizes to `[0, m-1]`

---

*Related: [[modular-arithmetic]] | [[index-tracking]] | [[sliding-window-patterns]] | [[bitmask-representation]]*
