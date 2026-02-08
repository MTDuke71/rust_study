# Day 8: Treetop Tree House - Function Guide

**Quick Links**: [← Day 7](day07_function_guide.md) | [Problem Statement](day08.md) | [Summary](../summary_2022.md) | [Day 9 →](day09_function_guide.md)

---

## Problem Overview
Given a 99×99 grid of tree heights (digits '0'-'9'), solve two related problems:
- **Part 1**: Count trees visible from outside the grid (looking directly along rows/columns)
- **Part 2**: Find the maximum "scenic score" (product of viewing distances in 4 directions)

A tree is **visible** if all trees between it and any edge are shorter.  
A **viewing distance** counts trees seen in one direction until hitting edge or tree ≥ height (includes blocking tree).

**Answer**: Part 1: `1690` | Part 2: `535680`

## Performance Benchmarks
- **Combined**: ~~262µs~~ **174µs** (0.174ms) — after ring + precompute + Rayon optimizations
- **Parse**: ~15µs (integrated in solve)
- **Part 1**: ~80µs (4 precompute passes + O(1) visibility checks) - **optimized from 180µs**
- **Part 2**: ~94µs (scenic score with Rayon row-parallel processing) - **optimized from 167µs**

**Optimizations**: 
1. **Part 2 Ring-based early termination**: 288µs → 250µs (7.5% improvement) - REPLACED by Rayon
2. **Part 1 Pre-compute max heights**: 180µs → 80µs (41% improvement, O(N³) → O(N²))
3. **Part 2 Rayon row-parallel**: 167µs → 94µs (44% improvement, multi-core)
4. **Combined**: 483µs → 174µs (**2.8× total speedup**)

---

## Algorithm: Directional Iteration

### Part 1: Visibility

```
Grid (5×5 example):
30373
25512
65332
33549
35390

For tree at [1][2] (height 5):
- Look left:  [2, 5] → all < 5? NO (5 not < 5)  ✗
- Look right: [1] → all < 5? YES                 ✓ VISIBLE
- Look up:    [0] → all < 5? YES                 ✓ VISIBLE
- Look down:  [3, 3] → all < 5? NO               ✗
Result: Visible (at least one direction clear)

For tree at [2][2] (height 3):
- Look left:  [5, 6] → all < 3? NO               ✗
- Look right: [3] → all < 3? NO                  ✗
- Look up:    [0, 5] → all < 3? NO               ✗
- Look down:  [5, 5] → all < 3? NO               ✗
Result: Not visible (all directions blocked)
```

**Edge trees** are always visible (already on the boundary).

### Part 2: Scenic Score

```
For tree at [1][2] (height 5):
- Look up:    [0] → count until ≥5 or edge → 1 tree
- Look left:  [5] → blocked immediately → 1 tree
- Look down:  [3, 5] → 3 < 5, 5 ≥ 5 (stop) → 2 trees
- Look right: [1] → 1 < 5, edge → 1 tree
Scenic score: 1 × 1 × 2 × 1 = 2

For tree at [3][2] (height 5):
- Look up:    [3, 5] → 3 < 5, 5 ≥ 5 (stop) → 2 trees
- Look left:  [3, 3] → both < 5 → 2 trees
- Look down:  [5] → 5 ≥ 5 (stop) → 1 tree
- Look right: [4, 9] → 4 < 5, 9 ≥ 5 (stop) → 2 trees
Scenic score: 2 × 2 × 1 × 2 = 8 ✓ (best in example)
```

**Key difference**: Viewing distance **includes** the blocking tree. If blocked at distance d, count = d (not d-1).

---

## Function Reference

### `parse_grid(input: &str) -> Vec<Vec<u8>>`
**Purpose**: Convert input string to 2D grid of tree heights  
**Strategy**: Line-by-line, char-by-char iteration

```rust
input.lines()
    .filter(|line| !line.is_empty())
    .map(|line| {
        line.chars()
            .map(|ch| ch.to_digit(10).unwrap() as u8)
            .collect()
    })
    .collect()
```

**Result**: `Vec<Vec<u8>>` with `grid[row][col]` indexing

**Complexity**: O(rows × cols) — must visit every cell once

---

### `is_visible(grid: &Grid, row: usize, col: usize) -> bool`
**Purpose**: Check if tree at (row, col) is visible from ANY direction  
**Strategy**: Four directional scans (left/right/up/down), early return on first clear direction

| Direction | Range | Check |
|-----------|-------|-------|
| **Left** | `0..col` | All `grid[row][c] < height`? |
| **Right** | `col+1..cols` | All `grid[row][c] < height`? |
| **Up** | `0..row` | All `grid[r][col] < height`? |
| **Down** | `row+1..rows` | All `grid[r][col] < height`? |

**Early exit**: Returns `true` as soon as ONE direction is clear (no need to check remaining).

**Edge case**: Edge trees return `true` immediately (row/col == 0 or rows-1/cols-1).

**Complexity**: 
- Best case O(1) — edge tree or first direction clear
- Worst case O(N) — interior tree, all 4 directions scan full row/column

