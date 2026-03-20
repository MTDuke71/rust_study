# Josephus Problem

**Concept**: Determining the survivor position in a circular elimination game.

**Created**: 2026-03-19
**Tags**: #mathematics #combinatorics #number-theory #closed-form #bit-manipulation

---

## Definition

**The Josephus Problem**: n people stand in a circle numbered 1 to n. Starting from person 1, every k-th person is eliminated. The problem asks: which position survives?

**Origin**: Flavius Josephus, a 1st-century Jewish historian. According to legend, he and 40 soldiers were trapped in a cave by Romans and chose circular elimination over capture. Josephus calculated where to stand to survive.

**Formal**: Given n participants and step size k, find J(n, k) — the position of the last survivor.

---

## Classic Case: k=2 (Eliminate Every Other Person)

### Recurrence Relation

```
J(1, 2) = 1
J(n, 2) = (J(n-1, 2) + 2 - 1) mod n + 1    (1-indexed)
```

Or equivalently (0-indexed):
```
J(1) = 0
J(n) = (J(n-1) + 2) mod n
```

### Closed-Form Solution

For k=2, there's a beautiful binary trick:

1. Find L = largest power of 2 <= n
2. **Winner = 2(n - L) + 1**

**Binary interpretation**: Write n in binary, rotate the leading 1 bit to the end.

```
Example: n = 41
  41 = 101001 (binary)
  Rotate leading bit: 010011 = 19
  Winner: position 19

Example: n = 5
  5 = 101 (binary)
  Rotate: 011 = 3
  Winner: position 3
```

### Why Binary Rotation Works

Consider what happens after the first pass around the circle:
- All even-numbered positions are eliminated
- We're left with n/2 (or (n-1)/2) people
- The subproblem is the same game with fewer people, shifted by the "leftover"

The power of 2 captures when the circle size halves cleanly, and the remainder
`n - L` determines the offset. Each halving corresponds to one bit position.

### Rust Implementation

```rust
fn josephus_k2(n: u64) -> u64 {
    let l = 1 << (63 - n.leading_zeros());  // highest power of 2 <= n
    2 * (n - l) + 1
}
```

`leading_zeros()` compiles to hardware `bsr`/`lzcnt` — single cycle, O(1).

---

## Across-Circle Variant (Eliminate Opposite Person)

This variant appears in AoC 2016 Day 19 Part 2: instead of eliminating every k-th person, eliminate the person **directly across** the circle (at distance `floor(remaining/2)`).

### Pattern Discovery

Write out winners for small n:

```
n:  1  2  3  4  5  6  7  8  9
w:  1  1  3  1  2  3  5  7  9

n: 10 11 12 13 14 15 16 17 18
w:  1  2  3  4  5  6  7  8  9

n: 19 20 21 22 23 24 25 26 27
w: 11 13 15 17 19 21 23 25 27
```

The pattern resets at **powers of 3** with two linear regions between:

### Closed-Form Solution

1. Find p = largest power of 3 <= n
2. Three cases:
   - **n = p**: winner is n (the last position)
   - **n <= 2p**: winner is **n - p** (climb by 1s)
   - **n > 2p**: winner is **2n - 3p** (climb by 2s)

### Why Powers of 3?

When eliminating the person across (distance `floor(remaining/2)`):
- At exact powers of 3, the last person always wins (circle reduces symmetrically)
- Between p and 2p: each additional person shifts the winner by 1
- Between 2p and 3p: the "across" pointer crosses the halfway mark, shifting the winner by 2 per additional person
- The slope change at 2p comes from the asymmetry of `floor(remaining/2)` — whether the eliminated person is before or after the current position in the circle

### Rust Implementation

```rust
fn josephus_across(n: u64) -> u64 {
    let mut p = 1;
    while p * 3 <= n {
        p *= 3;         // compiler optimizes to LEA (p + p*2)
    }
    if n == p {
        n
    } else if n <= 2 * p {
        n - p
    } else {
        2 * n - 3 * p
    }
}
```

Loop runs `floor(log3(n))` iterations (~13 for n=3M). Could use `(n as f64).log(3.0)`
but the loop is already effectively free and avoids floating-point precision concerns.

---

## General Case: Arbitrary k

For arbitrary k, no simple closed-form exists. The recurrence is:

```
J(1, k) = 0                          (0-indexed)
J(n, k) = (J(n-1, k) + k) mod n
```

This requires O(n) iterations — no shortcut for general k.

---

## Complexity Summary

| Variant | Time | Space | Method |
|---------|------|-------|--------|
| k=2 | O(1) | O(1) | Binary bit rotation |
| Across-circle | O(log3 n) | O(1) | Power-of-3 piecewise formula |
| General k | O(n) | O(1) | Recurrence iteration |
| Simulation | O(n log n) | O(n) | Balanced BST or BIT for removal |

---

## Connections

### AoC Applications
- **AoC 2016 Day 19**: Both variants in one puzzle — "An Elephant Named Joseph"
  - Part 1: k=2 elimination → binary bit rotation (73ns total with Part 2)
  - Part 2: across-circle → power-of-3 formula

### Related Math Concepts
- [[closed-form-formulas]] — Josephus k=2 is a classic example of replacing O(n) iteration with O(1) math
- [[modular-arithmetic]] — the recurrence uses modular arithmetic for circular indexing
- [[number-theory-basics]] — powers of 2 and 3 as structural boundaries
- [[base-conversion]] — binary representation directly encodes the k=2 solution

### Key Insight: Pattern Recognition over Simulation

The Josephus problem is a poster child for "don't simulate what you can calculate":
- Naive simulation: O(n^2) with linked list, O(n log n) with balanced BST
- k=2 closed form: O(1) — billions of times faster for large n
- The "across" variant: discovered by tabulating small cases and spotting the reset pattern at powers of 3

---

## References

- [Wikipedia — Josephus Problem](https://en.wikipedia.org/wiki/Josephus_problem)
- [Numberphile — "The Josephus Problem"](https://www.youtube.com/watch?v=uCsD3ZGzMgE) — covers history and k=2 binary derivation
- [0xdf — AoC 2016 Day 19](https://www.youtube.com/watch?v=M-V_gvJomzU) — derives the power-of-3 formula for Part 2
- Concrete Mathematics (Graham, Knuth, Patashnik) — Chapter 1 covers the Josephus problem in depth
