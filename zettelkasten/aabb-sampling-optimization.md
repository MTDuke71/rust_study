# AABB Sampling Optimization

**Tags:** #optimization #computational-geometry #sampling #performance #aoc #rectangle-validation #spatial-algorithms  
**Created:** 2025-12-09  
**Related:** [[computational-geometry]], [[sparse-data-structures]], [[ray-casting-algorithm]], [[aoc-2025-day09]]

---

## 🎯 Core Concept

**AABB Sampling** (Axis-Aligned Bounding Box Sampling) is an optimization technique that validates geometric constraints by checking **strategic sample points** instead of exhaustively iterating all points in a region.

**Key Insight**: For many geometric validation problems, checking corners, edges, and a sparse interior grid is sufficient to determine if a large rectangle satisfies constraints.

## 🧠 The Problem

### **Naive Approach Fails at Scale**

When validating large rectangles (e.g., 89,340 × 17,229 tiles = 1.54 billion points):

```rust
// ❌ TIMEOUT: Checking every single point
fn rectangle_valid_naive(x1: i64, y1: i64, x2: i64, y2: i64, 
                         validator: impl Fn(i64, i64) -> bool) -> bool {
    for x in x1..=x2 {
        for y in y1..=y2 {
            if !validator(x, y) {
                return false;
            }
        }
    }
    true
}
// Complexity: O(width × height) = billions of checks!
```

**Example from AoC 2025 Day 9**:
- Largest rectangles: 60K-90K tiles wide × 10K-20K tiles tall
- Total checks needed: 600M-1.8B per rectangle × ray casting = 2.3 trillion operations
- Result: **10-second timeout**

## ✅ AABB Sampling Solution

### **Strategic Point Selection**

Instead of checking ALL points, check:
1. **4 Corners** - Most critical boundary points
2. **Edge samples** - Points along all 4 edges at regular intervals
3. **Interior grid** - Sparse sampling of interior space

```rust
/// Validate rectangle using AABB sampling
fn rectangle_in_polygon_optimized(
    x1: i64, y1: i64, x2: i64, y2: i64,
    point_validator: impl Fn(i64, i64) -> bool
) -> bool {
    let width = (x2 - x1 + 1) as usize;
    let height = (y2 - y1 + 1) as usize;
    
    // Adaptive sample rate: larger rectangles need more samples
    let sample_rate = max(10, max(width, height) / 100);
    
    // 1. Check all 4 corners (CRITICAL)
    let corners = [(x1, y1), (x2, y1), (x1, y2), (x2, y2)];
    for &(x, y) in &corners {
        if !point_validator(x, y) {
            return false;
        }
    }
    
    // 2. Check top/bottom edges at intervals
    for x in (x1..=x2).step_by(sample_rate) {
        if !point_validator(x, y1) || !point_validator(x, y2) {
            return false;
        }
    }
    
    // 3. Check left/right edges at intervals
    for y in (y1..=y2).step_by(sample_rate) {
        if !point_validator(x1, y) || !point_validator(x2, y) {
            return false;
        }
    }
    
    // 4. Check interior grid at intervals
    for x in (x1..=x2).step_by(sample_rate) {
        for y in (y1..=y2).step_by(sample_rate) {
            if !point_validator(x, y) {
                return false;
            }
        }
    }
    
    true
}
```

### **Complexity Analysis**

**Naive approach**: `O(W × H)` where W, H are rectangle dimensions
- Example: 89,340 × 17,229 = **1.54 billion checks**

**AABB sampling**: `O(W/r + H/r + (W/r × H/r))` where r = sample_rate
- Sample rate: `r = max(10, max(W,H)/100)`
- For 89,340 × 17,229 rectangle: `r = 893`
- Corners: **4 checks**
- Edges: `2×(89340/893) + 2×(17229/893)` ≈ **240 checks**
- Interior: `(89340/893) × (17229/893)` ≈ **2,000 checks**
- **Total: ~2,244 checks** (vs 1.54 billion!)
- **Speedup: 686,000x**

## 🔑 Key Design Decisions

### **1. Adaptive Sample Rate**

```rust
let sample_rate = max(10, max(width, height) / 100);
```

**Why adaptive?**
- Small rectangles (100×100): Use minimum 10 → 10×10 = 100 interior samples
- Large rectangles (100,000×50,000): Use 1000 → 100×50 = 5,000 interior samples
- Scales logarithmically, not linearly with area

### **2. Corner Priority**

Always check corners **first** - they're most likely to violate constraints:
- Rectangle outside polygon? Corner check fails immediately
- Rectangle partially outside? Edge or corner catches it

### **3. Separate Edge and Interior Sampling**

```rust
// Edges get special treatment
for x in (x1..=x2).step_by(sample_rate) {
    check(x, y1);  // Top edge
    check(x, y2);  // Bottom edge
}

// Interior uses grid sampling
for x in (x1..=x2).step_by(sample_rate) {
    for y in (y1..=y2).step_by(sample_rate) {
        check(x, y);
    }
}
```

**Why separate?** Edges are 1D (cheaper), interior is 2D (more expensive). Fail fast on edges before checking interior.

## 📊 Performance Results (AoC 2025 Day 9)

### **Before AABB Sampling: Naive Ray Casting**
- Approach: Check every tile in rectangle
- Largest rectangle: 89,340 × 17,229 = 1.54B tiles
- Each tile: Ray casting through 496 polygon edges
- Total operations: **2.3 trillion**
- Result: **Timeout (>10 seconds)**

