# Project Euler Problem 30: Digit Fifth Powers

**Solved**: 2026-03-31
**Difficulty**: 5%
**Category**: Number Theory / Narcissistic Numbers

## Problem Summary

Find all numbers that equal the sum of fifth powers of their digits, then
sum them. These are a specific case of narcissistic numbers (also called
Armstrong numbers or pluperfect digital invariants).

## Mathematical Concepts

### Primary Concepts
- **Narcissistic numbers** — numbers equal to the sum of their digits raised to some power
- **Upper bound derivation** — proving when to stop searching via digit-count arguments

### Supporting Concepts
- **Digit extraction** — modular arithmetic to decompose numbers
- **Growth rate comparison** — polynomial (digit sum) vs exponential (number magnitude)

## Solution Approach

### Upper Bound: When to Stop Searching

The critical insight is that digit-power sums grow linearly with digit count,
while the numbers themselves grow exponentially. At some point, no number
can equal its digit-power sum.

For power p, a d-digit number n satisfies:
- **Minimum value**: n ≥ 10^(d−1)
- **Maximum digit-power sum**: d × 9^p

When d × 9^p < 10^(d−1), no d-digit number can be narcissistic.

For p = 5 (9⁵ = 59,049):

```text
d    d × 9⁵      10^(d−1)    Can a d-digit number work?
──   ────────     ────────    ──────────────────────────
1    59,049       1           Yes (but excluded by problem)
2    118,098      10          Yes
3    177,147      100         Yes
4    236,196      1,000       Yes
5    295,245      10,000      Yes
6    354,294      100,000     Yes (354,294 is 6 digits)
7    413,343      1,000,000   No! (413,343 is only 6 digits)
```

**Upper bound = 354,294**. Beyond this, impossible.

### Why the Bound Is Tight

The bound d × 9^p is the *theoretical maximum* (all digits are 9). Real numbers
have mixed digits, so actual digit-power sums are much lower. The six solutions
are all well below 354,294:

```text
Number    Digit⁵ Sum Computation                          Equal?
──────    ─────────────────────────────                    ──────
4,150     4⁵ + 1⁵ + 5⁵ + 0⁵ = 1024+1+3125+0 = 4,150     ✓
4,151     4⁵ + 1⁵ + 5⁵ + 1⁵ = 1024+1+3125+1 = 4,151     ✓
54,748    5⁵+4⁵+7⁵+4⁵+8⁵ = 3125+1024+16807+1024+32768   ✓
92,727    9⁵+2⁵+7⁵+2⁵+7⁵ = 59049+32+16807+32+16807      ✓
93,084    9⁵+3⁵+0⁵+8⁵+4⁵ = 59049+243+0+32768+1024       ✓
194,979   1⁵+9⁵+4⁵+9⁵+7⁵+9⁵ = 1+59049+1024+59049+16807+59049  ✓
```

### Generalization Across Powers

The same upper-bound technique works for any power:

| Power | Upper bound | Solutions | Sum |
|-------|-------------|-----------|-----|
| 3 | 2,916 | {153, 370, 371, 407} | 1,301 |
| 4 | 32,805 | {1634, 8208, 9474} | 19,316 |
| 5 | 354,294 | {4150, 4151, 54748, 92727, 93084, 194979} | 443,839 |
| 6 | 2,125,764 | {548834} | 548,834 |

As power increases, the upper bound grows but solutions become sparser.

### Why 4150 and 4151 Are Neighbors

An unusual property: 4150 and 4151 are consecutive narcissistic numbers.
Changing the last digit from 0⁵ = 0 to 1⁵ = 1 adds exactly 1 to both
the number and its digit-power sum. This only works when the changed
digit goes from 0 to 1 (since 0⁵ = 0 and 1⁵ = 1, difference is 1).

## Complexity

- **Time**: O(U × d) where U = upper bound, d = digits per number (~6)
- **Space**: O(1) beyond the results vector

| Metric | Value |
|--------|-------|
| Time | 5.0 ms |
| Candidates checked | 354,293 |
| Solutions found | 6 |

### Possible Optimizations (Not Needed)

1. **Precompute digit^5 table**: Store 0⁵ through 9⁵ in an array (10 entries).
   Avoids calling `pow()` repeatedly. Would reduce time but already fast.
2. **Digit combination enumeration**: Instead of checking all numbers, enumerate
   all multisets of digits and check if they form a valid number. Reduces search
   space dramatically for higher powers.

## Rust Implementation Details

### Generic Power Parameter

Both `digit_power_sum()` and `find_digit_power_numbers()` accept `power: u32`,
making them reusable for fourth-power validation (the example in the problem
statement) and any other power.

### Dynamic Upper Bound

```rust
let nine_pow = 9u64.pow(power);
let mut d = 2u64;
while d * nine_pow >= 10u64.pow((d - 1) as u32) {
    d += 1;
}
let upper_bound = (d - 1) * nine_pow;
```

This computes the correct upper bound for any power, not just 5.

## Answer

**443,839** (sum of 4150 + 4151 + 54748 + 92727 + 93084 + 194979)

## Related Problems

- **Problem 16** (Power Digit Sum) — sum of digits of 2¹⁰⁰⁰
- **Problem 20** (Factorial Digit Sum) — sum of digits of 100!
- **Problem 34** (Digit Factorials) — numbers equal to sum of factorials of digits
- **Problem 92** (Square Digit Chains) — iterating digit-square sums

## Links

- [[project-euler-p029]] — previous problem (distinct powers)
- [[project-euler-p016]] — digit sum techniques
- [Narcissistic Number (Wikipedia)](https://en.wikipedia.org/wiki/Narcissistic_number)
- [Armstrong Number (Wolfram MathWorld)](https://mathworld.wolfram.com/NarcissisticNumber.html)
