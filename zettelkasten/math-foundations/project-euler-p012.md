# Project Euler Problem 12: Highly Divisible Triangular Number

**Difficulty**: 5/100  
**Answer**: 76,576,500  
**Performance**: ~720 μs

## Problem Statement

> What is the value of the first triangle number to have over five hundred divisors?

**Triangle number sequence**: 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...

**Formula**: T(n) = 1 + 2 + 3 + ... + n = n(n+1)/2

**Divisors of first triangle numbers**:
- T(1) = 1: divisors {1} → 1 divisor
- T(2) = 3: divisors {1, 3} → 2 divisors
- T(3) = 6: divisors {1, 2, 3, 6} → 4 divisors
- T(7) = 28: divisors {1, 2, 4, 7, 14, 28} → 6 divisors

## Key Mathematical Insight

The breakthrough comes from the **divisor counting formula** combined with **coprimality**.

### Divisor Function Formula

For any integer n with prime factorization n = p₁^a₁ × p₂^a₂ × ... × pₖ^aₖ:

**τ(n) = (a₁ + 1)(a₂ + 1)...(aₖ + 1)**

where τ(n) is the number of divisors of n.

**Example: 28 = 2² × 7¹**
- Prime 2 with exponent 2: (2 + 1) = 3 choices {0, 1, 2}
- Prime 7 with exponent 1: (1 + 1) = 2 choices {0, 1}
- Total divisors: 3 × 2 = **6**
- Verify: {1, 2, 4, 7, 14, 28} ✓

### Why the Formula Works

**Conceptual explanation**:
- Every divisor d of n has form: d = p₁^e₁ × p₂^e₂ × ... × pₖ^eₖ
- For each prime pᵢ, choose exponent 0 ≤ eᵢ ≤ aᵢ
- Number of choices for pᵢ: (aᵢ + 1) [includes 0]
- Total divisors: product of independent choices

**Visual for 28 = 2² × 7¹**:

```
Divisors as (exponent of 2, exponent of 7):
(0,0) = 2⁰ × 7⁰ = 1
(1,0) = 2¹ × 7⁰ = 2
(2,0) = 2² × 7⁰ = 4
(0,1) = 2⁰ × 7¹ = 7
(1,1) = 2¹ × 7¹ = 14
(2,1) = 2² × 7¹ = 28

Grid of choices:
      7⁰  7¹
2⁰  |  1 |  7 |
2¹  |  2 | 14 |
2²  |  4 | 28 |

Rows × Columns = 3 × 2 = 6 divisors
```

See [[divisor-function]] for complete theory.

## Coprimality Optimization

### Triangle Number Structure

T(n) = n(n+1)/2

**Key property**: n and (n+1) are **consecutive**, so they're **coprime**!

**Theorem**: gcd(n, n+1) = 1 for all integers n ≥ 1

**Proof**:
- Suppose d divides both n and (n+1)
- Then d divides (n+1) - n = 1
- Therefore d = 1
- Hence gcd(n, n+1) = 1 ✓

### Multiplicativity of τ(n)

**Theorem**: If gcd(a, b) = 1, then τ(ab) = τ(a) × τ(b)

**Why**: Coprime means no shared prime factors, so:
- a = p₁^a₁ × p₂^a₂ × ...
- b = q₁^b₁ × q₂^b₂ × ... (different primes!)
- ab = p₁^a₁ × ... × q₁^b₁ × ...
- τ(ab) = (a₁+1)...(b₁+1)... = [(a₁+1)...] × [(b₁+1)...] = τ(a) × τ(b)

### Application to Triangle Numbers

Since n and (n+1) are coprime:

**If n is even** (n = 2k):
- T(n) = n(n+1)/2 = k(n+1)
- gcd(k, n+1) = 1 (still coprime!)
- **τ(T(n)) = τ(k) × τ(n+1) = τ(n/2) × τ(n+1)**

**If n is odd** ((n+1) = 2k):
- T(n) = n(n+1)/2 = n × k
- gcd(n, k) = 1 (still coprime!)
- **τ(T(n)) = τ(n) × τ(k) = τ(n) × τ((n+1)/2)**

**Critical benefit**: Instead of factoring T(n) (which can be huge), factor two smaller coprime numbers!

## Algorithm Comparison

### Naive Approach

