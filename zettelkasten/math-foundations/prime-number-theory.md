# Prime Number Theory and Distribution

**Category**: Number Theory, Prime Numbers, Mathematical Analysis  
**Difficulty**: Intermediate to Advanced

## Definition

**Prime Number Theory** studies the distribution, density, and patterns of prime numbers within the integers. Unlike prime factorization (which decomposes numbers), this theory focuses on **how primes are scattered** throughout the number line and what laws govern their appearance.

## The Prime Counting Function

**Definition**: π(n) = the number of primes less than or equal to n

**Examples**:
- π(10) = 4 (primes: 2, 3, 5, 7)
- π(100) = 25
- π(1,000) = 168
- π(10,000) = 1,229
- π(100,000) = 9,592
- π(1,000,000) = 78,498
- π(2,000,000) = 148,933 (verified in [[project-euler-p010]])

**Key Observation**: Primes become **less dense** as numbers grow larger.

### Prime Density

The **average gap** between consecutive primes near n is approximately **ln(n)**.

| Range | Primes | Density | Avg Gap |
|-------|--------|---------|---------|
| 1-100 | 25 | 25% | ~4 |
| 1-1,000 | 168 | 16.8% | ~6 |
| 1-10,000 | 1,229 | 12.3% | ~8 |
| 1-100,000 | 9,592 | 9.6% | ~10 |
| 1-1,000,000 | 78,498 | 7.8% | ~13 |
| 1-2,000,000 | 148,933 | 7.4% | ~13.4 |

**Pattern**: As n grows by 10×, prime density decreases but doesn't vanish.

## The Prime Number Theorem

### Statement

**Prime Number Theorem (PNT)**: The number of primes less than or equal to n is asymptotically:

$$\pi(n) \sim \frac{n}{\ln(n)}$$

This means:

$$\lim_{n \to \infty} \frac{\pi(n)}{n / \ln(n)} = 1$$

**In words**: The density of primes near n is approximately $\frac{1}{\ln(n)}$.

### Historical Context

- **Conjectured**: Gauss (1792) and Legendre (1798) independently observed the pattern
- **Proven**: Jacques Hadamard and Charles Jean de la Vallée-Poussin (1896) using complex analysis
- **Elementary proof**: Erdős and Selberg (1949) without complex analysis

**Note**: The theorem took **over 100 years** from conjecture to proof—one of the great achievements of 19th-century mathematics.

### Accuracy of the Approximation

For **practical use**, the approximation has known error bounds:

| n | Actual π(n) | n/ln(n) | Error | Percentage Error |
|---|-------------|---------|-------|------------------|
| 10² | 25 | 21.7 | +3.3 | 13% |
| 10³ | 168 | 144.8 | +23.2 | 14% |
| 10⁴ | 1,229 | 1,085.7 | +143.3 | 12% |
| 10⁵ | 9,592 | 8,685.9 | +906.1 | 9.4% |
| 10⁶ | 78,498 | 72,382.4 | +6,115.6 | 7.8% |
| 2×10⁶ | 148,933 | 138,790.7 | +10,142.3 | 6.8% |

**Key Insight**: The approximation **underestimates** π(n) but gets proportionally better as n increases.

### Better Approximations

**Logarithmic Integral** (more accurate):

$$\text{Li}(n) = \int_2^n \frac{1}{\ln(t)} \, dt$$

This gives errors < 0.1% for large n.

**Riemann's Function** (best known):

$$R(n) = \sum_{k=1}^{\infty} \frac{\mu(k)}{k} \text{Li}(n^{1/k})$$

where μ(k) is the Möbius function. This is accurate to within $\sqrt{n} \ln(n)$ (conjectured).

## Implications for Algorithms

### Sieve Memory Estimation

**Question**: How much memory to store all primes up to n?

**Answer**: π(n) ≈ n/ln(n) primes, so:
- For n = 2,000,000: ~148,933 primes
- Storage (64-bit): ~1.19 MB

