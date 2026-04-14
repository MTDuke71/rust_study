# Day 14: Disk Defragmentation — Function Guide

**Problem**: Build a 128×128 disk grid from 128 Knot Hashes. Part 1 counts used squares; Part 2 counts 4-connected regions of used squares.
**Answers**: Part 1 = **8106**, Part 2 = **1164**
**Code**: [day14.rs](../../src/solver/day14.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [parse_input](#parse_input)
3. [is_used](#is_used)
4. [solve_part1_with_data](#solve_part1_with_data)
5. [solve_part2_with_data](#solve_part2_with_data)
6. [Benchmarks](#benchmarks)
7. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: a short key string (mine is `oundnydw`).

The disk is a 128×128 grid of used (`#`) / free (`.`) squares. Row `r` is produced by the Knot Hash of `"{key}-{r}"` interpreted as 128 bits — the 16 dense-hash bytes expanded high-bit first.

- **Part 1**: total used squares across the whole grid.
- **Part 2**: number of 4-connected regions of used squares (diagonals do not connect).

---

## `parse_input`

```rust
type Grid = [[u8; 16]; 128];

fn parse_input(input: &str) -> Grid {
    let key = input.trim();
    let mut grid = [[0u8; 16]; 128];
    for (row, row_bytes) in grid.iter_mut().enumerate() {
        *row_bytes = knot_hash_bytes(&format!("{key}-{row}"));
    }
    grid
}
```

- Uses `day10::knot_hash_bytes` — the 16-byte dense hash, reused directly rather than reparsed from its hex string.
- Stores the grid as raw bytes, not unpacked bits: 128 × 16 = **2 KiB stack**, no heap allocation.
- Dominates runtime: 128 Knot Hashes at ~27µs each ≈ **3.4ms** (essentially all of total time).

---

## `is_used`

```rust
fn is_used(grid: &Grid, r: usize, c: usize) -> bool {
    let byte = grid[r][c / 8];
    let bit = 7 - (c % 8);
    (byte >> bit) & 1 == 1
}
```

Column `c` lives in byte `c / 8`, high-bit first (`0xFF` → columns 0..=7 all set), so bit index is `7 - (c % 8)`.

---

## `solve_part1_with_data`

```rust
fn solve_part1_with_data(grid: &Grid) -> u32 {
    grid.iter()
        .flat_map(|row| row.iter())
        .map(|b| b.count_ones())
        .sum()
}
```

Never unpacks bits — `u8::count_ones` (CPU `popcnt`) handles each byte in one instruction. Microseconds; the grid is already built.

---

## `solve_part2_with_data`

```rust
let mut uf = UnionFind::new(128 * 128);
for r in 0..128 {
    for c in 0..128 {
        if !is_used(grid, r, c) { continue; }
        if c + 1 < 128 && is_used(grid, r, c + 1) { uf.union(idx(r,c), idx(r,c+1))?; }
        if r + 1 < 128 && is_used(grid, r + 1, c) { uf.union(idx(r,c), idx(r+1,c))?; }
    }
}
```

- **Mission 10 reuse**: `UnionFind` with path compression + union by rank gives near-O(α(n)) unions.
- Only look at **right and down** neighbors — left and up get unioned when their owner visits, avoiding duplicate work.
- Count regions by collecting distinct `find(idx)` roots over used cells into a `HashSet`. Empty cells stay in their own singleton sets but are never counted.

Allocates a flat `UnionFind` over all 16,384 cells (used or not) to keep indexing trivial. Cheap — the size dominates over cleverness.

---

## Benchmarks

| Part | Time | Notes |
|------|------|-------|
| Combined (parse + both parts) | **3.47ms** | 128 Knot Hashes dominate |
| Parse (128 knot hashes) | ~3.4ms | 64 rounds × 128 rows |
| Part 1 (popcnt over 2 KiB) | <1µs | Negligible |
| Part 2 (Union-Find on 16K cells) | ~50µs | Rough estimate, lost in parse noise |

Target: <100ms ✅

---

## Key Patterns

### Bit-packed storage
Keeping the grid as 16 bytes per row instead of 128 `bool`s saves 8× memory and makes Part 1 a single `popcnt` per byte. `is_used` handles bit extraction on demand; there's no unpacking pass.

### Right+down unioning
A naive flood fill would check all 4 neighbors per cell. With Union-Find, unioning only with right and down neighbors is sufficient and cuts work in half — the `(r-1, c)` and `(r, c-1)` edges were already added from the other direction.

### Mission reuse as philosophy
This day is almost entirely composition: `knot_hash_bytes` (Day 10) + `UnionFind` (Mission 10). The only day-specific code is bit extraction and the union loop. Classic integrator day.
