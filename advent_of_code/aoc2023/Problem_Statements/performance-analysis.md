# AoC 2023 - Performance Analysis

Benchmarks, optimization insights, and performance learnings from AoC 2023.

---

## 📊 Overall Statistics

| Metric | Value |
|--------|-------|
| **Days Completed** | 7/25 |
| **Total Runtime** | 3.030ms |
| **Average per Day** | 432.9µs |
| **Fastest Day** | Day 6 (0.95µs) |
| **Slowest Day** | Day 3 (926.1µs) |

---

## 📈 Runtime by Day

| Day | Part 1 | Part 2 | Total | Optimized? |
|-----|--------|--------|-------|------------|
| 1 | 27.3µs | 37.6µs | 64.9µs | Yes |
| 2 | 52.5µs | 73.3µs | 125.8µs | No* |
| 3 | 622.1µs | 304.0µs | 926.1µs | Yes** |
| 4 | 176.6µs | 186.3µs | 362.9µs | Yes |
| 5 | 34.6µs | 783.8µs | 818.4µs | Yes |
| 6 | 0.65µs | 0.30µs | 0.95µs | Yes*** |
| 7 | 290.0µs | 435.3µs | 725.3µs | Yes |

*Day 2: Initial implementation, room for optimization (parsing can be improved)  
**Day 3: Part 2 faster than Part 1! Spatial indexing beats brute force adjacency checks  
***Day 6: Part 2 faster than Part 1! Quadratic formula O(1) beats brute force O(T)**  
| ... | - | - | - | - |

---

## 🚀 Optimization Wins

*Documented when significant optimizations are made (>2x speedup).*

### Day 3: Spatial Indexing for Gear Detection
**Before**: 88.92ms (old implementation)  
**After**: 360.25µs (optimized implementation)  
**Speedup**: **246.8x faster** (empirically measured)  
**Complexity**: O(gears × numbers × adjacency_calc) → O(numbers × digits + gears × 8)  
**Operations**: ~millions → ~160K  
**Technique**: Reverse the search direction with HashMap spatial index  
**Learning**: For grid problems with lookups, build a coordinate→entity index instead of iterating all entities  
**Benchmark**: Created `bench_day03` package comparing implementations across 100 iterations  
**Code**: 
```rust
// Before: For each gear, iterate ALL numbers (expensive!)
// Time: 88.92ms for 100 gears × ~1000 numbers
for gear_coord in gears {
    let adjacent = numbers.iter()
        .filter(|num| num.adjacent_coords().contains(&gear_coord))  // Recalculates Vec every time!
        .collect();
}

// After: Build index once, O(1) lookup per gear neighbor
// Time: 360.25µs with spatial index
let coord_to_number: HashMap<Coord, usize> = /* build index */;
for gear_coord in gears {
    for neighbor in gear_coord.neighbors_8() {
        if let Some(&num_id) = coord_to_number.get(&neighbor) {
            // O(1) lookup instead of O(n) iteration
        }
    }
}
```

**Additional Optimizations**:
- Changed `Vec` to `HashSet` for adjacency deduplication: O(n) → O(1) contains check
- Added `digit_coords()` helper for clarity and reuse
- Measured with actual puzzle input (140×140 grid, ~100 gears, ~1000 numbers)

**Validation**: Both implementations produce identical results (87263515)

### Template for Future Optimizations

```markdown
### Day XX: [Optimization Name]
**Before**: XX.Xms  
**After**: X.Xms  
**Speedup**: X.Xx  
**Technique**: [Description]  
**Learning**: [Key takeaway]  
**Code**: 
```rust
// Before
...
// After
...
```
```

---

## 🔧 Optimization Techniques Reference

### Compiler-Level Optimizations
- **Power-of-2 modulo**: `x % 16777216` → `x & 0xFFFFFF` (compiler does this automatically)
- **Release mode**: Always benchmark with `--release`
- **LTO (Link-Time Optimization)**: Enable in Cargo.toml for final benchmarks

### Algorithmic Optimizations
- **Early termination**: Break loops when answer found
- **Memoization**: Cache repeated computations
- **Better data structures**: HashMap vs Vec for lookups

### Rust-Specific Optimizations
- **Iterator chains**: Avoid intermediate allocations
- **`collect()` into `Vec` with capacity**: Pre-allocate when size known
- **`Entry` API**: Avoid double HashMap lookups
- **`&str` over `String`**: Avoid allocations in hot paths

### Parallelization (Rayon)
- **When to use**: Independent iterations, >10ms runtime
- **Pattern**: `.iter()` → `.par_iter()`
- **Typical speedup**: 4-8x on multi-core

---

## 📉 Performance Anti-Patterns

### Avoid These
1. **Cloning in loops**: Use references where possible
2. **String concatenation**: Use `format!` or `push_str`
3. **Nested loops with O(n²)**: Consider HashSet/HashMap
4. **Regex in hot paths**: Pre-compile or use string methods
5. **Box<dyn Trait> in hot paths**: Use enums or generics

---

## 🎯 Performance Targets

| Category | Target | Status |
|----------|--------|--------|
| Total (all 25 days) | <2 seconds | On track |
| Average per day | <50ms | ✅ |
| No day exceeds | 200ms | ✅ |

---

## 🔬 Profiling Tools

```bash
# Criterion benchmarks
cargo criterion --bench dayXX

# Flamegraph (requires cargo-flamegraph)
cargo flamegraph --bin aoc2023 -- XX

# Show assembly for hot functions
cargo asm aoc2023::solver::dayXX::hot_function
```

---

## 📝 Notes

- All benchmarks run on: [Your machine specs]
- Benchmarks use `criterion` with default settings
- Times are median of multiple runs
- "Optimized?" column tracks if Phase 2 optimization was done
