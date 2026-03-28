# Project Euler Problem 22: Names Scores

**Solved**: 2026-03-28
**Difficulty**: 5%
**Category**: String Processing / Sorting

## Problem Summary

Sort 5,163 names alphabetically, compute each name's alphabetical value (A=1, ..., Z=26), multiply by sorted position, and sum all name scores.

## Mathematical Concepts

### Primary Concepts
- **Ordinal encoding** — mapping characters to numerical values: `value(c) = c - 'A' + 1`
- **Weighted sum** — name score = alphabetical_value × position

### Supporting Concepts
- **Lexicographic ordering** — standard string comparison for alphabetical sorting
- **Comparison-based sorting** — O(n log n) lower bound applies; Rust uses TimSort

## Solution Approach

### Step 1: Parse
Input format is `"NAME1","NAME2",...` — split on commas, strip quotes.

### Step 2: Sort
Standard lexicographic sort. Rust's `sort()` uses a stable TimSort variant — O(n log n) worst case, O(n) on nearly-sorted data.

### Step 3: Score
For each name at 1-based position `i`:
```text
score(name, i) = i × Σ (c - 'A' + 1) for each character c in name
```

### Verification
COLIN at position 938: `(3+15+12+9+14) × 938 = 53 × 938 = 49,714` ✓

## Complexity

- **Time**: O(n log n) for sort + O(n × k) for scoring, where k = avg name length
- **Space**: O(n) for storing parsed names

## Rust Implementation Details

### `include_str!` for Compile-Time File Embedding
```rust
let input = include_str!("../../Problem_Statements/0022_names.txt");
```
Embeds the file contents into the binary at compile time — no filesystem access at runtime. This is idiomatic Rust for static data files.

### Byte Arithmetic for Character Values
```rust
name.bytes()
    .filter(|b| b.is_ascii_uppercase())
    .map(|b| (b - b'A' + 1) as u64)
    .sum()
```
Operating on `bytes()` instead of `chars()` avoids UTF-8 decoding overhead. Since all names are ASCII uppercase, this is both correct and efficient.

### Iterator Chain for Scoring
```rust
names.iter()
    .enumerate()
    .map(|(i, name)| alphabetical_value(name) * (i as u64 + 1))
    .sum()
```
Zero-allocation scoring — no intermediate collections needed.

## Performance

| Metric | Value |
|--------|-------|
| Time | 746 µs |
| Target | < 100 ms |
| Bottleneck | Parsing + sorting 5,163 names |

## Answer

**871,198,282**

## Related Problems

- **Problem 42** (Triangle Words) — also uses alphabetical values, checks if value is a triangle number
- **Problem 79** (Passcode Derivation) — another file-parsing + sorting problem

## Links

- [[set-theory-fundamentals]] — ordinal encoding relates to bijection between alphabet and ℕ₂₆
- [[project-euler-p021]] — previous problem (also O(n log n) but via sieve, not sort)
