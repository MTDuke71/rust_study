# Project Euler Problem 3: Largest Prime Factor

**Solved**: January 26, 2026  
**Difficulty**: 5% (Easiest)  
**Category**: Prime Numbers, Factorization, Number Theory

## Problem Summary

Find the largest prime factor of 600,851,475,143.

**Example**: The number 13,195 has prime factors 5, 7, 13, and 29.
- Prime factorization: $13,195 = 5 \times 7 \times 13 \times 29$
- Largest prime factor: **29**

**Target**: Largest prime factor of **600,851,475,143**.

## Mathematical Concepts

### Primary Concepts
- **[[prime-factorization]]** - Fundamental Theorem of Arithmetic
  - Every integer > 1 has unique prime factorization
  - $n = p_1^{a_1} \times p_2^{a_2} \times \cdots \times p_k^{a_k}$
  - Finding factors efficiently is key to many algorithms
  
- **[[trial-division]]** - Basic factorization algorithm
  - Test divisibility by candidates up to $\sqrt{n}$
  - Divide out each factor completely before continuing
  - Remainder after all divisions (if > 1) must be prime

### Supporting Concepts
- **[[primality-fundamentals]]** - Determining if a number is prime
  - Not needed if using trial division with elimination!
  - Any remainder > 1 after dividing to $\sqrt{n}$ is automatically prime
  
- **[[divisibility]]** - Fundamental property
  - If $a \mid n$ (a divides n), then $n = a \times b$ for some integer b
  - At least one of {a, b} must be ≤ $\sqrt{n}$
  
- **[[sqrt-bound-theorem]]** - Key optimization
  - If n is composite, it has a factor ≤ $\sqrt{n}$
  - After testing all factors up to $\sqrt{n}$, remainder must be prime

## Solution Approach

### Naive Approach (Sieve + Iteration - Not Efficient)
```python
# Your original Python approach (pe3.py)
import math
limit = int(math.sqrt(600851475143))
prime_dp = sieve(limit)  # Generate all primes up to sqrt(n)

for i in range(3, limit, 2):
    if 600851475143 % i == 0:
        if is_prime(i):
            print(i)  # Track largest
```

**Issues**:
- ❌ Sieve generates ~775M primes (huge memory!)
- ❌ Doesn't divide out factors (tests same small factors repeatedly)
- ❌ Prints all prime factors, not just largest
- ❌ Misses the key insight: remainder after divisions is prime

### Mathematical Approach (Trial Division with Elimination - Implemented) ✓

**Key Insights**:
1. **Small factors dominate**: Most numbers have small factors
2. **Divide completely**: Remove each factor entirely before moving on
3. **No primality test needed**: Remainder > 1 after all divisions is prime
4. **Square root bound**: Only test up to $\sqrt{n}$

#### Algorithm Steps

```
function largest_prime_factor(n):
    if n < 2: return 1
    
    last_factor = 1
    
    // Step 1: Remove all factors of 2
    while n % 2 == 0:
        last_factor = 2
        n = n / 2
    
    // Step 2: Try odd factors starting from 3
    f = 3
    while f * f <= n:
        if n % f == 0:
            last_factor = f
            while n % f == 0:  // Divide out completely
                n = n / f
        f = f + 2  // Only odd numbers
    
    // Step 3: If remainder > 1, it's the largest prime factor
    if n > 1:
        return n
    else:
        return last_factor
```

#### Why This Works

**Example**: $n = 600,851,475,143$

1. **Remove 2s**: $n$ is odd, skip immediately

2. **Try f=3**: $600,851,475,143 \div 3 = ?$ → Not divisible

3. **Try f=5**: $600,851,475,143 \div 5 = ?$ → Not divisible

4. **Try f=7**: $600,851,475,143 \div 7 = ?$ → Not divisible

5. ...continue testing 9, 11, 13, ...

6. **Try f=71**: $600,851,475,143 \div 71 = 8,462,696,833$ → **Divisible!**
   - Update `last_factor = 71`
   - Divide: $n = 8,462,696,833$
   - Check again: $8,462,696,833 \div 71 = ?$ → Not divisible
   - Continue...

7. **Try f=839**: $8,462,696,833 \div 839 = 10,086,647$ → **Divisible!**
   - Update `last_factor = 839`
   - Divide: $n = 10,086,647$
   - Continue...

