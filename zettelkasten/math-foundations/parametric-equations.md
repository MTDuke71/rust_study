# Parametric Equations

**Field**: Analytic Geometry / Computational Geometry

**Prerequisites**: [[linear-algebra-fundamentals]], [[computational-geometry-basics]]

---

## 📐 Definition

A **parametric equation** expresses a geometric object using one or more parameters instead of Cartesian coordinates.

**General Form**:
$$\mathbf{r}(t) = \mathbf{P}_0 + t \cdot \mathbf{V}$$

where:
- $\mathbf{P}_0$ is a **point** on the object (position vector)
- $\mathbf{V}$ is a **direction vector**
- $t$ is the **parameter** (often representing time)

**Intuition**: Instead of describing "what points satisfy this equation," parametric form says "start here, go in this direction, vary speed by parameter $t$."

---

## 🔑 Parametric Line in 2D

### **Mathematical Form**

$$\begin{cases}
x(t) = x_0 + t \cdot v_x \\
y(t) = y_0 + t \cdot v_y
\end{cases}$$

**Components**:
- **Initial position**: $(x_0, y_0)$ - where the line passes through at $t = 0$
- **Direction vector**: $(v_x, v_y)$ - which way the line points
- **Parameter**: $t \in \mathbb{R}$ - distance along the line (if $\mathbf{V}$ is unit vector)

### **Geometric Interpretation**

```
           ↗ V = (vx, vy)  [direction]
          /
         /
    P₀ •  [starting point at t=0]
       /
      / 
     • t=1
    /
   • t=2
  /
 • t=-1
```

- $t = 0$: Point is at $\mathbf{P}_0$
- $t > 0$: Point moves in direction of $\mathbf{V}$
- $t < 0$: Point moves opposite to $\mathbf{V}$
- $t = 1$: Point is at $\mathbf{P}_0 + \mathbf{V}$

---

## 🎯 Parametric Line in 3D

### **Mathematical Form**

$$\begin{cases}
x(t) = x_0 + t \cdot v_x \\
y(t) = y_0 + t \cdot v_y \\
z(t) = z_0 + t \cdot v_z
\end{cases}$$

**Compact Vector Notation**:
$$\mathbf{r}(t) = \begin{pmatrix}x_0\\y_0\\z_0\end{pmatrix} + t \begin{pmatrix}v_x\\v_y\\v_z\end{pmatrix}$$

### **Converting from Two-Point Form**

Given two points $\mathbf{P}_1$ and $\mathbf{P}_2$ on a line:

$$\mathbf{r}(t) = \mathbf{P}_1 + t(\mathbf{P}_2 - \mathbf{P}_1)$$

where:
- $t = 0$ gives $\mathbf{P}_1$
- $t = 1$ gives $\mathbf{P}_2$
- $0 < t < 1$ gives points between $\mathbf{P}_1$ and $\mathbf{P}_2$ (line segment)

---

## 🔧 Line-Line Intersection (2D)

### **Problem Statement**

Given two lines:
- Line 1: $\mathbf{r}_1(t) = \mathbf{P}_1 + t \cdot \mathbf{V}_1$
- Line 2: $\mathbf{r}_2(s) = \mathbf{P}_2 + s \cdot \mathbf{V}_2$

Find parameters $t$ and $s$ where $\mathbf{r}_1(t) = \mathbf{r}_2(s)$.

### **System of Equations**

$$\begin{cases}
x_1 + t \cdot v_{x1} = x_2 + s \cdot v_{x2} \\
y_1 + t \cdot v_{y1} = y_2 + s \cdot v_{y2}
\end{cases}$$

**Rearranged** (standard linear system form):
$$\begin{cases}
v_{x1} \cdot t - v_{x2} \cdot s = x_2 - x_1 \\
v_{y1} \cdot t - v_{y2} \cdot s = y_2 - y_1
\end{cases}$$

### **Solution via Cramer's Rule**