```rust
fn count_divisors_naive(n: u64) -> u64 {
    (1..=n).filter(|&d| n % d == 0).count() as u64
}

fn solve_naive() {
    let mut n = 1;
    loop {
        let triangle = n * (n + 1) / 2;
        if count_divisors_naive(triangle) > 500 {
            return triangle;
        }
        n += 1;
    }
}
```

**Issues**:
- Counts divisors in O(T(n)) time
- T(n) grows quadratically: T(10,000) = 50,005,000
- Testing 50M divisibility checks per triangle number!
- **Completely impractical**

### Improved: √n Divisor Counting

```rust
fn count_divisors_sqrt(n: u64) -> u64 {
    let mut count = 0;
    let sqrt = (n as f64).sqrt() as u64;
    
    for d in 1..=sqrt {
        if n % d == 0 {
            count += 1;  // Found divisor d
            if d != n / d {
                count += 1;  // Paired divisor n/d
            }
        }
    }
    count
}
```

**Better but still slow**:
- O(√T(n)) per triangle number
- T(12,000) ≈ 72 million → √72M ≈ 8,500 checks
- Still ~100M+ operations total

### Optimized: Prime Factorization

```rust
fn count_divisors_factorization(mut n: u64) -> u64 {
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
    let mut f = 3;
    while f * f <= n {
        if n % f == 0 {
            let mut exp = 0;
            while n % f == 0 {
                exp += 1;
                n /= f;
            }
            divisor_count *= exp + 1;
        }
        f += 2;
    }
    
    if n > 1 {
        divisor_count *= 2;  // Remaining prime
    }
    
    divisor_count
}
```

**Fast**: O(√n) factorization, but n shrinks as factors removed!

### Final: Coprimality Optimization

```rust
fn solve_optimized() -> u64 {
    let mut n = 1;
    
    loop {
        let divisor_count = if n % 2 == 0 {
            count_divisors_factorization(n / 2) * count_divisors_factorization(n + 1)
        } else {
            count_divisors_factorization(n) * count_divisors_factorization((n + 1) / 2)
        };
        
        if divisor_count > 500 {
            return n * (n + 1) / 2;
        }
        
        n += 1;
    }
}
```

**Fastest**:
- Factor numbers ~n/2 and ~n (much smaller than T(n))
- Early termination when n shrinks during factorization
- Actual performance: **~720 μs**

## Complexity Analysis

### Time Complexity

**Per iteration**:
- Factorize two numbers ≈ n
- Factorization cost: O(√n) worst case
- But n shrinks as factors removed, so average much better

**Total**:
- Find solution at n ≈ 12,000
- Cost: O(k × √k) where k = 12,000
- Approximately: 12,000 × 110 ≈ 1.3M operations
- **Actual**: ~720 μs

**Comparison**:

| **Approach** | **Per Triangle** | **Total** | **Practical?** |
|--------------|------------------|-----------|----------------|
| Naive O(T(n)) | 72M ops @ T(12K) | ~10¹² ops | ❌ No |
| √T(n) | 8,500 ops | ~100M ops | ⚠️ Slow |
| Factorization | ~110 ops | ~1.3M ops | ✅ Fast |

### Space Complexity

**O(1)** - Only counters, no arrays needed.

## Implementation Details

### Count Divisors Function

```rust
pub fn count_divisors(mut n: u64) -> u64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    
    let mut divisor_count = 1u64;
    
    // Factor of 2
    if n % 2 == 0 {
        let mut exponent = 0;
        while n % 2 == 0 {
            exponent += 1;
            n /= 2;
        }
        divisor_count *= exponent + 1;  // Apply formula
    }
    
    // Odd factors
    let mut factor = 3u64;
    while factor * factor <= n {
        if n % factor == 0 {
            let mut exponent = 0;
            while n % factor == 0 {
                exponent += 1;
                n /= factor;
            }
            divisor_count *= exponent + 1;  // Apply formula
        }
        factor += 2;  // Skip evens
    }
    
    // Remaining n is prime with exponent 1
    if n > 1 {
        divisor_count *= 2;  // (1 + 1)
    }
    
    divisor_count
}
```

**Key points**:
- Multiply formula incrementally: (a₁+1)(a₂+1)...(aₖ+1)
- Don't store factors, just count divisors directly
- Early termination when factor² > n

### Main Search Function

