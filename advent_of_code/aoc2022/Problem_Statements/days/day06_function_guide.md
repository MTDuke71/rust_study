# Day 6: Tuning Trouble - Function Guide

**Quick Links**: [← Day 5](day05_function_guide.md) | [Problem Statement](day06.md) | [Summary](../summary_2022.md) | [Day 7 →](day07_function_guide.md)

---

## Problem Overview
Find the first position in a datastream where the last N characters are all unique. Part 1 uses a window of 4 (start-of-packet marker). Part 2 uses a window of 14 (start-of-message marker).

**Answer**: Part 1: `1140` | Part 2: `3495`

## Performance Benchmarks (Final — Rolling XOR Bitset)
- **Part 1**: 1.11µs
- **Part 2**: 3.69µs
- **Combined**: 4.80µs
- **Parse**: N/A (no parsing step — works directly on raw bytes)

**Note**: Fastest day by far. No parse step, single `u32` of state, branchless rolling updates.

## Optimization Journey

This solution went through three iterations, each building on insights from the previous:

| Version | Technique | Part 1 | Part 2 | Combined | Key Change |
|---------|-----------|--------|--------|----------|------------|
| **v1** | Rebuild bitset per window | 1.46µs | 10.06µs | 11.43µs | Baseline |
| **v2** | Rolling frequency counter | 1.48µs | 4.48µs | 5.96µs | O(1) per slide |
| **v3** | Rolling XOR bitset | **1.11µs** | **3.69µs** | **4.80µs** | Branchless |

**Total speedup**: 2.4× (11.43µs → 4.80µs)

### v1 → v2: Eliminate redundant work (O(n×w) → O(n))
**Problem identified**: Every window position rebuilt the entire bitset by OR-ing all `w` bytes. The overlapping bytes between adjacent windows were reprocessed.

**Solution**: Rolling frequency counter — add entering character, remove leaving character. Track unique count incrementally with branch logic (freq goes 0→1: unique++, 1→2: unique--, 2→1: unique++, 1→0: unique--).

**Result**: Part 1 unchanged (w=4 too small to matter), Part 2 cut from 10.06µs → 4.48µs (2.25×). The O(n×w) → O(n) complexity reduction pays off when `w` grows.

### v2 → v3: Eliminate branches (frequency array → XOR bitset)
**Problem identified**: The frequency counter needed 4 conditional branches per slide (two for entering char, two for leaving char), plus array read/write operations.

**Insight from user**: *"What about XOR out the exiting bit and XOR in the new bit?"* — XOR toggles bits, giving odd-count = ON, even-count = OFF. With exactly `w` characters in the window, `popcount == w` means `w` distinct bits are ON, so each character appears an odd number of times. Since we have only `w` total characters, each must appear exactly once.

**Solution**: Replace 26-byte frequency array + 4 branches with single `u32` + 2 XOR ops + 1 popcount. Completely branchless per-slide update.

**Result**: Part 1 improved 1.48µs → 1.11µs (25%), Part 2 improved 4.48µs → 3.69µs (18%). The branch elimination benefits both window sizes.

### Why Each Approach Was Worth Exploring

| Approach | Teaches | Limitation Discovered |
|----------|---------|----------------------|
| **v1: Rebuild bitset** | `.windows()` iterator, bitset basics, popcount | Redundant work — rebuilds entire bitset each slide |
| **v2: Freq counter** | Rolling window technique, incremental state | Branch-heavy — 4 conditionals per slide position |
| **v3: XOR bitset** | XOR toggle property, branchless design | None for this problem — optimal |

---

## Core Algorithm: Rolling XOR Bitset

For each byte position, XOR its bit into a `u32`. XOR toggles — first occurrence sets the bit ON, second toggles it OFF. When the window is full, also XOR out the leaving byte. Check `count_ones() == window_size` for uniqueness.

**Algorithm**:
1. Maintain a single `u32` bitset (26 bits for a-z)
2. For each byte: `bits ^= 1 << (byte - b'a')` — XOR in
3. When window full: `bits ^= 1 << (leaving - b'a')` — XOR out
4. Check `bits.count_ones() == window_size`
5. Return 1-based position: `i + 1`

**Time Complexity**: O(n) — constant work per position regardless of window size  
**Space Complexity**: O(1) — single u32 register, no heap allocation

---

