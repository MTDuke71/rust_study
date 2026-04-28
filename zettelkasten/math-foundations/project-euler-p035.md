# Project Euler Problem 35: Circular Primes

**Solved**: 2026-04-27
**Difficulty**: 5%
**Category**: Number Theory / Primes / Cyclic Digit Operations

## Problem Summary

A **circular prime** is a prime p such that every cyclic rotation of its
decimal digits is also prime. Count circular primes below 1,000,000.

## Mathematical Concepts

### Primary Concepts
- **Circular primes** — invariance of primality under cyclic digit rotation
- **Rotation as digit cycle** — `[d₀, d₁, …, d_{k-1}] → [d₁, …, d_{k-1}, d₀]`

### Supporting Concepts
- **Sieve of Eratosthenes** ([[sieve-of-eratosthenes]]) — O(N log log N) prime marking
- **Primality fundamentals** ([[primality-fundamentals]]) — definitions and shortcut tests
- **Divisibility by 2 and 5** — digit-position based detection

## The Digit-Set Constraint

For any circular prime p > 5, **every digit of p must lie in {1, 3, 7, 9}.**

**Proof sketch.**
- If p has any digit in {0, 2, 4, 6, 8}, some rotation places that digit in
  the units position. That rotation is even, hence ≥ 2 and divisible by 2.
  The only even prime is 2 itself.
- If p has digit 5, some rotation ends in 5 and is divisible by 5. The only
  prime divisible by 5 is 5 itself.

**Implications.** The 4-out-of-10 digit restriction is severe. For 6-digit
candidates, only 4⁶ = 4,096 numbers have all-allowed digits versus 9 × 10⁵
= 900,000 6-digit numbers — a 220× reduction.

We did **not** apply this filter in the implementation since the sieve
already makes per-prime verification O(d) lookups (≤ 6 array reads). The
constraint is documented for understanding, not optimization.

## Solution Approach

### High-level algorithm

1. Build a Vec<bool> sieve `is_prime[0..N]` where N = 1,000,000.
2. For each n in 2..N: if `is_prime[n]`, generate all d rotations of n and
   confirm each is prime via O(1) sieve lookup.
3. Count successes.

### Rotation arithmetic (no string conversion)

For a d-digit number n with leading digit ℓ = n / 10^(d-1):

```text
rotate_left(n, d) = (n − ℓ · 10^(d-1)) · 10 + ℓ
```

This shifts the d-1 trailing digits left and appends ℓ. Pure integer math,
no allocation. Iterating d times brings us back to the original n.

### Why the sieve dominates per-call primality testing

For each prime p, we generate d rotations. Doing trial-division primality
on each rotation costs O(√p / log p) average. With 78,498 primes and up to
6 rotations each, that's hundreds of millions of operations.

The sieve precomputes everything in O(N log log N) ≈ 5 ms, then each rotation
check is a single bounds-checked array read. Total verification time: ~1 ms.

## Complexity

- **Time**: O(N log log N) sieve build + O(P · d) verification ≈ ~5 ms total
- **Space**: O(N) bytes for the sieve bitmap (1 MB at N = 10⁶)

| Metric | Value |
|--------|-------|
| Time | ~5 ms (release) |
| Sieve size | 1,000,000 |
| Primes below N | 78,498 |
| Circular primes found | 55 |

## All 55 Circular Primes Below 10⁶

```text
< 10:    2, 3, 5, 7
< 100:   11, 13, 17, 31, 37, 71, 73, 79, 97
< 10³:   113, 131, 197, 199, 311, 337, 373, 719, 733, 919, 971, 991
< 10⁴:   1193, 1931, 3119, 3779, 7793, 7937, 9311, 9377
< 10⁵:   11939, 19391, 19937, 37199, 39119, 71993, 91193, 93719, 93911, 99371
< 10⁶:   193939, 199933, 319993, 331999, 391939, 393919, 919393, 933199,
         939193, 939391, 993319, 999331
```

Note that several of these come in cyclic families: e.g., 1193, 1931, 3119,
9311 are all rotations of each other and all qualify.

## Rust Implementation Details

### Sieve as Vec<bool> rather than bitset

`Vec<bool>` uses one byte per entry (1 MB for N = 10⁶). A `BitSet` would
shrink this to 125 KB, but at N = 10⁶ the cache pressure is fine and `Vec<bool>`
gives clearer code. For larger N we'd switch.

### `sieve_bitmap` vs `sieve` in `utils::primes`

Two complementary forms now coexist in `utils/primes.rs`:

- `sieve(n) -> Vec<usize>` — the prime *list*. Use when you need to enumerate
  primes (e.g., summing primes below N, or iterating "for each prime p").
- `sieve_bitmap(limit) -> Vec<bool>` — primality *lookup*. Use when you need
  to test arbitrary values for primality (e.g., rotation checks here, or
  pandigital prime hunting in Problem 41).

Internally `sieve` now delegates to `sieve_bitmap` and filters, so the two
share their core implementation.

## Answer

**55** circular primes below 1,000,000.

## Related Problems

- **Problem 7** — finding the nth prime via sieve
- **Problem 10** — summing primes below N (same sieve scaffold)
- **Problem 41** — pandigital primes (digit-restricted prime hunt)
- **Problem 50** — consecutive prime sums (sieve + sliding window)

## Links

- [[project-euler-p034]] — previous problem (digit factorials)
- [[sieve-of-eratosthenes]] — algorithm and complexity
- [[primality-fundamentals]] — divisibility shortcuts and definitions
- [[number-theory-basics]] — base-10 digit operations
- [Circular Prime (Wolfram MathWorld)](https://mathworld.wolfram.com/CircularPrime.html)
