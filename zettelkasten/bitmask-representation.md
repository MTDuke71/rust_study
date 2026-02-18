# Bitmask Representation

**Type**: Algorithm Pattern
**First Encountered**: AoC 2022 Day 3 (Rucksack Reorganization)
**Related**: [[sos-dp]] | [[xor-properties]] | [[sliding-window-patterns]] | [[state-based-memoization]]

---

## Core Idea

Use individual bits of an integer to represent set membership, boolean states, or grid cells. A single integer becomes a compact, cache-friendly data structure where set operations reduce to single CPU instructions.

```
Bit position:  7  6  5  4  3  2  1  0
Value:         0  1  0  1  1  0  0  1    = 0x59
Meaning:       Items {0, 3, 4, 6} are present
```

**Why it works**: CPUs operate on 32/64/128-bit words natively. Bitwise AND, OR, XOR, and popcount are single-instruction operations. What takes O(n) with a HashSet takes O(1) with a bitmask.

---

## Fundamental Operations

### Set Operations as Bitwise Instructions

| Set Operation | Bitmask Equivalent | Example |
|---|---|---|
| Insert element `i` | `mask \|= 1 << i` | Add item 3: `0b0000 \|= 1 << 3` → `0b1000` |
| Remove element `i` | `mask &= !(1 << i)` | Remove item 3: `0b1000 &= !0b1000` → `0b0000` |
| Contains element `i` | `mask & (1 << i) != 0` | Check item 3: `0b1000 & 0b1000` → true |
| Toggle element `i` | `mask ^= 1 << i` | Flip item 3: XOR toggles ON↔OFF |
| Union (A ∪ B) | `a \| b` | Either set has item |
| Intersection (A ∩ B) | `a & b` | Both sets have item |
| Difference (A \ B) | `a & !b` | In A but not B |
| Complement | `!mask & valid_bits` | Everything not in set |
| Cardinality (\|A\|) | `mask.count_ones()` | Population count (popcnt instruction) |
| First element | `mask.trailing_zeros()` | Lowest set bit position |
| Is subset (A ⊆ B) | `a & b == a` | All bits of A are set in B |
| Is empty | `mask == 0` | No elements |

### Integer Type Capacity

| Type | Max Elements | Use Case |
|---|---|---|
| `u8` | 8 | Grid row (7 columns), small flags |
| `u16` | 16 | Valve sets, small graph nodes |
| `u32` | 32 | Alphabet (26 letters), moderate sets |
| `u64` | 64 | Chess bitboards, larger element sets |
| `u128` | 128 | Full ASCII range |

---

## Patterns in Practice

### Pattern 1: Set Membership (Character Sets)

**AoC 2022 Day 3** — Rucksack Reorganization

Represent item sets as bitmasks. Find common items via bitwise AND.

```rust
// Convert string to bitset: one bit per ASCII character
fn items_to_bitset(s: &str) -> u128 {
    s.bytes().fold(0u128, |acc, b| acc | (1u128 << b))
}

// Intersection = single AND instruction
let common = items_to_bitset(first) & items_to_bitset(second);
let item = common.trailing_zeros() as u8 as char;
```

**Performance**: 18x faster than HashSet (359us → 20us). The entire set operation compiles to a single `AND` instruction.

**Chess engine analogy**: Same technique as bitboards — u64 where bit N = piece at square N. "Which squares have both white pieces AND are attacked by opponent?" → `white_pieces & opponent_attacks`.

---

### Pattern 2: Rolling Bitset (Sliding Window)

**AoC 2022 Day 6** — Tuning Trouble

XOR toggles bits: entering character sets its bit, leaving character clears it. Uniqueness check via popcount.

```rust
fn find_marker(input: &[u8], window_size: usize) -> usize {
    let mut bits: u32 = 0;
    for (i, &b) in input.iter().enumerate() {
        bits ^= 1 << (b - b'a');                         // XOR in entering char
        if i >= window_size {
            bits ^= 1 << (input[i - window_size] - b'a'); // XOR out leaving char
        }
        if i >= window_size - 1 && bits.count_ones() as usize == window_size {
            return i + 1;  // All unique — each char appears exactly once
        }
    }
    panic!("No marker found");
}
```

