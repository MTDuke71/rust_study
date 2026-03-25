# Day 24: Air Duct Spelunking — Function Guide

## Problem Summary

Navigate a cleaning robot through an HVAC duct maze. The maze has walls (`#`), open passages (`.`), and numbered locations (`0`-`7`). Starting from location `0`, visit every other numbered location in the fewest total steps.

**Part 1**: Shortest route from `0` visiting all numbered locations. Answer: **500**
**Part 2**: Shortest route from `0` visiting all locations AND returning to `0`. Answer: **748**

---

## Answers

| Part | Answer | Time (release) |
|------|--------|------|
| 1 | `500` | — |
| 2 | `748` | — |
| Combined | — | 480us |

---

## Function Map

```
solve(input) -> (String, String)
  ├── parse_input(input) -> Maze              # BFS from each point, build distance matrix
  │     ├── find numbered points (0..n)       # scan grid for digits
  │     └── bfs(grid, start) -> dist[][]      # flood-fill from each point
  ├── solve_part1_with_data(&maze) -> u32     # TSP without return
  └── solve_part2_with_data(&maze) -> u32     # TSP with return to 0
        └── tsp(maze, return_home) -> u32
              └── next_permutation(&mut arr)  # lexicographic permutation generator
```

---

## Algorithm Details

### Two-Phase Approach: BFS + TSP

**Phase 1 — Pairwise BFS distances:**
The maze is a large grid (~37x183) but has only 8 numbered points (0-7). Run BFS from each numbered point to compute the shortest path to every other numbered point. This produces an 8x8 distance matrix.

**Phase 2 — Brute-force TSP:**
With only 7 non-zero points to visit, there are 7! = 5,040 permutations. Try every ordering starting from point 0, sum the pairwise distances along the route, and track the minimum.

### Why Brute Force Works

TSP is NP-hard in general, but with only 8 nodes the search space is tiny. 5,040 permutations × 7 additions each = ~35K operations. No need for dynamic programming (Held-Karp) or heuristics.

### BFS Implementation

Uses `wrapping_add` with `!0usize` (which is `usize::MAX`) for the -1 direction offsets. This avoids signed arithmetic — `r.wrapping_add(!0usize)` underflows to `usize::MAX` which fails the `< rows` bounds check naturally.

### Permutation Generator

Custom `next_permutation` follows the standard lexicographic algorithm:
1. Find rightmost ascent `arr[i-1] < arr[i]`
2. Swap `arr[i-1]` with smallest element to its right that's larger
3. Reverse the suffix after position `i-1`

Starting from sorted order `[1,2,3,4,5,6,7]`, this generates all 5,040 permutations in-place.

---

## Performance

| Metric | Value |
|--------|-------|
| Combined (release) | 480us |
| Grid size | ~37 x 183 (~6,800 cells) |
| Numbered points | 8 (0-7) |
| BFS runs | 8 |
| Permutations checked | 5,040 |

BFS dominates (~95% of runtime). The permutation phase is negligible.

---

## Key Insight

This is a classic "reduce to smaller problem" pattern. The maze grid has thousands of cells, but the actual question only cares about distances between 8 specific points. BFS collapses the grid into an 8x8 matrix, then the problem becomes a trivial-sized TSP. Part 2 is identical to Part 1 but adds the return-to-origin edge — a single addition per permutation.

---

## Data Flow

```
Input (37x183 grid with #, ., 0-7)
  │
  ├─ Scan: find 8 point locations
  │
  ├─ 8× BFS: flood-fill from each point
  │
  └─ 8x8 distance matrix
       │
       ├─ Part 1: min over 7! routes from 0
       │   → 500
       │
       └─ Part 2: min over 7! routes from 0, returning to 0
           → 748
```

---

**See also**: [AoC 2016 Summary](../summary_2016.md)
