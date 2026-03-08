# Day 8: Two-Factor Authentication --- Function Guide

**Problem**: Simulate a 50x6 pixel screen with `rect`, `rotate row`, and `rotate column` operations.
**Answers**: Part 1 = **123**, Part 2 = **AFBUPZBJPS**
**Code**: [day08.rs](../../src/solver/day08.rs) | **Visualization**: [day08_viz.rs](../../examples/day08_viz.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Algorithm Overview](#algorithm-overview)
3. [parse_instructions](#parse_instructions)
4. [simulate](#simulate)
5. [render](#render)
6. [Mission 6 Integration](#mission-6-integration)
7. [Interactive Visualization](#interactive-visualization)
8. [Benchmarks](#benchmarks)
9. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: 173 instructions for a 50-wide x 6-tall pixel screen (all pixels start off).

**Operations**:
- `rect AxB` — turn on all pixels in a rectangle at the top-left (A wide, B tall)
- `rotate row y=A by B` — shift row A right by B pixels (wrapping)
- `rotate column x=A by B` — shift column A down by B pixels (wrapping)

**Part 1**: Count how many pixels are lit after all instructions.

**Part 2**: Read the 10-letter code displayed on the screen. The instructions encode a bitmap as a program — each person's input spells different letters.

**Final screen**:
```
.##..####.###..#..#.###..####.###....##.###...###.
#..#.#....#..#.#..#.#..#....#.#..#....#.#..#.#....
#..#.###..###..#..#.#..#...#..###.....#.#..#.#....
####.#....#..#.#..#.###...#...#..#....#.###...##..
#..#.#....#..#.#..#.#....#....#..#.#..#.#.......#.
#..#.#....###...##..#....####.###...##..#....###..
```

---

## Algorithm Overview

Straightforward simulation — no optimization needed. Parse instructions into an enum, apply each to a `Grid<bool>`, count/render.

```
Input (173 lines)
    |
    v
parse_instructions() -> Vec<Instruction>
    |
    v
simulate() -> Grid<bool>     (apply all instructions sequentially)
    |
    +-> count lit pixels      (Part 1: 123)
    +-> render to string      (Part 2: CFLELOYFCS)
```

**Complexity**: O(n * W * H) worst case where n = number of instructions. In practice, each instruction touches at most one row or column, so it's effectively O(n * max(W, H)).

---

## `parse_instructions`

```rust
fn parse_instructions(input: &str) -> Vec<Instruction>
```

Uses `strip_prefix` to classify each line into one of three variants:
- `"rect "` → parse `WxH` with `split_once('x')`
- `"rotate row y="` → parse `Y by AMOUNT` with `split_once(" by ")`
- `"rotate column x="` → parse `X by AMOUNT` with `split_once(" by ")`

**Design choice**: `strip_prefix` + `split_once` is cleaner than regex for these simple fixed-prefix formats. Zero allocation beyond the `Vec`.

---

## `simulate`

```rust
fn simulate(instructions: &[Instruction]) -> Grid<bool>
```

Creates a `Grid::new(50, 6, false)` and applies each instruction:

| Operation | Implementation |
|-----------|---------------|
| `Rect(w, h)` | Nested loop setting `screen[(x, y)] = true` for `x < w, y < h` |
| `RotateRow(y, amt)` | Copy row into `Vec`, write back with `(x + amt) % WIDTH` offset |
| `RotateCol(x, amt)` | Copy column into `Vec`, write back with `(y + amt) % HEIGHT` offset |

**Key detail**: Rotation requires a temporary copy because pixels shift in-place — reading and writing to the same row/column would corrupt values mid-rotation.

---

## `render`

```rust
fn render(screen: &Grid<bool>) -> String
```

Converts the grid to a human-readable string with `#` for lit and `.` for dark pixels, rows joined by newlines. Used for Part 2 output and debugging.

---

## Mission 6 Integration

This day uses **Mission 6** `Grid<bool>` instead of a raw `[[bool; 50]; 6]` array.

| Feature | Raw Array | Grid<bool> |
|---------|-----------|------------|
| Indexing | `screen[y][x]` (row-major, easy to swap x/y) | `screen[(x, y)]` (explicit x,y order) |
| Bounds checking | None (panic on OOB) | Checked with clear error messages |
| Iteration | `iter().flatten()` | `iter()` directly |
| Type safety | Anonymous `[[bool; 50]; 6]` | Named `Grid<bool>` with width/height methods |

**Integrator philosophy**: Even though the raw array works, using the validated `Grid` component from Mission 6 is consistent with other AoC days, provides type-safe `(x, y)` indexing, and avoids reinventing infrastructure.

---

## Interactive Visualization

An interactive step-through visualization is available at [day08_viz.rs](../../examples/day08_viz.rs).

**Run with**:
```bash
cargo run --example day08_viz
```

**Controls**:
| Key | Action |
|-----|--------|
| Right / Enter | Advance one step |
| Left | Go back one step |
| Home / End | Jump to beginning / end |
| Space | Toggle auto-play |
| +/- | Adjust auto-play speed (10ms-1000ms) |
| q / Escape | Quit |

**Features**:
- Pre-computes all 174 screen snapshots for instant scrubbing forward/backward
- Row numbers (0-5) on the left, column markers every 5 along the bottom
- Three-line instruction display: previous (what you'd undo), current (just applied), next (what Right arrow will do)
- Progress bar showing position in the 173-instruction sequence
- Auto-play mode with adjustable speed
- Uses `crossterm` for raw terminal input and ANSI escape codes for rendering

**Implementation approach**: All screen states are pre-computed into a `Vec<Grid<bool>>` at startup. This trades ~52KB of memory (174 grids x 300 bytes each) for O(1) navigation in any direction — the key insight that makes backwards stepping trivial without needing an undo system.

---

## Benchmarks

| Benchmark | Time |
|-----------|------|
| Part 1    | 17.7µs |
| Part 2    | 18.3µs |
| Combined  | 18.2µs |

Combined ~= individual parts because parsing and simulation dominate, and Part 1/Part 2 extract different information from the same simulated screen. The `render()` string building in Part 2 adds negligible cost (~0.5µs).

At 18µs, this is one of the fastest days — no optimization needed.

---

## Key Patterns

| Pattern | Description |
|---------|-------------|
| Screen simulation | Apply instructions sequentially to a mutable grid — common in AoC "pixel art" problems |
| Modular rotation | `(index + amount) % dimension` for circular shift — handles wrap-around naturally |
| Temporary copy for rotation | Copy row/column before writing back to avoid read-after-write corruption |
| Mission 6 integration | `Grid<bool>` with `(x, y)` indexing instead of raw 2D arrays |
| Pre-computed snapshots | Store all intermediate states for O(1) bidirectional navigation in visualization |
| `strip_prefix` parsing | Cleaner than regex for simple fixed-prefix instruction formats |