**Example from P10**:
```rust
// We know π(2,000,000) ≈ 2,000,000 / ln(2,000,000) ≈ 138,791
// Actual: 148,933 primes
// Sieve storage: (2,000,000 - 3) / 2 + 1 = 999,999 bits ≈ 125 KB
```

See [[project-euler-p010]] for implementation.

### Prime Search Strategies

**Finding the nth prime**:

If we want the nth prime, PNT tells us it's approximately at position:

$$p_n \approx n \ln(n)$$

**Example**: The 10,001st prime (from [[project-euler-p007]]):
- Estimate: 10,001 × ln(10,001) ≈ 92,103
- Actual: 104,743
- Error: ~12% (reasonable ballpark!)

**Use case**: Size your sieve to avoid reallocation when searching for nth prime.

### Average Prime Gap

**Theorem**: Average gap between consecutive primes near n is **ln(n)**.

**Consequence**: To find next prime after n, expect to test ~ln(n) candidates.

**Example**: Near 2,000,000:
- Expected gap: ln(2,000,000) ≈ 14.5
- Actual gaps vary (3, 2, 6, 4, 2, 10, ...) but average to ~14.5

## Sum of Primes

### Formula for Sum

**Question**: What is the sum of all primes ≤ n?

While there's no exact closed form, there's an **asymptotic formula**:

$$\sum_{p \leq n} p \sim \frac{n^2}{2 \ln(n)}$$

### Verification with P10

From [[project-euler-p010]]:
- **Actual sum** (n = 2,000,000): 142,913,828,922
- **Prediction**: (2×10⁶)² / (2 × ln(2×10⁶)) = 4×10¹² / (2 × 14.51) ≈ 137.8 billion
- **Error**: 3.5% (excellent for asymptotic formula!)

**Why the formula works**:
- Average prime near n: roughly n/2
- Number of primes: n/ln(n)
- Product: (n/2) × (n/ln(n)) = n²/(2 ln(n))

### Exact Calculation

For exact sums, must compute explicitly:

```rust
pub fn sum_primes_below(limit: u64) -> u64 {
    // Use sieve to generate primes, then sum
    let primes = sieve_of_eratosthenes(limit);
    primes.iter().sum()
}
```

No shortcut—asymptotic formula only gives approximation!

## Prime Gaps

### Definition

**Prime gap** g_n = p_(n+1) - p_n (difference between consecutive primes)

**Examples**:
- Gap after 2: 3 - 2 = 1 (smallest gap)
- Gap after 3: 5 - 3 = 2 (most common small gap)
- Gap after 7: 11 - 7 = 4
- Gap after 89: 97 - 89 = 8
- Gap after 113: 127 - 113 = 14

### Average Gap: ln(n)

**Theorem**: The average gap between primes near n approaches **ln(n)**.

**Proof sketch**:
- Primes near n have density 1/ln(n)
- Reciprocal of density = average spacing = ln(n)

### Bertrand's Postulate

**Theorem** (Bertrand, 1845; proved by Chebyshev, 1850):

> For every integer n > 1, there exists at least one prime p such that n < p < 2n.

**In words**: Between any number and its double, there's always a prime!

**Examples**:
- Between 10 and 20: 11, 13, 17, 19 ✓
- Between 100 and 200: 101, 103, 107, ..., 199 ✓
- Between 1,000,000 and 2,000,000: guaranteed to exist!

**Implication**: Prime gaps are **bounded** relative to position—can't have arbitrarily large deserts of composites.

### Largest Known Gaps

Despite Bertrand's guarantee, gaps can be **large** in absolute terms:

| First Prime | Next Prime | Gap | Discovered |
|-------------|------------|-----|------------|
| 1,693 | 1,697 | 4 | - |
| 31,397 | 31,469 | 72 | - |
| 1,425,172,824,437,699,411 | (next) | 1,476 | 2017 |

**Cramer's Conjecture**: Maximum gap near n is O((ln n)²).

### Prime Gaps in Sieve Implementation

When generating primes with [[sieve-of-eratosthenes]], gaps inform:
- **Memory layout**: Can use bit arrays (skip evens, only 1 bit per odd number)
- **Next prime search**: From n, test n+2, n+4, ... expecting gap ~ln(n)

