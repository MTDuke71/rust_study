# Project Euler Problem 11: Largest Product in a Grid

**Answer:** 70,600,674  
**Performance:** ~6.8 μs  
**Difficulty:** 5%

## Problem Summary

Find the greatest product of four adjacent numbers in the same direction (horizontal, vertical, or diagonal) in a 20×20 grid.

**Example:** In the given grid, the diagonal sequence 26 × 63 × 78 × 14 = 1,788,696.

## Solution Approach

### Core Algorithm: Grid Traversal with Directional Checks

```rust
pub fn largest_product_in_grid(grid: &[Vec<u32>]) -> u64 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut max_product = 0u64;
    
    let directions = [
        Direction::Horizontal,
        Direction::Vertical,
        Direction::DiagonalDownRight,
        Direction::DiagonalDownLeft,
    ];
    
    for row in 0..rows {
        for col in 0..cols {
            for &dir in &directions {
                if let Some(product) = product_at(grid, row, col, dir) {
                    max_product = max_product.max(product);
                }
            }
        }
    }
    
    max_product
}
```

## Key Insight: Diagonal Index Patterns

### Visual Guide to Movement Patterns

```
Grid with indices (row, col):
    0   1   2   3
0  [A] [B] [C] [D]
1  [E] [F] [G] [H]
2  [I] [J] [K] [L]
3  [M] [N] [O] [P]

Horizontal (→) from A:
  A(0,0) → B(0,1) → C(0,2) → D(0,3)
  Pattern: grid[row][col+i]

Vertical (↓) from A:
  A(0,0) → E(1,0) → I(2,0) → M(3,0)
  Pattern: grid[row+i][col]

Diagonal Down-Right (↘) from A:
  A(0,0) → F(1,1) → K(2,2) → P(3,3)
  Pattern: grid[row+i][col+i]
  Both indices increment TOGETHER

Diagonal Down-Left (↙) from D:
  D(0,3) → G(1,2) → J(2,1) → M(3,0)
  Pattern: grid[row+i][col-i]
  Row increments, column DECREMENTS
```

### The Diagonal Trick

**Key Realization:** Diagonals move in **two dimensions simultaneously**:

```rust
// Down-Right (↘): Both indices move in SAME direction
for i in 0..4 {
    product *= grid[row + i][col + i];  // Both +i
}

// Down-Left (↙): Indices move in OPPOSITE directions
for i in 0..4 {
    product *= grid[row + i][col - i];  // row +i, col -i
}
```

**Mental Model:**
- **Horizontal/Vertical:** One dimension fixed, one moves
- **Diagonals:** Both dimensions move (same or opposite directions)

## Bounds Checking by Direction

Critical for avoiding out-of-bounds access:

```rust
match dir {
    Direction::Horizontal => {
        // Need 4 columns to the right
        if col + 3 < cols { /* calculate */ } else { None }
    }
    Direction::Vertical => {
        // Need 4 rows below
        if row + 3 < rows { /* calculate */ } else { None }
    }
    Direction::DiagonalDownRight => {
        // Need both row AND col space
        if row + 3 < rows && col + 3 < cols { /* calculate */ } else { None }
    }
    Direction::DiagonalDownLeft => {
        // Need rows below, but cols to the LEFT (col >= 3)
        if row + 3 < rows && col >= 3 { /* calculate */ } else { None }
    }
}
```

**Important:** Down-Left diagonal requires `col >= 3` because we're subtracting from column index!

## Complexity Analysis

### Time Complexity: O(n²)

For an n×n grid:
- **Cells to check:** n²
- **Directions per cell:** 4
- **Work per direction:** O(1) - just 4 multiplications
- **Total:** 4n² = O(n²)

For 20×20 grid:
- 400 cells × 4 directions = **1,600 direction checks**
- Each check: 4 multiplications + 3 additions = 7 operations
- Total operations: ~11,200

### Space Complexity: O(n²)

- Grid storage: n² elements
- All other variables: O(1)

### Practical Performance

| Grid Size | Cells | Direction Checks | Time (measured) |
|-----------|-------|------------------|-----------------|
| 4×4       | 16    | 64               | < 1 μs          |
| 10×10     | 100   | 400              | ~2 μs           |
| 20×20     | 400   | 1,600            | **~6.8 μs**     |
| 100×100   | 10,000| 40,000           | ~340 μs (est.)  |

**Observation:** Nearly perfect linear scaling with grid size (as predicted by O(n²)).

## Implementation Details

### Direction Enum for Clean Code

```rust
#[derive(Debug, Clone, Copy)]
enum Direction {
    Horizontal,
    Vertical,
    DiagonalDownRight,
    DiagonalDownLeft,
}
```

**Benefits:**
- Type-safe direction handling
- Clear pattern matching
- Self-documenting code
- Easy to extend (could add 4 more diagonals if needed)

### Product Calculation with u64

```rust
Some(
    grid[row][col] as u64
        * grid[row][col + 1] as u64
        * grid[row][col + 2] as u64
        * grid[row][col + 3] as u64
)
```

**Why u64?**
- Grid values are u32 (max 99 in this problem)
- Product of 4 values: 99⁴ = 96,059,601 (fits in u32)
- BUT: Use u64 to prevent overflow in general case
- Answer 70,600,674 fits comfortably in u64

### Grid Parsing

```rust
fn parse_grid(data: &str) -> Vec<Vec<u32>> {
    data.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}
```

**Clean parsing:**
- Split by lines
- Split each line by whitespace
- Parse each number to u32
- Collect into Vec<Vec<u32>>

