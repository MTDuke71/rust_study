# Day 10: Knot Hash — Function Guide

**Problem**: Implement a custom hash function based on circular list reversals — single round for Part 1, full 64-round dense hash for Part 2.
**Answers**: Part 1 = **20,056**, Part 2 = **d9a7de4a809c56bf3a9465cb84392c8e**
**Code**: [day10.rs](../../src/solver/day10.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Data Structures](#data-structures)
3. [knot_round](#knot_round)
4. [solve_part1_with_data](#solve_part1_with_data)
5. [solve_part2_with_data](#solve_part2_with_data)
6. [Benchmarks](#benchmarks)
7. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: `83,0,193,1,254,237,187,40,88,27,2,255,149,29,42,100`

A circular list of 256 values (0–255) is repeatedly modified by reversing spans of elements.

**Part 1** (single round):
- Parse input as comma-separated integers → lengths
- For each length: reverse that many elements starting at current position (wrapping), advance position by length + skip, increment skip
- Answer: product of first two elements

**Part 2** (full Knot Hash):
- Parse input as ASCII byte values (e.g., `'1'` = 49, `','` = 44)
- Append suffix `[17, 31, 73, 47, 23]`
- Run 64 rounds (position and skip persist across rounds)
- Dense hash: XOR each block of 16 → 16 bytes → 32-char hex string

**Example** (Part 1, list size 5):
```
List: [0, 1, 2, 3, 4], lengths: [3, 4, 1, 5]
After all reversals: [3, 4, 2, 1, 0]
Answer: 3 × 4 = 12
```

---

## Data Structures

No custom structs — the algorithm operates directly on a `Vec<u8>` (the circular list) with mutable `pos` and `skip` counters passed by reference.

The circular list uses `% n` modular arithmetic for all index calculations, avoiding actual rotation or reallocation.

---

## `knot_round`

```rust
fn knot_round(list: &mut [u8], lengths: &[usize], pos: &mut usize, skip: &mut usize)
```

Performs one complete round of knot-tying. For each length:
1. Reverse the span `[pos..pos+len]` using two-pointer swap with modular wrapping
2. Advance `pos` by `len + skip`
3. Increment `skip`

**Why `pos` and `skip` are `&mut`**: Part 2 runs 64 rounds where state carries over. By taking mutable references, the caller controls lifetime and persistence.

**Circular reversal**: Two pointers `a` (front) and `b` (back) walk inward, swapping elements. Both use `% n` for wrapping — handles spans that cross the list boundary naturally.

**Complexity**: O(sum of lengths) per round. For Part 2: 64 rounds × ~60 lengths × ~128 avg reversal = ~500K swaps.

---

## `solve_part1_with_data`

```rust
fn solve_part1_with_data(lengths: &[usize]) -> u32
```

Creates a fresh 0–255 list, runs one `knot_round`, returns `list[0] * list[1]`.

---

## `solve_part2_with_data`

```rust
fn solve_part2_with_data(input: &str) -> String
```

Three phases:

1. **ASCII conversion**: Input bytes become lengths, suffix `[17, 31, 73, 47, 23]` appended
2. **64 rounds**: Same `knot_round` function, `pos` and `skip` persist between rounds
3. **Dense hash**: 256 elements → 16 blocks of 16 → XOR each block → format as 2-digit hex

The XOR reduction uses `fold(0u8, |acc, &x| acc ^ x)` — idiomatic Rust for reducing a slice.

---

## Benchmarks

| Function | Time |
|----------|------|
| `solve` (combined) | 54.92µs |

**Performance breakdown**: Dominated by Part 2's 64 rounds of list manipulation. The `format!("{:02x}")` for hex output adds minor overhead.

**Reuse note**: `knot_round` is factored out as a standalone function. Day 14 reuses the full Knot Hash to generate grid data.

---

## Key Patterns

### Circular buffer with modular arithmetic

Instead of physically rotating the list or using a deque, all indices use `% n`. The two-pointer reversal handles wrapping naturally:
```rust
a = (a + 1) % n;     // forward pointer wraps at end
b = (b + n - 1) % n; // backward pointer wraps at start
```

This is a general pattern for any circular data structure — avoid reallocation, just do modular math.

### State persistence across rounds

Part 2's key requirement is that `pos` and `skip` carry over between rounds. The function signature makes this explicit with `&mut` references — the caller owns the state, the function borrows it. This is cleaner than returning a tuple of state values.

### Multi-stage hash construction

The Knot Hash follows a real cryptographic pattern:
1. **Input padding**: Append fixed suffix bytes
2. **Multiple rounds**: Iterated mixing (like SHA's rounds)
3. **Compression**: 256 bytes → 16 bytes via XOR (sparse → dense hash)
4. **Encoding**: Binary → hex string

### Dual input interpretation

The same raw input string is parsed two different ways:
- Part 1: `"83,0,193"` → numbers `[83, 0, 193]`
- Part 2: `"83,0,193"` → ASCII bytes `[56, 51, 44, 48, 44, 49, 57, 51]`

This is why `solve()` takes the raw string and handles both interpretations internally.
