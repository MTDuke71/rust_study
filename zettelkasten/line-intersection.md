# Line Intersection

*Date: 2024-12-19*
*Tags: #computational-geometry #algorithms #math #aoc-pattern*

## Overview

Line intersection is a fundamental computational geometry problem that determines whether and where two line segments intersect. This concept appears frequently in graphics, collision detection, pathfinding, and competitive programming problems.

## Types of Line Intersection

### 1. **Infinite Line Intersection**
Two lines in 2D space represented as $ax + by = c$ will intersect at exactly one point unless they are parallel or coincident.

**Formula**: For lines $L_1: a_1x + b_1y = c_1$ and $L_2: a_2x + b_2y = c_2$:
- Determinant: $D = a_1b_2 - a_2b_1$
- If $D = 0$: Lines are parallel (no intersection or infinite intersections)
- If $D \neq 0$: Intersection at $(x, y) = (\frac{c_1b_2 - c_2b_1}{D}, \frac{a_1c_2 - a_2c_1}{D})$

### 2. **Line Segment Intersection**
More common in practice - determines if two finite line segments intersect.

**Key Insight**: Use cross products to check orientation:
- Three points $(p, q, r)$ are clockwise, counterclockwise, or collinear based on the sign of $(q - p) \times (r - p)$

**Algorithm** (Orientation-based):
1. Compute orientations of triplets: $(p_1, q_1, p_2)$, $(p_1, q_1, q_2)$, $(p_2, q_2, p_1)$, $(p_2, q_2, q_1)$
2. **General case**: Segments intersect if orientations differ
3. **Special case**: If collinear, check if points lie on each segment (bounding box test)

## Rust Implementation Pattern

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    p1: Point,
    p2: Point,
}

impl Segment {
    fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }
}

/// Compute orientation of ordered triplet (p, q, r)
/// Returns: 0 = collinear, 1 = clockwise, 2 = counterclockwise
fn orientation(p: Point, q: Point, r: Point) -> i32 {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
    if val.abs() < 1e-10 { 0 } // Collinear
    else if val > 0.0 { 1 }     // Clockwise
    else { 2 }                  // Counterclockwise
}

/// Check if point q lies on segment pr (assumes collinearity)
fn on_segment(p: Point, q: Point, r: Point) -> bool {
    q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) &&
    q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y)
}

/// Check if two line segments intersect
fn segments_intersect(seg1: Segment, seg2: Segment) -> bool {
    let o1 = orientation(seg1.p1, seg1.p2, seg2.p1);
    let o2 = orientation(seg1.p1, seg1.p2, seg2.p2);
    let o3 = orientation(seg2.p1, seg2.p2, seg1.p1);
    let o4 = orientation(seg2.p1, seg2.p2, seg1.p2);

    // General case: different orientations
    if o1 != o2 && o3 != o4 {
        return true;
    }

    // Special cases: collinear points
    if o1 == 0 && on_segment(seg1.p1, seg2.p1, seg1.p2) { return true; }
    if o2 == 0 && on_segment(seg1.p1, seg2.p2, seg1.p2) { return true; }
    if o3 == 0 && on_segment(seg2.p1, seg1.p1, seg2.p2) { return true; }
    if o4 == 0 && on_segment(seg2.p1, seg1.p2, seg2.p2) { return true; }

    false
}