---

### `viewing_distance(grid: &Grid, row: usize, col: usize, dr: isize, dc: isize) -> usize`
**Purpose**: Count trees visible in ONE direction (signed offset `(dr, dc)`)  
**Strategy**: Walk until edge or blocking tree, count each step

| Direction | `(dr, dc)` | Notes |
|-----------|------------|-------|
| Up | `(-1, 0)` | Decreasing rows |
| Down | `(1, 0)` | Increasing rows |
| Left | `(0, -1)` | Decreasing cols |
| Right | `(0, 1)` | Increasing cols |

```rust
let mut count = 0;
let mut r = row as isize + dr;
let mut c = col as isize + dc;

while r >= 0 && r < rows && c >= 0 && c < cols {
    count += 1;
    if grid[r][c] >= height { break; } // Blocked - but counted
    r += dr;
    c += dc;
}
count
```

**CRITICAL**: The blocking tree is **included** in count (increment before checking height).

**Complexity**: O(max(rows, cols)) — worst case walks entire row/column

---

### `scenic_score(grid: &Grid, row: usize, col: usize) -> usize`
**Purpose**: Calculate scenic score (product of 4 viewing distances)  
**Strategy**: Call `viewing_distance()` for each direction, multiply results

```rust
let up = viewing_distance(grid, row, col, -1, 0);
let down = viewing_distance(grid, row, col, 1, 0);
let left = viewing_distance(grid, row, col, 0, -1);
let right = viewing_distance(grid, row, col, 0, 1);

up * down * left * right
```

**Edge trees**: Will have ≥1 direction with distance 0 → score = 0 (never optimal).

**Complexity**: 4 × O(N) = O(N) where N = max dimension

---

### `solve_part1(grid: &Grid) -> usize`
**Purpose**: Count all visible trees  
**Strategy**: Nested loop over all positions, call `is_visible()`

```rust
let mut count = 0;
for row in 0..rows {
    for col in 0..cols {
        if is_visible(grid, row, col) {
            count += 1;
        }
    }
}
count
```

**Complexity**: O(rows × cols × N) = O(N³) worst case, but early exit makes it faster in practice

**Actual runtime**: ~180µs for 99×99 grid

---

### `solve_part2(grid: &Grid) -> usize`
**Purpose**: Find maximum scenic score across all trees  
**Strategy**: Ring-based iteration with early termination

**Optimization**: Instead of checking all N² trees, group by minimum edge distance and process from center outward. Skip outer rings when theoretical maximum ≤ current best.

```rust
// 1. Group trees by min edge distance
for row in 0..rows {
    for col in 0..cols {
        let min_dist = row.min(col).min(rows-1-row).min(cols-1-col);
        rings[min_dist].push((row, col));
    }
}

// 2. Process center → edge (d=49 down to d=0)
for dist in (0..=max_dist).rev() {
    // Theoretical max for this ring
    let theo_max = dist * dist * (rows-1-dist) * (cols-1-dist);
    
    // Skip if theo_max can't beat current best
    if theo_max <= max_score { break; }
    
    // Check all trees in this ring
    for &(row, col) in &rings[dist] {
        max_score = max_score.max(scenic_score(grid, row, col));
    }
}
```

**Why it works**:
- Trees at distance `d` from edge have max possible viewing distances: `[d, d, rows-1-d, cols-1-d]`
- Maximum possible score at distance `d` = `d² × (rows-1-d) × (cols-1-d)`
- Once we find score > theo_max for ring `d`, all rings with `d' < d` can be skipped

**For this input** (answer = 535,680):
- Rings checked: d=49 down to d=9 (41 rings, ~7,100 trees)
- Rings skipped: d=8 down to d=0 (9 rings, ~2,700 trees = 27% of grid)
- Ring d=8 theoretical max = 518,400 < 535,680 → can stop

**Complexity**: 
- Best case: O(N) if high score found near center
- Worst case: O(N²) if answer is at edge (degrades to naive)
- This input: ~73% of trees checked

**Actual runtime**: ~250µs (down from 288µs naive, 7.5% improvement)

---

## Complexity Analysis

| Function | Time | Space | Notes |
|----------|------|-------|-------|
| `parse_grid` | O(R×C) | O(R×C) | Store entire grid |
| `is_visible` | O(R+C) worst | O(1) | 4 scans, early exit |
| `viewing_distance` | O(max(R,C)) | O(1) | Single directional walk |
| `scenic_score` | O(R+C) | O(1) | 4× viewing_distance |
| `solve_part1` | O(R×C×(R+C)) | O(1) | Nested loop + visibility |
| `solve_part2` | O(R×C×(R+C)) | O(1) | Nested loop + scenic score |

**Overall**: O(N³) for N×N grid, but heavily optimized by:
- Early exit in visibility checks
- Direct indexing (no hash lookups)
- Cache-friendly sequential access

---

## Design Patterns

