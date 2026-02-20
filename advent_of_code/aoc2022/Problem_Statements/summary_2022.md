# AoC 2022 - Summary

**Status**: 🎯 In Progress (19/25 complete)

---

## 📊 Stats Dashboard

| Metric | Value |
|--------|-------|
| **Progress** | 19/25 |
| **Total Runtime** | 23.84ms |
| **Average per Day** | 1.25ms |
| **Fastest Day** | Day 6 (4.80µs) |
| **Slowest Day** | Day 16 (4.94ms) |
| **Mission Integration** | 2 days (Day 9, Day 14: Mission 6 Grid) |
| **Patterns Extracted** | 22 patterns |
| **Optimizations Applied** | Day 3 bitset (15×), Day 6 rolling XOR (2.4×), Day 7 HashMap→Stack (23×), Day 8 Rayon (1.5×), Day 9 FxHashSet (1.25×), Day 12 backward BFS (164×!), Day 13 counting (33×!), **Day 15 line-based search (27,000×!)** |

**1-Second Goal**: 🎯 23.84ms / 1000ms (2.4%)

---

## 📈 Performance Table

| Day | Part 1 | Part 2 | Total | Algorithm | Mission | Notes |
|-----|--------|--------|-------|-----------|---------|-------|
| [1](days/day01.md) | 24.4µs | 25.4µs | 25.6µs | Group parsing + sort | - | Split by blank lines, max/top-3 · [Guide →](days/day01_function_guide.md) |
| [2](days/day02.md) | - | - | 21.5µs | Lookup tables | - | 3×3 precomputed scores, byte indexing · [Guide →](days/day02_function_guide.md) |
| [3](days/day03.md) | - | - | ~~359.2µs~~ **23.8µs** | Bitset intersection | - | **Optimized**: HashSet→Bitset (15× faster) · [Guide →](days/day03_function_guide.md) |
| [4](days/day04.md) | - | - | 27.7µs | Range overlap | - | Endpoint comparison, O(1) checks · [Guide →](days/day04_function_guide.md) |
| [5](days/day05.md) | 66.3µs | 78.7µs | 85.0µs | Stack simulation | - | ASCII art parsing, Vec as stack · [Guide →](days/day05_function_guide.md) |
| [6](days/day06.md) | ~~1.46µs~~ **1.11µs** | ~~10.06µs~~ **3.69µs** | ~~11.4µs~~ **4.80µs** | Rolling XOR bitset | - | **Optimized**: Rebuild→Rolling XOR (2.4× faster) · [Guide →](days/day06_function_guide.md) |
| [7](days/day07.md) | - | - | 9.32µs | Stack accumulation | - | **Optimized**: HashMap→Stack (23×), parse-once (2×) · [Guide →](days/day07_function_guide.md) |
| [8](days/day08.md) | - | - | ~~262µs~~ **174µs** | Grid visibility + parallel | - | **Optimized**: Rayon row-parallel (1.5×) · [Guide →](days/day08_function_guide.md) |
| [9](days/day09.md) | 180µs | 388µs | ~~756µs~~ **568µs** | Rope physics simulation | Mission 6 | **Optimized**: FxHashSet (20×), parse-once · [Guide →](days/day09_function_guide.md) |
| [10](days/day10.md) | 7.3µs | 7.3µs | ~~14.4µs~~ **7.5µs** | CPU cycle simulation | - | **Optimized**: Parse-once (48×) · [Guide →](days/day10_function_guide.md) |
| [11](days/day11.md) | 9.3µs | 3.0ms | **2.92ms** | Monkey simulation + modular arithmetic | - | Part 2: 10K rounds, modulo optimization · [Guide →](days/day11_function_guide.md) |
| [12](days/day12.md) | 484 | 478 | ~~28.89ms~~ ~~3.33ms~~ **435µs** | BFS pathfinding + backward search | - | **Optimized**: Backward BFS from E (164×) · [Guide →](days/day12_function_guide.md) |
| [13](days/day13.md) | 5852 | 24190 | ~~721µs~~ **341µs** | Packet comparison + counting | - | **Optimized**: Count instead of sort (33×) · [Guide →](days/day13_function_guide.md) |
| [14](days/day14.md) | 763 | 23921 | **8.62ms** | Sand simulation + grid | Mission 6 | Coordinate normalization, physics simulation · [Guide →](days/day14_function_guide.md) |
| [15](days/day15.md) | 4876693 | 11645454855041 | ~~432.52ms~~ ~~45.02ms~~ **23.02µs** | Interval merging + line-based search | - | **Optimized**: Line-based (Feng method) 27,000× faster! · [Guide →](days/day15_function_guide.md) |
| [16](days/day16.md) | 1638 | 2400 | **4.94ms** | Graph compression + bitmask DFS + SOS DP | - | BFS distances, bitmask partition for 2 actors · [Guide →](days/day16_function_guide.md) |
| [17](days/day17.md) | 3109 | 1541449275365 | **734µs** | Tetris simulation + cycle detection | - | Bitmask rows, top-30 profile fingerprinting · [Guide →](days/day17_function_guide.md) |
| [18](days/day18.md) | 3522 | 2074 | **1.38ms** | 3D surface area + BFS flood fill | - | HashSet neighbor lookup, exterior-only via flood · [Guide →](days/day18_function_guide.md) |
| [19](days/day19.md) | 1413 | 21080 | **3.50ms** | DFS + branch-and-bound | - | Skip-to-build, robot caps, triangle upper bound · [Guide →](days/day19_function_guide.md) |
| - | - | - | - | - | - | Not yet solved |

