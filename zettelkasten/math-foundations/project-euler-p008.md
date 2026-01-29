# Project Euler Problem 8: Largest Product in a Series

**Difficulty**: Easy (5%)  
**Published**: January 11, 2002  
**Solved**: January 29, 2026  
**Category**: Array Processing, Sliding Window  
**Related Concepts**: [[sliding-window-pattern]], [[array-algorithms]]

## Problem Statement

The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.

Find the **thirteen adjacent digits** in the 1000-digit number that have the **greatest product**.

**Example** (4 adjacent):
- Digits: 9, 9, 8, 9
- Product: $9 \times 9 \times 8 \times 9 = 5832$

**Find**: Largest product of 13 consecutive digits in the 1000-digit number

## Solution Overview

**Answer**: **23,514,624,000**

**Approach**: Sliding window with zero-skipping optimization

**Performance**: ~2.97 μs (Criterion benchmark)

## Mathematical Analysis

### Sliding Window Pattern

**Definition**: Process contiguous subsequences of a fixed size by "sliding" through the array.

**General Algorithm**:
1. Initialize window at position 0 with size $k$
2. Process current window (calculate product, sum, etc.)
3. Slide window one position right (drop leftmost, add rightmost)
4. Repeat until window reaches end of array
5. Return maximum/minimum/aggregate value

**Complexity**:
- **Time**: $O(n \times k)$ where $n$ = array length, $k$ = window size
  - For Problem 8: $O(1000 \times 13) = O(13000)$ operations
- **Space**: $O(n)$ for storing digits (or $O(1)$ if streaming)

### Optimization: Zero-Skipping

**Key Insight**: Any window containing a zero has product = 0

**Strategy**:
```rust
if window.contains(&0) {
    continue;  // Skip this window entirely
}
```

**Benefit**: Reduces redundant multiplications
- Without optimization: Calculate product even when result is 0
- With optimization: Skip entire window in O(k) check
- Trade-off: Check adds overhead but saves multiplication chain

### Alternative Approaches

**1. Naive Brute Force**:
```
for each position i from 0 to n-k:
    product = 1
    for j from i to i+k:
        product *= digits[j]
    max_product = max(max_product, product)
```
- Time: $O(n \times k)$ - multiply k digits per window
- Simple but doesn't exploit structure

**2. Rolling Product** (not used due to complexity):
```
Initial window product = ∏(digits[0..k])
For next window:
    product = (product / digits[i]) * digits[i+k]
```
- **Problem**: Division by zero! (and floating-point precision issues)
- Not viable when zeros present in data

**3. Zero-Aware Segmentation**:
```
Split array at zeros → [segment1] 0 [segment2] 0 [segment3]
For each segment:
    Apply sliding window if length ≥ k
Return global maximum
```
- **Complexity**: More code, marginal benefit for this problem size
- **Used when**: Very large arrays with many zeros

## Implementation Details

### Rust Implementation

**File**: `project_euler/src/problems/p008.rs`

**Key Functions**:

```rust
pub fn largest_product_in_series(number: &str, window_size: usize) -> u64
```
- **Input**: String of digits, window size
- **Output**: Maximum product found
- **Algorithm**: 
  1. Parse string to `Vec<u64>` digits
  2. Slide window across positions
  3. Skip windows with zeros
  4. Calculate product for valid windows
  5. Track maximum

**Critical Design Choices**:

1. **Data Type**: `u64` for digits and products
   - Products grow large: $9^{13} = 2,541,865,828,329$ (fits in u64)
   - Answer: 23,514,624,000 < u64::MAX ✓

2. **Zero Handling**: `window.contains(&0)`
   - Linear scan: $O(k)$ per window
   - Alternative: Track zero positions (more complex, negligible benefit)

3. **Product Calculation**: `window.iter().product::<u64>()`
   - Uses Rust iterator fold internally
   - Clean, idiomatic, compiler-optimized

### Testing Strategy

**Unit Tests**:
- ✅ Example case: 4 adjacent = 5832
- ✅ Simple cases: 1, 2, 3 digit windows
- ✅ Zero handling: Zeros don't affect max
- ✅ Edge cases: Window too large, all zeros
- ✅ Solution validation: 13 adjacent = 23514624000

**Performance Validation**:
```
Benchmark: ~2.97 μs
- Input: 1000 digits
- Windows: ~988 (1000 - 13 + 1)
- Operations: ~12,844 multiplications (worst case)
- Per-window avg: ~3 ns (incredibly fast!)
```

## Key Insights

### 1. Pattern Recognition

**Sliding Window** appears frequently in:
- **String processing**: Substring search, anagram detection
- **Time series**: Moving averages, signal processing
- **AoC Problems**: Many grid/sequence problems
- **Real systems**: Rate limiting, metrics aggregation

### 2. Optimization Trade-offs

**Zero-skipping overhead**:
- Check: $O(k)$ linear scan per window
- Benefit: Skip $k$ multiplications
- **Worth it?** Only if zeros are common enough

**Measurement** (approximate):
- Contains check: ~13 comparisons
- Product calculation: ~13 multiplications
- **Trade-off**: Roughly equal cost!