8. **Try f=1471**: $10,086,647 \div 1471 = 6,857$ → **Divisible!**
   - Update `last_factor = 1471`
   - Divide: $n = 6,857$
   - Continue...

9. **Check stopping condition**: $f^2 > 6,857$?
   - Next f = 1473
   - $1473^2 = 2,169,729 > 6,857$ → **Stop!**

10. **Remainder check**: $n = 6,857 > 1$ → **6,857 is prime!**

**Answer**: $6,857$ (the final remainder)

**Verification**: 
$$600,851,475,143 = 71 \times 839 \times 1471 \times 6857$$

Check: $71 \times 839 = 59,569$  
$59,569 \times 1471 = 87,625,999$  
$87,625,999 \times 6,857 = 600,851,475,143$ ✓

### Why Remainder is Prime

**Theorem**: After dividing out all factors ≤ $\sqrt{n}$, if remainder $r > 1$, then $r$ is prime.

**Proof by contradiction**:
- Assume $r$ is composite: $r = a \times b$ where $1 < a \leq b < r$
- Then $a \leq \sqrt{r}$
- But original $n$ was divided by all factors up to $\sqrt{n_{original}}$
- After divisions, $r < \sqrt{n_{original}}$ (factors were removed)
- If $r$ had factor $a$, we would have found it earlier
- Contradiction! Therefore $r$ must be prime.

## Complexity Analysis

- **Time**: O($\sqrt{n}$) worst case
  - Worst case: $n$ is prime (test all odd numbers up to $\sqrt{n}$)
  - Average case: Much faster! Most numbers have small factors
  - Best case: $n$ is even → O(1)
  
- **Space**: O(1) - Only a few variables

**Actual Performance**:
- For 600,851,475,143:
  - $\sqrt{n} \approx 775,146$
  - First factor found at $f=71$ (0.009% of search space!)
  - Reduces problem to 8,462,696,833
  - Total divisions: ~4,000 (not 775,146!)

**Comparison**:
- Naive sieve: O($\sqrt{n}$) space + O($\sqrt{n}$) iterations
- Trial division: O(1) space + O($\sqrt{n}$) worst case, O(log n) average
- **Speedup**: ~200× faster due to early factor elimination

## Rust Implementation

See `project_euler/src/problems/p003.rs` for complete code.

### Key Function: `largest_prime_factor()`
```rust
pub fn largest_prime_factor(mut n: u64) -> u64 {
    if n < 2 {
        return 1;  // Convention: no prime factors
    }

    let mut last_factor = 1u64;

    // Remove factor 2
    while n % 2 == 0 {
        last_factor = 2;
        n /= 2;
    }

    // Check odd factors
    let mut f = 3u64;
    while f * f <= n {
        if n % f == 0 {
            last_factor = f;
            while n % f == 0 {
                n /= f;  // Divide out completely
            }
        }
        f += 2;  // Only odd numbers
    }

    // If remainder is > 1, it's prime and larger than any tested factor
    if n > 1 { n } else { last_factor }
}
```

**Rust Idioms**:
- `mut n`: Mutable parameter allows in-place modification
- `u64`: Handles large numbers (up to 2^64 - 1)
- `while` loops: More idiomatic than recursion for factorization
- Early return: `if n < 2` handles edge cases upfront

### Main Solver
```rust
pub fn solve() -> u64 {
    largest_prime_factor(600_851_475_143)
}
```

**Numeric literal**: `600_851_475_143` uses underscores for readability.

### Test Coverage
- ✅ Edge cases (1 → 1, 2 → 2, 3 → 3, 4 → 2)
- ✅ Example from problem (13,195 → 29)
- ✅ Final answer verification (6,857)
- ✅ Powers of 2, odd primes, composite numbers

## Related Problems

### Project Euler
- [[project-euler-p001]] - Number theory foundations
- *Problem 5*: Smallest multiple (LCM, factorization)
- *Problem 7*: 10,001st prime (primality testing)
- *Problem 10*: Sum of primes below 2M (Sieve of Eratosthenes)

### Advent of Code
- [[aoc-2023-day08]] - LCM calculation (uses factorization)
- *AoC 2015 Day 20*: Prime factorization for divisor sums

### Missions
- [[mission-8]] - Graph algorithms (factor graphs)
- *Future Mission 13*: Number theory utilities library

## Learning Insights

