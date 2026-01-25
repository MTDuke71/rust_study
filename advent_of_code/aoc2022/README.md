# AoC 2022 - README

Advent of Code 2022 solutions with comprehensive documentation.

---

## 📁 Structure

```
advent_of_code/aoc2022/
├── Problem_Statements/
│   ├── summary_2022.md           # Single source: progress + performance + algorithms
│   └── days/
│       ├── day01_function_guide.md   # Deep dive per day
│       ├── day02_function_guide.md
│       └── ...
│
├── src/
│   ├── main.rs                   # CLI runner
│   ├── lib.rs                    # Library exports
│   └── solver/
│       ├── mod.rs                # Day dispatcher
│       ├── day01.rs              # Day 1 solution
│       └── ...
│
├── inputs/
│   ├── day01.txt                 # Puzzle inputs
│   └── ...
│
├── benches/
│   └── benchmarks.rs             # Criterion benchmarks
│
├── Cargo.toml                    # Package configuration
└── README.md                     # This file
```

---

## 📊 Documentation Philosophy

### Streamlined 2+1 Structure

**Problem**: AoC 2023 had 5 documents to update per day (30-45 min overhead)  
**Solution**: Consolidate to **2 documents + daily note** (20-25 min total)

1. **`summary_2022.md`** - Single source of truth
   - Progress tracking (X/25 ⭐)
   - Performance table (all days)
   - Algorithms used by category
   - Patterns catalog (extracted at end)
   - Quick navigation

2. **`days/dayXX_function_guide.md`** - Deep dive per day
   - Function reference with examples
   - Algorithm analysis
   - Complexity breakdown
   - Key insights

3. **`zettelkasten/Daily Notes/YYYY-MM-DD.md`** - Short summary
   - Problem + answer
   - Runtime + algorithm
   - Key insight (1-2 sentences)
   - Link to function guide

---

## 🚀 Quick Start

### Running Solutions
```bash
# Run specific day
cargo run --release -- 1

# Run all days
cargo run --release -- all

# Run with example input
cargo run --release -- 1 --example
```

### Benchmarking
```bash
# Benchmark specific day
cargo bench --bench benchmarks day01

# Benchmark all days
cargo bench --bench benchmarks

# View results
cat target/criterion/report/index.html
```

### Testing
```bash
# Test specific day
cargo test -p aoc2022 day01

# Test all days
cargo test --workspace
```

---

## 🎯 Code Pattern - Parse Once, Solve Both

**Key Innovation**: Parse input once for both parts (saves ~50% runtime)

### Day Template (Copy/Paste)

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

### mod.rs Pattern

```rust
// In src/solver/mod.rs
pub mod day01;

pub fn run_day(day: usize, input: &str) -> Result<(String, String)> {
    match day {
        1 => {
            let (p1, p2) = day01::solve(input);  // Calls combined solver
            Ok((p1.to_string(), p2.to_string()))
        },
        _ => bail!("Day {} not implemented", day),
    }
}
```

### Development Workflow

1. **Create skeleton** - Copy template to `dayXX.rs`, add to `mod.rs`
2. **Parse input** - Write `parse_input()`, test with example
3. **Solve Part 1** - Implement `solve_part1_impl()`, test, run
4. **Submit Part 1** - Get star ⭐
5. **Solve Part 2** - Implement `solve_part2_impl()` (reuses parsed data!)
6. **Submit Part 2** - Get star ⭐
7. **Benchmark** - `cargo bench --bench benchmarks dayXX`

### Why This Pattern?

✅ **Parse once, not twice** - saves ~50% runtime  
✅ **DRY code** - no duplication  
✅ **Easy testing** - parts isolated  
✅ **Clear structure** - parse → logic → combine  
✅ **Future-proof** - ready for Part 3 if it appears  

### Comparison

**❌ OLD (AoC 2023 Style)**:
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

**✅ NEW (AoC 2022+ Style)**:
```rust
pub fn solve(input: &str) -> (usize, usize) {
    let data = parse(input);  // ← Parse ONCE
    (solve_part1_impl(&data), solve_part2_impl(&data))
}
```
**Benefit**: Parse once, solve both!

---

## 📝 Daily Workflow

**After solving each day**:

1. **Benchmark** (2 min)
   ```bash
   cargo bench --bench benchmarks day01
   ```

2. **Update summary_2022.md** (2 min)
   - Add row to performance table
   - Note algorithm used

3. **Create function guide** (15-20 min)
   ```bash
   cp templates/aoc_documentation/function_guide_template.md \
      Problem_Statements/days/day01_function_guide.md
   ```
   - Fill in function reference
   - Document algorithm
   - Add examples

4. **Daily note entry** (5 min)
   - Use template from `templates/aoc_documentation/daily_note_aoc_entry_template.md`
   - Add to `zettelkasten/Daily Notes/YYYY-MM-DD.md`

5. **Commit**
   ```bash
   git add .
   git commit -m "AoC 2022 Day 1: Calorie Counting ⭐⭐ (65µs, linear scan)"
   ```

**Total time**: ~25 minutes per day

---

## 🎯 Performance Goals

**Target**: Complete all 25 days in < 1 second total runtime

**Benchmarking Tool**: Criterion.rs
- 100 iterations per benchmark
- 95% confidence intervals
- Outlier detection

**Hardware**: [To be filled when benchmarking]

---

## 🎓 Learning Objectives

1. **Pattern Recognition**: Identify recurring algorithmic patterns across days
2. **Performance Optimization**: Achieve < 1s total runtime for all 25 days
3. **Mission Integration**: Reuse validated components (Grid, Graph, etc.)
4. **Rust Mastery**: Apply advanced Rust patterns (iterators, enums, traits)
5. **Documentation**: Create reusable reference for future AoC years

---

## 📚 Resources

**Code Patterns**:
- `advent_of_code/AOC_SOLVER_TEMPLATE.md` - **Comprehensive guide**
  - Real refactoring examples (Day 22)
  - Advanced patterns (different preparation for parts)
  - Benchmarking strategies
  - Migration from old pattern
- `advent_of_code/AOC_2022_QUICKSTART.md` - Quick reference (condensed version)

**Documentation Templates**:
- `templates/aoc_documentation/function_guide_template.md`
- `templates/aoc_documentation/daily_note_aoc_entry_template.md`

**AoC 2023 Reference**:
- See `advent_of_code/aoc2023/` for comparison
- 50/50 stars completed, 4.51s total runtime
- Comprehensive documentation with 5-file structure (being streamlined for 2022)

**Missions (Reusable Components)**:
- Mission 6: Grid utilities
- Mission 8: Graph algorithms
- Mission 5: HashSet patterns
- Mission 2: Queue/deque
- Mission 10: Union-Find

---

## 📊 Progress Tracking

**Current Status**: 0/25 ⭐  
**Total Runtime**: 0.0ms / 1000ms (0.0%)  
**Mission Integration**: 0 days  

**Next**: Solve Day 1!

---

**Created**: 2026-01-25  
**Status**: Planning phase - structure ready  
**Target**: Complete all 25 days with < 1s total runtime
