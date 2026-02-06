# AoC 2022 - Summary

**Status**: 🎯 In Progress (6/25 complete)

---

## 📊 Stats Dashboard

| Metric | Value |
|--------|-------|
| **Progress** | 6/25 |
| **Total Runtime** | 195.0µs (bitset optimized) |
| **Average per Day** | 32.5µs |
| **Fastest Day** | Day 6 (11.4µs) |
| **Slowest Day** | Day 5 (85.0µs) |
| **Mission Integration** | 0 days |
| **Patterns Extracted** | 8 patterns |
| **Optimizations Applied** | Day 3 bitset (15× speedup) |

**1-Second Goal**: 🎯 0.195ms / 1000ms (0.020%)

---

## 📈 Performance Table

| Day | Part 1 | Part 2 | Total | Algorithm | Mission | Notes |
|-----|--------|--------|-------|-----------|---------|-------|
| [1](days/day01.md) | 24.4µs | 25.4µs | 25.6µs | Group parsing + sort | - | Split by blank lines, max/top-3 · [Guide →](days/day01_function_guide.md) |
| [2](days/day02.md) | - | - | 21.5µs | Lookup tables | - | 3×3 precomputed scores, byte indexing · [Guide →](days/day02_function_guide.md) |
| [3](days/day03.md) | - | - | ~~359.2µs~~ **23.8µs** | Bitset intersection | - | **Optimized**: HashSet→Bitset (15× faster) · [Guide →](days/day03_function_guide.md) |
| [4](days/day04.md) | - | - | 27.7µs | Range overlap | - | Endpoint comparison, O(1) checks · [Guide →](days/day04_function_guide.md) |
| [5](days/day05.md) | 66.3µs | 78.7µs | 85.0µs | Stack simulation | - | ASCII art parsing, Vec as stack · [Guide →](days/day05_function_guide.md) |
| [6](days/day06.md) | 1.46µs | 10.06µs | 11.4µs | Sliding window + bitset | - | No parse step, `.windows()` iterator · [Guide →](days/day06_function_guide.md) |
| - | - | - | - | - | - | Not yet solved |

**Cumulative Runtime**: 195.0µs (0.195ms)  
**Optimization Impact**: Day 3 bitset reduced total from 519.0µs → 183.6µs (2.8× improvement)

---

## 🔍 Quick Navigation

**By Day**: 
- [Day 1](days/day01.md) - Calorie Counting ✅ | [Function Guide](days/day01_function_guide.md) | [Code](../../aoc2022/src/solver/day01.rs)
- [Day 2](days/day02.md) - Rock Paper Scissors ✅ | [Function Guide](days/day02_function_guide.md) | [Code](../../aoc2022/src/solver/day02.rs)
- [Day 3](days/day03.md) - Rucksack Reorganization ✅ | [Function Guide](days/day03_function_guide.md) | [Code](../../aoc2022/src/solver/day03.rs)
- [Day 4](days/day04.md) - Camp Cleanup ✅ | [Function Guide](days/day04_function_guide.md) | [Code](../../aoc2022/src/solver/day04.rs)
- [Day 5](days/day05.md) - Supply Stacks ✅ | [Function Guide](days/day05_function_guide.md) | [Code](../../aoc2022/src/solver/day05.rs)
- [Day 6](days/day06.md) - Tuning Trouble ✅ | [Function Guide](days/day06_function_guide.md) | [Code](../../aoc2022/src/solver/day06.rs)
- Day 7-25: Not yet started

**All Days**: [Days Directory](days/README.md)

**Function Guides**: 
- [Day 1 Function Guide](days/day01_function_guide.md) - Group parsing, max/top-k patterns
- [Day 2 Function Guide](days/day02_function_guide.md) - Lookup tables, branchless scoring, byte manipulation
- [Day 3 Function Guide](days/day03_function_guide.md) - Set intersection, chunking, ASCII priority mapping
- [Day 4 Function Guide](days/day04_function_guide.md) - Range containment/overlap, interval arithmetic
- [Day 5 Function Guide](days/day05_function_guide.md) - Stack simulation, ASCII art parsing, Vec operations
- [Day 6 Function Guide](days/day06_function_guide.md) - Sliding window, bitset uniqueness, `.windows()` iterator

