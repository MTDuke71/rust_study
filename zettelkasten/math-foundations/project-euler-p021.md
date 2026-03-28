# Project Euler Problem 21: Amicable Numbers

**Solved**: 2026-03-27
**Difficulty**: 5%
**Category**: Number Theory / Divisor Functions

## Problem Summary

Find the sum of all amicable numbers under 10,000. Numbers a and b are amicable if d(a) = b and d(b) = a, where d(n) = sum of proper divisors and a ≠ b.

## Mathematical Concepts

### Primary Concepts
- **Divisor sum function** σ₁(n) — sum of all divisors; d(n) = σ₁(n) - n for proper divisors
- **Amicable numbers** — pairs where d(a) = b and d(b) = a
- **Divisor sum sieve** — compute d(n) for all n in O(n log n)

### Supporting Concepts
- **Perfect numbers** — d(n) = n (self-amicable, excluded by a ≠ b)
- **Harmonic series** — explains the O(n log n) sieve complexity

## Solution Approach

### The Divisor Sum Sieve (Key Algorithm)

Instead of computing d(n) one number at a time (O(√n) each), compute **all values at once** using a sieve:

```text
d[0..N] = all zeros

for divisor in 1..N:
    for multiple in (2*divisor, 3*divisor, ...):
        d[multiple] += divisor
```

**Why start at 2×divisor?** Because we want *proper* divisors (excluding n itself). If divisor = 5, we add 5 to d[10], d[15], d[20], ... but NOT d[5].

**Analogy**: Like the Sieve of Eratosthenes visits multiples to mark composites, this visits multiples to accumulate divisor sums.

### Complexity: Why O(n log n)?

The total work is the number of (divisor, multiple) pairs:

```text
n/1 + n/2 + n/3 + ... + n/n = n × (1 + 1/2 + 1/3 + ... + 1/n) = n × Hₙ ≈ n × ln(n)
```

where Hₙ is the nth harmonic number. For n = 10,000, that's about 92,000 operations — essentially instant.

### Amicable Check

After sieving, checking amicability is O(1) per number:

```rust
let partner = d[n];
partner != n && partner < limit && d[partner] == n
```

### Three Approaches Compared

| Approach | d(n) cost | All d(n) for n<N | Implementation |
|----------|-----------|------------------|----------------|
| Trial division (√n) | O(√n) | O(N√N) | Simple loop |
| Prime factorization | O(√n) | O(N√N) | σ formula from factors |
| **Divisor sum sieve** | O(1) lookup | **O(N log N)** | **Best for bulk** |

The sieve wins when you need d(n) for *many* values — exactly our case.

**Benchmark**: **36.66 µs** (Criterion, release build) — sieve + scan for 10,000 numbers.

## Rust Implementation

See [[project_euler/src/problems/p021.rs]] for complete code.
The sieve itself lives in [[project_euler/src/utils/number_theory.rs]] as `proper_divisor_sum_sieve`.

### Key Code Pattern

```rust
// Sieve: O(n log n) to compute all divisor sums
let d = proper_divisor_sum_sieve(limit);

// Check amicability: O(1) per number
(2..limit)
    .filter(|&n| {
        let partner = d[n] as usize;
        partner != n && partner < limit && d[partner] == n as u64
    })
    .sum()
```

### Utility Growth

Added `proper_divisor_sum_sieve` to `utils/number_theory.rs` — reusable for any problem needing bulk divisor sums (abundant numbers, deficient numbers, sociable chains, etc.).

## Amicable Pairs Below 10,000

| a | b | d(a) = b | d(b) = a |
|---|---|----------|----------|
| 220 | 284 | 284 | 220 |
| 1184 | 1210 | 1210 | 1184 |
| 2620 | 2924 | 2924 | 2620 |
| 5020 | 5564 | 5564 | 5020 |
| 6232 | 6368 | 6368 | 6232 |

Sum: 220+284+1184+1210+2620+2924+5020+5564+6232+6368 = **31,626**

## Related Problems

- **[[project-euler-p012]]** — Highly divisible triangle numbers (uses divisor counting)
- **Future**: P23 (abundant numbers — d(n) > n, perfect sieve candidate)

## Learning Insights

- The divisor sum sieve is the "bulk" version of trial division — same relationship as Sieve of Eratosthenes has to individual primality testing
- O(n log n) from harmonic series is a recurring complexity in number-theoretic sieves
- Amicable numbers have been studied since Pythagoras (~500 BC) — the 220/284 pair was known to the ancient Greeks
- The sieve utility will pay dividends in future problems (P23 abundant numbers is a near-certain reuse)

## References

- [Amicable Numbers (Wikipedia)](https://en.wikipedia.org/wiki/Amicable_numbers)
- [Harmonic Series (Wikipedia)](https://en.wikipedia.org/wiki/Harmonic_series_(mathematics))

---

*Links:*
- **Backlinks**: [[project-euler-p021|Problem Statement]], [[project-euler-p012]]
- **Concept Tags**: #number-theory #divisors #sieve #amicable-numbers #project-euler
- **Difficulty**: #euler-easy
