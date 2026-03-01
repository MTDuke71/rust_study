# Day 1: No Time for a Taxicab — Function Guide

**Problem**: Follow turn+walk instructions on a city grid. Find Manhattan distance from origin.
**Answers**: Part 1 = **161**, Part 2 = **110**
**Code**: [day01.rs](../../src/solver/day01.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Data Structures](#data-structures)
3. [parse_input](#parse_input)
4. [solve_part1_with_data](#solve_part1_with_data)
5. [solve_part2_with_data](#solve_part2_with_data)
6. [Benchmarks](#benchmarks)
7. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: Comma-separated instructions like `R4, L2, R5, ...`
Each instruction: turn 90° left or right, then walk N blocks.

**Part 1**: What is the Manhattan distance (`|x| + |y|`) from the start when all instructions are done?

**Part 2**: Walking the same path block-by-block, what is the Manhattan distance to the **first location visited twice**?

**Why Part 2 is harder**: You can't jump by `steps` — you must visit every intermediate block to catch revisits. This means tracking potentially hundreds of `(x, y)` positions in a `HashSet`.

---

## Data Structures

### `struct Instruction`

```rust
#[derive(Debug, Clone, Copy)]
struct Instruction {
    turn: i32,  // +1 = right, -1 = left
    steps: i32,
}
```

Stores a parsed instruction. `turn` encodes direction as a signed integer so that direction arithmetic is a single `rem_euclid(4)` call — no match needed.

### Direction encoding

```rust
const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
//                              North    East    South     West
```

Direction index rotates clockwise. Turning right = `+1 mod 4`, left = `-1 mod 4`. `rem_euclid` handles negative values correctly (unlike `%` which can return negative in Rust).

---

## `parse_input`

```rust
fn parse_input(input: &str) -> Vec<Instruction> {
    input.trim().split(", ").map(|s| {
        let turn = if s.starts_with('R') { 1 } else { -1 };
        let steps = s[1..].parse().unwrap();
        Instruction { turn, steps }
    }).collect()
}
```

**Input**: `"R4, L2, R5, R1"`
**Output**: `[{turn:1, steps:4}, {turn:-1, steps:2}, ...]`

- `trim()` removes trailing newline from puzzle input
- `split(", ")` — note the space: instructions are always `"X, Y"` format
- `s[1..]` slices off the direction character to get the number string

---

## `solve_part1_with_data`

```rust
fn solve_part1_with_data(instructions: &[Instruction]) -> i32 {
    let mut dir = 0i32;
    let (mut x, mut y) = (0i32, 0i32);

    for inst in instructions {
        dir = (dir + inst.turn).rem_euclid(4);
        let (dx, dy) = DIRS[dir as usize];
        x += dx * inst.steps;
        y += dy * inst.steps;
    }

    x.abs() + y.abs()
}
```

**Algorithm**: Walk all instructions, jumping `steps` at a time. No per-block tracking needed — just accumulate (x, y) and compute Manhattan distance at end.

**Trace on `R2, L3`**:
```
Start:    dir=0 (N),  x=0,  y=0
R2:       dir=1 (E),  x=2,  y=0
L3:       dir=0 (N),  x=2,  y=3
Result:   |2| + |3| = 5 ✓
```

**Complexity**: O(n) time, O(1) space.

---

## `solve_part2_with_data`

```rust
fn solve_part2_with_data(instructions: &[Instruction]) -> i32 {
    use std::collections::HashSet;

    let mut dir = 0i32;
    let (mut x, mut y) = (0i32, 0i32);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((x, y));

    'outer: for inst in instructions {
        dir = (dir + inst.turn).rem_euclid(4);
        let (dx, dy) = DIRS[dir as usize];
        for _ in 0..inst.steps {
            x += dx;
            y += dy;
            if !visited.insert((x, y)) {
                break 'outer;
            }
        }
    }

    x.abs() + y.abs()
}
```

**Key difference from Part 1**: Inner `for _ in 0..inst.steps` loop moves one block at a time, inserting each `(x, y)` into the `HashSet`.

`HashSet::insert` returns `false` if the value was already present — that's the first revisit. `break 'outer` exits both loops immediately, leaving `x, y` at the revisit location.

**Why `'outer` label?**: Breaking from the outer loop from inside the inner loop requires a named label. Without it, `break` would only exit the inner `for _ in 0..steps` loop.

**Complexity**: O(n × s) time where s = average steps per instruction, O(positions) space. In practice: ~few hundred positions for typical inputs.

---

## `solve` (Public API — Parse-Once)

```rust
pub fn solve(input: &str) -> (i32, i32) {
    let data = parse_input(input);
    (solve_part1_with_data(&data), solve_part2_with_data(&data))
}
```

Parses **once**, passes parsed `Vec<Instruction>` to both parts. This is the parse-once pattern: `data` is shared, not re-parsed for each part.

---

## Benchmarks

| Function | Time |
|----------|------|
| `solve_part1` | 1.6µs |
| `solve_part2` | 13.2µs |
| `solve` (combined) | 13.5µs |

**Combined ≈ Part 2 alone** — confirms parse-once is working. The parse and Part 1 add ~0.3µs on top of Part 2.

**Why Part 2 is 8× slower than Part 1**: `HashSet::insert` for every individual block. With ~hundreds of blocks total across all instructions, the `HashSet` allocation and repeated hashing dominates.

**Optimization headroom**: If needed, `FxHashSet` (faster hashing, no crypto) could reduce Part 2 time. At 13µs, well under the 100ms target — no optimization needed.

---

## Key Patterns

### `rem_euclid` for circular indexing

```rust
dir = (dir + inst.turn).rem_euclid(4);
```

`rem_euclid` always returns a non-negative result, even when the dividend is negative. This is essential for turning left from North (`0 + (-1) = -1`), which `% 4` would give `-1` (wrong), but `rem_euclid(4)` gives `3` (West, correct).

### Labeled break for nested loops

```rust
'outer: for inst in instructions {
    for _ in 0..inst.steps {
        if condition {
            break 'outer;  // exits the outer loop, not just inner
        }
    }
}
```

The cleanest way to exit nested loops in Rust when a condition is found mid-traversal. Equivalent to a `goto done` in other languages, but structured and safe.

### `HashSet::insert` returns bool

```rust
if !visited.insert((x, y)) {
    // (x, y) was already in the set — first revisit found
}
```

`insert` returns `true` if the value was newly inserted, `false` if it was already present. Combining insert-and-check in one call avoids a `contains` + `insert` double lookup.
