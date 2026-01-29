# Project Euler Problem 9: Special Pythagorean Triplet

**Difficulty**: Easy (5%)  
**Published**: January 25, 2002  
**Solved**: January 29, 2026  
**Category**: Number Theory, Search Algorithms  
**Related Concepts**: [[pythagorean-triplets]], [[number-theory-basics]], [[constrained-search]]

## Problem Statement

A Pythagorean triplet is a set of three natural numbers, $a < b < c$, for which:

$$a^2 + b^2 = c^2$$

For example, $3^2 + 4^2 = 9 + 16 = 25 = 5^2$.

There exists **exactly one** Pythagorean triplet for which $a + b + c = 1000$.

**Find the product $abc$.**

## Solution Overview

**Answer**: **31,875,000**

**The Triplet**: $(200, 375, 425)$

**Verification**:
- Pythagorean: $200^2 + 375^2 = 40000 + 140625 = 180625 = 425^2$ ✓
- Sum: $200 + 375 + 425 = 1000$ ✓
- Product: $200 \times 375 \times 425 = 31,875,000$ ✓

**Approach**: Constrained brute force search

**Performance**: ~45.5 μs (Criterion benchmark)

## Mathematical Analysis

### Problem Constraints

Given constraints:
1. $a < b < c$ (strictly increasing)
2. $a^2 + b^2 = c^2$ (Pythagorean condition)
3. $a + b + c = 1000$ (target sum)

### Deriving Search Bounds

**Bound on $a$**:

Since $a < b < c$ and all are positive:
$$a < b < c$$
$$a + a + a < a + b + c = 1000$$
$$3a < 1000$$
$$a < \frac{1000}{3} \approx 333.33$$

Therefore: $a \in [1, 333]$

**Bound on $b$ (given $a$)**:

From $a + b + c = 1000$:
$$c = 1000 - a - b$$

Since $b < c$:
$$b < 1000 - a - b$$
$$2b < 1000 - a$$
$$b < \frac{1000 - a}{2}$$

Also $b > a$ (given constraint).

Therefore: $b \in [a+1, \frac{1000-a}{2}]$

**Value of $c$**:

Once $a$ and $b$ are chosen:
$$c = 1000 - a - b$$

This is **deterministic** - no search needed for $c$.

### Search Space Reduction

**Naive approach**: Check all $(a, b, c)$ with $a < b < c$ and $a + b + c = 1000$
- Would need $O(n^3)$ iterations

**Optimized approach**: Use constraints to fix $c$
- Only need to iterate $(a, b)$
- $c$ is calculated, not searched
- Reduces to $O(n^2)$

**Further optimization**: Use tighter bounds
- $a < 333$ (not $a < 1000$)
- $b < (1000-a)/2$ (not $b < 1000$)
- Reduces constant factors significantly

### Complexity Analysis

**Time Complexity**: $O(n^2)$ where $n = 1000$

**Breakdown**:
```
Outer loop (a): 1 to 333 ≈ 333 iterations
Inner loop (b): a+1 to (1000-a)/2 ≈ (1000-a)/2 - a ≈ 500-1.5a iterations

Average inner loop: ~500 - 1.5×166.5 ≈ 250 iterations
Total operations: 333 × 250 ≈ 83,250 checks
```

**Actual performance**: ~45.5 μs
- Operations per check: multiplication, addition, comparison
- Modern CPU handles this extremely fast

**Space Complexity**: $O(1)$
- Only storing loop variables $a, b, c$
- No auxiliary data structures needed

## Alternative Approaches

### Approach 1: Brute Force (Implemented)

**Algorithm**:
```rust
for a in 1..target / 3 {
    for b in (a + 1)..=(target - a) / 2 {
        let c = target - a - b;
        if a * a + b * b == c * c {
            return (a, b, c);
        }
    }
}
```

**Pros**:
- ✅ Simple and straightforward
- ✅ Easy to understand and verify
- ✅ No special mathematical knowledge required
- ✅ Sufficient performance for $n = 1000$

**Cons**:
- ❌ $O(n^2)$ complexity
- ❌ Doesn't scale well to large $n$
- ❌ Checks many invalid combinations

### Approach 2: Euclid's Formula

**Algorithm** (using [[pythagorean-triplets]]):

All Pythagorean triplets can be generated as:
$$a = k(m^2 - n^2), \quad b = k(2mn), \quad c = k(m^2 + n^2)$$

where $m > n > 0$, $\gcd(m,n) = 1$, and $m-n$ is odd.

**Perimeter formula**:
$$P = a + b + c = k \cdot 2m(m + n)$$

For $P = 1000$:
$$k \cdot 2m(m + n) = 1000$$
$$k \cdot m(m + n) = 500$$

