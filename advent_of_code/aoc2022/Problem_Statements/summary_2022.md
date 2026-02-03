# AoC 2022 - Summary

**Status**: 🎯 In Progress (3/25 complete)

---

## 📊 Stats Dashboard

| Metric | Value |
|--------|-------|
| **Progress** | 3/25 |
| **Total Runtime** | 70.9µs (bitset optimized) |
| **Average per Day** | 23.6µs |
| **Fastest Day** | Day 2 (21.5µs) |
| **Slowest Day** | Day 1 (25.6µs) |
| **Mission Integration** | 0 days |
| **Patterns Extracted** | 4 patterns |
| **Optimizations Applied** | Day 3 bitset (15× speedup) |

**1-Second Goal**: 🎯 0.071ms / 1000ms (0.007%)

---

## 📈 Performance Table

| Day | Part 1 | Part 2 | Total | Algorithm | Mission | Notes |
|-----|--------|--------|-------|-----------|---------|-------|
| [1](days/day01.md) | 24.4µs | 25.4µs | 25.6µs | Group parsing + sort | - | Split by blank lines, max/top-3 · [Guide →](days/day01_function_guide.md) |
| [2](days/day02.md) | - | - | 21.5µs | Lookup tables | - | 3×3 precomputed scores, byte indexing · [Guide →](days/day02_function_guide.md) |
| [3](days/day03.md) | - | - | ~~359.2µs~~ **23.8µs** | Bitset intersection | - | **Optimized**: HashSet→Bitset (15× faster) · [Guide →](days/day03_function_guide.md) |
| - | - | - | - | - | - | Not yet solved |

**Cumulative Runtime**: 70.9µs (0.071ms)  
**Optimization Impact**: Day 3 bitset reduced total from 406.3µs → 70.9µs (5.7× improvement)

---

## 🔍 Quick Navigation

**By Day**: 
- [Day 1](days/day01.md) - Calorie Counting ✅ | [Function Guide](days/day01_function_guide.md) | [Code](../../aoc2022/src/solver/day01.rs)
- [Day 2](days/day02.md) - Rock Paper Scissors ✅ | [Function Guide](days/day02_function_guide.md) | [Code](../../aoc2022/src/solver/day02.rs)
- [Day 3](days/day03.md) - Rucksack Reorganization ✅ | [Function Guide](days/day03_function_guide.md) | [Code](../../aoc2022/src/solver/day03.rs)
- Day 4-25: Not yet started

**All Days**: [Days Directory](days/README.md)

**Function Guides**: 
- [Day 1 Function Guide](days/day01_function_guide.md) - Group parsing, max/top-k patterns
- [Day 2 Function Guide](days/day02_function_guide.md) - Lookup tables, branchless scoring, byte manipulation
- [Day 3 Function Guide](days/day03_function_guide.md) - Set intersection, chunking, ASCII priority mapping

**Daily Notes**:
- [[zettelkasten/Daily Notes/]] - Check Feb 2026 entries for solving notes

---

## 🎯 Algorithms Used

*Updated as days are completed*

### By Category
- **Parsing**: Day 1 (group splitting), Day 2 (byte manipulation), Day 3 (string splitting)
- **Sorting**: Day 1 (top-k)
- **Lookup Tables**: Day 2 (3×3 precomputed scores)
- **Set Operations**: Day 3 (bitset intersection - optimized from HashSet)
- **Bit Manipulation**: Day 3 (u128 bitset for ASCII set operations)
- **Grid**: Day -
- **Graph**: Day -
- **DP**: Day -
- **Math**: Day -
- **Simulation**: Day -

### Complexity Analysis
- **O(1)**: Day 2 (lookup per round), Day 3 (bitset operations)
- **O(log n)**: Day -
- **O(n)**: Day 1 (parsing), Day 2 (iteration), Day 3 (bitset construction)
- **O(n * m)**: Day 3 HashSet (replaced by O(n) bitset)
- **O(n log n)**: Day 1 (sorting for top-3)
- **O(n²)**: Day -
- **O(2ⁿ)**: Day -

---

