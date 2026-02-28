# Project Euler P018: Maximum Path Sum I (Triangle DP)

**Tags**: #project-euler #dynamic-programming #graph-theory #DAG #optimal-substructure
**Date**: 2026-02-28
**Answer**: 1,074
**Performance**: 1.92µs

---

## Problem

Find the maximum sum path from top to bottom of a 15-row triangle, where
each step moves to one of two adjacent numbers in the row below.

---

## Mathematical Foundation

### The Triangle as a DAG

The triangle is a **Directed Acyclic Graph (DAG)**:
- **Nodes**: Each cell `(row, col)`
- **Edges**: `(row, col) → (row+1, col)` and `(row, col) → (row+1, col+1)`
- **Goal**: Maximum weight path from root `(0,0)` to any leaf `(n-1, *)`

```
     (0,0)
    /     \
  (1,0)  (1,1)
  /  \   /  \
(2,0)(2,1)(2,2)
```

This is exactly **longest path in a DAG** — solvable in O(V+E) = O(n²) by
processing vertices in **topological order** (bottom-up).

### Why Greedy Fails — Counterexample

```
     5
    9  1
  1  1  100
```

| Strategy | Path | Sum |
|----------|------|-----|
| **Greedy** (local max) | 5 → 9 → 1 | **15** ❌ |
| **DP** (global max) | 5 → 1 → 100 | **106** ✅ |

Greedy makes an irrevocable choice at each step without knowing future values.
DP considers the full subtree before deciding.

### Optimal Substructure

Define `best(r, c)` = maximum path sum starting at `(r, c)`:

```
best(r, c) = T[r][c] + max(best(r+1, c), best(r+1, c+1))
best(last, c) = T[last][c]       ← base case: leaf nodes
```

This recurrence has **optimal substructure**: the best path through `(r,c)`
depends only on the best paths through its two children. No overlapping
decisions need reconsidering.

---

## Algorithm: Bottom-Up DP (In-Place)

Process rows from `n-2` down to `0`. At each cell, replace its value with
`value + max(left_child, right_child)`. After all rows, `T[0][0]` is the answer.

```
Start:            After row 2:      After row 1:      Final:
   3                  3                  3               23
  7 4               7  4              20  19
 2 4 6            10 13 15
8 5 9 3
```

Step by step on row 2: `[2+max(8,5), 4+max(5,9), 6+max(9,3)] = [10, 13, 15]`
Step by step on row 1: `[7+max(10,13), 4+max(13,15)] = [20, 19]`
Step on row 0: `[3+max(20,19)] = [23]`

### Rust Implementation

```rust
pub fn max_path_sum(triangle: &mut [Vec<u32>]) -> u32 {
    let n = triangle.len();
    for row in (0..n - 1).rev() {
        for col in 0..triangle[row].len() {
            let best_child = triangle[row + 1][col].max(triangle[row + 1][col + 1]);
            triangle[row][col] += best_child;
        }
    }
    triangle[0][0]
}
```

**Key design choices:**
- `&mut [Vec<u32>]` (slice of Vecs) not `&mut Vec<Vec<u32>>` — clippy prefers the slice
- In-place modification: O(1) extra space
- `.rev()` ensures bottom-up order

---

## Complexity

| Metric | Value |
|--------|-------|
| Time | O(n²) — two nested loops over triangle cells |
| Space | O(1) extra — modified in-place |
| Total cells | n(n+1)/2 = 120 for n=15 |
| Benchmark | 1.92µs |

---

## Connection to Problem 67

Problem 67 is the **same problem** with a 100-row triangle (from a file).
The same `max_path_sum` function works directly — just parse a different input.

- P18: 15 rows, 120 cells, could brute-force (16,384 paths)
- P67: 100 rows, 5,050 cells, cannot brute-force (2^99 ≈ 10^30 paths)

The DP solution handles both in microseconds.

---

## Pattern: DP on DAGs

This problem is a template for **longest/shortest path on a DAG**:

1. **Identify the DAG structure** (topological order = row order here)
2. **Define optimal substructure** (best path through node = node + best child)
3. **Process in topological order** (leaves first, root last)
4. **Read answer from root**

Applies to: triangle problems, grid DP, dependency chains, edit distance,
coin change, subset sum, and many more.

---

## Related

- [[project-euler-p067]] — Same algorithm, 100-row triangle from file
- [[project-euler-p015]] — Lattice paths (different DAG structure)
- [[dynamic-programming-fundamentals]] — Optimal substructure, overlapping subproblems

---

*Part of the [[project-euler-index]] series*