/// Find intersection point of two infinite lines (if it exists)
fn line_intersection(seg1: Segment, seg2: Segment) -> Option<Point> {
    let x1 = seg1.p1.x;
    let y1 = seg1.p1.y;
    let x2 = seg1.p2.x;
    let y2 = seg1.p2.y;
    let x3 = seg2.p1.x;
    let y3 = seg2.p1.y;
    let x4 = seg2.p2.x;
    let y4 = seg2.p2.y;

    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    
    if denom.abs() < 1e-10 {
        return None; // Parallel or coincident
    }

    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
    
    Some(Point::new(
        x1 + t * (x2 - x1),
        y1 + t * (y2 - y1),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segments_intersect() {
        let seg1 = Segment::new(Point::new(0.0, 0.0), Point::new(2.0, 2.0));
        let seg2 = Segment::new(Point::new(0.0, 2.0), Point::new(2.0, 0.0));
        assert!(segments_intersect(seg1, seg2));
    }

    #[test]
    fn test_segments_no_intersect() {
        let seg1 = Segment::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
        let seg2 = Segment::new(Point::new(2.0, 0.0), Point::new(3.0, 1.0));
        assert!(!segments_intersect(seg1, seg2));
    }

    #[test]
    fn test_line_intersection_point() {
        let seg1 = Segment::new(Point::new(0.0, 0.0), Point::new(2.0, 2.0));
        let seg2 = Segment::new(Point::new(0.0, 2.0), Point::new(2.0, 0.0));
        let intersection = line_intersection(seg1, seg2).unwrap();
        assert!((intersection.x - 1.0).abs() < 1e-10);
        assert!((intersection.y - 1.0).abs() < 1e-10);
    }
}
```

## Common Applications

### Advent of Code Patterns
- **Ray tracing**: Follow paths until intersection with walls/boundaries
- **Wire crossing**: Find where paths cross (AoC 2019 Day 3)
- **Polygon intersections**: Check if shapes overlap
- **Visibility graphs**: Determine line-of-sight between points
- **Laser/beam problems**: Trace reflections and intersections

### Performance Characteristics
- **Time Complexity**: $O(1)$ per intersection check
- **Space Complexity**: $O(1)$
- **Bulk intersections**: Use sweep line algorithm for $O(n \log n + k)$ where $k$ = number of intersections

## Edge Cases to Handle

1. **Floating-point precision**: Use epsilon comparisons (`1e-10`) for near-zero values
2. **Collinear segments**: Require special handling with bounding box checks
3. **Endpoint intersections**: May need to treat differently than mid-segment intersections
4. **Parallel lines**: Determinant is zero, no unique intersection
5. **Overlapping segments**: Collinear with multiple intersection points

## Advanced Techniques

### Sweep Line Algorithm
For finding all intersections among $n$ line segments:
1. Sort endpoints by x-coordinate
2. Maintain active segments in a balanced BST
3. Check only adjacent segments in sweep order
4. **Complexity**: $O((n + k) \log n)$ where $k$ = intersections

### Integer Coordinates
When working with integer grids (common in AoC):
- Use cross products with integers to avoid floating-point errors
- Orientation test becomes: `sign((q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y))`
- More robust for exact geometric predicates

## Related Concepts

### Mission Connections
- [[mission-6]]: Grid navigation where line intersection determines valid paths
- [[mission-8]]: Graph algorithms where edges represent line segments

### Geometric Algorithms
- [[computational-geometry]]: Broader field of geometric algorithms
- [[cross-product]]: Fundamental operation for orientation testing
- [[point-in-polygon]]: Uses ray casting (line intersection variant)
- [[convex-hull]]: Another geometric primitive often paired with line intersection

### AoC Patterns
- [[aoc-grid-traversal]]: Walking grids where intersections matter
- [[aoc-pathfinding]]: Line-of-sight and obstacle detection
- [[aoc-parsing-patterns]]: Parsing coordinate/line segment inputs

## Rust-Specific Considerations

### Type Safety
```rust
// Use newtype pattern for type safety
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point(f64, f64);

#[derive(Debug, Clone, Copy)]
struct Vector(f64, f64);

impl std::ops::Sub for Point {
    type Output = Vector;
    fn sub(self, other: Point) -> Vector {
        Vector(self.0 - other.0, self.1 - other.1)
    }
}
```

### Trait Implementation
```rust
trait Intersects {
    fn intersects(&self, other: &Self) -> bool;
    fn intersection_point(&self, other: &Self) -> Option<Point>;
}

impl Intersects for Segment {
    fn intersects(&self, other: &Self) -> bool {
        segments_intersect(*self, *other)
    }
    
    fn intersection_point(&self, other: &Self) -> Option<Point> {
        line_intersection(*self, *other)
    }
}
```

## Learning Resources

- **Book References**: Computational Geometry: Algorithms and Applications (de Berg et al.)
- **AoC Problems**: 2019 Day 3 (Crossed Wires), 2018 Day 13 (Mine Cart Madness)
- **Rust Crates**: `geo` crate provides robust geometric primitives
- **Visualization**: Draw examples to understand orientation-based approach

## Implementation Checklist

- [ ] Define `Point` and `Segment` types
- [ ] Implement `orientation()` function with epsilon handling
- [ ] Implement `on_segment()` for collinear case
- [ ] Implement `segments_intersect()` with all edge cases
- [ ] Implement `line_intersection()` for infinite lines (optional)
- [ ] Add comprehensive tests for edge cases
- [ ] Consider integer vs. floating-point coordinates
- [ ] Profile performance for bulk intersection problems

---

**Zettelkasten Links:**
- Incoming: [[computational-geometry]]
- Outgoing: [[mission-6]], [[mission-8]], [[computational-geometry]], [[cross-product]], [[point-in-polygon]], [[convex-hull]], [[aoc-grid-traversal]], [[aoc-pathfinding]], [[aoc-parsing-patterns]]

*Last Updated: 2024-12-19*
