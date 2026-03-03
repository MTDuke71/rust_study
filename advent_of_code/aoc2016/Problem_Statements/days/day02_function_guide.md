# Day 2: Bathroom Security — Function Guide

**Problem**: Navigate a keypad with UDLR instructions. Each line produces one digit of the bathroom code.
**Answers**: Part 1 = **69642**, Part 2 = **8CB23**
**Code**: [day02.rs](../../src/solver/day02.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Data Structures](#data-structures)
3. [parse_input](#parse_input)
4. [decode](#decode)
5. [solve_part1_with_data / solve_part2_with_data](#solve_part1_with_data--solve_part2_with_data)
6. [Benchmarks](#benchmarks)
7. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: Multiple lines of UDLR instructions (e.g., `ULL`, `RRDDD`, `LURDL`).
Each line: starting from the previous button, follow the directions on a keypad. The button you land on at the end of each line is the next digit of the code. Moves that would leave the keypad or land on an empty cell are ignored.

**Part 1**: Standard 3×3 keypad (`1-9`), start at `5`. What is the bathroom code?

```
1 2 3
4 5 6
7 8 9
```

**Part 2**: Diamond-shaped 5×5 keypad (`1-9, A-D`), start at `5`. Same instructions, different layout.

```
    1
  2 3 4
5 6 7 8 9
  A B C
    D
```

**Why Part 2 is harder**: The keypad is no longer rectangular — it's a diamond with empty cells (`.`). The bounds-checking logic must handle irregular shapes, not just row/column limits.

---

## Data Structures

### Keypad layouts with sentinel border

```rust
// Sentinel border: '.' ring eliminates all bounds/negative checks.
// Same idea as the 12×10 mailbox in chess programming.
const KEYPAD1: &[&[u8]] = &[
    b".....",
    b".123.",
    b".456.",
    b".789.",
    b".....",
];

const KEYPAD2: &[&[u8]] = &[
    b".......",
    b"...1...",
    b"..234..",
    b".56789.",
    b"..ABC..",
    b"...D...",
    b".......",
];
```

Both keypads are `&[&[u8]]` — a slice of byte-string slices, surrounded by a **sentinel border** of `b'.'` cells. This is the same technique as the **12×10 mailbox** in chess programming: surround the board with illegal squares so move generation never needs bounds checks.

**Why the border works**: Since you're never standing on a `'.'` cell, moving 1 step in any direction always lands inside the padded grid. The only check needed is `keypad[nr][nc] != b'.'` — no negative checks, no length checks, no type conversions.

**Why `u8` instead of `char`?**: Byte operations avoid UTF-8 decoding overhead. All keypad labels are ASCII, so `u8` is sufficient. The final `as char` conversion happens only once per output digit.

---

## `parse_input`

```rust
fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}
```

**Input**: Raw puzzle input with one instruction line per row.
**Output**: `Vec<&str>` — borrowed slices, no allocation of string content.

Minimal parsing — the actual character-by-character processing happens in `decode`.

---

## `decode`

```rust
fn decode(keypad: &[&[u8]], start: (usize, usize), lines: &[&str]) -> String
```

The core algorithm. Shared by both parts — only the keypad layout and start position differ.

### Walk logic

```rust
for line in lines {
    for &ch in line.as_bytes() {
        let (nr, nc) = match ch {
            b'U' => (r - 1, c),
            b'D' => (r + 1, c),
            b'L' => (r, c - 1),
            b'R' => (r, c + 1),
            _ => continue,
        };
        if keypad[nr][nc] != b'.' {
            r = nr;
            c = nc;
        }
    }
    code.push(keypad[r][c] as char);
}
```

**One check to rule them all**: The sentinel border collapses what was previously a 3-layer validation into a single `!= b'.'` test:

| Before (no border) | After (sentinel border) |
|---------------------|------------------------|
| 1. `nr >= 0 && nc >= 0` (negative check) | *Impossible* — never on row/col 0 |
| 2. `nr < len && nc < len` (bounds check) | *Impossible* — border absorbs overflow |
| 3. `keypad[nr][nc] != b'.'` (empty cell) | **This is the only check needed** |

**No type conversions**: All arithmetic stays in `usize`. Subtraction like `r - 1` can never underflow because the border guarantees `r >= 1` for any valid position.

### Trace on example (Part 1, KEYPAD1)

```
Start:    r=2, c=2 → '5'    (row/col +1 due to border)

Line "ULL":
  U: (2,2)→(1,2) = '2' ✓
  L: (1,2)→(1,1) = '1' ✓
  L: (1,1)→(1,0) = '.' ✗ → stay '1'   ← border absorbs the move
  → Output: '1'

Line "RRDDD":
  R: (1,1)→(1,2) = '2' ✓
  R: (1,2)→(1,3) = '3' ✓
  D: (1,3)→(2,3) = '6' ✓
  D: (2,3)→(3,3) = '9' ✓
  D: (3,3)→(4,3) = '.' ✗ → stay '9'   ← border absorbs the move
  → Output: '9'

Result: "1985" ✓
```

**Complexity**: O(L) time where L = total characters across all lines, O(1) space (excluding the output string).

---

## `solve_part1_with_data` / `solve_part2_with_data`

```rust
fn solve_part1_with_data(lines: &[&str]) -> String {
    decode(KEYPAD1, (2, 2), lines) // start at '5'
}

fn solve_part2_with_data(lines: &[&str]) -> String {
    decode(KEYPAD2, (3, 1), lines) // start at '5'
}
```

Both are one-liners delegating to `decode` with the appropriate keypad and start position. Coordinates are offset by +1 from the logical position due to the sentinel border.

- **Part 1**: `(2, 2)` is `KEYPAD1[2][2]` = `'5'` (center of 3×3, border shifts from logical `(1,1)`)
- **Part 2**: `(3, 1)` is `KEYPAD2[3][1]` = `'5'` (left edge of middle row, border shifts from logical `(2,0)`)

---

## `solve` (Public API — Parse-Once)

```rust
pub fn solve(input: &str) -> (String, String) {
    let data = parse_input(input);
    (solve_part1_with_data(&data), solve_part2_with_data(&data))
}
```

Parses **once**, passes the same `Vec<&str>` to both parts.

---

## Benchmarks

| Function | Time |
|----------|------|
| `solve_part1` | 6.7µs |
| `solve_part2` | 9.8µs |
| `solve` (combined) | 20.3µs |

**Combined ≈ sum of parts** — expected since parsing is trivial (just `lines().collect()`). The cost is dominated by the character-by-character keypad walks.

**Why Part 2 is ~1.5× slower than Part 1**: The 7×7 padded keypad is larger than the 5×5, with more `'.'` cells to reject. The diamond shape means more moves land on sentinels and get discarded, adding slightly more branch work per instruction line.

**At 20µs total, well under the 100ms target — no optimization needed.**

---

## Key Patterns

### Sentinel border (mailbox technique)

```
Without border:          With border:
  1 2 3                  . . . . .
  4 5 6        →         . 1 2 3 .
  7 8 9                  . 4 5 6 .
                         . 7 8 9 .
                         . . . . .
```

A 1-cell ring of `b'.'` surrounds the real keypad. This eliminates all bounds checking — the same technique used in chess programming's **12×10 mailbox** representation, where the 8×8 board is surrounded by illegal squares so move generation never needs edge tests.

**Before** (3 checks, type conversions):
```rust
let nr = r as i32 + dr;           // usize → i32
let nc = c as i32 + dc;
if nr >= 0 && nc >= 0 {           // check 1: negative
    let (nr, nc) = (nr as usize, nc as usize);  // i32 → usize
    if nr < keypad.len() && nc < keypad[nr].len()  // check 2: bounds
        && keypad[nr][nc] != b'.' {                // check 3: empty
```

**After** (1 check, pure `usize`):
```rust
let (nr, nc) = (r - 1, c);       // pure usize arithmetic
if keypad[nr][nc] != b'.' {      // the only check needed
```

The border costs a few extra bytes of static data (3×3 → 5×5, 5×5 → 7×7) but buys simpler, branchless-friendly code.

### Generic keypad via `&[&[u8]]`

Both keypads use the same representation: 2D byte arrays with `'.'` for both empty cells *and* the sentinel border. This lets one `decode` function serve both parts — the keypad shape is data, not code. Adding a third keypad layout would require zero code changes.

### Move-then-validate pattern

```rust
let (nr, nc) = match ch {
    b'U' => (r - 1, c),  // compute candidate
    // ...
};
if keypad[nr][nc] != b'.' {
    r = nr;               // accept move
    c = nc;
}
// else: silently stay put
```

Compute the candidate position first, then validate. If invalid, the current position is unchanged. This avoids pre-checking each direction with separate logic — one validation path handles all four directions uniformly.
