# Project Euler Problem 25: 1000-Digit Fibonacci Number

**Solved**: 2026-03-29
**Difficulty**: 5%
**Category**: Big Number Arithmetic / Fibonacci Sequence

## Problem Summary

Find the index of the first term in the Fibonacci sequence to contain 1000 digits.

## Mathematical Concepts

### Primary Concepts
- **Fibonacci sequence** — F(n) = F(n-1) + F(n-2), with F(1) = F(2) = 1
- **Big number arithmetic** — digit-array representation for numbers exceeding native integer types

### Supporting Concepts
- **Golden ratio (phi)** — phi = (1+sqrt(5))/2 ≈ 1.618, governs Fibonacci growth rate
- **Binet's formula** — closed-form: F(n) = (phi^n - psi^n) / sqrt(5)
- **Logarithmic digit counting** — number of digits d = floor(log10(n)) + 1

## Solution Approach

### Growth Rate Analysis

Fibonacci numbers grow exponentially at rate phi:

```text
F(n) ≈ phi^n / sqrt(5)     (Binet's formula, psi^n → 0 for large n)

where phi = (1 + sqrt(5)) / 2 ≈ 1.6180339887
      psi = (1 - sqrt(5)) / 2 ≈ -0.6180339887
```

Since |psi| < 1, the psi^n term vanishes for large n, so F(n) ≈ phi^n / sqrt(5).

### Estimating the Index

A number has d digits when it >= 10^(d-1). We need:

```text
phi^n / sqrt(5) >= 10^(d-1)

Taking log10 of both sides:
n * log10(phi) - log10(sqrt(5)) >= d - 1

Solving for n:
n >= (d - 1 + log10(sqrt(5))) / log10(phi)
n >= (d - 1 + 0.3495) / 0.20898

For d = 1000:
n >= (999 + 0.3495) / 0.20898
n >= 999.3495 / 0.20898
n >= 4782.0
```

The estimate of 4782 matches the exact answer — the logarithmic analysis is precise because psi^4782 is negligibly small (~10^(-993)).

### Why Not Just Use Binet's Formula Directly?

Tempting, but floating-point precision kills it:
- `f64` has ~15 significant digits
- We need to determine if a number has *exactly* 1000 digits
- The rounding errors in phi^4782 would be enormous
- Would need arbitrary-precision floats, which is more complex than digit-array addition

### Big Number Addition

The practical approach: iterate Fibonacci using digit arrays.

```text
F(1) = [1]                        (1 digit)
F(2) = [1]                        (1 digit)
F(3) = [2]                        (1 digit)
...
F(7) = [3, 1]                     (2 digits)  ← first 2-digit
...
F(12) = [4, 4, 1]                 (3 digits)  ← first 3-digit
...
F(4782) = [... 1000 digits ...]   (1000 digits)  ← answer!
```

Addition with carry propagation (digits stored LSB-first):

```text
  F(11) = [9, 8]           (89)
+ F(12) = [4, 4, 1]        (144)
  ─────────────────
  Step 1: 9 + 4 = 13  → digit 3, carry 1
  Step 2: 8 + 4 + 1 = 13  → digit 3, carry 1
  Step 3: 0 + 1 + 1 = 2   → digit 2, carry 0
  Result: [3, 3, 2]        (233) = F(13) ✓
```

### Digit Count is Free

The beauty of LSB-first digit arrays: `vec.len()` IS the digit count.
No log10 computation, no string conversion — just check the vector length.
This makes the termination check O(1) per iteration.

## Complexity

- **Time**: O(n × d) where n ≈ 4782 iterations, d ≈ 1000 digits per addition
- **Space**: O(d) — only two digit arrays in memory at any time (previous and current)

### Breakdown

| Component | Cost | Count | Total |
|-----------|------|-------|-------|
| Additions | O(d) per add | ~4780 | ~4,780,000 digit ops |
| Carry propagation | O(1) amortized | included | — |
| Digit count check | O(1) | ~4780 | negligible |

## Performance

| Metric | Value |
|--------|-------|
| Time | 3.67 ms |
| Target | < 100 ms |
| Bottleneck | ~4.78M single-digit additions with carry |

### Possible Optimizations (Not Needed)

1. **Base-10000 digits**: Store 4 decimal digits per element → 4× fewer operations
2. **Matrix exponentiation**: F(n) via [[a,b],[b,a+b]]^n in O(log n) big multiplications
3. **Pisano periods**: Use F(n) mod 10^k for digit counting (overkill here)

The simple approach runs in 3.67ms — well within budget.

## Rust Implementation Details

### Digit-Array Pattern (Reused from p016, p020)

```rust
pub fn big_add(a: &[u8], b: &[u8]) -> Vec<u8> {
    let max_len = a.len().max(b.len());
    let mut result = Vec::with_capacity(max_len + 1);
    let mut carry = 0u8;
    for i in 0..max_len {
        let da = if i < a.len() { a[i] } else { 0 };
        let db = if i < b.len() { b[i] } else { 0 };
        let sum = da + db + carry;
        result.push(sum % 10);
        carry = sum / 10;
    }
    if carry > 0 { result.push(carry); }
    result
}
```

Key choices:
- **`u8` vs `u32`**: Uses `u8` (sufficient for 0-9), more cache-friendly for 1000-element arrays
- **LSB-first**: Carry propagates left-to-right, matching Vec growth direction
- **Pre-allocated capacity**: `max_len + 1` avoids reallocation on carry overflow
- **Owned return**: Returns new Vec rather than mutating, enabling clean `a = b; b = next;` swap

### Fibonacci Iteration

```rust
pub fn first_fib_with_digits(digits: usize) -> u64 {
    let mut a: Vec<u8> = vec![1]; // F(1)
    let mut b: Vec<u8> = vec![1]; // F(2)
    let mut index = 2u64;
    while b.len() < digits {
        let next = big_add(&a, &b);
        a = b;
        b = next;
        index += 1;
    }
    index
}
```

The `a = b; b = next;` pattern reuses allocations via move semantics — `a`'s old buffer is dropped, `b`'s buffer moves to `a` (zero-copy), and `next`'s new buffer moves to `b`.

## Answer

**4782**

## Verification

```text
F(4781) has 999 digits  (not enough)
F(4782) has 1000 digits (first to reach 1000)

Cross-check with estimate: (999 + 0.3495) / 0.20898 = 4782.0 ✓
```

## Related Problems

- **Problem 2** (Even Fibonacci Numbers) — Fibonacci with filtering
- **Problem 16** (Power Digit Sum) — same digit-array technique for 2^1000
- **Problem 20** (Factorial Digit Sum) — digit-array multiplication
- **Problem 104** (Pandigital Fibonacci Ends) — Fibonacci with digit properties

## Links

- [[project-euler-p016]] — digit-array arithmetic (multiply variant)
- [[project-euler-p020]] — digit-array arithmetic (factorial variant)
- [[set-theory-fundamentals]] — related mathematical foundations
- [Fibonacci (Wikipedia)](https://en.wikipedia.org/wiki/Fibonacci_sequence)
- [Binet's Formula (Wikipedia)](https://en.wikipedia.org/wiki/Jacques_Philippe_Marie_Binet)
- [Golden Ratio (Wikipedia)](https://en.wikipedia.org/wiki/Golden_ratio)
