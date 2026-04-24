# Sieve of Eratosthenes

**Category**: Number Theory, Prime Numbers, Algorithms  
**Difficulty**: Fundamental  
**Complexity**: O(n log log n) time, O(n) space  
**Related Concepts**: [[prime-numbers]], [[primality-fundamentals]], [[prime-number-theorem]]

## Definition

The **Sieve of Eratosthenes** is an ancient algorithm for finding all prime numbers up to a given limit $n$. It works by iteratively marking the multiples of each prime as composite.

**Discovered**: Ancient Greece, attributed to Eratosthenes of Cyrene (276-194 BCE)

**Key idea**: If a number is prime, all its multiples are composite.

## Algorithm

### Basic Steps

1. Create a list of consecutive integers from 2 to n: (2, 3, 4, ..., n)
2. Start with the smallest number (2)
3. Mark all multiples of 2 (except 2 itself) as composite
4. Find the next unmarked number (3), mark its multiples as composite
5. Repeat until you've processed all numbers up to √n
6. All unmarked numbers are prime

### Visual Example (n = 30)

```
Initial: 2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30

Step 1 (mark multiples of 2):
         2  3  ✗  5  ✗  7  ✗  ✗  ✗  11 ✗  13 ✗  ✗  ✗  17 ✗  19 ✗  ✗  ✗  23 ✗  ✗  ✗  ✗  ✗  29 ✗

Step 2 (mark multiples of 3):
         2  3  ✗  5  ✗  7  ✗  ✗  ✗  11 ✗  13 ✗  ✗  ✗  17 ✗  19 ✗  ✗  ✗  23 ✗  ✗  ✗  ✗  ✗  29 ✗

Step 3 (mark multiples of 5):
         2  3  ✗  5  ✗  7  ✗  ✗  ✗  11 ✗  13 ✗  ✗  ✗  17 ✗  19 ✗  ✗  ✗  23 ✗  ✗  ✗  ✗  ✗  29 ✗

Result: Primes up to 30 = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29}
```

**Note**: We only need to process up to √30 ≈ 5.5, so after marking multiples of 5, we're done!

## Implementation

### Basic Rust Implementation

```rust
/// Generate all primes up to n using Sieve of Eratosthenes
pub fn sieve(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    
    // Step 1: Create boolean array
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    // Step 2: Mark multiples of each prime
    for i in 2..=((n as f64).sqrt() as usize) {
        if is_prime[i] {
            // Step 3: Mark multiples starting from i²
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    
    // Step 4: Collect primes
    is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i) } else { None })
        .collect()
}
```

### Key Optimizations (Built-in)

1. **Start from i²**: Mark multiples starting from $i^2$ instead of $2i$
   - All smaller multiples already marked by smaller primes
   - Example: For i=5, start at 25 (not 10, 15, 20 - already marked)

2. **Only check up to √n**: Only need to sieve up to $\sqrt{n}$
   - Any composite number ≤ n has a factor ≤ √n
   - After √n, remaining unmarked numbers are automatically prime

3. **Use step_by(i)**: Skip by i instead of checking every number
   - Direct marking of multiples: i², i²+i, i²+2i, ...

## Complexity Analysis

### Time Complexity

**Overall**: $O(n \log \log n)$

**Derivation**:
- Outer loop: Up to $\sqrt{n}$ iterations
- Inner loop for prime $p$: Marks $\frac{n}{p}$ multiples
- Total operations: $\sum_{p \text{ prime}, p \le \sqrt{n}} \frac{n}{p}$

Using prime harmonic series:
$$\sum_{p \le n} \frac{1}{p} \approx \log \log n$$

Therefore: $n \times \log \log n$ operations

**Comparison to trial division**:
- Trial division for all primes ≤ n: $O(n^{3/2})$ worst case
- Sieve: $O(n \log \log n)$ - **much faster!**

### Space Complexity

**Space**: $O(n)$ - boolean array of size n+1

