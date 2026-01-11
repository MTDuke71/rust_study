# Day 11: Cosmic Expansion - Function-by-Function Guide

## Overview

**Problem**: Calculate the sum of shortest paths between all pairs of galaxies in a universe that has expanded.

**Key Insights**:
1. **Don't physically expand the grid** - track empty rows/columns and adjust distances
2. **Expansion factor abstraction** - Part 1 (2x) and Part 2 (1,000,000x) use identical logic
3. **Manhattan distance on non-uniform grid** - empty spaces have different "weight"
4. **All pairs counting** - For g galaxies, need to compute C(g,2) = g×(g-1)/2 distances

**Mathematical Foundations**:
- **Combinatorics**: All pairs generation → C(g, 2) combinations
- **Graph Theory**: Complete graph with g vertices
- **Metric Spaces**: Manhattan distance with variable edge weights
- **Set Theory**: Complement operation for empty row/column detection

**Zettelkasten Links**:
- [[manhattan-distance]]
- [[graph-theory-fundamentals]]
- [[combinatorics-basics]]
- [[set-theory-fundamentals]]

---

## Type Definitions

### `Position`
```rust
type Position = (usize, usize);
```

**Purpose**: Represents a 2D coordinate `(row, col)` in the grid.

**Design Decision**: Simple tuple type instead of custom struct
- ✅ **Pros**: Zero overhead, pattern matching works naturally, Copy trait automatic
- ❌ **Cons**: No semantic meaning (could confuse row vs col), no methods
- **Choice**: For this problem, simplicity wins - coordinates are used in simple calculations only

**Usage**:
```rust
let galaxy1: Position = (5, 3);  // Row 5, Column 3
let (r, c) = galaxy1;            // Easy destructuring
```

---

## Core Implementation

### `parse_galaxies`

```rust
fn parse_galaxies(input: &str) -> Vec<Position> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(col, _)| (row, col))
        })
        .collect()
}
```

**Purpose**: Extract all galaxy positions (`'#'` characters) from input grid.

**Algorithm Walkthrough**:
1. **`lines().enumerate()`**: Iterate with row indices
   - Input: `"...#...\n.......#\n#......."`
   - Output: `[(0, "...#..."), (1, ".......#"), (2, "#.......")]`

2. **`flat_map`**: For each row, generate multiple galaxy positions
   - Processes one row at a time
   - Returns iterator of `(row, col)` tuples
   - `flat_map` flattens nested iterators into single stream

3. **Inner pipeline per row**:
   ```rust
   line.chars().enumerate()  // [(0, '.'), (1, '.'), (2, '.'), (3, '#'), ...]
       .filter(|(_, ch)| *ch == '#')  // [(3, '#')]
       .map(move |(col, _)| (row, col))  // [(0, 3)]  // row from outer scope
   ```

4. **`move` closure**: Captures `row` by value
   - Necessary because `flat_map` returns iterator that outlives loop iteration
   - Each inner iterator needs its own copy of `row`

**Example**:
```rust
Input grid:
...#......  (row 0)
.......#..  (row 1)
#.........  (row 2)

Step-by-step for row 0:
chars().enumerate() → [(0,'.'), (1,'.'), (2,'.'), (3,'#'), ...]
filter(#) → [(3, '#')]
map → [(0, 3)]  // Galaxy at (row=0, col=3)

Final result: vec![(0, 3), (1, 7), (2, 0), ...]
```

**Complexity**: O(n × m) where n = rows, m = cols per row

**Why `flat_map` over nested loops**:
```rust
// ❌ Imperative style - mutable Vec, explicit nesting
let mut galaxies = Vec::new();
for (row, line) in input.lines().enumerate() {
    for (col, ch) in line.chars().enumerate() {
        if ch == '#' {
            galaxies.push((row, col));
        }
    }
}

// ✅ Functional style - no mutation, clear data flow
let galaxies: Vec<_> = input.lines().enumerate()
    .flat_map(|(row, line)| {
        line.chars().enumerate()
            .filter(|(_, ch)| *ch == '#')
            .map(move |(col, _)| (row, col))
    })
    .collect();
```

**Testing**:
```rust
#[test]
fn test_parse_galaxies() {
    let input = "...#......\n.......#..";
    let galaxies = parse_galaxies(input);
    assert_eq!(galaxies.len(), 2);
    assert!(galaxies.contains(&(0, 3)));
    assert!(galaxies.contains(&(1, 7)));
}
```

---

### `find_empty_rows`

