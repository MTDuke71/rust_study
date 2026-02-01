# Divisor Function τ(n)

**Category**: Number Theory, Arithmetic Functions  
**Difficulty**: Fundamental to Intermediate

## Definition

The **divisor function** τ(n) (also written d(n) or σ₀(n)) counts the number of positive divisors of a positive integer n.

**Notation**: τ(n) = |{d ∈ ℕ : d | n}|

**Read as**: "tau of n equals the number of positive integers that divide n"

## Examples

| n | Divisors | τ(n) |
|---|----------|------|
| 1 | {1} | 1 |
| 2 | {1, 2} | 2 |
| 6 | {1, 2, 3, 6} | 4 |
| 12 | {1, 2, 3, 4, 6, 12} | 6 |
| 28 | {1, 2, 4, 7, 14, 28} | 6 |
| 36 | {1, 2, 3, 4, 6, 9, 12, 18, 36} | 9 |

## The Divisor Formula

### Main Theorem

**Theorem**: If n has prime factorization n = p₁^a₁ × p₂^a₂ × ... × pₖ^aₖ, then:

$$\tau(n) = (a_1 + 1)(a_2 + 1) \cdots (a_k + 1)$$

### Why It Works

**Conceptual Explanation**:

Every divisor d of n must have the form:
$$d = p_1^{e_1} \times p_2^{e_2} \times \cdots \times p_k^{e_k}$$

where $0 \leq e_i \leq a_i$ for each prime $p_i$.

**Counting divisors = counting exponent combinations**:
- For prime $p_1$: choose exponent from {0, 1, 2, ..., $a_1$} → $(a_1 + 1)$ choices
- For prime $p_2$: choose exponent from {0, 1, 2, ..., $a_2$} → $(a_2 + 1)$ choices
- ...
- For prime $p_k$: choose exponent from {0, 1, 2, ..., $a_k$} → $(a_k + 1)$ choices

**Total combinations** (by multiplication principle): $(a_1 + 1)(a_2 + 1) \cdots (a_k + 1)$

### Visual Example: τ(12)

**Factorization**: 12 = 2² × 3¹

**Exponent choices**:
- For prime 2: {0, 1, 2} → 3 choices
- For prime 3: {0, 1} → 2 choices

**Grid of divisors**:
```
         3⁰   3¹
    ┌─────┬─────┐
2⁰  │  1  │  3  │
    ├─────┼─────┤
2¹  │  2  │  6  │
    ├─────┼─────┤
2²  │  4  │ 12  │
    └─────┴─────┘

Divisors: {1, 2, 3, 4, 6, 12}
Count: 3 × 2 = 6 ✓
```

**Formula**: τ(12) = (2 + 1)(1 + 1) = 3 × 2 = **6**

### Proof

**Formal proof**:

1. Let $D_n$ = set of divisors of n
2. Every $d \in D_n$ has unique factorization $d = p_1^{e_1} \cdots p_k^{e_k}$ (Fundamental Theorem)
3. For d | n, must have $0 \leq e_i \leq a_i$ for all i
4. Number of valid $(e_1, ..., e_k)$ tuples:
   - $e_1$ has $(a_1 + 1)$ choices
   - $e_2$ has $(a_2 + 1)$ choices
   - ... (independent choices)
   - $e_k$ has $(a_k + 1)$ choices
5. By multiplication principle: Total = $(a_1 + 1) \cdots (a_k + 1)$
6. Each tuple corresponds to unique divisor (by uniqueness of factorization)
7. Therefore: $\tau(n) = (a_1 + 1) \cdots (a_k + 1)$ ∎

## Special Cases

### Prime Numbers

If p is prime: p = p¹

τ(p) = (1 + 1) = **2**

**Divisors**: {1, p}

**Characterization**: n is prime ⟺ τ(n) = 2

### Prime Powers

If n = p^a:

τ(p^a) = (a + 1)

**Examples**:
- τ(2³) = τ(8) = 3 + 1 = 4 → divisors {1, 2, 4, 8}
- τ(3²) = τ(9) = 2 + 1 = 3 → divisors {1, 3, 9}
- τ(5⁴) = τ(625) = 4 + 1 = 5 → divisors {1, 5, 25, 125, 625}

### Perfect Squares

If n = m²:

Each prime appears with **even exponent** $2b_i$

τ(m²) = (2b₁ + 1)(2b₂ + 1)...(2bₖ + 1)

**Result**: τ(m²) is **always odd**

**Why**: Each factor (2bᵢ + 1) is odd

**Example**: 36 = 2² × 3² → τ(36) = (2+1)(2+1) = 3 × 3 = **9** (odd) ✓

### Unity

