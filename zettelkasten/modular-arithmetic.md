# Modular Arithmetic

*Tags: #algorithms #math #pattern #aoc #modular-arithmetic #number-theory*
*Links: [[zettel-index]] | [[Algorithms MOC]] | [[circular-list]] | [[index-tracking]] | [[math-foundations/README]]*

---

## Core Idea

**Modular arithmetic** restricts numbers to a fixed range `[0, m-1]` by wrapping around at modulus `m`. It's the mathematics behind circular indexing, hash functions, and cyclic patterns.

```
Regular:    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...
Mod 5:      0, 1, 2, 3, 4, 0, 1, 2, 3, 4,  0,  1, ...
```

**Notation**: `a ≡ b (mod m)` means `a` and `b` have the same remainder when divided by `m`.

---

## Rust's `%` Operator: Remainder, Not Modulus

**Critical distinction**: Rust's `%` is the **remainder** operator, not mathematical modulus. It preserves the sign of the dividend:

```rust
 7 % 5  //  2  (positive dividend → positive result)
-7 % 5  // -2  (negative dividend → NEGATIVE result)
 7 % -5 //  2  (sign follows dividend, not divisor)
-7 % -5 // -2
```

Mathematical modulus always returns a non-negative result: `-7 mod 5 = 3` (not -2).

### The Double-Modulus Pattern

To get true non-negative modulus in Rust:

```rust
fn modulo(a: i64, m: i64) -> i64 {
    ((a % m) + m) % m
}

// Step by step for a = -7, m = 5:
// Step 1: -7 % 5 = -2        (Rust remainder, can be negative)
// Step 2: -2 + 5 = 3         (shift into positive range)
// Step 3: 3 % 5 = 3          (normalize if step 2 overshot)
```

**Why three `%` operations?**

| Operation | Purpose | Example (-7 mod 5) |
|-----------|---------|---------------------|
| `a % m` | Reduce to range `(-m, m)` | -7 % 5 = -2 |
| `+ m` | Shift negatives to positive | -2 + 5 = 3 |
| `% m` | Handle case where result was already positive | If a=7: 7%5=2, 2+5=7, 7%5=2 |

The third `%` is necessary because when `a % m` is already positive, adding `m` would put us above the range.

---

## Common Patterns

### Circular Indexing

Wrap array access around:

```rust
let items = vec![10, 20, 30, 40, 50];
let n = items.len();

// Forward
items[(pos + offset) % n]

// Backward (negative offset)
items[((pos as i64 - offset as i64) % n as i64 + n as i64) as usize % n]
```

### Cycle Detection

When a process repeats after `cycle_len` steps, skip ahead:

```rust
// AoC 2022 Day 17 (Tetris cycle detection)
let remaining = total_steps - current_step;
let full_cycles = remaining / cycle_len;
let leftover = remaining % cycle_len;
// Simulate only `leftover` more steps, multiply cycle contribution
```

### Large Number Modulus

Keep intermediate results bounded during multiplication:

```rust
// AoC 2022 Day 11: Monkey business with worry overflow
// Product of all divisors keeps values bounded
let lcm: u64 = monkeys.iter().map(|m| m.divisor).product();
worry = worry % lcm;  // Preserves all divisibility tests
```

### Modular Inverse Position

For circular list reinsertion (see [[circular-list]]):

```rust
let modulus = (n - 1) as i64;  // n-1 because element is removed
let new_pos = ((old_pos as i64 + value) % modulus + modulus) % modulus;
```

---

## Properties

### Arithmetic Preservation

Modular operations distribute over addition and multiplication:

```rust
// These are equivalent (mod m):
(a + b) % m == ((a % m) + (b % m)) % m
(a * b) % m == ((a % m) * (b % m)) % m
```

This allows reducing intermediate values to prevent overflow:

```rust
// Instead of computing huge product then taking mod:
let result = values.iter().fold(0u64, |acc, &v| (acc + v) % m);
```

### Division Does NOT Preserve Modulus

```rust
// WARNING: This is WRONG
(a / b) % m != ((a % m) / (b % m)) % m

// For modular division, you need modular inverse (Fermat's little theorem):
// a / b mod m = a * b^(m-2) mod m  (when m is prime)
```

### Useful Identities

| Identity | Meaning |
|----------|---------|
| `(a + m) % m == a % m` | Adding modulus doesn't change result |
| `a % 1 == 0` | Everything is divisible by 1 |
| `a % a == 0` | Self-modulus is always 0 |
| `0 % m == 0` | Zero mod anything is zero |
| `a % m < m` | Result is always less than modulus |

---

## AoC Connections

| Problem | Pattern | Notes |
|---------|---------|-------|
| **AoC 2022 Day 20** | Double-modulus, n-1 modulus | Circular list mixing with negative values |
| **AoC 2022 Day 17** | Cycle detection | Tetris pattern repeats → skip via modulus |
| **AoC 2022 Day 11** | LCM modulus | Keep monkey worry values bounded |
| **AoC 2024 Day 14** | Position wrapping | Robots wrapping around grid boundaries |
| **AoC 2015 Day 25** | Modular exponentiation | Cantor diagonal code generation |

---

## Common Pitfalls

1. **Rust `%` is remainder, not modulus**: `-7 % 5 = -2`, not `3`. Always use double-modulus pattern for negative values.
2. **Using `n` instead of `n-1`**: When an element is removed from a circular list, the modulus is `n-1` (see [[circular-list]]).
3. **Integer overflow**: `a * b` may overflow before `% m`. Use `(a as u128 * b as u128) % m as u128` for large values, or reduce inputs first.
4. **Division under modulus**: Can't just use `%` after division. Need modular inverse (Fermat's little theorem or extended Euclidean algorithm).
5. **Mixing signed and unsigned**: Converting between `i64` and `usize` near modular operations requires care — cast BEFORE modulus, not after.

---

## Key Takeaways

1. **`((x % m) + m) % m`** is the standard Rust pattern for non-negative modulus
2. **Each `%` has a purpose**: reduce range, shift positive, normalize
3. **Modulus preserves `+` and `×`** but NOT division
4. **Cycle detection + modulus** = skip billions of steps in O(1)
5. **LCM modulus** keeps values bounded while preserving multiple divisibility tests

---

*Related: [[circular-list]] | [[index-tracking]] | [[math-foundations/README]] | [[sliding-window-patterns]]*
