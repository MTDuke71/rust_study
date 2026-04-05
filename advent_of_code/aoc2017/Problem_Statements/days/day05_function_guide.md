# Day 5: A Maze of Twisty Trampolines, All Alike — Function Guide

**Problem**: Follow jump offsets through a list, mutating them after each step, until escaping.
**Answers**: Part 1 = **374,269**, Part 2 = **27,720,699**
**Code**: [day05.rs](../../src/solver/day05.rs)

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

**Input**: 1,070 signed integers, one per line — jump offsets.

**Part 1**: Start at index 0. Each step: read the offset at the current position, jump forward/backward by that amount, then **increment** the old position's offset by 1. Count steps until the program counter leaves the list.

```
(0) 3  0  1 -3   → offset 0, stay in place, increment to 1
(1) 3  0  1 -3   → offset 1, jump forward, increment to 2
 2 (3) 0  1 -3   → offset 3, jump to end, increment to 4
 2  4  0  1 (-3)  → offset -3, jump back, increment to -2
 2 (4) 0  1 -2   → offset 4, escape! → 5 steps
```

**Part 2**: Same as Part 1, but if the offset is **≥ 3**, **decrement** by 1 instead of incrementing. The example takes **10 steps** instead of 5.

---

## Data Structures

### Parsed input
```rust
Vec<i32>
```

Simple flat vector of signed integers. 1,070 elements. Each part clones this into a mutable working copy since the simulation mutates offsets in-place.

---

## `parse_input`

```rust
fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().parse().unwrap())
        .collect()
}
```

Straightforward line-per-integer parsing. `trim()` handles any trailing whitespace/CR.

---

## `solve_part1_with_data`

```rust
fn solve_part1_with_data(data: &[i32]) -> usize {
    let mut jumps = data.to_vec();
    let mut pc: i32 = 0;
    let mut steps = 0;
    let len = jumps.len() as i32;

    while pc >= 0 && pc < len {
        let offset = jumps[pc as usize];
        jumps[pc as usize] += 1;
        pc += offset;
        steps += 1;
    }
    steps
}
```

**Strategy**: Pure simulation. Read offset, increment it, advance program counter. Exit when `pc` goes out of bounds.

**Why `i32` for pc**: Offsets can be negative, so the program counter must handle negative values for the bounds check (`pc >= 0`).

**Trace** for `[0, 3, 0, 1, -3]`:
```
pc=0: offset=0, jumps[0]→1, pc=0  (stay)
pc=0: offset=1, jumps[0]→2, pc=1  (forward 1)
pc=1: offset=3, jumps[1]→4, pc=4  (forward 3)
pc=4: offset=-3, jumps[4]→-2, pc=1  (back 3)
pc=1: offset=4, jumps[1]→5, pc=5  (escape!) → 5 steps
```

**Complexity**: O(n) where n = number of steps (374,269). Each step is O(1) — array index + increment + comparison.

---

## `solve_part2_with_data`

```rust
fn solve_part2_with_data(data: &[i32]) -> usize {
    let mut jumps = data.to_vec();
    let mut pc: i32 = 0;
    let mut steps = 0;
    let len = jumps.len() as i32;

    while pc >= 0 && pc < len {
        let offset = jumps[pc as usize];
        if offset >= 3 {
            jumps[pc as usize] -= 1;
        } else {
            jumps[pc as usize] += 1;
        }
        pc += offset;
        steps += 1;
    }
    steps
}
```

**Key difference**: The mutation rule changes — offsets ≥ 3 get decremented, creating a "braking" effect that keeps the program counter bouncing inside the list much longer.

**Why 74× more steps**: Part 1's always-increment rule quickly pushes offsets large enough to escape. Part 2's conditional decrement creates oscillating behavior — large offsets shrink back toward the threshold, keeping the program counter trapped. 374,269 → 27,720,699 steps.

**Complexity**: O(n) where n = 27,720,699 steps. Still O(1) per step.

---

## Benchmarks

| Function | Time |
|----------|------|
| `solve` (combined) | 34.09ms |

**Performance breakdown**: Part 2 dominates (~95% of runtime) due to 74× more iterations. Part 1 is negligible by comparison.

**Why no optimization needed**: 34ms is well under the 100ms budget. The simulation is inherently sequential — each step depends on the previous state, so parallelization isn't applicable. The only potential optimization would be SIMD or cache-line tuning, but the simple loop is already cache-friendly (sequential array access).

---

## Key Patterns

### In-place mutation simulation

The core pattern: maintain mutable state (the jump array), read from it, modify it, advance. This is a common AoC pattern for cellular automata, instruction execution, and state machines. The `to_vec()` clone at the start preserves the parse-once invariant — both parts get their own mutable copy from the shared parsed data.

### Bounds checking as termination condition

`while pc >= 0 && pc < len` is the natural termination for unbounded simulations. Using `i32` for both offsets and program counter keeps the arithmetic consistent and avoids unsigned underflow issues.

### Conditional mutation changing problem complexity

Part 1 and Part 2 differ by exactly one branch (`if offset >= 3`), yet the step count changes by 74×. This demonstrates how a small rule change can dramatically affect convergence behavior — a theme that recurs throughout AoC (e.g., rule variations in Game of Life, Langton's Ant).
