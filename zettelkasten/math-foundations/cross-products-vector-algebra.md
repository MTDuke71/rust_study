# Cross Products and Vector Algebra

**Field**: Linear Algebra / Vector Calculus

**Prerequisites**: [[linear-algebra-fundamentals]], [[parametric-equations]]

---

## 📐 Definition

The **cross product** (or **vector product**) of two 3D vectors produces a vector perpendicular to both:

$$\mathbf{a} \times \mathbf{b} = \begin{pmatrix}a_1\\a_2\\a_3\end{pmatrix} \times \begin{pmatrix}b_1\\b_2\\b_3\end{pmatrix} = \begin{pmatrix}
a_2b_3 - a_3b_2 \\
a_3b_1 - a_1b_3 \\
a_1b_2 - a_2b_1
\end{pmatrix}$$

**Mnemonic (Right-Hand Rule)**:
```
      k (z-axis)
      ↑
      |
      |___→ i (x-axis)
     ↙
    j (y-axis)

i × j = k
j × k = i
k × i = j
```

**Magnitude**: $|\mathbf{a} \times \mathbf{b}| = |\mathbf{a}| \cdot |\mathbf{b}| \cdot \sin\theta$

where $\theta$ is the angle between vectors.

---

## 🔑 Key Properties

### **Algebraic Properties**

1. **Anti-commutativity**: $\mathbf{a} \times \mathbf{b} = -(\mathbf{b} \times \mathbf{a})$
2. **Not associative**: $(\mathbf{a} \times \mathbf{b}) \times \mathbf{c} \neq \mathbf{a} \times (\mathbf{b} \times \mathbf{c})$ in general
3. **Distributive**: $\mathbf{a} \times (\mathbf{b} + \mathbf{c}) = \mathbf{a} \times \mathbf{b} + \mathbf{a} \times \mathbf{c}$
4. **Scalar multiplication**: $(k\mathbf{a}) \times \mathbf{b} = k(\mathbf{a} \times \mathbf{b}) = \mathbf{a} \times (k\mathbf{b})$

### **Geometric Properties**

- **Perpendicularity**: $\mathbf{a} \times \mathbf{b}$ is perpendicular to both $\mathbf{a}$ and $\mathbf{b}$
  - Proof: $\mathbf{a} \cdot (\mathbf{a} \times \mathbf{b}) = 0$ and $\mathbf{b} \cdot (\mathbf{a} \times \mathbf{b}) = 0$

- **Parallel vectors**: $\mathbf{a} \times \mathbf{b} = \mathbf{0}$ iff $\mathbf{a}$ and $\mathbf{b}$ are parallel

- **Area interpretation**: $|\mathbf{a} \times \mathbf{b}|$ equals the area of parallelogram with sides $\mathbf{a}$ and $\mathbf{b}$

---

## 🎯 Applications

### **1. Finding Perpendicular Vectors**

**Problem**: Given two vectors defining a plane, find the normal vector.

**Solution**: $\mathbf{n} = \mathbf{a} \times \mathbf{b}$

### **2. Area of Triangle**

Given vertices $\mathbf{P}, \mathbf{Q}, \mathbf{R}$:

$$\text{Area} = \frac{1}{2} |(\mathbf{Q} - \mathbf{P}) \times (\mathbf{R} - \mathbf{P})|$$

### **3. Torque in Physics**

$$\boldsymbol{\tau} = \mathbf{r} \times \mathbf{F}$$

where $\mathbf{r}$ is position vector and $\mathbf{F}$ is force.

### **4. Angular Momentum**

$$\mathbf{L} = \mathbf{r} \times \mathbf{p}$$

where $\mathbf{p} = m\mathbf{v}$ is linear momentum.

---

## 🔧 Variable Elimination Technique

### **The Problem: Nonlinear Systems**

Consider finding a rock trajectory that collides with multiple hailstones:

**Collision equations** (rock at time $t_i$ equals hailstone $i$ at time $t_i$):
$$\begin{cases}
r_x + t_i \cdot rv_x = p_{x,i} + t_i \cdot v_{x,i} \\
r_y + t_i \cdot rv_y = p_{y,i} + t_i \cdot v_{y,i} \\
r_z + t_i \cdot rv_z = p_{z,i} + t_i \cdot v_{z,i}
\end{cases}$$