See [[linear-algebra-fundamentals#Cramer's Rule]] for details.

**Determinant of coefficient matrix**:
$$D = v_{x1} \cdot v_{y2} - v_{y1} \cdot v_{x2}$$

**Special Cases**:
- $D = 0$: Lines are **parallel** (no intersection or infinite intersections)
- $D \neq 0$: Lines intersect at exactly one point

**Parameters**:
$$t = \frac{(x_2 - x_1) \cdot v_{y2} - (y_2 - y_1) \cdot v_{x2}}{D}$$
$$s = \frac{(x_2 - x_1) \cdot v_{y1} - (y_2 - y_1) \cdot v_{x1}}{D}$$

**Intersection point**:
$$\mathbf{P}_{\text{int}} = \mathbf{P}_1 + t \cdot \mathbf{V}_1 = (x_1 + t \cdot v_{x1}, \; y_1 + t \cdot v_{y1})$$

---

## 📊 Parametric vs. Cartesian Forms

| **Aspect** | **Parametric** | **Cartesian** |
|------------|----------------|---------------|
| **2D Line** | $x = x_0 + tv_x, \; y = y_0 + tv_y$ | $ax + by = c$ |
| **Vertical Line** | Easy: $v_x = 0$ | Problematic: slope undefined |
| **Direction** | Explicit: $\mathbf{V}$ | Implicit: perpendicular to $(a, b)$ |
| **Point on Line** | Trivial: plug in $t$ | Must check: $ax + by \stackrel{?}{=} c$ |
| **Intersection** | Solve 2×2 system for $t, s$ | Solve 2×2 for $x, y$ |
| **Ray/Segment** | Restrict $t$: $t \geq 0$ (ray), $0 \leq t \leq 1$ (segment) | Complex inequalities |

**Advantages of Parametric Form**:
✅ Handles all cases uniformly (no vertical line exceptions)  
✅ Natural time/motion interpretation  
✅ Easy to restrict to rays ($t \geq 0$) or segments ($t \in [0, 1]$)  
✅ Direction vector explicit  
✅ Simple to generate points on the line

---

## 💻 Rust Implementation

### **AoC 2023 Day 24: Hailstone Trajectory Intersection**

**Problem**: Determine if two hailstone paths will intersect in the future within a test area.

**Data Structure**:

```rust
#[derive(Debug, Clone)]
struct Hailstone {
    // Position at t=0
    px: f64,
    py: f64,
    pz: f64,
    // Velocity (direction vector)
    vx: f64,
    vy: f64,
    vz: f64,
}

// Parametric form: (px, py, pz) + t·(vx, vy, vz)
```

**Parsing from Input**:

```rust
impl Hailstone {
    fn parse(line: &str) -> Result<Self> {
        // Input: "19, 13, 30 @ -2, 1, -2"
        let parts: Vec<&str> = line.split('@').collect();
        let pos: Vec<f64> = parts[0]
            .split(',')
            .map(|s| s.trim().parse())
            .collect::<Result<_, _>>()?;
        let vel: Vec<f64> = parts[1]
            .split(',')
            .map(|s| s.trim().parse())
            .collect::<Result<_, _>>()?;
        
        Ok(Hailstone {
            px: pos[0], py: pos[1], pz: pos[2],
            vx: vel[0], vy: vel[1], vz: vel[2],
        })
    }
}
```

**2D Intersection Test**:

```rust
impl Hailstone {
    fn intersects_2d_in_bounds(&self, other: &Hailstone, bounds: (f64, f64)) -> bool {
        // Determinant: check if parallel
        let det = self.vx * other.vy - self.vy * other.vx;
        if det.abs() < 1e-10 {
            return false;  // Parallel lines never intersect
        }
        
        // Cramer's rule for parameters t and s
        let dx = other.px - self.px;
        let dy = other.py - self.py;
        let t = (dx * other.vy - dy * other.vx) / det;
        let s = (dx * self.vy - dy * self.vx) / det;
        
        // Check if intersection is in the future for BOTH
        if t < 0.0 || s < 0.0 {
            return false;  // Past collision doesn't count
        }
        
        // Calculate intersection point using parameter t
        let ix = self.px + t * self.vx;
        let iy = self.py + t * self.vy;
        
        // Check if within test area bounds
        ix >= bounds.0 && ix <= bounds.1 && 
        iy >= bounds.0 && iy <= bounds.1
    }
}
```

**Performance**:
- O(1) per intersection test
- ~6.4 ns per pair
- 44,850 pairs tested in 315 µs

---

## 🎯 Applications

### **Ray Tracing**

Rays from camera through pixels:
$$\mathbf{r}(t) = \mathbf{camera} + t \cdot \mathbf{direction}, \quad t \geq 0$$

Find intersection with scene geometry to determine color.

### **Collision Detection**

Moving objects represented as:
$$\mathbf{pos}(t) = \mathbf{pos}_0 + t \cdot \mathbf{velocity}$$

Detect when $|\mathbf{pos}_1(t) - \mathbf{pos}_2(t)| < r_1 + r_2$ (sum of radii).

### **Animation**

Linear interpolation (lerp) between keyframes:
$$\mathbf{pos}(t) = (1-t) \mathbf{start} + t \cdot \mathbf{end}, \quad t \in [0, 1]$$

### **Pathfinding**

Check if line segment from A to B intersects obstacles:
$$\mathbf{path}(t) = \mathbf{A} + t(\mathbf{B} - \mathbf{A}), \quad t \in [0, 1]$$

---

## 🔬 Advanced Topics

### **Line-Plane Intersection (3D)**

Plane: $\mathbf{n} \cdot \mathbf{r} = d$ (normal form)  
Line: $\mathbf{r}(t) = \mathbf{P}_0 + t\mathbf{V}$

**Substitute**:
$$\mathbf{n} \cdot (\mathbf{P}_0 + t\mathbf{V}) = d$$
$$t = \frac{d - \mathbf{n} \cdot \mathbf{P}_0}{\mathbf{n} \cdot \mathbf{V}}$$

If $\mathbf{n} \cdot \mathbf{V} = 0$: line is parallel to plane.

### **Closest Point on Line to External Point**

Given line $\mathbf{r}(t) = \mathbf{P}_0 + t\mathbf{V}$ and point $\mathbf{Q}$:

$$t_{\text{closest}} = \frac{(\mathbf{Q} - \mathbf{P}_0) \cdot \mathbf{V}}{\mathbf{V} \cdot \mathbf{V}}$$

(Derived by minimizing $|\mathbf{r}(t) - \mathbf{Q}|^2$)

### **Line-Line Distance (3D)**

Lines in 3D generally do not intersect (they are **skew lines**).

**Minimum distance**:
$$d = \frac{|(\mathbf{P}_2 - \mathbf{P}_1) \cdot (\mathbf{V}_1 \times \mathbf{V}_2)|}{|\mathbf{V}_1 \times \mathbf{V}_2|}$$

Uses [[cross-products-vector-algebra|cross product]] to find perpendicular direction.

---

## 🔗 Links

**Related Notes**:
- [[linear-algebra-fundamentals]] - Cramer's rule for solving intersection systems
- [[cross-products-vector-algebra]] - 3D line operations, skew lines
- [[computational-geometry-basics]] - Applications in geometry

**Implementations**:
- **AoC 2023 Day 24 Part 1** (`advent_of_code/aoc2023/src/solver/day24.rs`)
  - `Hailstone` struct: Parametric line representation
  - `intersects_2d_in_bounds()`: Future intersection detection
  - Performance: 315 µs for 44,850 intersection tests

**External Resources**:
- [Paul's Online Math Notes: Parametric Equations](https://tutorial.math.lamar.edu/Classes/CalcII/ParametricEqn.aspx)
- [Khan Academy: Parametric Equations](https://www.khanacademy.org/math/precalculus/x9e81a4f98389efdf:parametric)

---

*Tags: #mathematics #geometry #parametric-equations #lines #intersection #computational-geometry #vectors*
