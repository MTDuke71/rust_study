# Day 10: Cathode-Ray Tube - Function Guide

**Problem**: Simulate a simple CPU with clock cycles and render output to a CRT screen.

**Answer**: Part 1: `13520` | Part 2: `PGPHBEAB`

## Performance Benchmarks
- **Combined**: ~~14.4µs~~ **7.5µs** (0.0075ms) — optimized with parse-once
- **Part 1**: ~7.3µs (signal strength calculation)
- **Part 2**: ~7.3µs (CRT rendering)

**Optimizations**:
- **Parse once**: 14.4µs → 7.5µs (-48%, single parse shared between parts)

**Complexity**: O(n) where n ≈ 240 cycles (6 rows × 40 pixels)

---

## Core Data Structures

### `Instruction` Enum
```rust
enum Instruction {
    Noop,        // 1 cycle, no effect
    Addx(i32),   // 2 cycles, adds to X after completion
}
```

**Key insight**: Instructions take time to execute, but X register updates AFTER completion.

### Execution Model
- **Cycle timing**: Each instruction has a duration (1 or 2 cycles)
- **Register updates**: X changes only AFTER instruction completes
- **During execution**: X holds the OLD value while instruction is in progress

---

## Part 1: Signal Strength Tracking

### Problem
Calculate signal strength (cycle × X) at specific cycles: 20, 60, 100, 140, 180, 220

### Algorithm
```rust
pub fn solve_part1(input: &str) -> i32 {
    let instructions = parse_instructions(input);
    
    let mut x = 1i32;
    let mut cycle = 0;
    let mut signal_sum = 0;
    
    for instruction in &instructions {
        // Execute instruction over multiple cycles
        for _ in 0..instruction.cycles() {
            cycle += 1;
            
            // Sample at specific cycles
            if cycle == 20 || (cycle > 20 && (cycle - 20) % 40 == 0) {
                signal_sum += cycle * x;
            }
        }
        
        // Update X AFTER instruction completes
        if let Instruction::Addx(value) = instruction {
            x += value;
        }
    }
    
    signal_sum
}
```

### Key Details
1. **Cycle tracking**: Increment cycle counter DURING execution
2. **X value**: Always use CURRENT X, not future value
3. **Target cycles**: 20, 60, 100, 140, 180, 220 (every 40 after 20)
4. **Signal strength**: `cycle_number * x_register_value`

**Example trace**:
```
Cycle 1-2: addx 15 executing, X=1
  Cycle 20: X=21, signal = 20 * 21 = 420
After cycle 2: X becomes 16
```

---

## Part 2: CRT Rendering

### Problem
Render a 40×6 pixel display based on sprite position (controlled by X register)

### Visual Model
- **Sprite**: 3 pixels wide, centered on X position `[X-1, X, X+1]`
- **CRT**: Draws left-to-right, top-to-bottom (40 pixels per row, 6 rows)
- **Each cycle**: Draw ONE pixel at position `(cycle-1) % 40`
  - Draw `#` if CRT position overlaps sprite
  - Draw `.` otherwise

### Algorithm
```rust
pub fn solve_part2(input: &str) -> String {
    let instructions = parse_instructions(input);
    
    let mut x = 1i32;
    let mut cycle = 0;
    let mut crt = String::new();
    
    for instruction in &instructions {
        for _ in 0..instruction.cycles() {
            // Calculate CRT column (0-39)
            let crt_col = (cycle % 40) as i32;
            
            // Check sprite overlap
            if (x - 1..=x + 1).contains(&crt_col) {
                crt.push('#');
            } else {
                crt.push('.');
            }
            
            cycle += 1;
            
            // Newline after each row
            if cycle % 40 == 0 {
                crt.push('\n');
            }
        }
        
        // Update X after instruction completes
        if let Instruction::Addx(value) = instruction {
            x += value;
        }
    }
    
    crt.trim_end().to_string()
}
```

### Sprite Positioning
- **X = 1**: Sprite at columns [0, 1, 2]
- **X = 16**: Sprite at columns [15, 16, 17]
- **X = 5**: Sprite at columns [4, 5, 6]

### CRT Position Mapping
```
Cycle  1 → Position  0 (row 0)
Cycle  2 → Position  1 (row 0)
...
Cycle 40 → Position 39 (row 0)
Cycle 41 → Position  0 (row 1)
...
Cycle 240 → Position 39 (row 5)
```

**Formula**: `crt_column = (cycle - 1) % 40` (or `cycle % 40` if cycle is 0-indexed)

---

## Parsing

