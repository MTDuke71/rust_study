# AoC 2022 - Summary

**Status**: 🎯 In Progress (5/25 complete)

---

## 📊 Stats Dashboard

| Metric | Value |
|--------|-------|
| **Progress** | 5/25 |
| **Total Runtime** | 183.6µs (bitset optimized) |
| **Average per Day** | 36.7µs |
| **Fastest Day** | Day 2 (21.5µs) |
| **Slowest Day** | Day 5 (85.0µs) |
| **Mission Integration** | 0 days |
| **Patterns Extracted** | 6 patterns |
| **Optimizations Applied** | Day 3 bitset (15× speedup) |

**1-Second Goal**: 🎯 0.184ms / 1000ms (0.018%)

---

## 📈 Performance Table

| Day | Part 1 | Part 2 | Total | Algorithm | Mission | Notes |
|-----|--------|--------|-------|-----------|---------|-------|
| [1](days/day01.md) | 24.4µs | 25.4µs | 25.6µs | Group parsing + sort | - | Split by blank lines, max/top-3 · [Guide →](days/day01_function_guide.md) |
| [2](days/day02.md) | - | - | 21.5µs | Lookup tables | - | 3×3 precomputed scores, byte indexing · [Guide →](days/day02_function_guide.md) |
| [3](days/day03.md) | - | - | ~~359.2µs~~ **23.8µs** | Bitset intersection | - | **Optimized**: HashSet→Bitset (15× faster) · [Guide →](days/day03_function_guide.md) |
| [4](days/day04.md) | - | - | 27.7µs | Range overlap | - | Endpoint comparison, O(1) checks · [Guide →](days/day04_function_guide.md) |
| [5](days/day05.md) | 66.3µs | 78.7µs | 85.0µs | Stack simulation | - | ASCII art parsing, Vec as stack · [Guide →](days/day05_function_guide.md) |
| - | - | - | - | - | - | Not yet solved |

**Cumulative Runtime**: 183.6µs (0.184ms)  
**Optimization Impact**: Day 3 bitset reduced total from 519.0µs → 183.6µs (2.8× improvement)

---

## 🔍 Quick Navigation

**By Day**: 
- [Day 1](days/day01.md) - Calorie Counting ✅ | [Function Guide](days/day01_function_guide.md) | [Code](../../aoc2022/src/solver/day01.rs)
- [Day 2](days/day02.md) - Rock Paper Scissors ✅ | [Function Guide](days/day02_function_guide.md) | [Code](../../aoc2022/src/solver/day02.rs)
- [Day 3](days/day03.md) - Rucksack Reorganization ✅ | [Function Guide](days/day03_function_guide.md) | [Code](../../aoc2022/src/solver/day03.rs)
- [Day 4](days/day04.md) - Camp Cleanup ✅ | [Function Guide](days/day04_function_guide.md) | [Code](../../aoc2022/src/solver/day04.rs)
- [Day 5](days/day05.md) - Supply Stacks ✅ | [Function Guide](days/day05_function_guide.md) | [Code](../../aoc2022/src/solver/day05.rs)
- Day 6-25: Not yet started

**All Days**: [Days Directory](days/README.md)

**Function Guides**: 
- [Day 1 Function Guide](days/day01_function_guide.md) - Group parsing, max/top-k patterns
- [Day 2 Function Guide](days/day02_function_guide.md) - Lookup tables, branchless scoring, byte manipulation
- [Day 3 Function Guide](days/day03_function_guide.md) - Set intersection, chunking, ASCII priority mapping
- [Day 4 Function Guide](days/day04_function_guide.md) - Range containment/overlap, interval arithmetic
- [Day 5 Function Guide](days/day05_function_guide.md) - Stack simulation, ASCII art parsing, Vec operations

**Daily Notes**:
- [[zettelkasten/Daily Notes/]] - Check Feb 2026 entries for solving notes

---

## 🎯 Algorithms Used

*Updated as days are completed*
- **Parsing**: Day 1 (groups), Day 2 (lines), Day 3 (chars), Day 4 (range parsing), Day 5 (ASCII art)
- **Sorting**: Day 1 (top-k)
- **Lookup Tables**: Day 2 (3×3 precomputed scores)
- **Set Operations**: Day 3 (bitset intersection - optimized from HashSet)
- **Bit Manipulation**: Day 3 (u128 bitset for ASCII set operations)
- **Range/Interval Operations**: Day 4 (containment, overlap)
- **Stack Operations**: Day 5 (Vec push/pop, split_off/extend)
- **Simulation**: Day 5 (crane operations)
- **Grid**: Day -
- **Graph**: Day -
- **DP**: Day -
- **Math**: Day 4 (interval arithmetic, set theory)

