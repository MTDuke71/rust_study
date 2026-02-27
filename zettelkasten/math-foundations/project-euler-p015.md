# Project Euler Problem 15: Lattice Paths - Mathematical Analysis

*Status: Solved*
*Difficulty: 5%*
*Answer: 137846528820*
*Date: 2026-02-26*

---

## Problem Statement

Starting in the top left corner of a 2x2 grid, and only being able to move right and down, there are exactly 6 routes to the bottom right corner. How many such routes are there through a 20x20 grid?

---

## Mathematical Foundation

### Core Insight: Path Encoding

Every path through an n x n grid consists of exactly:
- **n moves right** (R)
- **n moves down** (D)
- **2n total moves**

The path is completely determined by choosing *which* of the 2n steps are R (the rest must be D).

### Example: 2x2 Grid

The 6 paths encoded as move sequences:

```
RRDD    RDRD    RDDR
DRRD    DRDR    DDRR
```

Each is a sequence of 4 moves containing exactly 2 R's and 2 D's. The number of such sequences is C(4, 2) = 6.

### General Formula

For an n x n grid:

```
Routes = C(2n, n) = (2n)! / (n! * n!)
```

This is the **central binomial coefficient** - the middle entry of row 2n in Pascal's triangle.

### For n = 20:

```
C(40, 20) = 40! / (20! * 20!) = 137,846,528,820
```

---

## Why This Works: Bijection Proof

**Claim**: There is a one-to-one correspondence between lattice paths and binary strings of length 2n with exactly n ones.

**Proof**:

1. **Path → String**: Given a path, write "1" for each right move and "0" for each down move. The result is a binary string of length 2n with exactly n ones.

2. **String → Path**: Given a binary string of length 2n with exactly n ones, read left to right: move right on "1", move down on "0". This traces a valid path from (0,0) to (n,n).

3. **Bijection**: These maps are inverses of each other. Every path produces a unique string, and every valid string produces a unique path.

4. **Count**: The number of binary strings of length 2n with exactly n ones is C(2n, n) by definition of the binomial coefficient.

Therefore: number of lattice paths = C(2n, n). QED

---

## Connection to Pascal's Triangle

The central binomial coefficients form the middle diagonal of Pascal's triangle:

```
n=0:  C(0,0) = 1
n=1:  C(2,1) = 2
n=2:  C(4,2) = 6
n=3:  C(6,3) = 20
n=4:  C(8,4) = 70
...
n=20: C(40,20) = 137,846,528,820
```

### Growth Rate

The central binomial coefficient grows approximately as:

```
C(2n, n) ~ 4^n / sqrt(pi * n)    (Stirling's approximation)
```

For n=20: 4^20 / sqrt(20*pi) ~ 1.1 * 10^12 / 7.9 ~ 1.4 * 10^11, close to our answer.

---

## Alternative Approach: Dynamic Programming

While the combinatorial formula gives an instant answer, this problem can also be solved with DP on the grid itself:

```
Grid[i][j] = number of paths from (0,0) to (i,j)
Grid[0][j] = 1  (only way: all right moves)
Grid[i][0] = 1  (only way: all down moves)
Grid[i][j] = Grid[i-1][j] + Grid[i][j-1]  (arrive from above or left)
```

For a 2x2 grid (3x3 lattice points):

```
1  1  1
1  2  3
1  3  6  ← answer
```

For a 3x3 grid (4x4 lattice points):

```
1  1  1   1
1  2  3   4
1  3  6  10
1  4 10  20  ← answer
```

**Key observation**: This DP table IS Pascal's triangle rotated 45 degrees! Grid[i][j] = C(i+j, i).

### Complexity Comparison

| Approach | Time | Space | Overflow Risk |
|----------|------|-------|---------------|
| Binomial formula | O(n) | O(1) | Low (incremental multiply/divide) |
| DP grid | O(n^2) | O(n^2) or O(n) | Low (incremental addition) |
| Brute force recursion | O(C(2n,n)) | O(n) stack | N/A (too slow) |

The binomial formula wins on all fronts for this problem.