**Search strategy**:
```rust
for m in 2..=(500_f64.sqrt() as u32) {
    for n in 1..m {
        if gcd(m, n) == 1 && (m % 2 != n % 2) {
            let primitive_perimeter = 2 * m * (m + n);
            
            if 1000 % primitive_perimeter == 0 {
                let k = 1000 / primitive_perimeter;
                let a = k * (m * m - n * n);
                let b = k * (2 * m * n);
                let c = k * (m * m + n * n);
                return (a, b, c);
            }
        }
    }
}
```

**Pros**:
- ✅ More elegant mathematically
- ✅ Generates only valid Pythagorean triplets
- ✅ Better complexity: $O(\sqrt{n})$ range for $m$
- ✅ Demonstrates deeper understanding

**Cons**:
- ❌ More complex implementation
- ❌ Requires GCD computation
- ❌ Requires understanding of Euclid's formula
- ❌ For this specific problem, no practical advantage

**Complexity comparison**:
- Brute force: $\sim 333 \times 250 = 83,250$ operations
- Euclid: $\sim \sqrt{500} \times \sqrt{500} = 22 \times 22 = 484$ operations (with GCD overhead)

**For small problems** like this (target = 1000), both are effectively instant.

### Approach 3: Algebraic Optimization

From constraints:
$$c = 1000 - a - b$$
$$a^2 + b^2 = c^2 = (1000 - a - b)^2$$

Expanding:
$$a^2 + b^2 = 1000000 - 2000a - 2000b + a^2 + 2ab + b^2$$
$$0 = 1000000 - 2000a - 2000b + 2ab$$
$$2000(a + b) = 1000000 + 2ab$$
$$a + b = 500 + \frac{ab}{1000}$$

This gives a relationship between $a$ and $b$, but doesn't simplify the search significantly.

## Implementation Details

### Rust Implementation

**File**: `project_euler/src/problems/p009.rs`

**Key Design Decisions**:

1. **Type-safe struct**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PythagoreanTriplet {
    pub a: u32,
    pub b: u32,
    pub c: u32,
}
```

**Benefits**:
- Encapsulates the triplet concept
- Provides methods: `sum()`, `product()`
- Validation in constructor ensures invariants

2. **Validation in constructor**:
```rust
pub fn new(a: u32, b: u32, c: u32) -> Option<Self> {
    if a < b && b < c && a * a + b * b == c * c {
        Some(Self { a, b, c })
    } else {
        None
    }
}
```

**Benefits**:
- Guarantees valid triplets only
- Returns `Option` for explicit error handling
- Checks both ordering and Pythagorean condition

3. **Separate concerns**:
- `find_pythagorean_triplet(target)`: Generic solver
- `solve()`: Problem-specific (target = 1000)

**Benefits**:
- Reusable `find_pythagorean_triplet` for other problems
- Easy to test with different targets
- Clear separation of algorithm and problem instance

### Testing Strategy

**Unit Tests** (9 tests):
- ✅ Classic triplet creation: `(3, 4, 5)`
- ✅ Invalid triplet rejection: `(1, 2, 3)`, `(2, 3, 4)`
- ✅ Ordering validation: reject wrong order
- ✅ Find triplet for sum 12: expect `(3, 4, 5)`
- ✅ Find triplet for sum 30: expect `(5, 12, 13)`
- ✅ Find triplet for sum 24: expect `(6, 8, 10)`
- ✅ No triplet for sum 11: expect `None`
- ✅ Solve problem 9: expect `31875000`
- ✅ Verify properties of found triplet

**Integration Tests** (2 tests):
- ✅ Verify final answer: `31875000`
- ✅ Example case: sum 12 → product 60

**Edge Cases**:
- No solution exists (sum = 11)
- Multiple valid sums (12, 24, 30, etc.)
- Large target (1000)

## Triplet Analysis

### The Solution: (200, 375, 425)

**Is it primitive?**

Check $\gcd(200, 375, 425)$:
- $200 = 2^3 \times 5^2$
- $375 = 3 \times 5^3$
- $425 = 5^2 \times 17$
- $\gcd = 5^2 = 25$

**Not primitive!** It's a scaled triplet.

**Finding the primitive base**:
$$\frac{(200, 375, 425)}{25} = (8, 15, 17)$$

**Verify** $(8, 15, 17)$ is primitive:
- $8^2 + 15^2 = 64 + 225 = 289 = 17^2$ ✓
- $\gcd(8, 15, 17) = 1$ ✓ (primitive)

**Generating using Euclid's formula**:

For $(8, 15, 17)$, we need $m, n$ such that:
- $m^2 - n^2 = 8$ or $15$
- $2mn = 15$ or $8$
- $m^2 + n^2 = 17$

Try $m^2 + n^2 = 17$:
- $m = 4, n = 1$: $16 + 1 = 17$ ✓

Check:
- $a = 4^2 - 1^2 = 16 - 1 = 15$
- $b = 2 \times 4 \times 1 = 8$
- $c = 4^2 + 1^2 = 17$

**Result**: $(8, 15, 17)$ from $(m, n) = (4, 1)$ ✓

**For problem solution**:

Scale by $k = 25$:
- $a = 25 \times 15 = 375$
- $b = 25 \times 8 = 200$
- $c = 25 \times 17 = 425$

(Note: $a$ and $b$ swapped in our ordering convention)

**Final**: $(200, 375, 425) = 25 \times (8, 15, 17)$

**Perimeter check**:
$$P = 8 + 15 + 17 = 40$$
$$25 \times 40 = 1000$$ ✓

### Properties

**Area of triangle**:
$$A = \frac{1}{2} \times 200 \times 375 = 37,500$$

**Scaled area**:
$$A_{\text{primitive}} = \frac{1}{2} \times 8 \times 15 = 60$$
$$A = 25^2 \times 60 = 625 \times 60 = 37,500$$ ✓

**Inradius** (radius of inscribed circle):
$$r = \frac{a + b - c}{2} = \frac{200 + 375 - 425}{2} = \frac{150}{2} = 75$$

**Circumradius** (radius of circumscribed circle):
$$R = \frac{c}{2} = \frac{425}{2} = 212.5$$

## Key Insights

### 1. Constraint Exploitation

**Observation**: The problem gives us $a + b + c = 1000$, which is a **linear constraint**.

**Impact**: This allows us to eliminate one variable ($c$) from the search, reducing from 3D to 2D search space.

**General principle**: Always exploit given constraints to reduce search space.

### 2. Bound Optimization Matters

**Without bounds**:
```rust
for a in 1..1000 {
    for b in 1..1000 {
        let c = 1000 - a - b;
        if c > 0 && a < b && b < c && a*a + b*b == c*c {
            // Found it
        }
    }
}
```
Operations: $1000 \times 1000 = 1,000,000$

**With bounds**:
```rust
for a in 1..334 {
    for b in (a+1)..=(1000-a)/2 {
        let c = 1000 - a - b;
        if a*a + b*b == c*c {
            // Found it
        }
    }
}
```
Operations: $333 \times 250 \approx 83,250$ (12× reduction!)

**Lesson**: Mathematical analysis before coding pays off.

### 3. Problem Guarantees

**Statement**: "There exists **exactly one** Pythagorean triplet..."

**Implication**: We can return immediately upon finding **any** solution.

**Code**:
```rust
if a * a + b * b == c * c {
    return Some(...);  // Early return - no need to check more
}
```

**Benefit**: Avoid unnecessary computation after solution found.

### 4. Integer vs Floating Point

**Temptation**: Use `sqrt()` to find $c$:
```rust
let c = ((a*a + b*b) as f64).sqrt() as u32;
if c*c == a*a + b*b { /* ... */ }
```

**Problem**: Floating-point precision issues!
- $\sqrt{180625} = 425.0$ (exact in this case)
- But for large numbers, rounding errors possible

**Better**: Integer-only arithmetic
```rust
let c = 1000 - a - b;
if a*a + b*b == c*c { /* ... */ }
```

**Lesson**: Prefer integer arithmetic when possible for exact results.

### 5. Type Safety

**Design**: Using dedicated `PythagoreanTriplet` struct

**Alternative**: Just return tuple `(u32, u32, u32)`

**Benefits of struct**:
- Self-documenting code
- Methods: `.sum()`, `.product()`
- Validation enforced
- Can't accidentally mix up with other tuples

**Rust advantage**: Zero-cost abstraction - struct has same performance as raw tuple.

## Performance Analysis

### Benchmark Results

```
Problem 9: time: [45.418 µs 45.557 µs 45.726 µs]
```

**Analysis**:
- Average: ~45.5 microseconds
- Very consistent (low standard deviation)
- 11 outliers (normal for system scheduling)

### Scaling Behavior

**Question**: How would this scale to larger targets?

**Answer**: $O(n^2)$ where $n$ is the target sum.

**Projections**:
- $n = 1,000$: ~45 μs (measured)
- $n = 10,000$: ~4.5 ms (100× slower, $n^2$ scaling)
- $n = 100,000$: ~450 ms (10,000× slower)
- $n = 1,000,000$: ~45 seconds (1,000,000× slower)

**Alternative (Euclid's formula)**: $O(n)$ scaling
- $n = 1,000,000$: ~45 ms (1000× improvement over brute force)

**Conclusion**: For large $n$, Euclid's formula becomes essential.

### Where Time Is Spent

**Per iteration**:
1. Calculate $c = 1000 - a - b$: ~1 CPU cycle
2. Calculate $a \times a$: ~1 cycle
3. Calculate $b \times b$: ~1 cycle
4. Calculate $c \times c$: ~1 cycle
5. Add $a^2 + b^2$: ~1 cycle
6. Compare with $c^2$: ~1 cycle

Total: ~6 cycles per iteration

**Iterations**: ~83,250

**Expected CPU cycles**: ~500,000

**At 3 GHz**: $\frac{500,000}{3,000,000,000} \approx 0.17$ μs

**Measured**: 45.5 μs

**Discrepancy**: Cache misses, branch misprediction, loop overhead, etc.

Still incredibly fast for human perception!

## Connections to Other Problems

### Project Euler

**Similar problems**:
- **Problem 39**: Find perimeter $p \leq 1000$ with maximum number of integer right triangle solutions
- **Problem 75**: Find number of perimeters $\leq 1,500,000$ with exactly one integer right triangle
- **Problem 86**: Cuboid route problem (3D Pythagorean-like)

### General Programming

**Pattern**: Constrained search with mathematical bounds
- Appears in: optimization problems, scheduling, resource allocation
- Key: Transform constraints into search space reduction

**Technique**: Exploiting problem guarantees
- "Exactly one solution" → early return
- "At most one" → return first found
- "All solutions" → continue searching

## Lessons Learned

### 1. Read Problem Carefully

**Key phrase**: "There exists **exactly one**..."

**Implication**: Don't need to find all solutions or verify uniqueness.

**Impact**: Simplifies code - can return immediately.

### 2. Mathematical Analysis Before Coding

**Process**:
1. Understand constraints
2. Derive bounds
3. Estimate complexity
4. Choose algorithm
5. Implement

**Benefit**: Avoided $O(n^3)$ naive approach through upfront analysis.

### 3. Test with Smaller Examples

**Strategy**: Test with sum = 12, 30, 24 before tackling 1000

**Benefits**:
- Verify algorithm correctness on known cases
- Debug with simpler numbers
- Build confidence in approach

### 4. Measure, Don't Guess

**Benchmark results**: ~45 μs (excellent performance)

**Lesson**: $O(n^2)$ isn't always "slow" - depends on constants and $n$.

For $n = 1000$, brute force is perfectly acceptable.

### 5. Code Clarity Over Premature Optimization

**Our choice**: Brute force with bounds

**Alternative**: Euclid's formula (faster asymptotically)

**Justification**:
- Simpler code (easier to understand and verify)
- Performance already excellent (45 μs << human perception)
- Premature optimization is root of all evil

**When to optimize**: When performance actually matters (large $n$ or real-time requirements).

## Summary

**Key Takeaways**:
1. Problem asks for product of Pythagorean triplet with sum = 1000
2. Answer: 31,875,000 from triplet (200, 375, 425)
3. Triplet is $25 \times$ scaled version of primitive $(8, 15, 17)$
4. Constrained search with bounds: $O(n^2)$, ~45 μs performance
5. Alternative Euclid's formula: $O(n)$ but more complex
6. Mathematical analysis reduces search space dramatically
7. Integer arithmetic avoids floating-point precision issues

**Algorithm Selection**:
- ✅ **For $n \leq 10,000$**: Constrained brute force (simple, fast enough)
- ✅ **For $n > 10,000$**: Euclid's formula (better scaling)

**Problem-Solving Strategy**:
1. Analyze constraints mathematically
2. Derive tight search bounds
3. Choose simplest sufficient algorithm
4. Implement with type safety
5. Test thoroughly with known cases
6. Benchmark to verify performance

## References

### Documentation
- Implementation: `project_euler/src/problems/p009.rs`
- Problem statement: `project_euler/Problem_Statements/p009.md`
- Tests: Unit tests in p009.rs, integration in `tests/solutions.rs`
- Benchmarks: `project_euler/benches/benchmarks.rs`

### Mathematical Background
- [[pythagorean-triplets]] - Complete theory and Euclid's formula
- [[number-theory-basics]] - Fundamental concepts
- [[constrained-search]] - Optimization technique

### Related Problems
- Project Euler #39 - Integer right triangles
- Project Euler #75 - Singular integer right triangles

## Tags
*Tags: #project-euler #pythagorean-triplets #constrained-search #number-theory #brute-force #optimization #algorithm-analysis*

---

**Created**: January 29, 2026  
**Last Updated**: January 29, 2026  
**Status**: Complete ✓