**Key insight**: XOR is self-inverse. Character appearing odd times = bit ON, even times = bit OFF. If `count_ones() == window_size`, exactly `w` distinct characters each appear once.

**Complexity**: O(n) total — two XOR ops + one popcount per slide. No allocations.

See: [[xor-properties]], [[sliding-window-patterns]]

---

### Pattern 3: Grid Row Encoding

**AoC 2022 Day 17** — Pyroclastic Flow (Tetris)

Each row of a 7-wide chamber stored as a single `u8`. Collision detection and placement become bitwise operations.

```rust
struct Chamber {
    rows: Vec<u8>,  // Each row: 7 bits for 7 columns
    height: usize,
}

// Collision: is this cell occupied?
if self.rows[ny] & (1 << nx) != 0 { return true; }

// Placement: mark cell as occupied
self.rows[ny] |= 1 << nx;
```

```
|..##.#.| → 0b0101100 = 0x2C
|#####..| → 0b0011111 = 0x1F
|..####.| → 0b0111100 = 0x3C
+-------+
```

**Memory**: 1 byte per row instead of 7. Top-30 row profile (30 bytes) used as cycle detection fingerprint.

**Sprite extension**: Rock shapes could also be stored as per-row bitmasks, enabling whole-row collision checks via shift + AND instead of per-cell iteration.

---

### Pattern 4: State Tracking (DFS/DP)

**AoC 2022 Day 16** — Proboscidea Volcanium

Track which valves are opened during DFS. Bitmask enables subset enumeration and SOS DP.

```rust
// Check if valve i is open
if opened & (1 << i) != 0 { continue; }

// Open valve i
let new_opened = opened | (1 << i);

// Part 2: best[mask] = best pressure achievable with this valve set
// SOS DP propagates "best over any subset" in O(n × 2^n)
// Then: max(best[m] + best[full ^ m]) for all masks m
```

**Why bitmask**: 15 non-zero valves → 2^15 = 32,768 possible states. Fits in a `u32`. HashMap key with bitmask state is compact and hashable.

**Advanced**: SOS DP over bitmasks converts O(3^n) submask enumeration to O(n × 2^n). See [[sos-dp]].

---

### Pattern 5: Free Variable Enumeration

**AoC 2024 Day 24** — Logic Gates (Gaussian Elimination over GF(2))

Iterate all 2^n combinations of free variables using a bitmask counter.

```rust
for free_mask in 0..(1u64 << num_free) {
    // Extract bit i from free_mask
    let value = (free_mask >> i) & 1;
    // Try this combination...
}
```

**Pattern**: When you have N binary choices, a bitmask from `0` to `2^N - 1` enumerates every combination. Each bit position corresponds to one choice.

---

## When to Use Bitmasks

### Good Fit

- **Small element universe** (< 64 elements, ideally < 32)
- **Frequent set operations** (intersection, union, membership)
- **State space in DP/DFS** where state is "which items are active"
- **Fixed-width grids** where row width fits in an integer
- **Performance-critical inner loops** (HashSet → bitmask = 10-20x speedup typical)
- **Sliding windows** with character uniqueness checks

### Poor Fit

- **Large universe** (> 128 elements — need multiple integers or Vec<bool>)
- **Sparse membership** in huge domain (HashMap better)
- **Non-boolean state per element** (need more than 1 bit per item)
- **Readability priority** over performance (bitwise code is less obvious)

---

## Performance Characteristics

| Operation | Bitmask | HashSet | Speedup |
|---|---|---|---|
| Insert | O(1), ~1 ns | O(1) amortized, ~20 ns | ~20x |
| Contains | O(1), ~1 ns | O(1) avg, ~10 ns | ~10x |
| Intersection | O(1), single AND | O(min(n,m)) | ~n× |
| Union | O(1), single OR | O(n+m) | ~n× |
| Cardinality | O(1), popcnt | O(1), stored | ~1x |
| Memory | 1-16 bytes total | 48+ bytes overhead | ~10-50x |

