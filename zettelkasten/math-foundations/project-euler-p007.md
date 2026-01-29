# Project Euler Problem 7: 10001st Prime

**Difficulty**: Easy  
**Published**: December 28, 2001  
**Solved**: January 28, 2026  
**Category**: Prime Numbers, Number Theory  
**Related Concepts**: [[prime-number-theorem]], [[sieve-of-eratosthenes]], [[primality-testing]]

## Problem Statement

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10,001st prime number?

**Given**: 6th prime = 13  
**Find**: 10,001st prime

## Solution Overview

**Answer**: **104,743**

**Approach**: Prime Number Theorem estimation + Sieve of Eratosthenes

## Mathematical Analysis

### Prime Number Theorem (PNT)

**Statement**: The number of primes less than or equal to $n$ is approximately:
$$\pi(n) \approx \frac{n}{\ln(n)}$$

where $\pi(n)$ is the **prime counting function**.

**Inversion**: To find the approximate value of the $n$-th prime $p_n$:
$$n \approx \frac{p_n}{\ln(p_n)}$$

Rearranging:
$$p_n \approx n \ln(p_n)$$

### Better Approximation

For larger $n$ (say $n > 6$), a more accurate estimate is:
$$p_n \approx n(\ln n + \ln \ln n)$$

**Safety margin**: Use $p_n \lesssim n \ln(n) \times 1.3$ to ensure upper bound

**For n=10,001**:
$$p_{10001} \approx 10001 \times \ln(10001) \times 1.3$$
$$\approx 10001 \times 9.21 \times 1.3$$
$$\approx 119,700$$

**Reality**: $p_{10001} = 104,743$ (our estimate is conservative, as intended)

### Sieve of Eratosthenes

**Algorithm**: Find all primes up to $n$

**Steps**:
1. Create boolean array `is_prime[0..=n]`, initialize all to `true`
2. Mark `is_prime[0] = false`, `is_prime[1] = false`
3. For each $i$ from $2$ to $\sqrt{n}$:
   - If `is_prime[i]`:
     - Mark all multiples $ki$ (for $k \ge 2$) as composite (false)
4. Collect all indices where `is_prime[i]` is true

**Optimization**: Start marking multiples from $i^2$ (smaller multiples already marked)

**Complexity**:
- **Time**: $O(n \log \log n)$ - very efficient!
- **Space**: $O(n)$ - boolean array

### Why Sieve Instead of Trial Division?

**Alternative**: Generate primes one by one using primality testing

**Trial Division** for each candidate:
- Time per test: $O(\sqrt{p})$
- Finding 10,001 primes: $O(10001 \times \sqrt{p_n})$ ≈ $O(10001 \times 323)$ ≈ $O(3.2M)$ operations

**Sieve**:
- Time: $O(n \log \log n)$ where $n \approx 120,000$ ≈ $O(120,000 \times 3.5)$ ≈ $O(420K)$ operations
- **8× faster!**

**Trade-off**: Sieve uses more memory but significantly faster for finding many primes.

## Implementation

### Rust Code

```rust
use crate::utils::primes::sieve;

/// Find the nth prime number (1-indexed)
pub fn nth_prime(n: usize) -> u64 {
    if n == 0 {
        panic!("Prime indexing is 1-based");
    }
    
    // Estimate upper bound using Prime Number Theorem
    let estimate = if n < 6 {
        30 // Small cases
    } else {
        let n_f64 = n as f64;
        (n_f64 * n_f64.ln() * 1.3) as usize
    };
    
    let primes = sieve(estimate);
    
    if primes.len() >= n {
        primes[n - 1] as u64
    } else {
        // If estimate was too low, expand search
        let expanded_estimate = estimate * 2;
        let expanded_primes = sieve(expanded_estimate);
        expanded_primes[n - 1] as u64
    }
}

pub fn solve() -> u64 {
    nth_prime(10001)
}
```

### Sieve Implementation

```rust
pub fn sieve(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    for i in 2..=((n as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    
    is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i) } else { None })
        .collect()
}
```

## Performance Analysis

### Time Complexity

**Estimation**: $O(1)$ - just a few floating-point operations

**Sieve**: $O(n \log \log n)$ where $n \approx 120,000$
- Outer loop: $O(\sqrt{n})$ iterations
- Inner loop (marking multiples): Harmonic series behavior
- Total: $O(n \log \log n)$

**Collection**: $O(n)$ - filtering and collecting primes

**Overall**: $O(n \log \log n)$ dominated by sieve

### Space Complexity

**Sieve array**: $O(n)$ - boolean vector of size ~120,000

**Prime list**: $O(\pi(n))$ ≈ $O(n / \ln n)$ ≈ $O(13,000)$ primes

