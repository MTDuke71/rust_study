# Day 6: Signals and Noise --- Function Guide

**Problem**: Decode error-corrected messages by finding most/least common characters per column.
**Answers**: Part 1 = **zcreqgiv**, Part 2 = **pljvorrk**
**Code**: [day06.rs](../../src/solver/day06.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Algorithm Overview](#algorithm-overview)
3. [parse_input](#parse_input)
4. [solve_part1_with_data](#solve_part1_with_data)
5. [solve_part2_with_data](#solve_part2_with_data)
6. [Benchmarks](#benchmarks)
7. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: 572 lines of 8-character lowercase strings representing a noisy repetition-coded message.

**Part 1**: For each column position, find the **most common** character. Concatenate them to form the error-corrected message.

**Part 2**: For each column position, find the **least common** character (modified repetition code). Concatenate them to form the message.

**Example** (6-char wide, 16 messages):
```
eedadn    Column frequencies:
drvtee      col 0: e(3) d(2) r(1) a(1) t(1) s(2) n(2) v(2) -> most='e', least='a'
eandsr      col 1: e(2) r(1) a(3) t(1) s(2) n(2) d(1) v(2) -> most='a', least='d'
...         ...
```
- Part 1: `easter`
- Part 2: `advent`

---

## Algorithm Overview

Single-pass frequency accumulation into fixed-size `[u32; 26]` arrays (one per column), then extract max/min per column.

```
Parse:  572 lines x 8 cols -> Vec<[u32; 26]> (8 frequency arrays)
Part 1: For each array, find index with max count -> char
Part 2: For each array, find index with min count (>0) -> char
```

**Complexity**: O(n * w) where n = line count, w = message width. Constant-factor fast due to fixed 26-element arrays.

---

## `parse_input`

```rust
fn parse_input(input: &str) -> Vec<[u32; 26]>
```

Builds one `[u32; 26]` frequency array per column position. Iterates each line's bytes and increments `freq[col][(byte - b'a')]`.

**Key design**: Uses `bytes()` not `chars()` since input is ASCII-only. Zero allocation beyond the output vector.

---

## `solve_part1_with_data`

```rust
fn solve_part1_with_data(freq: &[[u32; 26]]) -> String
```

For each column's frequency array, finds the index with the maximum count via `max_by_key`, converts back to `char` with `(b'a' + idx as u8) as char`.

---

## `solve_part2_with_data`

```rust
fn solve_part2_with_data(freq: &[[u32; 26]]) -> String
```

Same as Part 1 but uses `min_by_key` with a `.filter(|&(_, &c)| c > 0)` guard to skip characters that never appeared. Without the filter, all 26 slots tie at 0 and the wrong character would be selected.

---

## Benchmarks

| Benchmark | Time |
|-----------|------|
| Combined  | 8.1us |
| Part 1    | 8.4us |
| Part 2    | 7.9us |

Combined < sum of parts confirms parse-once pattern working correctly.

---

## Key Patterns

| Pattern | Description |
|---------|-------------|
| Fixed-size freq array | `[u32; 26]` for lowercase alphabet --- same as Day 4 |
| Parse-once | Single parse into `Vec<[u32; 26]>`, both parts share data |
| Zero-filter for min | `.filter(\|c\| c > 0)` prevents unused slots from winning min |
| Byte arithmetic | `byte - b'a'` for index, `b'a' + idx` for char --- avoids char conversion overhead |
| Dual extraction | Same data structure, different aggregation (max vs min) |
