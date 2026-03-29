# Project Euler Problem 26: Reciprocal Cycles

**Solved**: 2026-03-29
**Difficulty**: 5%
**Category**: Number Theory / Modular Arithmetic

## Problem Summary

Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.

## Mathematical Concepts

### Primary Concepts
- **Multiplicative order** — smallest k where 10^k ≡ 1 (mod d)
- **Primitive roots** — when the order equals d-1 (maximum possible cycle)

### Supporting Concepts
- **Long division** — the mechanical process that generates decimal digits
- **Modular arithmetic** — remainder tracking in long division
- **Fermat's Little Theorem** — for prime p, a^(p-1) ≡ 1 (mod p)

## Solution Approach

### Why Remainders Determine Cycles

When computing 1/7 by long division:

```text
Step 1: 10 / 7 = 1 remainder 3    → digit 1
Step 2: 30 / 7 = 4 remainder 2    → digit 4
Step 3: 20 / 7 = 2 remainder 6    → digit 2
Step 4: 60 / 7 = 8 remainder 4    → digit 8
Step 5: 40 / 7 = 5 remainder 5    → digit 5
Step 6: 50 / 7 = 7 remainder 1    → digit 7
Step 7: 10 / 7 = 1 remainder 3    → back to Step 1!

Result: 1/7 = 0.(142857)  cycle length = 6
```

The remainder sequence is [3, 2, 6, 4, 5, 1, 3, ...]. When remainder 1 appears, the next step produces remainder 10 mod 7 = 3, restarting the cycle.

**Key insight**: The cycle length is the smallest k where 10^k ≡ 1 (mod d).

This is the **multiplicative order** of 10 modulo d, written ord_d(10).

### Connection to Your 0.999... Question

This is the same machinery! For 1/3:

```text
10 mod 3 = 1    → cycle length 1, repeating digit = 3
```

So 1/3 = 0.(3), and three copies give 0.(9) = 3 × 1/3 = 1. The cycle structure of decimal fractions is *why* 0.999... = 1: it's 9/9, and 9 divides evenly.

### Terminating vs Repeating

A fraction 1/d terminates when d = 2^a × 5^b (only factors of the base 10).

```text
1/2  = 0.5          (d = 2^1)
1/4  = 0.25         (d = 2^2)
1/5  = 0.2          (d = 5^1)
1/8  = 0.125        (d = 2^3)
1/20 = 0.05         (d = 2^2 × 5)
```

For mixed denominators like 1/6 = 1/(2 × 3):
- Factor of 2 → non-repeating prefix "1"
- Factor of 3 → repeating cycle "(6)"
- Result: 0.1(6)

**Algorithm**: Strip factors of 2 and 5 first, then find order of 10 mod (reduced d).

### Maximum Cycle Length and Primitive Roots

By Fermat's Little Theorem, for prime p: 10^(p-1) ≡ 1 (mod p).

So the cycle length always **divides** p-1. The maximum cycle length p-1 occurs when 10 is a **primitive root** mod p — meaning 10 generates all nonzero residues.

```text
d = 7 (prime):   cycle = 6 = 7-1     ✓ (10 is a primitive root mod 7)
d = 11 (prime):  cycle = 2 ≠ 10      (10 is NOT a primitive root mod 11)
d = 13 (prime):  cycle = 6 ≠ 12      (10 is NOT a primitive root mod 13)
d = 17 (prime):  cycle = 16 = 17-1   ✓ (10 is a primitive root mod 17)
d = 983 (prime): cycle = 982 = 983-1 ✓ (10 is a primitive root mod 983)
```

### Why the Answer Must Be Prime

For composite d, the cycle length divides phi(d) (Euler's totient), which is always less than d-1. So the longest possible cycle comes from a prime where 10 is a primitive root.

The search: find the largest prime p < 1000 where ord_p(10) = p-1.

Answer: **983** with cycle length **982**.

### Verification: 983 is Special

```text
983 is prime ✓
phi(983) = 982
ord_983(10) = 982    (10 is a primitive root mod 983)

The decimal 1/983 = 0.(d₁d₂d₃...d₉₈₂) with a 982-digit repeating block!
```

## Complexity

- **Time**: O(n × d_max) — for each d up to 1000, compute order up to d-1
- **Space**: O(1) — just tracking a single remainder

### Breakdown

| Component | Cost |
|-----------|------|
| Per denominator | O(d) modular multiplications |
| Total | O(sum of cycles) ≈ O(n × avg_cycle) |
| Measured | 216 µs |

## Performance

| Metric | Value |
|--------|-------|
| Time | 216 µs |
| Target | < 100 ms |
| Operations | ~998 denominators, ~250k modular multiplications total |

### Possible Optimizations (Not Needed)

1. **Search primes only**: Composites can't beat primes — skip them
2. **Search large primes first**: Start from 997 downward, stop when cycle = p-1
3. **Early exit**: If cycle_length(d) < current_best and d < current_best, skip

At 216µs these aren't worth the added complexity.

## Rust Implementation Details

### Stripping Base Factors

```rust
let mut reduced = d;
while reduced.is_multiple_of(2) { reduced /= 2; }
while reduced.is_multiple_of(5) { reduced /= 5; }
if reduced == 1 { return 0; }  // terminates
```

### Multiplicative Order via Simulation

```rust
let mut remainder = 10 % reduced;
let mut k = 1;
while remainder != 1 {
    remainder = (remainder * 10) % reduced;
    k += 1;
}
```

This simulates long division without storing digits — we only care about *when* the cycle repeats, not *what* digits it produces.

### Iterator-Based Search

```rust
(2..limit).max_by_key(|&d| cycle_length(d)).unwrap()
```

Clean functional style — `max_by_key` finds the d with the longest cycle.

## Answer

**983** (cycle length 982)

## Related Problems

- **Problem 64** (Odd Period Square Roots) — cycle detection in continued fractions
- **Problem 65** (Convergents of e) — continued fraction expansion
- **Problem 99** (Largest Exponential) — logarithmic comparison

## Links

- [[project-euler-p025]] — previous problem (Fibonacci / big numbers)
- [[set-theory-fundamentals]] — related mathematical foundations
- [Multiplicative Order (Wikipedia)](https://en.wikipedia.org/wiki/Multiplicative_order)
- [Primitive Root (Wikipedia)](https://en.wikipedia.org/wiki/Primitive_root_modulo_n)
- [Fermat's Little Theorem (Wikipedia)](https://en.wikipedia.org/wiki/Fermat%27s_little_theorem)
