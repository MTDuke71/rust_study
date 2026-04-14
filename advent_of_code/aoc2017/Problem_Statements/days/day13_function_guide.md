# Day 13: Packet Scanners — Function Guide

**Problem**: Cross a firewall of oscillating scanners. Part 1 sums the severity of getting caught on delay 0; Part 2 finds the smallest delay that avoids every scanner.
**Answers**: Part 1 = **1840**, Part 2 = **3850260**
**Code**: [day13.rs](../../src/solver/day13.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [The Period Insight](#the-period-insight)
3. [parse_input](#parse_input)
4. [solve_part1_with_data](#solve_part1_with_data)
5. [solve_part2_with_data](#solve_part2_with_data)
6. [Benchmarks](#benchmarks)
7. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: ~40 lines, one per scanner layer, format `depth: range`.

```
0: 3
1: 2
4: 4
6: 4
```

Each layer has a scanner bouncing top↔bottom within `range` positions. You move one layer per picosecond. You are caught at layer `d` if the scanner there is at position 0 the moment you enter.

**Part 1**: With zero delay, sum `depth * range` for every layer that catches you.

**Part 2**: Find the smallest delay `t` such that no scanner catches you at any layer.

---

## The Period Insight

A scanner of range `R` walks `0 → 1 → ... → R-1 → R-2 → ... → 1 → 0`, a cycle of length **`2*(R-1)`** picoseconds. It is at position 0 exactly when picosecond `p` satisfies `p mod 2*(R-1) == 0`.

- **Part 1** arrives at layer `d` at picosecond `d`, so caught iff `d mod 2*(R-1) == 0`.
- **Part 2** starts `t` picoseconds late, so arrives at layer `d` at picosecond `d+t`, caught iff `(d+t) mod 2*(R-1) == 0`.

This is the whole solution. There is **no picosecond-by-picosecond simulation** — the visuals in the problem statement are a red herring once you have the period formula.

---

## `parse_input`

```rust
fn parse_input(input: &str) -> Vec<(u32, u32)>
```

Splits each line on `": "` and parses the pair. Ranges in the real input are all ≥ 2, so `2 * (range - 1)` never underflows.

---

## `solve_part1_with_data`

```rust
fn solve_part1_with_data(data: &[(u32, u32)]) -> u32 {
    data.iter()
        .filter(|&&(depth, range)| depth % (2 * (range - 1)) == 0)
        .map(|&(depth, range)| depth * range)
        .sum()
}
```

One pass: filter the caught layers, map to severity, sum. O(n) where n is the number of scanners (~40).

---

## `solve_part2_with_data`

```rust
fn solve_part2_with_data(data: &[(u32, u32)]) -> u32 {
    (0..)
        .find(|&delay| {
            data.iter()
                .all(|&(depth, range)| (delay + depth) % (2 * (range - 1)) != 0)
        })
        .expect("a valid delay must exist")
}
```

`(0..)` is an unbounded iterator; `.find()` returns the first `delay` where `.all(...)` holds. `.all()` short-circuits on the first scanner that catches you, so most candidate delays reject in a handful of mod operations.

**Complexity**: worst case O(t · n) where `t` is the winning delay (3,850,260 here) and `n` is the scanner count. In practice early rejection keeps the average work per `delay` tiny.

---

## Benchmarks

| Function | Time |
|----------|------|
| `solve` (combined) | 13.01ms |

Part 2 dominates — 3.85M candidate delays × short-circuited modulo checks. Part 1 is microseconds.

### Optimization considered, not applied

Rayon parallelization: Part 2's delay space could be sharded across threads, taking the global min of per-chunk hits. Rejected for now — 13ms is comfortably within the "fast enough" zone, and preserving the minimum-delay semantics across chunks adds real complexity. Revisit if the bound tightens.

---

## Key Patterns

### Math over simulation

The problem *describes* a physical process (scanners bouncing, packets moving) and *shows* a picosecond-by-picosecond animation. That framing primes you to write a simulator. But the process is periodic and deterministic — once you notice the period is `2*(R-1)`, the simulation collapses to one modulo per scanner per delay.

This is a recurring AoC tell: when the problem spends half its statement drawing a timeline, ask whether the timeline has a closed-form schedule.

### `(0..).find(...)` for smallest-satisfying search

Unbounded range + `.find()` is the Rust idiom for "smallest `n` such that `P(n)`". No manual loop, no mutable counter, no explicit break — the iterator chain says exactly what you mean.

### `.all()` short-circuits

`data.iter().all(predicate)` stops on the first `false`. For Part 2, most delays are rejected by the *first* scanner that catches you, so `.all()` only runs a full pass on the rare delay that makes it through. This is why 3.85M iterations finish in ~13ms.

### One parse, two solves

`parse_input` runs once; both parts operate on `&[(u32, u32)]`. Splitting the solver into `_with_data` helpers keeps `solve()` from reparsing and makes the math visible without the string-munging noise.
