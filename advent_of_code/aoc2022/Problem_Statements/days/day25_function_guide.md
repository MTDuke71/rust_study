# Day 25: Full of Hot Air - Function Guide

**Problem**: Convert between SNAFU (balanced base-5) and decimal, sum all fuel requirements, return result as SNAFU.

**Navigation**: [← Day 24](day24_function_guide.md) | [Problem](day25.md) | [Code](../../src/solver/day25.rs) | [Summary](../summary_2022.md)

---

## Overview

### Problem Summary
- **Part 1**: Sum all SNAFU numbers from the input, output the result as a SNAFU string.
- **Part 2**: No Part 2 (Day 25 tradition — collect all other stars to complete the year).

### Performance
- **Total**: **6.09µs** (Criterion verified) — 143 lines of simple arithmetic and string conversion.

### Key Insight
**SNAFU is balanced base-5.** Normal base-5 uses digits 0–4; SNAFU uses digits -2, -1, 0, 1, 2 (written `=`, `-`, `0`, `1`, `2`). This means every integer has a unique representation without a separate sign. The conversion trick: when a base-5 remainder is 3 or 4, "borrow" from the next place — digit becomes negative (`=` or `-`) and you carry +1 upward.

---

## Algorithm Analysis

### SNAFU Number System

Standard positional notation with base 5, but digit values are shifted:

| SNAFU digit | Value |
|-------------|-------|
| `=` | -2 |
| `-` | -1 |
| `0` | 0 |
| `1` | 1 |
| `2` | 2 |

Each position represents a power of 5 (right to left: 1, 5, 25, 125, 625, ...).

### SNAFU → Decimal

Process left-to-right with Horner's method:

```
accumulator = 0
for each digit:
    accumulator = accumulator * 5 + digit_value
```

Example: `2=-01` → `((((0×5 + 2)×5 + (-2))×5 + (-1))×5 + 0)×5 + 1 = 976`

This is identical to how you'd parse any base-N number, just with negative digit values.

### Decimal → SNAFU

This is the interesting direction. Standard base-5 conversion gives remainders 0–4, but SNAFU only allows -2 to 2. The fix: when the remainder exceeds 2, represent it as a negative digit plus a carry:

```
while n != 0:
    remainder = n % 5
    n = n / 5
    if remainder <= 2:
        emit digit for remainder (0, 1, or 2)
    if remainder == 3:
        emit '='  (value -2, since 3 = 5×1 + (-2))
        carry +1 to n
    if remainder == 4:
        emit '-'  (value -1, since 4 = 5×1 + (-1))
        carry +1 to n
```

Digits are emitted least-significant first, then reversed.

### Why the Carry Works

The key identity:
- `3 = 5 × 1 + (-2)` → digit `=` with carry 1
- `4 = 5 × 1 + (-1)` → digit `-` with carry 1

By adding 1 to the next position (the carry), we've effectively "borrowed" 5 from the next place to make the current digit negative. This is analogous to how subtraction borrows in decimal, but it's baked into every step of the conversion.

### Worked Example

Convert decimal `4890` to SNAFU:

```
4890 ÷ 5 = 978 rem 0 → digit '0'
 978 ÷ 5 = 195 rem 3 → digit '=' (carry +1 → 196)
 196 ÷ 5 =  39 rem 1 → digit '1'
  39 ÷ 5 =   7 rem 4 → digit '-' (carry +1 → 8)
   8 ÷ 5 =   1 rem 3 → digit '=' (carry +1 → 2)
   2 ÷ 5 =   0 rem 2 → digit '2'

Reverse: 2 = - 1 = 0 → "2=-1=0" ✓
```

Verify: `2×3125 + (-2)×625 + (-1)×125 + 1×25 + (-2)×5 + 0×1 = 6250 - 1250 - 125 + 25 - 10 + 0 = 4890` ✓

---

## Implementation Details

### Data Flow
```
Input → lines → map(snafu_to_decimal) → sum → decimal_to_snafu → Part 1
                                                                    Part 2 = "Merry Christmas!"
```

### Function Signatures

| Function | Input | Output | Purpose |
|----------|-------|--------|---------|
| `snafu_digit` | `char` | `i64` | Map SNAFU character to value (-2..2) |
| `snafu_to_decimal` | `&str` | `i64` | Horner's method: left-to-right fold with base 5 |
| `decimal_to_snafu` | `i64` | `String` | Repeated div-mod with carry for remainders > 2 |
| `solve` | `&str` | `(String, String)` | Sum all SNAFU numbers, convert back |

### Key Design Decisions

**Why `fold` for SNAFU→decimal instead of `enumerate` with powers?**
Horner's method (`acc * 5 + digit`) processes left-to-right with no need to compute `5.pow(n)`. It's both simpler and avoids potential overflow from large powers.

**Why `i64` instead of `u64`?**
SNAFU digits can be negative, so intermediate values during conversion could theoretically be negative. Using `i64` avoids sign issues throughout.

**Why build digits in reverse then collect?**
The div-mod algorithm naturally produces the least-significant digit first. Pushing to a Vec and reversing is simpler and faster than pre-allocating in the correct order.

---

## Complexity Analysis

| Component | Complexity | Notes |
|-----------|------------|-------|
| SNAFU → decimal | O(d) | d = number of digits (~20 max) |
| Decimal → SNAFU | O(d) | d = digits in result |
| Sum all numbers | O(n × d) | n = 143 lines, d ≈ 20 |
| Total | O(n × d) | ~2,860 character operations |

This is about as simple as AoC gets algorithmically — pure number conversion with no search, simulation, or optimization needed.

---

## Edge Cases

- **Zero**: Returns `"0"` (special-cased since the while loop wouldn't execute)
- **Carry propagation**: A chain of 3s or 4s can propagate carries multiple positions (e.g., decimal 12 = SNAFU `22`, decimal 13 = SNAFU `1==`)
- **No Part 2**: Day 25 always has a free star for completing all other days
- **Empty lines**: Filtered out with `.filter(|l| !l.is_empty())`

---

## Mathematical Connection: Balanced Number Systems

SNAFU is a **balanced quinary** (base-5) system. Balanced representations exist for any odd base:
- **Balanced ternary** (base 3): digits -1, 0, 1 — used in some Soviet-era computers (Setun)
- **Balanced quinary** (base 5): digits -2, -1, 0, 1, 2 — SNAFU
- **Balanced septenary** (base 7): digits -3, -2, -1, 0, 1, 2, 3

The advantage: no need for a separate sign — negative numbers naturally have leading negative digits (e.g., decimal -1 = SNAFU `-`, decimal -7 = SNAFU `-2`).

The carry-on-remainder technique generalizes: for any balanced base `b`, if remainder > `b/2`, emit `remainder - b` and carry +1.

---

## Key Takeaways

1. **Balanced number systems** allow representing negatives without a sign bit — each digit can be negative, positive, or zero. The trade-off is slightly more complex conversion logic.
2. **Carry propagation for out-of-range remainders** is the core trick — when `rem > max_digit`, subtract the base and carry +1. This is analogous to decimal borrowing in subtraction.
3. **Horner's method** (`fold` with `acc * base + digit`) is the cleanest way to evaluate any positional number system, regardless of the digit values.
4. **Day 25 tradition**: A gentle finale that tests understanding of number representations rather than algorithmic complexity.

---

**Answer**: Part 1: `2=-0=01----22-0-1-10` | Part 2: Merry Christmas!

**Related patterns**: [[math-foundations/base-conversion]] | [[math-foundations/number-theory-basics]]