## Function Reference

### Core Algorithm

#### `find_marker(input: &[u8], window_size: usize) -> usize`
**Purpose**: Find first position where last `window_size` bytes are all unique

```rust
fn find_marker(input: &[u8], window_size: usize) -> usize {
    let mut bits: u32 = 0;

    for (i, &b) in input.iter().enumerate() {
        // XOR in the entering character (toggles its bit)
        bits ^= 1 << (b - b'a');

        // XOR out the leaving character (once window is full)
        if i >= window_size {
            bits ^= 1 << (input[i - window_size] - b'a');
        }

        // Check: window is full and all chars are unique
        if i >= window_size - 1 && bits.count_ones() as usize == window_size {
            return i + 1;
        }
    }
    panic!("No marker found in input");
}
```

**Key Points**:
- **XOR toggle**: `^= 1 << bit_pos` flips the bit — ON if odd count, OFF if even count
- **Correctness**: With `w` total chars, `popcount == w` means `w` distinct chars each appear odd times → exactly once
- **Branchless update**: Two XOR ops per slide (no conditionals on character state)
- **Hardware popcount**: `count_ones()` maps to single `POPCNT` instruction

**Why u32 not u128?**: Day 3 used u128 for a-z + A-Z (52 values). Day 6 only has lowercase (26 values), so u32 suffices and fits in a single register.

**Complexity**: O(n) per call — window size doesn't affect per-position cost  
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

---

#### `solve_part2_impl(input: &str) -> usize`
**Purpose**: Find start-of-message marker (14 unique characters)

```rust
fn solve_part2_impl(input: &str) -> usize {
    find_marker(input.trim().as_bytes(), 14)
}
```

---

### Public API

#### `solve(input: &str) -> (usize, usize)`
**Purpose**: Solve both parts

```rust
pub fn solve(input: &str) -> (usize, usize) {
    (solve_part1_impl(input), solve_part2_impl(input))
}
```

**Note**: No shared parsed state — each part independently scans the input. Combined benchmark (4.80µs) is roughly Part 1 + Part 2.

---

## Algorithm Deep Dive

### Why XOR Works for Uniqueness

XOR toggles bits. In a window of `w` characters:
- Character appearing **1 time** (odd) → bit is **ON**
- Character appearing **2 times** (even) → bit is **OFF** (toggled back)
- Character appearing **3 times** (odd) → bit is **ON** again

If `count_ones() == w`, then `w` distinct bits are ON. Each of those `w` characters appears an odd number of times. But we only have `w` total characters — so each must appear exactly **once**.

```
Window: "jpqm" (all unique)
  j → XOR bit 9    → bits = ...0010_0000_0000    count=1
  p → XOR bit 15   → bits = ...1000_0010_0000_0000    count=2
  q → XOR bit 16   → bits = ..1_0000_1000_0010_0000_0000    count=3
  m → XOR bit 12   → bits = ..1_0000_1001_0010_0000_0000    count=4

count_ones() = 4 == window_size(4) → All unique! ✓
```

```
Window: "jpjm" (j appears twice)
  j → XOR bit 9    → bits = ...0010_0000_0000    count=1
  p → XOR bit 15   → bits = ...1000_0010_0000_0000    count=2
  j → XOR bit 9    → bits = ...1000_0000_0000_0000    count=1  (j toggled OFF!)
  m → XOR bit 12   → bits = ...1001_0000_0000_0000    count=2

count_ones() = 2 ≠ 4 → Duplicate detected! ✗
```

### Rolling Update Visualization

```
Input: a b c d e f ...
       └─────┘         Window [0..4]: XOR in a,b,c,d → check
         └─────┘       Window [1..5]: XOR out a, XOR in e → check
           └─────┘     Window [2..6]: XOR out b, XOR in f → check

Per slide: 2 XOR ops + 1 popcount (constant, regardless of window size)
```

### Part 2 vs Part 1 Scaling (After Optimization)

| Factor | Part 1 (w=4) | Part 2 (w=14) |
|--------|-------------|---------------|
| Work per slide | 2 XORs + popcount | 2 XORs + popcount (same!) |
| Positions checked | ~1140 | ~3495 |
| Time | 1.11µs | 3.69µs |
| Ratio | 1× | 3.3× |

