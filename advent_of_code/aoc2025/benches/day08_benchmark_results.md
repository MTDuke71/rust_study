# Day 8 Distance Calculation Benchmark Results

## Question from Reddit
> Is there any benefit to not do the sqrt step, and keeping the numbers as integers, with respect to overall time?

## Answer: YES! Significant performance improvement

## Benchmark Setup
- **Input**: 1000 junction boxes (499,500 pairs to evaluate)
- **Comparison**: 
  - `f64` with `sqrt()` (original implementation)
  - `i64` squared distance (optimized - no sqrt)
- **Tool**: Criterion.rs benchmark framework
- **Build**: `--release` with optimizations

## Results

| Version | Part 1 Time | Part 2 Time | Speedup |
|---------|-------------|-------------|---------|
| **f64 with sqrt()** | 24.31 ms | 24.32 ms | baseline |
| **i64 squared (no sqrt)** | 21.64 ms | 21.58 ms | **~12% faster** |

### Detailed Timings

```
day08_distance_comparison/part1_with_sqrt_f64
    time:   [24.245 ms 24.313 ms 24.383 ms]

day08_distance_comparison/part1_squared_i64
    time:   [21.574 ms 21.644 ms 21.717 ms]
    
day08_distance_comparison/part2_with_sqrt_f64
    time:   [24.199 ms 24.316 ms 24.455 ms]
    
day08_distance_comparison/part2_squared_i64
    time:   [21.514 ms 21.577 ms 21.643 ms]
```

## Analysis

### Why the optimization works

1. **Monotonicity preserved**: For positive numbers, if `a² < b²` then `a < b`
   - Sorting by squared distance gives **identical ordering** to sorting by actual distance
   - We only care about **relative ordering**, not absolute distances

2. **Performance benefits**:
   - **No sqrt()**: Eliminates expensive floating-point square root (10-20x slower than multiply)
   - **Integer arithmetic**: `i64` multiplication vs `f64` operations
   - **Better cache**: `i64` is same size as `f64` but integers often have better CPU pipeline utilization
   - **SIMD friendly**: Integer operations vectorize more easily on modern CPUs

3. **Measured improvement**: **~12% speedup** (2.7ms saved per part)
   - 499,500 pairs × saved sqrt() = significant aggregate savings
   - Part 1: 24.31ms → 21.64ms (2.67ms saved)
   - Part 2: 24.32ms → 21.58ms (2.74ms saved)

### What's being optimized

```rust
// Original (f64 with sqrt)
fn calculate_distance(a: &JunctionBox, b: &JunctionBox) -> f64 {
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    let dz = (a.z - b.z) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()  // 👈 expensive sqrt!
}

// Optimized (i64 squared)
fn calculate_distance_squared(a: &JunctionBox, b: &JunctionBox) -> i64 {
    let dx = (a.x - b.x) as i64;
    let dy = (a.y - b.y) as i64;
    let dz = (a.z - b.z) as i64;
    dx * dx + dy * dy + dz * dz  // 👈 no sqrt! just integer multiply/add
}
```

### Code changes required

1. Change return type: `f64` → `i64`
2. Remove `.sqrt()` call
3. Update sort: `sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap())` → `sort_by_key(|&(_, _, dist)| dist)`
   - Bonus: `sort_by_key` is cleaner and faster than `sort_by` with `partial_cmp`!

## Correctness Verification

✅ **Both versions produce identical results:**
- Part 1: `50568`
- Part 2: `36045012`

Test added to verify equivalence:
```rust
#[test]
fn test_optimized_versions_match() {
    assert_eq!(solve_part1(input), solve_part1_optimized(input));
    assert_eq!(solve_part2(input), solve_part2_optimized(input));
}
```

## Conclusion

**Reddit answer**: Yes, absolutely! Skipping `sqrt()` and using `i64` squared distances provides:
- ✅ **12% performance improvement** (2.7ms saved per part)
- ✅ **Identical correctness** (same answers)
- ✅ **Cleaner sorting** (can use `sort_by_key` instead of `sort_by`)
- ✅ **Better numerical properties** (no floating-point precision concerns)

**When to use this optimization:**
- ✅ When you only need **relative ordering** of distances
- ✅ For sorting, finding min/max, or comparing distances
- ❌ When you need **actual distance values** (e.g., sum of distances, distance threshold checks)

**AoC takeaway**: This is a common competitive programming optimization. Always ask: "Do I need the exact value, or just the ordering?"

---

## Related Documentation

- **[[Problem_Statements/summary]]** - Day 8 solution context (nearest junction boxes)
- **[[Problem_Statements/day08]]** - Original problem statement
- **[[Mission10]]** - Union-Find data structure used in Day 8 solution
- **[[AoC Patterns MOC]]** - Distance optimization patterns

*Tags: #optimization #benchmarking #distance-calculation #integer-math #criterion*

---

*Benchmarked on: Windows, Rust 1.83, Criterion 0.5.1*
*Date: December 8, 2025*