---

## Rust Implementation Details

### Integrator Approach: Reuse `utils/combinatorics.rs`

The entire solution delegates to the shared `binomial()` utility:

```rust
use crate::utils::combinatorics;

pub fn lattice_paths(n: u64) -> u64 {
    combinatorics::binomial(2 * n, n)
}

pub fn solve() -> u64 {
    lattice_paths(20)
}
```

### Why `binomial()` Doesn't Overflow

The shared implementation uses incremental multiply-then-divide:

```rust
pub fn binomial(n: u64, k: u64) -> u64 {
    let k = k.min(n - k);  // C(40,20) = C(40,20), k=20
    let mut result = 1u64;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}
```

Each step computes: result = result * (40-i) / (i+1)

```
i=0:  1 * 40 / 1  = 40
i=1:  40 * 39 / 2  = 780
i=2:  780 * 38 / 3  = 9880
...
i=19: ... / 20     = 137,846,528,820
```

The division is always exact because C(n,k) is always an integer. The intermediate values stay manageable (max ~10^17, well within u64's ~1.8 * 10^19).

---

## Generalizations

### Rectangular Grids

For an m x n grid (not necessarily square):

```
Routes = C(m + n, m) = C(m + n, n)
```

Need m right moves and n down moves out of m+n total.

### Lattice Paths with Obstacles

If certain grid points are blocked, the DP approach is needed — the combinatorial formula doesn't apply directly. Use inclusion-exclusion or modify the DP grid:

```
Grid[i][j] = 0  if (i,j) is blocked
Grid[i][j] = Grid[i-1][j] + Grid[i][j-1]  otherwise
```

### Catalan Numbers

If we add the constraint that the path never crosses above the diagonal (stays below or on the line y=x), the count becomes the Catalan number:

```
Catalan(n) = C(2n, n) / (n + 1)
```

For n=20: C(40,20) / 21 = 6,564,120,420

---

## Related Problems

### Project Euler
- **Problem 18/67**: Maximum path sum (DP on triangle, similar grid traversal)
- **Problem 81**: Path sum (minimal path through grid)
- **Problem 15 variants**: With obstacles, 3D grids, weighted paths

### Mission Connections
- **Mission 6**: Grid<T> data structure (grid representation)
- **Mission 11**: Memoization patterns (DP approach uses same cache concept)

### Math Concepts
- [[combinatorics-fundamentals]]: Binomial coefficients, counting principles
- [[dynamic-programming-theory]]: Grid DP as alternative approach
- [[pascal's-triangle]]: Central binomial coefficients are the middle entries

---

## Code Repository

**Location**: `project_euler/src/problems/p015.rs`

**Key functions**:
- `lattice_paths(n)` - Count paths through n x n grid via C(2n, n)
- `solve()` - Returns answer for 20x20 grid (137846528820)

**Tests**: 4 unit tests (1x1, 2x2 example, 3x3, solve)

**Problem Statement**: `project_euler/Problem_Statements/p015.md`

---

## Metadata

**Tags**: `#combinatorics` `#binomial-coefficient` `#lattice-paths` `#pascal-triangle` `#counting`

**Concepts**: Central binomial coefficient, path encoding bijection, Pascal's triangle, grid DP

**Difficulty**: Mathematical reasoning (easy), Implementation (trivial with shared utils)

**Created**: 2026-02-26
**Language**: Rust
**Performance**: Essentially instant (O(n) = 20 multiply-divide operations)

---

## Bidirectional Links

**Links From This Note**:
- [[combinatorics-fundamentals]] - Binomial coefficients and counting theory
- [[dynamic-programming-theory]] - Alternative DP approach on grid
- [[project-euler-p014]] - Previous problem (also uses memoization/DP concepts)

**Links To This Note**:
- [[project-euler-problems]] - Full list of solved problems
- [[combinatorics-fundamentals]] - Uses P15 as binomial coefficient example

---

*Last updated: 2026-02-26*
*Solution verified: Answer 137846528820*
*Integrator win: One-liner using shared combinatorics::binomial()*
