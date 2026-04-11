# Day 8: I Heard You Like Registers — Function Guide

**Problem**: Execute conditional register instructions and track the largest register value both at the end and at any point during execution.
**Answers**: Part 1 = **3,612**, Part 2 = **3,818**
**Code**: [day08.rs](../../src/solver/day08.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Data Structures](#data-structures)
3. [parse_input](#parse_input)
4. [check_condition](#check_condition)
5. [execute](#execute)
6. [Benchmarks](#benchmarks)
7. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: ~1,000 lines of conditional register instructions:
```
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
```

Each instruction: modify a register (`inc`/`dec` by an amount) only if a condition on another register is met. All registers start at 0.

**Part 1**: What is the **largest value in any register** after all instructions complete?

**Part 2**: What is the **largest value held in any register** at any point during execution?

**Example** (4 instructions):
- After all instructions: `a=1, c=-10` → Part 1: **1**
- During execution: `c` briefly reaches 10 before being decremented → Part 2: **10**

---

## Data Structures

### Instruction struct
```rust
struct Instruction<'a> {
    target: &'a str,     // register to modify
    delta: i64,          // +amount for inc, -amount for dec (pre-computed)
    cond_reg: &'a str,   // register to check
    cond_op: &'a str,    // comparison operator
    cond_val: i64,       // comparison value
}
```

Key design: `delta` pre-computes the sign during parsing (`dec 5` becomes `-5`, `inc -3` becomes `-3`). This eliminates branching during execution.

### Register map
```rust
HashMap<&str, i64>
```

Maps register name → current value. Uses `unwrap_or(&0)` for reads (default 0) and `entry().or_insert(0)` for writes.

---

## `parse_input`

```rust
fn parse_input(input: &str) -> Vec<Instruction<'_>>
```

Splits each line into 7 whitespace-separated tokens:
```
[0]target [1]op [2]amount if [4]cond_reg [5]cond_op [6]cond_val
```

**Pre-computation**: Converts `inc`/`dec` + amount into a single `delta` value during parsing, so `execute` only needs addition.

**Zero-copy**: Register names borrow from the input string (`&'a str`).

---

## `check_condition`

```rust
fn check_condition(reg_val: i64, op: &str, cond_val: i64) -> bool
```

Simple match on 6 comparison operators: `>`, `<`, `>=`, `<=`, `==`, `!=`. Each branch is a single comparison.

---

## `execute`

```rust
fn execute(instructions: &[Instruction<'_>]) -> (i64, i64)
```

Single-pass execution over all instructions:
1. Look up condition register value (default 0)
2. If condition passes, apply delta to target register
3. Track `all_time_max` after every successful modification
4. After all instructions, find `final_max` across all registers

Returns `(final_max, all_time_max)` — both parts solved in one pass.

**Complexity**: O(n) where n = number of instructions. Each instruction does O(1) HashMap operations.

---

## Benchmarks

| Function | Time |
|----------|------|
| `solve` (combined) | 164.98µs |

**Performance breakdown**: ~1,000 instructions parsed and executed. Dominated by HashMap string hashing for register lookups.

**Why no optimization needed**: 165µs is well under the 100ms budget. Could optimize with `FxHashMap` or pre-assigned register indices, but unnecessary.

---

## Key Patterns

### Pre-computed delta during parsing

Converting `inc`/`dec` to a signed delta during parsing moves branching from the hot loop (execute) to the cold path (parse). This is a general optimization: do work once at parse time rather than N times at execution time.

### Single-pass dual-answer

Both Part 1 and Part 2 are answered in a single execution loop. The `all_time_max` tracker adds negligible cost (one comparison per modification) but avoids a second pass. This is the parse-once pattern extended to solve-once.

### Register machine simulation

This is a common AoC pattern: simple instruction set, conditional execution, register file. The key insight is that the "register file" is just a `HashMap<&str, i64>` with default values of 0. No need to pre-discover all register names.
