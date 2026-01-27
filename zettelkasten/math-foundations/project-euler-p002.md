# Project Euler Problem 2: Even Fibonacci Numbers

**Solved**: January 26, 2026  
**Difficulty**: 5% (Easiest)  
**Category**: Fibonacci Sequence, Recurrence Relations

## Problem Summary

Find the sum of all even-valued Fibonacci numbers not exceeding 4,000,000.

**Example**: Fibonacci sequence 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
- Even terms: {2, 8, 34}
- Pattern: Every third Fibonacci number is even

**Target**: Sum of even Fibonacci terms ≤ 4,000,000.

## Mathematical Concepts

### Primary Concepts
- **[[fibonacci-sequence]]** - Recurrence relation: $F_n = F_{n-1} + F_{n-2}$
  - Classical sequence: 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
  - Every third term is even (positions 3, 6, 9, 12, ...)
  
- **[[recurrence-relations]]** - Mathematical sequences defined recursively
  - Standard Fibonacci: $F_n = F_{n-1} + F_{n-2}$ with $F_1=1, F_2=1$
  - **Even Fibonacci recurrence**: $E_n = 4E_{n-1} + E_{n-2}$ with $E_1=2, E_2=8$
  - Derived by substituting into original recurrence

### Supporting Concepts
- **[[modular-arithmetic]]** - Parity analysis (even/odd)
  - Pattern: odd + odd = even, even + odd = odd, odd + even = odd
  - Fibonacci parity: O, E, O, O, E, O, O, E, ... (every 3rd is even)
- **[[sequence-analysis]]** - Pattern recognition in sequences

## Solution Approach

### Naive Approach (O(n) iterations - Not Used)
```rust
// Generate all Fibonacci, filter evens
let mut sum = 0;
let (mut a, mut b) = (1, 2);
while b <= 4_000_000 {
    if b % 2 == 0 {
        sum += b;
    }
    let next = a + b;
    a = b;
    b = next;
}
```
- Time: O(n) where n ≈ 33 Fibonacci numbers up to 4M
- Unnecessary parity checks every iteration

### Mathematical Approach (O(k) even terms - Implemented) ✓

**Key Insight**: Derive recurrence relation for even Fibonacci numbers only.

#### Pattern Discovery
Fibonacci sequence with indices:
```
n:   1  2  3  4  5   6   7   8   9   10  11  12
F_n: 1  2  3  5  8  13  21  34  55  89 144 233
     O  E  O  O  E   O   O   E   O   O   E   O
```

**Observation**: Even terms appear at positions 3, 6, 9, 12, ... (multiples of 3)

#### Deriving Even Recurrence

Let $E_1 = F_3 = 2$, $E_2 = F_6 = 8$, $E_3 = F_9 = 34$, ...

To find $E_{n+1}$ from $E_n$ and $E_{n-1}$:

Starting with standard Fibonacci:
- $F_{3k} = F_{3k-1} + F_{3k-2}$
- $F_{3k-1} = F_{3k-2} + F_{3k-3}$
- $F_{3k-2} = F_{3k-3} + F_{3k-4}$

Through substitution (expand $F_{3(k+1)}$ in terms of $F_{3k}$ and $F_{3(k-1)}$):

$$E_{n+1} = F_{3(k+1)} = 4 \cdot F_{3k} + F_{3(k-1)} = 4E_n + E_{n-1}$$

**Recurrence**: $E_n = 4E_{n-1} + E_{n-2}$ with $E_1=2, E_2=8$

#### Verification
- $E_1 = 2$
- $E_2 = 8$
- $E_3 = 4(8) + 2 = 34$ ✓ (matches $F_9$)
- $E_4 = 4(34) + 8 = 144$ ✓ (matches $F_{12}$)
- $E_5 = 4(144) + 34 = 610$ ✓

### Algorithm

```
function sum_even_fib(limit):
    if limit < 2: return 0
    
    sum = 2          // E_1 = 2
    e_prev = 2       // E_1
    e_curr = 8       // E_2
    
    while e_curr <= limit:
        sum += e_curr
        next = 4 * e_curr + e_prev  // Even Fibonacci recurrence
        e_prev = e_curr
        e_curr = next
    
    return sum
```

## Complexity Analysis

- **Time**: O(k) where k = number of even Fibonacci terms ≤ limit
  - For 4,000,000: k ≈ 11 iterations
  - Fibonacci grows exponentially: $F_n \approx \phi^n / \sqrt{5}$ where $\phi = \frac{1+\sqrt{5}}{2}$
  - Thus k ≈ O(log limit)
- **Space**: O(1) - Only three variables needed

**Comparison**:
- Naive (all Fibonacci): O(n) ≈ 33 iterations, parity check each time
- Even recurrence: O(k) ≈ 11 iterations, no parity checks
- **Speedup**: ~3× fewer iterations + no modulo operations

## Rust Implementation

See `project_euler/src/problems/p002.rs` for complete code.

### Key Function: `sum_even_fib()`
```rust
pub fn sum_even_fib(limit: u64) -> u64 {
    if limit < 2 {
        return 0;
    }
    // Even Fibonacci seed terms: E0=2 (F3), E1=8 (F6)
    let mut sum = 2u64;      // Include first even term
    let mut e_prev = 2u64;   // E_1
    let mut e_curr = 8u64;   // E_2

    while e_curr <= limit {
        sum += e_curr;
        let next = 4 * e_curr + e_prev;  // E_n = 4*E_{n-1} + E_{n-2}
        e_prev = e_curr;
        e_curr = next;
    }

    sum
}
```

