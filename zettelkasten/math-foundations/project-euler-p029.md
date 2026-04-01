# Project Euler Problem 29: Distinct Powers

**Solved**: 2026-03-31
**Difficulty**: 5%
**Category**: Number Theory / Combinatorics

## Problem Summary

Count distinct values of a^b for 2 ≤ a ≤ 100 and 2 ≤ b ≤ 100. Out of
9,801 total pairs, some produce duplicate values (e.g., 2⁴ = 4² = 16).

## Mathematical Concepts

### Primary Concepts
- **Canonical representation** — reducing expressions to a unique normal form
- **Perfect power detection** — identifying when an integer is a^k for some a, k

### Supporting Concepts
- **Fundamental theorem of arithmetic** — unique prime factorization guarantees canonical forms
- **Set cardinality** — counting distinct elements via deduplication
- **Logarithmic decomposition** — finding smallest base via integer roots

## Solution Approach

### The Duplication Problem

Without careful handling, we'd need to compare numbers as large as 100¹⁰⁰
(200 digits). Instead, we avoid computing the actual values entirely.

### Canonical Form Insight

Every integer a ≥ 2 can be written as r^k where r is the smallest possible
base (the "primitive base"):

```text
a     Decomposition    (r, k)
───   ─────────────    ──────
2     2¹               (2, 1)
4     2²               (2, 2)
8     2³               (2, 3)
16    2⁴               (2, 4)
32    2⁵               (2, 5)
64    2⁶               (2, 6)
9     3²               (3, 2)
27    3³               (3, 3)
81    3⁴               (3, 4)
25    5²               (5, 2)
```

Then a^b = (r^k)^b = r^(kb). Two terms a₁^b₁ and a₂^b₂ are equal iff
they share the same canonical pair (r, kb).

### Example: Why 4² = 2⁴

```text
4² → base 4 = 2², so canonical = (2, 2×2) = (2, 4)
2⁴ → base 2 = 2¹, so canonical = (2, 1×4) = (2, 4)
Same canonical form → same value = 16
```

### Duplicate Families

All duplicates come from bases that are perfect powers of smaller bases.
Within the range 2–100, the families are:

```text
Base 2 family: {2, 4, 8, 16, 32, 64}    (k = 1,2,3,4,5,6)
Base 3 family: {3, 9, 27, 81}            (k = 1,2,3,4)
Base 5 family: {5, 25}                   (k = 1,2)
Base 6 family: {6, 36}                   (k = 1,2)
Base 7 family: {7, 49}                   (k = 1,2)
Base 10 family: {10, 100}                (k = 1,2)
```

All other bases (primes > 10, composites that aren't perfect powers) contribute
99 unique terms each with zero duplicates.

### Counting

- Total pairs: 99 × 99 = 9,801
- Distinct values: 9,183
- Duplicates eliminated: 618

The base-2 family produces the most duplicates because it has the most members (6),
creating many overlapping exponent ranges.

## Complexity

- **Time**: O(n² log n) — n² pairs, each with O(log n) base decomposition
- **Space**: O(n²) — HashSet storing up to 9,801 canonical pairs

| Metric | Value |
|--------|-------|
| Time | 276 µs |
| Pairs | 9,801 |
| Distinct | 9,183 |

## Rust Implementation Details

### Avoiding Big Integers

The canonical form approach means we never compute values larger than u32.
The HashSet stores (u32, u32) pairs — tiny compared to 200-digit bigints.

### Floating-Point Root Detection

```rust
let root = (a as f64).powf(1.0 / exp as f64).round() as u32;
if root >= 2 && root.pow(exp) == a { ... }
```

The `.round()` handles floating-point imprecision, then the integer
`root.pow(exp) == a` check verifies exactness. This is safe because
we only test small values (a ≤ 100).

## Answer

**9,183**

## Related Problems

- **Problem 56** (Powerful Digit Sum) — digit sums of a^b (requires bigint)
- **Problem 97** (Large Non-Mersenne Prime) — large exponentiation modular arithmetic
- **Problem 99** (Largest Exponential) — comparing a^b without computing them

## Links

- [[project-euler-p028]] — previous problem (number spiral diagonals)
- [[project-euler-p030]] — next problem (digit fifth powers)
- [Perfect Power (Wikipedia)](https://en.wikipedia.org/wiki/Perfect_power) — integers expressible as a^k
- [Fundamental Theorem of Arithmetic (Wikipedia)](https://en.wikipedia.org/wiki/Fundamental_theorem_of_arithmetic)
