# AoC 2022 Quick Start - New Pattern

When you start AoC 2022, **copy this template for each day**:

## Day Template (Copy/Paste for Each Day)

```rust
//! Day XX: [Problem Title]
//!
//! **Brief problem description**

use std::collections::HashMap;  // Add imports as needed

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone)]
struct ParsedData {
    // Your parsed representation
}

// ============================================================================
// Parsing
// ============================================================================

fn parse_input(input: &str) -> ParsedData {
    // Parse logic here
    todo!()
}

// ============================================================================
// Part 1 Logic
// ============================================================================

fn solve_part1_impl(data: &ParsedData) -> usize {
    // Part 1 algorithm
    todo!()
}

// ============================================================================
// Part 2 Logic
// ============================================================================

fn solve_part2_impl(data: &ParsedData) -> usize {
    // Part 2 algorithm (revealed after Part 1 complete)
    todo!()
}

// ============================================================================
// Public API
// ============================================================================

/// Solve both parts efficiently (parse once)
pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    (solve_part1_impl(&data), solve_part2_impl(&data))
}

/// Solve Part 1 only (for testing)
pub fn solve_part1(input: &str) -> usize {
    solve_part1_impl(&parse_input(input))
}

/// Solve Part 2 only (for testing)
pub fn solve_part2(input: &str) -> usize {
    solve_part2_impl(&parse_input(input))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
...example input...
";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 0); // Replace 0 with expected
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 0); // Replace 0 with expected
    }
}
```

## mod.rs Pattern

```rust
// In src/solver/mod.rs
pub mod day01;

pub fn run_day(day: usize, input: &str) -> Result<(String, String)> {
    match day {
        1 => {
            let (p1, p2) = day01::solve(input);  // NEW: Call combined solver
            Ok((p1.to_string(), p2.to_string()))
        },
        _ => bail!("Day {} not implemented", day),
    }
}
```

## Development Workflow

### Step 1: Read Problem, Create Skeleton
```bash
# Copy template to dayXX.rs
# Replace XX, add to mod.rs
```

### Step 2: Parse Input
```rust
// Write parse_input() first
// Test with example input
#[test]
fn test_parse() {
    let data = parse_input(EXAMPLE);
    // Assert structure looks right
}
```

### Step 3: Solve Part 1
```rust
// Implement solve_part1_impl()
// Test with example
cargo test day01
```

### Step 4: Verify Real Input
```bash
cargo run -p aoc2022 -- 1
# Submit Part 1 answer
```

### Step 5: Solve Part 2
```rust
// Implement solve_part2_impl() when Part 2 revealed
// Often reuses same parsed data!
cargo test day01
cargo run -p aoc2022 -- 1
```

### Step 6: Benchmark (Optional)
```bash
cargo bench --bench benchmarks day01
```

---

## Why This Pattern?

✅ **Parse once, not twice** - saves ~50% runtime  
✅ **DRY code** - no duplication  
✅ **Easy testing** - parts isolated  
✅ **Clear structure** - parse → logic → combine  
✅ **Future-proof** - ready for Part 3 if it appears  

---

## Quick Comparison

### ❌ OLD (AoC 2023 Style)
```rust
pub fn solve_part1(input: &str) -> usize {
    let data = parse(input);  // ← Parse
    // solve...
}

pub fn solve_part2(input: &str) -> usize {
    let data = parse(input);  // ← Parse AGAIN!
    // solve...
}
```
**Problem**: When runner calls both, parses input twice!

### ✅ NEW (AoC 2022+ Style)
```rust
pub fn solve(input: &str) -> (usize, usize) {
    let data = parse(input);  // ← Parse ONCE
    (solve_part1_impl(&data), solve_part2_impl(&data))
}
```
**Benefit**: Parse once, solve both!

---

**See**: `advent_of_code/AOC_SOLVER_TEMPLATE.md` for full details  
**Example**: `advent_of_code/aoc2023/src/solver/day22.rs` (refactored)

Happy coding! 🎄