After optimization, the ratio is **3.3×** (purely from checking more positions) vs the original **6.9×** (positions × window rebuilding). Eliminating per-slide window-size dependence collapsed the ratio to just the position difference.

### All Three Approaches Compared

```
v1: Rebuild bitset          v2: Frequency counter       v3: Rolling XOR
─────────────────           ─────────────────           ─────────────────
Per window:                 Per slide:                  Per slide:
  for b in window:            freq[entering] += 1         bits ^= entering
    bits |= 1 << b           freq[leaving] -= 1          bits ^= leaving
  popcount(bits)              update unique count         popcount(bits)
                              (4 branches)
                            
O(w) per position           O(1) per position           O(1) per position
w=14: 14 OR ops             4 branches + array ops      2 XORs + popcount
11.43µs combined            5.96µs combined             4.80µs combined
```

**v1 → v2**: Algorithmic improvement (eliminate redundant work)  
**v2 → v3**: Implementation improvement (eliminate branches)  
Both matter. The best solution combines the right algorithm with the right implementation.

---

## Rust Techniques

### XOR for Toggle
```rust
let mut bits: u32 = 0;
bits ^= 1 << 5;  // Set bit 5 ON
bits ^= 1 << 5;  // Toggle bit 5 OFF
bits ^= 1 << 5;  // Toggle bit 5 ON again
```
**Property**: XOR is its own inverse — `a ^ b ^ b == a`. This makes it perfect for rolling window add/remove without tracking counts.

### `count_ones()` / Popcount
```rust
let bits: u32 = 0b1010_1100;
bits.count_ones()  // Returns 4
```
**Hardware instruction**: Maps to x86 `POPCNT` — single cycle on modern CPUs.

### No Parse Step Required
This is the only AoC 2022 problem so far that needs no parsing:
- Input is a single line of lowercase ASCII
- `trim().as_bytes()` is the entire "parse" — just a pointer + length

### Direct Byte Indexing
```rust
input[i - window_size]  // Access leaving byte by index
```
Unlike v1's `.windows()` iterator approach, v3 uses direct byte indexing into the input slice. This avoids creating window sub-slices entirely.

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

1. **Question the approach, not just the algorithm**: The initial bitset solution was "correct" and fast enough — but asking "are we doing redundant work?" led to the rolling approach, and asking "can we remove branches?" led to XOR. Each question yielded measurable improvement.

2. **XOR is its own inverse**: `a ^ b ^ b == a`. This property makes XOR perfect for rolling window add/remove without needing a frequency array to track counts. One register instead of 26 bytes.

3. **Branchless beats branchy for tight loops**: The frequency counter (v2) needed 4 conditional branches per slide. XOR (v3) needs zero. On modern CPUs with branch prediction, this matters in tight inner loops where every cycle counts.

4. **Optimization has diminishing returns with window size**: v1→v2 helped Part 2 dramatically (10µs → 4.5µs) but not Part 1. v2→v3 helped both by constant factor. Know when your optimization targets algorithm complexity vs. constant factor.

5. **Bitset reuse from Day 3**: Third application of bit manipulation in 6 days. Day 3 used OR+AND for intersection, Day 6 v1 used OR+popcount for uniqueness, Day 6 v3 uses XOR+popcount for rolling uniqueness. The building blocks compose.

6. **Simplest problem drove deepest optimization**: Day 6 has the simplest input and shortest solution, yet produced the richest optimization discussion. Simple problems create room to focus on technique rather than fighting problem complexity.

---

## Zettelkasten Links

*Mathematics & Algorithms*:
- [[sliding-window-technique]] - Fixed-size window over sequential data, rolling state updates
- [[bitset-optimization]] - Bit manipulation for set operations (reused from Day 3)
- [[set-theory-fundamentals]] - Uniqueness = set cardinality equals input count
- [[xor-properties]] - Self-inverse property, toggle semantics, branchless design

*Rust Patterns*:
- [[iterator-patterns]] - `.windows()`, `.position()`, method chaining
- [[aoc-parsing-patterns]] - Minimal parsing (no parse step needed)

*Related Problems*:
- Day 3: Same bitset technique for set intersection
- Future sliding window problems will reuse rolling XOR pattern

---

**Navigation**: [← Day 5](day05_function_guide.md) | [Problem](day06.md) | [Summary](../summary_2022.md) | [Day 7 →](day07_function_guide.md)
