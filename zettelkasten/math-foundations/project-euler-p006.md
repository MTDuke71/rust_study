# Project Euler Problem 6: Sum Square Difference

**Difficulty**: Easy  
**Published**: December 14, 2001  
**Solved**: January 28, 2026  
**Category**: Arithmetic Series, Algebra  
**Related Concepts**: [[arithmetic-series]], [[closed-form-formulas]]

## Problem Statement

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

**Example** (n=10):
- Sum of squares: $1^2 + 2^2 + \cdots + 10^2 = 385$
- Square of sum: $(1 + 2 + \cdots + 10)^2 = 55^2 = 3025$
- Difference: $3025 - 385 = 2640$

**Find**: Difference for the first 100 natural numbers

## Solution Overview

**Answer**: **25,164,150**

**Approach**: Closed-form formulas using arithmetic series

## Mathematical Analysis

### Arithmetic Series Formulas

**Sum of first n natural numbers**:
$$S_n = 1 + 2 + 3 + \cdots + n = \frac{n(n+1)}{2}$$

**Proof** (Gauss's method):
```
  S = 1   + 2   + 3   + ... + n
  S = n   + (n-1) + (n-2) + ... + 1
  ───────────────────────────────────
 2S = (n+1) + (n+1) + (n+1) + ... + (n+1)  [n times]
 2S = n(n+1)
  S = n(n+1)/2  ✓
```

**Sum of squares of first n natural numbers**:
$$S_n^2 = 1^2 + 2^2 + 3^2 + \cdots + n^2 = \frac{n(n+1)(2n+1)}{6}$$

**Proof** (by induction):
- **Base**: $n=1$: $1^2 = \frac{1 \cdot 2 \cdot 3}{6} = 1$ ✓
- **Inductive step**: Assume true for $n$, prove for $n+1$:
$$\sum_{k=1}^{n+1} k^2 = \sum_{k=1}^{n} k^2 + (n+1)^2$$
$$= \frac{n(n+1)(2n+1)}{6} + (n+1)^2$$
$$= \frac{n(n+1)(2n+1) + 6(n+1)^2}{6}$$
$$= \frac{(n+1)[n(2n+1) + 6(n+1)]}{6}$$
$$= \frac{(n+1)[2n^2 + n + 6n + 6]}{6}$$
$$= \frac{(n+1)[2n^2 + 7n + 6]}{6}$$
$$= \frac{(n+1)(n+2)(2n+3)}{6}$$
$$= \frac{(n+1)((n+1)+1)(2(n+1)+1)}{6}$$ ✓

### Problem Decomposition

**Square of sum**:
$$(S_n)^2 = \left(\frac{n(n+1)}{2}\right)^2 = \frac{n^2(n+1)^2}{4}$$

**Sum of squares**:
$$S_n^2 = \frac{n(n+1)(2n+1)}{6}$$

**Difference**:
$$D = (S_n)^2 - S_n^2 = \frac{n^2(n+1)^2}{4} - \frac{n(n+1)(2n+1)}{6}$$

### Simplifying the Difference

Factor out $n(n+1)$:
$$D = n(n+1)\left[\frac{n(n+1)}{4} - \frac{2n+1}{6}\right]$$

Find common denominator (12):
$$D = n(n+1)\left[\frac{3n(n+1)}{12} - \frac{2(2n+1)}{12}\right]$$

$$D = n(n+1)\left[\frac{3n^2 + 3n - 4n - 2}{12}\right]$$

$$D = n(n+1)\left[\frac{3n^2 - n - 2}{12}\right]$$

Factor $3n^2 - n - 2 = (3n+2)(n-1)$:
$$D = \frac{n(n+1)(n-1)(3n+2)}{12}$$

**Closed form**:
$$\boxed{D = \frac{n(n-1)(n+1)(3n+2)}{12}}$$

### Verification for n=10

Using simplified formula:
$$D = \frac{10 \cdot 9 \cdot 11 \cdot 32}{12} = \frac{31,680}{12} = 2640$$ ✓

Using step-by-step:
- Sum: $S_{10} = \frac{10 \cdot 11}{2} = 55$
- Square of sum: $55^2 = 3025$
- Sum of squares: $S_{10}^2 = \frac{10 \cdot 11 \cdot 21}{6} = \frac{2310}{6} = 385$
- Difference: $3025 - 385 = 2640$ ✓

## Implementation

### Rust Code

```rust
/// Sum of first n natural numbers: 1 + 2 + ... + n = n(n+1)/2
fn sum_of_naturals(n: u64) -> u64 {
    n * (n + 1) / 2
}

/// Sum of squares: 1² + 2² + ... + n² = n(n+1)(2n+1)/6
fn sum_of_squares(n: u64) -> u64 {
    n * (n + 1) * (2 * n + 1) / 6
}

/// Calculate difference between square of sum and sum of squares
pub fn sum_square_difference(n: u64) -> u64 {
    let sum = sum_of_naturals(n);
    let square_of_sum = sum * sum;
    let sum_of_sq = sum_of_squares(n);
    
    square_of_sum - sum_of_sq
}

pub fn solve() -> u64 {
    sum_square_difference(100)
}
```

### Alternative: Direct Formula

```rust
pub fn sum_square_difference_direct(n: u64) -> u64 {
    // D = n(n-1)(n+1)(3n+2) / 12
    n * (n - 1) * (n + 1) * (3 * n + 2) / 12
}
```

**Trade-offs**:
- **Step-by-step** (chosen): More readable, verifiable, reusable components
- **Direct formula**: Slightly faster (fewer multiplications), less clear

## Performance Analysis

### Time Complexity

**Both approaches**: $O(1)$ - constant time
- Fixed number of arithmetic operations
- No loops, no iteration
- Independent of input magnitude

### Space Complexity

**Both approaches**: $O(1)$ - constant space
- Only a few temporary variables

### Benchmark Results

**Performance**: ~111 picoseconds ⚡

**Analysis**:
- Blazingly fast due to constant-time formula
- Compiler likely inlines and optimizes to just a few CPU instructions
- No memory allocation, no branching

## Naive vs Optimal Approach

### Naive Iterative Approach

```rust
fn sum_square_difference_naive(n: u64) -> u64 {
    let sum: u64 = (1..=n).sum();
    let sum_of_sq: u64 = (1..=n).map(|x| x * x).sum();
    sum * sum - sum_of_sq
}
```

**Complexity**: $O(n)$ - requires iterating through all numbers

**Benchmark** (estimated for n=100):
- Naive: ~500 ns (100 iterations × 2 loops)
- Optimal: ~111 ps (constant time)
- **Speedup**: ~4500× faster! 🚀

### Why Closed-Form Wins

**Closed-form advantages**:
1. ✅ **Constant time** regardless of $n$
2. ✅ **No iteration overhead**
3. ✅ **Works for arbitrarily large $n$** (within `u64` limits)
4. ✅ **Mathematically elegant**

**When iteration acceptable**:
- Small $n$ (< 10) where clarity matters more
- Learning/teaching purposes
- When formula derivation is complex

**Lesson**: Invest time finding closed-form solutions for significant performance gains!

## Step-by-Step Calculation (n=100)

### Using Formulas

**Sum of naturals**:
$$S_{100} = \frac{100 \times 101}{2} = \frac{10,100}{2} = 5,050$$

**Square of sum**:
$$(S_{100})^2 = 5,050^2 = 25,502,500$$

**Sum of squares**:
$$S_{100}^2 = \frac{100 \times 101 \times 201}{6} = \frac{2,030,100}{6} = 338,350$$

**Difference**:
$$D = 25,502,500 - 338,350 = 25,164,150$$ ✓

### Using Direct Formula

$$D = \frac{100 \times 99 \times 101 \times 302}{12}$$

$$= \frac{100 \times 99 \times 101 \times 302}{12}$$

$$= \frac{301,969,800}{12} = 25,164,150$$ ✓

Both methods agree!

## Mathematical Insights

### Why Is Difference Always Positive?

**Claim**: For $n \ge 2$, $(S_n)^2 > S_n^2$

**Proof**:
$$\frac{n^2(n+1)^2}{4} > \frac{n(n+1)(2n+1)}{6}$$

Divide both sides by $n(n+1)$:
$$\frac{n(n+1)}{4} > \frac{2n+1}{6}$$

Multiply by 12:
$$3n(n+1) > 2(2n+1)$$
$$3n^2 + 3n > 4n + 2$$
$$3n^2 - n - 2 > 0$$
$$(3n+2)(n-1) > 0$$

For $n \ge 2$: both factors positive, so inequality holds. ✓

**Edge case**: $n=1$: $D = 1 - 1 = 0$ (neither positive nor negative)

### Asymptotic Growth

For large $n$:
$$D \approx \frac{n \cdot n \cdot n \cdot 3n}{12} = \frac{3n^4}{12} = \frac{n^4}{4}$$

**Growth rate**: $\Theta(n^4)$

**Observation**: Difference grows quartically - very rapid!

### Connection to Cross Terms

Expand $(1 + 2 + \cdots + n)^2$:
$$(1+2+\cdots+n)^2 = (1^2 + 2^2 + \cdots + n^2) + 2\sum_{i<j} ij$$

The difference is exactly **twice the sum of all cross products**:
$$D = 2\sum_{i<j} ij$$

**Example** (n=3):
$$(1+2+3)^2 = 1^2 + 2^2 + 3^2 + 2(1 \cdot 2 + 1 \cdot 3 + 2 \cdot 3)$$
$$36 = 14 + 2(2 + 3 + 6) = 14 + 22$$ ✓

**Insight**: The "extra" part comes from multiplying distinct numbers!

## Learning Insights

### 1. Power of Closed-Form Formulas

**Problem solving strategy**:
1. Identify pattern (arithmetic series)
2. Recall/derive formula
3. Apply directly

**Result**: $O(1)$ solution instead of $O(n)$!

**Broader applications**:
- Geometric series: $\sum r^k = \frac{r^{n+1}-1}{r-1}$
- Triangular numbers: $T_n = \frac{n(n+1)}{2}$
- Binomial coefficients: $\binom{n}{k} = \frac{n!}{k!(n-k)!}$

### 2. Algebraic Manipulation Skills

**Importance**: Simplifying the difference revealed elegant form:
$$\frac{n(n-1)(n+1)(3n+2)}{12}$$

Shows all factors clearly, makes patterns visible.

**Practice**: Derive formulas yourself - don't just memorize!

### 3. Test Multiple Approaches

**Validation strategy**:
1. Step-by-step calculation
2. Direct formula
3. Small examples by hand
4. Edge cases (n=1, n=2)

All methods should agree → confidence in correctness.

### 4. Benchmark Confirms Theory

**Measured**: ~111 picoseconds
**Expected**: $O(1)$ with minimal operations

Theory matches practice → implementation correct and optimal.

## Related Problems

### Project Euler Extensions

- **Problem 1**: Sum of multiples (arithmetic series)
- **Problem 42**: Triangle numbers (uses $T_n = \frac{n(n+1)}{2}$)
- **Problem 206**: Square root patterns

### Mathematical Connections

- [[arithmetic-series]] - Sum formulas
- [[quadratic-formulas]] - $n^2$ terms
- [[telescoping-series]] - Proof techniques
- [[binomial-theorem]] - Expansion of $(a+b)^2$

## Implementation Details

**File**: `project_euler/src/problems/p006.rs`

**Public API**:
```rust
pub fn sum_square_difference(n: u64) -> u64;
pub fn solve() -> u64;  // Solves for n=100
```

**Helper functions**:
```rust
fn sum_of_naturals(n: u64) -> u64;
fn sum_of_squares(n: u64) -> u64;
```

**Tests**: 5 unit tests covering examples, edge cases, and solution

**Performance**: 111 ps (constant time)

## Zettelkasten Connections

- [[arithmetic-series]] - Sum formulas and derivations
- [[closed-form-formulas]] - Power of direct formulas
- [[proof-by-induction]] - Sum of squares proof
- [[big-o-notation]] - Complexity analysis
- [[project-euler-p001]] - Related: arithmetic series
- [[2026-01-28]] - Daily note for this session

## References

- **Project Euler Problem 6**: https://projecteuler.net/problem=6
- **Arithmetic Series**: [[arithmetic-series]]
- **Implementation**: `project_euler/src/problems/p006.rs`
- *Concrete Mathematics* - Chapter 2: Sums

---

*Links:*
- **Foundation**: [[arithmetic-series]], [[closed-form-formulas]]
- **Techniques**: [[proof-by-induction]], [[algebraic-simplification]]
- **Related Problems**: [[project-euler-p001]], [[project-euler-p042]]
- **Session**: [[2026-01-28]]

*Tags:* #project-euler #arithmetic-series #closed-form #algebra #optimization #constant-time
