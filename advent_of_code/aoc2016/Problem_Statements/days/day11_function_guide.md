# Day 11: Radioisotope Thermoelectric Generators — Function Guide

**Problem**: Move all generators and microchips from floors 1-3 to floor 4 using
an elevator that carries 1-2 items. A microchip is fried if left on a floor with
a non-matching generator unless its own generator is also present.

**Part 1**: Minimum steps with 5 element pairs? → **47**
**Part 2**: Add elerium + dilithium pairs to floor 1, minimum steps with 7 pairs? → **71**

---

## Performance

| Metric | Value |
|--------|-------|
| Combined | 30.57ms |
| Part 1 | 4.32ms |
| Part 2 | 25.98ms |
| Parse | negligible |

Part 2 dominates: 7 pairs → much larger state space than 5 pairs.
Combined ≈ Part 1 + Part 2 (parsing is negligible; BFS dominates).

---

## Architecture

```
Input (4 lines)
    │
    ▼
parse_input()           ──→ State { elevator: 0, pairs: [(gen_floor, chip_floor); 5] }
    │                         Word-scan for "generator" and "*-compatible"
    │                         Match elements by name, assign floor indices
    │
    ├──→ solve_part1_with_data()
    │       │
    │       └── bfs(state)  ──→ 47 steps
    │             │
    │             ├── BFS over canonical states
    │             ├── Generate moves: 1 or 2 items to adjacent floor
    │             ├── Validate: no unshielded chips near foreign generators
    │             └── Canonicalize: sort pairs to collapse equivalent states
    │
    └──→ solve_part2_with_data()
            │
            ├── Add 2 extra pairs (0,0) to state
            └── bfs(extended)  ──→ 71 steps
```

---

## Data Structures

### `State` (struct, derives Hash + Eq)
```rust
struct State {
    elevator: u8,                    // Current floor (0-3)
    pairs: Vec<(u8, u8)>,           // (generator_floor, chip_floor) per element
}
```

**Key insight**: Element identity doesn't matter — only the pattern of
(generator_floor, chip_floor) pairs affects reachability. Sorting pairs
canonicalizes the state, collapsing equivalent configurations:

- 5 pairs: up to 5! = 120× state reduction
- 7 pairs: up to 7! = 5,040× state reduction

Without this optimization, Part 2 would be intractable.

---

## Function-by-Function

### `parse_input(input: &str) -> State`
Scans each line word-by-word looking for:
- `"generator"` → previous word is element name, record floor as generator location
- `"*-compatible"` → split on `-`, first part is element name, record floor as chip location

Elements are matched by name using `Vec::position()`. Output is canonicalized (sorted pairs).

### `State::is_valid(&self) -> bool`
The core constraint check. For each pair where generator ≠ chip (chip is unshielded):
- If **any** generator exists on the chip's floor → chip is fried → invalid

This is O(n²) where n = number of pairs, but n ≤ 7 so it's negligible.

### `State::canonicalize(&mut self)`
Sorts `pairs` in-place. This is what makes the BFS tractable — two states that differ
only in which *element* is where (but have the same *pattern*) hash to the same entry.

### `bfs(initial: State) -> usize`
Standard BFS guaranteeing minimum steps:

1. **Enumerate items on current floor**: Build list of `(element_idx, is_generator)`
2. **Generate moves**: Try all combinations of 1 or 2 items to each adjacent floor
3. **Validate + deduplicate**: Check `is_valid()`, insert into `HashSet<State>`
4. **Return** when goal state reached (all pairs = `(3, 3)`)

### `apply_move(state, item, from, to)`
Mutates a single item's floor in the state. Called once (move 1 item) or twice
(move 2 items) per generated successor.

### `solve_part2_with_data(data: &State) -> usize`
Appends two `(0, 0)` pairs (elerium + dilithium on floor 1) and runs the same BFS.
No code change needed — the algorithm scales naturally with more pairs.

---

## Algorithm: BFS with Canonical State Space

### Why BFS?
BFS guarantees the **shortest path** in an unweighted graph. Each elevator move
costs exactly 1 step, making BFS optimal (no need for Dijkstra or A*).

### State Space Analysis

