# Day 22: Grid Computing — Function Guide

## Problem Summary

A storage cluster is arranged as a 30x32 grid of nodes. Each node has a size, used space, and available space. Data can only move between adjacent nodes if the destination has enough room.

**Part 1**: Count **viable pairs** (A,B) where A is non-empty and A's used data fits in B's available space.
**Part 2**: Find the minimum moves to bring the goal data from node (29,0) to the access point (0,0).

---

## Answers

| Part | Answer | Time |
|------|--------|------|
| 1 | `937` | 487us |
| 2 | `188` | 194us |
| Combined | — | 517us |

---

## Function Map

```
solve(input) -> (String, String)
  ├── parse_input(input) -> Vec<Node>               # parse df output into node structs
  ├── count_viable_pairs(&[Node]) -> usize           # O(n^2) pair check
  └── solve_part2_geometry(&[Node]) -> usize
        └── bfs_empty(start, target, goal, wall)     # BFS empty node around wall to goal
            + geometric formula for sliding goal left
```

---

## Algorithm Details

### Grid Structure

The input is a `df -h` output with 960 nodes (30 columns x 32 rows):

| Node Type | Count | Used | Role |
|-----------|-------|------|------|
| Normal | 937 | 64-73T | Passable — data can move between these |
| Wall | 22 | 490-499T | Impassable — too much data to move anywhere |
| Empty | 1 | 0T | The "hole" in the sliding puzzle |

The wall forms a horizontal barrier at y=14, from x=8 to x=29:

```
(0,0) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . G (29,0)
  .   . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .
  .   . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .
  ...
  .   . . . . . . . . # # # # # # # # # # # # # # # # # # # # # # (y=14)
  ...
  .   . . . . . . . . . . _ . . . . . . . . . . . . . . . . . . . (y=22)
                               ^ empty at (11,22)
```

### Part 1: Viable Pairs

Brute-force O(n²) check: for each pair (A,B) where A != B, count where A.used > 0 and A.used <= B.avail.

With 960 nodes, this is ~920K comparisons — fast enough at 487us.

### Part 2: Sliding Puzzle

This is the key insight: the grid behaves like a **15-puzzle** (sliding tile game). There's exactly one empty node, and the goal data at (29,0) needs to reach (0,0).

The solution has two phases:

**Phase 1 — Move empty next to goal**: BFS the empty node from (11,22) to (28,0), navigating around the wall at y=14 and avoiding the goal position at (29,0).

**Phase 2 — Slide goal left**: Once empty is at (28,0) and goal at (29,0):
1. Swap goal into empty (goal moves to x=28, empty to x=29) — **1 move**
2. For each remaining shift left (28 more times):
   - Move empty around the goal: down, left, left, up — **4 moves**
   - Swap goal into empty — **1 move**
   - Total: **5 moves per shift**

Formula: `1 + (goal_x - 1) * 5 = 1 + 28 * 5 = 141 moves` for the sliding phase.

Total: BFS distance (47) + sliding (141) = **188 moves**.

### Complexity

- **Part 1**: O(n²) where n=960 nodes
- **Part 2**: O(width × height) for BFS + O(width) for sliding formula
- **Space**: O(width × height) for BFS visited grid

---

## Key Observations

1. **Only the empty node can receive data**: The smallest used value (excluding zero) is 64T, but the largest avail on any non-empty node is only 30T. Since 64 > 30, no normal node can move its data into another normal node. Data can *only* move into the empty node — this is what makes the grid a sliding puzzle with exactly one "hole".
2. **Three node types**: The ~500T "wall" nodes can't move their data anywhere (no node has 500T available). They form a barrier the empty node must navigate around.
2. **Sliding puzzle reduction**: With exactly one empty node and uniform normal nodes (any normal node's data fits in the empty), this reduces to a classic sliding puzzle.
3. **Geometry beats search**: Full state-space BFS on (empty_pos, goal_pos) would work but is overkill. The empty node's path to the goal is a simple BFS, then the sliding pattern is a fixed 5-move cycle.
4. **Wall threshold**: Normal nodes use 64-73T, walls use 490-499T. The `used > 100` threshold cleanly separates them.
5. **Part 1 dominates runtime**: The O(n²) viable pair counting takes 487us vs 194us for the Part 2 BFS + geometry. Could optimize with sorting, but not worth it at this scale.

---

## Benchmarks

```
day22_combined          517us
day22_part1             487us
day22_part2             194us
```

Part 1's O(n²) pair counting dominates. Part 2's BFS on a 30x32 grid is trivial.

---

**See also**: [AoC 2016 Summary](../summary_2016.md)
