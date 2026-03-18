# Day 17: Two Steps Forward — Function Guide

## Problem Summary

Navigate a 4×4 grid from (0,0) to (3,3) where door states are determined by the MD5 hash of `passcode + path_taken`. The first four hex characters of the hash control Up/Down/Left/Right doors: `b-f` = open, `0-a` = closed.

**Part 1**: Find the shortest path (as a direction string)
**Part 2**: Find the length of the longest path

---

## Answers

| Part | Answer | Time |
|------|--------|------|
| 1 | `DUDRLRRDDR` | 13.2µs |
| 2 | `788` | 22.9ms |
| Combined | — | 22.9ms |

---

## Function Map

```
solve(input) → (String, String)
  ├── parse_input(input) → &str              # Just trim the passcode
  ├── solve_part1_with_data(passcode)         # BFS — shortest path
  │   └── BFS loop:
  │       └── open_doors(passcode, path)      # MD5 hash → 4 door states
  └── solve_part2_with_data(passcode)         # BFS — longest path
      └── BFS loop (exhaustive):
          └── open_doors(passcode, path)
```

---

## Algorithm Details

### Why No Visited Set?

In a normal BFS, you track visited positions to avoid cycles. Here, **every state is unique** because the hash depends on the full path history. Visiting room (1,1) via "DDRU" produces completely different door states than visiting (1,1) via "RDLU". The state space is (position × path), and paths never repeat, so no deduplication is possible.

This means the search tree can be very large — Part 2 explores paths up to length 788.

### Part 1: BFS Shortest Path

Standard BFS guarantees shortest path. States are `(x, y, path_string)`. When we reach (3,3), that's the answer — BFS finds it first means it's shortest.

For our input, the shortest path is just 10 steps: `DUDRLRRDDR` (found in 13µs, very few nodes explored).

### Part 2: BFS Longest Path

Same BFS, but **don't stop at the first solution**. When reaching (3,3), record the path length and `continue` (don't enqueue further moves from the vault). Keep going until the queue is empty.

This is expensive because it must explore ALL valid paths. The longest path (788 steps) means the search tree has paths that wander the grid extensively before reaching the vault.

### MD5 Door State Computation

```
open_doors("lpvhkcbi", "DU") →
  1. MD5("lpvhkcbiDU") → hex string
  2. First 4 hex chars: e.g., "f2bc"
  3. f≥b → Up open,   2<b → Down closed
     b≥b → Left open, c≥b → Right open
  4. Returns [true, false, true, true]
```

The hash is computed fresh for every position in the BFS — no caching possible since every path is unique.

---

## Key Observations

1. **No visited set**: Path-dependent doors make every state unique. This is fundamentally different from Day 13's implicit graph BFS which could cache visited nodes.

2. **Part 2 dominates**: Part 1 finds the answer in ~10 BFS steps (13µs). Part 2 must exhaust all paths — the search tree explodes combinatorially.

3. **String cloning overhead**: Each BFS state carries a full path string. For Part 2 with paths up to 788 chars, this is significant allocation. A possible optimization would be path compression, but 23ms is acceptable.

4. **MD5 is the bottleneck**: Each BFS node requires one MD5 computation. Part 2 explores thousands of nodes, each needing a hash of `passcode + path` (up to 796 bytes).

5. **No mission integration**: Pure BFS with MD5 — no grid/graph abstractions needed since the 4×4 grid is trivially small and the complexity is in the hash-based door logic.

---

## Benchmarks

```
day17/combined          time:   [22.9 ms]
day17/part1             time:   [13.2 µs]
day17/part2             time:   [22.9 ms]
```

Part 1 is essentially instant. Part 2 dominates — exhaustive BFS exploring all paths up to length 788.

---

## Potential Optimizations (Not Needed)

- **Pre-allocate path strings**: Use a shared buffer instead of cloning strings for each BFS state
- **DFS for Part 2**: Could use less memory than BFS (only one active path instead of queue of all frontier paths), though BFS naturally finds shortest for Part 1
- **Nibble extraction without hex chars**: Skip the hex character conversion, compare nibbles directly (minor savings per hash)

None worthwhile at 23ms — well under the 100ms target.
