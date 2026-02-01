# Prime Factorization

**Category**: Number Theory, Prime Numbers, Algorithms  
**Difficulty**: Fundamental

## Definition

**Prime factorization** is the process of expressing a positive integer as a product of prime numbers.

**Fundamental Theorem of Arithmetic**: Every integer $n > 1$ has a **unique** prime factorization (up to order of factors):

$$n = p_1^{a_1} \times p_2^{a_2} \times \cdots \times p_k^{a_k}$$

where $p_1 < p_2 < \cdots < p_k$ are distinct primes and $a_i > 0$ are positive integers.

## Examples

- $12 = 2^2 \times 3$
- $60 = 2^2 \times 3 \times 5$
- $600,851,475,143 = 71 \times 839 \times 1,471 \times 6,857$
- $1024 = 2^{10}$ (power of 2)
- $13 = 13$ (prime numbers are their own factorization)

## Uniqueness

The Fundamental Theorem guarantees **uniqueness**:
- $12 = 2^2 \times 3 = 3 \times 2^2$ (order doesn't matter)
- But there is **no other way** to write 12 as a product of primes
- This property is fundamental to number theory and cryptography

## Algorithms

### 1. Trial Division (Basic)

**Idea**: Test divisibility by all integers from 2 to $\sqrt{n}$

```rust
pub fn prime_factors_naive(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    
    for d in 2..=(n as f64).sqrt() as u64 {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
    }
    
    if n > 1 {
        factors.push(n);  // Remaining n is prime
    }
    
    factors
}
```

**Complexity**: 
- Time: O(√n) worst case (when n is prime)
- Space: O(k) where k = number of prime factors

**Issues**: Slow for large n, tests many composite numbers

### 2. Trial Division with Elimination (Optimized) ✓

**Key optimization**: Divide out each factor **completely** before continuing

```rust
pub fn largest_prime_factor(mut n: u64) -> u64 {
    let mut last_factor = 1;
    
    // Remove factor 2
    while n % 2 == 0 {
        last_factor = 2;
        n /= 2;
    }
    
    // Try odd factors only
    let mut f = 3;
    while f * f <= n {  // Avoid sqrt() - use f*f instead!
        if n % f == 0 {
            last_factor = f;
            while n % f == 0 {
                n /= f;  // Divide out completely
            }
        }
        f += 2;
    }
    
    // Remainder is prime (guaranteed by √n bound)
    if n > 1 { n } else { last_factor }
}
```

**Optimizations**:
1. Test 2 separately, then only odd numbers (skip 50% of candidates)
2. Use `f * f <= n` instead of `f <= sqrt(n)` (no floating-point!)
3. Divide out each factor completely (n shrinks, loop terminates early)
4. If n > 1 after loop, it's automatically prime (no primality test needed!)

**Complexity**:
- Time: O(√n) worst case, **O(log n) average** (most numbers have small factors)
- Space: O(1)

See [[project-euler-p003]] for detailed analysis.

### 3. Wheel Factorization

Skip numbers divisible by small primes (2, 3, 5):
```
Test: 2, 3, 5, then {7, 11, 13, 17, 19, 23, 29, 31, 37, ...}
Skip: All multiples of 2, 3, 5
```

**Pattern** (wheel of 30): Test 1, 7, 11, 13, 17, 19, 23, 29 (mod 30)

**Speedup**: ~3× fewer candidates than testing only odd numbers

### 4. Pollard's Rho Algorithm

**For large numbers** with unknown factorization:
- Probabilistic algorithm
- Finds factors in O(n^(1/4)) expected time
- Uses "random walk" to detect cycles

**Used in**: Cryptanalysis, factoring large semiprimes

### 5. Sieve of Eratosthenes (Preprocessing)

**When factoring many numbers**:
1. Generate all primes up to N using sieve
2. Use those primes for trial division

**Complexity**:
- Preprocessing: O(N log log N)
- Per factorization: O(π(N)) where π(N) = number of primes ≤ N

**Trade-off**: Fast factorization but high memory (store all primes)

## Prime Form: 6k±1 Pattern

### Why All Primes > 3 Have Form 6k±1

**Observation**: Every integer can be written as one of: $6k$, $6k+1$, $6k+2$, $6k+3$, $6k+4$, or $6k+5$

**Analysis of each form**:

| **Form** | **Divisibility** | **Can be Prime?** |
|----------|------------------|-------------------|
| $6k$ | Divisible by 2 and 3 | No (composite) |
| $6k+1$ | Not necessarily divisible by 2 or 3 | **Yes** ✓ |
| $6k+2 = 2(3k+1)$ | Divisible by 2 | No (even) |
| $6k+3 = 3(2k+1)$ | Divisible by 3 | No (composite) |
| $6k+4 = 2(3k+2)$ | Divisible by 2 | No (even) |
| $6k+5 = 6(k+1)-1$ | Not necessarily divisible by 2 or 3 | **Yes** ✓ |

**Conclusion**: Only $6k+1$ and $6k-1$ (which is $6k+5$) can be prime for $k \geq 1$.

**Exception**: The primes 2 and 3 themselves don't fit this pattern (they are the basis for it).

### Examples

**Primes in 6k±1 form**:
- $5 = 6(1) - 1$ ✓
- $7 = 6(1) + 1$ ✓
- $11 = 6(2) - 1$ ✓
- $13 = 6(2) + 1$ ✓
- $17 = 6(3) - 1$ ✓
- $19 = 6(3) + 1$ ✓
- $23 = 6(4) - 1$ ✓

**Not all 6k±1 are prime** (necessary but not sufficient):
- $25 = 6(4) + 1$ but $25 = 5^2$ (composite)
- $35 = 6(6) - 1$ but $35 = 5 \times 7$ (composite)
- $49 = 6(8) + 1$ but $49 = 7^2$ (composite)

### Using 6k±1 for Optimization

**Improved trial division**:
```rust
pub fn prime_factors_6k_optimized(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    
    // Check 2 and 3 separately
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    while n % 3 == 0 {
        factors.push(3);
        n /= 3;
    }
    
    // Now only test 6k±1 candidates
    let mut k = 1u64;
    while (6*k - 1) * (6*k - 1) <= n {
        // Test 6k-1
        let candidate = 6*k - 1;
        while n % candidate == 0 {
            factors.push(candidate);
            n /= candidate;
        }
        
        // Test 6k+1
        let candidate = 6*k + 1;
        while n % candidate == 0 {
            factors.push(candidate);
            n /= candidate;
        }
        
        k += 1;
    }
    
    if n > 1 {
        factors.push(n);
    }
    
    factors
}
```

**Speedup**: Tests only ~33% of odd candidates (vs 50% when testing all odd numbers)
- Testing odd only: Skip 50% (all evens)
- Testing 6k±1: Skip 66% (all evens + multiples of 3)

**Comparison**:
- **Range 1-30**: Test all → 30 candidates
- **Skip evens**: 2, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29 → 15 candidates
- **6k±1 only**: 5, 7, 11, 13, 17, 19, 23, 25, 29 → 9 candidates (3× fewer than naive!)

### Why This Matters

**Theoretical**: Understanding prime structure
- Primes cluster around 6k±1 forms
- Related to [[prime-number-theorem]] and prime distribution

**Practical**: Algorithm optimization
- Further reduces trial division candidates
- Foundation for **wheel factorization** (using larger moduli)

**Connection to cryptography**:
- Large primes for RSA generated by testing 6k±1 candidates
- Reduces primality testing by 66%

## Key Theorems

### Square Root Bound

**Theorem**: If $n$ is composite, then $n$ has a prime factor $p \leq \sqrt{n}$.

**Proof**: 
- If $n = a \times b$ where $1 < a \leq b < n$
- Then $a \leq \sqrt{n}$ (otherwise $a \times b > n$)

**Consequence**: Only need to test factors up to $\sqrt{n}$!

### Remainder After √n is Prime

**Theorem**: After dividing out all factors up to $\sqrt{n}$, any remainder $r > 1$ is prime.

**Proof**: See [[project-euler-p003]] "Why Remainder is Prime" section.

**Impact**: No primality test needed! The algorithm itself proves remainder is prime.

## Applications

### In Cryptography
- **RSA encryption**: Based on difficulty of factoring large semiprimes
- **Key generation**: Product of two large primes
- **Security**: 2048-bit RSA ≈ 600-digit number (infeasible to factor)

### In Number Theory
- **GCD/LCM**: Computed from prime factorizations
  - $\gcd(a, b)$ = product of common prime factors (minimum exponents)
  - $\text{lcm}(a, b)$ = product of all prime factors (maximum exponents)
- **Divisor counting**: $\tau(n) = (a_1 + 1)(a_2 + 1) \cdots (a_k + 1)$
- **Perfect numbers**: Related to Mersenne primes $2^p - 1$

### In Algorithms
- **Integer optimization**: Reduce problems modulo prime factors
- **Chinese Remainder Theorem**: Solve congruences using factorization
- **Dynamic programming**: Factorization-based state representation

### In Competitive Programming
- **Advent of Code**: LCM calculations (Day 8, 2023)
- **Project Euler**: Many problems require efficient factorization
- **Codeforces**: Divisibility, GCD/LCM, prime counting

## Complexity Comparison

| **Algorithm** | **Time** | **Space** | **Use Case** |
|---------------|----------|-----------|--------------|
| Trial division (naive) | O(n) | O(1) | Small n < 10^6 |
| Trial division (√n) | O(√n) | O(1) | Medium n < 10^12 |
| Trial division (optimized) | O(√n) worst, O(log n) avg | O(1) | General purpose ✓ |
| Wheel factorization | O(√n / log n) | O(1) | Slight speedup |
| Sieve preprocessing | O(N log log N) + O(π(√n)) | O(N) | Many factorizations |
| Pollard's Rho | O(n^(1/4)) expected | O(1) | Large semiprimes |
| General Number Field Sieve | O(exp(c (log n)^(1/3))) | High | RSA-size numbers (research) |

## Why Factorization is Hard

**For most integers**: Factorization is easy (small prime factors exist)

**For semiprimes** (product of two large primes):
- No known polynomial-time algorithm
- Best classical: Exponential in log(n)
- Quantum: Shor's algorithm is polynomial (but requires quantum computer)

**Cryptographic security** relies on this hardness!

## Related Concepts

- **[[prime-number-theory]]** - Prime distribution and density (complementary focus)
- **[[trial-division]]** - Primary factorization algorithm
- **[[divisibility]]** - Fundamental property used in factorization
- **[[sqrt-bound-theorem]]** - Key optimization
- **[[primality-testing]]** - Related but distinct problem
- **[[sieve-of-eratosthenes]]** - Generate primes for factorization
- **[[gcd-lcm]]** - Computed using factorizations

## Project Euler Problems

- [[project-euler-p003]] - Largest prime factor of 600,851,475,143
  - Demonstrates trial division with elimination
  - Shows why remainder after √n divisions is prime
  - Optimizations: f*f vs sqrt(n), odd-only testing

## Code Examples

See `project_euler/src/problems/p003.rs` for production-quality implementation.

### Complete Factorization
```rust
pub fn prime_factorization(mut n: u64) -> Vec<(u64, usize)> {
    let mut factors = Vec::new();
    
    // Factor of 2
    if n % 2 == 0 {
        let mut count = 0;
        while n % 2 == 0 {
            count += 1;
            n /= 2;
        }
        factors.push((2, count));
    }
    
    // Odd factors
    let mut f = 3;
    while f * f <= n {
        if n % f == 0 {
            let mut count = 0;
            while n % f == 0 {
                count += 1;
                n /= f;
            }
            factors.push((f, count));
        }
        f += 2;
    }
    
    if n > 1 {
        factors.push((n, 1));
    }
    
    factors  // Vec of (prime, exponent) pairs
}
```

**Example output**: `prime_factorization(60)` → `[(2, 2), (3, 1), (5, 1)]` representing $2^2 \times 3 \times 5$

## Historical Note

The **Fundamental Theorem of Arithmetic** was first proven rigorously by **Carl Friedrich Gauss** in his *Disquisitiones Arithmeticae* (1801), though it was understood intuitively much earlier.

Factorization has been studied for millennia:
- Ancient Greeks: Euclid's algorithm for GCD (related to factorization)
- 1600s: Fermat developed factorization methods
- 1970s: RSA encryption made factorization security-critical
- 1994: Shor's algorithm (quantum factorization) discovered

## References

- *Number Theory* by Hardy & Wright - Chapters 1-2
- *The Art of Computer Programming Vol. 2* - Knuth (Section 4.5.4)
- [[project-euler-p003]] - Application to largest prime factor
- [[trial-division]] - Algorithm details
- [[sqrt-bound-theorem]] - Mathematical foundation

---

*Links:*
- **Applications**: [[project-euler-p003]], [[project-euler-p012]], [[gcd-lcm]], [[divisor-function]]
- **Algorithms**: [[trial-division]], [[sieve-of-eratosthenes]]
- **Theory**: [[sqrt-bound-theorem]], [[divisibility]], [[primality-testing]], [[prime-number-theory]]
- **Code**: `project_euler/src/problems/p003.rs`, `project_euler/src/problems/p012.rs`
- **Tags**: #prime-numbers #factorization #number-theory #algorithms #cryptography
