# Day 19: Not Enough Minerals - Function Guide

**Problem**: Maximize geode production by choosing which resource-collecting robots to build.

**Navigation**: [← Day 18](day18_function_guide.md) | [Problem](day19.md) | [Code](../../src/solver/day19.rs) | [Summary](../summary_2022.md) | [Day 20 →](day20_function_guide.md)

---

## Overview

### Problem Summary
- **Part 1**: 24 minutes, 30 blueprints — sum of (id × max geodes) = quality level
- **Part 2**: 32 minutes, first 3 blueprints only — product of max geodes

### Performance
- **Parse**: 7.48µs
- **Part 1**: 1.92ms (30 blueprints × 24 min DFS)
- **Part 2**: 1.56ms (3 blueprints × 32 min DFS)
- **Combined**: 3.50ms ✅ Parse-once verified

### Key Insight
**Skip-to-build eliminates "do nothing" turns.** Instead of deciding each minute whether to build or wait, jump directly to the time when enough resources accumulate for each robot type. This collapses the branching factor from 5 choices/minute (build one of 4 robots or wait) to at most 4 choices per state (which robot to build next).

---

## Algorithm Analysis

### Resource Chain

```
Ore → Ore Robot (self-sustaining)
Ore → Clay Robot → Clay
Ore + Clay → Obsidian Robot → Obsidian
Ore + Obsidian → Geode Robot → Geodes ← maximize this
```

Each robot type requires resources from earlier in the chain. The search explores the ORDER in which to build robots to maximize geode output.

### DFS with Branch-and-Bound

The core search is an explicit stack-based DFS. Each state contains:

```
SearchState = (time_left, ore, clay, obsidian, geodes, ore_bots, clay_bots, obs_bots, geo_bots)
```

For each state, try building each robot type by computing how long to wait:

```rust
// Example: building a geode robot
let wait_ore = if ore >= bp.geode_robot_ore {
    0
} else {
    (bp.geode_robot_ore - ore).div_ceil(r_ore)
};
let wait_obs = if obsidian >= bp.geode_robot_obsidian {
    0
} else {
    (bp.geode_robot_obsidian - obsidian).div_ceil(r_obs)
};
let wait = wait_ore.max(wait_obs) + 1;  // +1 for build time
```

**Why `.div_ceil()`?** If you need 7 ore and produce 3/min, you need ceil(7/3) = 3 minutes of waiting. The `+1` accounts for the build minute itself.

### Three Pruning Strategies

#### 1. Robot Caps (Don't overbuild)
```rust
let max_ore_cost = bp.ore_robot_ore
    .max(bp.clay_robot_ore)
    .max(bp.obsidian_robot_ore)
    .max(bp.geode_robot_ore);
```
Never build more ore robots than the max ore cost per minute — you'd produce ore faster than you could spend it. Same logic for clay and obsidian robots.

#### 2. Upper Bound (Triangle number)
```rust
let upper = geodes + r_geo * time + time * (time - 1) / 2;
if upper <= best { continue; }
```
Even if we built a geode robot EVERY remaining minute: current geodes + current production + new robots each contributing decreasing amounts = `time*(time-1)/2`. If this optimistic upper bound can't beat our best, prune.

#### 3. Skip-to-Build (Eliminate idle turns)
Instead of branching on "build or wait" each minute, compute exactly when we CAN build each robot, then jump to that future state. This eliminates the exponential blowup of idle turns.

```
Traditional:     Min 1 → Min 2 → Min 3 → ... → Build
Skip-to-build:   Current state → Build (after computed wait)
```

### Why No Memoization?

Unlike many optimization problems, the state space here is too large for practical memoization. With 9 dimensions (time + 4 resources + 4 robot counts), the number of unique states is enormous. The branch-and-bound pruning is effective enough — the search completes in ~1-2ms per blueprint.

---

## Implementation Details

### Data Flow

```
Input: "Blueprint 1: Each ore robot costs 4 ore..."
  ↓ parse_input()
Vec<Blueprint>     ← 30 blueprints, 7 cost values each
  ↓ solve_part1_with_data()
For each blueprint: max_geodes(bp, 24) → quality = id × geodes → sum
  ↓ solve_part2_with_data()
First 3 blueprints: max_geodes(bp, 32) → product of geodes
```

