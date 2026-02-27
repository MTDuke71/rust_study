---
name: aoc
description: Solve Advent of Code problems with parse-once pattern, mission integration, benchmarks, and documentation
---

# AoC Instructions - Advent of Code Problem Solving

**Purpose**: Solve Advent of Code problems efficiently with parse-once pattern, comprehensive documentation, and performance optimization.

---

## CRITICAL: Parse-Once Pattern

**Violations detected**: Days 9 and 10 (both caught by user review)
**Performance impact**: 40-50% slowdown when violated

```rust
// ❌ WRONG: Parses input twice
pub fn solve(input: &str) -> (usize, usize) {
    (solve_part1(input), solve_part2(input))  // Each parses internally!
}

// ✅ CORRECT: Parse once, share data
pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);  // ← Parse ONCE
    (
        solve_part1_with_data(&data),
        solve_part2_with_data(&data),
    )
}
```

**See**: `zettelkasten/parse-once-pattern.md` for comprehensive reference

---

## Mandatory 4-Function Code Structure

```rust
//! Day XX: [Problem Title]
//!
//! **Brief problem description**

// Data Structures
#[derive(Debug, Clone)]
struct ParsedData { /* parsed representation */ }

// Parsing
fn parse_input(input: &str) -> ParsedData { todo!() }

// Part 1 Logic (Internal - accepts parsed data, NO parsing here!)
fn solve_part1_with_data(data: &ParsedData) -> usize { todo!() }

// Part 2 Logic (Internal - accepts parsed data, NO parsing here!)
fn solve_part2_with_data(data: &ParsedData) -> usize { todo!() }

// Public API
pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    (solve_part1_with_data(&data), solve_part2_with_data(&data))
}

pub fn solve_part1(input: &str) -> usize {
    let data = parse_input(input);
    solve_part1_with_data(&data)
}

pub fn solve_part2(input: &str) -> usize {
    let data = parse_input(input);
    solve_part2_with_data(&data)
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "...example input...";

    #[test]
    fn test_parse() { let data = parse_input(EXAMPLE); }
    #[test]
    fn test_part1_example() { assert_eq!(solve_part1(EXAMPLE), 0); }
    #[test]
    fn test_part2_example() { assert_eq!(solve_part2(EXAMPLE), 0); }
    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/dayXX.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, 0); // Replace with actual answer
    }
    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/dayXX.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, 0); // Replace with actual answer
    }
    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/dayXX.txt");
        assert_eq!(solve(input), (0, 0)); // Replace with actual answers
    }
}
```

---

## Development Workflow

1. **Create skeleton** (5 min) - Copy template, add to mod.rs, `cargo test dayXX`
2. **Parse input** (10-15 min) - Write `parse_input()`, test with example
3. **Solve Part 1** (20-40 min) - Test example → run puzzle → submit → add actual test
4. **Solve Part 2** (20-40 min) - Test example → run puzzle → submit → add actual tests
5. **Benchmark** (2 min) - `cargo bench --bench benchmarks dayXX_combined` (combined < sum of parts!)
6. **Clippy** (1 min) - `cargo clippy -p aocYYYY --all-targets -- -D warnings` (zero warnings)
7. **Document** (20-25 min) - summary_YYYY.md + function_guide + daily note

---

## Mission Integration (ALWAYS scan first!)

- **Grid problems** → Mission 6 `Grid<T>`
- **Graph traversal** → Mission 8 `Graph` trait + BFS/DFS
- **Union-Find** → Mission 10 `UnionFind`
- **HashMap/HashSet** → Mission 5

---

## Performance Standards

- **Per day**: < 100ms (most days)
- **Optimization order**: Parse-once → FxHashSet/FxHashMap → Bitset → Rayon → Pre-allocation
- **Don't optimize** if already under 100ms

---

## Documentation: 2+1 Structure

1. **`summary_YYYY.md`** - Table with timings, algorithms, mission links (5 min)
2. **`days/dayXX_function_guide.md`** - Deep dive with benchmarks and algorithm (15-20 min)
3. **`zettelkasten/Daily Notes/YYYY-MM-DD.md`** - Session summary (5 min)

---

## Quick Reference Checklist

Before committing any AoC day:

- [ ] **Structure**: Internal `_with_data()` functions exist
- [ ] **Parse-once**: `solve()` parses exactly once
- [ ] **Tests**: All example tests pass + actual answer tests added
- [ ] **Benchmark**: Combined < sum of parts
- [ ] **Clippy**: `cargo clippy -p aocYYYY --all-targets -- -D warnings` (zero warnings)
- [ ] **Mission scan**: Checked for reusable components
- [ ] **Documentation**: summary + function_guide + daily note updated
- [ ] **User review**: Code reviewed before commit (if possible)

---

## Common Mistakes

- **Parse-once violation**: `solve()` calling `solve_part1(input)` directly (Days 9 & 10!)
- **Forgetting mission components**: Reimplementing Grid/Graph/UnionFind from scratch
- **Over-engineering parsing**: Use simple splits, not regex for simple formats
- **Missing line ending normalization**: Always `.replace("\r\n", "\n")` before `split("\n\n")`