**Cumulative Runtime**: 23.84ms
**Optimization Impact**: Day 3 bitset (15×), Day 6 rolling XOR (2.4×), Day 7 HashMap→Stack (23×), Day 8 Rayon row-parallel (1.5×), Day 9 FxHashSet (1.25×), Day 10 parse-once (2×), Day 11 modular arithmetic (prevents overflow), Day 12 backward BFS from goal (164×! - from 28.74ms → 175µs), Day 13 count positions instead of sorting (33×! - from 338µs → 10µs for Part 2), **Day 15 line-based search (Feng method) (27,000×! - from 460.89ms → 17.07µs for Part 2)**

---

## 🔍 Quick Navigation

**By Day**: 
- [Day 1](days/day01.md) - Calorie Counting ✅ | [Function Guide](days/day01_function_guide.md) | [Code](../../aoc2022/src/solver/day01.rs)
- [Day 2](days/day02.md) - Rock Paper Scissors ✅ | [Function Guide](days/day02_function_guide.md) | [Code](../../aoc2022/src/solver/day02.rs)
- [Day 3](days/day03.md) - Rucksack Reorganization ✅ | [Function Guide](days/day03_function_guide.md) | [Code](../../aoc2022/src/solver/day03.rs)
- [Day 4](days/day04.md) - Camp Cleanup ✅ | [Function Guide](days/day04_function_guide.md) | [Code](../../aoc2022/src/solver/day04.rs)
- [Day 5](days/day05.md) - Supply Stacks ✅ | [Function Guide](days/day05_function_guide.md) | [Code](../../aoc2022/src/solver/day05.rs)
- [Day 6](days/day06.md) - Tuning Trouble ✅ | [Function Guide](days/day06_function_guide.md) | [Code](../../aoc2022/src/solver/day06.rs)
- [Day 7](days/day07.md) - No Space Left On Device ✅ | [Function Guide](days/day07_function_guide.md) | [Code](../../aoc2022/src/solver/day07.rs)
- [Day 8](days/day08.md) - Treetop Tree House ✅ | [Function Guide](days/day08_function_guide.md) | [Code](../../aoc2022/src/solver/day08.rs)
- [Day 9](days/day09.md) - Rope Bridge ✅ | [Function Guide](days/day09_function_guide.md) | [Code](../../aoc2022/src/solver/day09.rs)
- [Day 10](days/day10.md) - Cathode-Ray Tube ✅ | [Function Guide](days/day10_function_guide.md) | [Code](../../aoc2022/src/solver/day10.rs)
- [Day 11](days/day11.md) - Monkey in the Middle ✅ | [Function Guide](days/day11_function_guide.md) | [Code](../../aoc2022/src/solver/day11.rs)
- [Day 12](days/day12.md) - Hill Climbing Algorithm ✅ | [Function Guide](days/day12_function_guide.md) | [Code](../../aoc2022/src/solver/day12.rs)
- [Day 13](days/day13.md) - Distress Signal ✅ | [Function Guide](days/day13_function_guide.md) | [Code](../../aoc2022/src/solver/day13.rs)
- [Day 14](days/day14.md) - Regolith Reservoir ✅ | [Function Guide](days/day14_function_guide.md) | [Code](../../aoc2022/src/solver/day14.rs)
- [Day 15](days/day15.md) - Beacon Exclusion Zone ✅ | [Function Guide](days/day15_function_guide.md) | [Code](../../aoc2022/src/solver/day15.rs)
- [Day 16](days/day16.md) - Proboscidea Volcanium ✅ | [Function Guide](days/day16_function_guide.md) | [Code](../../aoc2022/src/solver/day16.rs)
- [Day 17](days/day17.md) - Pyroclastic Flow ✅ | [Function Guide](days/day17_function_guide.md) | [Code](../../aoc2022/src/solver/day17.rs)
- [Day 18](days/day18.md) - Boiling Boulders ✅ | [Function Guide](days/day18_function_guide.md) | [Code](../../aoc2022/src/solver/day18.rs)
- [Day 19](days/day19.md) - Not Enough Minerals ✅ | [Function Guide](days/day19_function_guide.md) | [Code](../../aoc2022/src/solver/day19.rs)
- Day 20-25: Not yet started

