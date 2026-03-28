# Project Euler Problem 24: Lexicographic Permutations

**Solved**: 2026-03-28
**Difficulty**: 5%
**Category**: Combinatorics / Factoradic Number System

## Problem Summary

Find the 1,000,000th lexicographic permutation of the digits 0–9.

## Mathematical Concepts

### Primary Concepts
- **Factoradic number system** — represents integers using factorial place values
- **Lehmer code** — maps permutations to integers via inversion counts

### Supporting Concepts
- **Lexicographic ordering** — dictionary-style ordering of sequences
- **Factorial** — n! counts total permutations of n elements

## Solution Approach

### The Factoradic Insight

With n elements, there are n! total permutations. The key insight: permutations sharing the same first element form a contiguous block of (n-1)! entries.

For 10 digits: 10! = 3,628,800 total permutations, each "first digit" block has 9! = 362,880.

### Algorithm: Factorial Decomposition

To find the nth permutation (0-indexed) of digits [0..9]:

```text
Available: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
Target: 999,999 (1,000,000th, 0-indexed)

Step 1: 999,999 / 9! = 999,999 / 362,880 = 2 remainder 274,239
        → Pick digits[2] = 2, remaining: [0,1,3,4,5,6,7,8,9]

Step 2: 274,239 / 8! = 274,239 / 40,320 = 6 remainder 32,319
        → Pick digits[6] = 7, remaining: [0,1,3,4,5,6,8,9]

Step 3: 32,319 / 7! = 32,319 / 5,040 = 6 remainder 2,079
        → Pick digits[6] = 8, remaining: [0,1,3,4,5,6,9]

... (continue for all 10 positions)

Result: 2783915460
```

Each step: `index = remainder / (positions_left - 1)!`, pick that digit, remove it.

### Full Trace Table

```text
Orig.  Remaindr  digits   blck  blocks  digit used    remndr in
 size   searched  in use   size  passed   in result    curr. blk
            (R)             (B)   (Q)=R/B               (N)
3628800  1000000    10   362880    2   01<2>3456789 2    274240
 362880   274240     9    40320    6    013456<7>89 7     32320
  40320    32320     8     5040    6     013456<8>9 8      2080
   5040     2080     7      720    2      01<3>4569 3       640
    720      640     6      120    5       01456<9> 9        40
    120       40     5       24    1        0<1>456 1        16
     24       16     4        6    2         04<5>6 5         4
      6        4     3        2    2          04<6> 6         0
      2        0     2        1    0           <0>4 0         0
      1        0   n.a.    n.a.    0            <4> 4         0

                 2783915046
                 2783915064
                 2783915406
                 2783915460  <---1,000,000th
permutation is:  2783915604  member # 1000001
                 2783915640
```

### Why This Works

The factoradic decomposition is a bijection between [0, n!) and permutations of n elements. It's equivalent to converting the target number to the factorial number system, where place values are 1!, 2!, 3!, ..., (n-1)!.

## Complexity

- **Time**: O(n²) — n iterations, each with an O(n) removal from a vec
- **Space**: O(n)

For n=10 this is trivially fast. For large n, a Fenwick tree could reduce to O(n log n).

## Performance

| Metric | Value |
|--------|-------|
| Time | 84 ns |
| Target | < 100 ms |
| Note | ~1,000,000× faster than generating all permutations |

The factoradic approach makes this essentially instant — just 10 divides and 10 vec removals.

## Rust Implementation Details

### Factorial Reuse
```rust
use crate::utils::combinatorics::factorial;
```
Reuses the shared factorial utility.

### Digit Selection via Vec::remove
```rust
let mut digits: Vec<u64> = (0..num_digits).collect();
for i in (1..num_digits).rev() {
    let fact = factorial(i);
    let index = remainder / fact;
    remainder %= fact;
    result.push(digits.remove(index as usize));
}
```
`Vec::remove` shifts elements left — O(n) per call, but n=10 so negligible. For very large n, a BTreeSet or indexed structure would be better.

## Answer

**2,783,915,460**

## Related Problems

- **Problem 41** (Pandigital Prime) — also involves permutations of digits
- **Problem 43** (Sub-string Divisibility) — permutations with divisibility constraints
- **Problem 49** (Prime Permutations) — permutations that are all prime

## Links

- [[project-euler-p015]] — also uses combinatorics (binomial coefficients)
- [Factoradic (Wikipedia)](https://en.wikipedia.org/wiki/Factorial_number_system)
- [Lehmer code (Wikipedia)](https://en.wikipedia.org/wiki/Lehmer_code)