## 🎨 Patterns Catalog

*Extracted after Day 25 completion*

**Preliminary Notes**:
- **Group splitting pattern** (Day 1): Using `.split("\n\n")` for blank-line delimited groups
- **Parse-once pattern** (Days 1-3): Separate parsing from solving, reuse parsed data for both parts (49% speedup)
- **Lookup table pattern** (Day 2): Replace branches with array indexing when input space is small (3×3 = 9 cases)
- **Set intersection pattern** (Day 3): ~~HashSet~~ → Bitset for finding common elements (15× speedup when domain is bounded)
- **Bitset optimization** (Day 3): Use u128 bitmask for ASCII set operations - zero allocations, single AND instruction (359µs → 23.8µs)
- **Chunking pattern** (Day 3): Process data in fixed-size groups using `.chunks(n)`
- **Group splitting pattern** (Day 1): Using `.split("\n\n")` for blank-line delimited groups
- **Parse-once pattern** (Day 1): Separate parsing from solving, reuse parsed data for both parts (49% speedup)

---

## 🚀 Mission Integration

| Mission | Days Used | Components |
|---------|-----------|------------|
| Mission 6 (Grid) | - | - |
| Mission 8 (Graph) | - | - |
| Mission 5 (HashSet) | - | - |
| Mission 2 (Queue) | - | - |
| Mission 10 (Union-Find) | - | - |

**Integration Rate**: -% of days used mission components

---

## ⚡ Optimization Wins

*Documented when significant optimizations are made (>2x speedup)*

### Day X: [Optimization Name]
**Before**: XXX.Xms  
**After**: XX.Xµs  
**Speedup**: XXXx faster  
**Technique**: [Brief description]

---

## 🎓 Learning Highlights

4. **Lookup tables eliminate branches** - 3x faster than match-based scoring for small input spaces
5. **Byte manipulation for ASCII** - `wrapping_sub` provides safe, fast char→int conversion
6. **Cache locality matters** - 18-byte lookup table fits in single cache line for ~100% hit rate

**Rust Techniques Applied**:
- Iterator chains (`.split()` → `.map()` → `.collect()`)
- Error handling with `.ok()` and `filter_map`
- Custom comparators for descending sort: `|a, b| b.cmp(a)`
- `sort_unstable_by` for faster sorting when order doesn't matter
- `as_bytes()` for raw byte access (faster than char iteration)
- `wrapping_sub` for safe arithmetic without panics
- Const arrays for compile-time lookup tables
- Iterator chains (`.split()` → `.map()` → `.collect()`)
- Error handling with `.ok()` and `filter_map`
- Custom comparators for descending sort: `|a, b| b.cmp(a)`
- Day 2: Lookup table pattern, branchless optimization, byte manipulation, modular arithmetic alternatives

### Days with Quick Solutions
- Day 1: 25.6µs - straightforward group parsing and sorting
- Day 2: 21.5µs - optimal lookup table approach (already near-perfect)

## 📝 Implementation Notes2 (Day 2 complete with benchmarks)  
**Next Update**: After Day 3
### Days with Comprehensive Function Guides
- Day 1: Full breakdown of parsing, max/top-k patterns, performance analysis

### Days with Quick Solutions
- Day 1: 25.6µs - straightforward group parsing and sorting

---

**Last Updated**: 2026-02-03 (Day 3 bitset optimization applied - 15× speedup)  
**Next Update**: After Day 4 completion  
**Target Completion**: December 2022 (retroactive solving in 2026)

---

## 🔧 Optimization Notes

### Day 3: HashSet → Bitset (15× speedup)
- **Before**: 359.2µs (HashSet allocations dominated 87% of runtime)
- **After**: 23.8µs (u128 bitset, zero heap allocations)
- **Technique**: Represent ASCII character set as 128-bit bitmask
- **Key Insight**: Intersection = single bitwise AND vs HashSet iteration
- **Trade-off**: Only works for bounded character sets (perfect for a-z, A-Z)
- **Impact**: Reduced total AoC 2022 runtime from 406.3µs → 70.9µs (5.7× overall improvement)
