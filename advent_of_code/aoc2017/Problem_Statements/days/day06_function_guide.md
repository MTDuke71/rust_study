# Day 6: Memory Reallocation — Function Guide

**Problem**: Detect cycles in memory bank redistribution by tracking seen states.
**Answers**: Part 1 = **6,681**, Part 2 = **2,392**
**Code**: [day06.rs](../../src/solver/day06.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Data Structures](#data-structures)
3. [parse_input](#parse_input)
4. [redistribute](#redistribute)
5. [solve_both](#solve_both)
6. [Benchmarks](#benchmarks)
7. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: 16 tab-separated integers representing memory banks with block counts.

```
4	1	15	12	0	9	9	5	5	8	7	3	14	5	12	3
```

**Part 1**: Repeatedly redistribute blocks from the fullest bank (lowest index breaks ties) one-at-a-time to subsequent banks (wrapping around). Count how many redistribution cycles complete before producing a configuration **seen before**.

**Part 2**: How many cycles are in the **infinite loop**? (Distance between first and second occurrence of the repeated state.)

**Example** (`0 2 7 0`, 4 banks):
```
Step 0: [0, 2, 7, 0]  ← initial
Step 1: [2, 4, 1, 2]  ← bank 2 (max=7) redistributed
Step 2: [3, 1, 2, 3]
Step 3: [0, 2, 3, 4]
Step 4: [1, 3, 4, 1]
Step 5: [2, 4, 1, 2]  ← matches Step 1!
```
- Part 1: **5** (cycles before repeat)
- Part 2: **4** (loop length: step 5 − step 1)

---

## Data Structures

### Parsed input
```rust
Vec<u32>
```

16 unsigned integers. Small enough that `Vec<u32>` is used both as the working state and as HashMap keys (cloned each cycle).

### Cycle detection
```rust
HashMap<Vec<u32>, usize>
```

Maps each seen bank configuration to the step number when it was first observed. When a configuration is found in the map, the current step gives Part 1 and the difference gives Part 2.

---

## `parse_input`

```rust
fn parse_input(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}
```

Uses `split_whitespace()` which handles tabs, spaces, and newlines uniformly. No need for explicit `\t` splitting.

---

## `redistribute`

```rust
fn redistribute(banks: &mut [u32])
```

The core operation, performed once per cycle:

1. **Find max bank**: `max_by` with tie-breaking favoring lowest index (reverse-compare indices: `j.cmp(i)`)
2. **Zero out**: Set the selected bank to 0
3. **Distribute evenly**: Each bank gets `max_val / len` blocks (integer division)
4. **Distribute remainder**: The next `max_val % len` banks (wrapping from max_idx + 1) each get one extra block

**Key optimization**: Instead of distributing one block at a time in a loop of `max_val` iterations, this uses division to compute the even share in O(1), then only loops for the remainder (0..15 iterations max). For the actual input where max values reach ~15, this saves ~10 iterations per cycle — modest but clean.

**Tie-breaking**: The `max_by` comparator `a.cmp(b).then(j.cmp(i))` first compares values (ascending), then reverse-compares indices so that equal values select the lower index.

---

## `solve_both`

```rust
fn solve_both(input: &str) -> (usize, usize)
```

Single loop handles both parts simultaneously:

```
Parse input → banks
seen = {}  (HashMap: state → step number)
step = 0

loop:
    if banks ∈ seen:
        return (step, step - seen[banks])   ← Part 1, Part 2
    seen[banks.clone()] = step
    redistribute(banks)
    step += 1
```

**Why one loop, not two**: Both parts require the same cycle detection. Part 1 is the step count when the repeat is found. Part 2 is the difference between current step and the first occurrence. The HashMap stores step numbers (not just presence), making Part 2 a simple subtraction.

**Memory**: Each HashMap entry stores a `Vec<u32>` (16 elements = 64 bytes + Vec overhead). With 6,681 unique states, total map size is ~600KB — trivial.

---

## Benchmarks

| Function | Time |
|----------|------|
| `solve` (combined) | 861.60µs |

**Performance breakdown**: 6,681 redistribution cycles, each doing:
- One max scan (16 elements)
- One redistribute (16 + remainder iterations)
- One HashMap insert (hashing 16 × u32 = 64 bytes)
- One HashMap lookup

**Why no optimization needed**: 862µs is well under the 100ms budget. The dominant cost is HashMap operations (hashing + comparing 64-byte keys). Potential optimizations (not needed):
- **FxHashMap**: Faster hashing for small keys (~30% speedup)
- **Bitpacked state**: Pack 16 banks into a u128 (each bank < 256, so 8 bits each) for cheaper hashing/comparison
- **Floyd's cycle detection**: O(1) memory instead of O(n), but requires 2× the redistribution calls

---

## Key Patterns

### Cycle detection with HashMap

The classic pattern for detecting cycles in state machines:
1. Store each state with its step number
2. When a state repeats, you get both the total steps (Part 1) and the cycle length (Part 2)

This is more powerful than a simple HashSet — storing step numbers gives cycle length for free. Alternative approaches:
- **HashSet**: Only detects that a cycle exists, not its length (need a second pass)
- **Floyd's tortoise/hare**: O(1) memory but needs a way to advance the state without storing intermediates
- **Brent's algorithm**: Improved Floyd's, faster in practice

### Division-based redistribution

Instead of the naive one-at-a-time loop:
```rust
// ❌ Naive: O(max_val) per redistribution
for i in 0..max_val {
    banks[(max_idx + 1 + i) % len] += 1;
}

// ✅ Optimized: O(len) per redistribution
let full = max_val / len;
let remainder = max_val % len;
for bank in banks.iter_mut() { *bank += full; }
for offset in 1..=remainder { banks[(max_idx + offset) % len] += 1; }
```

Both are fast for 16 banks, but the division approach scales better and expresses the intent more clearly.

### State cloning for immutable keys

Each cycle clones the bank state for HashMap insertion. The alternative — converting to a hashable tuple or integer — would avoid allocation but adds complexity. For 16 × u32, the clone cost is negligible (~64 bytes copied per cycle).