τ(1) = **1**

**Convention**: 1 has no prime factors (empty product)
- Formula: τ(1) = (empty product) = 1
- Divisors: {1}

## Multiplicativity

### Definition

A function f is **multiplicative** if:

**f(mn) = f(m) × f(n) whenever gcd(m, n) = 1**

### Theorem: τ(n) is Multiplicative

**Proof**:

Let gcd(m, n) = 1, with factorizations:
- m = p₁^a₁ × ... × pᵣ^aᵣ
- n = q₁^b₁ × ... × qₛ^bₛ

Since coprime, {p₁, ..., pᵣ} and {q₁, ..., qₛ} are disjoint.

Then:
- mn = p₁^a₁ × ... × pᵣ^aᵣ × q₁^b₁ × ... × qₛ^bₛ

Apply formula:
$$\tau(mn) = (a_1 + 1) \cdots (a_r + 1) \times (b_1 + 1) \cdots (b_s + 1)$$
$$= [(a_1 + 1) \cdots (a_r + 1)] \times [(b_1 + 1) \cdots (b_s + 1)]$$
$$= \tau(m) \times \tau(n)$$

Therefore τ(n) is multiplicative. ∎

### Application: Coprime Factorization

**Example**: Calculate τ(60)

**Method 1** - Direct:
- 60 = 2² × 3¹ × 5¹
- τ(60) = (2+1)(1+1)(1+1) = 3 × 2 × 2 = **12**

**Method 2** - Coprime split:
- 60 = 4 × 15
- gcd(4, 15) = 1 (coprime!)
- τ(60) = τ(4) × τ(15)
- τ(4) = (2+1) = 3
- τ(15) = (1+1)(1+1) = 4
- τ(60) = 3 × 4 = **12** ✓

**Use case**: When factorization of pieces is easier than whole.

## Computational Methods

### Method 1: Brute Force (Naive)

```rust
fn count_divisors_naive(n: u64) -> u64 {
    (1..=n).filter(|&d| n % d == 0).count() as u64
}
```

**Complexity**: O(n)

**Usable for**: Small n (< 10⁶)

### Method 2: Square Root Optimization

**Key insight**: Divisors come in pairs!

If d | n, then (n/d) | n, and d × (n/d) = n

**Observation**: Either d ≤ √n or n/d ≤ √n (or both if d = √n)

**Algorithm**: Check divisors up to √n only

```rust
fn count_divisors_sqrt(n: u64) -> u64 {
    let mut count = 0;
    let sqrt_n = (n as f64).sqrt() as u64;
    
    for d in 1..=sqrt_n {
        if n % d == 0 {
            count += 1;        // Found divisor d
            if d != n / d {    // Avoid double-counting perfect squares
                count += 1;    // Paired divisor n/d
            }
        }
    }
    count
}
```

**Complexity**: O(√n)

**Usable for**: Medium n (< 10¹²)

**Example**: n = 36
- Check d = 1: 36/1 = 36 → count = 2 (1, 36)
- Check d = 2: 36/2 = 18 → count = 4 (2, 18)
- Check d = 3: 36/3 = 12 → count = 6 (3, 12)
- Check d = 4: 36/4 = 9 → count = 8 (4, 9)
- Check d = 5: 36 % 5 ≠ 0 → skip
- Check d = 6: 36/6 = 6 → count = 9 (6 only once!)
- Stop at √36 = 6
- Result: **9 divisors** ✓

### Method 3: Prime Factorization (Best)

**Algorithm**: Factor n, then apply formula

```rust
fn count_divisors_formula(mut n: u64) -> u64 {
    let mut divisor_count = 1;
    
    // Factor out 2
    if n % 2 == 0 {
        let mut exp = 0;
        while n % 2 == 0 {
            exp += 1;
            n /= 2;
        }
        divisor_count *= exp + 1;
    }
    
    // Odd factors
    let mut factor = 3;
    while factor * factor <= n {
        if n % factor == 0 {
            let mut exp = 0;
            while n % factor == 0 {
                exp += 1;
                n /= factor;
            }
            divisor_count *= exp + 1;
        }
        factor += 2;
    }
    
    // Remaining n is prime with exponent 1
    if n > 1 {
        divisor_count *= 2;  // (1 + 1)
    }
    
    divisor_count
}
```

**Complexity**: O(√n) worst case, but much faster in practice
- n shrinks as factors removed
- Early termination when factor² > n

**Usable for**: Large n (< 10¹⁸)

**Why fastest**: 
- Don't need to find all divisors
- Just count exponents
- n decreases during factorization

## Divisor Bounds

### Lower Bound

**Theorem**: τ(n) ≥ 2 for all n > 1

