# Day 1: Inverse Captcha — Function Guide

**Problem**: Find digits in a circular list that match a neighbor at a specific offset.
**Answers**: Part 1 = **1171**, Part 2 = **1024**
**Code**: [day01.rs](../../src/solver/day01.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Data Structures](#data-structures)
3. [parse_digits](#parse_digits)
4. [solve (Part 1 + Part 2)](#solve)
5. [Benchmarks](#benchmarks)
6. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: A single string of digits like `91212129`.

**Part 1**: Sum all digits where the digit equals the **next** digit in the circular list (last wraps to first).

**Part 2**: Sum all digits where the digit equals the one **halfway around** the list. The list always has an even number of digits.

**Example (Part 1)**: `1122` → 1 matches next 1, second 2 matches next 2 → sum = **3**
**Example (Part 2)**: `1212` → every digit matches its halfway partner → sum = **6**

**Why Part 2 is different**: The offset changes from 1 to `len/2`. Same algorithm, different stride — both are O(n) single-pass.

---

## Data Structures

### Digit vector

```rust
Vec<u32>  // e.g., "91212129" → [9, 1, 2, 1, 2, 1, 2, 9]
```

Digits stored as `u32` values (0–9) for direct arithmetic. No struct needed — this is pure numeric processing.

---

## `parse_digits`

```rust
fn parse_digits(input: &str) -> Vec<u32> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}
```

**Input**: `"1122\n"`
**Output**: `[1, 1, 2, 2]`

- `trim()` strips the trailing newline from puzzle input
- `to_digit(10)` converts a char to its numeric value in base 10 (returns `Option<u32>`)
- `unwrap()` is safe because AoC guarantees all-digit input

---

## `solve`

```rust
pub fn solve(input: &str) -> (u32, u32) {
    let digits = parse_digits(input);
    let len = digits.len();

    // Part 1: compare each digit with the next (wrapping around)
    let part1: u32 = digits
        .iter()
        .enumerate()
        .filter(|&(i, &d)| d == digits[(i + 1) % len])
        .map(|(_, &d)| d)
        .sum();

    // Part 2: compare each digit with the one halfway around
    let half = len / 2;
    let part2: u32 = digits
        .iter()
        .enumerate()
        .filter(|&(i, &d)| d == digits[(i + half) % len])
        .map(|(_, &d)| d)
        .sum();

    (part1, part2)
}
```

Both parts use the same pattern: iterate with index, compare against `digits[(i + offset) % len]`, sum matches.

**Trace on Part 1 with `1122`** (offset = 1):
```
i=0: digits[0]=1, digits[1]=1  → match!  sum += 1
i=1: digits[1]=1, digits[2]=2  → no match
i=2: digits[2]=2, digits[3]=2  → match!  sum += 2
i=3: digits[3]=2, digits[0]=1  → no match
Result: 3 ✓
```

**Trace on Part 2 with `1212`** (offset = 2):
```
i=0: digits[0]=1, digits[2]=1  → match!  sum += 1
i=1: digits[1]=2, digits[3]=2  → match!  sum += 2
i=2: digits[2]=1, digits[0]=1  → match!  sum += 1
i=3: digits[3]=2, digits[1]=2  → match!  sum += 2
Result: 6 ✓
```

**Complexity**: O(n) time, O(n) space (for the digit vector). Single pass per part.

**Design choice**: Both parts live in one function because the parse-once pattern shares `digits`. The only difference is the offset (`1` vs `len/2`), so extracting a helper would add abstraction without value.

---

## Benchmarks

| Function | Time |
|----------|------|
| `solve` (combined) | 4.26µs |

**Performance**: 4.26µs in release mode (1000 iterations). Purely CPU-bound arithmetic with perfect cache locality over ~2000 digits.

---

## Key Patterns

### Modular indexing for circular lists

```rust
digits[(i + offset) % len]
```

The `% len` wrap makes the list circular. When `i + offset >= len`, it wraps back to the beginning. This is the same pattern as the direction rotation in 2016 Day 1, but here applied to array indexing rather than direction state.

### `filter` + `map` + `sum` pipeline

```rust
digits.iter()
    .enumerate()
    .filter(|&(i, &d)| d == digits[(i + 1) % len])
    .map(|(_, &d)| d)
    .sum()
```

This is a classic functional pipeline:
1. `enumerate()` — attach indices to elements
2. `filter()` — keep only elements that match their neighbor
3. `map()` — extract the digit value (discard the index)
4. `sum()` — accumulate

The `filter` + `map` could be replaced with `filter_map`, but the two-step version reads more clearly for this pattern.

### `to_digit(10)` for char-to-number conversion

```rust
c.to_digit(10).unwrap()
```

Built-in method on `char` — cleaner than `c as u32 - '0' as u32` and handles non-digit chars safely via `Option`. The base parameter (10) supports any radix from 2 to 36.

### Parse-once pattern

Both parts share the same parsed `Vec<u32>`. Parsing happens once in `solve`, and both computations reuse the same data. For this problem parsing is trivial, but maintaining the pattern keeps the codebase consistent across all 25 days.
