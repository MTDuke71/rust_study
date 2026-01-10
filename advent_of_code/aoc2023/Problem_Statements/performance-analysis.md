# AoC 2023 - Performance Analysis

Benchmarks, optimization insights, and performance learnings from AoC 2023.

---

## 📊 Overall Statistics

| Metric | Value |
|--------|-------|
| **Days Completed** | 9/25 |
| **Total Runtime** | ~11.5ms |
| **Average per Day** | ~1.3ms |
| **Fastest Day** | Day 6 (0.95µs) |
| **Slowest Day** | Day 8 (8.2ms) |

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
| 8 | 1.5ms | 6.7ms | 8.2ms | Yes**** |
| 9 | 132.0µs | 191.0µs | 323.0µs | No* |

*Day 2: Initial implementation, room for optimization (parsing can be improved)  
**Day 3: Part 2 faster than Part 1! Spatial indexing beats brute force adjacency checks  
***Day 6: Part 2 faster than Part 1! Quadratic formula O(1) beats brute force O(T)**  
****Day 8: Part 2 uses LCM optimization - brute force would be intractable (8+ trillion steps)**  
*Day 9: Initial clean implementation, already fast (~323µs total), recursion depth typically low  
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

### Day 8: LCM for Cycle Alignment
**Before**: Intractable (would require simulating 8,811,050,362,409 steps)  
**After**: 6.7ms (Part 2)  
**Speedup**: Infinite (brute force impossible to complete)  
**Complexity**: O(steps) → O(k × m + k log k) where k = ghosts, m = avg cycle length  
**Technique**: Cycle detection + Least Common Multiple  
**Learning**: For simultaneous periodic processes, find cycle lengths and compute LCM instead of simulating  
**Code**:
```rust
// ❌ Brute force: Impossible (8+ trillion steps)
loop {
    // Move all ghosts simultaneously
    for ghost in &mut ghosts {
        ghost.step();
    }
    steps += 1;
    if ghosts.iter().all(|g| g.at_goal()) {
        return steps;  // Would never reach in reasonable time
    }
}

// ✅ Cycle detection + LCM: ~6.7ms
let cycle_lengths: Vec<usize> = ghosts
    .iter()
    .map(|g| g.find_cycle_length())
    .collect();

let result = cycle_lengths.iter()
    .fold(1, |acc, &x| lcm(acc, x));
```

**Mathematical Foundation**:
- Each ghost follows repeating pattern with specific period
- All cycles align at LCM of individual periods
- Example: cycles of 4 and 6 align at lcm(4,6) = 12

**Algorithm Details**:
- Find cycle length for each ghost: O(k × m) where k = ghosts, m = avg cycle
- Compute multi-way LCM: O(k log k) using fold + GCD
- GCD uses Euclidean algorithm: O(log min(a,b))

**Performance Impact**:
- Part 1: 1.5ms (19,637 steps - direct simulation)
- Part 2: 6.7ms (8.8 trillion steps - via LCM)
- **Key insight**: Problem designed to be unsolvable by brute force

**Zettelkasten**: [[number-theory-basics]], [[graph-theory-fundamentals]]

### Day 9: Recursive Finite Differences - Naturally Efficient
**Complexity**: O(n²) per sequence, but practical performance excellent  
**Runtime**: Part 1: 132µs, Part 2: 191µs, Total: 323µs  
**Technique**: Recursive difference pyramid with early termination  
**Status**: No optimization needed - already fast  

**Why It's Fast**:
```rust
// Polynomial sequences reach all-zeros quickly
// Most sequences in Day 9 are low-degree (1-3 levels deep)

// Example: Linear sequence (degree 1) - only 2 levels
0   3   6   9  12  15    ← Level 0
  3   3   3   3   3      ← Level 1
    0   0   0   0        ← All zeros, return

// Recursion depth typically 2-4 levels, not sequence length
```

**Complexity Analysis**:
- **Theoretical**: O(d × n) where d = polynomial degree, n = sequence length
- **Worst case**: O(n²) if d = n (rare in practice)
- **Typical case**: O(2n) to O(4n) since most sequences are low-degree polynomials
- **For Day 9**: 200 sequences, ~21 values each, depth ~2-3 levels

**Performance Breakdown** (from benchmarks):
- Parsing: Minimal (split + parse integers)
- Difference computation: `.windows(2).map()` is efficient
- Recursion overhead: Low due to small depth
- Memory: Allocates Vec per level, but levels are small

**Code Characteristics**:
```rust
// ✅ Efficient patterns used:
sequence.windows(2)           // Iterator, no allocation
    .map(|pair| pair[1] - pair[0])  // Simple arithmetic
    .collect()                // Single allocation per level

sequence.iter().all(|&x| x == 0)  // Early termination
sequence.last().unwrap()      // O(1) slice operation
```

**Why No Optimization Needed**:
1. **Natural efficiency**: Recursion depth bounded by polynomial degree (typically 2-4)
2. **Simple operations**: Integer arithmetic, no complex data structures
3. **Iterator usage**: Modern, zero-cost abstractions
4. **Early termination**: Base case stops as soon as all zeros reached
5. **Sub-millisecond already**: 323µs for 200 sequences is excellent

**Potential Optimizations (not worth it)**:
- ❌ **Iterative instead of recursive**: Would complicate code for minimal gain
- ❌ **Memoization**: No overlapping subproblems (each level computed once)
- ❌ **Pre-allocated buffers**: Memory allocation not bottleneck
- ❌ **Parallelization**: Each sequence independent but overhead > benefit

**Learning**: Sometimes the straightforward recursive solution is already optimal. Don't over-optimize clean code that runs in microseconds.

**Zettelkasten**: [[finite-differences]]

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
