# Day 17: Pyroclastic Flow - Function Guide

**Problem**: Simulate Tetris-like rocks falling in a 7-wide chamber, pushed by hot gas jets.

**Navigation**: [← Day 16](day16_function_guide.md) | [Problem](day17.md) | [Code](../../src/solver/day17.rs) | [Summary](../summary_2022.md) | [Day 18 →](day18_function_guide.md)

---

## Overview

### Problem Summary
- **Part 1**: How tall is the tower after 2,022 rocks have landed?
- **Part 2**: How tall after 1,000,000,000,000 rocks? (Cycle detection required)

### Performance
- **Parse**: 6.05µs
- **Part 1**: 341µs (simulate 2,022 rocks)
- **Part 2**: 400µs (simulate ~1,700 rocks + cycle math)
- **Combined**: 734µs ✅ Parse-once verified (734 < 341 + 400)

### Key Insight
**Part 2 can't be brute-forced** — a trillion rocks would take years. But the simulation is deterministic: same rock type + same jet position + same tower top shape = same future behavior. Detect one cycle and extrapolate.

---

## Algorithm Analysis

### The 5 Rock Shapes

```
Shape 0:  ####      Shape 1:  .#.     Shape 2:  ..#
                              ###               ..#
                              .#.               ###

Shape 3:  #         Shape 4:  ##
          #                   ##
          #
          #
```

Each shape is stored as `(x, y)` offsets from a bottom-left anchor point. Shapes cycle: 0, 1, 2, 3, 4, 0, 1, 2, ...

### Chamber Representation: Row Bitmasks

Each row of the 7-wide chamber is a single `u8` bitmask:

```
|..##.#.| → 0b0101100 = 0x2C
|#####..| → 0b0011111 = 0x1F
|..####.| → 0b0111100 = 0x3C
+-------+
```

Bit 0 = leftmost column, bit 6 = rightmost. This gives:
- **Collision detection**: `row & (1 << x) != 0` — single bitwise AND
- **Placement**: `row |= 1 << x` — single bitwise OR
- **Memory**: 1 byte per row instead of 7 bytes/chars

### Simulation Loop

For each rock:
1. **Spawn**: left edge at x=2, bottom at `height + 3` (3 units above the tower)
2. **Alternate**: jet push (left/right), then fall down
3. **Collision check**: If push would hit wall/rock, ignore it. If fall would hit floor/rock, place and stop.

```rust
loop {
    // Jet push (skip if would collide)
    let dx = jets[jet_idx % jets.len()];
    jet_idx += 1;
    if !chamber.is_blocked(x + dx, y, rock) {
        x += dx;
    }

    // Fall (place if would collide below)
    if chamber.is_blocked(x, y - 1, rock) {
        chamber.place(x, y, rock);
        break;
    }
    y -= 1;
}
```

### Collision Detection

Checks all cells of a rock shape against walls, floor, and existing rocks:

```rust
fn is_blocked(&self, x: i64, y: i64, rock: &[(usize, usize)]) -> bool {
    for &(dx, dy) in rock {
        let nx = x + dx as i64;
        let ny = y + dy as i64;
        // Wall check
        if nx < 0 || nx >= 7 || ny < 0 { return true; }
        // Rock collision check
        if ny < self.rows.len() && self.rows[ny] & (1 << nx) != 0 {
            return true;
        }
    }
    false
}
```

---

### Part 2: Cycle Detection

**State fingerprint**: `(rock_type, jet_index % jet_count, top_30_rows)`

If we see the same fingerprint twice, everything from this point will repeat identically.

```
Rock 0:    height = 0      ─┐
...                          │ Pre-cycle
Rock 142:  height = 220     ─┘ ─┐
...                               │ Cycle (len=1735, height_gain=2681)
Rock 1877: height = 2901    ─┘ ─┐
...                               │ Same cycle repeats
Rock 3612: height = 5582    ─┘
```

**The math**:
```rust
let cycle_len = rock_count - prev_rock;           // rocks per cycle
let cycle_height = height - prev_height;           // height gained per cycle
let remaining = num_rocks - rock_count;            // rocks still needed
let full_cycles = remaining / cycle_len;           // how many cycles fit
extra_height = full_cycles * cycle_height;         // skip ahead
rock_count += full_cycles * cycle_len;             // advance counter
// Simulate the leftover rocks normally
```

For the real input, Part 2 only simulates ~3,400 rocks total (pre-cycle + one cycle + remainder), not a trillion.

### Why Top-30 Rows?

The top profile needs to capture enough state that identical profiles guarantee identical futures. 30 rows is conservative — the tallest rock is 4 rows, and jet influence reaches at most a few rows down. In practice, even 20 rows would work, but 30 provides a safety margin.

---

## Implementation Details