**Trade-off**: Uses more memory than trial division (O(1)) but much faster for finding many primes.

## Optimizations

### 1. Odd Numbers Only (Space Halving)

Skip even numbers entirely (except 2):

```rust
pub fn sieve_odd_only(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    
    let mut primes = vec![2];
    
    if n < 3 {
        return primes;
    }
    
    // Array for odd numbers only: index i represents 2i+3
    let size = (n - 1) / 2;
    let mut is_prime = vec![true; size];
    
    for i in 0..((((n as f64).sqrt() as usize) - 1) / 2) {
        if is_prime[i] {
            let p = 2 * i + 3;
            // Mark multiples starting from p²
            let start = (p * p - 3) / 2;
            for j in (start..size).step_by(p) {
                is_prime[j] = false;
            }
        }
    }
    
    // Collect odd primes
    for (i, &prime) in is_prime.iter().enumerate() {
        if prime {
            primes.push(2 * i + 3);
        }
    }
    
    primes
}
```

**Benefit**: 50% memory reduction, ~2× speedup

### 2. Wheel Factorization (2, 3, 5)

Skip multiples of 2, 3, and 5:
- Only store candidates of form 30k + {1, 7, 11, 13, 17, 19, 23, 29}
- 8 candidates per 30 numbers (vs 15 odd numbers)
- **Space**: 26.7% of original (8/30)
- **Speedup**: ~3× faster than odd-only

### 3. Segmented Sieve

For very large n (> 10⁸), process in chunks:

**Algorithm**:
1. Sieve primes up to √n (fits in cache)
2. Process range in segments of size √n
3. Use small primes to mark multiples in each segment

**Benefits**:
- **Memory**: $O(\sqrt{n})$ instead of $O(n)$
- **Cache-friendly**: Each segment fits in L1/L2 cache
- **Parallelizable**: Process segments independently

**Typical use**: Finding primes up to 10¹² or beyond

### 4. Bit Array (Memory Optimization)

Use 1 bit per number instead of 1 byte:

```rust
// Instead of Vec<bool> (8 bits per element)
// Use bitvec crate or manual bit manipulation
// Reduces memory by 8×
```

**Trade-off**: Slightly slower bit operations, 8× less memory

## Applications

### Finding All Primes in Range

**Problem**: Find all primes between 1 and n

**Solution**: Sieve is optimal - generates all at once

**Example**: [[project-euler-p010]] - Sum of primes below 2 million
- Sieve generates all primes instantly
- Sum them up: $O(n \log \log n)$ total

### Finding nth Prime

**Problem**: Find the 10,001st prime ([[project-euler-p007]])

**Approach**:
1. Estimate upper bound using [[prime-number-theorem]]: $p_n \approx n \ln n$
2. Sieve up to estimate
3. Extract nth prime from list

**Complexity**: $O(n \ln n \times \log \log(n \ln n))$ ≈ $O(n \ln n)$

**Alternative** (trial division): $O(n \times \sqrt{p_n})$ - much slower!

### Prime Factorization

**Use case**: Factorize many numbers up to n

**Approach**:
1. Sieve primes up to n
2. Use generated primes for trial division
3. Much faster than testing all numbers

**Example**: Factorize all numbers 1 to 10⁶
- Sieve once: ~10 ms
- Factorize each: Fast (use only primes)
- vs. Trial division from scratch: Much slower

### Counting Primes (π(n))

**Problem**: Count primes ≤ n

**Solution**: Sieve and count

**Example**: How many primes ≤ 1,000,000?
```rust
let primes = sieve(1_000_000);
let count = primes.len();  // 78,498 primes
```

## Comparison with Other Methods

### vs. Trial Division

| **Aspect** | **Sieve** | **Trial Division** |
|------------|-----------|-------------------|
| **Find one prime** | Overkill | Good: $O(\sqrt{p})$ |
| **Find all primes ≤ n** | Optimal: $O(n \log \log n)$ | Slow: $O(n^{3/2})$ |
| **Memory** | $O(n)$ | $O(1)$ |
| **Implementation** | Medium complexity | Simple |

