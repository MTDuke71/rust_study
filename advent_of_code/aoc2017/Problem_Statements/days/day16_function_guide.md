# Day 16: Permutation Promenade — Function Guide

**Problem**: Apply dance moves (spin, exchange, partner) to 16 programs (`a`–`p`). Repeat the dance 1 billion times.
**Answers**: Part 1 = **olgejankfhbmpidc**, Part 2 = **gfabehpdojkcimnl**
**Code**: [day16.rs](../../src/solver/day16.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [parse_input](#parse_input)
3. [dance](#dance)
4. [solve_part1_with_data](#solve_part1_with_data)
5. [solve_part2_with_data](#solve_part2_with_data)
6. [Optimization: History Lookup vs Recompute](#optimization-history-lookup-vs-recompute)
7. [Benchmarks](#benchmarks)
8. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: comma-separated dance moves (~10,000 moves).

Three move types:
- **Spin** `sX` — rotate all programs right by X positions
- **Exchange** `xA/B` — swap programs at positions A and B
- **Partner** `pA/B` — swap programs named A and B (regardless of position)

Starting order: `abcdefghijklmnop` (16 programs).

- **Part 1**: Apply all moves once, report final order.
- **Part 2**: Apply the entire dance sequence 1,000,000,000 times. Report final order.

---

## `parse_input`

```rust
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

fn parse_input(input: &str) -> Vec<Move>
```

- Splits on commas, matches on the first byte (`s`, `x`, `p`).
- Partner stores raw `u8` bytes (e.g., `b'a'`, `b'p'`) — avoids char conversion overhead.
- Exchange parses the two position numbers from `A/B` via `split_once('/')`.

---

## `dance`

```rust
fn dance(programs: &mut [u8; 16], moves: &[Move])
```

Applies all moves in sequence to a fixed-size `[u8; 16]` array:

- **Spin**: Copy into a buffer with rotation — `buf[..n] = programs[16-n..]`, `buf[n..] = programs[..16-n]`. Stack-allocated `[0u8; 16]` buffer, no heap allocation.
- **Exchange**: Direct `programs.swap(a, b)`.
- **Partner**: Linear scan with `.position()` to find both names, then swap. O(16) per partner move — fine for 16 elements.

---

## `solve_part1_with_data`

```rust
fn solve_part1_with_data(moves: &[Move]) -> String
```

Straightforward: start with `abcdefghijklmnop`, call `dance()` once, convert to string.

---

## `solve_part2_with_data`

```rust
fn solve_part2_with_data(moves: &[Move]) -> String
```

**Key insight**: Any permutation has finite order — repeatedly applying it must eventually cycle back to the start. For our input, the cycle length is **60 dances**.

Algorithm:
1. Store every intermediate state in a `Vec<[u8; 16]>` during cycle detection
2. Dance repeatedly until programs return to `abcdefghijklmnop`
3. Index directly: `history[1_000_000_000 % 60]` — zero repeated dances needed

Storage cost: 60 × 16 bytes = **960 bytes** for the entire history table.

---

## Optimization: History Lookup vs Recompute

The naive approach after finding the cycle would recompute `1B % 60 = 40` dances from scratch. Storing the history during cycle detection eliminates this entirely.

| Approach | Time | Memory | Repeat Dances |
|----------|------|--------|---------------|
| Recompute remainder | 8.18ms | 0 bytes | 40 dances |
| History lookup | 4.82ms | 960 bytes | 0 dances |

**41% speedup** for 960 bytes — a classic space-time tradeoff.

---

## Benchmarks

| Metric | Value |
|--------|-------|
| **Combined** | 4.82ms |
| **Parse** | ~10k moves from comma-separated string |
| **Part 1** | Single dance application |
| **Part 2** | 60 dances (cycle detection) + O(1) lookup |

---

## Key Patterns

### Permutation Cycle Detection
Every permutation has finite order (group theory). For 16 elements, the maximum possible order is `lcm(1,2,...,16) = 720720`, but real dances cycle much faster. Our input cycles in 60.

### Fixed-Size Array Over Vec
Using `[u8; 16]` instead of `Vec<u8>`:
- Stack-allocated, no heap
- Direct equality comparison (`==`) for cycle detection
- Copy semantics for storing history snapshots

### Byte-Level String Manipulation
Programs stored as `u8` bytes (`b'a'`–`b'p'`), not `char`. Only converted to `String` at the very end via `String::from_utf8()`. This avoids UTF-8 encoding/decoding during the hot loop.