**Proof**: Every n > 1 has at least divisors {1, n}

**Equality**: τ(n) = 2 ⟺ n is prime

### Upper Bound

**Theorem** (Average order): 

$$\sum_{k=1}^{n} \tau(k) \sim n \log n$$

**Implication**: Average value of τ(k) for k ≤ n is approximately log(n)

**Consequence**: "Most" numbers have ~log(n) divisors

**Exception**: Highly composite numbers can have many more!

### Maximum τ(n) for n ≤ N

**Growth**: τ(n) can be as large as O(n^ε) for any ε > 0

**Highly composite numbers**: Numbers with more divisors than any smaller number
- n = 1: τ = 1
- n = 2: τ = 2
- n = 6: τ = 4
- n = 12: τ = 6
- n = 120: τ = 16
- n = 840: τ = 32
- n = 5,040: τ = 60

## Related Functions

### Divisor Sum Function σ(n)

**Definition**: Sum of all divisors of n

$$\sigma(n) = \sum_{d|n} d$$

**Formula**: If n = p₁^a₁ × ... × pₖ^aₖ, then:

$$\sigma(n) = \prod_{i=1}^{k} \frac{p_i^{a_i + 1} - 1}{p_i - 1}$$

**Example**: σ(12) = 1 + 2 + 3 + 4 + 6 + 12 = 28

**Relation**: τ(n) = σ₀(n), σ(n) = σ₁(n) (generalized divisor functions)

### Generalized Divisor Function σₖ(n)

**Definition**: Sum of k-th powers of divisors

$$\sigma_k(n) = \sum_{d|n} d^k$$

**Special cases**:
- σ₀(n) = τ(n) (count divisors)
- σ₁(n) = σ(n) (sum divisors)

### Möbius Function μ(n)

**Definition**:
- μ(1) = 1
- μ(n) = 0 if n has squared prime factor
- μ(n) = (-1)^k if n is product of k distinct primes

**Möbius Inversion** relates τ and μ:

$$n = \sum_{d|n} \tau(d) \cdot \mu(n/d)$$

## Perfect Numbers Connection

**Definition**: n is **perfect** if σ(n) = 2n (sum of proper divisors equals n)

**Examples**: 6, 28, 496, 8128, ...

**Euclid-Euler Theorem**: Even perfect numbers have form $2^{p-1}(2^p - 1)$ where $2^p - 1$ is prime (Mersenne prime)

**Divisor count**:
- For even perfect n = $2^{p-1}(2^p - 1)$:
- τ(n) = τ($2^{p-1}$) × τ($2^p - 1$) (coprime!)
- τ(n) = p × 2 = **2p**

**Example**: 28 = 2² × 7
- p = 3, so τ(28) = 2 × 3 = **6** ✓

## Highly Composite Numbers

**Definition**: n is **highly composite** if τ(n) > τ(m) for all m < n

**First few**: 1, 2, 4, 6, 12, 24, 36, 48, 60, 120, 180, 240, 360, 720, 840, 1260, ...

**Property**: Highly composite numbers have **small prime factors with high multiplicity**

**Example**: 120 = 2³ × 3 × 5
- τ(120) = (3+1)(1+1)(1+1) = 4 × 2 × 2 = **16**
- No number < 120 has ≥ 16 divisors

**Application**: Search space for Project Euler P12!

## Applications

### Number Theory

1. **Perfect numbers**: σ(n) = 2n characterization
2. **Abundant/deficient numbers**: σ(n) > 2n or σ(n) < 2n
3. **Prime testing**: τ(n) = 2 ⟺ n prime
4. **Factorization**: Knowing τ(n) constrains possible factorizations

### Combinatorics

1. **Lattice paths**: Divisors correspond to lattice points
2. **Partitions**: Related to partition function via generating functions
3. **Symmetries**: Count symmetries of regular polygons

### Competitive Programming

1. **Project Euler**: Many problems involve τ(n) or σ(n)
2. **Codeforces**: Divisor-based DP, number theory problems
3. **Advent of Code**: LCM calculations, period finding

### Cryptography

1. **RSA**: Security relies on difficulty of factoring (computing τ)
2. **Diffie-Hellman**: Uses multiplicative properties
3. **Primality testing**: Efficient τ(n) check

## Computational Complexity

| **Method** | **Time** | **Space** | **Use Case** |
|------------|----------|-----------|--------------|
| Brute force | O(n) | O(1) | n < 10⁶ |
| √n optimization | O(√n) | O(1) | n < 10¹² |
| Prime factorization | O(√n)* | O(1) | General |
| Precomputed sieve | O(log n)† | O(N) | Many queries |