## Twin Primes and Prime Constellations

### Twin Primes

**Definition**: Pairs of primes differing by 2: (p, p+2)

**Examples**: (3,5), (5,7), (11,13), (17,19), (29,31), (41,43), ...

**Twin Prime Conjecture** (unsolved!): There are **infinitely many** twin primes.

**Evidence**:
- Largest known twin primes: 2,996,863,034,895 × 2^(1,290,000) ± 1 (388,342 digits!)
- Heuristically, density suggests infinite count
- But **no proof yet** (unlike general prime infinitude proven by Euclid)

### Other Prime Patterns

**Cousin primes** (gap 4): (3,7), (7,11), (13,17), ...  
**Sexy primes** (gap 6): (5,11), (7,13), (11,17), ...  
**Prime triplets** (p, p+2, p+6): (5,7,11), (11,13,17), (17,19,23), ...

### Relevance to Project Euler

Some problems explore:
- Identifying prime patterns (consecutive primes with specific properties)
- Counting primes in arithmetic progressions
- Finding prime-generating formulas

## The Riemann Hypothesis

### Statement

**Riemann Hypothesis** (1859, unsolved):

> All non-trivial zeros of the Riemann zeta function ζ(s) have real part 1/2.

**Why it matters**: RH implies the **error term** in the Prime Number Theorem is:

$$\pi(n) = \text{Li}(n) + O(\sqrt{n} \ln(n))$$

This gives extremely tight bounds on prime distribution!

### Zeta Function Connection

**Riemann zeta function**:

$$\zeta(s) = \sum_{n=1}^{\infty} \frac{1}{n^s} = \prod_{p \text{ prime}} \frac{1}{1 - p^{-s}}$$

The product formula (Euler, 1737) directly connects ζ(s) to primes—the zeta function **encodes prime distribution**!

### Practical Impact

**If RH is true**:
- Prime gap bounds become much tighter
- Primality testing algorithms can be optimized
- Cryptographic estimates more reliable

**Current status**: 
- Verified computationally for billions of zeros
- Clay Millennium Prize ($1,000,000) for proof or disproof

## Infinitude of Primes

### Euclid's Proof (300 BCE)

**Theorem**: There are infinitely many primes.

**Proof**:
1. Suppose finitely many primes: p₁, p₂, ..., pₙ
2. Consider N = (p₁ × p₂ × ... × pₙ) + 1
3. N is not divisible by any pᵢ (remainder 1)
4. Either N is prime, or has a prime factor not in {p₁, ..., pₙ}
5. Contradiction! ∴ Infinitely many primes

**Elegance**: One of the most beautiful proofs in mathematics.

### Other Proofs

**Euler's proof** (1737): Using ζ(s), showed:

$$\sum_{p \text{ prime}} \frac{1}{p} = \infty$$

If only finitely many primes, sum would be finite—contradiction!

**Erdős's proof** (1938): Uses Fermat numbers 2^(2^n) + 1—infinitely many are pairwise coprime.

## Primes in Arithmetic Progressions

### Dirichlet's Theorem

**Theorem** (Dirichlet, 1837):

> If gcd(a, d) = 1, then the arithmetic progression a, a+d, a+2d, a+3d, ... contains infinitely many primes.

**Examples**:
- **4k+1**: 5, 13, 17, 29, 37, 41, 53, ... (infinite!)
- **4k+3**: 3, 7, 11, 19, 23, 31, 43, ... (infinite!)
- **6k+1**: 7, 13, 19, 31, 37, 43, ... (infinite!)
- **6k-1**: 5, 11, 17, 23, 29, 41, ... (infinite!)

**Connection to 6k±1 pattern**: All primes > 3 are in 6k±1 progressions (see [[prime-factorization]]).

### Green-Tao Theorem

**Theorem** (2004):

> The primes contain arbitrarily long arithmetic progressions.