**All Days**: [Days Directory](days/README.md)

**Function Guides**: 
- [Day 1 Function Guide](days/day01_function_guide.md) - Group parsing, max/top-k patterns
- [Day 2 Function Guide](days/day02_function_guide.md) - Lookup tables, branchless scoring, byte manipulation
- [Day 3 Function Guide](days/day03_function_guide.md) - Set intersection, chunking, ASCII priority mapping
- [Day 4 Function Guide](days/day04_function_guide.md) - Range containment/overlap, interval arithmetic
- [Day 5 Function Guide](days/day05_function_guide.md) - Stack simulation, ASCII art parsing, Vec operations
- [Day 6 Function Guide](days/day06_function_guide.md) - Rolling XOR bitset, 3-version optimization journey, branchless design
- [Day 7 Function Guide](days/day07_function_guide.md) - Stack-based size accumulation, HashMap→Stack optimization (23×)
- [Day 8 Function Guide](days/day08_function_guide.md) - Grid visibility, directional iteration, scenic score calculation
- [Day 9 Function Guide](days/day09_function_guide.md) - Rope physics, signum() diagonal movement, cascade following
- [Day 10 Function Guide](days/day10_function_guide.md) - CPU cycle simulation, CRT rendering, sprite overlap detection
- [Day 11 Function Guide](days/day11_function_guide.md) - Monkey simulation, modular arithmetic, worry level management
- [Day 12 Function Guide](days/day12_function_guide.md) - BFS pathfinding, grid neighbors, backward search (164× speedup!)
- [Day 13 Function Guide](days/day13_function_guide.md) - Recursive packet comparison, JSON parsing, sorting with custom comparator
- [Day 14 Function Guide](days/day14_function_guide.md) - Sand simulation, coordinate normalization, Mission 6 Grid integration
- [Day 15 Function Guide](days/day15_function_guide.md) - Interval merging, Manhattan distance, sparse coverage scanning
- [Day 16 Function Guide](days/day16_function_guide.md) - Graph compression, bitmask DFS, SOS DP for two-actor partition
- [Day 17 Function Guide](days/day17_function_guide.md) - Tetris simulation, bitmask rows, cycle detection for trillion-scale
- [Day 18 Function Guide](days/day18_function_guide.md) - 3D surface area, BFS flood fill, interior vs exterior faces
- [Day 19 Function Guide](days/day19_function_guide.md) - DFS branch-and-bound, skip-to-build, resource optimization

**Daily Notes**:
- [[zettelkasten/Daily Notes/]] - Check Feb 2026 entries for solving notes

---

## 🎯 Algorithms Used