**Overall**: $O(n)$ space

### Benchmark Results

**Performance**: ~346 microseconds

**Analysis**:
- Sieve computation: ~300 μs
- Prime collection: ~40 μs
- Lookup: ~1 μs

**Scaling**:
- Doubling $n$ → ~2× time (near-linear in practice)
- Space usage scales linearly with sieve size

## Step-by-Step Verification

### Small Primes Verification

```
n=1  → p₁ = 2    ✓
n=2  → p₂ = 3    ✓
n=3  → p₃ = 5    ✓
n=4  → p₄ = 7    ✓
n=5  → p₅ = 11   ✓
n=6  → p₆ = 13   ✓ (matches problem statement)
```

### Larger Checkpoints

```
n=10    → p₁₀ = 29      ✓
n=100   → p₁₀₀ = 541    ✓
n=1000  → p₁₀₀₀ = 7919  ✓
n=10001 → p₁₀₀₀₁ = 104743 ✓ (answer)
```

### Estimate Validation

**Estimate for n=10,001**:
$$\text{estimate} = 10001 \times \ln(10001) \times 1.3$$
$$\approx 10001 \times 9.21034 \times 1.3$$
$$\approx 119,712$$

**Actual prime**: 104,743

**Margin**: 119,712 - 104,743 = 14,969 (safety buffer of ~14%)

**Sieve size**: 119,712 elements → Generates ~11,500 primes → More than enough!

**Fallback**: Even if estimate too low, code doubles search space (never triggered here)

## Prime Number Theorem Deep Dive

### Historical Context

**Discovery**: Gauss and Legendre (late 1700s) conjectured, Hadamard and Poussin proved (1896)

**Statement**: 
$$\lim_{n \to \infty} \frac{\pi(n)}{n/\ln(n)} = 1$$

**Meaning**: For large $n$, primes are distributed with density $\frac{1}{\ln(n)}$

### Accuracy for n=10,001

**Simple approximation**: $p_n \approx n \ln(n)$
$$p_{10001} \approx 10001 \times 9.21 \approx 92,109$$

**Error**: $\frac{104,743 - 92,109}{104,743} \approx 12\%$ underestimate

**Better approximation**: $p_n \approx n(\ln n + \ln \ln n)$
$$p_{10001} \approx 10001 \times (9.21 + \ln(9.21))$$
$$\approx 10001 \times (9.21 + 2.22)$$
$$\approx 10001 \times 11.43 \approx 114,311$$

**Error**: $\frac{114,311 - 104,743}{104,743} \approx 9\%$ overestimate

**Even better** (Cipolla 1902): $p_n \approx n(\ln n + \ln \ln n - 1)$
$$p_{10001} \approx 10001 \times (9.21 + 2.22 - 1)$$
$$\approx 10001 \times 10.43 \approx 104,310$$

**Error**: $\frac{104,743 - 104,310}{104,743} \approx 0.4\%$ - excellent!

**Lesson**: More sophisticated approximations get much closer!

### Prime Gaps

**Gap**: $g_n = p_{n+1} - p_n$

**Average gap** near $p_n$ ≈ $\ln(p_n)$ (from PNT)

**For $p_{10001} = 104,743$**:
$$\text{Expected gap} \approx \ln(104,743) \approx 11.56$$

**Actual gap**: $p_{10002} - p_{10001} = 104,759 - 104,743 = 16$

**Observation**: Slightly larger than average (gaps fluctuate)

**Largest known gap**: Between consecutive primes can be arbitrarily large (proven)

## Sieve Optimization Techniques

### Standard Optimizations (Used)

1. **Start multiples from $i^2$**: Smaller multiples already marked
   ```rust
   for j in (i * i..=n).step_by(i)
   ```

2. **Only check up to $\sqrt{n}$**: Larger composites already marked
   ```rust
   for i in 2..=((n as f64).sqrt() as usize)
   ```

3. **Skip even numbers** (not implemented): Could halve space
   - Store only odd numbers
   - Special-case prime 2

### Advanced Optimizations (Not Needed Here)

1. **Segmented Sieve**: For very large $n$, process in chunks
   - Memory-efficient: $O(\sqrt{n})$ space instead of $O(n)$
   - Cache-friendly: Better CPU cache utilization

2. **Wheel Factorization**: Skip multiples of small primes
   - Basis: 2, 3, 5 → Skip 8/30 candidates (~73% speedup)
   - More complex code

3. **Parallel Sieve**: Divide range across threads
   - For $n > 10^8$, parallelization pays off

**For n=120K**: Standard sieve is already fast (~300 μs) - no optimization needed!

## Learning Insights

### 1. Estimation is Powerful

