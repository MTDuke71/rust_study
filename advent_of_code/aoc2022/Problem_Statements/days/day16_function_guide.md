# Day 16: Proboscidea Volcanium - Function Guide

**Problem**: Maximize pressure released by opening valves in a cave network within a time limit.

**Navigation**: [← Day 15](day15_function_guide.md) | [Problem](day16.md) | [Code](../../src/solver/day16.rs) | [Summary](../summary_2022.md) | [Day 17 →](day17_function_guide.md)

---

## Overview

### Problem Summary
- **Part 1**: Starting at valve AA, maximize total pressure released by opening valves within 30 minutes
- **Part 2**: You and an elephant both start at AA with 26 minutes each, working independently on disjoint valve sets

### Performance
- **Part 1**: 3.79ms (DFS with 30 minutes, larger search tree)
- **Part 2**: 1.18ms (DFS with 26 minutes + SOS DP + mask scan)
- **Combined**: 4.94ms ✅ Parse-once verified (4.94 < 3.79 + 1.18)

### Key Insight
**This looks like Dijkstra but isn't**:

| | Dijkstra | Day 16 |
|--|----------|--------|
| **Goal** | Minimize cost to reach destination | Maximize total flow over time |
| **State** | (node, cost) | (node, opened_valves, time_remaining) |
| **Decisions** | Which neighbor is cheapest? | Which valve should I open *next*? |

Opening a flow=24 valve with 20 minutes left = 480 pressure. Same valve with 5 minutes left = only 120. **When** you open matters as much as **what** you open. This is closer to the Traveling Salesman Problem.

---

## Algorithm Analysis

### Step 1: Graph Compression

**The problem**: 55 valves, but only 15 have non-zero flow rates. The other 40 are just hallways.

**Solution**: Compress the graph to only important nodes by precomputing BFS shortest distances.

```
Before (55 nodes, sparse connections):
AA ─── DD ─── CC ─── BB
 │            │
 II           EE ─── FF ─── GG ─── HH
  │
  JJ

After (7 key nodes, complete graph with distances):
     AA
    / | \
   1  1  1      ← distances from AA to each
  DD  II  BB
  ...etc...
```

**Implementation**:
```rust
// Identify important valves (non-zero flow)
let important: Vec<usize> = valves.iter()
    .enumerate()
    .filter(|(_, v)| v.flow_rate > 0)
    .map(|(i, _)| i)
    .collect();

// key_nodes = [AA] ++ important valves
// BFS from each key node to every other valve
let key_nodes: Vec<usize> = std::iter::once(start)
    .chain(important.iter().copied())
    .collect();

// distances[i][j] = shortest path between key_nodes[i] and key_nodes[j]
for (ki, &from) in key_nodes.iter().enumerate() {
    let dist = bfs_distances(&valves, from);
    for (kj, &to) in key_nodes.iter().enumerate() {
        distances[ki][kj] = dist[to];
    }
}
```

**Result**: 55-node sparse graph → 16-node complete graph with precomputed distances.

---

### Step 2: Part 1 — DFS with Bitmask

**State**: `(position, opened_bitmask, time_remaining, pressure_accumulated)`

**Branching**: At each position, try opening each unopened valve that's reachable in remaining time.

**Pressure calculation**: `flow_rate × time_remaining_after_opening`

```rust
fn dfs(data, pos, opened, time, pressure, n, best) {
    *best = (*best).max(pressure);  // Update best at every node

    for i in 0..n {
        if opened & (1 << i) != 0 { continue; }  // Already opened

        let cost = data.distances[pos][i + 1] + 1;  // Travel + open
        if cost >= time { continue; }               // Not enough time

        let remaining = time - cost;
        let new_pressure = pressure + data.flows[i] * remaining;

        dfs(data, i + 1, opened | (1 << i), remaining, new_pressure, n, best);
    }
}
```

**Why bitmask works**: 15 valves → 15 bits → fits in a `u32`. Bit `i` = 1 means important valve `i` is open.

**Search space**: Worst case is 15! ≈ 1.3 trillion orderings, but time budget prunes aggressively. Most branches die early when `cost >= time`.

---

### Step 3: Part 2 — Bitmask Partition + SOS DP

**Key insight**: You and the elephant work independently and open **disjoint** valve sets. Instead of simulating both simultaneously (massive state space), we:

1. **Run DFS once** (26 minutes), recording `best_pressure[mask]` for every opened bitmask
2. **SOS DP** to propagate: `best[m]` = best achievable using any subset of `m`
3. **Scan all masks**: Answer = `max(best[m] + best[complement(m)])`

#### Phase 1: Collect best per mask
```rust
// dfs_collect: same as dfs, but records best_for_mask[opened] = max(pressure)
let mut best_for_mask = vec![0u32; 1 << n];  // 2^15 = 32,768 entries
dfs_collect(data, 0, 0, 26, 0, n, &mut best_for_mask);
```

#### Phase 2: Sum over Subsets DP
```rust
// After this, best_for_mask[m] = max over ALL subsets of m
for i in 0..n {
    for mask in 0..size {
        if mask & (1 << i) != 0 {
            best_for_mask[mask] = best_for_mask[mask]
                .max(best_for_mask[mask ^ (1 << i)]);
        }
    }
}
```

**What SOS DP does**: For each mask `m`, it answers "what's the best pressure achievable by opening any subset of these valves?" This is needed because the elephant doesn't have to open ALL remaining valves — just the best subset it can reach in 26 minutes.

**Without SOS DP**: Would need to iterate all submask pairs = O(3^n) ≈ 14M operations.

**With SOS DP**: O(n × 2^n) propagation + O(2^n) scan = ~500K operations.