*Updated as days are completed*
- **Parsing**: Day 1 (groups), Day 2 (lines), Day 3 (chars), Day 4 (range parsing), Day 5 (ASCII art), Day 6 (none — raw bytes), Day 9 (direction + count), Day 10 (instruction parsing), Day 11 (multi-line monkey blocks)
- **Sorting**: Day 1 (top-k), Day 11 (top-2 inspection counts)
- **Lookup Tables**: Day 2 (3×3 precomputed scores)
- **Set Operations**: Day 3 (bitset intersection - optimized from HashSet), Day 9 (FxHashSet for visited positions - optimized from std HashMap)
- **Bit Manipulation**: Day 3 (u128 bitset for ASCII set operations), Day 6 (u32 XOR bitset for rolling uniqueness), Day 17 (u8 row bitmasks for 7-wide chamber)
- **Range/Interval Operations**: Day 4 (containment, overlap), Day 15 (interval merging, gap finding)
- **Stack Operations**: Day 5 (Vec push/pop, split_off/extend)
- **Sliding Window**: Day 6 (rolling XOR bitset, O(1) per slide)
- **Stack-Based Traversal**: Day 7 (DFS-style filesystem accumulation)
- **Simulation**: Day 5 (crane operations), Day 9 (rope physics, knot following), Day 10 (CPU cycles, CRT rendering), Day 11 (monkey item passing, 20/10,000 rounds), Day 14 (sand physics, falling + settling), Day 17 (Tetris-like rock falling with jet pushes)
- **3D Geometry**: Day 18 (3D surface area, bounding box, 6-connected neighbors)
- **Grid**: Day 8 (2D visibility checks, directional iteration), Day 12 (BFS pathfinding, backward search), Day 14 (Mission 6 Grid, coordinate normalization, dynamic bounds)
- **Coordinate Systems**: Day 9 (signed 2D coords, signum() movement)
- **Queue Operations**: Day 11 (VecDeque for FIFO item passing)
- **Recursive Comparison**: Day 13 (nested list comparison with type coercion)
- **JSON Parsing**: Day 13 (serde_json for nested structures)
- **Custom Sorting**: Day 13 (sort with custom comparator finding divider packets)
- **Graph**: Day 12 (BFS pathfinding, elevation constraints, neighbor generation), Day 16 (graph compression via BFS all-pairs distances, state-space DFS)
- **BFS**: Day 12 (shortest path, multi-source BFS for Part 2), Day 16 (all-pairs shortest distances between important valves), Day 18 (3D flood fill from exterior)
- **DFS + Branch-and-Bound**: Day 19 (resource optimization, skip-to-build, triangle upper bound, robot caps)
- **DP**: Day 16 (SOS DP — Sum over Subsets for bitmask partition)
- **Cycle Detection**: Day 17 (state fingerprinting via HashMap for trillion-scale simulation)
- **Math**: Day 4 (interval arithmetic, set theory), Day 9 (Chebyshev distance, signum), Day 11 (modular arithmetic, Chinese Remainder Theorem concept), Day 15 (Manhattan distance, geometric coverage)

