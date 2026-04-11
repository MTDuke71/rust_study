# Day 9: Stream Processing — Function Guide

**Problem**: Parse a stream of nested groups `{}` and garbage `<>` with `!` escape sequences to compute group scores and count garbage characters.
**Answers**: Part 1 = **7,640**, Part 2 = **4,368**
**Code**: [day09.rs](../../src/solver/day09.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Data Structures](#data-structures)
3. [parse_stream](#parse_stream)
4. [Benchmarks](#benchmarks)
5. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: A single line of ~16K characters containing nested groups and garbage.

Three character classes interact:
- **Groups**: `{` opens, `}` closes. Nestable. Score = nesting depth.
- **Garbage**: `<` opens, `>` closes. Everything inside (except `!` and `>`) is garbage content.
- **Escape**: `!` cancels the next character unconditionally (inside or outside garbage).

**Part 1**: What is the **total score** of all groups? (Each group scores its nesting depth.)

**Part 2**: How many **non-canceled characters** appear within garbage?

**Examples** (Part 1 — score):
```
{}              → 1
{{{}}}          → 1+2+3 = 6
{{},{}}         → 1+2+2 = 5
{{{},{},{{}}}}  → 1+2+3+3+3+4 = 16
```

**Examples** (Part 2 — garbage count):
```
<>                    → 0 (empty garbage)
<random characters>   → 17
<<<<>                 → 3 (first three < are garbage content)
<{!>}>                → 2 ({ and })
<!!>                  → 0 (both ! cancel each other)
```

---

## Data Structures

### StreamResult struct
```rust
struct StreamResult {
    total_score: u32,    // sum of all group depths
    garbage_count: u32,  // non-canceled characters inside garbage
}
```

Minimal structure — both answers computed in a single pass, stored together. No intermediate representation of the stream is needed.

---

## `parse_stream`

```rust
fn parse_stream(input: &str) -> StreamResult
```

A single-pass state machine with three states tracked by two booleans:

| State | `in_garbage` | `skip_next` | Behavior |
|-------|-------------|-------------|----------|
| Normal | `false` | `false` | `{` → depth++, `}` → score += depth then depth--, `<` → enter garbage |
| In Garbage | `true` | `false` | `>` → exit garbage, `!` → set skip, anything else → garbage_count++ |
| Escaped | either | `true` | Unconditionally skip one character, clear skip flag |

**Key insight**: The `skip_next` flag is checked first, making `!` work identically inside and outside garbage. The state machine processes each character exactly once with no lookahead or backtracking.

**Complexity**: O(n) time, O(1) space — just 4 integer/boolean variables.

---

## Benchmarks

| Function | Time |
|----------|------|
| `solve` (combined) | 9.17µs |

**Performance breakdown**: ~16K characters processed in a single pass. No allocations, no data structures beyond local variables. This is essentially the speed of iterating over characters.

**Why so fast**: No parsing into an intermediate representation. No HashMap, no Vec, no tree. Just a character-by-character state machine with O(1) work per character.

---

## Key Patterns

### State machine parsing

When input is a structured stream with modes (normal vs garbage) and escape sequences, a state machine is the natural approach. Each character is processed exactly once, and the current state determines how to interpret it. This avoids the complexity of recursive descent or regex-based parsing.

The state transitions form a simple graph:
```
Normal --'<'--> InGarbage --'>'--> Normal
   |                |
   '!'              '!'
   |                |
   v                v
 Skip             Skip
```

### Dual-answer single pass

Both Part 1 (group scores) and Part 2 (garbage count) are accumulated in the same character loop. The `depth` tracker for scoring and the `garbage_count` for Part 2 are independent counters updated in different states — zero interference, zero overhead.

### No intermediate representation

Unlike most AoC days where parsing builds a `Vec<T>` or `HashMap`, this problem is solved directly during parsing. The `StreamResult` struct holds only the final answers. This is the ideal case: when the input is consumed left-to-right and no random access or re-examination is needed.
