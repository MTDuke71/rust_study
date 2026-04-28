# Project Euler Problem 36: Double-base Palindromes

**Solved**: 2026-04-27
**Difficulty**: 5%
**Category**: Number Theory / Palindromes / Base Representation

## Problem Summary

Find the sum of all positive integers below 10⁶ whose decimal AND binary
representations are both palindromes (no leading zeros in either).

## Mathematical Concepts

### Primary Concepts
- **Palindromes** ([[palindromes]]) — sequences invariant under reversal
- **Base representation** ([[base-conversion]]) — same number, different digit sequences

### Supporting Concepts
- **Parity from binary representation** — least-significant bit determines odd/even

## Why Every Double-base Palindrome Is Odd

A binary palindrome with no leading zeros has its most-significant bit equal
to 1. Palindromicity forces the least-significant bit to also be 1.
Therefore the number ≡ 1 (mod 2), i.e., **odd**.

This halves the search space: we iterate only odd integers below 10⁶.

The decimal-side analogue is weaker: decimal palindromes can end in any
non-zero digit, so the parity constraint comes entirely from the binary
side. (And since the binary-side test is the more selective of the two,
filtering odd-only loses no double-base palindromes.)

## Solution Approach

### Integer-reversal palindrome check (no strings)

For decimal:
```text
reversed = 0
while n > 0:
    reversed = reversed * 10 + (n % 10)
    n /= 10
return reversed == original_n
```

For binary, the same idiom with shifts and masks:
```text
reversed = 0
while n > 0:
    reversed = (reversed << 1) | (n & 1)
    n >>= 1
return reversed == original_n
```

Both are O(d) where d is the digit count in the relevant base, with no heap
allocation. String-based palindrome checks would require a Vec<u8> per call
and dominate the runtime.

### Order matters: check decimal first

Decimal palindromes are denser at small values but sparser overall (only
~1,990 decimal palindromes below 10⁶), while binary palindromes are sparser
in absolute terms (~63 below 10⁶) but cheaper to test (longer digit string,
so any mismatch in the middle terminates the test fast).

We check **decimal first** because most odd integers fail it immediately,
short-circuiting the more expensive binary check. (At this scale either
order is fine; the choice matters more if `limit` grows by orders of
magnitude.)

## All 9 Double-base Palindromes Below 10⁶

```text
n          decimal          binary
─────      ───────          ─────────────────────
    1      "1"              "1"
    3      "3"              "11"
    5      "5"              "101"
    7      "7"              "111"
    9      "9"              "1001"
   33      "33"             "100001"
   99      "99"             "1100011"
  313      "313"            "100111001"
  585      "585"            "1001001001"
  717      "717"            "1011001101"
  ...
```

The full set under 10⁶ is:
1, 3, 5, 7, 9, 33, 99, 313, 585, 717, 7447, 9009, 15351, 32223, 39993,
53235, 53835, 73737, 585585.

Sum = **872,187**.

## Complexity

- **Time**: O(N · log N) — each candidate costs O(log₁₀ N + log₂ N) digit work
- **Space**: O(1)

| Metric | Value |
|--------|-------|
| Time | < 5 ms (release) |
| Candidates checked | 500,000 (odd-only) |
| Solutions found | 19 |

### Optimization Not Taken: Palindrome Generation

A faster algorithm generates decimal palindromes directly (~1,000 below 10⁶)
and tests each for binary palindromicity. This would cut the candidate count
by ~500× but adds palindrome-construction code complexity. The brute-force
sweep finishes well under our 100 ms target, so we keep the code simple.

If `limit` grew to 10⁹ or beyond, palindrome generation becomes essential.

## Rust Implementation Details

### `step_by(2)` over odd integers

```rust
(1..limit).step_by(2)
   .filter(|&n| is_decimal_palindrome(n) && is_binary_palindrome(n))
   .sum()
```

The iterator chain expresses the algorithm declaratively without
`for`/`if` boilerplate. The `&&` short-circuit enforces the
"decimal first" optimization.

### Bit-reversal as a shift-and-OR loop

The binary palindrome check uses Rust's native `<<`, `>>`, `&` operators on
`u64`. No `format!`, no allocation. Each iteration shifts the accumulator
left by one and ORs in the bottom bit of the residual.

## Answer

**872,187**

## Related Problems

- [[project-euler-p004]] — Largest palindrome product (decimal palindromes only)
- **Problem 55** — Lychrel numbers (palindromes via reverse-and-add iteration)
- **Problem 125** — Palindromic sums of consecutive squares

## Links

- [[project-euler-p035]] — previous problem (circular primes — also digit-rotation)
- [[palindromes]] — definitions, properties, divisibility patterns
- [[base-conversion]] — base-2 / base-10 mechanics
- [Palindromic Number (Wolfram MathWorld)](https://mathworld.wolfram.com/PalindromicNumber.html)
