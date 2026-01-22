# Project Euler Problem 1: Multiples of 3 or 5

**Solved**: January 22, 2026  
**Difficulty**: 5% (Easiest)  
**Category**: Number Theory, Arithmetic Series

## Problem Summary

Find the sum of all natural numbers below 1000 that are multiples of 3 or 5.

**Example**: Below 10, the multiples are {3, 5, 6, 9} with sum 23.

**Target**: Sum of multiples below 1000.

## Mathematical Concepts

### Primary Concepts
- **[[arithmetic-series]]** - Sum formula: $S = \frac{n(n+1)}{2}$ for natural numbers
  - Used to sum multiples efficiently in O(1) time
  - Formula: $\text{sum\_multiples}(d, L) = d \cdot \frac{n(n+1)}{2}$ where $n = \lfloor (L-1)/d \rfloor$
  
- **[[inclusion-exclusion]]** - Principle for counting union of sets
  - Avoids double-counting multiples of both 3 and 5
  - Formula: $|A \cup B| = |A| + |B| - |A \cap B|$

### Supporting Concepts
- **[[lcm-gcd]]** - Least Common Multiple to find overlap
  - Multiples of both 3 and 5 → multiples of LCM(3,5) = 15
- **[[divisibility]]** - Understanding multiples and remainders

## Solution Approach

### Naive Approach (O(n) - Not Used)
```rust
// Iterate through all numbers - inefficient
let mut sum = 0;
for i in 1..1000 {
    if i % 3 == 0 || i % 5 == 0 {
        sum += i;
    }
}
```
- Time: O(n) where n = 1000
- Unnecessary iteration

### Mathematical Approach (O(1) - Implemented) ✓

**Key Insight**: Use arithmetic series formula instead of iteration.

1. **Sum multiples of 3**: $3 + 6 + 9 + \ldots + 999 = 3(1 + 2 + 3 + \ldots + 333)$
   - Count: $n_3 = \lfloor 999/3 \rfloor = 333$
   - Sum: $3 \cdot \frac{333 \cdot 334}{2} = 166,833$

2. **Sum multiples of 5**: $5 + 10 + 15 + \ldots + 995 = 5(1 + 2 + 3 + \ldots + 199)$
   - Count: $n_5 = \lfloor 999/5 \rfloor = 199$
   - Sum: $5 \cdot \frac{199 \cdot 200}{2} = 99,500$

3. **Sum multiples of 15**: Avoid double-counting (overlap of 3 and 5)
   - Count: $n_{15} = \lfloor 999/15 \rfloor = 66$
   - Sum: $15 \cdot \frac{66 \cdot 67}{2} = 33,165$

4. **Apply inclusion-exclusion**:
   $$\text{Result} = 166,833 + 99,500 - 33,165 = 233,168$$

### Algorithm

```
function sum_multiples(divisor, limit):
    if divisor >= limit: return 0
    n = floor((limit - 1) / divisor)
    return divisor × n × (n + 1) / 2

function solve():
    sum_3 = sum_multiples(3, 1000)
    sum_5 = sum_multiples(5, 1000)
    sum_15 = sum_multiples(15, 1000)  // LCM(3, 5) = 15
    return sum_3 + sum_5 - sum_15
```

## Complexity Analysis

- **Time**: O(1) - Constant time arithmetic operations
- **Space**: O(1) - No additional memory allocation

**Comparison**:
- Naive iteration: O(n) = O(1000) operations
- Mathematical formula: O(1) = 3 arithmetic operations

**Speedup**: ~333× faster for this input size!

## Rust Implementation

See `project_euler/src/problems/p001.rs` for complete code.

### Key Function: `sum_multiples()`
```rust
pub fn sum_multiples(divisor: u64, limit: u64) -> u64 {
    if divisor >= limit {
        return 0;
    }
    let n = (limit - 1) / divisor;
    divisor * n * (n + 1) / 2
}
```

**Type Safety**: Uses `u64` to prevent overflow and negative values.

### Main Solver
```rust
pub fn solve() -> u64 {
    const LIMIT: u64 = 1000;
    let sum_3 = sum_multiples(3, LIMIT);
    let sum_5 = sum_multiples(5, LIMIT);
    let sum_15 = sum_multiples(15, LIMIT);
    sum_3 + sum_5 - sum_15  // Inclusion-exclusion
}
```

### Test Coverage
- ✅ Example test (below 10 = 23)
- ✅ Unit test for `sum_multiples()` helper
- ✅ Final answer verification (233,168)

## Related Problems

### Project Euler
- *Problem 2*: Fibonacci series (also uses mathematical formula vs iteration)
- *Problem 5*: LCM of range 1-20 (related LCM concept)

### Advent of Code
- [[aoc-2023-day04]] - Set intersections (similar inclusion-exclusion)
- [[aoc-2023-day09]] - Sequence extrapolation (arithmetic series)

### Missions
- [[mission-5]] - HashMap implementation (used for counting in naive approach)
- *Future Mission 12*: Sequence analysis library

## Learning Insights

### Mathematical Thinking
1. **Recognize patterns**: Multiples form arithmetic sequences
2. **Use formulas**: Avoid brute force when closed-form solutions exist
3. **Handle overlaps**: Inclusion-exclusion prevents double-counting

### Rust Idioms
1. **Type safety**: `u64` prevents negative/overflow issues
2. **Const generics**: Could parameterize for any limit
3. **Pure functions**: No side effects, easy to test
4. **Documentation**: Extensive examples and complexity notes

### Performance Mindset
- **Asymptotic analysis**: O(1) vs O(n) matters at scale
- **Mathematical optimization**: Formula beats iteration
- **Reusability**: `sum_multiples()` works for any divisor/limit

## Answer

**233,168**

## Verification

```
Multiples of 3: 333 numbers, sum = 166,833
Multiples of 5: 199 numbers, sum = 99,500
Multiples of 15: 66 numbers, sum = 33,165
Result: 166,833 + 99,500 - 33,165 = 233,168 ✓
```

## References

- [Project Euler Problem 1](https://projecteuler.net/problem=1)
- *Concrete Mathematics* - Summation formulas
- [[arithmetic-series]] - Foundational concept
- [[inclusion-exclusion]] - Set theory principle

---

*Links:*
- **Problem**: `project_euler/Problem_Statements/p001.md`
- **Solution**: `project_euler/src/problems/p001.rs`
- **Concepts**: [[arithmetic-series]], [[inclusion-exclusion]], [[lcm-gcd]]
- **Related**: [[aoc-2023-day04]], [[aoc-2023-day09]]
- **Tags**: #project-euler #easy #number-theory #arithmetic-series #inclusion-exclusion
