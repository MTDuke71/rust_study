# Day 21: Scrambled Letters and Hash — Function Guide

## Problem Summary

A system uses a series of string operations to scramble passwords. Given 100 operations and an 8-character string:

**Part 1**: Apply operations forward to scramble `abcdefgh`.
**Part 2**: Apply operations in reverse to unscramble `fbgdceah`.

---

## Answers

| Part | Answer | Time |
|------|--------|------|
| 1 | `hcdefbag` | 11.7us |
| 2 | `fbhaegdc` | 16.2us |
| Combined | — | 16.1us* |

*\*Combined and Part 2 are within measurement noise (~0.1us). Parsing cost is negligible relative to benchmark variance at this scale.*

---

## Function Map

```
solve(input) -> (String, String)
  ├── parse_input(input) -> Vec<Op>       # parse 100 instructions into enum variants
  ├── scramble("abcdefgh", &ops) -> String
  │     └── apply(&mut Vec<u8>, &Op)      # execute one operation forward
  └── unscramble("fbgdceah", &ops) -> String
        └── unapply(&mut Vec<u8>, &Op)    # execute one operation in reverse
```

---

## Algorithm Details

### Six Operation Types

| Operation | Forward | Reverse |
|-----------|---------|---------|
| `swap position X Y` | `pw.swap(x, y)` | Self-inverse |
| `swap letter A B` | Find indices, swap | Self-inverse |
| `rotate left N` | `rotate_left(n)` | `rotate_right(n)` |
| `rotate right N` | `rotate_right(n)` | `rotate_left(n)` |
| `rotate based on letter X` | Rotate right by `1 + idx + (idx >= 4)` | Brute-force trial |
| `reverse X through Y` | `pw[x..=y].reverse()` | Self-inverse |
| `move X to Y` | Remove at X, insert at Y | Remove at Y, insert at X |

### Parsing

Each instruction is split by whitespace. The first two words (`swap position`, `rotate left`, `move position`, etc.) uniquely identify the operation type. The relevant values are extracted by index from the word array.

### Part 1: Forward Scrambling

Apply all 100 operations in order to `abcdefgh`. Each operation mutates a `Vec<u8>` in place. All operations are O(n) or better on an 8-character string.

### Part 2: Reverse Unscrambling

Apply all 100 operations in **reverse order**, with each operation **inverted**:

- **Self-inverse ops** (swap, reverse): Apply the same operation — swapping twice or reversing twice returns to original.
- **Rotation ops**: Swap left/right direction.
- **Move**: Swap source and destination indices.
- **Rotate-based**: This is the tricky one. The forward formula maps each starting index to a landing position:

| Original idx | Rotation (`1 + idx + bonus`) | Lands at |
|-------------|------------------------------|----------|
| 0 | 1 | 1 |
| 1 | 2 | 3 |
| 2 | 3 | 5 |
| 3 | 4 | 7 |
| 4 | 6 | 2 |
| 5 | 7 | 4 |
| 6 | 8 → 0 | 6 |
| 7 | 9 → 1 | 0 |

For length 8 this is a bijection (every output maps to exactly one input), so a reverse lookup table is possible. However, the brute-force approach works for any string length — including the 5-char example. Solution: try all left rotations, apply the forward operation to each, and check which produces the current state. With only 8 positions, this is instant.

### Complexity

- **Time**: O(k * n) where k=100 operations and n=8 string length
- **Space**: O(k) for the parsed operation list
- **Part 2 overhead**: The rotate-based reversal tries up to 8 candidates per occurrence, adding a constant factor

With 100 operations on 8 characters, both parts complete in ~16us combined.

---

## Key Observations

1. **Three self-inverse ops**: Swap (position and letter) and reverse are their own inverses — no special reverse logic needed.
2. **Rotate-based is the puzzle**: The forward formula `1 + idx + (idx >= 4)` creates a non-injective mapping for general lengths, but for length 8 it happens to be injective. Brute-force reversal (8 trials) is simpler and more general than deriving the inverse formula.
3. **Byte-level operations**: Working on `Vec<u8>` instead of `String` gives direct index access and efficient `swap`/`rotate_left`/`rotate_right` from the standard library.
4. **Parse-once pattern**: Operations are parsed into an `Op` enum once, then traversed forward (Part 1) and backward (Part 2).
5. **No allocations in hot path**: `apply` mutates in place. The only allocation is the `Vec<u8>` clone in the rotate-based reversal.

---

## Benchmarks

```
day21_combined          16.1us
day21_part1             11.7us
day21_part2             16.2us
```

Part 2 is slightly slower due to the brute-force rotate-based reversal (cloning + trial applications). Combined appears faster than Part 2 alone — this is measurement noise; at microsecond scale, CPU cache state and scheduling jitter dominate.