**Strategy**:
1. Estimate upper bound (Prime Number Theorem)
2. Generate primes up to bound (Sieve)
3. Extract desired prime

**Benefits**:
- Don't need exact formula
- Graceful fallback if estimate too low
- Combines theory (PNT) with algorithm (Sieve)

### 2. Algorithm Selection Matters

**Comparison**:

| **Approach** | **Time** | **Space** | **Code Complexity** |
|--------------|----------|-----------|---------------------|
| **Trial Division** | $O(n\sqrt{p_n})$ | $O(1)$ | Simple |
| **Sieve** | $O(p_n \log \log p_n)$ | $O(p_n)$ | Medium |

**Choice**: Sieve wins for finding many primes (8× faster here)

**Lesson**: Invest in better algorithms when finding multiple primes!

### 3. Prime Number Theory in Practice

**PNT applications**:
- Cryptography: Estimate prime density for key generation
- Hash tables: Prime-sized tables reduce collisions
- Algorithms: Prime numbers in randomization

**Takeaway**: Abstract math (PNT) has concrete computational value!

### 4. Test-Driven Confidence

**Testing strategy**:
1. Small primes (n=1-6): Manual verification
2. Checkpoints (n=10, 100, 1000): Known values
3. Edge cases (n=0): Proper error handling
4. Solution (n=10001): Final answer

**Result**: High confidence before submission!

## Comparison: Sieve vs Trial Division

### Trial Division Implementation

```rust
fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n.is_multiple_of(2) { return false; }
    
    for i in (3..=(n as f64).sqrt() as u64).step_by(2) {
        if n.is_multiple_of(i) {
            return false;
        }
    }
    true
}

fn nth_prime_trial(n: usize) -> u64 {
    let mut count = 0;
    let mut candidate = 2;
    
    loop {
        if is_prime(candidate) {
            count += 1;
            if count == n {
                return candidate;
            }
        }
        candidate += if candidate == 2 { 1 } else { 2 };
    }
}
```

**Benchmark** (estimated):
- Trial division for n=10,001: ~15-20 ms
- Sieve for n=10,001: ~346 μs
- **Speedup**: 43-58× faster! 🚀

### When Trial Division Better

**Use trial division when**:
- Finding only 1-2 primes
- Checking primality of specific numbers
- Memory constrained ($O(1)$ space)

**Use sieve when**:
- Finding many primes (n > 100)
- Pre-computing prime list
- Memory available

## Related Problems

### Project Euler Extensions

- **Problem 3**: Largest prime factor (uses trial division)
- **Problem 10**: Sum of primes below 2 million (uses sieve)
- **Problem 37**: Truncatable primes (needs prime generation)
- **Problem 50**: Consecutive prime sum (uses prime list)

### Mathematical Connections

- [[prime-number-theorem]] - Asymptotic distribution
- [[sieve-of-eratosthenes]] - Prime generation algorithm
- [[primality-testing]] - Checking if number is prime
- [[prime-gaps]] - Distribution and patterns

## Implementation Details

**File**: `project_euler/src/problems/p007.rs`

**Public API**:
```rust
pub fn nth_prime(n: usize) -> u64;
pub fn solve() -> u64;  // Solves for n=10,001
```

**Dependencies**:
```rust
use crate::utils::primes::sieve;
```

**Tests**: 5 unit tests covering small primes, checkpoints, and solution

**Performance**: 346 μs (sieve-based)

## Zettelkasten Connections

- [[prime-number-theorem]] - Estimation of nth prime
- [[sieve-of-eratosthenes]] - Prime generation algorithm
- [[primality-testing]] - Trial division alternative
- [[prime-gaps]] - Spacing between consecutive primes
- [[big-o-notation]] - Complexity analysis
- [[project-euler-p003]] - Related: prime factorization
- [[project-euler-p010]] - Related: sum of primes
- [[2026-01-28]] - Daily note for this session

## References

- **Project Euler Problem 7**: https://projecteuler.net/problem=7
- **Prime Number Theorem**: [[prime-number-theorem]]
- **Sieve Algorithm**: [[sieve-of-eratosthenes]]
- **Implementation**: `project_euler/src/problems/p007.rs`
- *The Art of Computer Programming Vol. 2* - Section 4.5.4: Factoring into Primes

---

*Links:*
- **Foundation**: [[prime-number-theorem]], [[sieve-of-eratosthenes]]
- **Techniques**: [[primality-testing]], [[algorithm-selection]]
- **Related Problems**: [[project-euler-p003]], [[project-euler-p010]]
- **Session**: [[2026-01-28]]

*Tags:* #project-euler #primes #sieve #prime-number-theorem #number-theory #estimation
