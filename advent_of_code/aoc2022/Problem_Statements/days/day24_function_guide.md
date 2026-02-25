# Day 24: Blizzard Basin - Function Guide

**Problem**: Navigate through a valley of cyclically moving blizzards using BFS with time-periodic state.

**Navigation**: [← Day 23](day23_function_guide.md) | [Problem](day24.md) | [Code](../../src/solver/day24.rs) | [Summary](../summary_2022.md) | [Day 25 →](day25_function_guide.md)

---

## Overview

### Problem Summary
- **Part 1**: Find the fewest minutes to traverse the valley from start (top opening) to goal (bottom opening), avoiding all blizzards.
- **Part 2**: Traverse start→goal→start→goal. Report the total time for all three legs.

### Performance
- **Baseline**: 95.9ms (combined solve)
- **Part 1**: ~32ms (start→goal BFS)
- **Part 2**: ~96ms (three BFS legs total)

### Key Insight
**Blizzard positions are periodic.** Each blizzard wraps around the inner valley with period equal to the dimension it moves along (inner_h for vertical, inner_w for horizontal). The full state repeats every `LCM(inner_h, inner_w)` minutes. By precomputing blocked positions for each time step in the cycle, the problem reduces to a standard BFS where state = `(row, col, time mod period)`. This bounds the state space to `rows × cols × period` — finite and tractable.

---

## Algorithm Analysis

### Blizzard Periodicity

The valley has walls on all four sides. Blizzards move within the inner region (excluding walls):
- Inner height: `rows - 2`
- Inner width: `cols - 2`

A blizzard moving vertically wraps every `inner_h` steps. Horizontally, every `inner_w` steps. All blizzard configurations repeat every `LCM(inner_h, inner_w)` minutes.

For the actual input (27 rows × 122 cols): `LCM(25, 120) = 600` — only 600 unique time states.

### Blizzard Position Formula

Given a blizzard starting at `(r, c)` with direction `dir`, its position at time `t`:

```
Up (^):    row = ((r-1) + inner_h - (t % inner_h)) % inner_h + 1,  col = c
Down (v):  row = ((r-1) + t) % inner_h + 1,                        col = c
Left (<):  row = r,  col = ((c-1) + inner_w - (t % inner_w)) % inner_w + 1
Right (>): row = r,  col = ((c-1) + t) % inner_w + 1
```

The `- 1` / `+ 1` adjustments convert between 1-indexed grid coordinates and 0-indexed modular arithmetic. The subtraction pattern for `^` and `<` uses `+ inner_h - (t % inner_h)` instead of `- t` to avoid unsigned underflow.

### Precomputed Blocked Sets

Rather than recomputing blizzard positions during BFS, we precompute a `Vec<FxHashSet<Pos>>` of length `period`:

```rust
blocked[t] = { all (row, col) occupied by any blizzard at time t }
```

This trades ~600 HashSet allocations (one-time parse cost) for O(1) blizzard collision checks during BFS. With ~19 blizzards × 600 time steps = ~11,400 insertions total, this is fast.

### BFS with Time Dimension

State: `(row, col, time)`. Two states are equivalent if `(row, col, time % period)` match — revisiting the same position at the same phase of the blizzard cycle can't improve the result.

```
Queue: VecDeque<(row, col, absolute_time)>
Visited: FxHashSet<(row, col, time_mod_period)>
```

At each step, try 5 moves: wait, up, down, left, right. A move is valid if:
1. Position is within bounds (not out of grid)
2. Position is start, goal, or an inner cell (not a wall)
3. Position is not blocked by a blizzard at `time + 1`

BFS guarantees the first time we reach the goal is the shortest path.

### Three-Leg Part 2

Part 2 chains three BFS calls, each starting at the time the previous leg finished:

```rust
let t1 = bfs(start, goal, 0);       // Leg 1: start → goal
let t2 = bfs(goal, start, t1);      // Leg 2: goal → start (blizzards continue!)
let t3 = bfs(start, goal, t2);      // Leg 3: start → goal again
```

The `start_time` parameter is critical — blizzards don't reset between legs. The BFS picks up from the current blizzard phase.

### Worked Example

```
#.######      Inner: 4 rows × 6 cols
#>>.<^<#      Period: LCM(4, 6) = 12
#.<..<<#      Blizzards: 19 total
#>v.><>#      Start: (0, 1)
#<^v^^>#      Goal: (5, 6)
######.#

BFS from (0,1) at t=0:
  t=0: Start at (0,1)
  t=1: Move down to (1,1) — blizzards shifted, (1,1) is clear
  t=2: Move down to (2,1) — still avoiding blizzards
  ...navigating through gaps in blizzard patterns...
  t=18: Reach (5,6) = goal → Part 1 = 18

BFS from (5,6) at t=18:
  t=18..41: Navigate back through shifting blizzards
  t=41: Reach (0,1) = start

BFS from (0,1) at t=41:
  t=41..54: Navigate to goal again
  t=54: Reach (5,6) → Part 2 = 54
```

### Start/Goal as Safe Havens

The start `(0, 1)` and goal `(rows-1, goal_col)` positions are in the wall rows — blizzards never enter these cells. The player can wait at start or goal indefinitely, which is important for Part 2 (waiting at goal for blizzards to clear before heading back).

---

## Implementation Details