**Daily Notes**:
- [[zettelkasten/Daily Notes/]] - Check Feb 2026 entries for solving notes

---

## 🎯 Algorithms Used

*Updated as days are completed*
- **Parsing**: Day 1 (groups), Day 2 (lines), Day 3 (chars), Day 4 (range parsing), Day 5 (ASCII art), Day 6 (none — raw bytes)
- **Sorting**: Day 1 (top-k)
- **Lookup Tables**: Day 2 (3×3 precomputed scores)
- **Set Operations**: Day 3 (bitset intersection - optimized from HashSet)
- **Bit Manipulation**: Day 3 (u128 bitset for ASCII set operations), Day 6 (u32 bitset for uniqueness/popcount)
- **Range/Interval Operations**: Day 4 (containment, overlap)
- **Stack Operations**: Day 5 (Vec push/pop, split_off/extend)
- **Sliding Window**: Day 6 (`.windows(n)` iterator for fixed-size window)
- **Simulation**: Day 5 (crane operations)
- **Grid**: Day -
- **Graph**: Day -
- **DP**: Day -
- **Math**: Day 4 (interval arithmetic, set theory)

### Complexity Analysis
- **O(1)**: Day 2 (lookup per round), Day 3 (bitset operations), Day 4 (range comparisons), Day 5 (push/pop), Day 6 (popcount per window)
- **O(log n)**: Day -
- **O(n)**: Day 1 (parsing), Day 2 (iteration), Day 3 (bitset construction), Day 4 (parsing + filtering), Day 5 (parsing ASCII art)
- **O(n × w)**: Day 6 (sliding window: n positions × w window size)
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
- **Sliding window + bitset** (Day 6): `.windows(n)` with u32 popcount for uniqueness — zero allocations, ~11µs total
- **Parameterized core** (Day 6): Single `find_marker(input, window_size)` function serves both parts — differ only in a constant
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
- `.windows(n)` for zero-allocation sliding windows (Day 6)
- `.position()` for short-circuit first-match search (Day 6)
- `count_ones()` / popcount for hardware-accelerated bit counting (Day 6)

### Days with Quick Solutions
- Day 1: 25.6µs - straightforward group parsing and sorting
- Day 2: 21.5µs - optimal lookup table approach (already near-perfect)
- Day 3: 23.8µs - after bitset optimization (originally 359µs with HashSet)
- Day 4: 27.7µs - simple range endpoint comparisons, parsing dominates 95% of runtime
- Day 6: 11.4µs - fastest day, no parsing needed, sliding window + bitset

### Days with Comprehensive Function Guides
- Day 1: Full breakdown of parsing, max/top-k patterns, performance analysis
- Day 2: Lookup tables, branchless optimization, byte manipulation
- Day 3: Set intersection, bitset optimization (15× speedup)
- Day 4: Range operations, interval arithmetic, mathematical foundations
- Day 5: Stack simulation, ASCII art parsing, Vec operations, Part 1 vs Part 2 differences
- Day 6: Sliding window, bitset uniqueness, `.windows()` iterator, zero-allocation design

---

**Last Updated**: 2026-02-06 (Day 6 complete)  
**Next Update**: After Day 7

---

## 🔧 Optimization Notes

### Day 3: HashSet → Bitset (15× speedup)
- **Before**: 359.2µs (HashSet allocations dominated 87% of runtime)
- **After**: 23.8µs (u128 bitset, zero heap allocations)
- **Technique**: Represent ASCII character set as 128-bit bitmask
- **Key Insight**: Intersection = single bitwise AND vs HashSet iteration
- **Trade-off**: Only works for bounded character sets (perfect for a-z, A-Z)
- **Impact**: Reduced total AoC 2022 runtime from 406.3µs → 70.9µs (5.7× overall improvement)