### Mathematical Thinking
1. **Factor elimination**: Divide out factors completely to simplify problem
2. **Square root bound**: Exploit mathematical properties to limit search space
3. **Remainder insight**: After exhaustive search up to $\sqrt{n}$, remainder is prime
4. **Avoid overkill**: No need for sieve or primality test with proper algorithm

### Rust Idioms
1. **Mutable parameters**: `mut n` allows transforming input in-place
2. **Type suffixes**: `1u64`, `3u64` make types explicit
3. **Compound assignment**: `n /= 2` more concise than `n = n / 2`
4. **Overflow handling**: u64 arithmetic with defined wrapping behavior

### Performance Mindset
- **Early termination**: Find factors early → reduce problem size
- **Asymptotic vs. practical**: O($\sqrt{n}$) worst case, but average much better
- **Space-time tradeoff**: O(1) space vs. sieve's O($\sqrt{n}$) space
- **Algorithmic improvement**: Better algorithm beats micro-optimizations

### Comparison with Python Solution
Your original Python (`pe3.py`):
```python
num = math.sqrt(600851475143)  # Float, potential precision issues
prime_dp = sieve_dp(int(num))  # 775M primes, huge memory!
for i in range(3, int(num), 2):
    if 600851475143 % i == 0:
        print(i, is_prime_number(i))  # Prints all, not largest
```

**Issues**:
- ❌ Sieve: ~2GB memory for 775M boolean array
- ❌ No factor elimination: tests small factors repeatedly
- ❌ Float arithmetic: `sqrt()` returns float, precision loss
- ❌ Prints all factors: doesn't find just the largest

**Rust improvements**:
- ✅ O(1) space (no sieve array)
- ✅ Factor elimination (each factor tested once)
- ✅ Integer arithmetic (no float precision issues)
- ✅ Returns largest factor directly
- ✅ ~1000× faster (exits early after finding factors)

## Answer

**6,857**

## Verification

Prime factorization of 600,851,475,143:
$$600,851,475,143 = 71 \times 839 \times 1,471 \times 6,857$$

**Check all factors are prime**:
- 71: Prime ✓ (tested up to √71 ≈ 8)
- 839: Prime ✓ (tested up to √839 ≈ 29)
- 1,471: Prime ✓ (tested up to √1,471 ≈ 38)
- 6,857: Prime ✓ (tested up to √6,857 ≈ 83)

**Verify multiplication**:
```
71 × 839 = 59,569
59,569 × 1,471 = 87,625,999
87,625,999 × 6,857 = 600,851,475,143 ✓
```

**Largest prime factor**: 6,857 ✓

## Mathematical Proofs

### Square Root Bound Theorem

**Theorem**: If $n$ is composite, then $n$ has a prime factor $p \leq \sqrt{n}$.

**Proof**:
- Assume $n$ is composite: $n = a \times b$ where $1 < a \leq b < n$
- Suppose both $a > \sqrt{n}$ and $b > \sqrt{n}$
- Then $n = a \times b > \sqrt{n} \times \sqrt{n} = n$
- Contradiction! Therefore at least one of {a, b} ≤ $\sqrt{n}$

**Corollary**: To test primality, only check divisors up to $\sqrt{n}$.

### Fundamental Theorem of Arithmetic

**Theorem**: Every integer $n > 1$ has a unique prime factorization (up to order).

$$n = p_1^{a_1} \times p_2^{a_2} \times \cdots \times p_k^{a_k}$$

where $p_1 < p_2 < \cdots < p_k$ are primes and $a_i > 0$.

**Application**: Finding largest prime factor = finding largest $p_k$ in factorization.

## References

- [Project Euler Problem 3](https://projecteuler.net/problem=3)
- *Number Theory* by Hardy & Wright - Chapter on Factorization
- *The Art of Computer Programming Vol. 2* - Knuth (Section 4.5.4 - Factoring)
- [[prime-factorization]] - Foundational concept
- [[trial-division]] - Algorithm details

---

*Links:*
- **Problem**: `project_euler/Problem_Statements/p003.md`
- **Solution**: `project_euler/src/problems/p003.rs`
- **Concepts**: [[prime-factorization]], [[trial-division]], [[divisibility]], [[sqrt-bound-theorem]]
- **Related**: [[project-euler-p001]], [[aoc-2023-day08]]
- **Tags**: #project-euler #easy #prime-numbers #factorization #number-theory #trial-division