**Examples**:
- Length 3: 3, 7, 11 (gap 4)
- Length 5: 5, 11, 17, 23, 29 (gap 6)
- Length 26: Starting at 43,142,746,595,714,191 (gap 5,283,234,035,979,900)

**Implication**: No matter how long, you can find primes in arithmetic progression!

## Prime-Generating Functions

### Polynomial Impossibility

**Theorem** (No polynomial generates only primes):

No polynomial f(n) with integer coefficients produces only primes for all n.

**Proof sketch**: If f(n) is prime for all n, then f(n) and f(n + f(0)) have common factor f(0), but f(n + f(0)) ≠ f(0) for degree ≥ 1—contradiction.

### Near Misses

Some polynomials generate many primes:

**Euler's polynomial**: n² - n + 41
- Generates primes for n = 0, 1, 2, ..., 40 (40 consecutive!)
- Fails at n = 41: 41² - 41 + 41 = 41²

**Legendre's polynomial**: n² - 79n + 1,601
- Generates primes for n = 0, 1, ..., 79

### Mersenne Primes

**Form**: M_p = 2^p - 1 where p is prime

**Examples**:
- M₂ = 3 ✓
- M₃ = 7 ✓
- M₅ = 31 ✓
- M₇ = 127 ✓
- M₁₁ = 2,047 = 23 × 89 (composite!)

**Note**: Not all M_p are prime, but all Mersenne primes have this form.

**Computational significance**: Mersenne primes enable division-free modular arithmetic — see [[math-foundations/mersenne-primes-fast-arithmetic]].

**Largest known prime** (as of 2026): 2^(82,589,933) - 1 (24,862,048 digits!)

### Fermat Primes

**Form**: F_n = 2^(2^n) + 1

**Examples**:
- F₀ = 3 ✓
- F₁ = 5 ✓
- F₂ = 17 ✓
- F₃ = 257 ✓
- F₄ = 65,537 ✓
- F₅ = 4,294,967,297 = 641 × 6,700,417 (composite!)

**Conjecture**: Only F₀ through F₄ are prime (no proof either way!).

## Applications in Computer Science

### Cryptography

**RSA**: Security relies on difficulty of factoring products of two large primes
- Prime selection: Use PNT to estimate search space
- Primality testing: Miller-Rabin, AKS algorithms

**Diffie-Hellman**: Uses safe primes p where (p-1)/2 is also prime

### Hash Tables

**Prime-sized tables**: Reduces collisions
- Choose table size as prime near desired capacity
- PNT helps estimate next prime

### Randomization

**Modular arithmetic**: Prime moduli ensure maximum period
- Linear congruential generators use prime moduli
- Polynomial hashing uses prime bases

### Distributed Systems

**Consistent hashing**: Prime numbers reduce collision in ring partitioning

## Relevance to Project Euler

### Problems Using Prime Theory

- **P7**: Find 10,001st prime → Use PNT to estimate sieve size
- **P10**: Sum of primes below 2M → Verify using sum formula
- **P27**: Prime-generating quadratics → Related to Euler's polynomial
- **P35**: Circular primes → Understand prime density
- **P37**: Truncatable primes → Prime pattern analysis
- **P50**: Consecutive prime sum → Requires understanding prime distribution
- **P58**: Spiral primes → Prime density in sequences
- **P60**: Prime pair sets → Related to twin/cousin primes

### Common Patterns

1. **Estimating sieve size**: Use n ln(n) for nth prime
2. **Counting primes**: Use π(n) ≈ n/ln(n)
3. **Memory allocation**: π(n) tells how many primes to store
4. **Search bounds**: Average gap ln(n) informs iteration limits

## Comparison: Theory vs. Algorithms

| **Concept** | **Theory** | **Algorithm** | **Page** |
|-------------|-----------|---------------|----------|
| Breaking numbers | Fundamental Theorem | Trial division | [[prime-factorization]] |
| Counting primes | Prime Number Theorem | Sieve of Eratosthenes | This page + [[project-euler-p010]] |
| Finding nth prime | p_n ≈ n ln(n) | Generate + count | [[project-euler-p007]] |
| Prime testing | Probabilistic bounds | Miller-Rabin | Future |
| Prime gaps | Average ln(n) | Explicit computation | This page |