*Actual cost lower due to n shrinking  
†After O(N log log N) preprocessing

## Interesting Properties

### Even vs. Odd τ(n)

**Theorem**: τ(n) is odd ⟺ n is a perfect square

**Proof**:
- n = square ⟺ all exponents in factorization are even
- n = $p_1^{2a_1} \times \cdots \times p_k^{2a_k}$
- τ(n) = $(2a_1 + 1) \times \cdots \times (2a_k + 1)$
- Each factor odd → product odd

**Examples**:
- τ(16) = τ(2⁴) = 5 (odd) ✓ (16 = 4²)
- τ(36) = τ(2² × 3²) = 9 (odd) ✓ (36 = 6²)
- τ(12) = 6 (even) ✓ (12 not square)

### Divisor Density

**Question**: What fraction of integers ≤ N have exactly k divisors?

**Answer** (asymptotic):

$$\frac{|\{n \leq N : \tau(n) = k\}|}{N} \sim \frac{(\log N)^{k-1}}{(k-1)! \cdot N}$$

**Implication**: Numbers with many divisors are rare!

### Dirichlet Series

**Definition**: The Dirichlet series for τ(n):

$$\sum_{n=1}^{\infty} \frac{\tau(n)}{n^s} = \zeta(s)^2$$

where ζ(s) is the Riemann zeta function.

**Connection**: Relates divisor counting to deep analysis!

## Code Examples

### Complete Implementation

```rust
/// Count divisors using prime factorization formula
pub fn tau(mut n: u64) -> u64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    
    let mut count = 1;
    
    // Handle factor 2
    if n % 2 == 0 {
        let mut exp = 0;
        while n % 2 == 0 {
            exp += 1;
            n /= 2;
        }
        count *= exp + 1;
    }
    
    // Handle odd factors
    let mut f = 3;
    while f * f <= n {
        if n % f == 0 {
            let mut exp = 0;
            while n % f == 0 {
                exp += 1;
                n /= f;
            }
            count *= exp + 1;
        }
        f += 2;
    }
    
    // Remaining n is prime
    if n > 1 {
        count *= 2;
    }
    
    count
}

/// List all divisors (for small n)
pub fn divisors(n: u64) -> Vec<u64> {
    let mut divs = Vec::new();
    let sqrt_n = (n as f64).sqrt() as u64;
    
    for d in 1..=sqrt_n {
        if n % d == 0 {
            divs.push(d);
            if d != n / d {
                divs.push(n / d);
            }
        }
    }
    
    divs.sort_unstable();
    divs
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tau_formula() {
        assert_eq!(tau(1), 1);
        assert_eq!(tau(2), 2);
        assert_eq!(tau(6), 4);
        assert_eq!(tau(12), 6);
        assert_eq!(tau(28), 6);
        assert_eq!(tau(36), 9);
    }
    
    #[test]
    fn test_perfect_squares() {
        // Perfect squares have odd number of divisors
        assert_eq!(tau(4), 3);   // Odd
        assert_eq!(tau(9), 3);   // Odd
        assert_eq!(tau(16), 5);  // Odd
        assert_eq!(tau(25), 3);  // Odd
    }
    
    #[test]
    fn test_primes() {
        // Primes have exactly 2 divisors
        assert_eq!(tau(2), 2);
        assert_eq!(tau(3), 2);
        assert_eq!(tau(5), 2);
        assert_eq!(tau(7), 2);
        assert_eq!(tau(11), 2);
    }
}
```

## Historical Context

**Ancient**: Greek mathematicians studied perfect numbers (related to σ(n))

**Euler (1707-1783)**: Proved even perfect number theorem

**Dirichlet (1805-1859)**: Developed analytic number theory, divisor sum formulas

**Ramanujan (1887-1920)**: Studied highly composite numbers, τ(n) properties

**Modern**: Computational number theory, cryptography applications

## References

- *An Introduction to the Theory of Numbers* - Hardy & Wright (Ch. 16)
- *Elementary Number Theory* - Burton (Ch. 5)
- [[project-euler-p012]] - Application to triangle numbers
- [[prime-factorization]] - Foundation for divisor formula
- [[multiplicative-functions]] - Broader function class

---

*Links:*
- **Applications**: [[project-euler-p012]], [[perfect-numbers]], [[highly-composite-numbers]]
- **Theory**: [[prime-factorization]], [[fundamental-theorem-arithmetic]], [[multiplicative-functions]]
- **Related**: [[divisor-sum-function]], [[mobius-function]], [[arithmetic-functions]]
- **Code**: `project_euler/src/problems/p012.rs`

*Tags:* #divisor-function #number-theory #arithmetic-functions #multiplicative #prime-factorization #tau #divisors