**Verdict**: Sieve for many primes, trial division for individual primality tests

### vs. Probabilistic Tests (Miller-Rabin)

| **Aspect** | **Sieve** | **Miller-Rabin** |
|------------|-----------|------------------|
| **Purpose** | Generate primes ≤ n | Test if specific number is prime |
| **Deterministic** | Yes | Probabilistic (can be made deterministic) |
| **Large numbers** | Impractical (memory) | Fast: $O(k \log^3 n)$ |
| **Small range** | Optimal | Overkill |

**Verdict**: Sieve for small ranges, Miller-Rabin for large individual numbers

### vs. Atkin's Sieve

**Atkin's Sieve**: More complex algorithm, theoretically $O(n / \log \log n)$

**Reality**: Constant factors make it slower than Eratosthenes for n < 10⁹

**Conclusion**: Stick with Eratosthenes for practical use

## Why √n Bound Works

### Theorem

If $n$ is composite, it has a prime factor $p \le \sqrt{n}$.

**Proof**:
- Suppose $n = a \times b$ where $a, b > 1$
- If both $a, b > \sqrt{n}$, then $a \times b > n$ (contradiction)
- Therefore, at least one factor $\le \sqrt{n}$

**Consequence**: After marking multiples of all primes ≤ √n, any unmarked number must be prime!

### Example (n = 30)

$\sqrt{30} \approx 5.48$

Primes to check: 2, 3, 5

After marking multiples of 2, 3, 5:
- Unmarked: 7, 11, 13, 17, 19, 23, 29
- All are prime! ✓
- No need to check 7, 11, 13... as sieve candidates

## Historical Context

### Eratosthenes of Cyrene (276-194 BCE)

**Background**: Greek mathematician, geographer, astronomer
- Chief librarian at Library of Alexandria
- First to calculate Earth's circumference accurately

**The Sieve**: One of oldest known algorithms
- Still in use 2,300+ years later!
- Elegant simplicity and efficiency

### Modern Variations

- **1934**: Wheel factorization invented
- **1999**: Atkin's sieve proposed (theoretical improvement)
- **2000s**: Segmented sieves for massive ranges
- **Present**: Used in cryptography, number theory research

## Edge Cases and Considerations

### Small n

```rust
sieve(0) → []
sieve(1) → []
sieve(2) → [2]
sieve(10) → [2, 3, 5, 7]
```

### Memory Limits

**Problem**: n = 10¹² requires 1 TB of memory!

**Solutions**:
1. Segmented sieve (√n memory)
2. Wheel sieve (reduce to ~27% memory)
3. Primality testing instead (Miller-Rabin for specific numbers)

### Integer Overflow

For very large n:
- `i * i` can overflow
- Use checked arithmetic or `usize` carefully
- Consider `u64` for large ranges

## Performance Benchmarks

**Typical performance** (modern CPU):

| **n** | **Primes Found** | **Time** | **Memory** |
|-------|------------------|----------|------------|
| 10³ | 168 | ~1 μs | ~1 KB |
| 10⁴ | 1,229 | ~10 μs | ~10 KB |
| 10⁵ | 9,592 | ~100 μs | ~100 KB |
| 10⁶ | 78,498 | ~10 ms | ~1 MB |
| 10⁷ | 664,579 | ~100 ms | ~10 MB |
| 10⁸ | 5,761,455 | ~1 s | ~100 MB |

**Scaling**: Nearly linear in practice for n < 10⁸

## Common Mistakes

### ❌ Starting from 2i Instead of i²

```rust
// WRONG: Wastes operations
for j in (2*i..=n).step_by(i) {
    is_prime[j] = false;
}

// CORRECT: Start from i²
for j in (i*i..=n).step_by(i) {
    is_prime[j] = false;
}
```