## Historical Timeline

- **300 BCE**: Euclid proves infinitude of primes
- **~200 BCE**: Eratosthenes invents sieve algorithm
- **1792**: Gauss conjectures Prime Number Theorem
- **1837**: Dirichlet proves primes in arithmetic progressions
- **1859**: Riemann formulates hypothesis on zeta function
- **1896**: PNT proved (Hadamard & Vallée-Poussin)
- **1949**: Elementary proof of PNT (Erdős & Selberg)
- **2004**: Green-Tao prove arbitrary-length arithmetic progressions
- **2013**: Bounded gaps (Yitang Zhang) → Twin prime conjecture progress

## Code Applications

### Estimating Sieve Size for nth Prime

```rust
/// Estimate upper bound for nth prime using PNT
pub fn estimate_nth_prime_upper_bound(n: u64) -> u64 {
    if n < 6 {
        return 15;  // Small cases
    }
    
    let n_f64 = n as f64;
    let ln_n = n_f64.ln();
    
    // p_n < n * (ln(n) + ln(ln(n))) for n >= 6 (Rosser's theorem)
    let estimate = n_f64 * (ln_n + ln_n.ln());
    estimate.ceil() as u64
}
```

**Usage in P7**:
```rust
let upper_bound = estimate_nth_prime_upper_bound(10_001);
let primes = sieve_of_eratosthenes(upper_bound);
let answer = primes[10_000];  // 0-indexed, so 10,001st prime
```

### Verifying Prime Count

```rust
/// Approximate prime count using PNT
pub fn estimate_prime_count(n: u64) -> u64 {
    if n < 2 {
        return 0;
    }
    
    let n_f64 = n as f64;
    let estimate = n_f64 / n_f64.ln();
    estimate.round() as u64
}

#[test]
fn test_pnt_accuracy() {
    assert_eq!(estimate_prime_count(2_000_000), 138_791);  // Actual: 148,933
    // ~6.8% error - typical for PNT
}
```

## Deep Insights

### Why Primes Become Rare

**Intuition**: As numbers grow, they have more opportunities to be composite.

**Formal**: A number n has ~log(n) potential prime factors up to √n. Probability of avoiding all factors decreases as 1/ln(n).

### Prime Gaps Grow, But Slowly

**Key tension**:
- Bertrand: Always a prime within next doubling (gaps ≤ n)
- Cramér: Gaps grow like (ln n)² (much slower than n)
- Reality: Primes thin out, but never disappear

### Sum vs. Count

**Surprising fact**: Sum of primes grows faster than count!
- Count: π(n) ~ n/ln(n) (linear with logarithmic correction)
- Sum: ~ n²/(2 ln(n)) (**quadratic** with logarithmic correction)

**Why**: Larger primes contribute more to sum than to count.

## References

- *An Introduction to the Theory of Numbers* - Hardy & Wright (Ch. 1-3, 22)
- *The Music of the Primes* - Marcus du Sautoy (popular exposition)
- *Prime Obsession* - John Derbyshire (Riemann Hypothesis history)
- [[project-euler-p007]] - Finding nth prime
- [[project-euler-p010]] - Sum of primes with PNT verification
- [[prime-factorization]] - Complementary focus on decomposition
- [[sieve-of-eratosthenes]] - Prime generation algorithm

---

*Links:*
- **Related Theory**: [[prime-factorization]], [[divisibility]], [[number-theory-fundamentals]]
- **Algorithms**: [[sieve-of-eratosthenes]], [[primality-fundamentals]]
- **Applications**: [[project-euler-p007]], [[project-euler-p010]]
- **Advanced**: [[riemann-hypothesis]], [[twin-prime-conjecture]]
- **Code**: `project_euler/src/problems/p007.rs`, `project_euler/src/problems/p010.rs`

*Tags:* #prime-numbers #number-theory #prime-number-theorem #distribution #mathematics #theory #primes-analysis