```rust
fn find_empty_rows(galaxies: &[Position], max_row: usize) -> Vec<usize> {
    let occupied_rows: std::collections::HashSet<_> = 
        galaxies.iter().map(|(r, _)| *r).collect();
    (0..=max_row).filter(|r| !occupied_rows.contains(r)).collect()
}
```

**Purpose**: Identify which rows contain no galaxies.

**Algorithm**: Set complement operation
1. Build HashSet of occupied rows: O(g) where g = galaxy count
2. Check all rows against set: O(n) where n = max_row
3. Filter out occupied rows: O(n) iteration

**Mathematical Foundation**: 
- **Set Theory**: `empty_rows = all_rows \ occupied_rows` (set difference/complement)
- **HashSet**: O(1) membership testing vs O(g) linear search

**Example**:
```rust
Galaxies at: [(0, 3), (1, 7), (2, 0), (4, 6), (5, 1)]
             Row 0  Row 1  Row 2  Row 4  Row 5
max_row = 9 (grid is 10 rows tall, 0-indexed)

occupied_rows HashSet: {0, 1, 2, 4, 5}

Check all rows 0..=9:
0: in HashSet → occupied
1: in HashSet → occupied
2: in HashSet → occupied
3: NOT in HashSet → EMPTY ✓
4: in HashSet → occupied
5: in HashSet → occupied
6: NOT in HashSet → EMPTY ✓
7: NOT in HashSet → EMPTY ✓
8: NOT in HashSet → EMPTY ✓
9: NOT in HashSet → EMPTY ✓

Result: vec![3, 6, 7, 8, 9]
```

**Complexity**: 
- Build HashSet: O(g) iterations, O(g) space
- Filter range: O(n) iterations with O(1) lookups
- Total: O(g + n) time, O(g) space

**Alternative Approaches**:
```rust
// ❌ Without HashSet - O(n × g) quadratic!
(0..=max_row).filter(|r| {
    !galaxies.iter().any(|(gr, _)| gr == r)  // O(g) linear search per row!
}).collect()

// ✅ With HashSet - O(n + g) linear
let occupied: HashSet<_> = galaxies.iter().map(|(r, _)| *r).collect();
(0..=max_row).filter(|r| !occupied.contains(r)).collect()
```

---

### `find_empty_cols`

```rust
fn find_empty_cols(galaxies: &[Position], max_col: usize) -> Vec<usize> {
    let occupied_cols: std::collections::HashSet<_> = 
        galaxies.iter().map(|(_, c)| *c).collect();
    (0..=max_col).filter(|c| !occupied_cols.contains(c)).collect()
}
```

**Purpose**: Identify which columns contain no galaxies.

**Algorithm**: Identical to `find_empty_rows` but extracts column index instead of row.

**Code Difference**:
```rust
// Row version: Extract first element of tuple
galaxies.iter().map(|(r, _)| *r)
                      ^^^^^

// Col version: Extract second element of tuple
galaxies.iter().map(|(_, c)| *c)
                      ^^^^^
```

