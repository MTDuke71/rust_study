# Day 20: Grove Positioning System - Function Guide

**Problem**: Decrypt grove coordinates by "mixing" a circular list of numbers.

**Navigation**: [← Day 19](day19_function_guide.md) | [Problem](day20.md) | [Code](../../src/solver/day20.rs) | [Summary](../summary_2022.md) | [Day 21 →](day21_function_guide.md)

---

## Overview

### Problem Summary
- **Part 1**: Mix the list once, find values at positions 1000, 2000, 3000 after `0`
- **Part 2**: Multiply all values by 811,589,153, mix 10 times, same extraction

### Performance
- **Parse**: 78µs
- **Part 1**: 5.72ms (1 round, ~5000 elements)
- **Part 2**: 59.49ms (10 rounds)
- **Combined**: 65.46ms

### Key Insight
**Duplicates require index tracking.** The input contains repeated values, so you can't find elements by value alone. Each element is tagged with its original index `(orig_idx, value)` and located by scanning for its unique index each round.

**Modulus is `n-1`, not `n`.** When you remove an element from the circular list to reinsert it, there are only `n-1` gaps. Using `n` as the modulus would cause off-by-one errors at the wrap boundary.

---

## Algorithm Analysis

### Circular List Mixing

The mixing operation for each element:

```
1. Find element's current position in the list (by original index)
2. Remove it — list now has n-1 elements
3. Compute new position: (old_pos + value) mod (n-1)
4. Insert at new position
```

The key subtlety is step 3. After removal, the circular list has `n-1` positions. The element moves `value` positions from where it was, wrapping around:

```rust
let modulus = (n - 1) as i64;
let new_pos = ((pos as i64 + val) % modulus + modulus) % modulus;
```

The double-modulus pattern `((x % m) + m) % m` handles negative values correctly — Rust's `%` operator preserves the sign of the dividend, so `-7 % 5 = -2`, not `3`.

### Why `n-1`?

Consider a 7-element circular list. Remove element at position 2:

```
Before: [A, B, C, D, E, F, G]    (7 elements, 7 positions)
Remove C: [A, B, D, E, F, G]      (6 elements, 6 gaps to insert into)
```

If C's value is 7 (a full circle), it should return to the same spot. With modulus `n-1 = 6`: `(2 + 7) % 6 = 3` — C goes between D and E, which is equivalent to its original position in the circular sense. With modulus `n = 7`: `(2 + 7) % 7 = 2` — also works for this case, but breaks at boundaries.

### Worked Example

```
Initial: 1, 2, -3, 3, -2, 0, 4    (n=7, modulus=6)

Process 1 (orig_idx=0, val=1):
  pos=0, new=(0+1)%6=1
  → 2, 1, -3, 3, -2, 0, 4

Process 2 (orig_idx=1, val=2):
  pos=0, new=(0+2)%6=2
  → 1, -3, 2, 3, -2, 0, 4

Process -3 (orig_idx=2, val=-3):
  pos=1, new=((1-3)%6+6)%6=4
  → 1, 2, 3, -2, -3, 0, 4

Process 3 (orig_idx=3, val=3):
  pos=2, new=(2+3)%6=5
  → 1, 2, -2, -3, 0, 3, 4

Process -2 (orig_idx=4, val=-2):
  pos=2, new=((2-2)%6+6)%6=0
  → 1, 2, -3, 0, 3, 4, -2    (inserted at pos 0 = end in circular sense)

Process 0 (orig_idx=5, val=0):
  skip (no movement)

Process 4 (orig_idx=6, val=4):
  pos=5, new=(5+4)%6=3
  → 1, 2, -3, 4, 0, 3, -2
```

### Part 2: Scaling + Multiple Rounds

Part 2 multiplies every value by `811,589,153` before mixing, then mixes 10 times. The same algorithm works — `i64` handles the large values, and the `% (n-1)` modulus keeps positions bounded regardless of value magnitude.

---

## Implementation Details

### Data Flow

```
Input: "1\n2\n-3\n3\n-2\n0\n4"
  ↓ parse_input()
Vec<i64>           ← raw values [1, 2, -3, 3, -2, 0, 4]
  ↓ mix(values, rounds)
Tag with indices:  [(0,1), (1,2), (2,-3), (3,3), (4,-2), (5,0), (6,4)]
  ↓ for each round, for each orig_idx
Find → Remove → Compute new pos → Insert
  ↓ strip indices
Vec<i64>           ← mixed values
  ↓ grove_coordinates()
Find 0, extract at offsets 1000, 2000, 3000 (mod n) → sum
```

### Function Signatures