### Parsing Strategy

Extract all numbers from each line, ignoring non-digit characters:

```rust
line.split(|c: char| !c.is_ascii_digit())
    .filter(|s| !s.is_empty())
    .map(|s| s.parse().unwrap())
```

This handles any text formatting — just grab the 7 numbers in order (id, ore_ore, clay_ore, obs_ore, obs_clay, geo_ore, geo_obs).

### Type Alias for Clippy

The 9-tuple state triggered `clippy::type_complexity`. Extracted as:

```rust
type SearchState = (u32, u32, u32, u32, u32, u32, u32, u32, u32);
```

Same pattern as Day 17's `CycleKey`/`CycleVal` type aliases.

---

## Performance Analysis

### Benchmark Results

```
day19_parse:     7.48µs    ← 30 lines, extract 7 numbers each
day19_part1:     1.92ms    ← 30 blueprints × DFS(24 min)
day19_part2:     1.56ms    ← 3 blueprints × DFS(32 min)
day19_combined:  3.50ms    ← Parse once + both parts
```

### Part 2 Faster Than Part 1?

Despite 32 minutes vs 24, Part 2 is faster because it only processes 3 blueprints vs 30. Per-blueprint, the 32-minute search takes ~520µs vs ~64µs for 24 minutes — roughly 8× slower per blueprint, but 10× fewer blueprints.

### Complexity

| Component | Complexity | Notes |
|-----------|-----------|-------|
| Parsing | O(n) | n = 30 blueprints |
| Search per blueprint | O(B^d) | B = branching factor (~4), d = effective depth, heavily pruned |
| Part 1 total | O(30 × search) | ~1.92ms |
| Part 2 total | O(3 × search) | ~1.56ms, deeper search but fewer blueprints |
| Space | O(d) | Stack depth proportional to time limit |

The effective branching factor after pruning is much less than 4 — the upper bound and robot cap pruning eliminate most branches early.

### Potential Optimizations (Not Implemented)

- **Prioritize geode robots**: Always try building geode robots first — if one can be built in time, it dominates other options. Could reduce branching further.
- **DFS ordering**: Try most promising branches first to find better lower bounds earlier, improving pruning.
- **Parallel per-blueprint**: Each blueprint is independent — Rayon `par_iter()` could parallelize like Day 16's Part 2. But at 3.50ms total, not worth the complexity.

---

## Edge Cases

### Blueprint Dependencies
- Can't build obsidian robots without clay robots (need clay as input)
- Can't build geode robots without obsidian robots (need obsidian as input)
- The `if r_obs > 0` and `if r_clay > 0` guards handle this naturally

### Time Boundary
- `wait < time` ensures we don't try to build a robot that would finish after the timer
- Remaining resources still produce for the final minutes — `geodes + r_geo * time` accounts for this

### Single Blueprint (Part 2 edge case)
If the input had fewer than 3 blueprints, `.take(3)` handles this gracefully — it takes however many are available.

---

## Key Takeaways

1. **Skip-to-build collapses exponential branching**: Instead of 5^24 = 59 trillion states, we explore only ~thousands of meaningful build orderings per blueprint.

2. **Upper bound pruning with triangle numbers**: The "build a geode robot every minute" upper bound is tight enough to prune most of the search tree. `n*(n-1)/2` is the arithmetic series — each new robot contributes one fewer minute of production.

3. **Robot caps prevent waste**: Never build more robots of a type than you can spend per minute. This is the "never stockpile faster than you consume" principle.

4. **`.div_ceil()` for resource wait times**: Clean, idiomatic Rust for "how many minutes until I can afford this?" — replaces the manual `(a + b - 1) / b` pattern.

5. **Type aliases satisfy clippy**: Complex tuple types extracted as `type SearchState = (...)` — same pattern used for Day 17's cycle detection types.

---

**Answer**: Part 1: `1413` | Part 2: `21080`

**Related patterns**: [[branch-and-bound]], [[dfs-search]], [[resource-optimization]], [[pruning-strategies]]