**Issue**: 
- 6 unknowns: $(r_x, r_y, r_z, rv_x, rv_y, rv_z)$
- But equations contain **products** $t_i \cdot rv_x$ (quadratic terms!)
- Cannot use linear solvers (Gaussian elimination, etc.)
- Cannot use Integer Linear Programming (ILP) - requires linear constraints only

---

## 🎯 Cross-Product Elimination

### **The Trick: Eliminate Time Variables**

**Step 1: Take Differences**

Subtract collision equations for hailstones $j$ and $k$:

$$\begin{align}
(p_{x,j} - p_{x,k}) + t_j v_{x,j} - t_k v_{x,k} &= t_j rv_x - t_k rv_x \\
(p_{y,j} - p_{y,k}) + t_j v_{y,j} - t_k v_{y,k} &= t_j rv_y - t_k rv_y \\
(p_{z,j} - p_{z,k}) + t_j v_{z,j} - t_k v_{z,k} &= t_j rv_z - t_k rv_z
\end{align}$$

**Step 2: Cross Product with Velocity Difference**

Let:
- $\Delta \mathbf{P} = \mathbf{P}_j - \mathbf{P}_k$
- $\Delta \mathbf{V} = \mathbf{V}_j - \mathbf{V}_k$

The equation becomes:
$$\Delta \mathbf{P} + t_j \mathbf{V}_j - t_k \mathbf{V}_k = (t_j - t_k) \mathbf{RV}$$

**Cross both sides with $\Delta \mathbf{V}$**:
$$\Delta \mathbf{P} \times \Delta \mathbf{V} + (t_j \mathbf{V}_j - t_k \mathbf{V}_k) \times \Delta \mathbf{V} = (t_j - t_k) \mathbf{RV} \times \Delta \mathbf{V}$$

**Key insight**: 
- $\mathbf{V}_j \times (\mathbf{V}_j - \mathbf{V}_k) = \mathbf{V}_j \times \mathbf{V}_j - \mathbf{V}_j \times \mathbf{V}_k = -\mathbf{V}_j \times \mathbf{V}_k$
- $\mathbf{V}_k \times (\mathbf{V}_j - \mathbf{V}_k) = \mathbf{V}_k \times \mathbf{V}_j - \mathbf{V}_k \times \mathbf{V}_k = \mathbf{V}_k \times \mathbf{V}_j$

**Simplified**:
$$\Delta \mathbf{P} \times \Delta \mathbf{V} = \Delta \mathbf{P} \times \mathbf{RV} + \mathbf{RP} \times \Delta \mathbf{V}$$

**Result**: Left side is **constant** (known positions/velocities), right side is **linear** in unknowns $(r_x, r_y, r_z, rv_x, rv_y, rv_z)$!

---

## 💻 Rust Implementation

### **AoC 2023 Day 24 Part 2: Building Linear System**

```rust
fn build_linear_system_from_hailstones(hailstones: &[Hailstone]) 
    -> (Vec<Vec<f64>>, Vec<f64>) 
{
    let mut matrix = vec![vec![0.0; 6]; 6];  // 6×6 system
    let mut rhs = vec![0.0; 6];               // Right-hand side
    
    let h0 = &hailstones[0];  // Reference hailstone
    
    // Use hailstones 1 and 2 to create 6 equations (3 each)
    for (eq_base, &hi) in [1, 2].iter().enumerate() {
        let h = &hailstones[hi];
        
        // Position difference: ΔP = P_hi - P_0
        let dp = (h.px - h0.px, h.py - h0.py, h.pz - h0.pz);
        
        // Velocity difference: ΔV = V_hi - V_0
        let dv = (h.vx - h0.vx, h.vy - h0.vy, h.vz - h0.vz);
        
        // Cross product: ΔP × ΔV (constant term)
        let cross_pdv = (
            dp.1 * dv.2 - dp.2 * dv.1,  // y*vz - z*vy
            dp.2 * dv.0 - dp.0 * dv.2,  // z*vx - x*vz
            dp.0 * dv.1 - dp.1 * dv.0,  // x*vy - y*vx
        );
        
        let row_offset = eq_base * 3;
        
        // Equation: ΔP × ΔV = ΔP × RV + RP × ΔV
        // Expand: (dp.y*dv.z - dp.z*dv.y) = 
        //           (dp.y*rvz - dp.z*rvy) + (ry*dv.z - rz*dv.y)
        
        // X-component equation (row 0 or 3)
        matrix[row_offset][0] = 0.0;        // rx coefficient
        matrix[row_offset][1] = dv.2;       // ry coefficient (from RP × ΔV)
        matrix[row_offset][2] = -dv.1;      // rz coefficient
        matrix[row_offset][3] = 0.0;        // rvx coefficient
        matrix[row_offset][4] = -dp.2;      // rvy coefficient (from ΔP × RV)
        matrix[row_offset][5] = dp.1;       // rvz coefficient
        rhs[row_offset] = cross_pdv.0;
        
        // Y-component equation (row 1 or 4)
        matrix[row_offset + 1][0] = -dv.2;
        matrix[row_offset + 1][1] = 0.0;
        matrix[row_offset + 1][2] = dv.0;
        matrix[row_offset + 1][3] = dp.2;
        matrix[row_offset + 1][4] = 0.0;
        matrix[row_offset + 1][5] = -dp.0;
        rhs[row_offset + 1] = cross_pdv.1;
        
        // Z-component equation (row 2 or 5)
        matrix[row_offset + 2][0] = dv.1;
        matrix[row_offset + 2][1] = -dv.0;
        matrix[row_offset + 2][2] = 0.0;
        matrix[row_offset + 2][3] = -dp.1;
        matrix[row_offset + 2][4] = dp.0;
        matrix[row_offset + 2][5] = 0.0;
        rhs[row_offset + 2] = cross_pdv.2;
    }
    
    (matrix, rhs)
}
```