### Complexity Analysis
- **O(1)**: Day 2 (lookup per round), Day 3 (bitset operations), Day 4 (range comparisons), Day 5 (push/pop), Day 6 (XOR + popcount per slide), Day 9 (is_touching, follow, FxHashSet insert), Day 10 (cycle increment, pixel draw), Day 11 (VecDeque push/pop, modulo)
- **O(log n)**: Day -
- **O(n)**: Day 1 (parsing), Day 2 (iteration), Day 3 (bitset construction), Day 4 (parsing + filtering), Day 5 (parsing ASCII art), Day 6 (rolling XOR — window-size independent), Day 7 (single-pass stack accumulation), Day 9 (total steps × knots), Day 10 (240 cycles), Day 11 (parsing monkey blocks)
- **O(n * m)**: Day 3 HashSet (replaced by O(n) bitset), Day 5 (simulation: moves × crates)
- **O(n log n)**: Day 1 (sorting for top-3), Day 11 (sorting inspection counts for top-2)
- **O(n²)**: Day 8 (grid iteration: rows × cols)
- **O(rounds × monkeys × items)**: Day 11 (Part 1: 20 rounds, Part 2: 10,000 rounds)
- **O(n × m)**: Day 15 Part 2 (4M rows × n sensors, interval merging per row)
- **O(n! / (n-k)!)**: Day 16 Part 1 (permutations of valve orderings, pruned by time budget)
- **O(n × 2ⁿ)**: Day 16 Part 2 (SOS DP over bitmasks, n=15 important valves)
- **O(R × S)**: Day 17 (R = rocks simulated ~2K-3.4K, S = shape cells 4-5, cycle detection amortized O(1))
- **O(n + V)**: Day 18 (n cubes for Part 1, V bounding-box volume for Part 2 BFS flood fill)
- **O(n × B^d)**: Day 19 (n blueprints × DFS search, B~4 branching, d~time_limit depth, heavily pruned)

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
- **Rolling XOR bitset** (Day 6): XOR toggle + popcount for uniqueness — O(1) per slide, branchless (rebuild→freq→XOR: 11.4µs→6.0µs→4.8µs)
- **Parameterized core** (Day 6): Single `find_marker(input, window_size)` function serves both parts — differ only in a constant
- **Iterative optimization** (Day 6): Three-version journey: algorithm improvement (O(n×w)→O(n)), then implementation improvement (branches→branchless)
- **Stack-based DFS accumulation** (Day 7): cd/cd.. naturally form a stack; accumulate sizes on pop, eliminating tree construction entirely
- **Data structure elimination** (Day 7): HashMap<String, u64> → Vec<u64> stack (23× speedup). When you only need values, don't pay for keys.
- **Directional iteration pattern** (Day 8): Single function with signed offsets `(dr, dc)` handles all 4 directions, avoiding code duplication
- **Grid parsing pattern** (Day 8): `Vec<Vec<u8>>` from line-by-line character iteration, simple and cache-friendly
- **Group splitting pattern** (Day 1, 5): Using `.split("\n\n")` for blank-line delimited sections
- **Parse-once pattern** (Day 1): Separate parsing from solving, reuse parsed data for both parts (49% speedup)
- **Line-based geometric search (Feng method)** (Day 15): Manhattan distance diamonds → diagonal lines (y±x=c). Check ~15k line intersections instead of millions of points (27,000× speedup!)
- **Graph compression** (Day 16): Reduce large graph (55 nodes) to only important nodes (16) by precomputing BFS all-pairs shortest distances. Eliminates "hallway" nodes from search.
- **Bitmask DFS for ordering optimization** (Day 16): State = (position, opened_bitmask, time_remaining). Explores orderings of valve openings, pruning when time runs out. Similar to TSP with time budget.
- **SOS DP + bitmask partition** (Day 16): For two independent actors, record best result per bitmask, propagate with Sum over Subsets DP, then find best disjoint pair in O(2ⁿ). Avoids O(4ⁿ) brute-force pairing.
- **Bitmask row representation** (Day 17): Pack a fixed-width chamber (7 columns) into `u8` bitmasks — O(1) collision detection via bitwise AND, O(1) placement via bitwise OR. 1 byte per row instead of 7.
- **Cycle detection via state fingerprinting** (Day 17): For trillion-scale simulation, fingerprint state as (rock_type, jet_index, top_N_rows). HashMap lookup finds repeat → extrapolate with arithmetic. Simulates ~3,400 rocks instead of 10¹².
- **Exterior flood fill** (Day 18): BFS from outside padded bounding box to count only exterior surface faces. Water floods around the droplet; interior air pockets are unreachable. Simpler than detecting voids explicitly.
- **Skip-to-build DFS with branch-and-bound** (Day 19): Instead of deciding each minute (build or wait), jump to the time when enough resources accumulate for each robot type. Combined with robot caps (never overbuild), triangle-number upper bound, and prerequisite guards. Collapses branching from 5^T to ~thousands of states per blueprint.

---

## 🚀 Mission Integration

| Mission | Days Used | Components |
|---------|-----------|------------|
| Mission 6 (Grid) | Day 14 | Grid<Tile>, Coord (type-safe coordinates) |
| Mission 8 (Graph) | - | - |
| Mission 5 (HashSet) | - | - |
| Mission 2 (Queue) | - | - |
| Mission 10 (Union-Find) | - | - |

**Integration Rate**: 7.1% of days used mission components (1/14)

---

## ⚡ Optimization Wins

*Documented when significant optimizations are made (>2x speedup)*

