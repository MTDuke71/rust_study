# Project Euler Problem 34: Digit Factorials

**Solved**: 2026-04-27
**Difficulty**: 5%
**Category**: Number Theory / Factorions

## Problem Summary

Find all numbers n with n = Σ d! over the digits d of n, then sum them. The
single-digit cases 1 (1!) and 2 (2!) are excluded as "not sums". Numbers with
this property are called **factorions**.

## Mathematical Concepts

### Primary Concepts
- **Factorions** — numbers equal to the sum of factorials of their digits
- **Upper bound derivation** — the same digit-count argument used in [[project-euler-p030]]

### Supporting Concepts
- **Factorials** — see [[combinatorics-fundamentals]] for n! = n · (n-1)!
- **Digit extraction** — modular arithmetic to decompose numbers
- **Lookup tables** — precomputing the small fixed set FACT[0..=9] = {0!, …, 9!}

## Solution Approach

### Upper Bound: When to Stop Searching

A d-digit number is at least 10^(d−1). The maximum possible digit-factorial
sum for d digits is d × 9! = d × 362,880.

When d × 9! < 10^(d−1), no d-digit number can be a factorion.

```text
d    d × 9!         10^(d−1)      Reachable?
──   ──────────     ───────       ──────────
2      725,760            10      Yes
3    1,088,640           100      Yes
4    1,451,520         1,000      Yes
5    1,814,400        10,000      Yes
6    2,177,280       100,000      Yes
7    2,540,160     1,000,000      Yes (2,540,160 has 7 digits)
8    2,903,040    10,000,000      No  (still only 7 digits)
```

**Upper bound = 2,540,160**. Beyond this, no candidate can be a factorion
because the digit-factorial sum can never produce that many digits.

### Why the Bound Is Tight (And Loose)

The bound `d × 9!` is a *theoretical maximum* assuming every digit is 9.
Real numbers have mixed digits, so actual digit-factorial sums are far lower.
The two solutions are dwarfed by the bound:

```text
Number   Factorial-sum computation                      Equal?
──────   ─────────────────────────                      ──────
   145   1! + 4! + 5! = 1 + 24 + 120                       ✓
40,585   4! + 0! + 5! + 8! + 5! = 24 + 1 + 120 + 40,320 + 120  ✓
```

40,585 is the *largest* known factorion in base 10 — there are no others
above it (proven exhaustively by the bound argument).

### All Base-10 Factorions

There are exactly **four** factorions in base 10:

| n      | Digit factorial sum                     | Excluded by problem? |
|--------|-----------------------------------------|----------------------|
| 1      | 1! = 1                                  | ✓ (single digit)     |
| 2      | 2! = 2                                  | ✓ (single digit)     |
| 145    | 1! + 4! + 5! = 145                      |                      |
| 40,585 | 4! + 0! + 5! + 8! + 5! = 40,585         |                      |

## Complexity

- **Time**: O(U · d) where U = 2,540,160 and d ≤ 7
- **Space**: O(1) for the FACT[10] lookup table; O(k) for the (≤2) results

| Metric | Value |
|--------|-------|
| Time | ~16 ms (release) |
| Candidates checked | 2,540,150 |
| Solutions found | 2 |

### Possible Optimizations (Not Needed)

1. **Digit-multiset enumeration**: Instead of checking all numbers, enumerate
   non-decreasing digit multisets (≤ C(16,7) ≈ 11,440 for 7-digit numbers)
   and verify each. Drops the candidate count by ~200×.
2. **Early-exit when digit sum exceeds n**: For each candidate, accumulate
   digit factorials and break early if the partial sum exceeds n. Probably
   speeds up the average case slightly.

The plain bounded sweep is already fast enough (< 50 ms), so we keep it
straightforward.

## Rust Implementation Details

### Compile-Time Factorial Table

```rust
const fn digit_factorial_table() -> [u64; 10] {
    let mut table = [1u64; 10];
    let mut i = 1;
    let mut acc = 1u64;
    while i < 10 {
        acc *= i as u64;
        table[i] = acc;
        i += 1;
    }
    table
}
const FACT: [u64; 10] = digit_factorial_table();
```

`const fn` lets us build the lookup table at compile time, so the runtime
hot loop just does an array index — no `pow()` or repeated multiplication.

### Dynamic Upper Bound

```rust
let nine_fact = FACT[9];
let mut d = 2u64;
while d * nine_fact >= 10u64.pow((d - 1) as u32) {
    d += 1;
}
let upper_bound = (d - 1) * nine_fact;
```

Same idiom as p030 — derives 2,540,160 mechanically without hard-coding.

## Answer

**40,730** (sum of 145 + 40,585)

## Related Problems

- [[project-euler-p030]] — Digit Fifth Powers (same bounding technique with powers)
- [[project-euler-p020]] — Factorial Digit Sum (digits of 100!)
- **Problem 74** — Digit factorial chains (iterates the digit-factorial map)

## Links

- [[project-euler-p033]] — previous problem (digit-cancelling fractions)
- [[combinatorics-fundamentals]] — factorial definition and identities
- [[order-of-magnitude]] — bounding via digit-count arguments
- [Factorion (Wikipedia)](https://en.wikipedia.org/wiki/Factorion)