| Function | Input | Output | Purpose |
|----------|-------|--------|---------|
| `parse_input` | `&str` | `Vec<i64>` | Parse one number per line |
| `mix` | `&[i64], usize` | `Vec<i64>` | Core mixing algorithm, `rounds` iterations |
| `grove_coordinates` | `&[i64]` | `i64` | Find 0, sum values at +1000, +2000, +3000 |
| `solve_part1` | `&ParsedData` | `i64` | Mix 1 round |
| `solve_part2` | `&ParsedData` | `i64` | Scale by 811,589,153, mix 10 rounds |

### Key Code Patterns

**Index-tagged elements** — duplicates are distinguished by original position:
```rust
let mut seq: Vec<(usize, i64)> = values.iter().copied().enumerate().collect();
// Find by original index, not value:
let pos = seq.iter().position(|&(i, _)| i == orig_idx).unwrap();
```

**Circular extraction** — offsets from zero wrap around:
```rust
let zero_pos = mixed.iter().position(|&v| v == 0).unwrap();
[1000, 2000, 3000].iter()
    .map(|offset| mixed[(zero_pos + offset) % n])
    .sum()
```

---

## Performance Analysis

### Benchmark Results

```
day20_parse:      78µs     ← 5000 lines, parse i64
day20_part1:     5.72ms    ← 1 round × 5000 elements
day20_part2:    59.49ms    ← 10 rounds × 5000 elements
day20_combined: 65.46ms    ← Parse once + both parts
```

### Complexity

| Component | Complexity | Notes |
|-----------|-----------|-------|
| Parsing | O(n) | n ≈ 5000 numbers |
| Find element | O(n) | Linear scan for original index |
| Remove + Insert | O(n) | Vec shifts elements |
| One round | O(n²) | n elements × O(n) per element |
| Part 1 | O(n²) | 1 round |
| Part 2 | O(10 × n²) | 10 rounds |
| Grove extraction | O(n) | Find 0 + 3 indexed lookups |

### Why O(n²)?

Each `Vec::remove(pos)` and `Vec::insert(pos, val)` shifts up to `n` elements. This happens `n` times per round:
- Part 1: ~5000² = 25M element shifts → 5.72ms
- Part 2: 10 × 25M = 250M element shifts → 59.49ms

### Potential Optimizations (Not Implemented)

1. **Index array indirection**: Instead of moving elements in a Vec, maintain a permutation array. Finding an element is still O(n), but moves become O(1). Reduces to O(n²) from O(n²) — same asymptotic but lower constant.

2. **Doubly-linked list with index lookup**: Store elements in a linked list with a side array mapping `orig_idx → node`. Finding = O(1), moving = O(k) where k is the move distance (but k can be reduced via modulus). Combined: O(n × avg_move_distance).

3. **Indexed order-statistic tree**: O(n log n) per round using a balanced BST where each node tracks subtree size. Both find-by-rank and insert-at-rank become O(log n). Not idiomatic Rust (self-referential nodes).

At 65ms total, optimization isn't critical for correctness, but this is the slowest AoC 2022 day so far.

---

## Edge Cases

### Zero Values
Elements with value `0` don't move — the `if val == 0 { continue; }` guard skips them entirely, avoiding unnecessary remove/insert overhead.

### Large Values (Part 2)
Values like `811,589,153 × 4 = 3,246,356,612` are huge, but the modulus `n-1 ≈ 4999` keeps insertion positions bounded. The `% modulus` ensures we never insert beyond the list length.

### Negative Modulus
Rust's `%` returns negative remainders for negative dividends: `-7 % 6 = -1`. The `((x % m) + m) % m` pattern normalizes to `[0, m-1]`.

### Single Element with Value 0
If the list were `[0]`, `grove_coordinates` would return `0 + 0 + 0 = 0`. The modular indexing handles this naturally.

---

## Key Takeaways

1. **Tag elements with original indices** to handle duplicates. The list `[2, 1, 2, 3]` has two `2`s — you need to know WHICH `2` to move when processing in original order.

2. **Modulus `n-1` for circular reinsertion.** After removing an element, the list has `n-1` slots. This is a common off-by-one trap in circular list problems.

3. **Double-modulus for negative values.** `((x % m) + m) % m` is the standard pattern for non-negative modulus in languages where `%` preserves sign.

4. **Vec remove/insert is O(n) but cache-friendly.** Despite shifting elements, contiguous memory and CPU cache prefetching make Vec faster than linked lists for moderate sizes (~5000). The O(n²) total is acceptable here.

5. **Part 2 reuses the same algorithm.** Scaling values and adding rounds doesn't change the core logic — just the inputs to `mix()`. Good separation of concerns.

---

**Answer**: Part 1: `8028` | Part 2: `8798438007673`

**Related patterns**: [[circular-list]], [[modular-arithmetic]], [[index-tracking]]
