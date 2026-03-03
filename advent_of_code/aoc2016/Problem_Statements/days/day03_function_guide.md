# Day 3: Squares With Three Sides — Function Guide

**Problem**: Given lists of three side lengths, count how many form valid triangles.
**Answers**: Part 1 = **993**, Part 2 = **1849**
**Code**: [day03.rs](../../src/solver/day03.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Data Structures](#data-structures)
3. [parse_input](#parse_input)
4. [is_valid_triangle](#is_valid_triangle)
5. [solve_part1_with_data / solve_part2_with_data](#solve_part1_with_data--solve_part2_with_data)
6. [Benchmarks](#benchmarks)
7. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: 1911 lines, each with 3 whitespace-separated integers (side lengths).

```
  541  588  421
  827  272  126
  660  514  367
```

**Part 1**: Read each row as a triangle. Count how many are valid.
A triangle is valid when the sum of any two sides exceeds the third.

**Part 2**: Read triangles **vertically** — each group of 3 rows provides 3 triangles from columns 0, 1, 2.

```
Row 1:  541  588  421       Triangle A: (541, 827, 660)  ← column 0
Row 2:  827  272  126   →   Triangle B: (588, 272, 514)  ← column 1
Row 3:  660  514  367       Triangle C: (421, 126, 367)  ← column 2
```

**Why Part 2 is tricky**: You have to mentally re-orient from row-major to column-major reading. The input itself doesn't change — only how you group the numbers into triangles.

---

## Data Structures

### `[u32; 3]`

Each triangle is a fixed-size array of 3 unsigned integers. Using `[u32; 3]` rather than `Vec<u32>` or a tuple:
- **Fixed size**: Exactly 3 sides, enforced at the type level
- **No heap allocation**: Array lives on the stack (12 bytes)
- **Indexable**: `chunk[row][col]` works naturally for Part 2's column access

---

## `parse_input`

```rust
fn parse_input(input: &str) -> Vec<[u32; 3]> {
    input
        .lines()
        .map(|line| {
            let nums: Vec<u32> = line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            [nums[0], nums[1], nums[2]]
        })
        .collect()
}
```

**Input**: Raw puzzle input string.
**Output**: `Vec<[u32; 3]>` — one 3-element array per line.

`split_whitespace()` handles the variable-width spacing in the input (e.g., `"  541  588  421"`). No need for fixed-width column parsing.

**Note**: Parsing dominates runtime (~117µs). The actual triangle validation is nearly free by comparison.

---

## `is_valid_triangle`

### Direct 3-check approach (used)

```rust
fn is_valid_triangle(&[a, b, c]: &[u32; 3]) -> bool {
    a + b > c && a + c > b && b + c > a
}
```

Checks all three inequality constraints directly. The `&&` chain short-circuits: if the first check fails, the remaining checks are skipped.

**Rust pattern**: Destructuring `&[a, b, c]` in the function signature extracts all three values — no indexing needed.

### Sort-based approach (benchmark comparison)

```rust
pub fn is_valid_triangle_sort(&[a, b, c]: &[u32; 3]) -> bool {
    let mut s = [a, b, c];
    s.sort();
    s[0] + s[1] > s[2]
}
```

Sort so `s[0] ≤ s[1] ≤ s[2]`, then only check `smallest + middle > largest`. The other two inequalities are automatically satisfied when `s[2]` is the largest.

### Theoretical comparison

| Approach | Comparisons | Data movement | Short-circuit? |
|----------|-------------|---------------|----------------|
| Direct 3-check | 1–3 | None | Yes — fails fast on first violation |
| Sort + 1 check | 2–3 (sort) + 1 | Swaps | No — sort always completes |

**Prediction**: Direct should win on invalid triangles (short-circuit after 1 check) but both should be similar on valid ones.

**Reality**: Both measure identically at ~118µs because **parsing dominates** (~99% of runtime). The validation loop over 1911 triangles is negligible either way.

---

## `solve_part1_with_data` / `solve_part2_with_data`

### Part 1 — Row-major

```rust
fn solve_part1_with_data(triangles: &[[u32; 3]]) -> usize {
    triangles.iter().filter(|t| is_valid_triangle(t)).count()
}
```

Simple filter-and-count. Each row is already a triangle.

### Part 2 — Column-major via `chunks(3)`

```rust
fn solve_part2_with_data(rows: &[[u32; 3]]) -> usize {
    rows.chunks(3)
        .flat_map(|chunk| {
            (0..3).map(move |col| [chunk[0][col], chunk[1][col], chunk[2][col]])
        })
        .filter(|t| is_valid_triangle(t))
        .count()
}
```

**Step by step**:
1. `chunks(3)` — group rows into sets of 3
2. `flat_map` — each chunk yields 3 triangles (one per column)
3. `[chunk[0][col], chunk[1][col], chunk[2][col]]` — read down the column
4. Filter and count as before

**Why `flat_map` not `map`**: Each chunk produces 3 triangles, not 1. `flat_map` flattens the iterator of iterators into a single stream.

**Example trace**:
```
Chunk: [[541, 588, 421],
        [827, 272, 126],
        [660, 514, 367]]

col=0 → [541, 827, 660]  valid? sort→[541,660,827], 541+660=1201>827 ✓
col=1 → [588, 272, 514]  valid? sort→[272,514,588], 272+514=786>588  ✓
col=2 → [421, 126, 367]  valid? sort→[126,367,421], 126+367=493>421  ✓
```

---

## Benchmarks

| Function | Time |
|----------|------|
| `solve_part1` (direct) | 119.4µs |
| `solve_part1_sort` (sort) | 117.6µs |
| `solve_part2` | 118.9µs |
| `solve` (combined) | 121.3µs |

**Key observation**: All four benchmarks are within ~3µs of each other. This tells us:
- **Parsing dominates**: `split_whitespace().parse()` on 1911 lines costs ~117µs
- **Validation is free**: Checking 1911 triangles adds <2µs regardless of approach
- **Sort vs direct**: No measurable difference — the sort operates on 3 elements (effectively constant-time), and the compiler likely optimizes it to a sorting network

**Combined ≈ Part 1**: Because Part 2 reuses the same parsed data, `solve()` only adds one more pass over the already-parsed array — essentially free.

**At ~121µs total, well under the 100ms target — no optimization needed.**

---

## Key Patterns

### Array destructuring in function signatures

```rust
fn is_valid_triangle(&[a, b, c]: &[u32; 3]) -> bool {
```

Rust lets you destructure the array reference right in the parameter. This is equivalent to:
```rust
fn is_valid_triangle(sides: &[u32; 3]) -> bool {
    let (a, b, c) = (sides[0], sides[1], sides[2]);
```

Cleaner, no indexing, and the compiler enforces that the array has exactly 3 elements.

### `chunks(3)` for column-major reading

When the problem redefines how data is grouped, `chunks(n)` is the go-to tool. It produces non-overlapping windows without any allocation — just slices into the existing `Vec`.

```rust
// Row-major: each row is a triangle
rows.iter()

// Column-major: each group of 3 rows yields 3 triangles
rows.chunks(3).flat_map(|chunk| /* read columns */)
```

### Short-circuit `&&` for multi-condition checks

```rust
a + b > c && a + c > b && b + c > a
```

Rust's `&&` is lazy: if `a + b > c` is false, the triangle is already invalid and the remaining checks are skipped. For the ~48% invalid triangles in this input, many fail on the first check.

### Parse-once pattern (continued)

Same pattern as Days 1 and 2: parse the input once into `Vec<[u32; 3]>`, then pass the parsed data to both parts. Since parsing is ~99% of runtime here, this matters even more than usual.

---

**Navigation**: [<- Day 2](day02_function_guide.md) | [All Days](../summary_2016.md) | [Day 4 ->](day04.md)
