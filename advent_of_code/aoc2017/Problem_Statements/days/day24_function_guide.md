# Day 24: Electromagnetic Moat — Function Guide

**Problem**: Build a bridge out of magnetic components `a/b`. Adjacent components must share a port type, the first port must be `0` (the metallic shore), each component is used at most once, and either end of a component can face either side. Part 1 wants the strongest bridge possible (`Σ port`); Part 2 wants the strongest among the longest bridges.
**Answers**: Part 1 = **1,511**, Part 2 = **1,471**
**Code**: [day24.rs](../../src/solver/day24.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Modelling Components as a Port-Indexed Graph](#modelling-components-as-a-port-indexed-graph)
3. [The DFS](#the-dfs)
4. [One Search, Both Answers](#one-search-both-answers)
5. [Bitmask Trick: 56 Components Fit in a `u64`](#bitmask-trick-56-components-fit-in-a-u64)
6. [Self-Loops `p/p` and Why They're Stored Once](#self-loops-pp-and-why-theyre-stored-once)
7. [Benchmarks](#benchmarks)
8. [Why No Pruning](#why-no-pruning)
9. [Key Patterns](#key-patterns)
10. [Integrator Notes](#integrator-notes)

---

## Problem Summary

Each line of the input is a component with two port types separated by `/`:

```
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10
```

A bridge is a sequence `c₁ — c₂ — … — cₖ` where:

- the first port of `c₁` is `0` (matching the shore),
- the matching port of every `cᵢ` is the leftover port of `cᵢ₋₁`,
- no component appears twice.

The strength of the bridge is `Σ (port_a + port_b)` over its components. Both parts ask for an extremum over all possible bridges — Part 1 the strongest bridge, Part 2 the strongest among the longest. Brute-force enumeration with backtracking is the natural fit because the search tree is small (the real input has 56 components, branching factor of ~3–4).

---

## Modelling Components as a Port-Indexed Graph

The naïve representation — a `Vec<(u8, u8)>` of components plus a linear scan to find candidates each step — works, but pays O(n) per recursion frame to re-discover what's reachable. Instead, build an adjacency list keyed by **port value**:

```rust
#[derive(Debug, Clone)]
struct Components {
    /// adj[port] = list of (component_idx, other_port)
    adj: Vec<Vec<(u8, u8)>>,
}
```

For each parsed component `(a, b)`:
- push `(idx, b)` into `adj[a]` — "from port `a`, this component takes you to port `b`"
- if `a != b`, push `(idx, a)` into `adj[b]` — the component is reversible
- if `a == b`, push only once: a self-loop is the same component in both orientations

Now from the open port `p`, every candidate is exactly `comps.adj[p]` — no scan, no filter, just iterate the right slot.

Max port value in the real input is **50**, max component count is **56**. Both fit easily in `u8` and a `u64` bitmask respectively.

---

## The DFS

```rust
fn dfs(
    comps: &Components,
    port: u8,
    used: u64,
    strength: u32,
    length: u32,
    best_strength: &mut u32,
    best_long: &mut (u32, u32),
) {
    if strength > *best_strength {
        *best_strength = strength;
    }
    if length > best_long.0 || (length == best_long.0 && strength > best_long.1) {
        *best_long = (length, strength);
    }

    for &(idx, other) in &comps.adj[port as usize] {
        let bit = 1u64 << idx;
        if used & bit != 0 { continue; }
        dfs(
            comps,
            other,
            used | bit,
            strength + port as u32 + other as u32,
            length + 1,
            best_strength,
            best_long,
        );
    }
}
```

Three things to notice:

1. **The update happens on entry, not on a "leaf".** Every node in the search tree is itself a valid bridge — there is no "I have to extend further" rule. The strongest/longest bridge could end anywhere a component runs out, so we update the bests on every call.
2. **`used` is passed by value.** A `u64` is a register; `used | bit` is a single OR, no allocation. There's no explicit "unmark" step — the recursion's natural unwind restores the caller's `used` because we never mutated it.
3. **The lexicographic update for Part 2.** `(length, strength)` is compared length-first, ties broken by strength. This is just a tuple comparison written out explicitly; Rust's derived `PartialOrd` on `(u32, u32)` would do the same, but inline `||` keeps the hot loop branch-friendly.

---

## One Search, Both Answers

The cleanest implementation runs **one** DFS that updates both metrics simultaneously, and exposes three public entry points:

```rust
fn search(comps: &Components) -> (u32, u32) {
    let mut best_strength = 0u32;
    let mut best_long = (0u32, 0u32);
    dfs(comps, 0, 0, 0, 0, &mut best_strength, &mut best_long);
    (best_strength, best_long.1)
}

pub fn solve(input: &str) -> (u32, u32) {
    let comps = parse_input(input);
    search(&comps)
}
```

`solve_part1`/`solve_part2` re-parse and re-search to keep the API symmetric, but `solve()` — the path used by the runner and the `aoc2017 24` binary — does **one** parse and **one** search. Combined runtime ≈ each-part runtime, not the sum, which is the parse-once pattern's signature in the benchmarks below.

---

## Bitmask Trick: 56 Components Fit in a `u64`

Tracking "which components are already used" with a `Vec<bool>` works but allocates on every recursion frame, and equality/clone of a 56-element bool vec is O(n). A `u64` bitmask is:

- **O(1) membership**: `used & (1 << idx) != 0`
- **O(1) insertion (functional)**: `used | (1 << idx)`
- **No allocation**: lives in a register
- **Trivially cloneable**: passed by value through `Copy`

The `assert!(parts.len() <= 64)` in `parse_input` guards the assumption — if a future input ever ships >64 components the solver fails loudly at parse time rather than silently truncating bits.

For >64 components the next step up is a fixed-size `[u64; N]` array (still no allocation), then `BitVec`. None of those are needed at this scale.

---

## Self-Loops `p/p` and Why They're Stored Once

A component like `2/2` has both ports the same. Naïvely:

```rust
adj[a].push((idx, b));
adj[b].push((idx, a));
```

would push `(idx, 2)` into `adj[2]` **twice**, so the DFS would consider taking the component "in either orientation" — but those orientations are identical (the component is symmetric). The bitmask would correctly prevent the double-use within one path, but the wasted iterations show up as extra branches at every visit to the self-loop's port.

Fix: skip the second push when `a == b`:

```rust
adj[a as usize].push((i as u8, b));
if a != b {
    adj[b as usize].push((i as u8, a));
}
```

A unit test verifies this:

```rust
let two_via_one = comps.adj[2].iter().filter(|&&(i, _)| i == 1).count();
assert_eq!(two_via_one, 1);   // self-loop 2/2 is recorded exactly once
```

---

## Benchmarks

Measured with Criterion (Linux release):

| Target           | Time         |
|------------------|--------------|
| `day24_part1`    | **4.94 ms**  |
| `day24_part2`    | **5.00 ms**  |
| `day24` combined | **4.98 ms**  |

Observations:

- **Both parts are the same DFS.** The full search visits every reachable bridge state regardless of which metric we're maximising — Part 1 and Part 2 share work entirely. Their per-part runtimes are within measurement noise.
- **Combined ≈ each part.** `solve()` runs one search and returns both stats; `solve_part1` and `solve_part2` each run a fresh search. Combined runtime is half of (part1 + part2), exactly what the parse-once pattern predicts.
- **5 ms total for 56 components** ≈ 90 µs per component of search budget. The actual reachable-bridge count for this input is in the low millions; the DFS amortises to ~1–2 ns per recursion frame.

---

## Why No Pruning

The first instinct on a backtracking puzzle is to prune — `current_strength + Σ(remaining ports) < best_strength` cuts dead branches. For this input it isn't worth it:

- The full search runs in 5 ms. Doubling that would still be unnoticeable; halving it doesn't change anything observable.
- The pruning bound (sum of all remaining ports' contributions) is loose because most components touching a given port type can't actually be reached from the current frontier — computing a tighter bound costs more than the saved branches.
- Part 2 needs the **longest** bridge, not the strongest. A strength bound doesn't help eliminate length-suboptimal branches; you'd need a separate "remaining-reachable-length" bound, which is computed by… another DFS.

Pruning would matter if the input were 200+ components, or if we genuinely needed sub-millisecond performance. Neither holds.

---

## Key Patterns

### Backtracking with a `u64` used-mask

The pattern — `used | bit` on the recurse, no manual `used &= !bit` on unwind — works for **any** subset-search up to 64 elements. Day 24 here, the [Day 11 hex walks](day11_function_guide.md) (different shape but same backtracking idea), and many a SAT-style search. The `Copy` semantics of `u64` make the implicit unwind correct.

### Adjacency keyed by domain value, not by item index

Most graph problems index adjacency by **node**: `adj[u] = neighbours(u)`. Here, a "node" in the search graph is the (open port, used set) pair — but the open port is what determines candidates, so we key adjacency by **port value**. That's a small re-framing that turns a per-step linear scan into a single index.

The same trick shows up in [Day 12 (Mission 10 Union-Find)](day12_function_guide.md) where adjacency is by program-ID, and in any "match-by-attribute" problem (job matching, interval graphs, etc.).

### Lexicographic max in one pass

`max_by_key(|&b| (length(b), strength(b)))` over all bridges is the same as tracking `(length, strength)` and updating with a 2-tuple comparison on every visit. Doing it inline avoids materialising every bridge, which would be millions of `Vec<usize>` allocations.

For Part 1 only, the second tuple field is unused; sharing one DFS is essentially free since the comparison is two integer compares per call.

### Parse-once with a single shared search

When both parts answer the same recursive search with different aggregations, the right shape is **one** internal `search` returning a tuple, called once by `solve()`. The convenience `solve_part1`/`solve_part2` wrappers re-parse and re-search for API symmetry but aren't on the hot path. This is the same shape used by [Day 19](day19_function_guide.md) (one maze walk yields letters and step count).

---

## Integrator Notes

- **No mission reuse.** This problem is a backtracking search on a custom graph keyed by port value — it doesn't fit Mission 6 (`Grid<T>`), Mission 8 (`Graph` trait) or Mission 10 (`UnionFind`). The closest match would be Mission 8's BFS/DFS, but those operate on node indices and produce reachability/path data, not extremum-over-all-paths. A "search-over-subsets" mission could exist someday; right now, 30 lines of inline DFS is the right call.
- **AUTOSAR analogue.** Components with matching port types are exactly AUTOSAR SWCs with matching interface signatures — connect P-Port `0` to R-Port `0`, then the leftover R-Port becomes the next P-Port to match. Bridge-building is essentially manual port-mapping. The "starts at port 0" rule is the runtime entry point; the "use each component once" rule is the inventory constraint.
- **Cross-link to AoC 2017 Day 12.** Both days are graph problems on a small domain — Day 12 uses Union-Find for connectivity, Day 24 uses DFS for path-strength extrema. The shared insight is "model the input as a graph keyed by the natural domain value" (program-ID or port-type respectively) before reaching for an algorithm.
- **Project Euler crossover.** Subset-sum with extremum is a recurring PE shape (e.g. the "longest path in a graph with weights" family). The bitmask-DFS template here generalises directly: replace `port`-keyed adjacency with whatever the domain's matching rule is, keep the `u64` used-mask, swap the strength accumulator for the relevant aggregate.

---

**Navigation**: [← Day 23](day23_function_guide.md) | [All Days](../summary_2017.md) | Day 25 →
