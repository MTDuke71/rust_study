# AoC 2023 Workflow Guide & Agent Instructions

**Version**: 1.0  
**Created**: 2024-12-22  
**Purpose**: Standardized workflow for Advent of Code 2023 solving with focus on Rust mastery, performance, and knowledge building

---

## 📋 Table of Contents

1. [Philosophy & Goals](#philosophy--goals)
2. [Pre-Season Setup](#pre-season-setup)
3. [Daily Workflow](#daily-workflow)
4. [Documentation Standards](#documentation-standards)
5. [Templates](#templates)
6. [Agent Instructions](#agent-instructions)
7. [Quality Checklists](#quality-checklists)

---

## 🎯 Philosophy & Goals

### Primary Objectives
1. **Rust Mastery** - Idiomatic patterns, iterator fluency, type-driven design
2. **Performance Focus** - Benchmark everything, optimize hot paths, parallel when beneficial
3. **Knowledge Building** - Zettelkasten for novel algorithms, pattern extraction for reuse
4. **Mission Integration** - Leverage proven libraries, identify new mission opportunities
5. **Sustainable Pace** - Solve daily, polish later, maintain momentum

### What Changed from AoC 2024
- ❌ **NO Python comparisons** - Focus purely on Rust excellence
- ✅ **Two-pass approach** - Solve first, polish second
- ✅ **Performance emphasis** - Benchmark and optimize systematically
- ✅ **Visualization priority** - Create trace files for complex algorithms
- ✅ **Modular documentation** - Keep summary.md navigable (<2000 lines)

---

## 🛠️ Pre-Season Setup

### Directory Structure
```
advent_of_code/aoc2023/
├── src/
│   ├── solver/
│   │   ├── mod.rs
│   │   ├── day01.rs
│   │   └── ...
│   ├── patterns/              # NEW: Extracted reusable patterns
│   │   ├── mod.rs
│   │   ├── grid_search.rs     # BFS/DFS grid templates
│   │   ├── pathfinding.rs     # Dijkstra, A* templates
│   │   ├── combinatorics.rs   # Permutations, combinations
│   │   └── parsing.rs         # Common input patterns
│   └── main.rs
├── inputs/
│   ├── day01.txt
│   └── day01_example.txt
├── examples/                   # NEW: Visualization examples
│   ├── day01_visualization.rs
│   └── ...
├── benches/                    # NEW: Criterion benchmarks
│   ├── day01.rs
│   └── ...
├── Problem_Statements/
│   ├── README.md               # Overview + quick stats
│   ├── summary.md              # Daily entries (100-150 lines each)
│   ├── patterns-catalog.md     # Extracted patterns
│   ├── algorithms-reference.md # Deep dives → zettelkasten links
│   └── performance-analysis.md # Benchmarks summary
└── tests/
    └── integration_tests.rs
```

### Infrastructure Setup (Before December 1)
```bash
# 1. Create pattern library structure
mkdir -p src/patterns
touch src/patterns/{mod,grid_search,pathfinding,combinatorics,parsing}.rs

# 2. Set up benchmark infrastructure
mkdir -p benches examples
cargo install cargo-criterion

# 3. Create template files
cp templates/dayXX_solver.rs src/solver/day01.rs
cp templates/dayXX_bench.rs benches/day01.rs
cp templates/dayXX_visualization.rs examples/day01_visualization.rs

# 4. Set up documentation structure
cd Problem_Statements
touch README.md summary.md patterns-catalog.md algorithms-reference.md performance-analysis.md
```

### Cargo.toml Updates
```toml
[dependencies]
anyhow = "1.0"
criterion = "0.5"
rayon = "1.8"

[[bench]]
name = "day01"
harness = false

[[example]]
name = "day01_visualization"
```

---

## 🔄 Daily Workflow

### Phase 1: Solve (Morning - ~45 minutes)

**Goal**: Get both stars ⭐⭐ with working solution

```bash
# 1. Create day files from templates
cp templates/dayXX_solver.rs src/solver/day23.rs
cp templates/dayXX_test.rs tests/day23_test.rs

# 2. Read problem, download input
# Note: Part 2 NOT visible until Part 1 solved!

# 3. Implement Part 1
# - Write example test first (from problem statement)
# - Implement parse_input()
# - Implement solve_part1()
# - Verify with example, submit

# 4. Read Part 2 (now revealed!)
# - Write Part 2 example test
# - Implement solve_part2() (often extends Part 1)
# - Verify with example, submit

# 5. Quick commit
git add src/solver/day23.rs inputs/day23*.txt tests/day23_test.rs
git commit -m "feat(aoc2023): Solve Day 23 - [Title] (⭐⭐)"
```

**Acceptance Criteria (Phase 1)**:
- ✅ Both parts pass example tests
- ✅ Both parts produce correct answers
- ✅ Basic error handling (anyhow::Result)
- ✅ Code compiles with zero warnings
- ✅ Basic tests committed

**Phase 1 Time Budget**: 45 minutes MAX
- If stuck >1 hour, defer and solve next day's problem
- Return to difficult problems later

### Phase 2: Polish (Evening or Next Day - ~1 hour)

**Goal**: Production quality, performance, documentation

```bash
# 1. Refactor for mission integration (if applicable)
# Check if Mission 6 (Grid), Mission 8 (Graph), etc. apply
# Example: Grid problem → use Mission 6 instead of Vec<Vec<T>>

# 2. Add comprehensive tests
# - Edge cases (empty input, max values, single element)
# - Property tests (if applicable)
# - Regression tests (if bugs found)

# 3. Create visualization (if algorithm complex)
cp templates/dayXX_visualization.rs examples/day23_visualization.rs
# Implement intermediate state rendering
cargo run --example day23_visualization > traces/day23_trace.txt

# 4. Benchmark
cp templates/dayXX_bench.rs benches/day23.rs
cargo criterion --bench day23
# If >10ms, investigate optimization opportunities

# 5. Optimize (if needed)
# - Profile with cargo-flamegraph
# - Consider Rayon parallelization
# - Check assembly for hot loops (cargo-show-asm)
# - Re-benchmark to verify improvement

# 6. Document
# Update summary.md with day entry (use template)
# Create zettelkasten note if novel algorithm

# 7. Comprehensive commit
git add benches/day23.rs examples/day23_visualization.rs Problem_Statements/summary.md
git commit -m "refactor(aoc2023): Day 23 polish - [details]

- Add Mission X integration (Y% code reduction)
- Benchmark: Z ms (optimized from W ms)
- Visualization: trace file showing algorithm flow
- Tests: N comprehensive tests added
- Zettelkasten: [[algorithm-name]] note created"
```

**Acceptance Criteria (Phase 2)**:
- ✅ Mission integration attempted (if applicable)
- ✅ >80% test coverage
- ✅ Benchmarked (if >1ms runtime)
- ✅ Visualization created (if complex algorithm)
- ✅ Summary.md entry completed
- ✅ Zettelkasten note (if novel algorithm)
- ✅ Zero clippy warnings: `cargo clippy --all -- -D warnings`

### Weekly Review (Sunday - ~30 minutes)

**Goal**: Extract patterns, identify mission opportunities

```bash
# 1. Pattern extraction
# If 3+ days used similar technique, extract to src/patterns/
# Example: Days 5, 8, 12 all used BFS on grids → extract to patterns/grid_search.rs

# 2. Update patterns-catalog.md
# Document new patterns with references to days used

# 3. Mission candidate identification
# Example: "Days 7, 14, 21 all need range intersection → Mission 11: Interval Trees?"

# 4. Update README.md stats dashboard
# - Days completed
# - Average solve time
# - Mission usage count
# - Patterns extracted

# 5. Commit weekly summary
git commit -m "docs(aoc2023): Week X summary and pattern extraction"
```

---

## 📝 Documentation Standards

### summary.md Structure

**Total Target Length**: 1,500-2,000 lines (not 2,949!)

```markdown
# AoC 2023 - Summary

## 📊 Stats Dashboard
[Auto-generated or manually updated]
- **Progress**: 23/25 ⭐⭐
- **Total Runtime**: 847ms (average 34ms/day)
- **Mission Integration**: 8 days
- **Patterns Extracted**: 12

## 🔍 Quick Navigation
[Day 1](#day-1) | [Day 2](#day-2) | ... | [Patterns](#patterns-catalog)

---

## 📅 Daily Solutions

### Day 1: Historian Hysteria
**Algorithm**: List processing with sorting + HashMap frequency  
**Complexity**: O(n log n)  
**Runtime**: 2.3ms  
**Mission**: None  

**Key Insight**: Iterator chains eliminate intermediate allocations  

**Code Highlight** (10-20 lines):
```rust
// Only show critical section
let similarity: i64 = left.iter()
    .map(|&num| num * right_counts.get(&num).unwrap_or(&0))
    .sum();
```

**Tests**: ✅ Example, ✅ 3 edge cases  
**Zettelkasten**: [[iterator-patterns]]  

**Links**: [[day02]] →

---

### Day 2: [Title]
...

[Repeat template for each day]

---

## 🧩 Patterns Catalog
[Moved to patterns-catalog.md for space]
See: [patterns-catalog.md](patterns-catalog.md)

---

## 🎓 Algorithm Deep Dives
[Moved to algorithms-reference.md + zettelkasten]
See: [algorithms-reference.md](algorithms-reference.md)

---

## 📈 Performance Analysis
[Moved to performance-analysis.md]
See: [performance-analysis.md](performance-analysis.md)
```

**Day Entry Template** (100-150 lines max):

```markdown
### Day XX: [Problem Title]

**Part 1**: [One sentence description] → [Answer]  
**Part 2**: [One sentence description] → [Answer]  

**Algorithm**: [Name/approach]  
**Complexity**: O(...)  
**Runtime**: X.Xms (Part 1: Y.Yms, Part 2: Z.Zms)  
**Mission**: [Mission name or "None"]  

**Key Insight**: [1-2 sentence main learning]  

**Rust Highlights**:
- [Specific Rust pattern used, e.g., "Iterator fold for state accumulation"]
- [Another pattern, e.g., "Entry API for HashMap updates"]
- [Performance technique, e.g., "Rayon parallel iterator: 8x speedup"]

**Code Highlight** (10-20 lines critical section only):
```rust
// Show only the most interesting/novel part
// NOT the entire implementation
```

**Tests**: 
- ✅ Example test (from problem)
- ✅ Edge case 1: [description]
- ✅ Edge case 2: [description]

**Performance**:
- Benchmark: X.Xms ± Y.Yms
- Optimization: [What was optimized, if any]

**Zettelkasten**: 
- [[algorithm-name]] - Deep dive (if created)
- [[pattern-name]] - Related pattern

**Links**: ← [[dayXX-1]] | [[dayXX+1]] →
```

### patterns-catalog.md Structure

```markdown
# AoC 2023 - Patterns Catalog

Reusable patterns extracted from daily solutions.

---

## Grid Processing Patterns

### Pattern #1: BFS Neighbor Exploration
**Used**: Days 10, 12, 18  
**Code**: `src/patterns/grid_search.rs::bfs_neighbors()`  

**Template**:
```rust
fn bfs_neighbors<T>(grid: &Grid<T>, start: Coord) -> Vec<Coord> {
    // Implementation in patterns library
}
```

**When to use**: Finding shortest paths, flood fill, reachability  
**Zettelkasten**: [[bfs-patterns]]

---

## Mathematical Patterns

### Pattern #2: GCD/LCM for Cycle Detection
**Used**: Days 8, 20  
**Code**: `src/patterns/number_theory.rs::lcm()`  

[Continue for each pattern...]
```

### algorithms-reference.md Structure

```markdown
# AoC 2023 - Algorithms Reference

Links to zettelkasten deep dives for complex algorithms.

---

## Graph Algorithms

### Bron-Kerbosch Maximum Clique (Day 23)
**Zettelkasten**: [[bron-kerbosch-algorithm]] (~600 line comprehensive guide)  
**Implementation**: `src/solver/day23.rs`  
**Complexity**: O(3^(n/3)) worst-case, practical with pivoting  
**Key Concept**: Three-set recursion (R/P/X)  

**When to use**: Finding maximum cliques in sparse graphs  
**Related Problems**: [[karps-21-np-complete-problems]] (NP-complete)

---

### Dijkstra with Priority Queue (Day 17)
**Zettelkasten**: [[dijkstra-algorithm]]  
[Continue for each complex algorithm...]
```

### performance-analysis.md Structure

```markdown
# AoC 2023 - Performance Analysis

Benchmarks and optimization insights.

---

## Overall Statistics
- **Total Runtime**: 847ms (all 25 days)
- **Average**: 34ms/day
- **Fastest**: Day 1 (0.3ms)
- **Slowest**: Day 23 (98ms)

---

## Optimization Wins

### Day 22: Power-of-2 Modulo Optimization
**Before**: 40.4ms  
**After**: 18.8ms  
**Speedup**: 2.15x  
**Technique**: Compiler optimizes `x % 16777216` → `x & 0xFFFFFF`  
**Learning**: Use powers of 2 for modulo when possible  

---

### Day 14: Rayon Parallelization
**Before**: 124ms (serial)  
**After**: 16ms (parallel)  
**Speedup**: 7.75x  
**Technique**: `.par_iter()` on independent iterations  
**Learning**: Embarrassingly parallel problems benefit massively  

[Continue for each significant optimization...]
```

---

## 📐 Templates

### Template: src/solver/dayXX.rs

```rust
use anyhow::{Context, Result};

type Input = Vec<String>; // Adjust as needed

pub fn solve_part1(input: &str) -> Result<String> {
    let data = parse_input(input)?;
    let result = compute_part1(&data)?;
    Ok(result.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let data = parse_input(input)?;
    let result = compute_part2(&data)?;
    Ok(result.to_string())
}

fn parse_input(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| {
            // Parse line
            Ok(line.to_string())
        })
        .collect()
}

fn compute_part1(data: &Input) -> Result<i64> {
    // Implementation
    Ok(0)
}

fn compute_part2(data: &Input) -> Result<i64> {
    // Implementation
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        [paste example input]
    ";

    #[test]
    fn test_part1_example() -> Result<()> {
        assert_eq!(solve_part1(EXAMPLE)?, "expected_answer");
        Ok(())
    }

    #[test]
    fn test_part2_example() -> Result<()> {
        assert_eq!(solve_part2(EXAMPLE)?, "expected_answer");
        Ok(())
    }

    #[test]
    fn test_edge_case_empty() -> Result<()> {
        // Edge case tests
        Ok(())
    }
}
```

### Template: benches/dayXX.rs

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc2023::solver::dayXX::{solve_part1, solve_part2};

const INPUT: &str = include_str!("../inputs/dayXX.txt");

fn benchmark_part1(c: &mut Criterion) {
    c.bench_function("day_XX_part1", |b| {
        b.iter(|| solve_part1(black_box(INPUT)).unwrap())
    });
}

fn benchmark_part2(c: &mut Criterion) {
    c.bench_function("day_XX_part2", |b| {
        b.iter(|| solve_part2(black_box(INPUT)).unwrap())
    });
}

criterion_group!(benches, benchmark_part1, benchmark_part2);
criterion_main!(benches);
```

### Template: examples/dayXX_visualization.rs

```rust
use anyhow::Result;
use aoc2023::solver::dayXX::*;

fn main() -> Result<()> {
    let input = include_str!("../inputs/dayXX_example.txt");
    
    println!("=== Day XX Visualization ===\n");
    
    // Parse and visualize initial state
    let data = parse_input(input)?;
    println!("Initial state:");
    render_state(&data);
    
    // Step through algorithm showing intermediate states
    for step in 0..10 {
        println!("\n--- Step {} ---", step);
        // Show state after each step
        // This becomes your regression test AND learning tool
    }
    
    Ok(())
}

fn render_state(data: &Input) {
    // Render current state
}
```

---

## 🤖 Agent Instructions

### For GitHub Copilot Coding Agent

**When user says "solve day XX"**:

**Phase 1 (Solve)**:
1. Create `src/solver/dayXX.rs` from template
2. Read problem from user (they'll paste Part 1)
3. Ask: "What's the example input and expected output?"
4. Implement `parse_input()` and `solve_part1()`
5. Add example test, verify it passes
6. User solves Part 1, AoC reveals Part 2
7. Ask: "What does Part 2 require?" (user pastes Part 2 description)
8. Implement `solve_part2()` extending Part 1 logic
9. Add Part 2 example test, verify it passes
10. Quick commit: `feat(aoc2023): Solve Day XX - [Title] (⭐⭐)`

**Phase 2 (Polish)**:
1. Ask: "Does this problem use grids/graphs/ranges?" → Check mission applicability
2. Add edge case tests (empty, single element, max values)
3. Create visualization if algorithm is:
   - Recursive backtracking
   - Graph traversal
   - Simulation with state changes
   - Anything user struggled to understand
4. Benchmark with Criterion
5. If >10ms:
   - Profile with suggestions
   - Check for parallelization opportunities (Rayon)
   - Inspect hot loops (suggest assembly analysis if needed)
6. Create summary.md entry using template (100-150 lines MAX)
7. If algorithm is novel/complex, create zettelkasten note
8. Comprehensive commit with details

**When user asks "what patterns can we extract?"**:
1. Review last 5-7 solved days
2. Identify repeated code structures
3. Suggest extraction to `src/patterns/[category].rs`
4. Update `patterns-catalog.md`

**When user asks "should we build a mission?"**:
1. Check if pattern used in 5+ days
2. Verify it's a fundamental data structure/algorithm
3. Propose mission specification with REQ-IDs
4. Reference V-Cycle methodology

**Response Style**:
- **Phase 1**: Fast, pragmatic, get stars quickly
- **Phase 2**: Thoughtful, educational, production quality
- **Always**: Concise explanations, no unnecessary verbosity
- **Code**: Idiomatic Rust, leverage iterators, type-driven design

**Forbidden**:
- ❌ Suggesting Python comparisons
- ❌ Creating >150 line summary.md entries
- ❌ Duplicating zettelkasten content in summary.md
- ❌ Over-engineering Phase 1 (stars first!)
- ❌ Writing entire implementations in chat (use files!)

**Encouraged**:
- ✅ Asking for example input/output
- ✅ Suggesting mission integration
- ✅ Proposing performance optimizations
- ✅ Creating visualizations for complex algorithms
- ✅ Extracting patterns to reusable libraries
- ✅ Linking to existing zettelkasten notes

---

## ✅ Quality Checklists

### Phase 1 Checklist (Before First Commit)

```markdown
- [ ] Both parts pass example tests
- [ ] Both parts produce correct answers (stars submitted)
- [ ] Code compiles with `cargo build`
- [ ] Zero compiler warnings
- [ ] Zero clippy warnings: `cargo clippy -- -D warnings`
- [ ] Basic error handling (anyhow::Result throughout)
- [ ] Example input saved as inputs/dayXX_example.txt
- [ ] Real input saved as inputs/dayXX.txt
```

### Phase 2 Checklist (Before Second Commit)

```markdown
- [ ] Mission integration attempted (if applicable)
- [ ] Edge case tests added (minimum 2)
- [ ] Benchmark created (if runtime >1ms)
- [ ] Visualization created (if algorithm complex)
- [ ] Summary.md entry completed (100-150 lines)
- [ ] Zettelkasten note created (if novel algorithm)
- [ ] Clippy --all passes with -D warnings
- [ ] Formatted with `cargo fmt --all`
- [ ] Tests pass: `cargo test --all`
```

### Weekly Checklist (Sunday Review)

```markdown
- [ ] Reviewed week's solutions for patterns
- [ ] Extracted patterns to src/patterns/ (if 3+ uses)
- [ ] Updated patterns-catalog.md
- [ ] Identified mission candidates
- [ ] Updated README.md stats dashboard
- [ ] Committed weekly summary
- [ ] Reviewed benchmark results for outliers
- [ ] Updated performance-analysis.md (if significant wins)
```

---

## 📊 Success Metrics

### Daily Targets
- **Phase 1**: Complete within 45 minutes (both stars)
- **Phase 2**: Complete within 1 hour (polish)
- **Total**: <2 hours per day

### Weekly Targets
- **Pattern Extraction**: Identify 1-2 reusable patterns
- **Test Coverage**: Maintain >80% across all days
- **Performance**: Average <50ms per solution

### Season Targets (25 Days)
- **Mission Building**: Create 2-3 new missions from patterns
- **Zettelkasten**: Create 10-15 comprehensive algorithm notes
- **Pattern Library**: Extract 15-20 reusable patterns
- **Performance**: Total runtime <2 seconds for all 25 days
- **Documentation**: Keep summary.md under 2,000 lines

---

## 🚀 Getting Started (December 1, 2023)

### Day Before (November 30)
1. ✅ Run pre-season setup commands
2. ✅ Verify all templates exist
3. ✅ Create README.md with empty stats dashboard
4. ✅ Test benchmark infrastructure: `cargo criterion`
5. ✅ Review this workflow guide one final time

### Day 1 (December 1)
1. Follow Phase 1 workflow exactly
2. Time yourself - aim for <45 minutes
3. Complete Phase 2 same day (get into rhythm)
4. Note any workflow friction points
5. Adjust templates if needed

### Week 1 (Days 1-7)
- Focus on workflow consistency
- Build muscle memory for templates
- Don't optimize prematurely
- Establish daily rhythm

### Week 2+ (Days 8-25)
- Begin pattern extraction
- Identify first mission candidate
- Optimize hot paths
- Enjoy the learning journey! 🎉

---

## 📝 Notes

- This guide is a living document - adjust based on what works
- Consistency > perfection - follow templates even if not optimal
- The workflow serves learning, not the other way around
- When in doubt, prioritize: Stars → Tests → Performance → Documentation

---

**Version History**:
- v1.0 (2024-12-22): Initial guide based on AoC 2024 learnings

**Next Review**: After AoC 2023 Day 7 (end of Week 1) - assess workflow effectiveness