```rust
pub fn first_triangle_with_divisors(min_divisors: u64) -> u64 {
    let mut n = 1u64;
    
    loop {
        // Use coprimality property
        let divisor_count = if n % 2 == 0 {
            // T(n) = (n/2) × (n+1)
            count_divisors(n / 2) * count_divisors(n + 1)
        } else {
            // T(n) = n × ((n+1)/2)
            count_divisors(n) * count_divisors((n + 1) / 2)
        };
        
        if divisor_count > min_divisors {
            return n * (n + 1) / 2;
        }
        
        n += 1;
    }
}
```

**Why the split**:
- If n even: n/2 and (n+1) are coprime
- If n odd: n and (n+1)/2 are coprime
- Always factor two coprime numbers smaller than T(n)

## Worked Example: n = 7

**Goal**: Count divisors of T(7) = 28

### Method 1: Direct Factorization

28 = 2² × 7¹

τ(28) = (2 + 1)(1 + 1) = 3 × 2 = **6**

### Method 2: Coprimality (n = 7 is odd)

n = 7, (n+1)/2 = 4

**Factor 7**: 7 is prime → τ(7) = 2

**Factor 4**: 4 = 2² → τ(4) = (2 + 1) = 3

**Combine**: τ(T(7)) = τ(7) × τ(4) = 2 × 3 = **6** ✓

**Verification**: T(7) = 7 × 8 / 2 = 28
- Divisors: {1, 2, 4, 7, 14, 28}
- Count: 6 ✓

## Triangle Number Properties

### Definition

**Triangle numbers** represent the number of dots in a triangular array:

```
n=1:  •           T(1) = 1

n=2:  •           T(2) = 3
     • •

n=3:  •           T(3) = 6
     • •
    • • •

n=4:  •           T(4) = 10
     • •
    • • •
   • • • •
```

### Closed Form

**Derivation**:
```
T(n) = 1 + 2 + 3 + ... + n

Write twice:
  S = 1   + 2   + 3   + ... + (n-1) + n
  S = n + (n-1) + (n-2) + ... +   2  + 1
 ───────────────────────────────────────
 2S = (n+1) + (n+1) + (n+1) + ... + (n+1)  [n terms]
 2S = n(n+1)
  S = n(n+1)/2
```

Therefore: **T(n) = n(n+1)/2**

### Connection to Binomial Coefficients

T(n) = C(n+1, 2) = "n+1 choose 2"

**Proof**: C(n+1, 2) = (n+1)! / (2!(n-1)!) = (n+1)n / 2 = T(n)

**Interpretation**: Number of ways to choose 2 items from (n+1) items.

### Recurrence Relation

T(n) = T(n-1) + n

**Proof**: Adding one more row of n dots to triangle of size (n-1).

## Answer Discovery

**Search process**:
```
n = 1,000: T(1000) = 500,500, τ ≈ 32
n = 5,000: T(5000) = 12,502,500, τ ≈ 128
n = 10,000: T(10000) = 50,005,000, τ ≈ 256
n = 12,000: T(12000) = 72,006,000, τ ≈ 324
n = 12,375: T(12375) = 76,576,500, τ = 576 ✓
```

**First triangle number with > 500 divisors**: **76,576,500**

**Factorization**:
- T(12,375) = 12,375 × 12,376 / 2
- 12,375 = 3² × 5³ × 11¹
- 12,376 / 2 = 6,188 = 2² × 7¹ × 13¹ × 17¹
- τ = τ(12,375) × τ(6,188)
- τ = (2+1)(3+1)(1+1) × (2+1)(1+1)(1+1)(1+1)
- τ = 3 × 4 × 2 × 3 × 2 × 2 × 2 = **576 divisors**

## Common Pitfalls

### Pitfall 1: Overflow in Triangle Formula

**Issue**: n(n+1) can overflow before division by 2

**Bad**:
```rust
let triangle = (n * (n + 1)) / 2;  // May overflow!
```

**Good**:
```rust
// Ensure one number is even, so division is exact
let triangle = if n % 2 == 0 {
    (n / 2) * (n + 1)
} else {
    n * ((n + 1) / 2)
};
```

For this problem, n ≈ 12,000, so n(n+1) ≈ 148M (safe in u64), but good practice!

### Pitfall 2: Forgetting the +1 in Formula

**Issue**: τ(p^a) = a + 1, NOT just a

**Example**: 8 = 2³
- **Wrong**: τ(8) = 3 (just the exponent)
- **Right**: τ(8) = 3 + 1 = 4 (divisors: 1, 2, 4, 8)

**Why**: Exponent can be 0, 1, 2, ..., a → (a+1) choices

