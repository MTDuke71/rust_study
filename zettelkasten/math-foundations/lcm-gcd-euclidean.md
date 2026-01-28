# Least Common Multiple, Greatest Common Divisor, and Euclidean Algorithm

**Category**: Number Theory, Algorithms  
**Created**: January 27, 2026  
**Related Problems**: [[project-euler-p005]]

## Definitions

### Greatest Common Divisor (GCD)

The **greatest common divisor** of two integers $a$ and $b$ is the largest positive integer that divides both $a$ and $b$.

**Notation**: $\gcd(a, b)$ or $(a, b)$

**Properties**:
- $\gcd(a, b) = \gcd(b, a)$ (commutative)
- $\gcd(a, 0) = a$ (base case)
- $\gcd(a, b) = \gcd(b, a \bmod b)$ (Euclidean algorithm)
- $\gcd(a, b) \cdot \text{lcm}(a, b) = a \cdot b$ (fundamental relationship)

**Examples**:
- $\gcd(12, 8) = 4$ (common divisors: 1, 2, 4)
- $\gcd(17, 19) = 1$ (coprime/relatively prime)
- $\gcd(48, 18) = 6$

### Least Common Multiple (LCM)

The **least common multiple** of two integers $a$ and $b$ is the smallest positive integer that is divisible by both $a$ and $b$.

**Notation**: $\text{lcm}(a, b)$ or $[a, b]$

**Properties**:
- $\text{lcm}(a, b) = \text{lcm}(b, a)$ (commutative)
- $\text{lcm}(a, b) = \frac{a \cdot b}{\gcd(a, b)}$ (efficient computation)
- $\text{lcm}(a, b, c) = \text{lcm}(\text{lcm}(a, b), c)$ (associative)
- $\text{lcm}(a, 1) = a$

**Examples**:
- $\text{lcm}(4, 6) = 12$ (multiples of 4: 4, 8, 12, ...; multiples of 6: 6, 12, ...)
- $\text{lcm}(12, 8) = 24$
- $\text{lcm}(7, 11) = 77$ (coprime → lcm = product)

## Euclidean Algorithm

### Algorithm Description

**Purpose**: Efficiently compute $\gcd(a, b)$ using repeated division.

**Principle**: $\gcd(a, b) = \gcd(b, a \bmod b)$

**Why it works**: Any common divisor of $a$ and $b$ also divides $a \bmod b$ (the remainder).

### Mathematical Proof

**Claim**: $\gcd(a, b) = \gcd(b, r)$ where $r = a \bmod b$

**Proof**:
1. Write $a = qb + r$ (division algorithm) where $0 \le r < b$
2. Let $d = \gcd(a, b)$. Then $d | a$ and $d | b$
3. Since $r = a - qb$, we have $d | r$ (divisibility is closed under linear combinations)
4. Therefore, $d$ is a common divisor of $b$ and $r$
5. Conversely, any common divisor of $b$ and $r$ divides $a = qb + r$
6. Thus, $\gcd(a, b) = \gcd(b, r)$ □

### Pseudocode

```
function gcd(a, b):
    while b ≠ 0:
        temp = b
        b = a mod b
        a = temp
    return a
```

### Example Execution

Compute $\gcd(48, 18)$:

```
gcd(48, 18)
= gcd(18, 48 mod 18)
= gcd(18, 12)
= gcd(12, 18 mod 12)
= gcd(12, 6)
= gcd(6, 12 mod 6)
= gcd(6, 0)
= 6
```

**Steps**: 48 → 18 → 12 → 6 → 0

### Complexity

**Time Complexity**: $O(\log \min(a, b))$

**Why**: Each step reduces the larger number by at least half (worst case: consecutive Fibonacci numbers).

**Proof sketch**: 
- If $b > a/2$, then $a \bmod b < a/2$ (remainder is small)
- If $b \le a/2$, then next step has $a' = b \le a/2$
- Either way, the larger number shrinks by at least 2× every 2 steps
- Therefore, $O(\log a)$ steps

**Space Complexity**: $O(1)$ (iterative), $O(\log \min(a, b))$ (recursive stack)

## Computing LCM from GCD

### Formula

$$\text{lcm}(a, b) = \frac{a \cdot b}{\gcd(a, b)}$$

### Why This Works

**Proof**:
1. Let $d = \gcd(a, b)$
2. Write $a = d \cdot m$ and $b = d \cdot n$ where $\gcd(m, n) = 1$ (coprime)
3. Any common multiple must be divisible by both $a$ and $b$
4. Smallest such multiple is $d \cdot m \cdot n = \frac{a \cdot b}{d} = \frac{a \cdot b}{\gcd(a, b)}$ □

### Example

Compute $\text{lcm}(12, 8)$:
- $\gcd(12, 8) = 4$
- $\text{lcm}(12, 8) = \frac{12 \cdot 8}{4} = \frac{96}{4} = 24$ ✓

