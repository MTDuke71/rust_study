# AoC Solver Template - Efficient Pattern

**Problem Identified in AoC 2023**: Parsing and data preparation duplicated in `solve_part1()` and `solve_part2()`, causing:
- 2× parsing overhead when running both parts
- Code duplication across functions
- Harder to benchmark combined runtime

**Solution**: Use a `solve()` function that parses once and solves both parts.

---

## Recommended Pattern (Use for AoC 2022+)

### Template Structure

```rust
//! Day XX: Problem Title
//!
//! **Problem Summary**: Brief description
//!
//! **Approach**:
//! - Part 1: Algorithm description
//! - Part 2: Algorithm description
//!
//! **Key Insights**:
//! - Insight 1
//! - Insight 2

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone)]
struct ParsedData {
    // Your parsed input representation
}

// ============================================================================
// Parsing (Done Once!)
// ============================================================================

/// Parse input into structured data
fn parse_input(input: &str) -> ParsedData {
    // Parsing logic here
    ParsedData { /* ... */ }
}

// ============================================================================
// Part 1 Logic
// ============================================================================

/// Solve Part 1 given prepared data
fn solve_part1_impl(data: &ParsedData) -> usize {
    // Part 1 algorithm using data
    0
}

// ============================================================================
// Part 2 Logic
// ============================================================================

/// Solve Part 2 given prepared data
fn solve_part2_impl(data: &ParsedData) -> usize {
    // Part 2 algorithm using data
    0
}

// ============================================================================
// Public API
// ============================================================================

/// **EFFICIENT**: Solve both parts with single parse
///
/// This is the primary entry point when running both parts.
/// Parsing happens exactly once, then both solutions computed.
pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    let part1 = solve_part1_impl(&data);
    let part2 = solve_part2_impl(&data);
    (part1, part2)
}

/// Solve Part 1 only (for individual testing)
pub fn solve_part1(input: &str) -> usize {
    solve_part1_impl(&parse_input(input))
}

/// Solve Part 2 only (for individual testing)
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
your example input here
";

    #[test]
    fn test_parse() {
        let data = parse_input(EXAMPLE);
        // Assert parsed data structure
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 42);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 84);
    }
    
    #[test]
    fn test_solve_combined() {
        let (p1, p2) = solve(EXAMPLE);
        assert_eq!(p1, 42);
        assert_eq!(p2, 84);
    }
}
```

---

## Update mod.rs Runner

```rust
// In src/solver/mod.rs

pub fn run_day(day: usize, input: &str) -> Result<(String, String)> {
    match day {
        1 => {
            // NEW PATTERN: Call combined solver
            let (p1, p2) = day01::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        2 => {
            let (p1, p2) = day02::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        // ... etc
        
        // OLD PATTERN (still works, just less efficient)
        // 99 => Ok((day99::solve_part1(input)?, day99::solve_part2(input)?)),
        
        _ => bail!("Day {} not implemented", day),
    }
}
```

---

## Benefits of This Pattern

### ✅ Performance
- **Parsing happens once** when running both parts (not twice)
- Easy to benchmark combined runtime accurately
- Can measure parse time separately from solve time

### ✅ Code Organization
- **Clear separation**: Parse → Part1 → Part2
- **DRY principle**: No duplicated parsing logic
- **Type safety**: Parsed data structure documents assumptions

### ✅ Flexibility
- `solve()` for efficient combined solving
- `solve_part1()` / `solve_part2()` still available for isolated testing
- Easy to add Part 3 if needed (some problems have it)

### ✅ Testability
- Test parsing separately: `parse_input()`
- Test parts separately: `solve_part1()`, `solve_part2()`
- Test combined: `solve()`
- Mock data: `solve_part1_impl(&mock_data)`

---

## Real Example: AoC 2023 Day 22 Refactored

See `advent_of_code/aoc2023/src/solver/day22_refactored.rs` for complete example.

**Before (Duplicated)**:
```rust
pub fn solve_part1(input: &str) -> usize {
    let mut bricks = parse_bricks(input);        // ← Parse
    simulate_falling(&mut bricks);                // ← Prepare
    let (supports, supported_by) = build_support_graph(&bricks);  // ← Prepare
    
    // Part 1 logic...
}

pub fn solve_part2(input: &str) -> usize {
    let mut bricks = parse_bricks(input);        // ← Parse AGAIN!
    simulate_falling(&mut bricks);                // ← Prepare AGAIN!
    let (supports, supported_by) = build_support_graph(&bricks);  // ← Prepare AGAIN!
    
    // Part 2 logic...
}
```

