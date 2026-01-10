# Day 9: Mirage Maintenance - Function Guide

**Problem**: AoC 2023 Day 9 - Polynomial sequence extrapolation using finite differences

**Solution**: `advent_of_code/aoc2023/src/solver/day09.rs`

**Performance**: Part 1: 132µs | Part 2: 191µs | Total: 323µs

---

## 📋 Table of Contents

1. [Overview](#overview)
2. [Type Definitions](#type-definitions)
3. [Core Implementation](#core-implementation)
4. [Mathematical Algorithms](#mathematical-algorithms)
5. [Public API](#public-api)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Testing Strategy](#testing-strategy)
9. [Common Pitfalls](#common-pitfalls)
10. [Key Takeaways](#key-takeaways)
11. [Follow-Up Questions](#follow-up-questions)

---

## 🎯 Overview

### Problem Summary
The OASIS (Oasis And Sand Instability Sensor) produces sequences of integer values. We need to:
- **Part 1**: Extrapolate the next value for each sequence
- **Part 2**: Extrapolate the previous value for each sequence

### Example
```
Input sequence: 0 3 6 9 12 15

Difference pyramid:
  0   3   6   9  12  15    ← Level 0 (original)
    3   3   3   3   3      ← Level 1 (constant differences!)
      0   0   0   0        ← Level 2 (all zeros - stop)

Forward extrapolation: 15 + 3 = 18
Backward extrapolation: 0 - 3 = -3
```

### Key Insights
1. **Polynomial detection**: Sequences are polynomials with constant nth differences
2. **Recursive structure**: Difference pyramid naturally suggests recursion
3. **Bidirectional**: Same algorithm works forward/backward with sign change
4. **Efficiency**: Low-degree polynomials reach all-zeros quickly (typically 2-4 levels)

### Mathematical Foundation
**Finite Differences**: A method from numerical analysis dating back to Isaac Newton (1670s).
- Polynomial of degree n has constant nth differences
- Building difference pyramid detects polynomial degree automatically
- Extrapolation works because polynomial pattern continues

**See**: `zettelkasten/math-foundations/finite-differences.md` for complete theory

---

## 📦 Type Definitions

### No Custom Types!
This solution uses only built-in Rust types:
- `Vec<i64>` - Sequences of integers
- `&[i64]` - Slices for viewing sequences without ownership
- `i64` - Individual values (signed 64-bit integers)

**Design Decision**: No custom types needed because:
- Sequences are simple integer collections
- No state to encapsulate
- No behavior to attach to types
- Rust's slice operations provide everything needed

**Alternative approach** (more complex, not justified):
```rust
struct Sequence {
    values: Vec<i64>,
}

impl Sequence {
    fn differences(&self) -> Sequence { ... }
    fn extrapolate_next(&self) -> i64 { ... }
}
```
❌ **Why not**: Adds complexity without benefit. Direct functions on slices are clearer.

---

## 🔧 Core Implementation

### Function 1: `parse_sequence`

**Signature**:
```rust
fn parse_sequence(line: &str) -> Result<Vec<i64>>
```

**Purpose**: Parse a line of space-separated integers into a vector

**Implementation**:
```rust
fn parse_sequence(line: &str) -> Result<Vec<i64>> {
    line.split_whitespace()
        .map(|s| s.parse::<i64>().map_err(|e| anyhow::anyhow!("Parse error: {}", e)))
        .collect()
}
```

**Step-by-Step Walkthrough**:
```rust
// Input: "0 3 6 9 12 15"

// 1. split_whitespace() → Iterator<Item = &str>
//    ["0", "3", "6", "9", "12", "15"]

// 2. map(|s| s.parse::<i64>()...) → Iterator<Item = Result<i64, Error>>
//    [Ok(0), Ok(3), Ok(6), Ok(9), Ok(12), Ok(15)]
//    Note: parse() returns Result<i64, ParseIntError>
//    We map_err to convert to anyhow::Error

// 3. collect::<Result<Vec<i64>>>()
//    Result<Vec<i64>, Error> = Ok([0, 3, 6, 9, 12, 15])
//    Note: collect() on Iterator<Result<T, E>> produces Result<Vec<T>, E>
//    Stops at first error (short-circuits)
```

**Why `.collect::<Result<Vec<i64>>>()`**?
- Rust's `FromIterator` trait has impl for `Result<C, E>` where `C: FromIterator<T>`
- Automatically collects all `Ok` values or returns first `Err`
- No need for manual error handling in loop

**Error Handling**:
```rust
// Input: "0 3 abc 9"
// Result: Err("Parse error: invalid digit found in string")
```

**Complexity**: O(n) where n = number of integers in line

---

### Function 2: `compute_differences`

**Signature**:
```rust
fn compute_differences(sequence: &[i64]) -> Vec<i64>
```

**Purpose**: Compute forward differences: each element is `next - current`

**Implementation**:
```rust
fn compute_differences(sequence: &[i64]) -> Vec<i64> {
    sequence
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect()
}
```

**The `windows(2)` Pattern**:
```rust
// Input: [0, 3, 6, 9, 12, 15]

// windows(2) produces overlapping pairs:
// [0, 3]  → 3 - 0 = 3
// [3, 6]  → 6 - 3 = 3
// [6, 9]  → 9 - 6 = 3
// [9, 12] → 12 - 9 = 3
// [12, 15] → 15 - 12 = 3

// Result: [3, 3, 3, 3, 3]
```

**Why `windows(2)` vs manual indexing**?
```rust
// ❌ Manual indexing - error-prone
let mut diffs = Vec::new();
for i in 0..sequence.len()-1 {
    diffs.push(sequence[i+1] - sequence[i]);
}

// ✅ windows(2) - iterator pattern, no index errors
sequence.windows(2)
    .map(|pair| pair[1] - pair[0])
    .collect()
```

**Properties**:
- Output length = input length - 1
- For sequence of length n, produces n-1 differences
- No allocation during iteration (iterator is lazy)
- Single allocation for final `Vec` via `collect()`

**Complexity**: O(n) where n = sequence length

---

### Function 3: `all_zeros`

**Signature**:
```rust
fn all_zeros(sequence: &[i64]) -> bool
```

**Purpose**: Check if all elements in sequence are zero (base case for recursion)

**Implementation**:
```rust
fn all_zeros(sequence: &[i64]) -> bool {
    sequence.iter().all(|&x| x == 0)
}
```

**How `.all()` works**:
```rust
// Input: [0, 0, 0, 0]
// Check each: 0 == 0? Yes → 0 == 0? Yes → ... → Result: true

// Input: [0, 1, 0]
// Check each: 0 == 0? Yes → 1 == 0? No → STOP → Result: false
```

**Early termination**:
- `.all()` stops at first `false` (doesn't check remaining elements)
- For `[0, 0, 1, 0, 0]`, checks only first 3 elements
- Efficient for detecting non-zeros early

**Alternative approaches**:
```rust
// ❌ Manual loop - more verbose
let mut all_zero = true;
for &x in sequence {
    if x != 0 {
        all_zero = false;
        break;
    }
}

// ✅ .all() - idiomatic and clear
sequence.iter().all(|&x| x == 0)

// ⚠️ .count() - works but doesn't short-circuit
sequence.iter().filter(|&&x| x != 0).count() == 0
```

**Complexity**: O(n) worst case, O(1) best case (early termination)

---

### Function 4: `extrapolate_next` (⭐ Core Algorithm)

**Signature**:
```rust
fn extrapolate_next(sequence: &[i64]) -> i64
```

**Purpose**: Recursively extrapolate the next value in a sequence

**Implementation**:
```rust
fn extrapolate_next(sequence: &[i64]) -> i64 {
    // Base case: if all zeros, next value is also 0
    if all_zeros(sequence) {
        return 0;
    }
    
    // Recursive case: compute differences and extrapolate
    let differences = compute_differences(sequence);
    let diff_extrapolated = extrapolate_next(&differences);
    
    // Next value = last value + extrapolated difference
    sequence.last().unwrap() + diff_extrapolated
}
```

**Recursive Trace Example**:
```
extrapolate_next([0, 3, 6, 9, 12, 15])
├─ all_zeros([0, 3, 6, 9, 12, 15])? No
├─ differences = [3, 3, 3, 3, 3]
├─ extrapolate_next([3, 3, 3, 3, 3])
│  ├─ all_zeros([3, 3, 3, 3, 3])? No
│  ├─ differences = [0, 0, 0, 0]
│  ├─ extrapolate_next([0, 0, 0, 0])
│  │  ├─ all_zeros([0, 0, 0, 0])? Yes
│  │  └─ return 0                     ← Base case
│  ├─ 3 + 0 = 3
│  └─ return 3                        ← Level 1 result
├─ 15 + 3 = 18
└─ return 18                          ← Final answer
```

**Visual Pyramid**:
```
  0   3   6   9  12  15  [18]  ← return 15 + 3
    3   3   3   3   3  [3]     ← return 3 + 0
      0   0   0   0  [0]       ← base case
```

**Why Recursion Works**:
1. **Natural structure**: Each level is a smaller subproblem (reduced degree polynomial)
2. **Base case guarantee**: Polynomials always reach all-zeros eventually
3. **Build-up phase**: Combine results from bottom of pyramid upward

**Complexity Analysis**:
- **Recursion depth**: O(d) where d = polynomial degree
- **Work per level**: O(n - level) to compute differences
- **Total**: O(d × n) ≈ O(n²) worst case (when d ≈ n)
- **Typical**: O(2n) to O(4n) for low-degree polynomials (d = 2-4)

---

### Function 5: `extrapolate_prev` (Backward Variant)

**Signature**:
```rust
fn extrapolate_prev(sequence: &[i64]) -> i64
```

**Purpose**: Recursively extrapolate the previous value in a sequence

**Implementation**:
```rust
fn extrapolate_prev(sequence: &[i64]) -> i64 {
    // Base case: if all zeros, previous value is also 0
    if all_zeros(sequence) {
        return 0;
    }
    
    // Recursive case: compute differences and extrapolate
    let differences = compute_differences(sequence);
    let diff_extrapolated = extrapolate_prev(&differences);
    
    // Previous value = first value - extrapolated difference
    sequence.first().unwrap() - diff_extrapolated
}
```

**Differences from `extrapolate_next`**:
| Aspect | Forward (next) | Backward (prev) |
|--------|---------------|-----------------|
| Base case | Same (`all_zeros`) | Same (`all_zeros`) |
| Recursive call | Same (`extrapolate_next(diffs)`) | **Different** (`extrapolate_prev(diffs)`) |
| Access point | `.last()` | `.first()` |
| Operation | `+` | `-` |

**🎯 Clever Alternative - The Reverse Hack**:

Instead of implementing `extrapolate_prev` separately, you can reuse `extrapolate_next` by reversing the input!

```rust
// Part 2 - The elegant way:
pub fn solve_part2(input: &str) -> Result<String> {
    let sum: i64 = input.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut sequence = parse_sequence(line)?;
            sequence.reverse();  // ✨ The hack!
            Ok(extrapolate_next(&sequence))  // Reuse forward extrapolation!
        })
        .collect::<Result<Vec<i64>>>()?
        .iter()
        .sum();
    Ok(sum.to_string())
}
```

**Why it works**:
```
Original: [0, 3, 6, 9, 12, 15]
  Backward extrapolation → -3 (what comes before 0?)

Reversed: [15, 12, 9, 6, 3, 0]
  Forward extrapolation → -3 (what comes after 0?)
```

The difference pyramid is structurally identical, just mirrored! This eliminates an entire function and is a beautiful example of DRY (Don't Repeat Yourself).

**Trade-off**: Current implementation is more explicit about intent (forward vs backward), but the reverse hack is more elegant and reduces code duplication.

**Why subtraction**?
```
Difference pyramid (backward view):
[-3]  0   3   6   9  12  15    ← prev = 0 - 3 = -3
    [3]  3   3   3   3   3     ← prev_diff = 3 - 0 = 3
      [0] 0   0   0   0        ← base case 0
```

**Recursive Trace Example**:
```
extrapolate_prev([0, 3, 6, 9, 12, 15])
├─ all_zeros([0, 3, 6, 9, 12, 15])? No
├─ differences = [3, 3, 3, 3, 3]
├─ extrapolate_prev([3, 3, 3, 3, 3])
│  ├─ all_zeros([3, 3, 3, 3, 3])? No
│  ├─ differences = [0, 0, 0, 0]
│  ├─ extrapolate_prev([0, 0, 0, 0])
│  │  ├─ all_zeros([0, 0, 0, 0])? Yes
│  │  └─ return 0                     ← Base case
│  ├─ 3 - 0 = 3
│  └─ return 3                        ← Level 1 result
├─ 0 - 3 = -3
└─ return -3                          ← Final answer
```

---

## 🧮 Mathematical Algorithms

### Polynomial Degree Detection

**Theorem**: A polynomial of degree n has constant nth differences.

**Proof by Example**:
```rust
// Degree 0 (constant): f(x) = 5
Sequence: [5, 5, 5, 5, 5]
Diffs:     [0, 0, 0, 0]    ← 1st differences constant (zero)

// Degree 1 (linear): f(x) = 2x + 1
Sequence: [1, 3, 5, 7, 9]  → f(0)=1, f(1)=3, f(2)=5, ...
Diffs:     [2, 2, 2, 2]    ← 1st differences constant

// Degree 2 (quadratic): f(x) = x²
Sequence: [0, 1, 4, 9, 16, 25]  → 0², 1², 2², 3², 4², 5²
1st diffs:  [1, 3, 5, 7, 9]    → Increasing
2nd diffs:    [2, 2, 2, 2]    ← 2nd differences constant

// Degree 3 (cubic): f(x) = x³
Sequence: [0, 1, 8, 27, 64, 125]  → 0³, 1³, 2³, 3³, 4³, 5³
1st diffs:  [1, 7, 19, 37, 61]
2nd diffs:    [6, 12, 18, 24]
3rd diffs:      [6, 6, 6]          ← 3rd differences constant
```

**Calculus Connection** (Intuition):
- Taking differences ≈ taking derivatives
- Derivative of x^n has degree n-1
- Derivative of constant = 0
- nth derivative of degree-n polynomial = constant

### Extrapolation Formula Derivation

**Forward extrapolation**:
```
Given sequence S = [a₀, a₁, a₂, ..., aₙ]
Differences D = [d₀, d₁, d₂, ..., dₙ₋₁] where dᵢ = aᵢ₊₁ - aᵢ

To find aₙ₊₁:
  aₙ₊₁ = aₙ + dₙ
  
But we need dₙ (the next difference).
Recursively: dₙ = extrapolate_next(D)

Therefore: aₙ₊₁ = aₙ + extrapolate_next(D)
```

**Backward extrapolation**:
```
To find a₋₁ (value before a₀):
  a₀ = a₋₁ + d₋₁
  Therefore: a₋₁ = a₀ - d₋₁

Recursively: d₋₁ = extrapolate_prev(D)

Therefore: a₋₁ = a₀ - extrapolate_prev(D)
```

---

## 🔌 Public API

### Function: `solve_part1`

**Signature**:
```rust
pub fn solve_part1(input: &str) -> Result<String>
```

**Purpose**: Solve Part 1 - sum of forward extrapolations for all sequences

**Implementation**:
```rust
pub fn solve_part1(input: &str) -> Result<String> {
    let sum: i64 = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let sequence = parse_sequence(line)?;
            Ok(extrapolate_next(&sequence))
        })
        .collect::<Result<Vec<i64>>>()?
        .iter()
        .sum();
    
    Ok(sum.to_string())
}
```

**Pipeline Breakdown**:
```rust
// 1. input.lines() → Iterator over lines
// 2. .filter(...) → Skip empty lines
// 3. .map(...) → Parse and extrapolate each line
//    Returns Iterator<Result<i64, Error>>
// 4. .collect::<Result<Vec<i64>>>() → Collect all or fail at first error
//    Short-circuits on parse errors
// 5. .iter().sum() → Sum all extrapolated values
// 6. .to_string() → Convert to string for AoC answer format
```

**Error Propagation**:
```rust
// If any line fails to parse:
parse_sequence(line)?   // Returns Err
  ↓
map returns Err
  ↓
collect returns Err     // Short-circuits
  ↓
? propagates Err upward
  ↓
solve_part1 returns Err
```

**Performance**: 132µs for 200 sequences

---

### Function: `solve_part2`

**Signature**:
```rust
pub fn solve_part2(input: &str) -> Result<String>
```

**Purpose**: Solve Part 2 - sum of backward extrapolations for all sequences

**Implementation**:
```rust
pub fn solve_part2(input: &str) -> Result<String> {
    let sum: i64 = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let sequence = parse_sequence(line)?;
            Ok(extrapolate_prev(&sequence))
        })
        .collect::<Result<Vec<i64>>>()?
        .iter()
        .sum();
    
    Ok(sum.to_string())
}
```

**Only Difference from Part 1**: Uses `extrapolate_prev` instead of `extrapolate_next`

**Performance**: 191µs for 200 sequences (slightly slower due to more complex arithmetic in backward direction)

---

## 🎨 Design Patterns

### Pattern 1: Iterator Chains
```rust
input.lines()
    .filter(...)
    .map(...)
    .collect()
```
**Benefits**:
- Lazy evaluation (no intermediate allocations)
- Composable transformations
- Idiomatic Rust

### Pattern 2: Error Handling with `?`
```rust
let sequence = parse_sequence(line)?;
```
**Benefits**:
- Early return on error
- Propagates error context
- Clean, readable code

### Pattern 3: Recursion with Base Case
```rust
fn extrapolate(seq: &[i64]) -> i64 {
    if base_case(seq) { return terminal_value; }
    let subproblem = transform(seq);
    combine(seq, extrapolate(&subproblem))
}
```
**Benefits**:
- Natural for pyramid structures
- Automatic degree detection
- Self-documenting algorithm

### Pattern 4: `windows(2)` for Pairwise Operations
```rust
sequence.windows(2).map(|pair| pair[1] - pair[0])
```
**Benefits**:
- No index errors
- Iterator pattern
- Clear intent

---

## ⚡ Performance Analysis

### Time Complexity

**Per Sequence**:
- Recursion depth: O(d) where d = polynomial degree
- Work per level: O(n - level)
- Total: O(d × n)

**Worst Case**: O(n²) when d ≈ n (rare)
**Typical Case**: O(2n) to O(4n) when d = 2-4 (common)

**For Day 9**:
- 200 sequences
- Average length: ~21 values
- Typical depth: 2-4 levels
- Total time: 323µs (excellent!)

### Space Complexity

**Recursion Stack**: O(d) frames
**Memory per Level**: O(n - level) for difference Vec
**Total Space**: O(d × n) worst case

**Actual Usage**:
- Small recursion depth (2-4 levels)
- Small vectors (< 25 elements)
- Total memory negligible

### Benchmark Results
```
day09_part1             time:   [132.0 µs]
day09_part2             time:   [191.0 µs]
Total                   time:   [323.0 µs]
```

---

## 🧪 Testing Strategy

### Test Coverage

**Unit Tests**:
1. `test_parse_sequence` - Parsing integers
2. `test_compute_differences` - Pairwise differences
3. `test_all_zeros` - Base case detection
4. `test_extrapolate_next_simple` - Linear sequence (degree 1)
5. `test_extrapolate_next_example2` - Quadratic sequence (degree 2)
6. `test_extrapolate_next_example3` - Cubic sequence (degree 3)
7. `test_extrapolate_prev_simple` - Backward linear
8. `test_extrapolate_prev_example` - Backward cubic
9. `test_part1_example` - Full Part 1 with example data
10. `test_part2_example` - Full Part 2 with example data

### Test Philosophy

**Bottom-Up Testing**:
1. Test atomic functions first (`parse_sequence`, `compute_differences`)
2. Test core algorithm (`extrapolate_next`, `extrapolate_prev`)
3. Test public API (`solve_part1`, `solve_part2`)

**Example-Driven**:
- Use examples from problem statement
- Verify against known answers
- Cover different polynomial degrees

---

## ⚠️ Common Pitfalls

### Pitfall 1: Off-by-One in Manual Indexing
```rust
// ❌ Easy to get wrong
for i in 0..sequence.len() {  // Bug: goes one past end
    let diff = sequence[i+1] - sequence[i];
}

// ✅ windows(2) prevents this
sequence.windows(2).map(|pair| pair[1] - pair[0])
```

### Pitfall 2: Forgetting Base Case
```rust
// ❌ Infinite recursion
fn extrapolate(seq: &[i64]) -> i64 {
    let diffs = compute_differences(seq);
    seq.last().unwrap() + extrapolate(&diffs)  // Never stops!
}

// ✅ Always check base case
fn extrapolate(seq: &[i64]) -> i64 {
    if all_zeros(seq) { return 0; }  // Essential!
    // ... recursive case
}
```

### Pitfall 3: Sign Error in Backward Extrapolation
```rust
// ❌ Wrong sign
sequence.first().unwrap() + diff_prev  // Should be minus!

// ✅ Correct
sequence.first().unwrap() - diff_prev
```

### Pitfall 4: Not Handling Empty Lines
```rust
// ❌ Crashes on empty line
input.lines().map(parse_sequence)

// ✅ Filter empties
input.lines().filter(|line| !line.trim().is_empty())
```

---

## 🎓 Key Takeaways

1. **Recursion naturalness**: Difference pyramid structure makes recursion obvious choice
2. **Iterator power**: `windows(2)`, `.all()`, `.map()` eliminate manual indexing
3. **Error handling**: `Result` + `?` operator provides clean error propagation
4. **Mathematical foundation**: 350-year-old algorithm still elegant and efficient
5. **Performance**: Sometimes straightforward solution is already optimal
6. **Symmetry**: Forward/backward differ only in sign - beautiful design
7. **Degree detection**: Recursion automatically finds polynomial degree

---

## 🤔 Follow-Up Questions

### Conceptual
1. **Q**: What if the sequence isn't polynomial?  
   **A**: Algorithm still terminates (all sequences reach zeros eventually with enough differences), but extrapolation may not be meaningful.

2. **Q**: Could we detect the polynomial formula?  
   **A**: Yes! Use polynomial interpolation (Lagrange, Newton's divided differences) to recover coefficients.

3. **Q**: Why not iterative instead of recursive?  
   **A**: Possible but more complex. Would need explicit stack management. Recursion mirrors mathematical structure.

### Implementation
4. **Q**: Could we parallelize across sequences?  
   **A**: Yes with Rayon! Each sequence is independent. But overhead > benefit for ~200 sequences at 323µs.

5. **Q**: What if we need to extrapolate multiple steps?  
   **A**: Call extrapolate n times, or extend algorithm to build polynomial formula once.

### Optimization
6. **Q**: Can we avoid allocating difference vectors?  
   **A**: Yes, but complex. Would need iterative approach with pre-allocated buffers. Not worth it.

7. **Q**: What's the theoretical lower bound?  
   **A**: Ω(n) to read input. Current O(n²) is near-optimal for this approach.

### Extensions
8. **Q**: Handle floating-point sequences?  
   **A**: Change `i64` to `f64`. Watch for precision issues in base case (`abs(x) < EPSILON` instead of `x == 0`).

9. **Q**: Support 2D sequences (matrices)?  
   **A**: Extend to 2D finite differences - used in image processing!

10. **Q**: Connect to calculus?  
    **A**: Finite differences ≈ discrete derivatives. As step size → 0, finite differences → true derivatives.

---

**Mathematical Beauty**: This 350-year-old algorithm demonstrates timeless elegance - recursion, pattern detection, and polynomial theory unite in 50 lines of Rust!

**Practical Impact**: Sub-millisecond performance proving that sometimes the simplest solution is the best solution.

---

*See also*:
- `zettelkasten/math-foundations/finite-differences.md` - Complete mathematical theory
- `advent_of_code/aoc2023/Problem_Statements/algorithms-reference.md` - Algorithm catalog
- `advent_of_code/aoc2023/Problem_Statements/summary_2023.md#day-9` - Problem summary

*Tags: #finite-differences #recursion #polynomials #aoc2023 #numerical-analysis*
