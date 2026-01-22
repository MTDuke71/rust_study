# Polynomial Interpolation - Lagrange Method

**Concept**: Given n points, find a polynomial of degree n-1 that passes through all of them.

**Created**: 2026-01-21  
**Tags**: #mathematics #polynomials #interpolation #lagrange #extrapolation

---

## Problem Statement

**Input**: n points (x₀, y₀), (x₁, y₁), ..., (xₙ₋₁, yₙ₋₁)  
**Output**: Polynomial P(x) of degree ≤ n-1 such that P(xᵢ) = yᵢ for all i

**Uniqueness Theorem**: Such a polynomial exists and is unique (assuming distinct x values).

---

## Lagrange Formula

For n points, the interpolating polynomial is:

```
P(x) = Σ yᵢ × Lᵢ(x)
      i=0 to n-1

where Lᵢ(x) = Π (x - xⱼ) / (xᵢ - xⱼ)
             j≠i
```

**Lᵢ(x)** are called **Lagrange basis polynomials**.

**Key Property**: 
- Lᵢ(xᵢ) = 1 (numerator and denominator equal)
- Lᵢ(xⱼ) = 0 for j ≠ i (numerator has factor (xⱼ - xⱼ) = 0)

This ensures P(xₖ) = yₖ for each data point.

---

## Example: Quadratic (3 points)

**Given**: (0, 2), (1, 3), (2, 6)  
**Find**: P(x) = ax² + bx + c

### Lagrange Approach

**Basis polynomials**:
```
L₀(x) = (x-1)(x-2) / ((0-1)(0-2)) = (x-1)(x-2) / 2
L₁(x) = (x-0)(x-2) / ((1-0)(1-2)) = x(x-2) / (-1) = -x(x-2)
L₂(x) = (x-0)(x-1) / ((2-0)(2-1)) = x(x-1) / 2
```

**Polynomial**:
```
P(x) = 2×L₀(x) + 3×L₁(x) + 6×L₂(x)
     = 2(x-1)(x-2)/2 + 3(-x)(x-2) + 6x(x-1)/2
     = (x-1)(x-2) - 3x(x-2) + 3x(x-1)
     = x² - 3x + 2 - 3x² + 6x + 3x² - 3x
     = x² + 2
```

**Verify**:
- P(0) = 0 + 2 = 2 ✓
- P(1) = 1 + 2 = 3 ✓
- P(2) = 4 + 2 = 6 ✓

---

## Simplified Formula: Evenly-Spaced Points

When points are evenly spaced (x₀=0, x₁=1, x₂=2, ...), formulas simplify dramatically.

### Quadratic (x=0,1,2)

Given y₀, y₁, y₂:

```
a = (y₀ - 2y₁ + y₂) / 2
b = (-3y₀ + 4y₁ - y₂) / 2  
c = y₀

P(x) = ax² + bx + c
```

**Derivation** (system of equations):
```
P(0) = c = y₀                    →  c = y₀
P(1) = a + b + c = y₁            →  a + b = y₁ - y₀
P(2) = 4a + 2b + c = y₂          →  4a + 2b = y₂ - y₀

From equation 2: b = y₁ - y₀ - a
Substitute into 3: 4a + 2(y₁ - y₀ - a) = y₂ - y₀
                   4a + 2y₁ - 2y₀ - 2a = y₂ - y₀
                   2a = y₂ - 2y₁ + y₀
                   a = (y₀ - 2y₁ + y₂) / 2

Then: b = y₁ - y₀ - a
        = y₁ - y₀ - (y₀ - 2y₁ + y₂)/2
        = (2y₁ - 2y₀ - y₀ + 2y₁ - y₂) / 2
        = (-3y₀ + 4y₁ - y₂) / 2
```

---

## Connection to Finite Differences

The coefficient `a` in the quadratic equals **half the second finite difference**!

**First differences**:
```
Δ₀ = y₁ - y₀
Δ₁ = y₂ - y₁
```

**Second difference**:
```
Δ²₀ = Δ₁ - Δ₀ = (y₂ - y₁) - (y₁ - y₀) = y₂ - 2y₁ + y₀
```

**Observe**: 
```
Δ²₀ = y₀ - 2y₁ + y₂  (rearranged)
2a = y₀ - 2y₁ + y₂
a = Δ²₀ / 2
```

**For constant second difference** (perfect quadratic data):
- Δ²₀ = Δ²₁ = Δ²₂ = ... = 2a
- This validates the quadratic model!

---

## Rust Implementation

### Quadratic Fitting (x=0,1,2)

```rust
/// Fit quadratic f(x) = ax² + bx + c through 3 evenly-spaced points
fn fit_quadratic(y0: i64, y1: i64, y2: i64) -> (i64, i64, i64) {
    let a = (y0 - 2*y1 + y2) / 2;
    let b = (-3*y0 + 4*y1 - y2) / 2;
    let c = y0;
    (a, b, c)
}

/// Evaluate quadratic at point x
fn eval_quadratic(a: i64, b: i64, c: i64, x: i64) -> i64 {
    a * x * x + b * x + c
}

// Example usage
let (a, b, c) = fit_quadratic(3797, 34009, 94353);
// a=15066, b=15146, c=3797
// f(x) = 15066x² + 15146x + 3797

let result = eval_quadratic(a, b, c, 202300);
// result = 616583483179597
```

