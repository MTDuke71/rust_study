# Day 13: A Maze of Twisty Little Cubicles — Function Guide

**Problem**: Navigate a procedurally-generated maze where walls are determined
by a bit-counting formula applied to (x, y) coordinates.

**Part 1**: Fewest steps from (1,1) to (31,39)? → **92**
**Part 2**: How many locations reachable in ≤50 steps? → **124**

---

## Performance

| Metric | Value |
|--------|-------|
| Combined | 18.9µs |
| Part 1 | 18.9µs |
| Part 2 | 19.0µs |
| Parse | negligible (single integer) |

All three are identical — a single BFS answers both questions simultaneously.

---

## Architecture

```
Input: "1350"
    │
    ▼
parse_input(input)           ──→ u64 (favorite number)
    │
    ▼
bfs(fav=1350, target=(31,39), max_steps=50)
    │
    ├── is_open(x, y, fav)   ──→ bool (wall or open?)
    │     └── x²+3x+2xy+y+y²+fav → count_ones → even=open
    │
    ├── BFS from (1,1)
    │     ├── Track dist_to_target  ──→ Part 1: 92
    │     └── Count nodes ≤50 steps ──→ Part 2: 124
    │
    └── BfsResult { dist_to_target, reachable_within_limit }
```

---

## Data Structures

### `BfsResult` (struct)
```rust
struct BfsResult {
    dist_to_target: u64,       // Shortest path to target (Part 1)
    reachable_within_limit: usize,  // Positions within max_steps (Part 2)
}
```

Bundles both answers from a single BFS traversal. No heap allocation —
just two integers.

---

## Function-by-Function

### `is_open(x: u64, y: u64, fav: u64) -> bool`
The maze generator. Computes:
1. `x*x + 3*x + 2*x*y + y + y*y + fav`
2. Count the `1` bits in the binary representation
3. Even count → open space, odd → wall

Uses `count_ones()` which maps to the hardware `popcnt` instruction —
single-cycle on modern CPUs.

**Key insight**: The maze is deterministic and infinite. No grid needs to
be stored — each cell is computed on demand.

### `bfs(fav, target, max_steps) -> BfsResult`
Standard BFS from (1,1) that answers both parts in one pass:

1. **Dequeue** node (x, y, steps)
2. If `steps ≤ max_steps` → count it for Part 2
3. If `(x, y) == target` → record distance for Part 1
4. If `steps ≥ max_steps` AND target found → stop expanding (optimization)
5. **Enqueue** valid neighbors (open, unvisited, non-negative)

The `visited.insert()` trick: `HashSet::insert` returns `true` if the
value was new — combines "check if visited" and "mark as visited" in one call.

**Neighbor generation**: Uses `wrapping_sub(1)` for the left/up directions.
On a `u64`, wrapping 0 gives `u64::MAX` which fails the `< 1000` bounds
check, so negative coordinates are implicitly rejected without needing `i64`.

### `parse_input(input: &str) -> u64`
Trims and parses the single-integer input. The simplest parser in the
entire AoC 2016 series.

### `solve_both(fav: u64) -> (u64, usize)`
Convenience wrapper that calls `bfs` with the puzzle-specific target (31,39)
and step limit (50).

### `solve(input) -> (String, String)`
Entry point. Parse → BFS → format both answers.