### Coordinate System

```
y ↑
  |  (0,3) (1,3) (2,3) ... (6,3)
  |  (0,2) (1,2) (2,2) ... (6,2)
  |  (0,1) (1,1) (2,1) ... (6,1)
  |  (0,0) (1,0) (2,0) ... (6,0)  ← floor is y < 0
  +────────────────────────────── x →
```

- y=0 is the bottom row of the chamber
- x=0 is the left wall
- Rocks use `i64` coordinates (allow negative values during collision checks)

### Rock Placement

```rust
fn place(&mut self, x: i64, y: i64, rock: &[(usize, usize)]) {
    for &(dx, dy) in rock {
        let ny = (y + dy as i64) as usize;
        while self.rows.len() <= ny {
            self.rows.push(0);  // Extend chamber upward
        }
        self.rows[ny] |= 1 << (x + dx as i64);
        self.height = self.height.max(ny + 1);
    }
}
```

The `rows` Vec grows dynamically as the tower gets taller. Pre-allocated with capacity 4096 to avoid early reallocations.

### Parse-Once Pattern

Jets are parsed once and shared between Part 1 and Part 2:

```rust
pub fn solve(input: &str) -> (u64, u64) {
    let jets = parse_input(input);         // Parse ONCE
    (
        solve_part1_with_data(&jets),      // Simulate 2,022 rocks
        solve_part2_with_data(&jets),      // Simulate with cycle detection
    )
}
```

Note: Unlike many AoC problems where both parts share simulation state, here Part 1 and Part 2 run independent simulations. The shared work is only the parsing (6µs), so the parse-once savings are modest but the structure is correct.

---

## Performance Analysis

### Benchmark Results

```
day17_parse:     6.05µs  ← Convert '<'/'>' to -1/+1
day17_part1:     341µs   ← 2,022 rocks, no cycle detection
day17_part2:     400µs   ← ~3,400 rocks + cycle detection + HashMap
day17_combined:  734µs   ← Parse once + both parts
```

Parse-once verification: 734µs < 341µs + 400µs = 741µs ✅

### Why Part 2 is Only ~17% Slower Than Part 1

Part 2 simulates more rocks (~3,400 vs 2,022) but the overhead of:
- HashMap lookups for cycle detection
- `top_profile()` Vec allocations every rock
- Cycle math

...is relatively small. The simulation loop itself dominates.

### Complexity Summary

| Component | Complexity | Notes |
|-----------|-----------|-------|
| Parsing | O(J) where J = jet count | Simple byte mapping |
| Simulation | O(R × F) | R = rocks simulated, F = avg fall distance |
| Collision check | O(S) per check | S = cells in rock shape (4-5) |
| Cycle detection | O(R) HashMap inserts | Plus O(30) profile copy per rock |
| Part 1 total | O(2022 × F × S) | ~341µs |
| Part 2 total | O(~3400 × F × S) | ~400µs (cycle skips the rest) |

---

## Edge Cases

### Rocks Pushed Against Walls
Jets that would push a rock into a wall are simply ignored — no error, the rock stays put. Multiple consecutive wall-blocked jets are common.

### Very Tall Rocks
The I-piece (shape 3) is 4 rows tall. Combined with the 3-row gap above the tower, it spawns 7 rows above the highest settled rock. The `rows` Vec extends dynamically.

### Part 2 Cycle Not Found
If the chamber pattern never repeats (theoretically possible with very long jet sequences), the simulation would run forever. In practice, with 5 rock types and ~10K jet positions, a cycle always appears within a few thousand rocks.

---

## Key Takeaways

1. **Bitmask rows are natural for fixed-width chambers**: 7 columns fits in a `u8`, giving O(1) collision detection via bitwise AND.

2. **Cycle detection enables astronomical scale**: Instead of simulating 10¹² rocks, detect a repeating pattern and extrapolate with arithmetic. Only ~3,400 rocks actually simulated.

3. **State fingerprint must capture enough context**: Rock type + jet position + top N rows guarantees identical future behavior. Too few rows risks false matches; too many wastes memory in the HashMap.

4. **Independent simulations can still share parsing**: Even though Part 1 and Part 2 run separate simulations, the jet parsing is shared via the parse-once pattern.

5. **Tetris physics are simple**: Alternate between horizontal push and vertical fall, with collision checks. No gravity, no rotation, no scoring — just stacking.

---

**Answer**: Part 1: `3109` | Part 2: `1541449275365`

**Fun fact**: The Part 2 tower is ~24.3 million inches tall — roughly the distance from Earth to Venus at inferior conjunction (~24.8 million miles). One rock per inch, stacked almost to another planet.

**Related patterns**: [[cycle-detection]], [[bitmask-representation]], [[simulation]], [[parse-once-pattern]]
