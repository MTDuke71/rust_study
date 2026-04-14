# Day 12: Digital Plumber — Function Guide

**Problem**: Given a list of bidirectional pipe connections between programs, find the size of the group containing program `0` and the total number of groups.
**Answers**: Part 1 = **134**, Part 2 = **193**
**Code**: [day12.rs](../../src/solver/day12.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Why Union-Find](#why-union-find)
3. [build_uf](#build_uf)
4. [solve_both](#solve_both)
5. [Benchmarks](#benchmarks)
6. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: 2,000 lines, one per program, format `id <-> peer1, peer2, ...`.

```
0 <-> 889, 1229, 1736
1 <-> 1, 480, 793, 1361
2 <-> 607
...
```

**Part 1**: How many programs are in the group containing program `0`?

**Part 2**: How many distinct groups are there in total?

Pipes are bidirectional — if program `8` lists `11`, then `11` will also list `8`. Self-loops (`1 <-> 1`) are allowed.

---

## Why Union-Find

Counting connected components is the canonical use case for [Union-Find](../../../../missions/Mission10/README.md) (a.k.a. Disjoint Set Union). Every edge is a `union` call; every component is one equivalence class.

**Alternatives considered**:
- **BFS/DFS with adjacency map**: `HashMap<usize, Vec<usize>>`, visit-queue per component. Works, but requires two data structures and explicit visited-set bookkeeping.
- **Union-Find** (chosen): single `UnionFind` instance, one `union` per edge. `members(0).count()` answers Part 1 directly; `components().count()` answers Part 2 directly.

Mission 10's `UnionFind` uses path compression + union by rank, giving near-O(α(n)) per operation — effectively constant time in practice.

---

## `build_uf`

```rust
fn build_uf(input: &str) -> UnionFind
```

Parses the input and returns a fully-unioned `UnionFind`.

**Algorithm**:
1. Count non-blank lines → this is the number of programs (and the `UnionFind` capacity)
2. For each line, split on `" <-> "` into `(lhs, rhs)`
3. Parse `lhs` as the program ID
4. Split `rhs` on `,`, parse each peer, call `uf.union(id, peer_id)`

Self-loops (`1 <-> 1`) become `uf.union(1, 1)` which is a no-op in the Union-Find (same root, short-circuits).

---

## `solve_both`

```rust
fn solve_both(input: &str) -> (usize, usize)
```

```rust
let mut uf = build_uf(input);
let group_zero = uf.members(0)?.count();
let total_groups = uf.components().count();
(group_zero, total_groups)
```

Both answers come from the same `UnionFind` state — no reparsing, no second pass.

**Complexity**: O(n · α(n)) for building, where n is the number of edges. α(n) is the inverse Ackermann function, effectively ≤ 4 for any realistic input.

---

## Benchmarks

| Function | Time |
|----------|------|
| `solve` (combined) | 150.98µs |

**Performance breakdown**: ~6,000 edges parsed and unioned, then two linear scans for `members(0)` and `components()`. Parsing dominates — Union-Find operations are effectively free.

---

## Key Patterns

### Integrator philosophy: reuse validated components

This solver is ~15 lines because all the hard work lives in Mission 10. The pattern: recognize the problem shape (connected components), pull the validated component off the shelf, and write a parser. No custom graph traversal, no visited-set bookkeeping, no tricky recursion.

This is the same instinct that refactored AoC 2023 Day 16 to use Mission 6's `Grid<T>` — compose, don't reinvent.

### `split_once` for deterministic parsing

```rust
let (lhs, rhs) = line.split_once(" <-> ").expect("invalid line format");
```

When the input has exactly one separator per line, `split_once` returns a tuple directly — no need to collect into a `Vec` or pattern-match on iterator results. Cleaner and faster than `split(" <-> ").collect::<Vec<_>>()`.

### Both parts, one data structure

Many AoC problems expose Part 2 by asking for a different view of the same state. Here, `members(0).count()` and `components().count()` both read from a single fully-built `UnionFind`. Computing them together in `solve_both` is cheaper than treating them as independent problems.

### Self-loops are harmless

The input contains `1 <-> 1`. In a Union-Find, `union(1, 1)` finds the same root for both arguments and returns `false` (already connected) — no special handling needed. The abstraction absorbs the edge case for free.
