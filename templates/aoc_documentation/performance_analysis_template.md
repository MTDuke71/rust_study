# AoC 20XX - Performance Analysis Template

**Quick Links**: [← Templates](../README.md) | [AoC Main](../../advent_of_code/README.md) | [Stats Dashboard Template](stats_dashboard_template.md) | [Function Guide Template](function_guide_template.md)

---

**DEPRECATED**: This template described a separate performance analysis file in the old 4-file system.

**Current Approach (2-File System)**: Performance analysis is now integrated into:
1. **`summary_20XX.md`** - Overall statistics, runtime table, optimization highlights
2. **`dayXX_function_guide.md`** - Per-day performance details, benchmarks, complexity analysis

**Why Deprecated?**: Separate performance file created duplication with summary and function guides. The 2-file system reduces maintenance burden.

---

## 📝 Migration Guide

If you need performance analysis content, integrate it as follows:

### For Overall Performance Tracking → Use `summary_20XX.md`
- Runtime table with all days
- Total runtime progress toward 1-second goal
- Top fastest/slowest days
- Optimization highlights

### For Day-Specific Performance → Use `dayXX_function_guide.md`
- Benchmark results in performance section
- Before/after optimization comparisons
- Algorithm complexity analysis
- Profiling results (if applicable)

---

## 📊 Performance Content Examples

### In `summary_20XX.md` (Stats Dashboard)

**Performance Table**:
```markdown
| Day | Part 1 | Part 2 | Total | Algorithm |
|-----|--------|--------|-------|-----------|
| 1 | 24.4µs | 25.4µs | 25.6µs | Group parsing + sort |
```

**Optimization Highlights**:
```markdown
## 🚀 Notable Optimizations
- **Day 5**: HashMap indexing → 100x speedup (10ms → 100µs)
- **Day 12**: Cycle detection → Infinite speedup (intractable → 5ms)
```

### In `dayXX_function_guide.md` (Function Guide)

**Performance Section**:
```markdown
## Performance

**Benchmarks** (Criterion, 100 iterations):
- Part 1: 24.4µs ± 0.5µs
- Part 2: 25.4µs ± 0.6µs
- Combined: 25.6µs ± 0.4µs (parse once)

**Speedup**: 49% faster than separate parsing (50.8µs → 25.6µs)

**Complexity**:
- Parsing: O(n) where n = input lines
- Part 1: O(n) single scan for max
- Part 2: O(n log n) sort for top-3

**Budget**: 0.003% of 1-second total runtime budget
```

---

**Navigation**: [← Templates](../README.md) | [POST_IMPLEMENTATION_PLAN](POST_IMPLEMENTATION_PLAN.md) | [AoC Main](../../advent_of_code/README.md)

**See Also**:
- [POST_IMPLEMENTATION_PLAN.md](POST_IMPLEMENTATION_PLAN.md) - Complete 2-file system workflow
- [Stats Dashboard Template](stats_dashboard_template.md) - Where overall performance goes
- [Function Guide Template](function_guide_template.md) - Where per-day performance goes
- [AoC 2022 Summary](../../advent_of_code/aoc2022/Problem_Statements/summary_2022.md) - Live example

---

**Created**: 2026-01-20  
**Deprecated**: 2026-02-01 - Use 2-file system instead  
**Status**: Archived - Reference only

---

## 🗄️ Original Template Content (Archived)

*Below is the original template preserved for reference. Do not use this - integrate content into summary/function guides instead.*

---

## 📊 Overall Statistics

| Metric | Value |
|--------|-------|
| **Days Completed** | X/25 |
| **Total Runtime** | XXX.Xms |
| **Average per Day** | XX.Xms |
| **Fastest Day** | Day X (X.XXµs) |
| **Slowest Day** | Day X (XXX.Xms) |
| **Target Progress** | XXX.X / 1000ms (XX.X%) |

**1-Second Goal**: ✓ Achieved / ⚠️ XX.Xms over / 🎯 On track

---

## 📈 Cumulative Runtime Chart

| Day | Part 1 | Part 2 | Day Total | Running Total | % of Target |
|-----|--------|--------|-----------|---------------|-------------|
| 1   | XX.Xµs | XX.Xµs | XX.Xµs    | XX.Xµs        | 0.0X%       |
| 2   | XX.Xµs | XX.Xµs | XX.Xµs    | XXX.Xµs       | 0.XX%       |
| 3   | XX.Xµs | XX.Xµs | XX.Xµs    | X.XXms        | 0.XX%       |
| 4   | XX.Xµs | XX.Xµs | XX.Xµs    | X.XXms        | 0.XX%       |
| 5   | XX.Xµs | XX.Xµs | XX.Xµs    | X.XXms        | 0.XX%       |
| ... | ...    | ...    | ...       | ...           | ...         |
| 25  | XX.Xµs | XX.Xµs | XX.Xµs    | XXX.Xms       | XX.X%       |

**Visual Progress**:
```
Cumulative Runtime (ms)
1000 |━━━━━━━━━━━━━━━━━━━━━━━━━━ Target (1 second)
 900 |
 800 |
 700 |
 600 |                                    ╱─ Current (XXXms)
 500 |                              ╱────
 400 |                        ╱─────
 300 |                  ╱─────
 200 |            ╱─────
 100 |      ╱─────
   0 |──────────────────────────────────
      1    5    10   15   20   25 (days)
```

