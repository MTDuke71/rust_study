# Modular Exponentiation

**Concept**: Computing `a^b mod m` efficiently via exponentiation by squaring — O(log b) multiplications instead of O(b).

**Created**: 2026-04-15
**Tags**: #mathematics #number-theory #modular-arithmetic #algorithms #lcg

---

## The Problem

Computing `a^b mod m` naively requires `b` multiplications:

```
16807^1000000 mod 2147483647
```

The intermediate result `16807^1000000` has ~4.2 million digits. You can't even *store* it, let alone compute it. But the modular result fits in 31 bits.

---

## Key Insight: Binary Decomposition

Any exponent can be decomposed into powers of 2:

```
1000000 = 2^19 + 2^18 + 2^17 + 2^16 + 2^14 + 2^9 + 2^6
        = 0b11110100001001000000
```

Therefore:

```
a^1000000 = a^(2^19) × a^(2^18) × a^(2^17) × a^(2^16) × a^(2^14) × a^(2^9) × a^(2^6)
```

Each `a^(2^k)` is obtained by squaring the previous: `a^(2^k) = (a^(2^(k-1)))^2`.

Only 20 squarings + 7 multiplications = 27 total operations, vs 1,000,000 naive multiplications.

---

## Algorithm: Exponentiation by Squaring

### Step-by-Step

Walk the exponent bits right-to-left:

```
result = 1
base = a

while exp > 0:
    if exp is odd:          # current bit is 1
        result = result * base mod m
    base = base * base mod m    # square for next bit
    exp = exp >> 1              # shift to next bit
```

### Concrete Example

Compute `3^13 mod 100`:

```
13 in binary = 1101

Step  exp(bin)  bit  base           result
────  ────────  ───  ────           ──────
init  1101      -    3              1
  0   1101      1    3              1 × 3 = 3
  1   110       0    3² = 9        3 (skip)
  2   11        1    9² = 81       3 × 81 = 243 → 43
  3   1         1    81² = 6561    43 × 6561 → 43 × 61 = 2623 → 23
                     → 61

Result: 3^13 mod 100 = 23
```

Verify: 3^13 = 1,594,323. 1,594,323 mod 100 = 23.

---

## Rust Implementation

### Basic Version

```rust
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus;
    }
    result
}
```

### With Mersenne Fast-Mod (AoC 2017 Day 15)

When the modulus is a Mersenne prime `2^31 - 1`, replace `%` with bit operations:

```rust
const MODULUS: u64 = 2147483647; // 2^31 - 1

#[inline]
fn mersenne_mod(n: u64) -> u64 {
    let sum = (n & MODULUS) + (n >> 31);
    if sum >= MODULUS { sum - MODULUS } else { sum }
}

fn mod_pow(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    base %= MODULUS;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mersenne_mod(result * base);
        }
        exp >>= 1;
        base = mersenne_mod(base * base);
    }
    result
}
```

See [[math-foundations/mersenne-primes-fast-arithmetic]] for why this works.

---

## Killer Application: LCG Jump-Ahead

A Linear Congruential Generator (LCG) has the recurrence:

```
x_{n+1} = a * x_n mod m
```

Unrolling this reveals a pattern:

```
x_1 = a * x_0 mod m
x_2 = a * x_1 = a^2 * x_0 mod m
x_3 = a * x_2 = a^3 * x_0 mod m
...
x_k = a^k * x_0 mod m
```

Therefore, to jump ahead `k` steps:

```rust
fn jump_ahead(seed: u64, factor: u64, steps: u64) -> u64 {
    mersenne_mod(seed * mod_pow(factor, steps))
}
```

### Why This Matters

Without jump-ahead: computing `x_1000000` requires 1,000,000 sequential multiplications.

With jump-ahead: one `mod_pow` call (~20 multiply-and-square steps) computes `factor^1000000`, then one final multiplication gives `x_1000000`.

This enables **parallel block decomposition** of an otherwise sequential recurrence:

```
Block 0: start at x_0         = seed * factor^0
Block 1: start at x_1000000   = seed * factor^1000000
Block 2: start at x_2000000   = seed * factor^2000000
...
```

Each block independently generates its portion. Rayon distributes blocks across cores.

