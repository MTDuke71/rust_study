# Day 6: Tuning Trouble - Function Guide

**Quick Links**: [← Day 5](day05_function_guide.md) | [Problem Statement](day06.md) | [Summary](../summary_2022.md) | [Day 7 →](day07_function_guide.md)

---

## Problem Overview
Find the first position in a datastream where the last N characters are all unique. Part 1 uses a window of 4 (start-of-packet marker). Part 2 uses a window of 14 (start-of-message marker).

**Answer**: Part 1: `1140` | Part 2: `3495`

## Performance Benchmarks
- **Part 1**: 1.46µs
- **Part 2**: 10.06µs
- **Combined**: 11.43µs
- **Parse**: N/A (no parsing step — works directly on raw bytes)

**Note**: This is the fastest day so far and the simplest solution (~90 lines). No separate parse step means zero overhead from data transformation.

## Core Algorithm: Sliding Window + Bitset Uniqueness

Slide a fixed-size window across the input bytes. For each window position, build a u32 bitset where each bit represents a letter (a=0, b=1, ..., z=25). If `popcount == window_size`, all characters are unique — we found the marker.

**Algorithm**:
1. Convert input to byte slice (`as_bytes()`)
2. Use `.windows(n)` iterator for sliding window
3. For each window, build bitset via `bits |= 1 << (byte - b'a')`
4. Check `bits.count_ones() == window_size`
5. Return 1-based position: `index + window_size`

**Time Complexity**: O(n × w) where n = input length, w = window size  
**Space Complexity**: O(1) — single u32 register, no heap allocation

---

## Function Reference

### Core Algorithm

#### `find_marker(input: &[u8], window_size: usize) -> usize`
**Purpose**: Find first position where last `window_size` bytes are all unique

```rust
fn find_marker(input: &[u8], window_size: usize) -> usize {
    input
        .windows(window_size)
        .position(|window| {
            let mut bits: u32 = 0;
            for &b in window {
                bits |= 1 << (b - b'a');
            }
            bits.count_ones() as usize == window_size
        })
        .map(|pos| pos + window_size)
        .expect("No marker found in input")
}
```

**Key Points**:
- **Input**: Raw byte slice — avoids any string allocation or char conversion
- **Bitset**: `u32` is sufficient for 26 lowercase letters (only 26 of 32 bits used)
- **Uniqueness check**: If any letter repeats, its bit is already set, so `count_ones() < window_size`
- **Return value**: 1-based position (number of characters processed), not 0-based index

**Why u32 not u128?**: Day 3 used u128 because it needed to represent all ASCII (a-z, A-Z = 52 values). Day 6 only has lowercase letters, so u32 (32 bits) is more than enough and fits in a single register.

**Complexity**: O(n × w) per call  
**Used By**: `solve_part1_impl()`, `solve_part2_impl()`

---

### Part 1 & Part 2 Entry Points

#### `solve_part1_impl(input: &str) -> usize`
**Purpose**: Find start-of-packet marker (4 unique characters)

```rust
fn solve_part1_impl(input: &str) -> usize {
    find_marker(input.trim().as_bytes(), 4)
}
```

**Window Size**: 4 — finds the first 4-character sequence with all unique chars

---

#### `solve_part2_impl(input: &str) -> usize`
**Purpose**: Find start-of-message marker (14 unique characters)

```rust
fn solve_part2_impl(input: &str) -> usize {
    find_marker(input.trim().as_bytes(), 14)
}
```

**Window Size**: 14 — finds the first 14-character sequence with all unique chars

---

### Public API

#### `solve(input: &str) -> (usize, usize)`
**Purpose**: Solve both parts

```rust
pub fn solve(input: &str) -> (usize, usize) {
    (solve_part1_impl(input), solve_part2_impl(input))
}
```

**Note**: Unlike Day 5, there is no shared parsed state — each part independently scans the input. The combined benchmark (11.43µs) is roughly Part 1 + Part 2, confirming no parse-once optimization is needed (or possible) here.

---

## Algorithm Deep Dive

### Why Bitset Works

The problem asks: "are all characters in this window unique?" The bitset answers this in O(w) time with O(1) space:

```
Window: "jpqm"
  j → bit 9 set     → bits = 0000...0000_0010_0000_0000
  p → bit 15 set    → bits = 0000...1000_0010_0000_0000
  q → bit 16 set    → bits = 0001...1000_0010_0000_0000
  m → bit 12 set    → bits = 0001...1001_0010_0000_0000

count_ones() = 4 == window_size(4) → All unique! ✓
```

If a character repeats, its bit is already set — OR is idempotent:
```
Window: "jpjm"
  j → bit 9 set     → bits = 0000...0000_0010_0000_0000
  p → bit 15 set    → bits = 0000...1000_0010_0000_0000
  j → bit 9 ALREADY SET → bits unchanged
  m → bit 12 set    → bits = 0000...1001_0010_0000_0000

count_ones() = 3 ≠ 4 → Duplicate detected! ✗
```

### Part 2 is ~7× Slower Than Part 1

