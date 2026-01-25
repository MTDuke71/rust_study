# Linear Algebra Fundamentals

**Field**: Linear Algebra / Mathematics

**Prerequisites**: [[algebra-basics]]

---

## 📐 Definition

**Linear Algebra** is the study of linear equations, vector spaces, matrices, and linear transformations. It provides the mathematical foundation for:
- Solving systems of linear equations
- 3D graphics and transformations
- Machine learning and neural networks
- Physics simulations
- Optimization problems

**Core Objects**:
- **Vectors**: Quantities with magnitude and direction
- **Matrices**: Rectangular arrays of numbers representing linear transformations
- **Linear Systems**: Sets of equations of the form $Ax = b$

---

## 🔑 Key Concepts

### **Linear System**

A system of $m$ linear equations in $n$ unknowns:

$$\begin{cases}
a_{11}x_1 + a_{12}x_2 + \cdots + a_{1n}x_n = b_1 \\
a_{21}x_1 + a_{22}x_2 + \cdots + a_{2n}x_n = b_2 \\
\vdots \\
a_{m1}x_1 + a_{m2}x_2 + \cdots + a_{mn}x_n = b_m
\end{cases}$$

**Matrix Form**: $Ax = b$ where:
- $A$ is the $m \times n$ coefficient matrix
- $x$ is the column vector of unknowns
- $b$ is the column vector of constants

---

## 🔧 Cramer's Rule (2×2 Systems)

For a 2×2 system:
$$\begin{cases}
a_{11}x_1 + a_{12}x_2 = b_1 \\
a_{21}x_1 + a_{22}x_2 = b_2
\end{cases}$$

**Solution via Determinants**:

$$x_1 = \frac{\det\begin{pmatrix}b_1 & a_{12}\\b_2 & a_{22}\end{pmatrix}}{\det\begin{pmatrix}a_{11} & a_{12}\\a_{21} & a_{22}\end{pmatrix}}, \quad x_2 = \frac{\det\begin{pmatrix}a_{11} & b_1\\a_{21} & b_2\end{pmatrix}}{\det\begin{pmatrix}a_{11} & a_{12}\\a_{21} & a_{22}\end{pmatrix}}$$

where the **determinant** of a 2×2 matrix is:

$$\det\begin{pmatrix}a & b\\c & d\end{pmatrix} = ad - bc$$

### **When to Use Cramer's Rule**

✅ **Good for**:
- Small systems (2×2, 3×3)
- Symbolic computation
- One-off solutions
- O(1) per solution after determinant calculation

❌ **Avoid for**:
- Large systems (n > 3) - O(n!) complexity
- Multiple systems with same coefficient matrix
- Systems where det(A) ≈ 0 (numerical instability)

---

## 🎯 Gaussian Elimination

**Algorithm**: Transform $Ax = b$ into upper triangular form, then back-substitute.

### **Forward Elimination**

Convert to row echelon form:
$$\begin{pmatrix}
a_{11} & a_{12} & a_{13} \\
a_{21} & a_{22} & a_{23} \\
a_{31} & a_{32} & a_{33}
\end{pmatrix}
\begin{pmatrix}x_1\\x_2\\x_3\end{pmatrix}
=
\begin{pmatrix}b_1\\b_2\\b_3\end{pmatrix}
\quad\Rightarrow\quad
\begin{pmatrix}
a_{11}' & a_{12}' & a_{13}' \\
0 & a_{22}' & a_{23}' \\
0 & 0 & a_{33}'
\end{pmatrix}
\begin{pmatrix}x_1\\x_2\\x_3\end{pmatrix}
=
\begin{pmatrix}b_1'\\b_2'\\b_3'\end{pmatrix}$$

### **Partial Pivoting**

Before eliminating column $k$:
1. Find row $r$ where $|a_{rk}|$ is maximum among rows $k$ to $n$
2. Swap rows $k$ and $r$
3. Proceed with elimination

