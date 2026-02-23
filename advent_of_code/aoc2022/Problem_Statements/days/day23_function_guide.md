# Day 23: Unstable Diffusion - Function Guide

**Problem**: Simulate elf spreading on an infinite grid via proposal/collision phases with rotating direction priority.

**Navigation**: [← Day 22](day22_function_guide.md) | [Problem](day23.md) | [Code](../../src/solver/day23.rs) | [Summary](../summary_2022.md) | [Day 24 →](day24_function_guide.md)

---

## Overview

### Problem Summary
- **Part 1**: Simulate 10 rounds of elf movement. Count empty ground tiles in the smallest bounding rectangle containing all elves.
- **Part 2**: Continue simulating until no elf moves. Report that round number.

### Performance
- **Combined**: 104.62ms (~930 rounds × ~2,500 elves)
- **Part 1**: ~1.1ms (first 10 rounds only)
- **Part 2**: ~103ms (remaining 920 rounds to quiescence)

### Key Insight
**The grid is infinite and sparse** — elves occupy ~2,500 of potentially millions of positions. A `FxHashSet<(i32, i32)>` naturally represents this without wasting memory on empty space. Each round has two phases: (1) propose a direction based on 3-neighbor checks with rotating priority, (2) move only if no collision. The simulation is O(R × n) per round but requires ~930 rounds to converge, making it inherently expensive.

---

## Algorithm Analysis

### Parsing: HashSet from Grid

The input is a rectangular grid of `#` (elf) and `.` (empty). Only elf positions are stored:

```rust
fn parse(input: &str) -> FxHashSet<Pos> {
    // For each '#', insert (row, col) into the set
}
```

Input size: ~73 lines × ~73 cols → ~2,500 elves out of ~5,300 total cells.

### Direction Table Design

The core data structure is two const arrays that encode the movement rules compactly:

```
NEIGHBORS[8]: N, NE, E, SE, S, SW, W, NW    (all 8 adjacencies)

DIRECTIONS[4]:                                 (the 4 proposal rules)
  N → move (-1, 0), check indices [0, 1, 7]   (N, NE, NW)
  S → move (+1, 0), check indices [4, 3, 5]   (S, SE, SW)
  W → move (0, -1), check indices [6, 7, 5]   (W, NW, SW)
  E → move (0, +1), check indices [2, 1, 3]   (E, NE, SE)
```

The `DIRECTIONS` array stores indices into `NEIGHBORS` rather than duplicating coordinate offsets — 3 lookups per direction check, avoiding repeated coordinate arithmetic.

### Phase 1: Proposals

For each elf in the set:

1. **Isolation check**: Scan all 8 neighbors. If none occupied → elf stays put (skip proposal entirely)
2. **Direction search**: Try 4 directions in rotated order `(round + i) % 4`. For each, check the 3 relevant neighbors:
   - If all 3 are empty → propose that direction, break
   - If any occupied → try next direction
3. **Record proposal**: `FxHashMap<Pos, Vec<Pos>>` maps destination → list of proposing elves

The isolation check is critical — it's an early-out that skips ~80% of elves in later rounds when most are already spread out.

### Phase 2: Movement

Iterate over the proposals map. For each destination:
- **1 proposer** → remove old position from set, insert new position
- **2+ proposers** → nobody moves (collision)

### Direction Rotation

Round 0: N, S, W, E
Round 1: S, W, E, N
Round 2: W, E, N, S
Round 3: E, N, S, W
Round 4: (same as round 0)

Implemented as `(round + i) % 4` indexing into the `DIRECTIONS` array — zero-cost rotation with no array shifting.

### Worked Example (Small Input)

```
Round 0 (check N first):
  ..##.          ..##.     Northernmost 2 → propose N (no N neighbors)
  ..#..    →     .....     Middle → proposes S, collides with SW elf
  .....          ..#..     SW elf → proposes N, collides with middle
  ..##.          ...#.     SE elf → proposes N (no N neighbors)
  .....          ..#..

Round 1 (check S first):
  ..##.          .....     Both top → propose S
  .....          ..##.     West middle → can't go S, proposes W
  ..#..    →     .#...     East middle → can't go S or W, proposes E
  ...#.          ....#     Bottom → doesn't move (no neighbors)
  ..#..          .....
                 ..#..

Round 2 (check W first):
  .....          ..#..     All 3 with neighbors propose different dirs
  ..##.          ....#     West → proposes W
  .#...    →     #....     East top → proposes E
  ....#          ....#     Middle → proposes N
  .....          .....
  ..#..          ..#..

Round 3: No elf has any neighbors → nobody moves → Part 2 answer = 3
```

### Part 1: Bounding Box

After 10 rounds, find the axis-aligned bounding rectangle:
```
area = (max_row - min_row + 1) × (max_col - min_col + 1)
empty_tiles = area - num_elves
```

### Part 2: Quiescence Detection

The `step()` function returns `bool` — whether any elf moved. Run rounds until it returns `false`. The answer is that round number (1-indexed).

---

## Implementation Details

### Data Flow
```
Input → parse() → FxHashSet<(i32, i32)>
                       ↓
                  clone for Part 1
                       ↓
              step() × 10 rounds → bounding_box_empty() → Part 1
                       ↓
              step() until !moved → round count → Part 2
```

