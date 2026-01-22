# Inclusion-Exclusion Principle

**Category**: Set Theory, Combinatorics  
**Prerequisites**: Basic set theory

## Definition

The **inclusion-exclusion principle** is a counting technique that computes the size of a union of sets by adding individual set sizes and subtracting overlaps.

### Two Sets
For sets $A$ and $B$:
$$|A \cup B| = |A| + |B| - |A \cap B|$$

**Intuition**: Adding $|A|$ and $|B|$ counts elements in the intersection twice, so subtract once.

### Three Sets
For sets $A$, $B$, and $C$:
$$|A \cup B \cup C| = |A| + |B| + |C| - |A \cap B| - |A \cap C| - |B \cap C| + |A \cap B \cap C|$$

**Pattern**: Alternating signs - add individual sets, subtract pairwise intersections, add three-way intersection.

### General Form (n sets)
$$\left|\bigcup_{i=1}^{n} A_i\right| = \sum_{k=1}^{n} (-1)^{k+1} \sum_{|S|=k} \left|\bigcap_{i \in S} A_i\right|$$

Where the inner sum ranges over all subsets $S$ of size $k$.

## Venn Diagram

For two sets:
```
    A           B
  ┌───────────┬───────────┐
  │           │           │
  │     A-B   │  A∩B      │   B-A
  │           │           │
  └───────────┴───────────┘
```

Count: $|A-B| + |A \cap B| + |B-A|$ = $|A| + |B| - |A \cap B|$

## Key Applications

### 1. Counting Multiples

**Problem**: Count numbers divisible by 3 OR 5 below 1000.

**Solution**:
- $A$ = multiples of 3: $|A| = \lfloor 999/3 \rfloor = 333$
- $B$ = multiples of 5: $|B| = \lfloor 999/5 \rfloor = 199$
- $A \cap B$ = multiples of 15 (LCM): $|A \cap B| = \lfloor 999/15 \rfloor = 66$
- Result: $|A \cup B| = 333 + 199 - 66 = 466$

### 2. Derangements

Count permutations where no element is in its original position.

### 3. Euler's Totient Function

Count integers $\leq n$ coprime to $n$ using prime factorization.

### 4. Sieve Patterns

Sieve of Eratosthenes uses inclusion-exclusion implicitly.

## Complexity Analysis

- **Naive union counting**: O(|A| + |B|) - iterate all elements
- **Inclusion-exclusion**: O(1) - if set sizes are known

For $n$ sets, general formula requires $2^n - 1$ terms (expensive for large $n$).

## Common Patterns

### LCM for Overlaps
When counting multiples of $a$ and $b$, the overlap is multiples of $\text{lcm}(a, b)$.

**Example**: 
- Multiples of 3 and 5 → overlap = multiples of 15
- Multiples of 6 and 9 → overlap = multiples of 18

### Boolean Logic Connection
$$|A \cup B| = |A| + |B| - |A \cap B|$$
corresponds to:
$$P(A \text{ OR } B) = P(A) + P(B) - P(A \text{ AND } B)$$

## Rust Implementations

### Project Euler
- [[project-euler-p001]] - Multiples of 3 or 5
  - Implementation: `project_euler/src/problems/p001.rs::solve()`
  - Pattern: `sum_3 + sum_5 - sum_15` (LCM of 3 and 5 = 15)

### Advent of Code
- [[aoc-2023-day04]] - Scratchcard matching (set intersections)
- [[aoc-2024-day08]] - Antinode counting with overlapping frequencies

### Missions
*(Potential Mission 13+ topic: set operations library)*

## Related Concepts

- [[set-theory-fundamentals]] - Basic set operations
- [[arithmetic-series]] - Often combined for counting multiples
- [[number-theory]] - LCM/GCD for finding overlaps
- [[combinatorics]] - Counting without overcounting

## Examples

### Example 1: Two Sets
Students taking Math (30), Science (25), both (10):
- Math OR Science = 30 + 25 - 10 = 45 students

### Example 2: Three Sets (Multiples of 2, 3, 5 below 100)
- $|A|$ = 50 (multiples of 2)
- $|B|$ = 33 (multiples of 3)
- $|C|$ = 20 (multiples of 5)
- $|A \cap B|$ = 16 (multiples of 6)
- $|A \cap C|$ = 10 (multiples of 10)
- $|B \cap C|$ = 6 (multiples of 15)
- $|A \cap B \cap C|$ = 3 (multiples of 30)

Result: $50 + 33 + 20 - 16 - 10 - 6 + 3 = 74$

### Example 3: Avoiding Double-Counting in Sums
Sum multiples of 4 or 6 below 50:
```rust
sum_4 = 4 * (1 + 2 + ... + 12) = 4 * 78 = 312
sum_6 = 6 * (1 + 2 + ... + 8) = 6 * 36 = 216
sum_12 = 12 * (1 + 2 + 3 + 4) = 12 * 10 = 120
result = 312 + 216 - 120 = 408
```

## Visual Proof (Two Sets)

```
Without inclusion-exclusion (wrong):
A: ████████████ (count A)
B:       ████████████ (count B)
         ^^^^^ counted twice!

With inclusion-exclusion (correct):
|A| + |B| - |A∩B| = unique elements
```

## References

- *Concrete Mathematics* - Graham, Knuth, Patashnik (Chapter 2)
- *Introduction to Combinatorics* - Richard A. Brualdi
- [Mathworld: Inclusion-Exclusion Principle](https://mathworld.wolfram.com/Inclusion-ExclusionPrinciple.html)

---

*Links:*
- Related Problems: [[project-euler-p001]], [[aoc-2023-day04]], [[aoc-2024-day08]]
- Related Concepts: [[set-theory-fundamentals]], [[arithmetic-series]], [[lcm-gcd]]
- Tags: #set-theory #combinatorics #counting #principle