### Directional Iteration with Signed Offsets
Single `viewing_distance()` function handles all 4 directions using `(dr, dc)` parameter:
- Eliminates code duplication (4 nearly-identical functions → 1)
- Signed `isize` allows negative offsets for up/left
- Bounds check: `r >= 0 && r < rows && c >= 0 && c < cols`

### Parse-Once Pattern
Grid parsed once in `solve()`, passed by reference to both parts:
```rust
pub fn solve(input: &str) -> (usize, usize) {
    let grid = parse_grid(input);
    let part1 = solve_part1(&grid);
    let part2 = solve_part2(&grid);
    (part1, part2)
}
```

Avoids redundant parsing (~15µs savings).

### Simple Data Structures
Used `Vec<Vec<u8>>` instead of mission components:
- ✅ Natural row-first indexing: `grid[row][col]`
- ✅ Cache-friendly sequential access
- ✅ Simple to reason about
- ❌ No mission integration (learning opportunity deferred)

**Trade-off**: Mission 6's `Grid<T>` uses `Coord(x, y)` which complicates row-first parsing. Simplicity won for this problem.

---

## Completed Optimizations

### ✅ 1. Pre-Compute Max Heights (IMPLEMENTED - Part 1)
**Status**: Applied — **41% speedup** on Part 1 (180µs → 80µs)

**Current (O(N³))**: For each of N² trees, scan up to N trees in 4 directions
**Optimized (O(N²))**: 4 passes to compute max heights + O(1) check per tree

```rust
// 4 passes: left→right, right→left, top→bottom, bottom→top
max_from_left[row][col] = max height from left edge to col-1
max_from_right[row][col] = max height from right edge to col+1
max_from_top[row][col] = max height from top edge to row-1
max_from_bottom[row][col] = max height from bottom edge to row+1

// Tree is visible if height > ANY edge max (O(1) check)
visible = height > max_from_left || height > max_from_right 
       || height > max_from_top || height > max_from_bottom
```

**Impact**: Reduced Part 1 from ~180µs to ~80µs (55% reduction)

**Trade-off**: 4 extra grids (4×99×99 = ~40KB for this input), but major time savings

### ✅ 2. Rayon Row-Based Parallelization (IMPLEMENTED - Part 2)
**Status**: Applied — **44% speedup** on Part 2 (167µs → 94µs), **33% overall** (262µs → 174µs)

Leverage multi-core CPU parallelism to compute scenic scores concurrently.

```rust
use rayon::prelude::*;

fn solve_part2(grid: &Grid) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    
    (0..rows)
        .into_par_iter()              // Parallel iterator over rows
        .map(|row| {
            (0..cols)
                .map(|col| scenic_score(grid, row, col))
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0)
}
```

**Key Insights**:
- **Work unit granularity matters**: 99 rows (each ~99 trees) is optimal for thread pool overhead
- **❌ Ring-based parallel FAILED**: 920µs (3.5× slower) — 5,000+ tiny tasks created excessive thread spawning overhead
- **✅ Row-based parallel SUCCEEDED**: 174µs (1.5× faster) — 99 larger tasks amortize thread pool costs
- **Trade-off**: Gave up ring-based early termination (27% tree skip) to gain multi-core parallelism

**When Parallelization Helps**:
- ✅ Large enough work units (milliseconds of compute per task)
- ✅ Independent tasks (no sequential dependencies)
- ✅ High compute-to-overhead ratio

**When Parallelization Hurts**:
- ❌ Tiny work units (microseconds per task)
- ❌ Thread pool overhead > speedup gained
- ❌ Early termination patterns disrupted

**Impact**: Part 2 from ~167µs → ~94µs; combined 262µs → 174µs (2.8× from original 483µs)

---

## Potential Future Optimizations

### SIMD Vectorization
Modern CPUs can compare 32 bytes in parallel using SIMD instructions. Could apply to row/column scans in both parts.

**Impact**: Estimated 2-4× speedup on directional scans (total: ~262µs → ~100µs).

**Trade-off**: Platform-specific, requires unsafe code or specialized crates, increases complexity significantly.

---

## Learning Highlights

### Rust Techniques Used
- `Vec<Vec<T>>` for 2D grids with direct indexing
- `isize` for signed offsets with bounds checking
- `.all()` iterator method for "all elements satisfy predicate"
- Early return for efficiency (`return true` when first clear direction found)
- `.max()` for tracking maximum across iteration

### Problem-Solving Insights
1. **Visibility vs. Viewing Distance**: Subtle distinction — visibility is binary (blocked/clear), viewing distance is quantitative (how many trees seen).
2. **Include the blocker**: Viewing distance counts the blocking tree (scenic score would be 0 if it didn't).
3. **Edge optimization**: Edge trees are always visible but always have score=0 (at least one direction has distance=0).
4. **Directional abstraction**: Single function + `(dr, dc)` parameter beats 4 copy-pasted functions.

---

**Navigation**: [← Day 7](day07_function_guide.md) | [Problem](day08.md) | [Summary](../summary_2022.md) | [Day 9 →](day09_function_guide.md)