**Why Pivot?**
- Avoids division by zero
- Reduces floating-point rounding errors
- Improves numerical stability
- Essential for ill-conditioned matrices

### **Back Substitution**

Starting from the last equation, solve upward:
$$\begin{align}
x_n &= \frac{b_n'}{a_{nn}'} \\
x_{n-1} &= \frac{b_{n-1}' - a_{n-1,n}'x_n}{a_{n-1,n-1}'} \\
&\vdots \\
x_1 &= \frac{b_1' - \sum_{j=2}^n a_{1j}'x_j}{a_{11}'}
\end{align}$$

### **Complexity**

- **Forward elimination**: $O(n^3)$ - dominant cost
- **Back substitution**: $O(n^2)$
- **Total**: $O(n^3)$ operations
- **Space**: $O(1)$ if modifying matrix in-place

---

## 📊 Algorithm Comparison

| **Method** | **Time** | **Space** | **Best For** |
|------------|----------|-----------|--------------|
| Cramer's Rule | $O(n!)$ | $O(n^2)$ | Small systems (n ≤ 3) |
| Gaussian Elimination | $O(n^3)$ | $O(n^2)$ | General dense systems |
| LU Decomposition | $O(n^3)$ | $O(n^2)$ | Multiple right-hand sides |
| Iterative Methods | $O(kn^2)$ | $O(n^2)$ | Sparse systems, approximations |

---

## 💻 Rust Implementations

### **AoC 2023 Day 24 Part 1: Cramer's Rule for Line Intersection**

**Problem**: Find where two parametric lines intersect in 2D.

**Mathematical Model**:
```
Line 1: (x₁, y₁) + t·(vx₁, vy₁)
Line 2: (x₂, y₂) + s·(vx₂, vy₂)

Intersection when:
  x₁ + t·vx₁ = x₂ + s·vx₂
  y₁ + t·vy₁ = y₂ + s·vy₂

Rearranged:
  vx₁·t - vx₂·s = x₂ - x₁
  vy₁·t - vy₂·s = y₂ - y₁
```

**Solution using Cramer's Rule**:

```rust
fn intersects_2d(&self, other: &Hailstone) -> Option<(f64, f64, f64)> {
    // Coefficient matrix determinant
    // | vx₁  -vx₂ |
    // | vy₁  -vy₂ |
    let det = self.vx * other.vy - self.vy * other.vx;
    
    if det.abs() < 1e-10 { 
        return None;  // Parallel lines (det = 0)
    }
    
    let dx = other.px - self.px;
    let dy = other.py - self.py;
    
    // Cramer's rule:
    // t = det([dx, -vx₂; dy, -vy₂]) / det
    let t = (dx * other.vy - dy * other.vx) / det;
    
    // s = det([vx₁, dx; vy₁, dy]) / det
    let s = (dx * self.vy - dy * self.vx) / det;
    
    // Intersection point
    let ix = self.px + t * self.vx;
    let iy = self.py + t * self.vy;
    
    Some((ix, iy, t))
}
```

**Performance**: 
- O(1) per intersection test
- ~6.4 ns per pair (Day 24: 44,850 pairs in 315 µs)
- No matrix allocation needed

---

### **AoC 2023 Day 24 Part 2: Gaussian Elimination for 6×6 System**

**Problem**: Find rock trajectory (6 unknowns: rx, ry, rz, rvx, rvy, rvz) that collides with all hailstones.

**Mathematical Model**:
After [[cross-products-vector-algebra|cross-product elimination]], we have:
$$Ax = b \quad\text{where}\quad A \in \mathbb{R}^{6\times6}, \; b \in \mathbb{R}^6$$

**Implementation with Partial Pivoting**:

```rust
fn gaussian_elimination(matrix: &mut Vec<Vec<f64>>, rhs: &mut Vec<f64>) -> Vec<f64> {
    let n = matrix.len();
    
    // Forward elimination with partial pivoting
    for col in 0..n {
        // Find pivot (largest absolute value in column)
        let pivot_row = (col..n)
            .max_by(|&a, &b| {
                matrix[a][col].abs()
                    .partial_cmp(&matrix[b][col].abs())
                    .unwrap()
            })
            .unwrap();
        
        // Swap rows to move pivot to diagonal
        matrix.swap(col, pivot_row);
        rhs.swap(col, pivot_row);
        
        // Eliminate entries below pivot
        for row in (col + 1)..n {
            let factor = matrix[row][col] / matrix[col][col];
            for c in col..n {
                matrix[row][c] -= factor * matrix[col][c];
            }
            rhs[row] -= factor * rhs[col];
        }
    }
    
    // Back substitution
    let mut solution = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = rhs[i];
        for j in (i + 1)..n {
            sum -= matrix[i][j] * solution[j];
        }
        solution[i] = sum / matrix[i][i];
    }
    
    solution
}
```

**Performance**:
- 6×6 system: 216 operations (O(6³))
- ~25 µs to solve (Day 24 Part 2: 64 µs total)
- Partial pivoting adds minimal overhead but ensures stability

**Numerical Considerations**:
- Produces floating-point solution: `(200027938836082.375, ...)`
- Requires integer refinement (see Day 24 local search)
- Partial pivoting critical for accuracy

---

## 🔗 Related Concepts

### **Linear Independence**

Vectors $v_1, v_2, \ldots, v_n$ are **linearly independent** if:
$$c_1v_1 + c_2v_2 + \cdots + c_nv_n = 0 \quad\Rightarrow\quad c_1 = c_2 = \cdots = c_n = 0$$

**Relevance**: System $Ax = b$ has unique solution if columns of $A$ are linearly independent (equivalently, $\det(A) \neq 0$).

### **Determinant Properties**

- $\det(AB) = \det(A) \cdot \det(B)$
- $\det(A^T) = \det(A)$
- $\det(A^{-1}) = \frac{1}{\det(A)}$ (if $A$ invertible)
- Swapping rows changes sign: $\det(\text{swap}(A)) = -\det(A)$
- Determinant = 0 ⟺ rows/columns are linearly dependent

---

## 📚 Applications

### **Computer Graphics**
- Transformations (rotation, scaling, shear)
- Camera projections
- Lighting calculations

### **Physics Simulations**
- Force balance equations
- Circuit analysis (Kirchhoff's laws)
- Structural mechanics

### **Machine Learning**
- Linear regression (least squares)
- Principal Component Analysis (PCA)
- Neural network weight updates

### **Optimization**
- Linear programming (simplex method)
- Constraint satisfaction
- Resource allocation

---

## 🎓 Learning Path

1. **Start**: Solve 2×2 systems by hand using Cramer's rule
2. **Practice**: Implement Gaussian elimination for 3×3 systems
3. **Apply**: Use in geometric problems ([[parametric-equations]])
4. **Extend**: Learn LU decomposition, QR factorization
5. **Specialize**: Study sparse matrix methods, iterative solvers

---

## 🔗 Links

**Related Notes**:
- [[parametric-equations]] - Application of linear systems to line intersection
- [[cross-products-vector-algebra]] - Eliminating variables to create linear systems
- [[computational-geometry-basics]] - Geometric applications of linear algebra

**Implementations**:
- **AoC 2023 Day 24** (`advent_of_code/aoc2023/src/solver/day24.rs`)
  - Cramer's rule: `intersects_2d_in_bounds()`
  - Gaussian elimination: `gaussian_elimination()`
  - Performance: 379 µs total (Part 1: 315 µs, Part 2: 64 µs)

**External Resources**:
- [3Blue1Brown: Essence of Linear Algebra](https://www.youtube.com/playlist?list=PLZHQObOWTQDPD3MizzM2xVFitgF8hE_ab)
- Gilbert Strang: *Introduction to Linear Algebra*

---

*Tags: #mathematics #linear-algebra #systems #matrices #determinants #gaussian-elimination #cramers-rule #numerical-methods*