**Why**: Multiples 2i, 3i, ..., (i-1)i already marked by smaller primes

### ❌ Sieving Beyond √n

```rust
// WRONG: Unnecessary iterations
for i in 2..=n {
    if is_prime[i] {
        // ...
    }
}

// CORRECT: Stop at √n
for i in 2..=((n as f64).sqrt() as usize) {
    if is_prime[i] {
        // ...
    }
}
```

**Why**: All composites ≤ n already marked by factors ≤ √n

### ❌ Not Handling n < 2

```rust
// WRONG: May panic or return incorrect results
pub fn sieve(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];  // Panic if n = 0?
    // ...
}

// CORRECT: Handle edge cases
pub fn sieve(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    // ...
}
```

## Related Algorithms

### Sieve of Sundaram

Generates odd primes by eliminating numbers of form $i + j + 2ij$

**Complexity**: Similar to Eratosthenes but more complex

**Use**: Mainly historical/educational interest

### Sieve of Atkin

Uses modular arithmetic and number theory results

**Complexity**: $O(n / \log \log n)$ theoretically

**Reality**: Slower than Eratosthenes for practical n

### Segmented Sieve

Extension of Eratosthenes for huge ranges

**Memory**: $O(\sqrt{n})$ instead of $O(n)$

**Use**: Finding primes in ranges like [10¹², 10¹² + 10⁶]

## Project Euler Applications

- [[project-euler-p007]] - Find 10,001st prime
  - Sieve up to estimate from Prime Number Theorem
  - Extract nth element from generated list
  
- [[project-euler-p010]] - Sum of primes below 2 million
  - Sieve generates all primes
  - Sum them: $O(n \log \log n)$ total
  
- [[project-euler-p037]] - Truncatable primes
  - Generate candidates with sieve
  - Test truncation property

- [[project-euler-p050]] - Consecutive prime sum
  - Pre-generate primes with sieve
  - Find longest consecutive sum

## Learning Path

### Beginner
1. Understand basic algorithm with examples
2. Implement simple version
3. Test with small n (< 100)

### Intermediate
4. Add √n optimization
5. Start from i² optimization
6. Understand time complexity derivation

### Advanced
7. Implement odd-only sieve
8. Understand wheel factorization
9. Implement segmented sieve
10. Parallel sieve for multi-core

## Code Repository

**Implementation**: `project_euler/src/utils/primes.rs`

**Usage examples**:
```rust
use project_euler::utils::primes::sieve;

let primes = sieve(100);
assert_eq!(primes[0], 2);
assert_eq!(primes[24], 97);  // 25th prime
assert_eq!(primes.len(), 25);  // π(100) = 25
```

## Related Concepts

- [[prime-number-theorem]] - Asymptotic distribution of primes
- [[primality-fundamentals]] - Alternative approach for individual numbers
- [[trial-division]] - Simpler but slower method
- [[prime-factorization]] - Uses sieve for preprocessing
- [[miller-rabin-primality-test]] - For large individual numbers
- [[wheel-factorization]] - Optimization technique

## References

- *The Art of Computer Programming Vol. 2* - Knuth (Section 4.5.4)
- [[project-euler-p007]] - Application to finding nth prime
- [[project-euler-p010]] - Application to summing primes
- *Prime Numbers: A Computational Perspective* - Crandall & Pomerance
- Wikipedia: "Sieve of Eratosthenes"

---

*Links:*
- **Applications**: [[project-euler-p007]], [[project-euler-p010]], [[prime-factorization]]
- **Theory**: [[prime-number-theorem]], [[sqrt-bound-theorem]]
- **Algorithms**: [[primality-fundamentals]], [[trial-division]], [[segmented-sieve]]
- **Code**: `project_euler/src/utils/primes.rs`

*Tags:* #sieve #primes #algorithms #number-theory #optimization #eratosthenes #project-euler