**Verification**: Multiples of 12: {12, 24, 36, ...}, Multiples of 8: {8, 16, 24, ...}

### Implementation (Rust)

```rust
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

// For multiple numbers (associative property)
fn lcm_range(n: u64) -> u64 {
    (1..=n).fold(1, lcm)
}
```

**Note**: Compute $a \cdot b$ before division to avoid precision loss. Watch for overflow!

## LCM for Multiple Numbers

### Associative Property

$\text{lcm}(a, b, c) = \text{lcm}(\text{lcm}(a, b), c)$

This allows folding/reducing over a sequence:

```
lcm(1, 2, 3, 4, 5) = lcm(lcm(lcm(lcm(1, 2), 3), 4), 5)
```

### Example: LCM(1..10)

```
lcm(1, 2) = 2
lcm(2, 3) = 6
lcm(6, 4) = 12
lcm(12, 5) = 60
lcm(60, 6) = 60
lcm(60, 7) = 420
lcm(420, 8) = 840
lcm(840, 9) = 2520
lcm(2520, 10) = 2520  ✓
```

**Result**: $\text{lcm}(1, 2, ..., 10) = 2520$

## Prime Factorization Method

### Alternative Approach

Instead of iterative LCM, use prime factorization:

1. Factor each number into primes
2. For each prime $p$, take the **highest power** appearing in any factorization
3. Multiply these highest powers

### Example: LCM(12, 18, 20)

**Factorizations**:
- $12 = 2^2 \cdot 3$
- $18 = 2 \cdot 3^2$
- $20 = 2^2 \cdot 5$

**Highest powers**:
- $2^2$ (from 12 or 20)
- $3^2$ (from 18)
- $5^1$ (from 20)

**LCM**: $2^2 \cdot 3^2 \cdot 5 = 4 \cdot 9 \cdot 5 = 180$ ✓

### When to Use

- **Iterative LCM**: Small number of operands, numbers already available
- **Prime factorization**: Range of consecutive integers, need all prime powers

## Applications

### 1. Fraction Addition

Adding fractions requires common denominator (LCM of denominators):

$$\frac{a}{b} + \frac{c}{d} = \frac{a \cdot \frac{\text{lcm}(b,d)}{b} + c \cdot \frac{\text{lcm}(b,d)}{d}}{\text{lcm}(b, d)}$$

### 2. Scheduling Problems

**Problem**: Two events occur every $a$ and $b$ time units. When do they coincide?

**Answer**: Every $\text{lcm}(a, b)$ time units.

**Example**: Bells ring every 6 and 8 minutes → coincide every $\text{lcm}(6, 8) = 24$ minutes.

### 3. Gear Ratios

Two gears with $a$ and $b$ teeth return to starting position after $\text{lcm}(a, b)$ rotations.

### 4. Number Theory

- **Modular arithmetic**: Chinese Remainder Theorem uses LCM
- **Cryptography**: RSA key generation uses GCD (coprimality check)
- **Diophantine equations**: Solutions involve GCD/LCM

## Extended Euclidean Algorithm

### Purpose

Find integers $x$ and $y$ such that:

$$ax + by = \gcd(a, b)$$

This is Bézout's identity.

### Algorithm

```
function extended_gcd(a, b):
    if b = 0:
        return (a, 1, 0)  // gcd, x, y
    else:
        (g, x1, y1) = extended_gcd(b, a mod b)
        x = y1
        y = x1 - (a // b) * y1
        return (g, x, y)
```

### Example

Compute $\gcd(48, 18)$ and find $x, y$ where $48x + 18y = \gcd(48, 18)$:

```
gcd(48, 18) = 6
48 × (-1) + 18 × 3 = -48 + 54 = 6  ✓
```

### Applications

- **Modular inverses**: Find $a^{-1} \pmod{m}$
- **Linear congruences**: Solve $ax \equiv b \pmod{m}$
- **Chinese Remainder Theorem**: Combine modular equations

## Related Concepts

- [[prime-factorization]] - Alternative method for LCM using prime powers
- [[modular-arithmetic]] - Uses GCD for inverse computation
- [[project-euler-p005]] - LCM of range 1-20
- [[number-theory-fundamentals]] - Divisibility, primes, factorization

## References

- *Introduction to Algorithms* (CLRS) - Section 31.2: GCD
- *Concrete Mathematics* - Chapter 4: Number Theory
- *Project Euler Problem 5* - LCM application
- [[project-euler-p005]] - Rust implementation

---

*Links:*
- **Applications**: [[project-euler-p005]]
- **Related Theory**: [[prime-factorization]], [[modular-arithmetic]]
- **Implementations**: `project_euler/src/problems/p005.rs`, `project_euler/src/utils/number_theory.rs`

*Tags:* #number-theory #algorithms #gcd #lcm #euclidean-algorithm #divisibility