**Type Considerations**:
- Use `i64` for coefficients (a, b can be negative during calculation)
- Cast to `usize` only for final result if counts can't be negative
- Watch for overflow: 15066 × (202300)² ≈ 616 trillion (needs i64!)

### General Lagrange (arbitrary points)

```rust
/// Compute Lagrange basis polynomial Lᵢ(x)
fn lagrange_basis(x: f64, i: usize, x_points: &[f64]) -> f64 {
    let mut result = 1.0;
    for (j, &xj) in x_points.iter().enumerate() {
        if i != j {
            result *= (x - xj) / (x_points[i] - xj);
        }
    }
    result
}

/// Lagrange interpolation
fn lagrange_interpolate(x: f64, x_points: &[f64], y_points: &[f64]) -> f64 {
    assert_eq!(x_points.len(), y_points.len());
    
    let mut sum = 0.0;
    for i in 0..x_points.len() {
        sum += y_points[i] * lagrange_basis(x, i, x_points);
    }
    sum
}

// Example
let x_pts = vec![0.0, 1.0, 2.0];
let y_pts = vec![3797.0, 34009.0, 94353.0];
let result = lagrange_interpolate(202300.0, &x_pts, &y_pts);
```

---

## Applications

### AoC 2023 Day 21: Quadratic Extrapolation

**Problem**: Count reachable plots on infinite grid after 26,501,365 steps.

**Insight**: Reachable area grows quadratically (diamond shape on 2D grid).

**Solution**:
1. Sample 3 points: y₀=3797@65 steps, y₁=34009@196, y₂=94353@327
2. Fit quadratic: f(n) = 15066n² + 15146n + 3797
3. Extrapolate: f(202300) = 616,583,483,179,597

**Why works**: 
- 26,501,365 = 65 + 131×202,300 (puzzle design!)
- After 65 steps (reach edge), pattern repeats every 131 steps
- Quadratic growth confirmed by constant second difference

**See**: [[aoc2023-day21]], [[quadratic-growth-2d-grids]]

### Numerical Analysis: Function Approximation

**Use case**: Approximate expensive function f(x) by sampling a few points.

**Limitations**:
- High-degree polynomials can oscillate wildly (Runge's phenomenon)
- Better for local approximation (splines) than global
- Errors grow exponentially outside interpolation range

### Physics: Trajectory Modeling

**Projectile motion**: y(t) = -½gt² + v₀t + h₀ (quadratic in time)

Given 3 time-height measurements, fit quadratic to predict future position.

---

## Complexity Analysis

**Time Complexity**: 
- Lagrange method: O(n²) for n points (compute n basis polynomials, each O(n))
- Evaluation: O(n) per point
- Simplified quadratic: O(1) for 3 points!

**Space Complexity**: O(n) to store points

**Numerical Stability**:
- Evenly-spaced points: Good
- Clustered points: Division by small numbers → unstable
- Alternative: Newton divided differences (more stable)

---

## Related Concepts

**Other Interpolation Methods**:
- **Newton's Divided Differences** - More numerically stable
- **Cubic Splines** - Piecewise polynomials, C² continuous
- **Chebyshev Interpolation** - Minimizes oscillation

**Extrapolation Warning**: Predictions outside [x₀, xₙ₋₁] can be wildly inaccurate!
- Interpolation (inside range): Usually reliable
- Extrapolation (outside range): High risk, validate with domain knowledge

**Finite Difference Connection**:
- Forward differences extract polynomial coefficients directly
- Backward differences useful for derivative approximation
- Central differences minimize error in numerical derivatives

---

## Zettelkasten Links

**Mathematics**:
- [[math-foundations/finite-differences]] - Alternative coefficient extraction
- [[math-foundations/numerical-stability]] - Avoiding catastrophic cancellation
- [[math-foundations/polynomial-theory]] - Degree bounds, uniqueness theorems

**AoC Applications**:
- [[aoc2023-day21]] - Quadratic extrapolation for infinite grid
- [[quadratic-growth-2d-grids]] - Why reachable area is O(r²)
- [[pattern-recognition-in-algorithms]] - Recognizing when to sample vs brute-force

**Mission Integration**:
- Could create Mission for polynomial utilities (fitting, evaluation, validation)
- Useful for AoC problems with predictable growth patterns

---

**References**:
- Wikipedia: [Lagrange Polynomial](https://en.wikipedia.org/wiki/Lagrange_polynomial)
- Numerical Recipes: Chapter on Interpolation
- [AoC 2023 Day 21 Solution](d:/repos/rust_study/advent_of_code/aoc2023/src/solver/day21.rs)