**Real measurements** (AoC 2022 Day 3): HashSet 359us → Bitset 20us = **18x speedup**.

---

## Implementation Tips

### Choosing the Integer Type

```rust
// Match type to universe size
let flags: u8  = 0;  // Up to 8 elements (grid rows, small flags)
let flags: u32 = 0;  // Up to 32 elements (alphabet, valve sets)
let flags: u64 = 0;  // Up to 64 elements (chess boards)
let flags: u128 = 0; // Up to 128 elements (full ASCII)
```

### Common Idioms

```rust
// Iterate all set bits
let mut mask = 0b10110100u8;
while mask != 0 {
    let lowest_bit = mask.trailing_zeros();  // Position of lowest set bit
    // Process element at position `lowest_bit`
    mask &= mask - 1;  // Clear lowest set bit
}

// Enumerate all subsets of a mask
let mut sub = mask;
loop {
    // Process subset `sub`
    if sub == 0 { break; }
    sub = (sub - 1) & mask;  // Next subset
}

// All masks with exactly k bits set (combinations)
// Use Gosper's hack or itertools
```

### Avoid Common Pitfalls

```rust
// WRONG: Shifting by >= type width is undefined behavior in C, panic in Rust debug
let x: u32 = 1 << 32;  // panic in debug, wraps in release

// RIGHT: Check bounds or use wider type
let x: u64 = 1u64 << 32;  // Fine — u64 has 64 bits

// WRONG: Forgetting that bit 0 is the FIRST element
// If your elements are 1-indexed, either subtract 1 or waste bit 0

// RIGHT: Be consistent about indexing
let has_item_3 = mask & (1 << 3) != 0;  // 0-indexed: items 0,1,2,3,...
```

---

## AoC Usage Summary

| Year | Day | Type Width | Represents | Speedup |
|---|---|---|---|---|
| 2022 | Day 3 | `u128` | ASCII character sets | 18x vs HashSet |
| 2022 | Day 6 | `u32` | Rolling letter window (XOR) | 2.4x vs brute force |
| 2022 | Day 16 | `u32` | Opened valve set (DFS + SOS DP) | Enables O(n × 2^n) |
| 2022 | Day 17 | `u8` | Chamber row (7 columns) | 7x memory reduction |
| 2023 | Day 4 | shift | `1 << (n-1)` for 2^(n-1) points | — |
| 2024 | Day 24 | `u64` | Free variable enumeration (GF(2)) | — |

---

## Connection to Chess Programming

The chess engine connection (noted in Day 3) is worth remembering:

| Chess Bitboard | AoC Bitmask |
|---|---|
| `u64` — 64 squares | `u128` — 128 ASCII values |
| `white_pawns & black_attacks` = threatened pawns | `rucksack_a & rucksack_b` = common items |
| `popcount(attacks)` = number of attacked squares | `popcount(window)` = unique characters |
| Magic bitboards for move generation | SOS DP for subset propagation |
| LSB iteration for piece scanning | `trailing_zeros()` for element extraction |

Same fundamental technique, different domain. If you've worked with chess engines, you already know bitmasks.

---

## Key Takeaways

1. **Bitmask = compact set**: Each bit is a boolean "is element present?"
2. **CPU-native operations**: AND/OR/XOR/POPCNT are single instructions — no loops, no allocations
3. **Sweet spot**: < 32 elements. Beyond 64, consider alternatives.
4. **Set operations shine**: Intersection of two 128-element sets = one AND instruction
5. **State compression**: Perfect for DP/DFS where state is "which items are chosen"
6. **XOR for toggling**: Self-inverse property enables elegant rolling windows

---

*Tags: #bitmask #bit-manipulation #optimization #set-operations #algorithm-pattern #aoc*
*Links: [[sos-dp]] | [[xor-properties]] | [[sliding-window-patterns]] | [[state-based-memoization]] | [[AoC Patterns MOC]]*