**Milestones**:
- 100ms reached: Day X
- 250ms reached: Day X
- 500ms reached: Day X
- 750ms reached: Day X (if applicable)

---

## 📊 Runtime Distribution by Day

| Day | Part 1 | Part 2 | Total | Optimized? | Notes |
|-----|--------|--------|-------|------------|-------|
| 1   | XX.Xµs | XX.Xµs | XX.Xµs | Yes/No | [Brief note] |
| 2   | XX.Xµs | XX.Xµs | XX.Xµs | Yes/No | [Brief note] |
| 3   | XX.Xµs | XX.Xµs | XX.Xµs | Yes/No | [Brief note] |
| ... | ...    | ...    | ...    | ...    | ... |
| 25  | XX.Xµs | XX.Xµs | XX.Xµs | Yes/No | [Brief note] |

**Legend**:
- ✓ Optimized - No further optimization needed
- ⚠️ Room for improvement - Known optimization opportunities
- 🎯 On track - Performance acceptable for problem size
- 🐢 Bottleneck - Largest contributor to total runtime

---

## 🚀 Optimization Wins

*Documented when significant optimizations are made (>2x speedup).*

### Day X: [Optimization Name]
**Before**: XXX.Xms  
**After**: XX.Xµs  
**Speedup**: XXXx faster  
**Complexity**: O(old) → O(new)  
**Technique**: [Brief description]  
**Learning**: [Key takeaway]  

**Code Example**:
```rust
// ❌ Before: O(n²) - Explanation
for item in items {
    for other in items {
        // expensive operation
    }
}

// ✅ After: O(n) - Explanation
let index: HashMap<_, _> = items.iter().enumerate().collect();
for item in items {
    // O(1) lookup
}
```

**Validation**: Both produce identical results (expected_answer)

---

### Day X: [Optimization Name]
**Before**: Intractable (would require X trillion operations)  
**After**: XX.Xms  
**Speedup**: Infinite (brute force impossible)  
**Complexity**: O(brute_force) → O(optimized)  
**Technique**: [e.g., Cycle detection, Mathematical formula, etc.]  
**Learning**: [Key takeaway]  

[Additional optimization sections as needed]

---

## 💡 Performance Patterns & Learnings

### Common Bottlenecks
1. **Parsing** (X days): String splitting/parsing often dominates runtime for simple problems
2. **Graph Traversal** (X days): BFS/DFS state-space exploration
3. **Nested Loops** (X days): Grid scanning, all-pairs comparisons
4. **State Space Explosion** (X days): Without memoization/cycle detection

### Effective Optimizations
1. **Spatial Indexing**: HashMap<Coord, Entity> for grid lookups → ~XXXx speedup (Day X)
2. **Cycle Detection**: HashMap state tracking + modulo fast-forward (Day X)
3. **Mathematical Shortcuts**: Closed-form solutions vs simulation (Day X, X)
4. **Memoization**: HashMap caching for recursive DP (Day X)
5. **Range Propagation**: Count combinations mathematically vs enumeration (Day X)

### Rust-Specific Wins
1. **Iterator Chains**: Zero-allocation processing with `.filter().map().fold()`
2. **HashSet/HashMap**: O(1) lookups replace O(n) linear search
3. **Enums**: Type-safe state machines catch missing cases at compile time
4. **Mission Integration**: Reusing validated Grid/Graph components saves implementation time

---

## 🎯 Performance by Algorithm Type

| Algorithm Type | Days | Avg Runtime | Notes |
|----------------|------|-------------|-------|
| Parsing | X, X, X | XX.Xµs | Often faster than computation itself |
| Grid Scan | X, X, X | XX.Xms | O(rows × cols) baseline |
| BFS/DFS | X, X, X | XX.Xms | State-space size critical |
| DP | X, X | XX.Xms | Memoization essential |
| Math | X, X | XX.Xµs | Closed-form solutions are fast |
| Simulation | X, X | XX.Xms | Cycle detection prevents intractability |

---

## 🔬 Benchmark Details

**Hardware**: [CPU model, RAM, etc.]  
**Benchmark Tool**: Criterion.rs  
**Iterations**: 100 per benchmark  
**Confidence**: 95%  

**Benchmark Commands**:
```bash
# Single day
cargo bench --bench benchmarks day01

# All days
cargo bench --bench benchmarks

# With timing output
cargo bench --bench benchmarks -- --verbose
```

---

## 📝 Optimization Opportunities (Future Work)

| Day | Current | Potential | Strategy | Expected Gain |
|-----|---------|-----------|----------|---------------|
| X   | XX.Xms  | ~X.Xms    | [Strategy] | XXx faster |
| X   | XX.Xms  | ~X.Xms    | [Strategy] | XXx faster |

**Note**: These are "nice to have" optimizations. Current runtime already meets 1-second goal.

---

**Last Updated**: YYYY-MM-DD  
**Benchmark Version**: vX.X.X  
**Next Review**: After Day X completion