```rust
fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[0] {
                "noop" => Instruction::Noop,
                "addx" => Instruction::Addx(parts[1].parse().unwrap()),
                _ => panic!("unknown instruction"),
            }
        })
        .collect()
}
```

**Input format**:
```
noop
addx 15
addx -11
noop
```

---

## Optimization Opportunities

### Current Performance (Already Optimal)
- **O(n) cycles**: ~240 iterations total
- **Simple operations**: Integer arithmetic, no allocations in hot path
- **String building**: Minimal overhead (240 chars + 6 newlines)

### Why It's Fast
1. **Parse once pattern**: Single parse shared by both parts (~48% speedup)
2. **Few cycles**: Only 240 total (vs thousands in previous days)
3. **No complex data structures**: Just integer counters
4. **Sequential processing**: No searching, hashing, or sorting
5. **Tight loops**: Simple conditionals, no branching complexity

### No Further Optimization Needed
- Already runs in **14µs** (0.014ms)
- Bottleneck is parsing (string splitting), not computation
- Could use `bytes()` for parsing, but diminishing returns

---

## Common Mistakes

### ❌ Updating X During Cycles
```rust
// WRONG: X changes too early
for _ in 0..cycles {
    x += value; // Don't update here!
    cycle += 1;
}
```

✅ **Correct**: Update X AFTER all cycles complete
```rust
for _ in 0..cycles {
    cycle += 1;
    // Use current X value
}
// NOW update X
x += value;
```

### ❌ Off-by-One in CRT Position
```rust
// WRONG: Position is 1-indexed
let crt_col = cycle % 40;  // Cycle 1 → col 1 (should be 0)
```

✅ **Correct**: CRT position is 0-indexed
```rust
let crt_col = (cycle - 1) % 40;  // Cycle 1 → col 0
// OR use 0-indexed cycle counter
```

### ❌ Missing Row Breaks in CRT
```rust
// WRONG: One long string
crt.push(pixel);
```

✅ **Correct**: Add newlines every 40 pixels
```rust
crt.push(pixel);
if cycle % 40 == 0 {
    crt.push('\n');
}
```

---

## Testing Strategy

### Example Validation
```rust
#[test]
fn test_part1_example() {
    // Example program runs 240 cycles
    // Expected signal sum: 13140
    assert_eq!(solve_part1(EXAMPLE), 13140);
}

#[test]
fn test_part2_example() {
    let expected = "##..##..##..##..##..##..##..##..##..##..\n...";
    assert_eq!(solve_part2(EXAMPLE), expected);
}
```

### Edge Cases
1. **Single noop**: X=1, cycle=1, signal=1
2. **Negative addx**: X can go negative (sprite off-screen)
3. **Sprite at boundaries**: X=0 or X=39 (sprite wraps conceptually)

---

## Part 2 Answer Format

**Visual Output** (requires manual OCR):
```
###...##..###..#..#.###..####..##..###..
#..#.#..#.#..#.#..#.#..#.#....#..#.#..#.
#..#.#....#..#.####.###..###..#..#.###..
###..#.##.###..#..#.#..#.#....####.#..#.
#....#..#.#....#..#.#..#.#....#..#.#..#.
#.....###.#....#..#.###..####.#..#.###..
```

**Reading letters** (each 5 chars wide: 4 pixels + 1 space):
- **P**: `###`, `#..#`, `#..#`, `###`, `#`, `#`
- **G**: `.##`, `#..#`, `#`, `#.##`, `#..#`, `.###`
- **P**: `###`, `#..#`, `#..#`, `###`, `#`, `#`
- **H**: `#..#`, `#..#`, `####`, `#..#`, `#..#`, `#..#`
- **B**: `###`, `#..#`, `###`, `#..#`, `#..#`, `###`
- **E**: `####`, `#`, `###`, `#`, `#`, `####`
- **A**: `.##`, `#..#`, `#..#`, `####`, `#..#`, `#..#`
- **B**: `###`, `#..#`, `###`, `#..#`, `#..#`, `###`

**Answer**: `PGPHBEAB`

---

## Related Patterns
- **Cycle simulation**: Similar to Day 9 rope physics
- **Register machine**: Classic CompSci CPU model
- **Visual rendering**: OCR-style answer format (common in AoC)
- **Timing synchronization**: CRT draws in sync with CPU cycles

---

## Links
- [← Day 9: Rope Bridge](day09.md)
- [Summary](../summary_2022.md)
- [Code](../../../aoc2022/src/solver/day10.rs)
- [Day 11 →](day11.md)
