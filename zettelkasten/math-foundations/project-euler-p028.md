# Project Euler Problem 28: Number Spiral Diagonals

**Solved**: 2026-03-31
**Difficulty**: 5%
**Category**: Number Theory / Closed-Form Summation

## Problem Summary

Find the sum of diagonal elements in a 1001×1001 number spiral formed by
placing consecutive integers in a clockwise spiral pattern starting from 1.

## Mathematical Concepts

### Primary Concepts
- **Number spirals** — Ulam-style spiral arrangement of natural numbers
- **Closed-form summation** — converting iterative sums to algebraic formulas
- **Summation identities** — Σk = n(n+1)/2, Σk² = n(n+1)(2n+1)/6

### Supporting Concepts
- **Quadratic sequences** — corner values follow quadratic patterns in layer index
- **Arithmetic progressions** — gap between consecutive corners on the same layer

## Solution Approach

### Spiral Structure

The spiral is built layer by layer outward from center value 1:

```text
Layer 0: just the center, value 1
Layer 1: side length 3, corners at 3, 5, 7, 9
Layer 2: side length 5, corners at 13, 17, 21, 25
Layer 3: side length 7, corners at 31, 37, 43, 49
```

### Corner Value Formula

At layer k (k = 1, 2, ..., m where m = (n−1)/2 for an n×n spiral):
- Side length = 2k + 1
- The top-right corner is the last number placed: (2k+1)²
- Moving counter-clockwise, each corner is 2k less than the previous

```text
Corner       Value              Example (k=2, side=5)
─────────    ─────────────      ─────────────────────
Top-right    (2k+1)²            25
Top-left     (2k+1)² − 2k      21
Bottom-left  (2k+1)² − 4k      17
Bottom-right (2k+1)² − 6k      13
```

### Layer Sum

Sum of four corners at layer k:

```
S(k) = (2k+1)² + [(2k+1)² − 2k] + [(2k+1)² − 4k] + [(2k+1)² − 6k]
     = 4(2k+1)² − 12k
     = 4(4k² + 4k + 1) − 12k
     = 16k² + 4k + 4
```

### Total Diagonal Sum

```
Total = 1 + Σ_{k=1}^{m} (16k² + 4k + 4)
      = 1 + 16·Σk² + 4·Σk + 4m
```

Applying summation identities:

```
Σ_{k=1}^{m} k  = m(m+1)/2
Σ_{k=1}^{m} k² = m(m+1)(2m+1)/6
```

Therefore:

```
Total = 1 + 16·m(m+1)(2m+1)/6 + 4·m(m+1)/2 + 4m
      = 1 + 8m(m+1)(2m+1)/3 + 2m(m+1) + 4m
```

### Verification

For n = 5 (m = 2):
```
Total = 1 + 8·2·3·5/3 + 2·2·3 + 4·2
      = 1 + 80 + 12 + 8
      = 101 ✓
```

For n = 1001 (m = 500):
```
Total = 1 + 8·500·501·1001/3 + 2·500·501 + 4·500
      = 1 + 668,667,000 + 501,000 + 2,000
      = 669,171,001 ✓
```

### Why Closed-Form Matters

The iterative approach loops 500 times — already fast at ~400 ns. But the
closed-form computes the same answer in ~2.5 ns (a few multiplications and
additions), demonstrating how summation identities eliminate loops entirely.

This is a general pattern: whenever a loop accumulates a polynomial expression
in the loop variable, Faulhaber's formulas can replace the loop with O(1) algebra.

## Complexity

- **Time**: O(1) — closed-form (vs O(m) iterative)
- **Space**: O(1)

| Approach | Time | Notes |
|----------|------|-------|
| Closed-form | 2.5 ns | Pure arithmetic |
| Iterative | 402 ns | Loop over 500 layers |
| Speedup | ~160× | Summation identities |

## Rust Implementation Details

### Integer Division Safety

The formula includes division by 3: `8 * m * (m+1) * (2*m+1) / 3`.
This is always exact because among three consecutive integers m, m+1, 2m+1,
one of the first two is always divisible by... actually, more precisely:
m(m+1)(2m+1) is always divisible by 6 (this is what makes Σk² = n(n+1)(2n+1)/6
always an integer). So dividing by 3 after multiplying by 8 is safe in integer
arithmetic.

### No Overflow Risk

For m = 500: 8 × 500 × 501 × 1001 = 2,006,004,000, well within u64 range.

## Answer

**669,171,001**

## Related Problems

- **Problem 58** (Spiral Primes) — prime density along spiral diagonals
- **Problem 6** (Sum Square Difference) — another closed-form from summation identities
- **Problem 25** (1000-digit Fibonacci) — Binet's closed-form for Fibonacci

## Links

- [[project-euler-p027]] — previous problem (quadratic primes)
- [[project-euler-p029]] — next problem (distinct powers)
- [[project-euler-p006]] — Σk² summation identity also used there
- [Ulam Spiral (Wikipedia)](https://en.wikipedia.org/wiki/Ulam_spiral) — visual patterns in number spirals
- [Faulhaber's Formulas (Wikipedia)](https://en.wikipedia.org/wiki/Faulhaber%27s_formulas) — closed forms for power sums
