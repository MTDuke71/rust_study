# Project Euler Problem 23: Non-Abundant Sums

**Solved**: 2026-03-28
**Difficulty**: 5%
**Category**: Number Theory / Divisor Functions

## Problem Summary

Find the sum of all positive integers that cannot be expressed as the sum of two abundant numbers. Upper bound: 28123.

## Mathematical Concepts

### Primary Concepts
- **Number classification by divisor sum**:
  - **Deficient**: d(n) < n (e.g., 8: d(8) = 1+2+4 = 7 < 8)
  - **Perfect**: d(n) = n (e.g., 6: d(6) = 1+2+3 = 6)
  - **Abundant**: d(n) > n (e.g., 12: d(12) = 1+2+3+4+6 = 16 > 12)
- **Divisor sum sieve** — compute d(n) for all n in O(n log n)

### Supporting Concepts
- **Goldbach-like property**: Every sufficiently large integer is the sum of two abundant numbers
- **Upper bound proof**: 28123 is the proven limit; actual largest non-expressible is 20161

## Solution Approach

### Step 1: Divisor Sum Sieve (Reused from Problem 21)
Compute `d[n]` for all n < 28124 using `proper_divisor_sum_sieve`. This is the **integrator pattern** — reusing a validated utility rather than reimplementing.

### Step 2: Collect Abundant Numbers
Filter: `n where d[n] > n`. There are ~6,965 abundant numbers below 28124.

### Step 3: Boolean Sieve for Pairwise Sums
For each pair (a, b) of abundant numbers where a ≤ b:
```text
if a + b < 28124: mark is_abundant_sum[a + b] = true
```
Optimization: Start inner loop at `i` (not 0) since addition is commutative — each pair checked once.

### Step 4: Sum Unmarked Numbers
All numbers where `is_abundant_sum[n] == false` contribute to the answer.

## Complexity

- **Time**: O(n log n) for sieve + O(a²) for pairwise marking, where a ≈ 6,965 abundant numbers
- **Space**: O(n) for sieve and boolean array

## Performance

| Metric | Value |
|--------|-------|
| Time | 3.84 ms |
| Target | < 100 ms |
| Bottleneck | Pairwise abundant sum marking (~6,965² / 2 ≈ 24M pairs) |

## Rust Implementation Details

### Utility Reuse (Integrator Pattern)
```rust
use crate::utils::number_theory::proper_divisor_sum_sieve;
```
Same sieve from Problem 21 — no reimplementation needed.

### Commutative Optimization
```rust
for (i, &a) in abundants.iter().enumerate() {
    for &b in &abundants[i..] {  // start at i, not 0
        let sum = a + b;
        if sum >= limit { break; }  // sorted order → early exit
        is_abundant_sum[sum] = true;
    }
}
```
Since a + b = b + a, only check each pair once. The `break` exploits sorted order — if a + b exceeds the limit, all subsequent b values will too.

## Answer

**4,179,871**

## Related Problems

- **Problem 21** (Amicable Numbers) — same divisor sum sieve, different classification
- **Problem 12** (Highly Divisible Triangle Number) — divisor counting
- **Problem 44** (Pentagon Numbers) — similar "sum of two from a set" structure

## Links

- [[project-euler-p021]] — shared utility: `proper_divisor_sum_sieve`
- [[set-theory-fundamentals]] — abundant numbers as a set, pairwise sums as Minkowski sum
- [Abundant number (Wikipedia)](https://en.wikipedia.org/wiki/Abundant_number)