**Why still beneficial?**
- Short-circuits on first zero found (avg case: 6.5 comparisons)
- Avoids unnecessary product accumulation
- Clearer code intent (explicit zero handling)

### 3. Algorithm Complexity Analysis

**Time Complexity Breakdown**:
```
n = 1000 digits
k = 13 window size

Outer loop: (n - k + 1) = 988 iterations
Inner operations per iteration:
  - Contains check: O(k) = 13 comparisons (worst)
  - Product calc: O(k) = 13 multiplications (if no zero)

Total: O(n × k) = O(1000 × 13) = O(13,000)
```

**Space Complexity**:
- Digit array: $O(n)$ = 1000 elements
- Window reference: $O(1)$ (slice, not copy)
- Accumulator: $O(1)$
- **Total**: $O(n)$ dominated by input storage

### 4. Mathematical Properties

**Product Monotonicity**:
- Adding digit $d > 1$: Product increases
- Adding digit $d = 1$: Product unchanged  
- Adding digit $d = 0$: Product becomes 0

**Implication**: Look for windows with:
- Largest possible digits (9s are best)
- No zeros (instant disqualification)
- No ones (better to have larger digits)

**Optimal theoretical window**: Thirteen 9s
- Product: $9^{13} = 2,541,865,828,329$
- Actual answer: $23,514,624,000$ (much smaller)
- **Why?** The 1000-digit number doesn't contain thirteen consecutive 9s

## Connections to Other Concepts

### Related Patterns

**Sliding Window Variants**:
- [[two-pointer-technique]] - Dynamic window sizing
- [[kadanes-algorithm]] - Maximum subarray sum (DP variant)
- [[rabin-karp]] - String hashing with sliding window
- [[minimum-window-substring]] - Variable size window optimization

### Computational Techniques

**Array Processing**:
- [[prefix-sums]] - Cumulative aggregation (for sums)
- [[sparse-table]] - Range query optimization (for min/max)
- [[segment-tree]] - Dynamic range queries
- [[monotonic-queue]] - Sliding window minimum/maximum

### Project Euler Connections

**Related Problems**:
- [[project-euler-p011]] - Largest product in grid (2D sliding window)
- [[project-euler-p013]] - Large sum (arbitrary precision)
- [[project-euler-p016]] - Power digit sum (digit manipulation)

## Lessons Learned

### 1. Simplicity Over Premature Optimization

**Initial thought**: "Should I optimize with rolling product?"
**Reality**: Division-by-zero makes it impractical
**Lesson**: Start simple, measure, then optimize if needed

### 2. Data Structure Choices Matter

**Choice**: `Vec<u64>` for digits
**Alternative**: Keep as string, parse on demand
**Trade-off**: 
- ✅ Vec: Faster access, clearer arithmetic
- ❌ String: Lower memory, more parsing overhead

**Decision**: Vec wins for this problem (only 1000 digits)

### 3. Test-Driven Problem Solving

**Workflow**:
1. Write test for example (4 adjacent = 5832) ✓
2. Implement minimal solution
3. Write edge case tests (zeros, too short)
4. Refactor with confidence

**Benefit**: Caught off-by-one errors in window indexing early

## Code Examples

### Core Implementation

```rust
pub fn largest_product_in_series(number: &str, window_size: usize) -> u64 {
    // Convert string to digit array
    let digits: Vec<u64> = number
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u64)
        .collect();

    // Handle edge case: number too short
    if digits.len() < window_size {
        return 0;
    }

    let mut max_product = 0u64;

    // Slide window across all valid positions
    for window_start in 0..=digits.len() - window_size {
        let window = &digits[window_start..window_start + window_size];

        // Optimization: skip windows containing zeros
        if window.contains(&0) {
            continue;
        }

        // Calculate product of current window
        let product = window.iter().product::<u64>();

        if product > max_product {
            max_product = product;
        }
    }

    max_product
}
```

### Alternative: Functional Style

```rust
pub fn largest_product_functional(number: &str, window_size: usize) -> u64 {
    number
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>()
        .windows(window_size)
        .filter(|w| !w.contains(&0))
        .map(|w| w.iter().map(|&d| d as u64).product())
        .max()
        .unwrap_or(0)
}
```

**Comparison**:
- ✅ More concise, declarative
- ❌ Potentially harder to debug
- ⚖️ Performance: Similar (compiler optimizes both)

## References

### Documentation
- Implementation: `project_euler/src/problems/p008.rs`
- Problem statement: `project_euler/Problem_Statements/p008.md`
- Tests: Unit tests in p008.rs, integration in `tests/solutions.rs`

### Mathematical Background
- Sliding window: Classic algorithms textbook pattern
- Product overflow: Consider when $k$ large or digits not bounded

### Related Zettelkasten Notes
- [[sliding-window-pattern]] - General technique documentation
- [[array-algorithms]] - Array processing patterns
- [[project-euler-p011]] - Extension to 2D grids

## Tags
*Tags: #project-euler #sliding-window #array-processing #optimization #algorithm-patterns #problem-solving*

---

**Created**: January 29, 2026  
**Last Updated**: January 29, 2026  
**Status**: Complete ✓