### Function Signatures

| Function | Input | Output | Purpose |
|----------|-------|--------|---------|
| `parse` | `&str` | `FxHashSet<Pos>` | Extract elf positions from grid |
| `step` | `&mut FxHashSet<Pos>, usize` | `bool` | One round: propose + move, return if any moved |
| `bounding_box_empty` | `&FxHashSet<Pos>` | `usize` | Count empty tiles in min bounding rectangle |
| `solve` | `&str` | `(usize, usize)` | Run both parts, return (Part 1, Part 2) |

### Key Design Decisions

**Why FxHashSet instead of a 2D grid?**
Elves spread across an unbounded area — after 930 rounds they can be ~950 positions away from origin in any direction. A dense grid would need ~1900×1900 = 3.6M cells for ~2,500 elves (0.07% occupancy). FxHashSet stores only occupied positions, using ~2,500 entries with O(1) lookup.

**Why `Vec<Pos>` in proposals map instead of counting?**
We need to know *which* elf proposed (to remove its old position), not just the count. Storing the proposing elf directly avoids a second lookup. For sole proposers (the common case), this Vec has exactly 1 element — no heap allocation due to small-vec optimization in practice.

**Why separate Part 1 and Part 2 runs?**
Part 1 needs state after round 10; Part 2 needs to run to convergence. Cloning the initial set and running independently is simpler than checkpointing at round 10 mid-stream. The Part 1 cost (10 rounds, ~1ms) is negligible compared to Part 2 (~103ms).

**Why `(i32, i32)` coordinates?**
Elves can move in negative directions from the initial grid. `i32` range (±2B) is far more than sufficient for ~930 rounds of movement and avoids `usize` underflow concerns.

---

## Performance Analysis

### Benchmark Results
```
Combined: 104.62ms  (930 rounds × ~2,500 elves)
```

The simulation runs 930 full rounds. Each round:
- Iterates all ~2,500 elves for proposals (8 neighbor checks each = ~20,000 hash lookups)
- Builds and iterates the proposals HashMap (~500-1500 entries depending on round)
- Performs HashSet remove/insert for each moving elf

Total: ~930 × ~20,000 = ~18.6M hash lookups dominate runtime.

### Complexity

| Component | Complexity | Notes |
|-----------|------------|-------|
| Parse | O(rows × cols) | Single pass through input grid |
| Per-round proposals | O(n) | n elves × up to 8+12 neighbor checks |
| Per-round movement | O(p) | p proposals, typically p < n |
| Part 1 total | O(10 × n) | 10 rounds, ~1ms |
| Part 2 total | O(R × n) | R ≈ 930 rounds, ~104ms |
| Hash operations | O(1) amortized | FxHash is non-cryptographic, very fast |

### Where Time Is Spent

The bottleneck is hash lookups — each elf checks 8 neighbors for isolation, then up to 3×4=12 more for direction proposals. With ~2,500 elves and ~930 rounds, that's ~18.6M `contains()` calls. FxHashSet is already one of the fastest hash implementations available.

### Potential Optimizations

1. **Reduce allocations**: Reuse the proposals HashMap across rounds with `clear()` instead of rebuilding
2. **Count-based proposals**: Use `FxHashMap<Pos, (Pos, u8)>` storing (first_proposer, count) instead of `Vec<Pos>` to avoid Vec allocations entirely
3. **Spatial partitioning**: Chunk the grid into regions and skip chunks with no neighbors — reduces neighbor checks for sparse areas in later rounds
4. **Early isolation skip**: Track which elves are isolated (no neighbors) between rounds — in later rounds, >80% of elves don't move

---

## Edge Cases

- **No neighbors**: An elf with no adjacent elves does nothing — the 8-neighbor isolation check is an explicit early-out
- **All directions blocked**: If all 4 directional checks fail (all 8 neighbors occupied), the elf stays put
- **Collision cancellation**: Two or more elves proposing the same destination means *none* of them move — they all stay in their original positions
- **Round 1 vs round 0**: Round numbering is 0-indexed internally but Part 2 reports 1-indexed (the first round where nobody moves)
- **Windows line endings**: Input normalized with `.replace("\r\n", "\n")` before parsing

---

## Key Takeaways

1. **FxHashSet for sparse infinite grids** — when occupancy is <1%, a hash set uses far less memory than a dense grid and has the same O(1) lookup
2. **Direction tables with index indirection** — `DIRECTIONS` stores indices into `NEIGHBORS`, avoiding duplicate coordinate constants and making rotation trivial via `(round + i) % 4`
3. **Two-phase simulation pattern** — propose-then-move with collision detection is a common cellular automaton pattern; the HashMap naturally groups proposals by destination
4. **Isolation early-out matters** — checking 8 neighbors first lets ~80% of elves skip the full 4-direction search in later rounds when most are already spread out
5. **Quiescence detection is cheap** — tracking a `moved` boolean during Phase 2 adds zero overhead; no need for expensive state comparison between rounds

---

**Answer**: Part 1: `3780` | Part 2: `930`

**Related patterns**: [[cellular-automaton]] | [[sparse-grid-simulation]] | [[proposal-collision]]