### Day 6: Rebuild Bitset → Rolling XOR Bitset (2.4× speedup)
- **v1 (Rebuild bitset)**: 11.43µs — OR all `w` bytes per window, O(n×w)
- **v2 (Freq counter)**: 5.96µs — rolling add/remove, O(n) but 4 branches per slide
- **v3 (Rolling XOR)**: 4.80µs — XOR toggle + popcount, O(n) branchless
- **Technique**: XOR is its own inverse (`a ^ b ^ b == a`), so XOR-out leaving char and XOR-in entering char. Popcount == window_size means all unique.
- **Key Insight (from learner)**: "XOR out the exiting bit, XOR in the new bit" — eliminates both redundant work AND branches
- **Impact**: Part 2 improved most (10.06µs → 3.69µs = 2.7×) because v1's O(w) inner loop hurt more with w=14

### Day 7: HashMap → Stack Accumulation (23×) + Parse-Once (2×)
- **v1 (HashMap<String, u64>)**: 455µs — build path string keys per file × depth, hash lookups
- **v2 (Vec<u64> stack)**: 19.6µs — push/pop running totals, zero String allocations (23× faster)
- **v3 (Parse-once)**: 9.32µs — compute_dir_sizes() once, shared by both parts (2× faster)
- **Technique**: cd/cd.. naturally form a stack. Sizes accumulate at top; cd.. pops + rolls to parent.
- **Key Insight**: Only need *sizes*, not names. HashMap keys were overhead. Parse-once pattern avoids double work.
- **Impact**: Combined 49× speedup (455µs → 9.32µs)

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
- XOR toggle (`^=`) for branchless rolling window state (Day 6)
- `count_ones()` / popcount for hardware-accelerated bit counting (Day 3, 6)
- Direct byte indexing for O(1) sliding window access (Day 6)
- `Vec` as stack with `push(0)` / `pop()` for DFS accumulation (Day 7)
- `last_mut().unwrap()` for O(1) mutable access to stack top (Day 7)
- `starts_with()` for fast line classification without full parsing (Day 7)
- `saturating_sub()` for safe arithmetic avoiding underflow (Day 7)
- Direct 2D indexing `grid[row][col]` for cache-friendly grid access (Day 8)
- Signed `isize` offsets for directional movement with bounds checking (Day 8)

### Days with Quick Solutions
- Day 1: 25.6µs - straightforward group parsing and sorting
- Day 2: 21.5µs - optimal lookup table approach (already near-perfect)
- Day 3: 23.8µs - after bitset optimization (originally 359µs with HashSet)
- Day 4: 27.7µs - simple range endpoint comparisons, parsing dominates 95% of runtime
- Day 6: 4.80µs - fastest day, no parsing, rolling XOR bitset (optimized from 11.4µs)
- Day 7: 9.32µs - stack accumulation + parse-once (optimized from 455µs HashMap, then parse-once from 18.5µs)
- Day 8: 174µs - grid visibility with Rayon row-parallel (optimized from 483µs → 262µs → 174µs)

### Days with Comprehensive Function Guides
- Day 1: Full breakdown of parsing, max/top-k patterns, performance analysis
- Day 2: Lookup tables, branchless optimization, byte manipulation
- Day 3: Set intersection, bitset optimization (15× speedup)
- Day 4: Range operations, interval arithmetic, mathematical foundations
- Day 5: Stack simulation, ASCII art parsing, Vec operations, Part 1 vs Part 2 differences
- Day 6: Three-version optimization journey (rebuild→freq→XOR), rolling XOR bitset, branchless design
- Day 7: Stack-based DFS accumulation, HashMap→Stack optimization (23×), structural insight: sizes don't need names
- Day 8: Grid visibility, directional iteration pattern, visibility vs viewing distance algorithms

---

**Last Updated**: 2026-02-19 (Day 19 complete)
**Next Update**: After Day 20

---

## 🔧 Optimization Notes

### Day 3: HashSet → Bitset (15× speedup)
- **Before**: 359.2µs (HashSet allocations dominated 87% of runtime)
- **After**: 23.8µs (u128 bitset, zero heap allocations)
- **Technique**: Represent ASCII character set as 128-bit bitmask
- **Key Insight**: Intersection = single bitwise AND vs HashSet iteration
- **Trade-off**: Only works for bounded character sets (perfect for a-z, A-Z)
- **Impact**: Reduced total AoC 2022 runtime from 406.3µs → 70.9µs (5.7× overall improvement)

