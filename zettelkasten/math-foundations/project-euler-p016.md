# Project Euler Problem 16: Power Digit Sum - Mathematical Analysis

*Status: Solved*
*Difficulty: 5%*
*Answer: 1366*
*Date: 2026-02-26*

---

## Problem Statement

2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 2^1000?

---

## Mathematical Foundation

### Why This Exceeds Native Integers

2^1000 has approximately 302 decimal digits:

```
log10(2^1000) = 1000 * log10(2) ≈ 1000 * 0.30103 ≈ 301.03
```

| Type | Max Digits | Sufficient? |
|------|-----------|-------------|
| u32  | 10        | No          |
| u64  | 19        | No          |
| u128 | 39        | No          |
| **Digit array** | **Unlimited** | **Yes** |

There is no closed-form formula for the digit sum of 2^n. We must compute the full number.

### Digit Sum Properties

The digit sum function S(n) has some known properties:
- S(n) ≡ n (mod 9) — the digit sum preserves the remainder mod 9
- For 2^1000: 2^1000 mod 9 = ? Since 2^1 ≡ 2, 2^2 ≡ 4, 2^3 ≡ 8, 2^4 ≡ 7, 2^5 ≡ 5, 2^6 ≡ 1 (mod 9), the cycle length is 6. 1000 mod 6 = 4, so 2^1000 ≡ 2^4 ≡ 7 (mod 9).
- Therefore S(2^1000) ≡ 7 (mod 9). Our answer 1366 mod 9 = 7. Confirmed.

This serves as a useful **verification check** but cannot replace computing the full number.

---

## Approach: Digit-Array Multiplication

### Algorithm

Store the number as a vector of decimal digits (least significant first). To multiply by 2, iterate through digits with carry propagation:

```
Initialize: digits = [1]   (represents 2^0 = 1)

For each of 1000 doublings:
  carry = 0
  For each digit d in digits:
    product = d * 2 + carry
    d = product % 10
    carry = product / 10
  While carry > 0:
    push carry % 10
    carry = carry / 10

Result: sum all digits
```

### Trace: Computing 2^4 = 16

```
Start:  [1]           → 1
×2:     [2]           → 2       (1*2=2, no carry)
×2:     [4]           → 4       (2*2=4, no carry)
×2:     [8]           → 8       (4*2=8, no carry)
×2:     [6, 1]        → 16      (8*2=16, d=6, carry=1, push 1)
```

Digits are stored LSB-first: `[6, 1]` represents 16 (6 in ones place, 1 in tens place).

### Why LSB-First?

Storing least significant digit at index 0 means:
- New digits from carry are **appended** (O(1) amortized with Vec)
- No need to shift existing digits when the number grows
- Natural for carry propagation (process low digits first)

---

## Generalization: Any Base

The implementation accepts any base, not just 2:

```rust
pub fn power_digit_sum(base: u32, exponent: u32) -> u64
```

The multiply-with-carry algorithm works identically for any base:
- `product = d * base + carry` (instead of `d * 2 + carry`)
- Each digit still stays in range [0, 9] after `% 10`
- Carry can be > 9 for larger bases (handled by the while loop)

---

## Complexity Analysis

### Time: O(exponent * digits)

Each doubling touches every digit once. The number of digits grows as:

```
digits(2^n) = floor(n * log10(2)) + 1
```

Total work = sum over n from 1 to 1000 of digits(2^n):

```
≈ sum(n=1..1000) of (n * 0.301)
≈ 0.301 * (1000 * 1001 / 2)
≈ 150,650 digit operations
```

In practice, about 150K simple multiply-and-carry operations — essentially instant.

### Space: O(digits) ≈ O(302)

Only the digit vector is allocated. Peak size: 302 u32 values = ~1.2 KB.

---

## Rust Implementation Details

```rust
pub fn power_digit_sum(base: u32, exponent: u32) -> u64 {
    let mut digits: Vec<u32> = vec![1]; // LSB-first

    for _ in 0..exponent {
        let mut carry = 0;
        for d in digits.iter_mut() {
            let product = *d * base + carry;
            *d = product % 10;
            carry = product / 10;
        }
        while carry > 0 {
            digits.push(carry % 10);
            carry /= 10;
        }
    }

    digits.iter().map(|&d| d as u64).sum()
}
```

**Key Rust features**:
- `iter_mut()`: In-place modification of digit vector
- `Vec::push()`: O(1) amortized growth for new digits from carry
- Iterator chain for final sum: `.iter().map().sum()`
- No allocations in hot loop (only `push` when number grows in digit count)

### Zero Dependencies

This avoids pulling in `num-bigint` or similar crates. The digit-array approach:
- Is more educational (understand the algorithm vs. black-box library)
- Keeps the zero-dependency philosophy of the project
- Is sufficient for Project Euler's typical big-number needs
- Could be extracted to `utils/` if future problems need arbitrary precision

---

## Verification

### Mod 9 Check

```
2^1000 mod 9 = 7  (since 2^6 ≡ 1 mod 9, and 1000 mod 6 = 4, so 2^4 = 16 ≡ 7)
1366 mod 9 = 1 + 3 + 6 + 6 = 16, 1 + 6 = 7 ✓
```

### Cross-Check with Example

```
2^15 = 32768
Digit sum: 3 + 2 + 7 + 6 + 8 = 26 ✓
```

---

## Related Problems

### Project Euler
- **Problem 13**: Large sum (also big numbers, but solved with truncation trick)
- **Problem 20**: Factorial digit sum (same digit-array technique, different operation)
- **Problem 25**: First Fibonacci with 1000 digits (digit-array addition)
- **Problem 56**: Powerful digit sum (a^b digit sums, generalizes this problem)

### Concept Connections
- [[big-number-arithmetic]]: Digit-array technique for arbitrary precision
- [[order-of-magnitude]]: log10 estimation for digit count
- [[project-euler-p013]]: Related big-number problem (used truncation instead)

---

## Code Repository

**Location**: `project_euler/src/problems/p016.rs`

**Key functions**:
- `power_digit_sum(base, exponent)` - Digit sum of base^exponent via digit-array arithmetic
- `solve()` - Returns answer (1366)

**Tests**: 3 unit tests (small powers, example, solve)

**Problem Statement**: `project_euler/Problem_Statements/p016.md`

---

## Metadata

**Tags**: `#big-number` `#digit-sum` `#arbitrary-precision` `#digit-array` `#carry-propagation`

**Concepts**: Digit-array arithmetic, multiply-with-carry, LSB-first storage, mod 9 verification

**Difficulty**: Mathematical reasoning (easy), Implementation (easy)

**Created**: 2026-02-26
**Language**: Rust
**Performance**: Essentially instant (~150K digit operations)

---

## Bidirectional Links

**Links From This Note**:
- [[big-number-arithmetic]] - Digit-array technique used here
- [[order-of-magnitude]] - log10 estimation for digit count
- [[project-euler-p013]] - Related big-number problem

**Links To This Note**:
- [[project-euler-problems]] - Full list of solved problems
- [[big-number-arithmetic]] - Uses P16 as primary example

---

*Last updated: 2026-02-26*
*Solution verified: Answer 1366 (mod 9 check: 1366 mod 9 = 7 = 2^1000 mod 9)*
