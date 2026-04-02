# Day 2: Corruption Checksum — Function Guide

**Problem**: Compute checksums from a spreadsheet — row range differences (Part 1) and evenly divisible quotients (Part 2).
**Answers**: Part 1 = **41919**, Part 2 = **303**
**Code**: [day02.rs](../../src/solver/day02.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Data Structures](#data-structures)
3. [parse_input](#parse_input)
4. [solve_part1_with_data](#solve_part1_with_data)
5. [solve_part2_with_data](#solve_part2_with_data)
6. [Benchmarks](#benchmarks)
7. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: A spreadsheet of tab-separated integers, one row per line.

```
5 1 9 5
7 5 3
2 4 6 8
```

**Part 1**: For each row, compute `max - min`. Sum all row differences.

**Part 2**: For each row, find the only pair of numbers where one evenly divides the other. Sum all quotients.

**Example (Part 1)**: `[5,1,9,5]` → 9-1=8, `[7,5,3]` → 7-3=4, `[2,4,6,8]` → 8-2=6 → sum = **18**
**Example (Part 2)**: `[5,9,2,8]` → 8/2=4, `[9,4,7,3]` → 9/3=3, `[3,8,6,5]` → 6/3=2 → sum = **9**

**Why Part 2 is different**: Part 1 uses extremes (min/max), Part 2 uses pairwise divisibility. Both are O(n·m) but Part 2 has a nested inner loop over pairs.

---

## Data Structures

### Row matrix

```rust
Vec<Vec<u32>>  // e.g., "5 1 9 5\n7 5 3" → [[5,1,9,5], [7,5,3]]
```

Each row is a `Vec<u32>`. No struct needed — the data is naturally tabular with variable-length rows.

---

## `parse_input`

```rust
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}
```

**Input**: `"5 1 9 5\n7 5 3\n2 4 6 8\n"`
**Output**: `[[5,1,9,5], [7,5,3], [2,4,6,8]]`

- `split_whitespace()` handles both tabs and spaces (actual puzzle input uses tabs)
- `trim()` removes trailing newline from puzzle input
- Nested iterator: outer over lines, inner over values

---

## `solve_part1_with_data`

```rust
fn solve_part1_with_data(rows: &[Vec<u32>]) -> u32 {
    rows.iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum()
}
```

**Trace on example**:
```
[5,1,9,5] → max=9, min=1 → 8
[7,5,3]   → max=7, min=3 → 4
[2,4,6,8] → max=8, min=2 → 6
Sum: 18 ✓
```

**Complexity**: O(n·m) where n = rows, m = columns. Two passes per row for `max()` and `min()` — could be done in one pass, but the iterator version is clearer and the data is tiny (16×16 = 256 values).

---

## `solve_part2_with_data`

```rust
fn solve_part2_with_data(rows: &[Vec<u32>]) -> u32 {
    rows.iter()
        .map(|row| {
            for (i, &a) in row.iter().enumerate() {
                for &b in &row[i + 1..] {
                    let (big, small) = if a > b { (a, b) } else { (b, a) };
                    if big % small == 0 {
                        return big / small;
                    }
                }
            }
            unreachable!("each row should have an evenly divisible pair")
        })
        .sum()
}
```

**Trace on example**:
```
[5,9,2,8]:
  (5,9) → 9%5=4 ✗
  (5,2) → 5%2=1 ✗
  (5,8) → 8%5=3 ✗
  (9,2) → 9%2=1 ✗
  (9,8) → 9%8=1 ✗
  (2,8) → 8%2=0 ✓ → 8/2 = 4

[9,4,7,3]:
  (9,4) → 9%4=1 ✗
  (9,7) → 9%7=2 ✗
  (9,3) → 9%3=0 ✓ → 9/3 = 3

[3,8,6,5]:
  (3,8) → 8%3=2 ✗
  (3,6) → 6%3=0 ✓ → 6/3 = 2

Sum: 4+3+2 = 9 ✓
```

**Complexity**: O(n·m²) worst case per row. Each row checks pairs until finding the divisible one. With 16 values per row, worst case is C(16,2) = 120 pairs — negligible.

**Design choices**:
- `&row[i + 1..]` avoids checking pairs twice (a,b) and (b,a)
- `let (big, small)` normalizes ordering so `%` and `/` always use the larger value
- `unreachable!()` documents the guarantee from the puzzle that exactly one pair exists per row

---

## Benchmarks

| Function | Time |
|----------|------|
| `solve` (combined) | 4.87µs |

**Performance**: 4.87µs in release mode. The input is 16 rows × 16 columns = 256 values. Parsing and both parts are dominated by trivial integer operations with excellent cache locality.

---

## Key Patterns

### `split_whitespace()` for flexible delimiters

```rust
line.split_whitespace()
```

Handles tabs, spaces, or any whitespace — more robust than `split('\t')`. AoC inputs sometimes mix whitespace types, so this is the safer default for numeric tables.

### Pair iteration with index slicing

```rust
for (i, &a) in row.iter().enumerate() {
    for &b in &row[i + 1..] {
```

Classic O(n²/2) pair enumeration that avoids duplicate pairs. The `&row[i + 1..]` slice starts after the current element, so each pair (a, b) is checked exactly once. This is the same pattern used in collision detection, closest-pair problems, and two-sum variants.

### Normalize before comparing

```rust
let (big, small) = if a > b { (a, b) } else { (b, a) };
```

When the operation isn't commutative (division), normalizing the pair order simplifies the logic. One check `big % small == 0` replaces two checks `a % b == 0 || b % a == 0`.

### Parse-once pattern

Both parts share the same parsed `Vec<Vec<u32>>`. Parsing happens once in `solve()`, and both computations reuse the same data. For this problem parsing is cheap (~16 lines), but maintaining the pattern keeps the codebase consistent.
