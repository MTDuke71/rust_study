# Day 19: A Series of Tubes — Function Guide

**Problem**: Follow an ASCII-art packet maze from the single `|` in the top row. Go straight through `|`/`-`, turn 90° at `+`, collect letters A–Z in visit order, stop on the first space.
**Answers**: Part 1 = **BPDKCZWHGT**, Part 2 = **17728**
**Code**: [day19.rs](../../src/solver/day19.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Data Model](#data-model)
3. [parse_input](#parse_input)
4. [walk](#walk)
5. [Corner Handling](#corner-handling)
6. [Benchmarks](#benchmarks)
7. [Key Patterns](#key-patterns)
8. [Integrator Notes](#integrator-notes)

---

## Problem Summary

The input is a 201×201 grid of characters:

| Char | Meaning |
|------|---------|
| `\|` | vertical tube — continue in the current vertical direction |
| `-`  | horizontal tube — continue in the current horizontal direction |
| `+`  | corner — turn 90° toward the only non-space perpendicular neighbour |
| A–Z  | waypoint letter — collect, keep going |
| ` `  | void — the journey ends when the next cell is space (or off-grid) |

Start position is the unique `|` on row 0, heading **down**. Crossings (where a vertical tube runs over a horizontal tube) are **not** corners — `|` and `-` never change direction, so the interpretation `|` over `-` just means "keep going".

- **Part 1**: the collected letters, in order.
- **Part 2**: total cells visited, including the start cell.

Both answers come from a **single traversal** — there is no reason to walk twice.

---

## Data Model

```rust
enum Dir { Up, Down, Left, Right }

impl Dir {
    fn offset(self) -> (isize, isize) { /* cardinal deltas */ }
    fn perpendicular(self) -> [Dir; 2] {
        match self {
            Dir::Up | Dir::Down    => [Dir::Left, Dir::Right],
            Dir::Left | Dir::Right => [Dir::Up,   Dir::Down],
        }
    }
}
```

Grid is `mission6::Grid<u8>` (byte-per-cell). Positions are `mission6::Coord { x: usize, y: usize }`. Signed deltas are applied via `as isize` arithmetic and bounds-checked before converting back.

---

## `parse_input`

```rust
fn parse_input(input: &str) -> Grid<u8>
```

1. Normalize CRLF to LF (`.replace("\r\n", "\n")`).
2. Find the max line length — input lines can be shorter than the widest row if trailing spaces were trimmed.
3. Pad each row to that width with `b' '`.
4. Wrap in `Grid::from_vec2d` for uniform `get(Coord)` access.

Padding matters: the grid is ragged if trailing spaces are stripped, and off-the-right-edge reads must return "space" semantically. Rectangularising once is cheaper than handling ragged rows in the hot loop.

---

## `walk`

```rust
fn walk(grid: &Grid<u8>) -> (String, usize)
```

Single loop, returns both answers:

```rust
let start_x = grid.row(0).unwrap().position(|&b| b == b'|').unwrap();
let mut pos  = Coord::new(start_x, 0);
let mut dir  = Dir::Down;
let mut steps = 1usize;         // start cell counts
let mut letters = String::new();

loop {
    match *grid.get(pos).unwrap() {
        b'+'                     => dir = pick_turn(pos, dir, grid),
        c if c.is_ascii_uppercase() => letters.push(c as char),
        _                        => {} // '|' or '-' — pass through
    }
    match step(pos, dir, grid) {
        Some(next) => { pos = next; steps += 1; }
        None       => break,       // off-grid or blank = stop
    }
}
```

The termination condition is embedded in `step()`:

```rust
fn step(pos: Coord, dir: Dir, grid: &Grid<u8>) -> Option<Coord> {
    let (dx, dy) = dir.offset();
    let nx = pos.x as isize + dx;
    let ny = pos.y as isize + dy;
    if nx < 0 || ny < 0 { return None; }
    match grid.get(Coord::new(nx as usize, ny as usize)) {
        Some(&b) if b != b' ' => Some(Coord::new(nx as usize, ny as usize)),
        _                     => None,          // off-grid OR space = stop
    }
}
```

Off-grid and space collapse into the same `None` — the caller never needs to distinguish them.

---

## Corner Handling

At `+`, exactly one of the two perpendicular directions leads to a non-space cell. The turn picks the first one for which `step()` succeeds:

```rust
dir.perpendicular()
   .into_iter()
   .find(|&d| step(pos, d, grid).is_some())
   .expect("corner must have one perpendicular exit")
```

This is why `step()` returning `Option<Coord>` is the right shape: the corner check and the forward move use **the same predicate** — "is there a non-space cell that direction?" No bespoke look-ahead logic.

Crossings look like `+` but aren't: at a true crossing the current tile is `|` over `-` (or vice versa), **not** `+`. The `match tile { b'+' => turn, ... }` dispatch keeps crossings implicit — they hit the `_ => {}` arm and pass through unchanged.

---

## Benchmarks

| Stage | Time |
|-------|------|
| Part 1 alone | **85.50 µs** |
| Part 2 alone | **85.26 µs** |
| Combined `solve()` | **89.97 µs** |

- Part 1 ≈ Part 2 because both run the **same full traversal** — the only difference is which field of the returned tuple is consumed.
- Combined time ≈ a single traversal, which confirms the parse-once / walk-once pattern: `solve()` pays for one parse and one walk, then extracts both answers.
- Traversal length ≈ 17,728 steps × a constant per step (one `grid.get` plus a byte match). Dominates over the 201×201 parse pass.

---

## Key Patterns

### One traversal, two answers
The letters and the step count are both side effects of the **same** walk. Splitting into `solve_part1` and `solve_part2` that each re-walk would double the runtime for no benefit — so the internal primitive is `walk() -> (String, usize)` and the public part-1/part-2 functions just project the tuple. `cargo bench` confirms this: combined ≈ single walk, not 2× single walk.

### `Option<Coord>` unifies move + boundary check
Returning `Option<Coord>` from `step()` means the caller doesn't branch on "off-grid vs blank" — both are dead-ends. The corner logic reuses the same primitive to probe which direction to turn. Two call sites, one predicate.

### Pad-then-index instead of guard-every-read
Normalising ragged rows at parse time (`row.resize(width, b' ')`) means every subsequent `grid.get` returns a meaningful byte. The alternative — checking `x < row.len()` in the hot loop — would add a branch per read for zero benefit; padding front-loads the cost to parse.

### `match` on byte instead of `if` chains
```rust
match tile {
    b'+' => turn,
    c if c.is_ascii_uppercase() => collect(c),
    _ => pass_through,
}
```
The `is_ascii_uppercase` guard matches A–Z in one test; no range check, no `if c >= b'A' && c <= b'Z'`. Reads cleanly and the compiler generates the same range check under the hood.

---

## Integrator Notes

- **Mission 6 reuse**: `Grid<u8>` + `Coord` replace a bespoke `Vec<Vec<u8>>`. The win isn't speed (both are O(1) indexed) — it's that `grid.get(Coord) -> Option<&T>` already does the bounds check and returns the type signature the caller wants. Fewer `if x < grid.len() && y < grid[0].len()` guards in day code.
- **Signed-delta bridge**: Mission 6's `Coord` is `usize`-based, so the ergonomic answer to "move up/left" is the `as isize` arithmetic + bounds-check pattern seen in 2023 Day 16. The `step()` helper contains this bridge in one place so the main loop never sees raw `isize`.
- **AUTOSAR analogue**: The walk is a stateless SWC reading sensor data (`grid.get`) and emitting control decisions (`turn` vs `pass`). No shared state, no concurrency — this would be a single runnable on a basic task in classic AUTOSAR terms.

---

**Navigation**: [← Day 18](day18_function_guide.md) | [All Days](../summary_2017.md) | Day 20 →