### Day 6: Rebuild Bitset → Rolling XOR (2.4× speedup)
- **Before**: 11.43µs (rebuild u32 bitset per window, O(n×w))
- **v2**: 5.96µs (rolling frequency counter, O(n) but 4 branches per slide)
- **After**: 4.80µs (rolling XOR bitset, O(n) branchless)
- **Technique**: XOR toggle + popcount — XOR is self-inverse, so XOR-out leaving / XOR-in entering
- **Key Insight**: Learner-driven — "XOR out the exiting bit, XOR in the new bit"
- **Trade-off**: Only works when window size ≤ 32 (u32 bits) and chars map to unique bit positions
- **Impact**: Part 2 improved most dramatically (10.06µs → 3.69µs) because O(w) inner loop eliminated

### Day 7: HashMap→Stack (23×) + Parse-Once (2×)
- **v1 (HashMap)**: 455µs — String allocations for path keys
- **v2 (Stack)**: 19.6µs — Vec<u64> running totals (23×)
- **v3 (Parse-once)**: 9.32µs — compute_dir_sizes() called once (2×)
- **Technique**: cd/cd.. = stack push/pop. Parse-once pattern: share parsed data between parts.
- **Key Insight**: Only need sizes, not names. Don't reparse when you can share data.
- **Trade-off**: Name-free stack can't handle directory revisits (works for AoC's strict DFS inputs).

### Day 8: Ring-Based Early Termination (1.08× speedup)
- **Before**: 483µs — check all 9,801 trees (99×99 grid)
- **After**: 445µs — skip outer rings when theoretical max ≤ current best (7.5% faster)
- **Technique**: Group trees by min edge distance, process center→edge, calculate theoretical max per ring
- **Key Insight**: Trees at distance `d` have theoretical max score = `d² × (rows-1-d) × (cols-1-d)`. Once current best exceeds this, skip all remaining outer rings.
- **Trade-off**: Ring grouping adds O(N²) setup overhead, but pruning saves checking ~27% of trees (rings d=0 to d=8 skipped)
- **Impact**: Modest improvement due to setup cost, but demonstrates mathematical pruning technique

### Day 8 Part 1: Pre-Compute Max Heights (1.7× speedup)
- **Before**: 445µs — O(N³) checking all 4 directions per tree
- **After**: 262µs — O(N²) with 4 precompute passes (41% faster, combined 1.85× from original 483µs)
- **Technique**: Four passes to compute max heights from each edge direction, then O(1) visibility check per tree
  ```
  max_from_left[row][col] = max of all trees from left edge to col-1
  max_from_right[row][col] = max of all trees from right edge to col+1  
  max_from_top[row][col] = max of all trees from top edge to row-1
  max_from_bottom[row][col] = max of all trees from bottom edge to row+1
  
  visible if: height > ANY of the 4 precomputed maxes
  ```
- **Key Insight**: Instead of scanning N trees in 4 directions (O(N) per tree), precompute once and check in O(1)
- **Trade-off**: 4 extra grids (4×N² memory), but reduces time complexity from O(N³) to O(N²)
- **Impact**: Major speedup - eliminated redundant directional scans
- **Impact**: 49× total speedup; parse-once saved 9.3µs (would add 455µs without any optimization)

### Day 8: Rayon Row-Based Parallelization (1.5× speedup)
- **Before**: 262µs — sequential processing with ring pruning + precompute
- **After**: 174µs — parallel row processing using Rayon (33% faster)
- **Technique**: Process each row in parallel using `into_par_iter()`, find max scenic score per row, then take global max
  ```rust
  (0..rows)
      .into_par_iter()
      .map(|row| {
          (0..cols).map(|col| scenic_score(grid, row, col)).max().unwrap_or(0)
      })
      .max()
      .unwrap_or(0)
  ```
- **Key Insight**: Row-level parallelism (99 tasks) beats fine-grained tree-level parallelism (9,801 tasks) due to lower thread overhead
- **Failed Approach**: Ring-based parallel (920µs, 3.5× slower) — thread pool overhead dominated with small per-tree work
- **Trade-off**: Gave up ring-based early termination (skipping 27% of trees) to gain multi-core parallelism (1.5× speedup on this system)
- **Work Unit Size Matters**: 99 rows × ~99 trees per row = ideal granularity for thread pool, vs. thousands of individual tree tasks creating excessive overhead
- **Learning**: Parallelization isn't always faster — need large enough work units to amortize thread spawning costs
