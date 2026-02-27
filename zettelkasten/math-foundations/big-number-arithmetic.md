# Big Number Arithmetic - Digit-Array Technique

*Arbitrary precision integer arithmetic using decimal digit vectors*

---

## Core Concept

When numbers exceed native integer types (u64 ~18 digits, u128 ~38 digits), we represent them as **vectors of individual decimal digits** and implement arithmetic operations manually using carry propagation.

**Key idea**: Store digits least-significant-first in a `Vec<u32>`, perform operations digit-by-digit with carries, just like pencil-and-paper arithmetic.

---

## Mental Model

Think of it as **grade-school multiplication**, automated:

```
    3 2 7 6 8    (digits of 32768)
  ×         2
  -----------
carry: 1 1 1 0
    6 5 5 3 6    → 65536 = 2^16
```

The computer does exactly this, one digit at a time, propagating carries left.

---

## Operations

### Multiplication by Small Number (Most Common)

```rust
fn multiply_digit_array(digits: &mut Vec<u32>, factor: u32) {
    let mut carry = 0;
    for d in digits.iter_mut() {
        let product = *d * factor + carry;
        *d = product % 10;
        carry = product / 10;
    }
    while carry > 0 {
        digits.push(carry % 10);
        carry /= 10;
    }
}
```

**Used by**: Exponentiation (repeated multiply), factorial (multiply by 1, 2, 3, ...)

### Addition of Two Big Numbers

```rust
fn add_digit_arrays(a: &mut Vec<u32>, b: &[u32]) {
    let mut carry = 0;
    for i in 0..a.len().max(b.len()) {
        let sum = a.get(i).copied().unwrap_or(0)
                + b.get(i).copied().unwrap_or(0)
                + carry;
        if i < a.len() { a[i] = sum % 10; } else { a.push(sum % 10); }
        carry = sum / 10;
    }
    if carry > 0 { a.push(carry); }
}
```

**Used by**: Fibonacci computation, large sum problems

### Digit Sum

```rust
fn digit_sum(digits: &[u32]) -> u64 {
    digits.iter().map(|&d| d as u64).sum()
}
```

---

## Why LSB-First Storage?

Storing the least significant digit at index 0:

| Operation | LSB-First | MSB-First |
|-----------|-----------|-----------|
| Carry propagation | Natural (left to right) | Backwards |
| Number growth | `push()` at end (O(1)) | Insert at front (O(n)) |
| Digit access | `digits[0]` = ones | Shifts when number grows |
| Printing | Reverse needed | Natural order |

LSB-first is overwhelmingly better for arithmetic. Only downside: reverse for display.

---

## Digit Count Estimation

Before computing, estimate how many digits the result will have:

```
digits(n^k) = floor(k * log10(n)) + 1
digits(n!)  ≈ floor(n * log10(n/e) + 0.5 * log10(2*pi*n)) + 1  (Stirling)
```

Useful for `Vec::with_capacity()` pre-allocation.

---

## Verification Trick: Mod 9

The digit sum S(n) satisfies S(n) ≡ n (mod 9). This provides a quick sanity check:

```
2^1000 mod 9 = 7  →  digit_sum(2^1000) mod 9 should equal 7
1366 mod 9 = 7 ✓
```

Cannot prove correctness, but catches most errors.

---

## Implementations

- [[project-euler-p016]]: 2^1000 digit sum via repeated doubling (primary example)
- [[project-euler-p013]]: Large sum (avoided full big-number by truncation trick)
- **Future**: P20 (factorial digit sum), P25 (Fibonacci digit count), P56 (a^b digit sums)

---

## When to Use vs. Crate

| Approach | Pros | Cons |
|----------|------|------|
| Digit array (hand-rolled) | Zero deps, educational, sufficient for PE | Slower for very large numbers, limited ops |
| `num-bigint` crate | Optimized, full arithmetic, Display | External dependency, less learning value |

**Rule of thumb**: For Project Euler, digit arrays suffice. For production code, use a crate.

---

## Key Takeaways

1. **LSB-first** storage makes carry propagation natural and growth O(1)
2. **Multiply-with-carry** is the core primitive — exponentiation, factorial, etc. all reduce to it
3. **Mod 9 check** provides cheap verification of digit sums
4. **Pre-estimate digits** with log10 for efficient allocation
5. **Zero dependencies** — this technique needs only `Vec<u32>`

---

*Tags: #mathematics #big-number #arbitrary-precision #digit-array #carry-propagation #project-euler*

*Created*: 2026-02-26
*Last Updated*: 2026-02-26
*Implementations*: 1 (Project Euler P16)

*Links: [[zettel-index]] | [[math-foundations/README]] | [[project-euler-p016]] | [[project-euler-p013]] | [[order-of-magnitude]]*
