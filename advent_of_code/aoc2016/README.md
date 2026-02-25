# AoC 2016 - README

Advent of Code 2016 solutions in Rust.

**Quick Links**: [AoC Main](../README.md) | [Summary](Problem_Statements/summary_2016.md) | [Solver Template](../AOC_SOLVER_TEMPLATE.md)

---

## Structure

```
advent_of_code/aoc2016/
├── Problem_Statements/
│   ├── summary_2016.md              # Progress + performance + algorithms
│   └── days/
│       ├── day01.md                  # Problem statements
│       ├── day01_function_guide.md   # Deep dive per day (created after solving)
│       └── ...
│
├── src/
│   ├── main.rs                      # CLI runner (with --bench flag)
│   ├── lib.rs                       # Library exports
│   └── solver/
│       ├── mod.rs                   # Day dispatcher
│       ├── day01.rs                 # Day 1 solution
│       └── ...
│
├── inputs/
│   ├── day01.txt                    # Puzzle inputs
│   └── ...
│
├── benches/
│   └── benchmarks.rs                # Criterion benchmarks
│
├── Cargo.toml
└── README.md                        # This file
```

---

## Documentation: 2-Document Structure

Streamlined from the 2022 approach — dropped the daily note step (proved redundant when function guides already capture everything).

1. **`summary_2016.md`** - Single source of truth
   - Progress tracking (X/25)
   - Performance table (all days)
   - Algorithms used by category
   - Patterns catalog

2. **`days/dayXX_function_guide.md`** - Deep dive per day
   - Function reference
   - Algorithm analysis
   - Complexity breakdown
   - Key insights

---

## Quick Start

### Running Solutions
```bash
# Run specific day
cargo run --release -- 1

# Run all days with timing
cargo run --release -- all

# Run with example input
cargo run --release -- 1 --example
```

### Built-in Benchmarking
```bash
# Single timed run (warmup + measure)
cargo run --release -- 1 --bench

# Average over 100 iterations
cargo run --release -- 1 --bench=100

# Benchmark all days
cargo run --release -- all --bench=50
```

### Criterion Benchmarks
```bash
# Benchmark specific day (statistical analysis)
cargo bench --bench benchmarks day01

# Benchmark all days
cargo bench --bench benchmarks
```

### Testing
```bash
# Test specific day
cargo test -p aoc2016 day01

# Test all days
cargo test -p aoc2016
```

---

## Code Pattern - Parse Once, Solve Both

```rust
//! Day XX: [Problem Title]

fn parse_input(input: &str) -> ParsedData {
    // Parse logic
}

fn solve_part1(data: &ParsedData) -> usize {
    // Part 1 algorithm
}

fn solve_part2(data: &ParsedData) -> usize {
    // Part 2 algorithm
}

pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    (solve_part1(&data), solve_part2(&data))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "...";

    #[test]
    fn test_part1_example() {
        let data = parse_input(EXAMPLE);
        assert_eq!(solve_part1(&data), 0);
    }
}
```

---

## Workflow Per Day

1. **Solve** - Parse → Part 1 → Part 2 → tests pass
2. **Benchmark** - `cargo run --release -- N --bench=100`
3. **Update summary** - Add row to performance table
4. **Create function guide** - Algorithm, complexity, insights
5. **Commit**

---

## Performance Goals

**Target**: All 25 days in < 1 second total runtime

**Tools**:
- Built-in `--bench` flag for quick timing
- Criterion.rs for statistical benchmarks

---

## Missions (Reusable Components)

- Mission 5: HashMap patterns
- Mission 6: Grid utilities
- Mission 8: Graph algorithms (BFS/DFS)
- Mission 10: Union-Find

---

**Created**: 2026-02-25
**Status**: Project scaffolded, ready for Day 1