**Result**: 6×6 linear system ready for [[linear-algebra-fundamentals#Gaussian Elimination]]!

---

## 📊 Why This Works

### **Mathematical Justification**

**Original problem**: 
- Nonlinear (contains $t_i \cdot rv_x$ products)
- Cannot use standard linear solvers
- ILP solvers reject quadratic terms

**After cross-product elimination**:
- All time variables $t_i$ eliminated
- System is linear in $(r_x, r_y, r_z, rv_x, rv_y, rv_z)$
- Gaussian elimination gives exact solution (up to floating-point precision)

**Key property used**: Cross product distributes over addition and scalar multiplication, allowing algebraic manipulation to separate terms.

---

## 🎓 Learning Path

### **Understanding Cross Products**

1. **Geometric intuition**: Right-hand rule, perpendicularity
2. **Component calculation**: Practice expanding determinant form
3. **Properties**: Anti-commutativity, distributivity
4. **Applications**: Area calculations, normal vectors

### **Variable Elimination**

1. **Identify nonlinear terms**: Products of unknowns
2. **Find algebraic operations** that cancel problematic terms
3. **Cross products**: Eliminate time/parameter variables
4. **Verify linearity**: Ensure resulting system is linear

---

## 🔬 Advanced: Triple Products

### **Scalar Triple Product**

$$\mathbf{a} \cdot (\mathbf{b} \times \mathbf{c}) = \det\begin{pmatrix}
a_1 & a_2 & a_3 \\
b_1 & b_2 & b_3 \\
c_1 & c_2 & c_3
\end{pmatrix}$$

**Geometric meaning**: Volume of parallelepiped with edges $\mathbf{a}, \mathbf{b}, \mathbf{c}$.

### **Vector Triple Product**

$$\mathbf{a} \times (\mathbf{b} \times \mathbf{c}) = \mathbf{b}(\mathbf{a} \cdot \mathbf{c}) - \mathbf{c}(\mathbf{a} \cdot \mathbf{b})$$

**Mnemonic**: "BAC minus CAB"

---

## 🔗 Links

**Related Notes**:
- [[linear-algebra-fundamentals]] - Linear systems, Gaussian elimination
- [[parametric-equations]] - Lines in 3D, collision detection
- [[computational-geometry-basics]] - Geometric applications

**Implementations**:
- **AoC 2023 Day 24 Part 2** (`advent_of_code/aoc2023/src/solver/day24.rs`)
  - `build_linear_system_from_hailstones()`: Cross-product elimination
  - `solve_part2_linear_system()`: Full solution pipeline
  - Performance: 64 µs (constant time, O(1) in hailstone count)
  - Complexity: 6×6 system → O(6³) = 216 operations

**External Resources**:
- [3Blue1Brown: Cross Products](https://www.youtube.com/watch?v=eu6i7WJeinw)
- [Paul's Online Math Notes: Cross Product](https://tutorial.math.lamar.edu/Classes/CalcIII/CrossProduct.aspx)

---

*Tags: #mathematics #linear-algebra #vector-algebra #cross-product #variable-elimination #computational-geometry #3d-geometry*