### Data Flow
```
Input → parse() → Valley { blocked[0..period], start, goal }
                      ↓
              bfs(start, goal, 0) → t1 (Part 1)
                      ↓
              bfs(goal, start, t1) → t2
                      ↓
              bfs(start, goal, t2) → t3 (Part 2)
```

### Function Signatures

| Function | Input | Output | Purpose |
|----------|-------|--------|---------|
| `gcd` | `usize, usize` | `usize` | Greatest common divisor (Euclidean) |
| `lcm` | `usize, usize` | `usize` | Least common multiple via GCD |
| `parse` | `&str` | `Valley` | Extract blizzards, precompute blocked sets |
| `bfs` | `&Valley, start, goal, start_time` | `usize` | Shortest path avoiding blizzards, returns arrival time |
| `solve` | `&str` | `(usize, usize)` | Three-leg BFS for both parts |

### Key Types

```rust
type Pos = (usize, usize);        // (row, col)

struct Valley {
    rows: usize,                   // Total grid height (including walls)
    cols: usize,                   // Total grid width (including walls)
    blocked: Vec<FxHashSet<Pos>>,  // blocked[t] = positions with blizzards at time t
    period: usize,                 // LCM(inner_h, inner_w)
    start: Pos,                    // (0, start_col)
    goal: Pos,                     // (rows-1, goal_col)
}
```

### Key Design Decisions

**Why precompute all blocked sets instead of computing on-the-fly?**
The BFS visits the same time step from many different positions. Precomputing means each blizzard position is calculated once total, not once per BFS node. With ~600 time steps × ~19 blizzards = ~11,400 computations vs potentially millions of on-the-fly checks.

**Why `(usize, usize)` tuples instead of packed `i64` (like Day 23)?**
Day 23 had ~18.6M hash lookups where packed coordinates measurably helped. Day 24's BFS has far fewer lookups — the bottleneck is the state space exploration, not individual hash operations. Tuples are simpler and sufficient.

**Why `FxHashSet` for visited instead of a 3D array?**
A 3D array `[rows][cols][period]` = 27 × 122 × 600 = ~1.97M entries would work but wastes memory on unreachable states. The BFS only visits a fraction of the full state space. FxHashSet adapts to actual reachability.

**Why check goal before validating inner cell?**
Early return optimization — if we've reached the goal, we don't need to check walls or blizzards. The goal is checked before the more expensive wall/blizzard tests.

**Why does the BFS queue store absolute time but visited uses modular time?**
The queue needs absolute time to compute blizzard state (`t + 1`) and return the final answer. But for deduplication, only `time % period` matters — same position at same blizzard phase is equivalent.

---

## Complexity Analysis

| Component | Complexity | Notes |
|-----------|------------|-------|
| Parse + precompute | O(period × B) | B blizzards, period = LCM(h, w), ~600 × 19 |
| BFS state space | O(R × C × P) | R=rows, C=cols, P=period, upper bound ~1.97M |
| BFS per state | O(5) | 5 moves: wait + 4 directions |
| BFS visited check | O(1) amortized | FxHashSet lookup |
| Blizzard check | O(1) amortized | Precomputed FxHashSet lookup |
| Total Part 1 | O(R × C × P) | Single BFS, ~95.9ms total for 3 legs |
| Total Part 2 | O(3 × R × C × P) | Three BFS calls |

### Where Time Is Spent

The BFS explores a large state space: up to `27 × 122 × 600 ≈ 2M` states across three legs. Each state requires:
- 5 candidate moves with bounds checking
- HashSet lookups for blizzard collision and visited deduplication
- VecDeque push/pop operations

The precomputation phase is fast (~11,400 HashSet insertions). The BFS dominates runtime.

---

## Edge Cases

- **Waiting at start**: Player can wait at `(0, 1)` indefinitely — blizzards never reach wall rows
- **Waiting at goal**: Same for goal position — safe haven between Part 2 legs
- **Blizzard overlap**: Multiple blizzards can share a cell — HashSet naturally deduplicates
- **Start column**: Found by scanning top row for `.` — not hardcoded
- **Goal column**: Found by scanning bottom row for `.` — not hardcoded
- **Windows line endings**: Normalized with `.replace("\r\n", "\n")` before parsing
- **Unsigned underflow in blizzard math**: `+ inner_h - (t % inner_h)` avoids subtracting from small unsigned values

---

## Key Takeaways

1. **Periodic state spaces enable BFS** — blizzards cycle with period LCM(h, w), collapsing infinite time into a finite state space. The key insight is recognizing that `(pos, time mod period)` fully characterizes the reachable state.
2. **Precompute time-varying obstacles** — rather than computing blizzard positions per BFS node, precompute `blocked[t]` for all `t` in the cycle. Amortizes the cost across millions of collision checks.
3. **Chain BFS calls for multi-leg paths** — Part 2 is three independent BFS problems connected by their start times. Blizzard state carries over via the `start_time` parameter.
4. **Start/goal as safe havens** — positions in the wall rows are never blocked by blizzards, allowing the player to wait indefinitely. Critical for Part 2's return trip.
5. **State deduplication with modular time** — visited set uses `time % period` instead of absolute time, preventing infinite exploration while correctly identifying equivalent states.

---

**Answer**: Part 1: `242` | Part 2: `720`

**Related patterns**: [[bfs-shortest-path]] | [[periodic-state-space]] | [[time-varying-obstacles]] | [[multi-leg-pathfinding]]