## Alternative Approaches

### ❌ Check All 8 Directions

```rust
// Could check up, down, left, right, and all 4 diagonals
// BUT: Redundant! Up from (3,3) == Down from (0,0)
```

**Why we only need 4:**
- Horizontal covers both → and ← (same line)
- Vertical covers both ↓ and ↑ (same line)
- Each diagonal covers both directions

### ✅ Our Approach: 4 Directions, Down/Right Bias

By only checking down and right directions, we avoid duplicate checks while covering all possible sequences.

## Edge Cases and Testing

### Bounds Testing

```rust
#[test]
fn test_horizontal_out_of_bounds() {
    let grid = vec![vec![1, 2, 3]]; // Only 3 columns
    // Can't fit 4 consecutive horizontally
    assert_eq!(product_at(&grid, 0, 0, Direction::Horizontal), None);
}
```

### Small Grid Verification

```rust
#[test]
fn test_largest_product_small_grid() {
    let grid = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];
    // Max: Horizontal from (3,0): 13*14*15*16 = 43,680
    assert_eq!(largest_product_in_grid(&grid), 43680);
}
```

### Direction-Specific Tests

Each direction tested individually:
- `test_horizontal_product` - Row traversal
- `test_vertical_product` - Column traversal
- `test_diagonal_down_right` - Main diagonal pattern
- `test_diagonal_down_left` - Anti-diagonal pattern

## Common Pitfalls

### 1. **Off-by-One in Bounds Checking**

```rust
// ❌ WRONG: Would miss last valid position
if col + 4 < cols { /* ... */ }

// ✅ CORRECT: +3 because we need positions i, i+1, i+2, i+3
if col + 3 < cols { /* ... */ }
```

### 2. **Forgetting col >= 3 for Down-Left**

```rust
// ❌ WRONG: Will panic when col < 3
if row + 3 < rows { /* col - i will be negative! */ }

// ✅ CORRECT: Check column has room to go left
if row + 3 < rows && col >= 3 { /* safe */ }
```

### 3. **Integer Overflow**

```rust
// ❌ WRONG: Could overflow with large values
let product: u32 = grid[row][col] * grid[row][col+1] * ...;

// ✅ CORRECT: Cast to u64 before multiplication
let product: u64 = grid[row][col] as u64 * grid[row][col+1] as u64 * ...;
```

## Visualization of Solution

For the 20×20 grid, the maximum product is found in one of the directions. The algorithm efficiently checks all possibilities:

```
Total search space:
- Horizontal: 20 rows × 17 valid starting cols = 340 checks
- Vertical: 17 valid starting rows × 20 cols = 340 checks
- Diagonal ↘: 17×17 valid starting positions = 289 checks
- Diagonal ↙: 17×17 valid starting positions = 289 checks
Total: 1,258 valid sequences checked (some positions can't start all 4 directions)
```

## Connection to Computer Graphics

This problem introduces fundamental **grid traversal patterns** used in:

### Image Processing
```rust
// Convolution kernels, edge detection
for row in 0..height {
    for col in 0..width {
        apply_kernel(image, row, col, kernel);
    }
}
```

### Game Development
```rust
// Pathfinding, line-of-sight, tile adjacency
fn get_neighbors(row: usize, col: usize) -> Vec<(usize, usize)> {
    // Check 4 or 8 directions from current position
}
```

### Sliding Window Algorithms
```rust
// 2D pattern matching, feature detection
for row in 0..rows - window_size {
    for col in 0..cols - window_size {
        check_window(grid, row, col, window_size);
    }
}
```

## Rust-Specific Features Used

### Pattern Matching on Enums

```rust
match dir {
    Direction::Horizontal => { /* ... */ }
    Direction::Vertical => { /* ... */ }
    // Compiler ensures all cases handled
}
```

### Option for Safe Bounds

```rust
fn product_at(...) -> Option<u64> {
    if bounds_ok {
        Some(product)
    } else {
        None  // Safe: no panic, just no value
    }
}
```

### Iterator Chains for Parsing

```rust
data.lines()
    .map(|line| line.split_whitespace().map(...).collect())
    .collect()
// Functional, composable, efficient
```

## Key Takeaways

1. **Diagonals = 2D movement:** Both row and column change simultaneously
2. **Bounds checking varies by direction:** Consider movement pattern
3. **Avoid redundancy:** 4 directions sufficient (no need for 8)
4. **Type safety:** Enum for directions prevents bugs
5. **Grid problems are O(n²):** Linear in number of cells

## Performance Optimization Notes

Current implementation is already well-optimized:
- ✅ Single pass through grid
- ✅ Minimal allocations (only grid storage)
- ✅ Early termination via bounds checking (returns None immediately)
- ✅ u64 arithmetic is very fast

**Possible micro-optimizations** (not needed for this problem):
- Pre-calculate valid ranges for each direction
- Use unsafe indexing (skip bounds checks) after validation
- Parallelize row processing (overkill for 20×20)

## Implementation Location

- **Problem:** `project_euler/src/problems/p011.rs`
- **Tests:** 7 unit tests + 2 integration tests + 1 doctest
- **Benchmark:** `benches/benchmarks.rs`

---

*Tags: #project-euler #grid-algorithms #2d-arrays #diagonal-traversal #pattern-matching*

*Difficulty: 5/100*

**Links:**
- [[sorting-algorithms]] - Related: grid/array manipulation
- [[project-euler-p008]] - Related: sliding window product
- [[project-euler-p010]] - Previous problem (sieve of Eratosthenes)