**After (Efficient)**:
```rust
struct PreparedBricks {
    bricks: Vec<Brick>,
    supports: Vec<HashSet<usize>>,
    supported_by: Vec<HashSet<usize>>,
}

fn parse_and_prepare(input: &str) -> PreparedBricks {
    let mut bricks = parse_bricks(input);
    simulate_falling(&mut bricks);
    let (supports, supported_by) = build_support_graph(&bricks);
    PreparedBricks { bricks, supports, supported_by }
}

fn solve_part1_impl(data: &PreparedBricks) -> usize {
    // Part 1 logic using data
}

fn solve_part2_impl(data: &PreparedBricks) -> usize {
    // Part 2 logic using data
}

pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_and_prepare(input);  // ← Parse ONCE!
    let part1 = solve_part1_impl(&data);
    let part2 = solve_part2_impl(&data);
    (part1, part2)
}

// Keep for individual testing
pub fn solve_part1(input: &str) -> usize {
    solve_part1_impl(&parse_and_prepare(input))
}

pub fn solve_part2(input: &str) -> usize {
    solve_part2_impl(&parse_and_prepare(input))
}
```

---

## When Parts Need Different Preparation

Sometimes Part 2 needs different data structures. Two approaches:

### Approach 1: Prepare Both Structures

```rust
struct PreparedData {
    common: CommonData,
    part1_specific: Part1Data,
    part2_specific: Part2Data,
}

pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    // Each part uses what it needs
    (solve_part1_impl(&data), solve_part2_impl(&data))
}
```

### Approach 2: Progressive Enhancement

```rust
fn parse_input(input: &str) -> CommonData {
    // Parse common structures
}

fn enhance_for_part2(common: &CommonData) -> Part2Data {
    // Build Part 2 specific structures from common
}

pub fn solve(input: &str) -> (usize, usize) {
    let common = parse_input(input);
    let part1 = solve_part1_impl(&common);
    let part2_data = enhance_for_part2(&common);
    let part2 = solve_part2_impl(&part2_data);
    (part1, part2)
}
```

---

## Benchmarking with This Pattern

```rust
// benches/benchmarks.rs

fn benchmark_day22(c: &mut Criterion) {
    let input = include_str!("../inputs/day22.txt");
    
    // Benchmark combined (most realistic)
    c.bench_function("Day 22 Combined", |b| {
        b.iter(|| day22::solve(black_box(input)))
    });
    
    // Benchmark parts separately (includes parsing overhead)
    c.bench_function("Day 22 Part 1", |b| {
        b.iter(|| day22::solve_part1(black_box(input)))
    });
    
    c.bench_function("Day 22 Part 2", |b| {
        b.iter(|| day22::solve_part2(black_box(input)))
    });
    
    // Benchmark just the algorithms (parse once, reuse)
    let data = day22::parse_input(input);  // If you expose it
    c.bench_function("Day 22 Part 1 Logic Only", |b| {
        b.iter(|| day22::solve_part1_impl(black_box(&data)))
    });
}
```

---

## Migration Strategy

### For New Years (AoC 2022)
✅ Use this pattern from Day 1

### For Existing Years (AoC 2023)
- ⚠️ Don't refactor all at once
- ✅ Refactor opportunistically:
  - When adding tests
  - When benchmarking
  - When documenting
  - When you notice duplication bothering you
- ✅ Both patterns can coexist in mod.rs

---

## Quick Reference Checklist

When implementing a new day:

- [ ] Create data structure for parsed input
- [ ] Write `parse_input()` function
- [ ] Write `solve_part1_impl(data: &ParsedData)` function
- [ ] Write `solve_part2_impl(data: &ParsedData)` function
- [ ] Write `solve()` function that combines them
- [ ] Write `solve_part1()` and `solve_part2()` wrappers
- [ ] Update `mod.rs` to call `solve()`
- [ ] Add tests for parsing and both parts
- [ ] Update benchmarks to use `solve()`

---

**Bottom Line**: Parse once, solve twice. Your future self (and your CPU) will thank you! 🚀
