# Quadratic Equations

**Field**: Algebra / Applied Mathematics

**Prerequisites**: [[algebra-basics]]

---

## 📐 Definition

A **quadratic equation** is a polynomial equation of degree 2:

$$ax^2 + bx + c = 0$$

where $a \neq 0$, and $a$, $b$, $c$ are constants.

The graph of the related quadratic function $f(x) = ax^2 + bx + c$ is a **parabola**:
- Opens **upward** if $a > 0$
- Opens **downward** if $a < 0$

---

## 🔧 Quadratic Formula

The solutions (roots) are given by:

$$x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$$

### **Discriminant**

The expression $\Delta = b^2 - 4ac$ determines the nature of roots:

| **Discriminant** | **Roots** |
|------------------|-----------|
| $\Delta > 0$ | Two distinct real roots |
| $\Delta = 0$ | One repeated real root |
| $\Delta < 0$ | Two complex conjugate roots |

---

## 🎯 Quadratic Inequalities

For inequalities like $ax^2 + bx + c < 0$:

1. Find roots using the quadratic formula
2. Determine where the parabola is negative/positive
3. For $a < 0$ (opens downward): parabola is positive **between** roots
4. For $a > 0$ (opens upward): parabola is negative **between** roots

**Integer solutions**: Count integers in the interval between roots.

---

## 💻 Rust Implementations

### **Advent of Code 2023 - Day 6: Wait For It**

**Problem**: Boat race where distance $d = h \times (T - h)$ for hold time $h$ and total time $T$.

**Mathematical Model**:
```
d(h) = -h² + T×h
Need: d(h) > R (record distance)
Rearrange: h² - T×h + R < 0
```

**Solution**: Find integer values between the roots of $h^2 - Th + R = 0$:

```rust
fn count_ways_quadratic(time: u64, record: u64) -> u64 {
    let t = time as f64;
    let r = record as f64;
    
    // Discriminant: T² - 4R
    let discriminant = t * t - 4.0 * r;
    if discriminant < 0.0 { return 0; }
    
    let sqrt_disc = discriminant.sqrt();
    
    // Roots: h = (T ± √Δ) / 2
    let root1 = (t - sqrt_disc) / 2.0;  // Lower bound
    let root2 = (t + sqrt_disc) / 2.0;  // Upper bound
    
    // Count integers strictly between roots
    let min_hold = if root1.fract() == 0.0 
        && (root1 as u64) * (time - root1 as u64) == record {
        (root1 as u64) + 1  // Exclude exact tie
    } else {
        root1.ceil() as u64
    };
    
    let max_hold = if root2.fract() == 0.0 
        && (root2 as u64) * (time - root2 as u64) == record {
        (root2 as u64) - 1  // Exclude exact tie
    } else {
        root2.floor() as u64
    };
    
    max_hold.saturating_sub(min_hold).saturating_add(1)
}
```

**Key Implementation Details**:
- Uses `f64` for square root computation
- `fract()` checks if root is exactly an integer
- Excludes boundary values that **equal** the record (need strictly greater)
- `ceil()` / `floor()` convert to nearest integers inside the interval

**File**: `advent_of_code/aoc2023/src/solver/day06.rs`  
**Complexity**: O(1) - constant time regardless of input size  
**Performance**: 0.30µs (Part 2 with T=48,938,595)

**Algorithm Comparison**:
- Brute force: O(T) - iterate all possible values
- Quadratic formula: O(1) - direct calculation
- Speedup: Avoided ~49 million iterations!

---

## 🔢 Numerical Considerations

### **Floating-Point Precision**
When implementing in Rust:
- Use `f64` for sufficient precision
- Be careful with exact integer roots (boundary cases)
- Test edge cases where discriminant is zero
- Consider rounding errors near boundaries

### **Integer Conversion**
```rust
root.ceil() as u64  // Round UP to next integer
root.floor() as u64 // Round DOWN to previous integer
root.fract() == 0.0 // Check if exactly integer
```

---

## 📊 Visual Example (Day 6, Race 1)

**Given**: T = 7, R = 9

**Equation**: $h^2 - 7h + 9 < 0$

**Discriminant**: $\Delta = 49 - 36 = 13$

**Roots**:
- $h_1 = \frac{7 - \sqrt{13}}{2} \approx 1.697$
- $h_2 = \frac{7 + \sqrt{13}}{2} \approx 5.303$

**Integer solutions**: $\{2, 3, 4, 5\}$ → **4 ways to win**

**Verification**:
- h=2: $2 \times 5 = 10 > 9$ ✅
- h=3: $3 \times 4 = 12 > 9$ ✅
- h=4: $4 \times 3 = 12 > 9$ ✅
- h=5: $5 \times 2 = 10 > 9$ ✅

---

## 🎓 Key Insights

1. **Parabolas model many optimization problems** (max/min, boundaries)
2. **Quadratic formula provides O(1) solution** vs brute force O(n)
3. **Integer constraints** require careful boundary handling
4. **Discriminant** determines feasibility before calculation
5. **Floating-point precision** matters for boundary cases

---

## 🔗 Related Concepts

**Mathematics**:
- [[algebra-basics]] - Polynomial operations, factoring
- [[numerical-methods]] - Root-finding algorithms, Newton's method
- [[optimization-theory]] - Finding maxima/minima of functions

**Implementations**:
- [[aoc-2023-day06]] - Boat race optimization
- [[complexity-theory]] - O(1) vs O(n) algorithm comparison

**Future Applications**:
- Trajectory problems (physics simulations)
- Area/perimeter optimization
- Resource allocation with quadratic constraints

---

*Tags: #mathematics #algebra #quadratic-equations #optimization #aoc2023 #numerical-methods*

*Last Updated*: January 6, 2025