**Type Safety**: Uses `u64` for large Fibonacci numbers (F_50 ≈ 10^10).

**Edge Cases**:
- `limit < 2`: Returns 0 (no even Fibonacci numbers)
- `limit == 2`: Returns 2 (only first even term)
- Large limits: u64 sufficient for F_93 ≈ 10^19

### Main Solver
```rust
pub fn solve() -> u64 {
    sum_even_fib(4_000_000)
}
```

### Test Coverage
- ✅ Small limits (8 → sum=10, 34 → sum=44)
- ✅ Final answer verification (4,613,732)
- ✅ Edge case handling (limit < 2)

## Related Problems

### Project Euler
- [[project-euler-p001]] - Arithmetic series (also uses mathematical formula vs iteration)
- *Problem 25*: First Fibonacci with 1000 digits
- *Problem 144*: Fibonacci modulo arithmetic

### Advent of Code
- [[aoc-2023-day09]] - Sequence extrapolation (pattern recognition)
- *AoC 2015 Day 20*: Infinite house delivery (sequence generation)

### Missions
- *Future Mission 11*: Dynamic programming library (Fibonacci memoization)
- *Future Mission 12*: Sequence analysis utilities

## Learning Insights

### Mathematical Thinking
1. **Pattern recognition**: Identify even terms occur every 3rd position
2. **Derive recurrences**: Transform problem to simpler recurrence relation
3. **Verification**: Always verify derived formulas with examples
4. **Asymptotic growth**: Fibonacci grows exponentially → logarithmic iterations

### Rust Idioms
1. **Explicit types**: `2u64` prevents type inference ambiguity
2. **Early returns**: Handle edge cases upfront
3. **Variable naming**: `e_prev`, `e_curr` clearly show recurrence relationship
4. **Const parameters**: Could generalize with `sum_even_fib<const LIMIT: u64>()`

### Performance Mindset
- **Mathematical optimization**: Derived recurrence eliminates 67% of work
- **Avoid unnecessary operations**: No modulo checks in inner loop
- **Constant factors**: 4*x + y is cheaper than general Fibonacci iteration

### Comparison with Python Solution
Your original Python (`ps2.py`):
```python
fib = []
for i in range(2, 100):
    fibnext = int(fib[i-1] + fib[i-2])
    if fibnext > 4000000:
        break
    fib.append(fibnext)
    if fibnext % 2 == 0:
        sum += fibnext
```

**Issues**:
- ❌ Stores entire sequence in memory (unnecessary)
- ❌ Shadows builtin `sum`
- ❌ Checks parity every iteration
- ❌ Uses list indexing (slower than variables)

**Rust improvements**:
- ✅ O(1) space (no array storage)
- ✅ Even recurrence (no parity checks)
- ✅ Type safety (u64 explicit)
- ✅ 3× fewer iterations

## Answer

**4,613,732**

## Verification

```
Even Fibonacci sequence up to 4,000,000:
E_1  = 2
E_2  = 8
E_3  = 34
E_4  = 144
E_5  = 610
E_6  = 2,584
E_7  = 10,946
E_8  = 46,368
E_9  = 196,418
E_10 = 832,040
E_11 = 3,524,578

Sum = 2 + 8 + 34 + 144 + 610 + 2,584 + 10,946 + 46,368 + 196,418 + 832,040 + 3,524,578
    = 4,613,732 ✓
```

Next even term: $E_{12} = 4(3,524,578) + 832,040 = 14,930,352 > 4,000,000$ (stop)

## Mathematical Derivation Detail

### Why Every Third Fibonacci is Even

Parity sequence analysis:
- $F_1 = 1$ (O)
- $F_2 = 1$ (O)
- $F_3 = 1 + 1 = 2$ (E) — O + O = E
- $F_4 = 2 + 1 = 3$ (O) — E + O = O
- $F_5 = 3 + 2 = 5$ (O) — O + E = O
- $F_6 = 5 + 3 = 8$ (E) — O + O = E
- $F_7 = 8 + 5 = 13$ (O) — E + O = O
- $F_8 = 13 + 8 = 21$ (O) — O + E = O
- $F_9 = 21 + 13 = 34$ (E) — O + O = E

**Pattern**: O, O, E, O, O, E, O, O, E, ...

**Proof by induction**: If positions k and k+1 are odd, then k+2 is even (O+O=E), k+3 is odd (E+O=O), k+4 is odd (O+E=O), k+5 is even (O+O=E). Pattern repeats every 3 terms.

## References

- [Project Euler Problem 2](https://projecteuler.net/problem=2)
- *Concrete Mathematics* by Graham, Knuth, Patashnik - Chapter 6 (Recurrences)
- [[fibonacci-sequence]] - Foundational concept
- [[recurrence-relations]] - Mathematical framework

---

*Links:*
- **Problem**: `project_euler/Problem_Statements/p002.md`
- **Solution**: `project_euler/src/problems/p002.rs`
- **Concepts**: [[fibonacci-sequence]], [[recurrence-relations]], [[modular-arithmetic]]
- **Related**: [[project-euler-p001]], [[aoc-2023-day09]]
- **Tags**: #project-euler #easy #fibonacci #recurrence-relations #sequence-analysis