### **After AABB Sampling**
- Sample points per rectangle: ~2,000-10,000
- Each sample: Ray casting through 496 edges
- Total rectangles checked: 122,760
- Total operations: **~633 million** (vs 2.3 trillion)
- Result: **3-5 seconds** ✅
- **Speedup: 3,650x**

## 🚨 When AABB Sampling Works

✅ **Good fit:**
- **Convex constraints**: Point-in-polygon, point-in-circle, collision detection
- **Continuous properties**: Temperature fields, elevation maps
- **Large regions**: Where exhaustive checking is prohibitive
- **Smooth boundaries**: Where interior/edge distinction is meaningful

❌ **Poor fit:**
- **Discrete pixel-perfect requirements**: Every single pixel must match exactly
- **Adversarial inputs**: Pathological cases designed to break sampling
- **Non-continuous properties**: Checkerboard patterns, random noise
- **Security-critical validation**: Where "probably correct" isn't good enough

## 🎓 Learning Insights

### **From AoC 2025 Day 9**

**The Journey**:
1. **Attempt 1**: Dense Grid → 37GB memory crash
2. **Attempt 2**: Sparse HashSet with interior → 18GB crash
3. **Attempt 3**: Boundary-only + naive ray casting → Timeout
4. **Attempt 4**: AABB sampling + ray casting → **SUCCESS!**

**Key Lessons**:
- **Input scale matters**: 8-point example works with full iteration, 496-point real input requires optimization
- **Strategic sampling**: Checking 0.0001% of points can validate constraints with high confidence
- **Adaptive algorithms**: Sample rate should scale with problem size
- **Fail fast**: Check corners/edges before expensive interior validation

### **Reddit Community Hint**

The breakthrough came from a Reddit hint: *"Use AABB collision or compacted space"*

This demonstrates the value of:
- Community knowledge sharing
- Recognizing when to pivot from implementation to research
- Understanding that not all optimizations are obvious - learning from others is key

## 🔗 Related Patterns

### **Spatial Optimization Techniques**

1. **[[sparse-data-structures]]** - When to avoid grid materialization
2. **[[ray-casting-algorithm]]** - Point-in-polygon validation used by AABB sampling
3. **[[computational-geometry]]** - Geometric algorithms foundation
4. **[[bresenham-line-algorithm]]** - Discrete line drawing for polygon boundaries

### **Performance Optimization Strategies**

1. **Early termination** - Check corners first, fail fast
2. **Adaptive sampling** - Scale sample density with problem size
3. **Avoiding sqrt()** - Use squared distances when possible (see [[aoc-2025-day08]])
4. **Lazy evaluation** - Only compute what you need

## 💡 Implementation Tips

### **Choosing Sample Rate**

```rust
// Too few samples: False positives (invalid rectangles pass)
let sample_rate = 1000;  // ❌ Only 100 samples for 100K rectangle

// Too many samples: Performance degrades
let sample_rate = 10;    // ❌ 10,000 samples for 100K rectangle

// Just right: Adaptive to problem scale
let sample_rate = max(10, max(width, height) / 100);  // ✅ Scales logarithmically
```

### **When to Sample More Aggressively**

Increase sampling (smaller rate) if:
- Validation is cheap (simple bounds check)
- Constraints are complex (many holes, concave polygons)
- False positives are costly (triggering expensive downstream work)

Decrease sampling (larger rate) if:
- Validation is expensive (ray casting, database lookups)
- Constraints are simple (convex shapes, single boundary)
- False negatives are acceptable (heuristic search)

## 🏆 Competitive Programming Applications

### **Common AoC Patterns**

| **Problem Type** | **AABB Application** | **Example** |
|------------------|---------------------|-------------|
| Rectangle in polygon | Sample corners/edges/interior | 2025 Day 9 |
| Collision detection | AABB pre-filter before exact check | Physics simulations |
| Range queries | Sparse grid sampling | 2D prefix sums |
| Flood fill bounds | Check perimeter before full fill | Connected components |

### **When to Reach for AABB Sampling**

**🚩 Red flags that suggest AABB optimization:**
- Nested loops over large coordinate ranges
- Timeout on geometric validation
- Checking every point in a rectangle/region
- Brute force passes example, times out on real input

**✅ Green lights for applying AABB:**
- Convex or simple polygonal constraints
- Large rectangles/regions to validate
- Expensive per-point validation (ray casting, database query)
- Continuous or smooth spatial properties

---

## 📚 References

**AoC 2025 Day 9**:
- [[aoc-2025-day09]] - Rectangle area maximization problem
- [[../advent_of_code/aoc2025/examples/day09_approaches]] - Full journey including AABB breakthrough
- Implementation: `advent_of_code/aoc2025/src/solver/day09.rs`

**Related Concepts**:
- [[computational-geometry]] - Geometric algorithm foundations
- [[ray-casting-algorithm]] - Point-in-polygon testing
- [[sparse-data-structures]] - When to avoid materialization
- [[performance-optimization]] - General optimization strategies

**External Resources**:
- Axis-Aligned Bounding Boxes in collision detection
- Monte Carlo sampling methods
- Spatial hashing and grid acceleration structures

---

*AABB sampling exemplifies a fundamental principle in algorithm design: **strategic approximation can achieve acceptable correctness at drastically reduced cost**. By understanding where geometric constraints are most likely to fail (corners, edges), we can validate massive regions by checking a tiny fraction of points. This pattern appears throughout computational geometry, computer graphics, and spatial databases - anywhere exhaustive checking becomes prohibitively expensive.*

**Links:** [[computational-geometry]] | [[sparse-data-structures]] | [[ray-casting-algorithm]] | [[aoc-2025-day09]] | [[performance-optimization]] | [[adaptive-algorithms]]
