# Finite Differences

**Field**: Numerical Analysis / Polynomial Interpolation

**Prerequisites**: Polynomials, recursion, basic calculus concepts

---

## 📐 Definition

**Finite differences** is a method from numerical analysis for detecting polynomial patterns in sequences and extrapolating future/past values.

**Intuition**: By repeatedly computing differences between consecutive terms, polynomial sequences eventually reach constant differences. The "degree" of differences that become constant reveals the polynomial's degree.

---

## 🔑 Key Concepts

### **Forward Difference**
- **Statement**: The difference between consecutive terms: Δf(x) = f(x+1) - f(x)
- **Example**: For sequence [0, 3, 6, 9, 12], forward differences are [3, 3, 3, 3]

### **Polynomial Degree Theorem**
- **Theorem**: A polynomial of degree n has constant nth differences
  - Degree 0 (constant): 0th differences constant (values don't change)
  - Degree 1 (linear): 1st differences constant
  - Degree 2 (quadratic): 2nd differences constant
  - Degree n: nth differences constant

### **Difference Pyramid**
- **Structure**: Build levels of differences recursively until all zeros
- **Base case**: When all differences are zero, extrapolation is complete
- **Example**:
```
Level 0:  0   3   6   9  12  15    ← Original sequence (linear)
Level 1:    3   3   3   3   3      ← 1st differences (constant!)
Level 2:      0   0   0   0        ← All zeros (stop)
```

### **Extrapolation Formula**
- **Forward**: next_value = last_value + extrapolate(differences)
- **Backward**: prev_value = first_value - extrapolate(differences)
- **Recursive structure**: Each level builds on the level below

---

## 🧮 Mathematical Examples

### Linear Sequence (Degree 1)
```
Sequence: 0, 3, 6, 9, 12, 15  (arithmetic progression: +3 each time)
Polynomial: f(x) = 3x

Difference pyramid:
Level 0:  0   3   6   9  12  15    ← f(x)
Level 1:    3   3   3   3   3      ← Δf(x) - constant!
Level 2:      0   0   0   0        ← Δ²f(x) - all zeros

Next value: 15 + 3 = 18
Previous value: 0 - 3 = -3
```

### Quadratic Sequence (Degree 2)
```
Sequence: 1, 3, 6, 10, 15, 21  (triangular numbers)
Polynomial: f(x) = x(x+1)/2

Difference pyramid:
Level 0:  1   3   6  10  15  21    ← f(x)
Level 1:    2   3   4   5   6      ← Δf(x) - increasing
Level 2:      1   1   1   1        ← Δ²f(x) - constant!
Level 3:        0   0   0          ← Δ³f(x) - all zeros

Next value: 21 + (6 + 1) = 28
```

### Cubic Sequence (Degree 3)
```
Sequence: 10, 13, 16, 21, 30, 45
Polynomial: f(x) = ax³ + bx² + cx + d

Difference pyramid:
Level 0: 10  13  16  21  30  45    ← f(x)
Level 1:    3   3   5   9  15      ← Δf(x)
Level 2:      0   2   4   6        ← Δ²f(x)
Level 3:        2   2   2          ← Δ³f(x) - constant!
Level 4:          0   0            ← Δ⁴f(x) - all zeros

Next value: 45 + (15 + (6 + 2)) = 45 + 23 = 68
```

---

## 💻 Rust Implementations

### **AoC 2023 Day 9**: Mirage Maintenance (OASIS Sensor Readings)
- **What**: Extrapolate next/previous values from sensor reading sequences
- **How it uses this concept**: 
  - Build difference pyramid recursively for each sequence
  - Detect polynomial patterns automatically via recursion depth
  - Extrapolate forward (Part 1) and backward (Part 2)
  - 200 sequences, mostly low-degree polynomials (2-4 levels deep)
- **Link**: `advent_of_code/aoc2023/src/solver/day09.rs`
- **Performance**: 323µs total (132µs Part 1 + 191µs Part 2)

---

## 📚 Code Examples

### Recursive Finite Differences
```rust
/// Compute forward differences between consecutive elements
fn compute_differences(sequence: &[i64]) -> Vec<i64> {
    sequence
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect()
}

/// Check if all elements are zero (base case)
fn all_zeros(sequence: &[i64]) -> bool {
    sequence.iter().all(|&x| x == 0)
}

/// Extrapolate next value recursively
fn extrapolate_next(sequence: &[i64]) -> i64 {
    // Base case: zeros extrapolate to zero
    if all_zeros(sequence) {
        return 0;
    }
    
    // Recursive case: compute differences and extrapolate
    let differences = compute_differences(sequence);
    let diff_next = extrapolate_next(&differences);
    
    // Build up: next = last + extrapolated_difference
    sequence.last().unwrap() + diff_next
}

/// Extrapolate previous value (backward)
fn extrapolate_prev(sequence: &[i64]) -> i64 {
    if all_zeros(sequence) {
        return 0;
    }
    let differences = compute_differences(sequence);
    let diff_prev = extrapolate_prev(&differences);
    
    // Build up backward: prev = first - extrapolated_difference
    sequence.first().unwrap() - diff_prev
}
```

**Why recursion works**:
1. **Natural structure**: Each level of differences is a smaller subproblem
2. **Automatic degree detection**: Recursion depth reveals polynomial degree
3. **Base case guarantee**: Polynomials always reach all-zeros eventually
4. **Build-up phase**: Combine results bottom-up from base case

---

## 🔗 Connections

**Related Concepts**:
- [[polynomial-interpolation]] - Fitting polynomial to data points
- [[recursive-algorithms]] - Base case + recursive case pattern
- [[numerical-methods]] - Broader field of numerical analysis

**Rust Patterns**:
- `windows(2)` for pairwise operations
- Recursion with clear base and recursive cases
- Iterator patterns for difference computation

**Applications**:
- Time series prediction (extrapolate future values)
- Polynomial fitting (determine degree from data)
- Numerical differentiation (approximate derivatives)
- Signal processing (detecting patterns)

---

## ⚡ Performance Characteristics

**Complexity**:
- **Time**: O(d × n) where d = polynomial degree, n = sequence length
- **Worst case**: O(n²) if degree = sequence length
- **Typical case**: O(2n) to O(4n) for low-degree polynomials

**Space**:
- **Recursion depth**: O(d) where d = polynomial degree
- **Memory per level**: O(n - level) for difference vector

**Why it's efficient**:
- Real-world sequences are usually low-degree (linear, quadratic, cubic)
- Early termination at all-zeros
- Simple arithmetic operations
- No overlapping subproblems (each level computed once)

---

## 📖 Historical Context

**Origin**: Newton's divided differences (1670s) - Isaac Newton
**Applications**: 
- Early computational tables (logarithms, trigonometric functions)
- Charles Babbage's Difference Engine (1820s) - mechanical computer
- Still used in modern numerical analysis

**Fun fact**: The method is 350+ years old but remains elegant for polynomial pattern detection!

---

## 🎯 Key Takeaways

1. **Pattern detection**: Constant differences reveal polynomial structure
2. **Automatic degree**: No need to know degree beforehand - recursion finds it
3. **Bidirectional**: Same algorithm works forward and backward (sign changes)
4. **Recursive elegance**: Natural fit for pyramid structure
5. **Practical efficiency**: Low-degree polynomials reach base case quickly

---

*Tags: #numerical-analysis #polynomials #recursion #pattern-recognition #aoc2023*

*Links:*
- [[polynomial-interpolation]]
- [[recursive-algorithms]]
- [[number-theory-basics]]
- AoC 2023 Day 9: `advent_of_code/aoc2023/src/solver/day09.rs`
