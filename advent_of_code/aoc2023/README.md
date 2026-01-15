# Advent of Code 2023

**Status**: Infrastructure ready, problems downloaded, ready to solve! 🎄

## 🎯 Workflow

Following the **two-pass approach** from [AOC2023_WORKFLOW_GUIDE.md](../../.github/AOC2023_WORKFLOW_GUIDE.md):

### Phase 1: Solve (45 minutes)
1. Read problem in `Problem_Statements/days/dayXX.md`
2. Copy template: `cp src/solver/day_template.rs src/solver/dayXX.rs`
3. Implement working solution (no optimization yet)
4. Test with example: `cargo test -p aoc2023 dayXX`
5. Run with real input: `cargo run -p aoc2023 -- XX`

### Phase 2: Polish (1 hour)
1. Refactor for clarity and idiomatic Rust
2. Add comprehensive documentation
3. Extract reusable patterns to `patterns/` module
4. Optimize with profiling data
5. Add benchmarks

## 🏃‍♂️ Quick Commands

```bash
# Run a specific day with real input
cargo run -p aoc2023 -- 1

# Run with example input
cargo run -p aoc2023 -- 1 advent_of_code/aoc2023/inputs/day01_example.txt

# Run with debug output
cargo run -p aoc2023 -- 1 --debug

# Test a specific day
cargo test -p aoc2023 day01

# Run all tests
cargo test -p aoc2023

# Benchmark a day
cargo bench -p aoc2023
```

## 📁 Structure

```
aoc2023/
├── inputs/
│   ├── day01.txt              # Real puzzle input
│   ├── day01_example.txt      # Example input (extract manually from problem)
│   └── ...
├── Problem_Statements/days/
│   ├── day01.md               # Problem description (Markdown)
│   ├── day01.html             # Problem description (HTML backup)
│   └── ...
├── src/
│   ├── main.rs                # CLI runner
│   ├── lib.rs                 # Library interface
│   ├── parser.rs              # Parsing utilities
│   ├── solver/
│   │   ├── mod.rs             # Day dispatcher
│   │   ├── day_template.rs    # Template for new days
│   │   ├── day01.rs           # Day 1 solution
│   │   └── ...
│   └── patterns/              # Reusable algorithms (create as needed)
└── benches/
    └── benchmarks.rs          # Performance benchmarks
```

## 🔗 Mission Integration

Reuse validated mission components:
- **Mission 5**: HashMap/HashSet for frequency, caching
- **Mission 6**: Grid for 2D pathfinding, regions
- **Mission 8**: Graph algorithms (BFS/DFS/shortest paths)
- **Mission 10**: UnionFind for connected components

## 📊 Progress Tracker

| Day | Part 1 | Part 2 | Polished | Notes |
|-----|--------|--------|----------|-------|
| 01  | ⬜     | ⬜     | ⬜       |       |
| 02  | ⬜     | ⬜     | ⬜       |       |
| 03  | ⬜     | ⬜     | ⬜       |       |
| 04  | ✅     | ✅     | ✅       | Set intersections |
| 05  | ✅     | ✅     | ✅       | Range mapping |
| 06  | ✅     | ✅     | ✅       | Quadratic formula |
| 07  | ✅     | ✅     | ✅       | Poker hands |
| 08  | ✅     | ✅     | ✅       | LCM cycle detection |
| 09  | ✅     | ✅     | ✅       | Polynomial sequences |
| 10  | ✅     | ✅     | ✅       | Grid BFS + Mission 6/8 |
| 11  | ✅     | ✅     | ✅       | Manhattan distance |
| 12  | ✅     | ✅     | ✅       | DP pattern matching |
| 13  | ✅     | ✅     | ✅       | Hamming distance, reflection |
| 14  | ✅     | ✅     | ⬜       | Platform tilt, cycle detection |
| ... | ⬜     | ⬜     | ⬜       |       |
| 25  | ⬜     | ⬜     | ⬜       |       |

Legend: ⬜ Not started | 🟨 In progress | ✅ Complete

## 🎓 Learning Goals

1. **Rust Mastery**: Idiomatic patterns, zero-copy optimizations, trait design
2. **Algorithm Practice**: Pattern recognition, complexity analysis
3. **Mission Application**: Practical use of validated data structures
4. **Documentation**: Clear explanations of approach and trade-offs