#### Phase 3: Find best disjoint pair
```rust
let full = (1 << n) - 1;
for m in 0..size {
    best = best.max(best_for_mask[m] + best_for_mask[full ^ m]);
}
```

**Why this works**: `m` is your valve set, `full ^ m` is the complement (elephant's available valves). Since SOS DP propagated, `best_for_mask[full ^ m]` already represents the best the elephant can do with any subset of those valves.

---

## Implementation Details

### Parsing Strategy

**Two-pass approach**:

1. **First pass**: Assign numeric indices to valve names (AA → 0, BB → 1, etc.)
2. **Second pass**: Parse flow rates and tunnel connections using indices

```rust
// Handle both "tunnels lead to valves AA, BB" and "tunnel leads to valve AA"
let tunnel_str = if parts[1].contains("valves") {
    parts[1].split("valves ").nth(1).unwrap()
} else {
    parts[1].split("valve ").nth(1).unwrap()
};
```

**Gotcha**: Some valves have only one tunnel ("tunnel leads to valve" — singular).

### BFS Distance Computation

Standard single-source BFS returning distance to every node:

```rust
fn bfs_distances(valves: &[Valve], start: usize) -> Vec<u32> {
    let mut dist = vec![u32::MAX; valves.len()];
    dist[start] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        for &next in &valves[node].tunnels {
            if dist[next] == u32::MAX {
                dist[next] = dist[node] + 1;
                queue.push_back(next);
            }
        }
    }
    dist
}
```

Called 16 times (once per key node). Each BFS is O(V + E) where V=55, E≈110.

### Key Node Indexing Convention

```
key_nodes[0] = AA (start position)
key_nodes[1] = important[0]  (first non-zero valve)
key_nodes[2] = important[1]  (second non-zero valve)
...
key_nodes[n] = important[n-1] (last non-zero valve)

distances[pos][i+1] = distance from key_nodes[pos] to important valve i
```

The `+1` offset accounts for AA being at index 0.

---

## Performance Analysis

### Why Part 2 is Faster Than Part 1

Counterintuitive result: Part 2 (1.18ms) < Part 1 (3.79ms), despite Part 2 doing more work (SOS DP + mask scan).

**Explanation**: The DFS dominates runtime, and Part 2 has **26 minutes** vs Part 1's **30 minutes**. Fewer minutes = more aggressive pruning = much smaller search tree. The SOS DP + mask scan is only O(n × 2^15 + 2^15) ≈ 500K operations, negligible compared to DFS.

### Parse-Once Verification

```
day16_part1:    3.79ms  ← DFS with 30 min
day16_part2:    1.18ms  ← DFS with 26 min + SOS DP
day16_combined: 4.94ms  ← Parse once + both parts
```

Verification: 4.94ms < 3.79ms + 1.18ms = 4.97ms ✅ (parse shared)

### Complexity Summary

| Component | Complexity | Actual Time |
|-----------|-----------|-------------|
| Parsing | O(V) — two passes over input | ~30µs |
| BFS distances | O(K × (V + E)) — 16 BFS calls | ~10µs |
| Part 1 DFS | O(n! / (n-k)!) pruned by time | ~3.79ms |
| Part 2 DFS | O(n! / (n-k)!) pruned (26 min) | ~1.15ms |
| SOS DP | O(n × 2^n) = 15 × 32768 | ~5µs |
| Mask scan | O(2^n) = 32768 | ~1µs |

---

## Why NOT Dijkstra?

This deserves emphasis because the problem *looks* like shortest path:

1. **No single destination**: Dijkstra answers "cheapest path to X." Here there's no target — you're choosing an *ordering* of multiple stops.

2. **Value depends on time**: Each valve's contribution = `flow_rate × remaining_time`. The same valve has different values depending on when you arrive.

3. **Multiple visits needed**: You must visit multiple valves (up to 15), not just find one shortest path.

4. **Maximization, not minimization**: Even if you found shortest paths to all valves, you still need to decide the *order* to visit them.

**Closest analogy**: Traveling Salesman Problem with time-dependent rewards and a budget constraint.

---

## Edge Cases

### Valves with Flow Rate 0
40 of 55 valves are "hallways" — never worth opening, only traversing. Graph compression eliminates them entirely.

### Unreachable Valves in Time Budget
The `if cost >= time { continue; }` check naturally handles this — valves too far away are simply skipped.

### Part 2: Elephant Takes No Valves
It's valid for one actor to open all profitable valves while the other opens none. The mask scan handles this because `best_for_mask[0]` = 0 (opening nothing yields 0 pressure).

---

## Key Takeaways

1. **Graph compression eliminates noise**: 55 nodes → 16 nodes by precomputing BFS distances. Only important nodes participate in the search.

2. **Bitmask DFS for ordering problems**: When you need to try orderings of ~15 items with pruning, bitmask + DFS is the sweet spot (too big for brute force, small enough for exponential search with pruning).

3. **Bitmask partition for two actors**: Instead of simulating both simultaneously, solve independently and find best disjoint pair. Reduces state space dramatically.

4. **SOS DP (Sum over Subsets)**: Propagates "best over any subset" in O(n × 2^n), enabling O(2^n) partition lookup instead of O(3^n) submask enumeration.

5. **Time budget as natural pruning**: The 30/26 minute limit eliminates most of the exponential search tree — most valve orderings run out of time.

6. **Not every graph problem is shortest path**: Maximizing cumulative reward over time with ordering decisions is closer to TSP than Dijkstra.

---

**Answer**: Part 1: `1638` | Part 2: `2400`

**Related patterns**: [[graph-compression]], [[bitmask-dp]], [[sos-dp]], [[state-space-search]], [[parse-once-pattern]]
