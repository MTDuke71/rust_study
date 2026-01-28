# Project Euler Problem 5: Smallest Multiple

**Difficulty**: Easy  
**Published**: November 30, 2001  
**Solved**: January 27, 2026  
**Category**: Number Theory, Least Common Multiple  
**Related Concepts**: [[lcm-gcd-euclidean]], [[prime-factorization]]

## Problem Statement

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

**Given**: Example: LCM(1..10) = 2520  
**Find**: LCM(1..20)

## Solution Overview

**Answer**: **232,792,560**

**Approach**: Iterative LCM using Euclidean GCD algorithm

## Mathematical Analysis

### The LCM Function

**Definition**: The **Least Common Multiple** of a set of numbers is the smallest number divisible by all of them.

**Properties**:
- $\text{lcm}(a, b) \times \gcd(a, b) = a \times b$ (fundamental relationship)
- $\text{lcm}(a, b) = \frac{a \times b}{\gcd(a, b)}$ (efficient computation)
- LCM is **associative**: $\text{lcm}(a, b, c) = \text{lcm}(\text{lcm}(a, b), c)$

**Associativity allows folding**:
$$\text{lcm}(1, 2, 3, \ldots, n) = \text{lcm}(\text{lcm}(\cdots\text{lcm}(\text{lcm}(1, 2), 3)\cdots), n)$$

### Why Associativity Matters

Can compute LCM of range by reduction/folding:

```
lcm(1, 2, 3, 4, 5)
= lcm(lcm(1, 2), 3, 4, 5)
= lcm(2, 3, 4, 5)
= lcm(lcm(2, 3), 4, 5)
= lcm(6, 4, 5)
= lcm(lcm(6, 4), 5)
= lcm(12, 5)
= 60  ✓
```

**In Rust**: Perfect for `fold()` iterator method!

## Two Approaches

### Approach 1: Iterative LCM (Chosen)

**Algorithm**:
1. Start with LCM = 1
2. For each number $k$ from 1 to $n$:
   - Compute $\text{lcm}(\text{current}, k)$ using $\gcd$
   - Update current LCM
3. Return final LCM

**Implementation**:
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

fn smallest_multiple(n: u64) -> u64 {
    (1..=n).fold(1, lcm)
}
```

**Complexity**:
- **Time**: $O(n \log(\text{max}))$ where max is the LCM result
  - Each GCD computation: $O(\log(\text{max}))$ (Euclidean algorithm)
  - $n$ LCM operations: $O(n \log(\text{max}))$
- **Space**: $O(1)$

**Advantages**:
- ✅ Simple, elegant, functional style
- ✅ Uses standard mathematical operations (GCD, LCM)
- ✅ Generalizes to any range
- ✅ Leverages Rust's iterator methods

**Disadvantages**:
- ❌ Computes intermediate LCMs (some redundancy)
- ❌ Doesn't exploit structure of consecutive integers

### Approach 2: Prime Factorization

**Algorithm**:
1. Find all primes $p \le n$
2. For each prime $p$, find highest power $p^k$ where $p^k \le n$
3. Multiply all highest powers: $\text{lcm} = \prod p_i^{k_i}$

**Example for n = 20**:

**Primes ≤ 20**: 2, 3, 5, 7, 11, 13, 17, 19

**Highest powers**:
- $2^4 = 16$ (since $2^5 = 32 > 20$)
- $3^2 = 9$ (since $3^3 = 27 > 20$)
- $5^1 = 5$ (since $5^2 = 25 > 20$)
- $7^1 = 7$ (since $7^2 = 49 > 20$)
- $11^1 = 11$
- $13^1 = 13$
- $17^1 = 17$
- $19^1 = 19$

**LCM**:
$$\text{lcm}(1..20) = 2^4 \times 3^2 \times 5 \times 7 \times 11 \times 13 \times 17 \times 19$$

$$= 16 \times 9 \times 5 \times 7 \times 11 \times 13 \times 17 \times 19 = 232,792,560$$

**Complexity**:
- **Time**: $O(n \log \log n)$ (sieve for primes) + $O(\pi(n) \log n)$ (power computation)
  - $\pi(n)$ = number of primes ≤ n ≈ $n / \ln(n)$
  - Total: $O(n \log n)$
- **Space**: $O(n)$ (sieve array)

**Advantages**:
- ✅ More efficient for large $n$
- ✅ Directly computes minimal factorization
- ✅ No redundant intermediate computations

**Disadvantages**:
- ❌ More complex (requires prime sieve)
- ❌ More code, harder to verify
- ❌ Overkill for small $n$ (like 20)

**Choice**: Used **iterative LCM** for simplicity and clarity. For $n = 20$, performance difference is negligible.

## Step-by-Step Execution (n = 10)

Let's trace the algorithm for the example case:

```
Initial: lcm = 1

