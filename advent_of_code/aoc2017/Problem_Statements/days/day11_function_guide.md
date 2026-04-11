# Day 11: Hex Ed — Function Guide

**Problem**: Follow a path on a hex grid and compute the minimum distance from the origin, both at the end and at the farthest point during the walk.
**Answers**: Part 1 = **722**, Part 2 = **1,551**
**Code**: [day11.rs](../../src/solver/day11.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Cube Coordinates](#cube-coordinates)
3. [solve_both](#solve_both)
4. [hex_distance](#hex_distance)
5. [Benchmarks](#benchmarks)
6. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: A single line of ~8,000 comma-separated hex directions (`n`, `ne`, `se`, `s`, `sw`, `nw`).

**Part 1**: After following the entire path, what is the **fewest steps** to reach the final position from the origin?

**Part 2**: What is the **farthest** distance reached at any point during the walk?

**Examples**:
```
ne,ne,ne         → 3 (straight line)
ne,ne,sw,sw      → 0 (back to start)
ne,ne,s,s        → 2 (ne+s collapses to se)
se,sw,se,sw,sw   → 3 (simplifies to s,s,sw)
```

---

## Cube Coordinates

Hex grids have several coordinate systems. Cube coordinates use three axes (q, r, s) where **q + r + s = 0** always holds.

```
     +s / \ -r
       /   \
      +--+--+
     /   |   \
   -q    |    +q
     \   |   /
      +--+--+
       \   /
     +r \ / -s
```

Each direction maps to a fixed offset:

| Direction | dq | dr | ds |
|-----------|----|----|-----|
| n         | 0  | -1 | +1  |
| ne        | +1 | -1 | 0   |
| se        | +1 | 0  | -1  |
| s         | 0  | +1 | -1  |
| sw        | -1 | +1 | 0   |
| nw        | -1 | 0  | +1  |

Note every offset sums to 0 (dq + dr + ds = 0), preserving the invariant.

---

## `hex_distance`

```rust
fn hex_distance(q: i32, r: i32, s: i32) -> i32
```

Returns `max(|q|, |r|, |s|)`. This works because each step changes exactly two coordinates by 1 (one up, one down). The largest absolute coordinate tells you the minimum steps — you can always walk diagonally to reduce two coordinates simultaneously.

---

## `solve_both`

```rust
fn solve_both(input: &str) -> (i32, i32)
```

Single pass over the comma-separated directions:
1. Update (q, r, s) by the direction offset
2. Compute `hex_distance` at each step, track maximum
3. Return (final distance, max distance)

**Complexity**: O(n) time, O(1) space — just three coordinate variables and a max tracker.

---

## Benchmarks

| Function | Time |
|----------|------|
| `solve` (combined) | 37.40µs |

**Performance breakdown**: ~8,000 steps, each doing a string match + 3 additions + 1 max comparison. Dominated by the string splitting and matching.

---

## Key Patterns

### Cube coordinates for hex grids

The key insight is choosing the right coordinate system. Offset coordinates (row/col) make hex distance ugly — you need conditional formulas depending on even/odd rows. Cube coordinates make it trivial: distance = `max(|q|, |r|, |s|)`. No conditionals, no special cases.

This is worth remembering for any hex grid AoC problem. The [Red Blob Games hex guide](https://www.redblobgames.com/grids/hexagons/) is the definitive reference.

### Cancellation is implicit

The problem description talks about steps "canceling out" (ne + sw = 0) or "collapsing" (ne + s = se). With cube coordinates, you don't need to implement cancellation rules — just accumulate the offsets and the distance formula handles it automatically. The final (q, r, s) position already reflects all cancellations.

### Max-during-walk pattern

Part 2 asks for the maximum distance at any point during the walk. This is the same "track all-time max during single pass" pattern from Day 8 — one extra comparison per step, zero extra passes.
