# Day 9: Explosives in Cyberspace --- Function Guide

**Problem**: Compute the decompressed length of a string with `(LENxREP)` markers — without building the string.
**Answers**: Part 1 = **120,765**, Part 2 = **11,658,395,076**
**Code**: [day09.rs](../../src/solver/day09.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Algorithm Overview](#algorithm-overview)
3. [parse_input](#parse_input)
4. [solve_part1_with_data](#solve_part1_with_data)
5. [decompressed_len (Part 2)](#decompressed_len-part-2)
6. [Part 1 vs Part 2 — The Key Difference](#part-1-vs-part-2--the-key-difference)
7. [Worked Examples](#worked-examples)
8. [Benchmarks](#benchmarks)
9. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: A single line of 16,654 characters — compressed text with `(LENxREP)` markers.

**Markers**: `(10x2)` means "take the next 10 characters and repeat them 2 times."

**Part 1**: Decompress with **single-level** expansion — markers inside repeated sections are treated as literal text (not expanded). Return the decompressed length.

**Part 2**: Decompress with **recursive** expansion — markers inside repeated sections are themselves expanded. Return the fully decompressed length.

**Key constraint**: The Part 2 answer is ~11.6 billion characters. You cannot build the string — you must count bytes with pointer arithmetic only.

---

## Algorithm Overview

```
Input (single line, 16,654 bytes)
    |
    v
parse_input() -> &[u8]     (trim + as_bytes, zero allocation)
    |
    +--> solve_part1_with_data()    (linear scan, pointer walk)
    |        |
    |        v
    |    120,765
    |
    +--> decompressed_len()         (recursive scan, slice recursion)
             |
             v
         11,658,395,076
```

**Complexity**:
- Part 1: O(n) — single pass through the input, each byte visited exactly once
- Part 2: O(n × d) where d = maximum nesting depth. Each marker spawns a recursive call on a sub-slice, but the total bytes across all recursion levels at any depth is bounded by n.

---

## `parse_input`

```rust
fn parse_input(input: &str) -> &[u8]
```

Trims whitespace and converts to a byte slice. Zero allocation — the `&[u8]` borrows directly from the input string.

**Why bytes?** We only need to check for `(`, `)`, and `x` — all single ASCII bytes. Working at the byte level avoids UTF-8 overhead and enables direct indexing with `data[i]`.

---

## `solve_part1_with_data`

```rust
fn solve_part1_with_data(data: &[u8]) -> usize
```

Linear pointer walk through the byte slice:

```
Position:  A ( 1 x 5 ) B C
Index:     0 1 2 3 4 5 6 7

i=0: data[0]='A' → not '(' → len += 1, i=1
i=1: data[1]='(' → find ')' at index 5
     parse marker: chars=1, repeat=5
     len += 1 * 5 = 5
     i = 5 + 1 + 1 = 7   (skip past ')' + 1 consumed char)
i=7: data[7]='C' → not '(' → len += 1, i=8
i=8: done. len = 7 ✓
```

**Critical detail**: After processing a marker, `i` jumps past the closing `)` AND past the `chars` consumed characters. Those consumed characters are counted (via `chars * repeat`) but never individually visited — this is what makes it O(n).

**Markers inside consumed characters are ignored** — they're just bytes being "repeated" as-is. This is the Part 1 rule.

---

## `decompressed_len` (Part 2)

```rust
fn decompressed_len(data: &[u8]) -> usize
```

Same pointer walk structure as Part 1, with one change on the key line:

| Part 1 | Part 2 |
|--------|--------|
| `len += chars * repeat` | `len += decompressed_len(&data[start..start + chars]) * repeat` |

Instead of counting the consumed characters as literal bytes, we **recursively compute their decompressed length** before multiplying by the repeat count.

**The recursion bottoms out** when a slice has no markers — every byte is literal, so `decompressed_len` just counts them one by one (the `else` branch). This makes plain text the base case.

**No memoization needed**: Each sub-slice is unique (different position in the input), so there's no overlapping subproblem structure. The recursion tree has O(n) total work across all levels.

---

## Part 1 vs Part 2 — The Key Difference

Consider `X(8x2)(3x3)ABCY`:

**Part 1** (single-level):
```
X  →  1 (literal)
(8x2)  →  take next 8 bytes: "(3x3)ABC", repeat 2x = 16
Y  →  1 (literal)
Total: 1 + 16 + 1 = 18

Decompressed: X(3x3)ABC(3x3)ABCY  (markers are literal text)
```

**Part 2** (recursive):
```
X  →  1 (literal)
(8x2)  →  take next 8 bytes: "(3x3)ABC"
           recursively decompress "(3x3)ABC":
             (3x3)  →  take "ABC", repeat 3x = 9
           inner result = 9
           9 × 2 = 18
Y  →  1 (literal)
Total: 1 + 18 + 1 = 20

Decompressed: XABCABCABCABCABCABCY  (markers inside markers ARE expanded)
```

**The insight**: Part 1 treats inner markers as opaque bytes. Part 2 recursively interprets them. Same algorithm, different depth.

---

## Worked Examples

### Example 1: Deeply nested — `(27x12)(20x12)(13x14)(7x10)(1x12)A`

Evaluated inside-out (recursion unwinds from innermost):

| Level | Expression | Length |
|-------|-----------|--------|
| 5 (innermost) | `A` | 1 |
| 4 | `(1x12)A` → 1 × 12 | 12 |
| 3 | `(7x10)(1x12)A` → 12 × 10 | 120 |
| 2 | `(13x14)(7x10)...` → 120 × 14 | 1,680 |
| 1 | `(20x12)(13x14)...` → 1,680 × 12 | 20,160 |
| 0 (outermost) | `(27x12)(20x12)...` → 20,160 × 12 | **241,920** |

The multiplicative chain: 1 × 12 × 10 × 14 × 12 × 12 = 241,920

This is why Part 2 produces ~11.6 billion from a 16KB input — nested multipliers compound exponentially.

### Example 2: Mixed — `(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN`

```
(25x3) captures: "(3x3)ABC(2x3)XY(5x2)PQRST"
  Recursively:
    (3x3)ABC  → 9
    (2x3)XY   → 6
    (5x2)PQRST → 10
  Inner = 25, × 3 = 75

X → 1 (literal)

(18x9) captures: "(3x2)TWO(5x7)SEVEN"
  Recursively:
    (3x2)TWO  → 6
    (5x7)SEVEN → 35
  Inner = 41, × 9 = 369

Total: 75 + 1 + 369 = 445
```

---

## Benchmarks

| Benchmark | Time |
|-----------|------|
| Part 1    | 157ns |
| Part 2    | 17.3µs |
| Combined  | 17.5µs |

Part 1 is extremely fast (~157 nanoseconds) because it's a single linear scan — no recursion, no allocation, just pointer arithmetic on a byte slice.

Part 2 at 17.3µs is dominated by the recursive marker parsing. The recursion depth mirrors the nesting depth of markers in the input.

Combined ≈ Part 2 because Part 1 is negligible (~0.9% of total time).

At 17.5µs total, this is among the fastest days — well under the 100ms threshold.

---

## Key Patterns

| Pattern | Description |
|---------|-------------|
| Byte-level scanning | Work on `&[u8]` instead of `&str` for direct indexing and ASCII comparisons |
| Pointer walk | Manual index `i` with variable-length jumps — more natural than iterators for skip-ahead parsing |
| Length-only computation | Count bytes without building the output string — essential when output is billions of bytes |
| Slice recursion | `decompressed_len(&data[start..start+chars])` — zero-copy sub-slice as recursive input |
| Multiplicative nesting | Nested `(LENxREP)` markers create exponential growth: small input → huge output |
| Same algorithm, different depth | Part 1 and Part 2 share identical structure; only the "count consumed chars" line differs |


**See also**: [Assembly Analysis](day09_asm_analysis.md) — annotated LLVM output showing how Rust compiles to 157ns | [AoC 2016 Summary](../summary_2016.md)