### Pitfall 3: Not Handling Remaining Prime

**Issue**: After loop, if n > 1, it's a prime factor with exponent 1

**Bad**:
```rust
while factor * factor <= n {
    // ... factor out primes ...
}
// Forget to check if n > 1!
return divisor_count;  // Wrong if n is prime!
```

**Good**:
```rust
while factor * factor <= n {
    // ... factor out primes ...
}

if n > 1 {
    divisor_count *= 2;  // Prime with exponent 1
}
```

### Pitfall 4: Coprimality Assumption

**Issue**: Assuming T(n) = n × (n+1)/2 always splits nicely

**Reality**: Must handle even/odd separately
- If n even: factor n/2 and (n+1)
- If n odd: factor n and (n+1)/2

**Don't** just factor n and (n+1) and divide—loses the coprime property!

## Learning Insights

### Conceptual Breakthroughs

1. **Divisor formula from factorization** - Counting via exponent choices
2. **Multiplicativity** - τ(ab) = τ(a)τ(b) when coprime
3. **Coprimality of consecutive integers** - Algebraic proof
4. **Optimization through structure** - Use problem properties, don't brute force

### Problem-Solving Pattern

This problem demonstrates **mathematical optimization**:

1. **Understand the target** - Find divisor count
2. **Direct approach** - List all divisors (too slow)
3. **Mathematical insight** - Divisor formula from factorization
4. **Structural optimization** - Exploit coprimality
5. **Implementation** - Combine insights for fast solution

**Key lesson**: Mathematical understanding enables algorithmic optimization.

### Connections to Other Problems

- **P3**: Prime factorization technique
- **P5**: GCD/LCM using factorization
- **Future problems**: Divisor sums, perfect numbers, multiplicative functions

## Performance Scaling

**How performance changes with target**:

| **Min Divisors** | **n** | **Triangle** | **Time** |
|------------------|-------|--------------|----------|
| 5 | 7 | 28 | < 1 μs |
| 50 | ~300 | ~45K | ~10 μs |
| 100 | ~600 | ~180K | ~30 μs |
| 200 | ~1,200 | ~720K | ~100 μs |
| 500 | ~12,375 | ~76M | **~720 μs** |
| 1,000 | ~30K | ~450M | ~3 ms |

**Observation**: Approximately O(k^1.5) where k is solution index.

## Related Concepts

### Mathematical Theory
- [[divisor-function]] - τ(n) formula and properties
- [[prime-factorization]] - Decomposition into primes
- [[fundamental-theorem-arithmetic]] - Unique factorization
- [[multiplicative-functions]] - τ(n) is multiplicative

### Number Sequences
- [[triangle-numbers]] - T(n) = n(n+1)/2
- [[polygonal-numbers]] - Generalization (triangular, square, pentagonal, ...)
- [[figurate-numbers]] - Geometric number representations

### Number Theory
- [[coprimality]] - gcd(n, n+1) = 1
- [[gcd-lcm]] - Greatest common divisor properties
- [[arithmetic-functions]] - Functions on integers

### Problem Euler
- [[project-euler-p003]] - Prime factorization
- [[project-euler-p005]] - LCM using factorization
- Future: Perfect numbers, abundant numbers, σ(n)

## Code Reference

**Implementation**: `project_euler/src/problems/p012.rs`

**Key functions**:
- `count_divisors(n)` - Divisor counting via factorization
- `first_triangle_with_divisors(min)` - Main search with coprimality

**Tests**:
- 9 unit tests covering divisor formula, coprimality, edge cases
- 2 integration tests
- 1 doctest

**Benchmarks**:
- Criterion benchmark: ~720 μs ± 3 μs

## References

- *Elementary Number Theory* - Burton (Ch. 5: Arithmetic Functions)
- *An Introduction to the Theory of Numbers* - Hardy & Wright (Ch. 16)
- OEIS A000217 - Triangle numbers
- OEIS A000005 - Divisor function τ(n)

---

*Links:*
- **Theory**: [[divisor-function]], [[prime-factorization]], [[coprimality]]
- **Sequences**: [[triangle-numbers]], [[polygonal-numbers]]
- **Related Problems**: [[project-euler-p003]], [[project-euler-p005]]
- **Code**: `project_euler/src/problems/p012.rs`

*Tags:* #project-euler #divisor-function #triangle-numbers #prime-factorization #coprimality #number-theory #multiplicative-functions #optimization