| Factor | Part 1 (w=4) | Part 2 (w=14) |
|--------|-------------|---------------|
| Window size | 4 bytes | 14 bytes |
| Inner loop iterations | 4 per window | 14 per window |
| Positions checked | ~1140 | ~3495 |
| Time | 1.46µs | 10.06µs |

Part 2 is slower because:
1. **Larger window** → 3.5× more work per position (14 vs 4 OR operations)
2. **Later marker** → 3× more positions to check before finding answer (3495 vs 1140)
3. **Combined**: ~3.5 × ~2 ≈ ~7× slowdown — matches observed 6.9× ratio

### Alternative Approaches Not Taken

| Approach | Pros | Cons |
|----------|------|------|
| **HashSet per window** | Readable, handles any chars | Heap allocation per window, ~10-50× slower |
| **Rolling hash** | O(1) per slide | Complex implementation, hash collisions |
| **Frequency counter** | O(w) update, O(1) check | Array of 26 counters, more bookkeeping |
| **Bitset (chosen)** | Zero alloc, simple, fast | Only works for bounded char set |

The frequency counter with rolling updates (add entering char, remove leaving char) would give O(1) per slide instead of O(w), but for w ≤ 14 the constant factor overhead makes it slower in practice. The bitset approach wins by being dead simple.

---

## Rust Techniques

### `.windows(n)` Iterator
```rust
// Creates overlapping windows of size n from a slice
[1, 2, 3, 4, 5].windows(3)
// Yields: [1,2,3], [2,3,4], [3,4,5]
```
**Key property**: Yields `&[T]` slices — zero allocation, just pointer arithmetic over the original data.

### `.position()` for First Match
```rust
iterator.position(|item| predicate(item))
// Returns: Option<usize> — index of first match
```
**Short-circuits**: Stops iterating as soon as the predicate returns true. Perfect for "find first" problems.

### `count_ones()` / Popcount
```rust
let bits: u32 = 0b1010_1100;
bits.count_ones()  // Returns 4
```
**Hardware instruction**: Maps to x86 `POPCNT` — single cycle on modern CPUs. This makes the uniqueness check essentially free.

### No Parse Step Required
This is the only AoC 2022 problem so far that needs no parsing:
- Input is a single line of lowercase ASCII
- `trim().as_bytes()` is the entire "parse" — just a pointer + length

---

## Test Cases

### Part 1 Examples (window = 4)
| Input | Expected | First Unique Window |
|-------|----------|-------------------|
| `mjqjpqmgbljsphdztnvjfqwrcgsmlb` | 7 | `jpqm` at positions 4-7 |
| `bvwbjplbgvbhsrlpgdmjqwftvncz` | 5 | `vwbj` at positions 2-5 |
| `nppdvjthqldpwncqszvftbrmjlhg` | 6 | `pdvj` at positions 3-6 |
| `nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg` | 10 | `rfnt` at positions 7-10 |
| `zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw` | 11 | `zqfr` at positions 8-11 |

### Part 2 Examples (window = 14)
| Input | Expected |
|-------|----------|
| `mjqjpqmgbljsphdztnvjfqwrcgsmlb` | 19 |
| `bvwbjplbgvbhsrlpgdmjqwftvncz` | 23 |
| `nppdvjthqldpwncqszvftbrmjlhg` | 23 |
| `nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg` | 29 |
| `zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw` | 26 |

---

## Key Insights

1. **Simplest problem, simplest solution**: ~90 lines total including tests. Both parts differ only in a single constant (4 vs 14). The right abstraction (`find_marker` parameterized by window size) makes the whole thing trivial.

2. **Bitset reuse from Day 3**: Same core technique (u32/u128 bitmask for character sets), different application. Day 3 used intersection (AND); Day 6 uses uniqueness (popcount). Pattern recognition across problems is the real skill.

3. **No parsing needed**: Working directly on `&[u8]` eliminates an entire category of complexity. The input format is so simple that transformation would only add overhead.

4. **`.windows()` is the perfect tool**: Rust's stdlib iterator produces exactly the sliding windows we need with zero allocation. This is why knowing your stdlib matters — without `.windows()`, you'd write manual index arithmetic.

5. **Hardware popcount**: `count_ones()` compiles to a single CPU instruction (`POPCNT`), making the bitset uniqueness check essentially free compared to any collection-based approach.

---

## Zettelkasten Links

*Mathematics & Algorithms*:
- [[sliding-window-technique]] - Fixed-size window over sequential data
- [[bitset-optimization]] - Bit manipulation for set operations (reused from Day 3)
- [[set-theory-fundamentals]] - Uniqueness = set cardinality equals input count

*Rust Patterns*:
- [[iterator-patterns]] - `.windows()`, `.position()`, method chaining
- [[aoc-parsing-patterns]] - Minimal parsing (no parse step needed)

*Related Problems*:
- Day 3: Same bitset technique for set intersection
- Future sliding window problems will reuse `find_marker` pattern

---

**Navigation**: [← Day 5](day05_function_guide.md) | [Problem](day06.md) | [Summary](../summary_2022.md) | [Day 7 →](day07_function_guide.md)