### AoC 2017 Day 15 — Concrete Example

Two Park-Miller generators with factors 16807 and 48271, modulus `2^31 - 1`:

```rust
// Part 1: Split 40M pairs into 40 blocks of 1M
(0..40).into_par_iter().map(|block| {
    let offset = block * 1_000_000;
    let a = jump_ahead(seed_a, 16807, offset);  // ~20 operations
    let b = jump_ahead(seed_b, 48271, offset);  // ~20 operations
    count_matches_in_block(a, b, 1_000_000)     // 1M sequential operations
}).sum()
```

Result: **333ms → 85ms** (Part 1 parallelized), then further to **22.7ms** with Part 2 also parallelized.

---

## Complexity

| Approach | Time | Space |
|----------|------|-------|
| Naive (`a * a * ... * a`) | O(b) multiplications | O(1) |
| Exponentiation by squaring | O(log b) multiplications | O(1) |
| With Mersenne fast-mod | O(log b) multiplications, no division | O(1) |

For `b = 1,000,000`: 20 operations vs 1,000,000. That's a **50,000× reduction**.

---

## Overflow Considerations

The intermediate product `base * base` must fit in the integer type:

| Modulus Range | Product Bits | Type Needed |
|---------------|-------------|-------------|
| m < 2^16 | up to 32 | u32 |
| m < 2^31 | up to 62 | u64 |
| m < 2^63 | up to 126 | u128 |
| m > 2^63 | > 126 | BigInt or Montgomery |

For Park-Miller (`m = 2^31 - 1`), the maximum product is `(2^31 - 2)^2 ≈ 2^62`, which fits comfortably in `u64`.

---

## Variants

### Modular Multiplicative Inverse

Using Fermat's Little Theorem (when m is prime):

```
a^(-1) ≡ a^(m-2) mod m
```

So `mod_pow(a, m - 2, m)` computes the modular inverse.

### Matrix Exponentiation

Same squaring technique works for matrices. Used for:
- Fibonacci in O(log n): `[F(n), F(n-1)] = [[1,1],[1,0]]^n × [1, 0]`
- Linear recurrences: any `x_n = c1*x_{n-1} + c2*x_{n-2} + ...`

---

## Assembly-Level View

In the AoC 2017 Day 15 hot loop, `mod_pow` compiles to ~20 iterations of:

```asm
imul  rbx, rbx              ; base = base * base
and   r12d, 2147483647      ; low 31 bits
shr   rbx, 31               ; high bits
lea   r13, [r12 + rbx]      ; sum = low + high
cmp   r13, 2147483647       ; compare to MODULUS
lea   rbx, [r12 + rbx - 2147483647]  ; speculative subtract
cmovb rbx, r13              ; branchless select
```

No `div` instruction anywhere. See AoC 2017 Day 15 ASM guide for full annotated assembly.

---

## Related Concepts

**Prerequisites**:
- [[math-foundations/modular-arithmetic]] - Modular operations, congruence
- [[math-foundations/mersenne-primes-fast-arithmetic]] - Division-free modulo for 2^p - 1

**Applications**:
- [[math-foundations/prime-number-theory]] - Fermat primality test uses mod_pow
- [[math-foundations/chinese-remainder-theorem]] - CRT with modular inverses
- AoC 2017 Day 15 — LCG jump-ahead for parallel block decomposition

**Extensions**:
- [[math-foundations/fibonacci-sequence]] - Matrix exponentiation variant
- [[math-foundations/linear-feedback-shift-registers]] - Related PRNG structures

---

**References**:
- [Wikipedia: Modular Exponentiation](https://en.wikipedia.org/wiki/Modular_exponentiation)
- [Wikipedia: Exponentiation by Squaring](https://en.wikipedia.org/wiki/Exponentiation_by_squaring)
- [Nayuki: Fast Skipping in LCG](https://www.nayuki.io/page/fast-skipping-in-a-linear-congruential-generator)
- [AoC 2017 Day 15 Implementation](../../advent_of_code/aoc2017/src/solver/day15.rs)
- [AoC 2017 Day 15 ASM Guide](../../advent_of_code/aoc2017/Problem_Statements/days/day15_asm_guide.md)
