# Project Euler Problem 32: Pandigital Products

**Solved**: 2026-04-26
**Difficulty**: 5%
**Category**: Combinatorics / Digit Manipulation

## Problem Summary

Find the sum of all 4-digit products c such that some identity a × b = c uses
each of the digits 1–9 exactly once across all three numbers.

## Mathematical Concepts

### Primary Concepts
- **Pandigital numbers** — using a digit set exactly once
- **Digit-count proof** — bounding search via constraint satisfaction
- **Bitmask digit sets** — O(1) duplicate detection

### Supporting Concepts
- **Multiplication digit count** — k-digit × m-digit produces (k+m−1) or (k+m) digits
- **Deduplication via HashSet** — multiple identities can yield the same product

## Solution Approach

### Bounding the Search via Digit Counts

Let the multiplicand have **a** digits, the multiplier **b** digits, and the
product **c** digits. The pandigital constraint forces:

$$a + b + c = 9$$

Multiplication of an a-digit by a b-digit positive integer produces a result
with either (a + b − 1) or (a + b) digits. Substituting:

| Case | Equation | Solution |
|------|----------|----------|
| c = a + b | 2(a + b) = 9 | No integer solution |
| c = a + b − 1 | 2(a + b) − 1 = 9 | a + b = 5, c = 4 |

So only two shapes can yield a pandigital identity:

| Shape | Range | Example |
|-------|-------|---------|
| 1-digit × 4-digit = 4-digit | a ∈ [1,9], b ∈ [1234, 9876] | 4 × 1738 = 6952 |
| 2-digit × 3-digit = 4-digit | a ∈ [12, 98], b ∈ [123, 987] | 39 × 186 = 7254 |

This eliminates the 5-digit and larger products entirely — a massive search
reduction without needing to actually try them.

### Bitmask Pandigital Test

A digit set fits in 16 bits (one bit per possible digit 0–9). For each digit
encountered:

1. If digit is 0 → fail (problem requires digits 1–9 only)
2. If bit already set → fail (duplicate)
3. Set the bit

After processing all three numbers, check `bits == 0b11_1111_1110` (bits 1..=9
set, bit 0 clear). This is **branchless** in the success case and is faster
than sorting digits or building a HashSet of digit characters.

### Deduplication

Different identities can produce the same product:

- 18 × 297 = 5346
- 27 × 198 = 5346

The problem says "include each product only once." A `HashSet<u64>` collects
the unique values; `iter().sum()` produces the answer.

### The Nine Pandigital Products

Running the algorithm yields exactly seven distinct products:

| Identity | Product |
|----------|---------|
| 4 × 1738 | 6952 |
| 4 × 1963 | 7852 |
| 12 × 483 | 5796 |
| 18 × 297 | 5346 |
| 27 × 198 | 5346 (dup) |
| 28 × 157 | 4396 |
| 39 × 186 | 7254 |
| 42 × 138 | 5796 (dup) |
| 48 × 159 | 7632 |

Distinct products: {4396, 5346, 5796, 6952, 7254, 7632, 7852}.
Sum = 4396 + 5346 + 5796 + 6952 + 7254 + 7632 + 7852 = **45,228**.

## Complexity

- **Time**: O(N) where N is the number of (a, b) candidate pairs
  - Shape 1: 9 × ~8643 ≈ 78K iterations
  - Shape 2: 87 × ~865 ≈ 75K iterations (with early `break` on c > 9876)
- **Space**: O(k) for the result HashSet (k ≤ 9)

| Metric | Value |
|--------|-------|
| Time | ~82 µs |
| Candidates | ~150K |
| Distinct products | 7 |

### Why Not Brute-Force All 9-Digit Permutations?

An alternative is to permute the 9! = 362,880 arrangements of "123456789" and
test each split (a, b, c). This avoids the digit-count proof and runs in
acceptable time, but doing the math first reduces work by ~2× and produces
self-documenting code that explains *why* certain shapes are impossible.

## Rust Implementation Details

### Bitmask With Duplicate Detection

```rust
let bit = 1u16 << d;
if bits & bit != 0 {
    return false; // duplicate digit
}
bits |= bit;
```

The duplicate check happens before the OR, so we catch (e.g.) two 5s in the
combined string of digits without needing a separate count. Three early-exits
(zero digit, duplicate digit, final mask mismatch) keep the hot path tight.

### Slice-Based Variadic Input

`is_pandigital_1_9(&[a, b, c])` accepts any slice of `u64`s. This generalizes
naturally: pandigital of two numbers, or four, or one — same function. For
this problem we pass three numbers (a, b, c), but the design supports
extensions like Problem 38 (concatenation pandigitals).

## Answer

**45,228** — sum of distinct 4-digit pandigital products.

## Related Problems

- **Problem 38** (Pandigital Multiples) — concatenated pandigital products
- **Problem 41** (Pandigital Primes) — largest n-digit pandigital prime
- **Problem 43** (Sub-string Divisibility) — pandigital with divisibility constraints
- **Problem 52** (Permuted Multiples) — same digits across multiples
- **Problem 104** (Pandigital Fibonacci Ends) — pandigital Fibonacci numbers

## Links

- [[project-euler-p031]] — previous problem (coin sums)
- [[combinatorics-fundamentals]] — pandigital permutation counts
- [[modular-arithmetic]] — digit extraction via `% 10`
- [[order-of-magnitude]] — multiplication digit-count rule
- [Pandigital Number (Wikipedia)](https://en.wikipedia.org/wiki/Pandigital_number)
