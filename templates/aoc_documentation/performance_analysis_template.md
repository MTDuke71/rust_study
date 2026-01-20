# AoC 2022 - Performance Analysis

Benchmarks, optimizations, and runtime analysis across all days.

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
