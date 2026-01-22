# Arithmetic Series

**Category**: Algebra, Sequences  
**Prerequisites**: Basic algebra

## Definition

An **arithmetic series** is the sum of terms in an arithmetic sequence, where each term differs from the next by a constant difference.

### Arithmetic Sequence
A sequence where consecutive terms have a constant difference $d$:
$$a_1, a_2, a_3, \ldots, a_n$$
where $a_{i+1} = a_i + d$

### Arithmetic Series Formula
The sum of the first $n$ terms:
$$S_n = \frac{n(a_1 + a_n)}{2}$$

**Alternative form** (using first term and difference):
$$S_n = \frac{n}{2}[2a_1 + (n-1)d]$$

**Special case** (sum of first $n$ natural numbers):
$$\sum_{i=1}^{n} i = \frac{n(n+1)}{2}$$

## Derivation

Consider the series written forwards and backwards:
```
S = 1 + 2 + 3 + ... + (n-1) + n
S = n + (n-1) + (n-2) + ... + 2 + 1
```

Adding both lines:
```
2S = (n+1) + (n+1) + (n+1) + ... + (n+1) + (n+1)
2S = n(n+1)
S = n(n+1)/2
```

## Key Properties

1. **Symmetry**: Order doesn't matter - sum is the same
2. **Average**: $S_n = n \times \text{average}$ where average = $(a_1 + a_n)/2$
3. **Linearity**: Sum of multiples: $\sum_{i=1}^{n} k \cdot i = k \cdot \frac{n(n+1)}{2}$

## Common Applications

### Sum of Multiples
To find sum of multiples of $d$ below limit $L$:

1. Find count: $n = \lfloor (L-1) / d \rfloor$
2. Apply formula: $S = d \cdot \frac{n(n+1)}{2}$

**Example**: Sum of multiples of 3 below 10:
- Multiples: 3, 6, 9 → $n = 3$
- Sum: $3 \cdot \frac{3 \cdot 4}{2} = 3 \cdot 6 = 18$ ✓

### Triangular Numbers
The $n$-th triangular number $T_n = \frac{n(n+1)}{2}$

Represents dots forming an equilateral triangle:
```
n=1:  •           T₁ = 1
n=2:  • •         T₂ = 3
      •
n=3:  • • •       T₃ = 6
      • •
      •
```

## Complexity Analysis

- **Iterative approach**: O(n) time to sum all terms
- **Formula approach**: O(1) time - constant time calculation

## Rust Implementations

### Project Euler
- [[project-euler-p001]] - Sum of multiples of 3 or 5 using series formula
  - Implementation: `project_euler/src/problems/p001.rs::sum_multiples()`
  - Key insight: O(1) instead of O(n) iteration

### Advent of Code
- [[aoc-2023-day09]] - Extrapolating sequences (OASIS report)
  - Uses differences to identify arithmetic progressions

### Mission Implementations
*(None yet - potential Mission 12+ topic: sequence analysis)*

## Related Concepts

- [[geometric-series]] - Series with constant ratio (vs constant difference)
- [[inclusion-exclusion]] - Often used with arithmetic series for set problems
- [[number-theory]] - Divisibility and multiples
- [[summation-notation]] - Sigma notation $\sum$

## Examples

### Example 1: First 100 Natural Numbers
$$\sum_{i=1}^{100} i = \frac{100 \cdot 101}{2} = 5050$$

### Example 2: Even Numbers 2 to 20
$$2 + 4 + 6 + \ldots + 20 = 2(1 + 2 + 3 + \ldots + 10) = 2 \cdot \frac{10 \cdot 11}{2} = 110$$

### Example 3: Multiples of 7 Below 100
- Count: $n = \lfloor 99/7 \rfloor = 14$
- Sum: $7 \cdot \frac{14 \cdot 15}{2} = 7 \cdot 105 = 735$

## Historical Note

The story of young **Carl Friedrich Gauss** (age 7-10) being asked to sum 1 to 100 and immediately recognizing the pattern $(1+100) + (2+99) + \ldots = 50 \times 101 = 5050$ demonstrates the power of this insight.

## References

- *The Art of Computer Programming* Vol 1 - Knuth (summation formulas)
- [OEIS A000217](https://oeis.org/A000217) - Triangular numbers
- [Mathworld: Arithmetic Series](https://mathworld.wolfram.com/ArithmeticSeries.html)

---

*Links:*
- Related Problems: [[project-euler-p001]], [[aoc-2023-day09]]
- Related Concepts: [[inclusion-exclusion]], [[number-theory]], [[geometric-series]]
- Tags: #algebra #sequences #summation #number-theory