**Why Separate Functions**: 
- Could be combined into `find_empty_indices(galaxies, max, axis)` with enum
- Trade-off: Simple duplication (7 lines) vs abstraction complexity
- **Choice**: YAGNI (You Aren't Gonna Need It) - duplication is clear and fast

---

### `calculate_distance_with_expansion`

```rust
fn calculate_distance_with_expansion(
    pos1: Position,
    pos2: Position,
    empty_rows: &[usize],
    empty_cols: &[usize],
    expansion_factor: usize,
) -> usize {
    let (r1, c1) = pos1;
    let (r2, c2) = pos2;
    
    let min_row = r1.min(r2);
    let max_row = r1.max(r2);
    let min_col = c1.min(c2);
    let max_col = c1.max(c2);
    
    // Count empty rows between the two galaxies
    let empty_rows_between = empty_rows.iter()
        .filter(|&&r| r > min_row && r < max_row)
        .count();
    
    // Count empty columns between the two galaxies
    let empty_cols_between = empty_cols.iter()
        .filter(|&&c| c > min_col && c < max_col)
        .count();
    
    // Base Manhattan distance
    let base_distance = (max_row - min_row) + (max_col - min_col);
    
    // Each empty row/col expands by (expansion_factor - 1) additional units
    let expansion_offset = empty_rows_between * (expansion_factor - 1)
                         + empty_cols_between * (expansion_factor - 1);
    
    base_distance + expansion_offset
}
```

**Purpose**: Calculate shortest path distance between two galaxies, accounting for cosmic expansion.

**Mathematical Foundation**:

**Manhattan Distance** on uniform grid:
```
d(p1, p2) = |x2 - x1| + |y2 - y1|
```

**Modified Manhattan Distance** with expansion:
```
d(p1, p2) = base_manhattan + (empty_count × (k - 1))

where:
  base_manhattan = |x2 - x1| + |y2 - y1|
  empty_count = # of empty rows/cols crossed
  k = expansion_factor (2 for Part 1, 1,000,000 for Part 2)
```

**Algorithm Walkthrough**:

1. **Normalize coordinates**: Get min/max for both dimensions
   ```rust
   Galaxy 1: (5, 3)
   Galaxy 2: (9, 1)
   
   min_row = min(5, 9) = 5
   max_row = max(5, 9) = 9
   min_col = min(3, 1) = 1
   max_col = max(3, 1) = 3
   ```

2. **Count empty spaces crossed**:
   ```rust
   empty_rows: [3, 7]  // Known from earlier analysis
   
   Rows between 5 and 9: [6, 7, 8]
   Which are empty? r > 5 && r < 9 && r in empty_rows
     6: NOT in [3, 7]
     7: IN [3, 7] ✓
     8: NOT in [3, 7]
   Count: 1
   
   Similarly for columns...
   ```

3. **Calculate base distance**:
   ```rust
   Horizontal: |9 - 5| = 4
   Vertical:   |3 - 1| = 2
   Base:       4 + 2 = 6
   ```

4. **Apply expansion**:
   ```rust
   Part 1 (expansion_factor = 2):
     offset = 1 empty row × (2-1) + 1 empty col × (2-1)
            = 1 × 1 + 1 × 1
            = 2
     total = 6 + 2 = 8
   
   Part 2 (expansion_factor = 1,000,000):
     offset = 1 × 999,999 + 1 × 999,999
            = 1,999,998
     total = 6 + 1,999,998 = 2,000,004
   ```

**Why `expansion_factor - 1`**:
- Original grid: Empty row has width 1
- After 2x expansion: Width becomes 2 (adds 1 extra)
- After 1,000,000x expansion: Width becomes 1,000,000 (adds 999,999 extra)
- General: k× expansion adds (k-1) extra distance per empty space

**Visual Example**:
```
Original grid (expansion_factor = 1):
Col: 0123456
Row:
0    ...#...
1    .......
2    .......  ← Empty row
3    .#.....

Galaxy at (0,3) to (3,1):
Path: (0,3) → (1,3) → (2,3) → (3,3) → (3,2) → (3,1)
Distance: 3 down + 2 left = 5
Empty rows crossed: 1 (row 2)
Extra distance: 1 × (1-1) = 0
Total: 5 + 0 = 5 ✓

After 2x expansion (expansion_factor = 2):
Col: 01234567
Row:
0    ...#....
1    ........
2    ........  ← Empty row (now 2 units tall)
3    ........  ← (extra row from expansion)
4    .#......

Same galaxies now at: (0,3) and (4,1)
Direct calculation: |4-0| + |3-1| = 4 + 2 = 6
Using formula:
  base = |3-0| + |3-1| = 3 + 2 = 5  // Original coordinates
  empty_count = 1  // Row 2 is between row 0 and row 3
  offset = 1 × (2-1) = 1
  total = 5 + 1 = 6 ✓
```

**Complexity**: 
- min/max: O(1)
- Filtering empty rows: O(e_r) where e_r = # empty rows
- Filtering empty cols: O(e_c) where e_c = # empty cols
- Total: O(e_r + e_c) ≈ O(n + m) worst case

**Common Pitfalls**:
```rust
// ❌ Wrong: Using expansion_factor directly
let offset = empty_count * expansion_factor;  // Too much!

// ✅ Correct: Using (expansion_factor - 1)
let offset = empty_count * (expansion_factor - 1);

// Example why it's wrong:
// 2x expansion with 1 empty row should add 1 extra distance
// Wrong: 1 × 2 = 2 (too much!)
// Right: 1 × (2-1) = 1 ✓
```

---

### `solve_with_expansion`

```rust
fn solve_with_expansion(input: &str, expansion_factor: usize) -> Result<String> {
    let galaxies = parse_galaxies(input);
    
    let max_row = input.lines().count() - 1;
    let max_col = input.lines().next().unwrap_or("").len() - 1;
    
    let empty_rows = find_empty_rows(&galaxies, max_row);
    let empty_cols = find_empty_cols(&galaxies, max_col);
    
    // Calculate sum of all pairwise distances
    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let dist = calculate_distance_with_expansion(
                galaxies[i],
                galaxies[j],
                &empty_rows,
                &empty_cols,
                expansion_factor,
            );
            total += dist;
        }
    }
    
    Ok(total.to_string())
}
```

**Purpose**: Main solver - orchestrates all steps and computes final answer.

**Algorithm Pipeline**:
1. Parse galaxies from input
2. Determine grid dimensions
3. Find empty rows/columns
4. Calculate all pairwise distances
5. Sum distances

**All Pairs Generation**:
```rust
for i in 0..galaxies.len() {
    for j in (i + 1)..galaxies.len() {
        // Process pair (i, j)
    }
}
```

**Why `j in (i+1)..n` instead of `j in 0..n`**:
```rust
Galaxies: [A, B, C]  (indices 0, 1, 2)

❌ All combinations with duplicates/mirroring:
i=0, j=0: (A,A) - self-pair
i=0, j=1: (A,B) ✓
i=0, j=2: (A,C) ✓
i=1, j=0: (B,A) - duplicate of (A,B)
i=1, j=1: (B,B) - self-pair
i=1, j=2: (B,C) ✓
i=2, j=0: (C,A) - duplicate of (A,C)
i=2, j=1: (C,B) - duplicate of (B,C)
i=2, j=2: (C,C) - self-pair

✅ Unique pairs only:
i=0, j=1: (A,B) ✓
i=0, j=2: (A,C) ✓
i=1, j=2: (B,C) ✓

Total: C(3,2) = 3!/(2!×1!) = 3 pairs
```

**Combinatorics**:
- Formula: C(g, 2) = g × (g-1) / 2
- For 9 galaxies: 9 × 8 / 2 = 36 pairs
- For 450 galaxies (actual input): 450 × 449 / 2 = 101,025 pairs

**Complexity Analysis**:
```
Parse:          O(n × m)
Find empty:     O(g + n + m)  // g for HashSet, n+m for filtering
All pairs:      O(g²)         // C(g,2) = g²/2
Distance calc:  O(e_r + e_c) per pair ≈ O(n + m)
Total:          O(n×m + g² × (n+m))

Typical values:
- Grid: 140×140 = 19,600 cells
- Galaxies: ~450
- Empty rows/cols: ~100 total
- Pairs: 101,025
- Distance calcs: 101,025 × 100 ≈ 10M operations
- Still fast! (~728µs)
```

**Grid Dimensions**:
```rust
let max_row = input.lines().count() - 1;
let max_col = input.lines().next().unwrap_or("").len() - 1;
```

**Why subtract 1**: Zero-indexed coordinates
```
Grid with 10 lines:
  Row 0: ...
  Row 1: ...
  ...
  Row 9: ...

max_row = 10 - 1 = 9 (highest valid index)
```

**Accumulation Pattern**:
```rust
let mut total = 0;
for pair in all_pairs {
    total += distance(pair);
}
```

**Functional Alternative**:
```rust
let total: usize = (0..galaxies.len())
    .flat_map(|i| ((i+1)..galaxies.len()).map(move |j| (i, j)))
    .map(|(i, j)| {
        calculate_distance_with_expansion(
            galaxies[i], galaxies[j], 
            &empty_rows, &empty_cols, 
            expansion_factor
        )
    })
    .sum();
```

**Trade-off**: Imperative loop is clearer for this problem.

---

## Public API

### `solve_part1`

```rust
pub fn solve_part1(input: &str) -> Result<String> {
    solve_with_expansion(input, 2)
}
```

**Purpose**: Solve Part 1 with 2x expansion factor.

**Simple wrapper** - delegates to shared solver with `expansion_factor = 2`.

---

### `solve_part2`

```rust
pub fn solve_part2(input: &str) -> Result<String> {
    solve_with_expansion(input, 1_000_000)
}
```

**Purpose**: Solve Part 2 with 1,000,000x expansion factor.

**Simple wrapper** - delegates to shared solver with `expansion_factor = 1_000_000`.

**Why identical runtime for both parts**: 
- No grid expansion happens - same algorithm complexity
- Only difference: multiplication factor in distance calculation
- 728.3µs for Part 1, 728.2µs for Part 2 (within measurement noise)

---

## Design Patterns

### Pattern: Abstraction via Parameter

```rust
// ❌ Code duplication
fn solve_part1(input: &str) -> Result<String> {
    // ... parse ...
    // ... find empty ...
    // ... calculate with expansion_factor=2 ...
}

fn solve_part2(input: &str) -> Result<String> {
    // ... SAME parse ...
    // ... SAME find empty ...
    // ... calculate with expansion_factor=1_000_000 ...
}

// ✅ Shared solver with parameter
fn solve_with_expansion(input: &str, expansion_factor: usize) -> Result<String> {
    // Single implementation
}

pub fn solve_part1(input: &str) -> Result<String> {
    solve_with_expansion(input, 2)
}

pub fn solve_part2(input: &str) -> Result<String> {
    solve_with_expansion(input, 1_000_000)
}
```

**Benefits**:
- DRY (Don't Repeat Yourself)
- Single source of truth for algorithm
- Easy to test different expansion factors
- Public API remains simple

---

### Pattern: Set Complement for Finding Missing Elements

```rust
// Find elements in range [0, max] NOT in collection

// Step 1: Build HashSet of existing elements
let existing: HashSet<_> = collection.iter().map(extract).collect();

// Step 2: Filter range for non-existent elements
let missing: Vec<_> = (0..=max)
    .filter(|x| !existing.contains(x))
    .collect();
```

**Applications**:
- Finding empty rows/columns (this problem)
- Finding missing numbers in sequence
- Identifying gaps in data

**Why HashSet**: 
- O(1) membership test vs O(n) linear search
- Total: O(n + m) instead of O(n × m) quadratic

---

### Pattern: Nested Loop for All Pairs

```rust
// Generate all unique pairs from collection
for i in 0..items.len() {
    for j in (i + 1)..items.len() {
        process_pair(items[i], items[j]);
    }
}
```

**Properties**:
- No duplicates: Each pair processed exactly once
- No self-pairs: Never `i == j`
- Count: C(n, 2) = n × (n-1) / 2 iterations

**Index Pattern**:
```
n=4 items [A, B, C, D]:

i=0: j ∈ [1,2,3] → (0,1), (0,2), (0,3)
i=1: j ∈ [2,3]   → (1,2), (1,3)
i=2: j ∈ [3]     → (2,3)
i=3: j ∈ []      → (none)

Total: 3 + 2 + 1 = 6 pairs = C(4,2)
```

---

## Performance Analysis

### Time Complexity

| Operation | Complexity | Count | Total |
|-----------|------------|-------|-------|
| Parse galaxies | O(n × m) | 1 | O(n × m) |
| Build HashSets | O(g) | 2 | O(g) |
| Filter empty rows/cols | O(n + m) | 1 | O(n + m) |
| All pairs iteration | O(g²) | 1 | O(g²) |
| Distance per pair | O(e_r + e_c) | g²/2 | O(g² × (e_r + e_c)) |

**Overall**: O(n×m + g² × (n+m))

**Typical Values** (actual input):
- Grid: 140 × 140 = 19,600
- Galaxies: ~450
- Empty rows/cols: ~100
- Operations: 19,600 + 101,025 × 100 ≈ 10.1M
- Runtime: ~728µs → ~72 CPU cycles per operation (modern CPUs)

### Space Complexity

| Structure | Size | Purpose |
|-----------|------|---------|
| Galaxies Vec | O(g) | Store all positions |
| Empty rows/cols Vecs | O(n + m) | Track expansion |
| HashSets (temp) | O(g) | Find empty spaces |

**Total**: O(g + n + m) ≈ O(max(g, n, m))

For typical input: 450 + 140 + 140 = 730 positions × 16 bytes = ~12KB

### Optimization Opportunities

**Current Implementation**: Already well-optimized!
- ✅ No grid expansion (avoid memory bloat)
- ✅ HashSet for O(1) membership
- ✅ Shared solver (no code duplication)
- ✅ Functional iterators (compiler optimizes)

**Potential Micro-optimizations** (not worth it):
1. Pre-allocate Vec capacity: `Vec::with_capacity(expected_size)`
2. Use `&[usize]` slices instead of vectors for empty rows/cols
3. Parallel distance calculations with rayon (g² is embarrassingly parallel)

**Benchmark Evidence**: 728µs is fast enough - no optimization needed.

---

## Testing Strategy

### Example-Driven Tests

```rust
#[test]
fn test_part1_example() {
    let result = solve_part1(EXAMPLE).unwrap();
    assert_eq!(result, "374");
}
```

**Philosophy**: AoC provides example input + expected output → use it!

### Property-Based Tests

```rust
#[test]
fn test_part2_expansion_10x() {
    let result = solve_with_expansion(EXAMPLE, 10).unwrap();
    assert_eq!(result, "1030");
}

#[test]
fn test_part2_expansion_100x() {
    let result = solve_with_expansion(EXAMPLE, 100).unwrap();
    assert_eq!(result, "8410");
}
```

**Insight**: Problem statement gives multiple expansion factors → test them all!

### Unit Tests for Components

```rust
#[test]
fn test_parse_galaxies() {
    let galaxies = parse_galaxies(EXAMPLE);
    assert_eq!(galaxies.len(), 9);
    assert!(galaxies.contains(&(0, 3)));
}

#[test]
fn test_find_empty_rows() {
    let galaxies = parse_galaxies(EXAMPLE);
    let empty_rows = find_empty_rows(&galaxies, 9);
    assert_eq!(empty_rows, vec![3, 7]);
}
```

**Benefits**: 
- Catch bugs in individual functions
- Easier debugging when tests fail
- Documentation of expected behavior

---

## Common Pitfalls

### ❌ Pitfall 1: Using `expansion_factor` instead of `expansion_factor - 1`

```rust
// Wrong:
let offset = empty_count * expansion_factor;

// Right:
let offset = empty_count * (expansion_factor - 1);
```

**Why**: Empty space already has width 1 in original grid.

### ❌ Pitfall 2: Including empty spaces at boundaries

```rust
// Wrong: r >= min && r <= max
let empty_between = empty_rows.iter()
    .filter(|&&r| r >= min_row && r <= max_row)
    .count();

// Right: r > min && r < max (exclusive bounds)
let empty_between = empty_rows.iter()
    .filter(|&&r| r > min_row && r < max_row)
    .count();
```

**Example**:
```
Galaxy at row 3, galaxy at row 7
Empty row at row 3

Should row 3 count as "between" them?
NO - it's AT galaxy 1's position, not between them
Use > and < (not >= and <=)
```

### ❌ Pitfall 3: Counting pairs twice

```rust
// Wrong: Counts each pair twice
for i in 0..n {
    for j in 0..n {
        if i != j {
            // (A,B) and (B,A) both counted
        }
    }
}

// Right: Each pair once
for i in 0..n {
    for j in (i+1)..n {
        // Only (A,B), never (B,A)
    }
}
```

---

## Key Takeaways

1. **Abstraction over expansion**: Don't expand grid physically - track offsets logically
2. **Shared solver pattern**: When Part 1 and Part 2 differ only by parameter, extract common function
3. **Set complement**: Use HashSet to find missing elements in range efficiently
4. **All pairs generation**: Nested loop with `j in (i+1)..n` avoids duplicates
5. **Manhattan distance**: Generalizes to non-uniform grids by adding weighted offsets
6. **Functional iterators**: `flat_map`, `filter`, `map` create clear data pipelines
7. **Zero allocation optimization**: No intermediate grid creation saves time and memory

---

## Follow-Up Questions

1. **Extension**: How would you handle 3D cosmic expansion? (Add z-coordinate + empty planes)
2. **Optimization**: Could you use parallel processing for distance calculations? (Yes, with rayon)
3. **Generalization**: What if different empty rows had different expansion factors? (Vec<(usize, usize)> for (index, factor))
4. **Alternative Distance**: What if galaxies used Euclidean distance instead of Manhattan? (sqrt((x2-x1)² + (y2-y1)²) + same expansion logic)
5. **Data Structure**: When would you need a full Grid<T> instead of Vec<Position>? (When you need to query arbitrary positions, not just galaxies)

---

## Zettelkasten Cross-References

**Related Concepts**:
- [[manhattan-distance]] - L1 metric on grids
- [[graph-theory-fundamentals]] - Complete graphs, all pairs
- [[combinatorics-basics]] - C(n,k) combinations
- [[set-theory-fundamentals]] - Set complement operations
- [[mission-6]] - Grid<T> for when you need full 2D storage

**Pattern Connections**:
- [[aoc-parsing-patterns]] - flat_map for nested iteration
- [[aoc-grid-patterns]] - Common grid problem approaches
- [[functional-iteration-patterns]] - Iterator combinators in Rust

---

**Benchmarks**: 728.3µs (Part 1), 728.2µs (Part 2)  
**Lines of Code**: ~90 (excluding tests)  
**Test Coverage**: 6 tests, all passing ✅