### `solve_part1(input) -> u64` / `solve_part2(input) -> usize`
Individual part solvers. Each runs the full BFS (both parts are answered
by the same traversal, so there's no waste).

---

## Algorithm: BFS on an Implicit Graph

This is a textbook BFS shortest-path problem, but with a twist: the graph
is **never constructed**. Instead of building adjacency lists, each node's
neighbors are computed on the fly via `is_open()`.

### Why BFS Works
- All edges have weight 1 (one step per move)
- BFS guarantees shortest path in unweighted graphs
- No need for Dijkstra or A* — uniform cost means BFS is optimal

### Why No Grid Storage
Traditional maze problems pre-build a 2D array. Here:
- The maze is **infinite** (no boundaries except non-negative coordinates)
- Only ~200 cells are ever visited (tiny fraction of the space)
- Computing `is_open()` is cheaper than a hash lookup for cached values
- `HashSet<(u64, u64)>` for visited is sufficient

### Dual-Answer BFS
Both parts share the same traversal:
- **Part 1**: Record distance when target is dequeued (BFS guarantees minimum)
- **Part 2**: Count every node dequeued with `steps ≤ 50`

The BFS continues past 50 steps only if the target hasn't been found yet
(it's at distance 92, well beyond 50). Once both answers are collected,
expansion stops.

### Complexity
- **Time**: O(V + E) where V = reachable cells, E = 4V (grid neighbors)
- **Space**: O(V) for the visited set and queue
- In practice: ~800 cells visited, ~3200 neighbor checks → 18.9µs

---

## The Wall Formula

```
value = x² + 3x + 2xy + y + y² + favorite_number
is_wall = popcount(value) is odd
```

This creates a deterministic pseudo-random maze from any integer seed.
The formula is a **quadratic form** in x and y — it produces a smooth
distribution of wall/open cells that creates navigable corridors.

### Example (fav=10, top-left corner)
```
  0123456789
0 .#.####.##
1 ..#..#...#
2 #....##...
3 ###.#.###.
4 .##..#..#.
5 ..##....#.
6 #.##.#.##.
```

Where `.` = open, `#` = wall. Start at (1,1), target varies by puzzle.

### Why `count_ones()` is Perfect Here
The `popcnt` hardware instruction counts set bits in a single cycle.
Combined with a few multiplications and additions, each cell classification
costs ~5 nanoseconds — far cheaper than any lookup table or cache.

---

## Concepts for Study

### Implicit vs Explicit Graphs
| Aspect | Explicit Graph | Implicit Graph (this problem) |
|--------|---------------|-------------------------------|
| Storage | Adjacency list/matrix | None — computed on demand |
| Memory | O(V + E) | O(visited only) |
| Setup | Parse/build upfront | Zero setup time |
| Best for | Reused traversals | Single traversal |

Many AoC problems use implicit graphs: the "graph" is defined by rules
(valid moves, wall checks) rather than stored edges.

### HashSet as Visited Set
```rust
if visited.insert((nx, ny)) {  // returns true if NEW
    queue.push_back((nx, ny, steps + 1));
}
```

This is an idiomatic Rust pattern — `insert` returns whether the value
was actually inserted (i.e., wasn't already present). Combines the
"contains?" check and "add" operation into one hash lookup.

### `wrapping_sub` for Unsigned Boundary Handling
```rust
x.wrapping_sub(1)  // 0u64.wrapping_sub(1) = u64::MAX
```

Instead of using `i64` and checking for negatives, stay in `u64` and let
wrapping arithmetic produce values that fail the bounds check naturally.
The `< 1000` guard catches `u64::MAX` without a separate negative check.

---

## Potential Optimizations (Not Applied)

| Optimization | Expected Impact | Why Not |
|-------------|-----------------|---------|
| A* with Manhattan distance | ~2× fewer nodes explored | 18.9µs is already trivial |
| FxHashSet instead of HashSet | ~30% hash speedup | Not worth the dependency for µs |
| Bitset instead of HashSet | Cache-friendly, no hashing | Would need bounded coordinates |
| Pre-compute grid | Amortize is_open cost | Only ~800 cells visited — not worth it |

At 18.9µs, this problem is I/O-bound if anything. The algorithm is already
optimal (BFS for unweighted shortest path).

---

## Input Analysis

- **Input**: Single integer `1350` (office designer's favorite number)
- **Target**: (31, 39) — about 50 Manhattan distance from start
- **Actual shortest path**: 92 steps (longer than Manhattan due to walls)
- **Reachable in 50 steps**: 124 distinct positions
- **Total cells visited by BFS**: ~800 (to reach distance 92)