k=1: lcm(1, 1) = 1
k=2: lcm(1, 2) = 2
k=3: lcm(2, 3) = 6
k=4: lcm(6, 4) = 12   [gcd(6,4)=2, lcm=6*4/2=12]
k=5: lcm(12, 5) = 60  [gcd(12,5)=1, lcm=12*5/1=60]
k=6: lcm(60, 6) = 60  [gcd(60,6)=6, lcm=60*6/6=60]
k=7: lcm(60, 7) = 420 [gcd(60,7)=1, lcm=60*7/1=420]
k=8: lcm(420, 8) = 840 [gcd(420,8)=4, lcm=420*8/4=840]
k=9: lcm(840, 9) = 2520 [gcd(840,9)=3, lcm=840*9/3=2520]
k=10: lcm(2520, 10) = 2520 [gcd(2520,10)=10, lcm=2520*10/10=2520]

Result: 2520  ✓
```

**Observations**:
- LCM grows, but not always (k=6, k=10 don't increase it)
- When $k$ divides current LCM, LCM stays same
- GCD computation prevents overflow in intermediate products

## Verification for n = 20

### Prime Factorization Check

$$232,792,560 = 2^4 \times 3^2 \times 5 \times 7 \times 11 \times 13 \times 17 \times 19$$

**Verification**:
```rust
16 * 9 * 5 * 7 * 11 * 13 * 17 * 19 = 232,792,560  ✓
```

### Divisibility Check

Must be divisible by all numbers 1-20:

```rust
for i in 1..=20 {
    assert_eq!(232792560 % i, 0);  // All pass ✓
}
```

**Key checks**:
- Divisible by 20 = $2^2 \times 5$: Yes ($2^4 \times 5$ in factorization)
- Divisible by 18 = $2 \times 3^2$: Yes ($2^4 \times 3^2$ in factorization)
- Divisible by 16 = $2^4$: Yes ($2^4$ in factorization)

All numbers 1-20 have prime factorizations covered by $2^4 \times 3^2 \times 5 \times 7 \times 11 \times 13 \times 17 \times 19$ ✓

## Euclidean GCD Algorithm Deep Dive

### Why GCD Matters

**Formula**: $\text{lcm}(a, b) = \frac{a \times b}{\gcd(a, b)}$

**Without GCD**: Would need prime factorization for every LCM computation
**With GCD**: Fast $O(\log \min(a, b))$ algorithm available

### Euclidean Algorithm

**Principle**: $\gcd(a, b) = \gcd(b, a \bmod b)$

**Proof sketch**:
- Any divisor of $a$ and $b$ also divides $a - kb$ for any integer $k$
- Choosing $k = \lfloor a/b \rfloor$ gives $a - kb = a \bmod b$
- Therefore, common divisors of $(a, b)$ are same as common divisors of $(b, a \bmod b)$

**Example**: $\gcd(60, 24)$
```
gcd(60, 24)
= gcd(24, 60 mod 24)
= gcd(24, 12)
= gcd(12, 24 mod 12)
= gcd(12, 0)
= 12  ✓
```

**Complexity**: $O(\log \min(a, b))$ steps

**Why $\log$?**: Each step reduces larger number by at least half (amortized)

### Implementation Details

```rust
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;       // Save b before overwriting
        b = a % b;          // New b = remainder
        a = temp;           // New a = old b
    }
    a  // When b=0, a holds the GCD
}
```

**Iterative vs Recursive**:
- **Iterative** (used): $O(1)$ space, faster in practice
- **Recursive**: Cleaner code, $O(\log n)$ stack space

**Choice**: Iterative for efficiency (no function call overhead).

## Performance Analysis

### Time Complexity

**Iterative LCM approach**:
- Outer loop: $O(n)$ iterations (1 to 20)
- Each iteration: 
  - GCD computation: $O(\log M)$ where $M$ is current LCM
  - LCM computation: $O(1)$ (one multiplication, one division)
- **Total**: $O(n \log M)$ ≈ $O(20 \log(232,792,560))$ ≈ $O(20 \times 28)$ ≈ $O(560)$ operations

**Prime factorization approach**:
- Sieve of Eratosthenes: $O(n \log \log n)$ ≈ $O(20 \times 2.5)$ ≈ $O(50)$
- Find highest powers: $O(\pi(n) \log n)$ ≈ $O(8 \times 4.3)$ ≈ $O(34)$
- Multiply powers: $O(\pi(n))$ ≈ $O(8)$
- **Total**: $O(92)$ operations

**Winner**: Prime factorization is faster asymptotically, but for $n = 20$, both are instant (<1μs).

### Space Complexity

**Iterative LCM**: $O(1)$ - only a few variables

**Prime factorization**: $O(n)$ - sieve array

**Winner**: Iterative LCM for space efficiency.

### Practical Performance

**Benchmark** (estimated):
- Iterative LCM: ~200 ns
- Prime factorization: ~500 ns (sieve overhead)

**Conclusion**: For small $n$ (≤ 100), iterative LCM is simpler and equally fast.

## Learning Insights

### 1. Functional Programming in Rust

**Iterator fold pattern**:
```rust
(1..=n).fold(1, lcm)
```

Elegant, functional, performant. Rust's iterators are zero-cost abstractions!

**Equivalent imperative code**:
```rust
let mut result = 1;
for k in 1..=n {
    result = lcm(result, k);
}
result
```

**Preference**: `fold` is more expressive and idiomatic Rust.

### 2. Mathematical Elegance

**Problem reduces to**: Apply LCM repeatedly using associativity.

**Key insight**: Don't need to understand deep number theory - just use GCD/LCM definitions!

**Lesson**: Sometimes simple algorithms beat complex optimizations.

### 3. GCD is Fundamental

GCD appears everywhere in number theory:
- Computing LCM
- Simplifying fractions
- Modular inverses (Extended Euclidean Algorithm)
- Cryptography (RSA key generation)

**Investment**: Understanding Euclidean algorithm pays dividends across many problems.

### 4. Test-Driven Validation

**Testing strategy**:
1. Test GCD with known examples
2. Test LCM with known examples
3. Validate example case (n=10, result=2520)
4. Verify divisibility for solution (n=20)
5. Check edge cases (n=1, n=2, etc.)

**Result**: High confidence before running on actual problem.

## Comparison: Iterative vs Prime Factorization

| **Aspect** | **Iterative LCM** | **Prime Factorization** |
|------------|-------------------|-------------------------|
| **Complexity** | $O(n \log M)$ | $O(n \log \log n)$ |
| **Space** | $O(1)$ | $O(n)$ |
| **Code** | 10 lines | 30+ lines |
| **Clarity** | Very clear | Requires understanding primes |
| **Generality** | Any sequence | Best for consecutive integers |
| **Performance (n=20)** | ~200 ns | ~500 ns |
| **Performance (n=1000)** | ~10 μs | ~5 μs |

**Verdict**: Iterative LCM wins for simplicity and small $n$. Prime factorization wins for large $n$.

## Related Problems

### Project Euler Extensions

- **Problem 9**: Pythagorean triples (uses GCD for primitive triples)
- **Problem 12**: Highly divisible triangular numbers (divisor counting)
- **Problem 21**: Amicable numbers (uses divisor sums)

### Number Theory Applications

- **Modular arithmetic**: GCD for modular inverses
- **Cryptography**: RSA uses GCD to verify coprimality
- **Fraction arithmetic**: LCM for common denominators

## Implementation Details

**File**: `project_euler/src/problems/p005.rs`

**Public API**:
```rust
pub fn smallest_multiple(n: u64) -> u64;
pub fn solve() -> u64;  // Solves for n=20
```

**Helper functions**:
```rust
fn gcd(a: u64, b: u64) -> u64;
fn lcm(a: u64, b: u64) -> u64;
```

**Tests**: 7 unit tests covering GCD, LCM, examples, and solution

**Alternative implementation available**: `src/utils/number_theory.rs` has reusable GCD/LCM

## Zettelkasten Connections

- [[lcm-gcd-euclidean]] - Comprehensive GCD/LCM theory
- [[prime-factorization]] - Alternative approach
- [[euclidean-algorithm]] - Deep dive into GCD algorithm
- [[project-euler-p009]] - Future: GCD in Pythagorean triples
- [[2026-01-27]] - Daily note for this session

## References

- **Project Euler Problem 5**: https://projecteuler.net/problem=5
- **Euclidean Algorithm**: [[lcm-gcd-euclidean]]
- **Implementation**: `project_euler/src/problems/p005.rs`
- *Concrete Mathematics* - Chapter 4: Number Theory

---

*Links:*
- **Foundation**: [[lcm-gcd-euclidean]], [[prime-factorization]]
- **Techniques**: [[euclidean-algorithm]], [[fold-pattern]]
- **Related Problems**: [[project-euler-p009]], [[project-euler-p012]]
- **Session**: [[2026-01-27]]

*Tags:* #project-euler #lcm #gcd #euclidean-algorithm #number-theory #functional-programming