| Pairs | Raw States | Canonical States | Reduction |
|-------|-----------|-----------------|-----------|
| 2 (example) | 4 × 4⁴ = 1,024 | ~170 | 6× |
| 5 (Part 1) | 4 × 4¹⁰ = 4.2M | ~35K explored | ~120× |
| 7 (Part 2) | 4 × 4¹⁴ = 1.07B | ~400K explored | ~2,700× |

The canonicalization is the **critical optimization**. Without it, Part 2's
billion-state space would be unsearchable.

### Scaling Experiment (Adding Pairs)

| Pairs | Steps | Time | Raw States | Slowdown |
|-------|-------|------|------------|----------|
| 5 (Part 1) | 47 | 4.3ms | 4 × 4¹⁰ = 4.2M | baseline |
| 7 (Part 2) | 71 | 26.9ms | 4 × 4¹⁴ = 1.07B | 6.3× vs P1 |
| 8 (experiment) | 83 | 55.9ms | 4 × 4¹⁶ = 17.2B | 2.1× vs 7 pairs |

Each additional pair on floor 0 adds ~12 steps and roughly doubles the time.
Raw states grow 16× per pair (4²), but canonicalization keeps actual time
scaling to ~2× — the N! symmetry reduction absorbs most of the explosion.
Without canonicalization, 8 pairs (17.2B raw states) would be unsearchable.

### Validity Check Logic
A floor is safe if:
- It has **no generators**, OR
- Every chip on the floor has **its own generator present**

Equivalently: a chip is fried iff `gen_floor ≠ chip_floor` AND
`∃ other_gen on chip_floor`. The code checks this per-pair.

### Move Generation
From the current floor with N items, generate:
- **C(N, 2)** two-item moves (up to 45 for N=10 on a crowded floor)
- **N** single-item moves

Each move is tried in both directions (up/down), filtered by floor bounds (0-3).

---

## Concepts for Study

### State Space Search as Graph Traversal
This problem is equivalent to finding the shortest path in an implicit graph:
- **Nodes** = valid arrangements of items + elevator position
- **Edges** = legal elevator moves (1-2 items to adjacent floor)
- **Goal** = all items on floor 4

The graph is never explicitly constructed — nodes are generated on the fly
during BFS, which is memory-efficient for large state spaces.

### Canonicalization for Symmetry Breaking
The key insight: if you swap all "polonium" items with all "thulium" items,
the resulting state is **functionally identical**. Sorting pairs exploits this
symmetry, reducing the search space by up to N! for N element pairs.

This is the same principle as:
- **Chess endgame tablebases**: Black/white piece symmetry
- **Rubik's cube solvers**: Rotation equivalence classes
- **AUTOSAR**: Component instances with identical interfaces are interchangeable

### BFS vs A* Trade-off
A* with a good heuristic (e.g., "items below floor 4 need at least X moves")
could prune more aggressively. But at 32ms with plain BFS, the added complexity
isn't warranted. The canonicalization alone provides sufficient pruning.

---

## Potential Optimizations (Not Applied)

| Optimization | Expected Impact | Why Not |
|-------------|-----------------|---------|
| A* with admissible heuristic | 2-5× faster | 32ms already fine |
| Bitset state representation | Smaller HashSet entries | Would complicate code |
| FxHashSet | ~20% faster hashing | Marginal at 32ms |
| Bidirectional BFS | ~√N search reduction | Complex; not needed |
| Prune "move 1 down when can move 2" | Fewer states explored | Correctness risk |

At 32ms combined, this is well within the 100ms budget.

---

## Input Analysis

```
Floor 1: polonium-G, thulium-G, thulium-M, promethium-G,
         ruthenium-G, ruthenium-M, cobalt-G, cobalt-M
Floor 2: polonium-M, promethium-M
Floor 3: (empty)
Floor 4: (empty)
```

**Canonical pairs** (sorted):
```
(0, 0)  — cobalt:     gen=F1, chip=F1
(0, 0)  — ruthenium:  gen=F1, chip=F1
(0, 0)  — thulium:    gen=F1, chip=F1
(0, 1)  — polonium:   gen=F1, chip=F2
(0, 1)  — promethium: gen=F1, chip=F2
```

Three paired elements on F1, two with chips separated on F2.
Part 2 adds two more `(0, 0)` pairs, making 7 total.

---

**See also**: [AoC 2016 Summary](../summary_2016.md)