### Complexity Analysis
- **O(1)**: Day 2 (lookup per round), Day 3 (bitset operations), Day 4 (range comparisons), Day 5 (push/pop)
- **O(log n)**: Day -
- **O(n)**: Day 1 (parsing), Day 2 (iteration), Day 3 (bitset construction), Day 4 (parsing + filtering), Day 5 (parsing ASCII art)
- **O(n * m)**: Day 3 HashSet (replaced by O(n) bitset), Day 5 (simulation: moves × crates)
- **O(n log n)**: Day 1 (sorting for top-3)
- **O(n²)**: Day -
- **O(2ⁿ)**: Day -

---

## 🎨 Patterns Catalog

*Extracted after Day 25 completion*
- **Parse-once pattern** (Day 1, 4, 5): Separate parsing from solving, reuse parsed data for both parts
- **Lookup table pattern** (Day 2): Replace branches with array indexing when input space is small (3×3 = 9 cases)
- **Set intersection pattern** (Day 3): ~~HashSet~~ → Bitset for finding common elements (15× speedup when domain is bounded)
- **Bitset optimization** (Day 3): Use u128 bitmask for ASCII set operations - zero allocations, single AND instruction (359µs → 23.8µs)
- **Chunking pattern** (Day 3): Process data in fixed-size groups using `.chunks(n)`
- **Range operations pattern** (Day 4): Endpoint comparison for containment/overlap - O(1) instead of enumerating elements
- **Type-driven design** (Day 4): Create domain types (`Range`, `RangePair`) that encapsulate logic and prevent errors
- **ASCII art parsing** (Day 5): Character position extraction at fixed columns (1, 5, 9, 13...), bottom-to-top processing
- **Vec as stack** (Day 5): Natural LIFO with push/pop; `split_off` for bulk operations preserving order
- **Group splitting pattern** (Day 1, 5): Using `.split("\n\n")` for blank-line delimited sections
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
7. **Range endpoint comparison** - No need to enumerate elements; containment/overlap determined by comparing start/end
8. **Type-driven design** - Custom types (`Range`, `RangePair`) encode domain logic and prevent mixing up unrelated values
6. **Cache locality matters** - 18-byte lookup table fits in single cache line for ~100% hit rate

**Rust Techniques Applied**:
- Iterator chains (`.split()` → `.map()` → `.collect()`)
- Error handling with `.ok()` and `filter_map`
- Custom comparators for descending sort: `|a, b| b.cmp(a)`
- `sort_unstable_by` for faster sorting when order doesn't matter
- `as_bytes()` for raw byte access (faster than char iteration)
- `wrapping_sub` for safe arithmetic without panics
- Const arrays for compile-time lookup tables
- `split_off()` and `extend()` for bulk Vec operations (Day 5)
- Character position indexing in strings for ASCII art parsing (Day 5)

### Days with Quick Solutions
- Day 1: 25.6µs - straightforward group parsing and sorting
- Day 2: 21.5µs - optimal lookup table approach (already near-perfect)
- Day 3: 23.8µs - after bitset optimization (originally 359µs with HashSet)
- Day 4: 27.7µs - simple range endpoint comparisons, parsing dominates 95% of runtime

### Days with Comprehensive Function Guides
- Day 1: Full breakdown of parsing, max/top-k patterns, performance analysis
- Day 2: Lookup tables, branchless optimization, byte manipulation
- Day 3: Set intersection, bitset optimization (15× speedup)
- Day 4: Range operations, interval arithmetic, mathematical foundations
- Day 5: Stack simulation, ASCII art parsing, Vec operations, Part 1 vs Part 2 differences

---

**Last Updated**: 2026-02-05 (Day 5 complete)  
**Next Update**: After Day 6

---

## 🔧 Optimization Notes

### Day 3: HashSet → Bitset (15× speedup)
- **Before**: 359.2µs (HashSet allocations dominated 87% of runtime)
- **After**: 23.8µs (u128 bitset, zero heap allocations)
- **Technique**: Represent ASCII character set as 128-bit bitmask
- **Key Insight**: Intersection = single bitwise AND vs HashSet iteration
- **Trade-off**: Only works for bounded character sets (perfect for a-z, A-Z)
- **Impact**: Reduced total AoC 2022 runtime from 406.3µs → 70.9µs (5.7× overall improvement)
