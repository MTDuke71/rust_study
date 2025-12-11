# Advent of Code 2025 - Problem Summary

This document provides a categorized overview of all Advent of Code 2025 problems, organized by day with problem types for both parts.

## Problem Categories

- **AABB Sampling**: Axis-Aligned Bounding Box optimization for large rectangle validation
- **Advanced Pattern Matching**: Complex pattern constraints, non-overlapping patterns
- **Brute Force**: Exhaustive search through solution space
- **Cellular Automaton**: Conway's Game of Life, state evolution, neighbor counting, grid simulation
- **Combinatorial Optimization**: Subset sum, container packing, constrained combination enumeration
- **Computational Geometry**: Point-in-polygon tests, ray casting, polygon boundaries
- **Conditional Logic**: Property-based filtering, range-based matching, rule-based comparisons
- **DFS/Path Counting**: Depth-first search with path enumeration, memoization for exponential problems
- **Cryptographic**: Hash functions, encryption, cryptographic puzzles
- **Data Structures**: Working with arrays, lists, sets, maps
- **Encoding**: String encoding, character escaping
- **Graph Algorithms**: Graph traversal, shortest path, connectivity analysis
- **Greedy Algorithms**: Optimal greedy strategies, reverse optimization, exploiting problem structure
- **Grid Processing**: 2D grid parsing, coordinate-based access, neighbor queries, spatial reasoning
- **Integer Linear Programming**: Constraint optimization with specialized solvers, minimizing objective functions subject to linear constraints
- **Interval Merging**: Combining overlapping ranges, computing union of intervals, deduplication by range
- **Iterative Erosion**: Repeated state modification until convergence, layer-by-layer removal, fixpoint algorithms
- **Mathematical**: Arithmetic calculations, formulas, geometric problems
- **Number Theory**: Divisor sums, highly composite numbers, sieve algorithms, multiplicative functions
- **Optimization**: Finding minimum/maximum values
- **Parsing**: Escape sequence parsing, character-level analysis
- **Pattern Matching**: Regular expressions, string validation, substring detection
- **Range Containment**: Checking membership in inclusive ranges, interval queries, boundary testing
- **Real-time Analysis**: Temporal scoring, moment-by-moment leader tracking, time-dependent calculations
- **Search**: Informed search algorithms, A* search, heuristics, state space exploration
- **Search/Traversal**: Finding positions, tracking states
- **Simulation**: State tracking, following instructions step-by-step
- **Sparse Representation**: HashSet/HashMap for large coordinate spaces, avoiding grid materialization
- **String Processing**: Character manipulation, parsing, pattern matching

---

## Day-by-Day Summary

### Day 1: Secret Entrance
**Title**: Secret Entrance  
**Part 1 Type**: Simulation + Mathematical  
**Part 1 Description**: Simulate safe dial rotations (L/R with distance) on circular 0-99 positions, count how many times dial ends at position 0  
**Part 2 Type**: Simulation + Mathematical  
**Part 2 Description**: Count every time dial points at 0 (both final positions AND during rotations), using password method 0x434C49434B  
**Key Concepts**: Circular arithmetic, modular mathematics, boundary crossing detection, signed integer arithmetic, edge case handling for position 0  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Simulation escalation (endpoint counting → comprehensive zero detection including intermediate states)
- **Data Structure**: Simple state tracking with signed integers, boundary crossing arithmetic
- **Complexity**: Part 1: O(n) single pass through rotations, Part 2: O(n) with boundary crossing calculations
- **AoC Theme**: "Safe dial manipulation" with classic Part 2 escalation (simple counting → complex state tracking)

**🦀 Rust Implementation Highlights**:
- **Circular arithmetic safety** → **Modular operations with overflow protection** using signed `i32` arithmetic
- **Edge case handling** → **Special position 0 logic** preventing overcounting when starting from zero
- **Boundary detection** → **Division-based crossing counts** `(dial / 100).abs()` for natural zero detection
- **Error resilience** → **Comprehensive parsing with `anyhow::Result`** and detailed error context

**Debugging Journey**: 
- **Initial overcounting (6037)** → Fixed edge cases when starting from position 0
- **Overcorrection (5015)** → Balanced approach using signed arithmetic and boundary crossing
- **Final solution (5941)** → Elegant division-based detection with special Left-from-0 handling

**🔧 Key Implementation Insights**:
- **Special case**: `if dial == 0 && rotation.direction == Direction::Left { dial = 100; }` prevents edge case issues
- **Boundary crossing**: `zero_count += (dial / 100).abs();` naturally detects zero passages
- **Wrap handling**: `if dial < 1 { zero_count += 1; }` catches wrap-around cases
- **Normalization**: Final `dial % 100` with negative correction ensures 0-99 range

**Performance**: Pure Rust implementation, no Python comparison available for AoC 2025

---

### Day 2: Gift Shop
**Title**: Gift Shop  
**Part 1 Type**: String Processing + Pattern Matching  
**Part 1 Description**: Find invalid product IDs where the number's string representation is a sequence repeated exactly twice (e.g., 55, 1010, 123123)  
**Part 2 Type**: String Processing + Pattern Matching  
**Part 2 Description**: Find invalid product IDs where the number's string representation is a sequence repeated at least twice (e.g., 111, 999, 123123123)  
**Key Concepts**: String manipulation, pattern detection, range iteration, divisibility checks  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Pattern matching escalation (exact doubling → general repetition detection)
- **Data Structure**: String-based pattern matching, range iteration
- **Complexity**: Part 1: O(n) per number (string halving), Part 2: O(n²) per number (trying all pattern lengths)
- **AoC Theme**: "Invalid product IDs" with classic Part 2 escalation (simple pattern → complex pattern)

**🦀 Rust Implementation Highlights**:
- **String slicing** → **`split_at()` for efficient halving** in Part 1
- **Pattern repetition** → **Try all divisors of length** using `is_multiple_of()` for clean divisibility
- **Functional iteration** → **`filter()` and `sum()`** for range processing
- **Error handling** → **`anyhow::Result`** for consistent error propagation

**Key Implementation Insights**:
- **Part 1 Algorithm**: Check if string length is even, split at midpoint, compare halves
- **Part 2 Algorithm**: For pattern lengths 1 to len/2, check if length is divisible and pattern repeats
- **Optimization**: Early exit when pattern doesn't match, avoiding unnecessary string operations
- **String reconstruction**: `pattern.repeat(repetitions)` for verification

**Answers**:
- Part 1: `23534117921`
- Part 2: `31755323497`

**⏱️ Performance Note**: The actual puzzle input uses small, bounded ranges that complete instantly with the brute force approach. The O(R×d²) complexity is perfectly acceptable when R (range size) is small.

**🚀 Reddit Challenge: Full 32-bit Range Benchmark**

The Reddit community posed an extreme scaling challenge: what if the input was a single range from 1 to 4,294,967,296 (2³²)? This exposed the limitations of the brute force approach and required a fundamentally different algorithm.

**Brute Force Approach** (`day02_full_range.rs`) - works fine for puzzle input, struggles at scale:
- Iterates through all 4.3 billion numbers, checking each one
- Part 1: 114.42 seconds (~1.9 minutes)
- Part 2: 479.98 seconds (~8 minutes)
- Total: **594.40 seconds** (~10 minutes)

**Optimized Approach** (`day02_full_range_optimized.rs`):
- Key insight: **Generate valid patterns directly** instead of checking every number
- For D-digit numbers, only pattern lengths P where D % P == 0 and D/P ≥ 2 are valid
- Example: 7-digit numbers (prime) only have pattern length 1 → just 9 valid numbers (1111111-9999999)

| Version | Part 1 | Part 2 | Total |
|---------|--------|--------|-------|
| Brute force | 114.42s | 479.98s | **594.40s** |
| Optimized | 287.70µs | 1.15ms | **1.43ms** |
| **Speedup** | 397,000x | 417,000x | **~415,000x** |

**Pattern Analysis by Digit Count**:
```
1 digits: NO valid patterns (prime)
2 digits: pattern lengths [1]
3 digits: pattern lengths [1]
4 digits: pattern lengths [1, 2]
5 digits: pattern lengths [1]
6 digits: pattern lengths [1, 2, 3]
7 digits: pattern lengths [1]
8 digits: pattern lengths [1, 2, 4]
9 digits: pattern lengths [1, 3]
10 digits: pattern lengths [1, 2, 5]
```

**Full Range Results**:
- Part 1 (doubled): 87,729,849,870,725 (42,949 numbers)
- Part 2 (repeated): 88,304,989,965,662 (43,987 numbers)

**Lesson**: Classic example of when brute force is "good enough" for the actual problem, but extreme inputs reveal the need for algorithmic thinking. The optimized approach changes from "check every number" O(N × d²) to "generate only valid patterns" O(valid_patterns), reducing 4.3 billion checks to ~44,000 generations.

---

### Day 3: Lobby
**Title**: Lobby  
**Part 1 Type**: Greedy Algorithms + String Processing  
**Part 1 Description**: From each battery bank (row of digits), select exactly 2 batteries to form the largest possible two-digit number (preserving digit order)  
**Part 2 Type**: Greedy Algorithms + String Processing  
**Part 2 Description**: Same as Part 1, but select exactly 12 batteries to form the largest possible 12-digit number  
**Key Concepts**: Greedy selection, tie-breaking strategy, digit ordering constraints  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Greedy selection with constraints (k-out-of-n digit maximization)
- **Data Structure**: Simple array iteration, no complex structures needed
- **Complexity**: Part 1: O(n²) per line, Part 2: O(k×n) per line where k=12
- **AoC Theme**: "Battery joltage" with classic Part 2 scaling (2 digits → 12 digits)

**🦀 Rust Implementation Highlights**:
- **Greedy iteration** → **Two-phase max finding**: first find max digit, then find first occurrence
- **Tie-breaking** → **Use `.find()` for earliest max** to leave maximum flexibility for remaining positions
- **Large numbers** → **`u64` for 12-digit results** (up to 999,999,999,999)
- **Clean iteration** → **Range-based selection** with `(start_idx..=end_idx)` bounds

**Key Implementation Insights**:
- **Part 1 Algorithm**: For each first position i, find max(digits[i] × 10 + max(digits[i+1..]))
- **Part 2 Algorithm**: Greedy k-digit selection - at each position, pick largest digit leaving k-1 positions available
- **Critical Fix**: When multiple positions have same max digit, pick EARLIEST (not last via `max_by_key`)
- **Bounds calculation**: For position p of k total, search range is `[start..=(n-k+p)]`

**Answers**:
- Part 1: `17427`
- Part 2: `173161749617495`

**⏱️ Performance**: O(k×n) per line is efficient for k=12 and n~100.

---

### Day 4: Printing Department
**Title**: Printing Department  
**Part 1 Type**: Cellular Automaton + Grid Processing  
**Part 1 Description**: Count paper rolls (@) accessible by forklifts - a roll is accessible if it has fewer than 4 adjacent rolls in its 8 neighboring positions  
**Part 2 Type**: Cellular Automaton + Iterative Erosion  
**Part 2 Description**: Iteratively remove accessible rolls until none remain accessible, counting total rolls removed  
**Key Concepts**: 8-connected neighbor counting, grid erosion, iterative state modification, boundary handling  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Cellular automaton with erosion (single-pass counting → iterative removal)
- **Data Structure**: 2D Grid with 8-connectivity neighbor queries
- **Complexity**: Part 1: O(W×H), Part 2: O(W×H×iterations) where iterations depends on grid density
- **AoC Theme**: "Forklift accessibility" with Part 2 escalation (static count → dynamic erosion)

**🦀 Rust Implementation Highlights**:
- **Mission 6 Integration** → **`Grid<char>`, `Coord`, `AocGridParser`** for grid infrastructure
- **Bounded neighbors** → **`neighbors_8_bounded(width, height)`** handles edge/corner cases automatically
- **Functional filtering** → **`.filter(|&coord| is_accessible(&grid, coord))`** for clean accessibility checks
- **In-place mutation** → **`grid[coord] = '.'`** for Part 2 roll removal

**Mission Integration**:
- **Mission 6**: `AocGridParser::parse_char_grid()` for parsing, `Coord::neighbors_8_bounded()` for neighbor iteration, `Grid` indexing for access/mutation

**Key Implementation Insights**:
- **Accessibility rule**: Roll has <4 adjacent rolls → accessible (includes corners with ≤3 neighbors)
- **Part 2 algorithm**: Collect all accessible → remove all → repeat until empty set
- **No flood fill needed**: Each cell evaluated independently based on immediate neighbors only
- **Erosion pattern**: Like peeling an onion - outer accessible layers removed first, exposing inner layers

**Answers**:
- Part 1: `1604`
- Part 2: `9397`

**⏱️ Performance**: Part 1 is single-pass O(W×H). Part 2 completes quickly due to efficient batch removal per iteration.

---

### Day 5: Cafeteria
**Title**: Cafeteria  
**Part 1 Type**: Range Containment + Conditional Logic  
**Part 1 Description**: Count how many ingredient IDs are "fresh" by checking if each falls within any of the given inclusive ranges  
**Part 2 Type**: Interval Merging + Mathematical  
**Part 2 Description**: Count total unique IDs considered fresh across all overlapping ranges  
**Key Concepts**: Inclusive ranges, range containment checking, interval merging algorithm, overlapping range deduplication  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Range query escalation (membership testing → total unique counting)
- **Data Structure**: Sorted intervals for merging
- **Complexity**: Part 1: O(n×m) where n=ingredients, m=ranges, Part 2: O(m log m) for sort + O(m) for merge
- **AoC Theme**: "Fresh ingredient detection" with classic Part 2 scope expansion (check given items → enumerate all valid items)

**🦀 Rust Implementation Highlights**:
- **Two-section parsing** → **`split("\n\n")`** to separate ranges from ingredients
- **Struct for clarity** → **`FreshRange { start, end }`** with `contains()` method
- **Functional filtering** → **`.filter(|&&id| is_fresh(id, &ranges))`** for Part 1 counting
- **Interval merging** → **Sort by start, extend overlapping ranges** for Part 2

**🚨 Initial Approach & Why It Failed**:
- **Initial idea**: Use `HashSet` to collect all IDs from ranges, count unique values
- **Code**: `ranges.iter().flat_map(|r| r.start..=r.end).collect::<HashSet<u64>>()`
- **Problem**: Input ranges were ENORMOUS (e.g., `169486974574545-170251643963353` = ~765 billion values!)
- **Result**: Would require trillions of HashSet entries - memory impossible, runtime infinite
- **Lesson**: Always check input scale before choosing data structures!

**✅ Correct Approach - Interval Merging**:
- **Algorithm**: Sort ranges by start, merge overlapping/adjacent ranges, sum lengths
- **Key insight**: Don't enumerate IDs - compute count mathematically
- **Merge logic**: If `current.start <= last.end + 1`, extend last range; otherwise add new range
- **Final count**: Sum of `(end - start + 1)` for each merged interval

**🔑 Why Sorting is Critical**:
- **Without sorting**: Must compare each new range against ALL existing merged ranges → O(n²)
- **With sorting**: Only compare against the LAST merged range → O(n) single pass
- **Guarantee**: After sorting by start, if current range doesn't overlap with last merged range, it can't overlap with ANY earlier range
- **Bonus**: Sorting also handles cascading merges automatically (e.g., ranges A, B, C where A overlaps B and B overlaps C all merge naturally in one pass)
- **Trade-off**: O(n log n) sort cost is far better than O(n²) comparison cost

**Key Implementation Insights**:
- **Part 1**: Simple O(n×m) nested iteration - fine for typical AoC input sizes
- **Part 2**: O(m log m) sorting + O(m) single-pass merge - instant regardless of range sizes
- **Adjacent handling**: `start <= last.end + 1` catches both overlapping AND touching ranges (e.g., 3-5 and 6-8 merge to 3-8)
- **Inclusive math**: Range length = `end - start + 1` (not `end - start`)

**Answers**:
- Part 1: `679`
- Part 2: `358155203664116`

**⏱️ Performance**: Part 1 is O(n×m) which is fine for small inputs. Part 2 interval merging runs instantly even with trillion-scale ranges.

---

### Day 6: Trash Compactor
**Title**: Trash Compactor  
**Part 1 Type**: Parsing + Mathematical  
**Part 1 Description**: Parse vertical math worksheet with numbers arranged in columns, operators at bottom. Compute each problem (multiply or add) and sum all results.  
**Part 2 Type**: Parsing + Column-wise Reading  
**Part 2 Description**: Same worksheet, but read numbers column-by-column right-to-left (digits top-to-bottom form each number), then compute and sum results.  
**Key Concepts**: Whitespace-based token alignment, column-based text parsing, directional reading  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Parsing twist (row-wise reading → column-wise right-to-left reading)
- **Data Structure**: `split_whitespace()` token alignment for Part 1, column ranges for Part 2
- **Complexity**: O(W×H) where W=worksheet width, H=number of rows
- **AoC Theme**: "Cephalopod math" with Part 2 twist (same data, different reading direction)

**🦀 Rust Implementation Highlights**:
- **Part 1 simplification** → **`split_whitespace()` on each row**, tokens at same index belong to same problem
- **Iterator elegance** → **`enumerate()` over operator tokens**, collect numbers at matching indices via `filter_map`
- **Part 2 necessity** → **Column-based parsing still needed** for vertical digit extraction
- **Operator-based ranges** → **Find operators directly with `char_indices()`** - no separator scanning needed
- **Struct encapsulation** → **`Problem { numbers, operator }`** with `compute()` method

**Key Implementation Insights**:
- **Part 1 key insight**: `split_whitespace()` naturally aligns tokens by position - no column math needed!
- **Part 2 range finding**: Operators mark problem boundaries - `char_indices().filter()` finds them directly
- **Part 2 algorithm**: For each column range, iterate columns right-to-left `(start..end).rev()`, collect digits top-to-bottom
- **Refactoring journey**: Initial over-engineered column-index approach (640 lines) → simplified (212 lines, 67% reduction)
- **No missions used**: Pure string parsing, no grid/graph infrastructure needed

**Answers**:
- Part 1: `5873191732773`
- Part 2: `11386445308378`

**⏱️ Performance**: Single-pass parsing, O(W×H) for both parts. No heavy data structures needed.

---

### Day 7: Laboratories
**Title**: Laboratories (Tachyon Manifold)  
**Part 1 Type**: Simulation + Grid Processing  
**Part 1 Description**: Simulate tachyon beams moving downward through a grid, count how many times beams hit splitters (^) that create left/right beam emissions  
**Part 2 Type**: DFS/Path Counting + Graph Algorithms  
**Part 2 Description**: Count total unique quantum timelines - each path from start to exit where particle takes both left/right at splitters (many-worlds interpretation)  
**Key Concepts**: BFS simulation, DFS path enumeration, memoization for exponential complexity, quantum timeline branching  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Simulation escalation (event counting → path enumeration)
- **Data Structure**: Grid<Cell> for manifold, VecDeque for BFS, HashMap for memoization
- **Complexity**: Part 1: O(W×H) BFS simulation, Part 2: O(2^n) paths reduced to O(W×H) with memoization
- **AoC Theme**: "Tachyon beam splitting" with Part 2 quantum mechanics twist (classical → quantum many-worlds)

**🦀 Rust Implementation Highlights**:
- **Mission 6 Integration** → **`Grid<Cell>`, `Coord`, `Direction`, `try_move_in_direction()`** for grid infrastructure
- **Mission 8 Integration** → **Implemented `Graph` trait for `ManifoldGraph`** demonstrating component composition
- **BFS simulation** → **`VecDeque` queue with `HashSet` visited tracking** for Part 1 split counting
- **Memoized DFS** → **`HashMap<Coord, usize>` cache** prevents exponential blowup in Part 2
- **Pattern matching** → **`match cell` for beam physics, `if let` for bounds checking, `while let` for queue processing**

**Mission Integration - The Integrator Approach**:
- **Mission 6**: Grid storage, coordinate system, directional movement primitives
- **Mission 8**: Implemented `Graph` trait with `neighbors()`, `contains()`, `nodes()` methods
- **Custom optimization**: Generic Mission 8 `dfs()` counts nodes, not paths - added memoized path counter
- **Key insight**: Use mission components as building blocks, add problem-specific logic where needed

**🔑 Key Implementation Insights**:
- **Part 1 physics**: ALL beams move South; splitters stop beam and spawn left/right beams (also moving South)
- **Part 2 algorithm**: Recursive DFS counting all paths from start to any exit (bottom edge or no next cell)
- **Memoization critical**: Without cache, 390 trillion paths cause timeout; with cache, instant
- **Cycle detection**: `visited` HashSet prevents infinite loops when beams overlap
- **Graph trait demo**: Shows how Mission 6 Grid + Mission 8 Graph compose, even when generic algorithms don't fit exactly

**Debugging Journey**:
- **Initial misunderstanding**: Thought beams continued in original direction after split
- **Corrected physics**: Splitters create beams to left/right that ALSO move downward
- **Part 2 timeout**: Naive DFS enumerated 2^n paths - added HashMap memoization
- **Mission integration**: Demonstrated integrator pattern by implementing Graph trait for educational value

**Answers**:
- Part 1: `1687`
- Part 2: `390684413472684`

**⏱️ Performance**: Part 1 BFS completes instantly. Part 2 memoized DFS reduces exponential O(2^n) to linear O(W×H) passes.

**🎓 Learning Highlights (Ch19.1 Pattern Matching Applied)**:
- **match expression**: Cell type branching (`match cell { Splitter => ..., Empty => ... }`)
- **if let**: Option handling for cache lookups and grid bounds (`if let Some(cached) = memo.get(&pos)`)
- **while let**: Queue processing (`while let Some((pos, dir)) = queue.pop_front()`)
- **let destructuring**: Tuple patterns (`let (grid, start_pos) = parse_manifold(input)?`)

---

### Day 8: Playground
**Title**: Playground (Junction Box Connectivity)  
**Part 1 Type**: Graph Algorithms + Greedy Algorithms  
**Part 1 Description**: Connect 1000 closest pairs of junction boxes (3D coordinates by Euclidean distance), find product of three largest circuit sizes  
**Part 2 Type**: Graph Algorithms + Optimization  
**Part 2 Description**: Continue connecting pairs until all boxes form one circuit, multiply X coordinates of the last pair connected  
**Key Concepts**: Union-Find (Disjoint Set Union), 3D Euclidean distance, greedy edge selection, minimum spanning tree variant, component tracking  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Classic Union-Find connectivity problem (partial connectivity → full connectivity)
- **Data Structure**: Mission 10 UnionFind with path compression and union by rank
- **Complexity**: Part 1: O(n² log n) for pair generation/sorting + O(n α(n)) for union-find operations, Part 2: O(n² α(n)) worst case
- **AoC Theme**: "Electrical junction boxes" with Part 2 escalation (limited connections → complete spanning tree)

**🦀 Rust Implementation Highlights**:
- **Mission 10 Integration** → **`UnionFind::new(n)`, `find()`, `union()`, `components()`** for connectivity tracking
- **3D geometry** → **`sqrt(dx² + dy² + dz²)`** for Euclidean distance calculation
- **Pair generation** → **Nested loops `(i..n, i+1..n)`** creating all unique pairs with distances
- **Greedy sorting** → **`sort_by()` with `partial_cmp()`** ordering pairs by distance (shortest first)
- **Component iteration** → **`components().map(|c| c.len())`** extracting circuit sizes from member lists

**Mission Integration - Validating Mission 10**:
- **Mission 10**: UnionFind data structure with path compression optimization
- **Perfect use case**: Junction box connectivity is textbook Union-Find problem
- **API discovery**: `components()` returns `ComponentIter` yielding `Vec<usize>` (member lists, not sizes)
- **Performance validation**: Handled 1000 boxes (499,500 pairs) with instant execution
- **Integrator win**: Reused battle-tested V-Cycle implementation instead of reimplementing DSU

**🔑 Key Implementation Insights**:
- **Part 1 critical detail**: "Examine 1000 closest pairs" ≠ "Make 1000 connections" (some pairs already connected)
- **Part 1 algorithm**: Sort all pairs by distance, iterate first 1000, union if not already connected
- **Part 1 result**: Examined 1000 pairs, made 698 connections, created multiple circuits
- **Part 2 algorithm**: Continue union-find until `num_components == 1`, track last connection made
- **Part 2 optimization**: Decrement component counter on successful union to avoid recomputing
- **Component tracking**: Start with n components (each box solo), decrease by 1 on each union

**Debugging Journey**:
- **Initial error**: All 1000 boxes in ONE circuit (answer: 1 component)
- **Root cause**: Stopped after making 1000 connections, not examining 1000 pairs
- **The fix**: Change loop to `for (i, (box_a, box_b, _)) in pairs.iter().enumerate()` with `if i >= 1000 { break }`
- **Result**: Examined 1000 pairs, made only 698 connections (302 skipped as already connected)
- **Mission 10 API learning**: `components()` returns iterator, not HashMap - needed `.map(|c| c.len())`

**Answers**:
- Part 1: `50568`
- Part 2: `36045012`

**⏱️ Performance**: Part 1 completes instantly despite 499,500 pair evaluations. Part 2 continues until full connectivity with efficient Union-Find operations.

**🔬 Reddit Optimization Study - Distance Calculation**:

**Question**: Is there benefit to skipping `sqrt()` and using integer squared distances?

**Answer**: **YES! ~12% performance improvement**

**Benchmark Results** (Criterion.rs on 1000 boxes, 499,500 pairs):
- Part 1: 24.31ms → 21.64ms (**11% faster**, saved 2.67ms)
- Part 2: 24.32ms → 21.58ms (**11% faster**, saved 2.74ms)
- ✅ **Identical correctness**: Both versions produce same answers

**Why it works**:
```rust
// Original (f64 with sqrt)
fn calculate_distance(a, b) -> f64 {
    let dx = (a.x - b.x) as f64;
    (dx*dx + dy*dy + dz*dz).sqrt()  // 👈 expensive!
}

// Optimized (i64 squared)
fn calculate_distance_squared(a, b) -> i64 {
    let dx = (a.x - b.x) as i64;
    dx*dx + dy*dy + dz*dz  // 👈 no sqrt, just multiply/add
}
```

**Key insight**: Monotonicity preserved - if `a² < b²` then `a < b`
- Sorting by squared distance gives **identical ordering** to sorting by actual distance
- We only need **relative ordering**, not absolute values
- Eliminates 499,500 expensive `sqrt()` calls (10-20x slower than multiply)
- Integer arithmetic often has better CPU pipeline/SIMD utilization

**Bonus**: Can use cleaner `sort_by_key(|&(_, _, d)| d)` instead of `sort_by()` with `partial_cmp()`

**When to apply**: Use squared distances when you only need relative ordering (sorting, min/max). Don't use if you need actual distance values (sums, thresholds).

**Files**: See `benches/day08_benchmark.rs` and [[../benches/day08_benchmark_results]] for full analysis

**🎓 Learning Highlights (Mission 10 Validation)**:
- **Union-Find in practice**: Real AoC problem demonstrating DSU's power for connectivity queries
- **Component iteration**: Understanding iterator vs HashMap APIs (`ComponentIter` yields members, not sizes)
- **Greedy correctness**: Sorting pairs by distance ensures optimal circuit formation
- **The integrator approach**: Mission 10 composition saved hours of DSU implementation/testing
- **Performance optimization**: Squared distance pattern is common in competitive programming - question absolute value necessity

---

### Day 9: Movie Theater
**Title**: Movie Theater (Rectangle Area Maximization)  
**Part 1 Type**: Brute Force + Mathematical  
**Part 1 Description**: Find largest rectangle using any two red tiles (496 points) as opposite corners, inclusive tile counting  
**Part 2 Type**: Computational Geometry + AABB Sampling + Sparse Representation  
**Part 2 Description**: Connect points into circular polygon boundary, flood fill interior, find largest rectangle containing only red/green tiles  
**Key Concepts**: Inclusive area calculation, point-in-polygon ray casting, AABB sampling optimization, sparse HashSet boundaries, Bresenham's line algorithm  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Geometric constraint escalation (unconstrained rectangles → polygon-bounded rectangles)
- **Data Structure**: HashSet for sparse boundary (~589K tiles), no grid materialization
- **Complexity**: Part 1: O(n²) = 122,760 pairs, Part 2: O(n² × samples × polygon_edges) with adaptive sampling
- **AoC Theme**: "Movie theater seating" with Part 2 dramatic constraint (any rectangle → interior-only rectangle)

**🦀 Rust Implementation Highlights**:
- **Part 1 simplicity** → **Pure brute force checking all pairs** with inclusive tile counting
- **Sparse boundary** → **HashSet<(i64, i64)>** for 588,656 boundary tiles (not 9.36B grid!)
- **Ray casting** → **Point-in-polygon algorithm** crossing edges to determine interior membership
- **AABB sampling** → **Strategic point checks** (corners, edges, interior grid) instead of all tiles
- **Mission 6 awareness** → **Grid used only for small examples**, geometric algorithms for real input

**🚨 Failed Approaches - The Journey**:

**Attempt 1: Dense Grid** (❌ 37GB memory crash)
- Tried `Grid<char>` for full coordinate space (96,800 × 96,600)
- **Problem**: 9.36 billion cells = 37GB allocation failure
- **Lesson**: Never materialize dense grids for sparse, large coordinate spaces

**Attempt 2: Sparse HashSet with Interior Fill** (❌ 18GB memory crash)
- Tried storing all boundary + interior points in HashSet
- **Problem**: Circular interior has ~7.4 billion tiles = 18GB crash
- **Lesson**: Circular shapes maximize area - even sparse representation can be too large

**Attempt 3: Boundary-Only with Naive Ray Casting** (❌ 10s timeout)
- Stored boundary only, checked every tile in rectangles
- **Problem**: Largest rectangles have billions of tiles × ray casting = 2.3 trillion operations
- **Lesson**: Cannot iterate every tile in large rectangles

**Attempt 4: AABB Sampling + Ray Casting** (✅ SUCCESS!)
- **Key insight from Reddit**: "Use AABB collision or compacted space"
- Only check strategic sample points (corners, edges at intervals, interior grid)
- Sample rate adapts to rectangle size: `max(10, dimension/100)`
- **Result**: 122,760 pairs × ~10K samples × 496 ray casts = 633M operations (feasible!)

**Key Implementation Insights**:
- **Inclusive counting**: Rectangle from (2,5) to (9,7) has area 8×3=24, not 7×2=14
- **Circular polygon**: Input forms near-perfect circle with radius ~48,386 tiles
- **Boundary tiles**: Bresenham's line creates ~589K edge tiles (vs 304K theoretical circumference)
- **Part 1 strategy**: Diagonal square spanning circle diameter (68,593 × 69,388)
- **Part 2 strategy**: Wide horizontal band through center (89,340 × 17,229)
- **Area reduction**: Part 2 only 32% of Part 1 due to circular boundary constraint

**Answers**:
- Part 1: `4,759,531,084` - Points (83188, 85814) and (14596, 16427)
- Part 2: `1,539,238,860` - Points (5398, 67501) and (94737, 50273)

**⏱️ Performance**: Part 1 instant (milliseconds), Part 2 ~3-5 seconds with AABB sampling

**📊 Visualizations**:
Three Python scripts in `examples/` directory visualize the problem:
1. **`plot_day09_polygon.py`**: Shows circular polygon shape (reveals ~7.4B interior tiles)
2. **`plot_day09_largest_rectangle.py`**: Part 1 diagonal square solution
3. **`plot_day09_part2_largest_rectangle.py`**: Part 2 horizontal band solution

**🎓 Learning Highlights**:
- **Input scale matters**: Example (8 points) vs real (496 points, 100K coordinates) requires completely different approaches
- **Geometric over materialization**: Ray casting beats flood fill when interior is huge
- **AABB sampling**: Strategic point checking reduces billions of checks to thousands
- **Mission applicability**: Grid infrastructure perfect for small problems, not massive sparse spaces
- **The integrator lesson**: Use Mission 6 concepts (flood fill theory) even when Grid itself doesn't fit

**Deep Dive**: See [[../examples/day09_approaches]] for complete analysis of all 4 approaches, memory calculations, performance breakdowns, and geometric insights about circular constraints.

---

### Day 10: Factory
**Title**: Factory (Button Optimization)  
**Part 1 Type**: Mathematical + Graph Algorithms  
**Part 1 Description**: Binary toggle problem - find minimum button presses to reach target light configuration using Gaussian elimination over GF(2) (binary field)  
**Part 2 Type**: Integer Linear Programming + Optimization  
**Part 2 Description**: Integer counter problem - find minimum button presses to reach exact joltage counter values using ILP solver  
**Key Concepts**: Gaussian elimination over GF(2), XOR operations, free variable enumeration, Integer Linear Programming, constraint optimization, floating-point precision handling  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Two distinct optimization problems (binary toggles → integer accumulation)
- **Data Structure**: Part 1: Augmented matrix for RREF, Part 2: good_lp constraint solver
- **Complexity**: Part 1: O(n³) Gaussian elimination, Part 2: Polynomial LP relaxation + branch-and-bound
- **AoC Theme**: "Factory button optimization" with Part 2 fundamentally different problem (GF(2) → ILP)

**🦀 Rust Implementation Highlights**:
- **Part 1: Pure Rust** → **Gaussian elimination with XOR**, free variable enumeration, optimal 2^f combination search
- **Part 2: Library-assisted** → **`good_lp` crate with minilp solver** (pure Rust, no native dependencies)
- **Critical precision fix** → **`.round() as usize`** instead of truncation (16754 → 16757 correct answer)
- **Credit** → **Based on Tom Wilkinson's ILP approach** adapted for our infrastructure

**🚨 Failed Approaches - The Long Journey**:

See **[[../examples/day10_solution_analysis]]** for complete 6-attempt analysis with code samples and performance comparisons.

**Attempt 1: Backtracking DFS** (❌ Timeout)
- Tried all button combinations with upper bound pruning
- **Problem**: O(T^B) exponential, machine 3 has 194^9 ≈ 10^20 states
- **Result**: Exceeded 10-second timeout

**Attempt 2: Greedy Heuristic** (❌ Suboptimal)
- Selected buttons affecting most unfulfilled counters
- **Problem**: Local optimum ≠ global optimum
- **Result**: Got 19 presses instead of optimal 10

**Attempt 3: BFS (Breadth-First Search)** (❌ Timeout on large inputs)
- Level-by-level state space exploration
- **Pros**: Guaranteed optimal solution
- **Test cases**: ✅ All passed (10, 12, 11 → 33)
- **Actual input**: ❌ Machine 3 timeout - visited HashMap grew to millions of entries
- **State space**: O(∏ target_i) - for targets [168, 164, 176, ...] = billions of states

**Attempt 4: A* Search with Heuristic** (❌ Inadmissible heuristic)
- Used heuristic `h = Σ(remaining / affecting_buttons)`
- **Problem**: Heuristic not truly admissible (overestimates in some cases)
- **Result**: Got 12 instead of optimal 11 for test case 3

**Attempt 5: Z3 SMT Solver** (❌ Compilation failed)
- Tried `z3 = "0.12"` Rust crate for constraint solving
- **Problem**: Requires libclang.dll (native dependency), CMake, Visual Studio Build Tools
- **Errors**: "couldn't find libclang.dll", cmake version detection failure
- **Conclusion**: Native dependency hell on Windows

**Attempt 6: Integer Linear Programming with good_lp** (✅ SUCCESS!)
- **Mathematical formulation**:
  ```
  Minimize: Σ x_i (total button presses)
  Subject to:
    For each counter j: Σ (a_ij * x_i) = t_j
    x_i ≥ 0, x_i ∈ ℤ (non-negative integers)
  ```
- **Implementation**: `good_lp = { version = "1.8", features = ["minilp"] }`
- **Result**: All 157 machines solved instantly, correct answer 16757

**Key Implementation Insights**:
- **Part 1 algorithm**: Build augmented matrix [A|b], row-reduce to RREF using XOR, enumerate 2^f free variable combinations, return minimum
- **Part 2 algorithm**: Model as ILP with integer button variables, counter equality constraints, minimize objective function
- **Critical fix**: `minilp` returns near-integer solutions (16756.999...) - must use `.round()` not truncation
- **Why ILP works**: Polynomial-time LP relaxation + branch-and-bound beats exponential search
- **Solver choice**: `minilp` (pure Rust) over `highs` (faster but requires CMake/C++)

**Performance Comparison**:

| Approach | Test Cases | Machine 3 (Large) | Complexity |
|----------|------------|-------------------|------------|
| Backtracking | ❌ Timeout | ❌ Timeout | O(T^B) exponential |
| Greedy | ❌ Wrong (19 vs 10) | Fast | O(B×C) suboptimal |
| BFS | ✅ Optimal | ❌ Timeout | O(∏ T_i) exponential space |
| A* | ❌ Wrong (12 vs 11) | ❌ Timeout | O(∏ T_i) bad heuristic |
| Z3 | N/A | N/A | Compilation failed |
| **ILP (minilp)** | ✅ Optimal | ✅ <1s | **Polynomial + branch-and-bound** |

**Answers**:
- Part 1: `385`
- Part 2: `16757` (initially got 16754 due to truncation bug)

**⏱️ Performance**: Part 1 instant (Gaussian elimination), Part 2 instant for all 157 machines (ILP solver)

**🎓 Learning Highlights**:
- **Problem recognition**: Part 2 is NP-hard ILP - requires specialized solvers, not naive search
- **Native dependencies are painful**: Z3 would work but Windows compilation issues blocked it
- **Pure Rust alternatives exist**: good_lp with minilp feature compiles out-of-the-box
- **Floating-point precision matters**: Always `.round()` when converting solver results to integers
- **Test early on large inputs**: BFS passed tests but actual input revealed scalability issues
- **Community resources valuable**: Reddit/Tom Wilkinson's solution provided the ILP approach
- **State space analysis crucial**: Machine 3 (9 counters × 194 max) = massive search space needs proper algorithm

**Deep Dive**: See [[../examples/day10_solution_analysis]] for complete documentation of all 6 attempts, mathematical formulations, code samples, and why ILP succeeds where search fails.

**Additional Resources**:
- **[[../docs/day10_solve_machine_examples]]** - Detailed walkthrough of Gaussian elimination over GF(2) with 2 complete examples
- **[[../Part2_ILP_NOTES]]** - ILP problem formulation and why search approaches fail (state space analysis)
- **[[../docs/day10_z3_setup]]** - Alternative Z3 SMT solver implementation (exact integers vs floating-point)

---

### Day 11: Network
**Title**: Network (Graph Path Counting)  
**Part 1 Type**: DFS/Path Counting + Graph Algorithms  
**Part 1 Description**: Count all distinct paths from node "you" to node "out" in a directed acyclic graph  
**Part 2 Type**: DFS/Path Counting + Advanced Pattern Matching  
**Part 2 Description**: Count paths from "svr" to "out" that visit BOTH "dac" AND "fft" (in any order)  
**Key Concepts**: Depth-first search, memoization, state-based caching, bitmask for set membership, composite state representation, exponential complexity reduction  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Path counting escalation (simple counting → path filtering with constraints)
- **Data Structure**: HashMap<String, Vec<String>> for adjacency list, HashMap for memoization caches
- **Complexity**: Part 1: O(V+E) with memo, Part 2: O(V × 2^R) where R=required nodes (O(V × 4) for 2 required)
- **AoC Theme**: "Network packet routing" with Part 2 adding state complexity (position → position + history)

**🦀 Rust Implementation Highlights**:
- **Simple memoization** → **`HashMap<String, usize>`** for Part 1 (state = node only)
- **Composite state memoization** → **`HashMap<(String, usize), usize>`** for Part 2 (state = node + visited_mask)
- **Bitmask encoding** → **`visited_mask |= 1 << i`** for compact set representation (2 required → 4 states)
- **Mission 8 attempted** → **Discovered Graph trait requires Copy nodes** (String not Copy, couldn't use)
- **HashMap direct use** → **`.get()`, `.contains_key()`** work fine without trait abstraction

**🚨 Failed Approach - Part 2 v1**:
- **Naive DFS**: Track full path as `Vec<String>`, check requirements at target
- **Problem**: No memoization → explores every path completely
- **Result**: Timed out after 10 seconds (exit code 0xc000013a)
- **Diagnosis**: 549 trillion paths exist, can't enumerate them all

**✅ Successful Approach - Part 2 v2 (State-Based Memoization)**:
- **Key insight**: State = (current_node, set_of_visited_required_nodes)
- **Bitmask representation**: 
  ```
  0b00 (0): visited neither dac nor fft
  0b01 (1): visited dac only
  0b10 (2): visited fft only
  0b11 (3): visited both (valid path!)
  ```
- **Update logic**: When visiting required[i], set bit i: `new_mask |= 1 << i`
- **Memoization**: Cache `(node, mask)` → count, reuse across recursive calls
- **Result**: Completes instantly despite 549 trillion paths

**Mission Integration Attempt**:
- **Tried**: Mission 8's Graph trait for HashMap<String, Vec<String>>
- **Compiler error**: 
  ```rust
  error[E0277]: the trait bound `std::string::String: Copy` is not satisfied
  required for `HashMap<String, Vec<String>>` to implement `Graph`
  ```
- **Mission 8 design**: Trait requires `type Node: Copy` for performance (optimized for u32, &str)
- **Resolution**: Used HashMap methods directly without trait abstraction
- **Learning**: Mission libraries have design constraints, adapt when needed

**Key Implementation Insights**:
- **Part 1 algorithm**: Recursive DFS with memo[node] = sum of paths from neighbors
- **Part 2 algorithm**: Recursive DFS with memo[(node, visited_mask)] = count
- **Base case**: At target with mask==0b11 → return 1, else return 0
- **Exponential → linear**: Memoization reduces O(2^n) path enumeration to O(V × 2^R) state computation
- **For 2 required**: Only 4 states per node (00, 01, 10, 11) → effectively O(V)

**Debugging Journey**:
- **Part 1**: Clean implementation, worked first try (466 paths)
- **Part 2 v1**: Naive full-path tracking → timeout after 10 seconds
- **Recognition**: User said "Ok" → immediately understood need for optimization
- **Part 2 v2**: Redesigned with state-based memo → instant completion
- **The fix**: Changed from tracking full path to tracking only visited required nodes

**Answers**:
- Part 1: `466`
- Part 2: `549705036748518` (549 trillion paths!)

**⏱️ Performance**: 
- Part 1: Instant (simple memoization)
- Part 2 v1: Timeout after 10 seconds
- Part 2 v2: Instant (state memoization handles exponential paths)

**🎓 Learning Highlights**:
- **State representation matters**: (node) vs (node, history) fundamentally different complexity
- **Bitmask for small sets**: When tracking 2-5 items, bit flags are compact and fast
- **Memoization prevents enumeration**: Can count 549 trillion paths without listing them
- **Mission constraints**: Copy requirement in Mission 8 Graph trait excludes owned String types
- **Pragmatic adaptation**: Use mission concepts (DFS, memoization) even when mission code doesn't fit exactly
- **Performance debugging**: Timeout → recognize exponential growth → redesign state representation

---

## Problem Type Distribution (Available Days)

| Category | Part 1 Count | Part 2 Count |
|----------|--------------|--------------|
| AABB Sampling | 0 | 1 |
| Advanced Pattern Matching | 0 | 1 |
| Brute Force | 1 | 0 |
| Cellular Automaton | 1 | 1 |
| Combinatorial Optimization | 0 | 0 |
| Computational Geometry | 0 | 1 |
| Conditional Logic | 0 | 0 |
| DFS/Path Counting | 1 | 2 |
| Range Containment | 1 | 0 |
| Interval Merging | 0 | 1 |
| Cryptographic | 0 | 0 |
| Data Structures | 0 | 0 |
| Encoding | 0 | 0 |
| Graph Algorithms | 3 | 2 |
| Greedy Algorithms | 2 | 1 |
| Grid Processing | 2 | 0 |
| Integer Linear Programming | 0 | 1 |
| Iterative Erosion | 0 | 1 |
| Mathematical | 4 | 3 |
| Number Theory | 0 | 0 |
| Optimization | 0 | 2 |
| Parsing | 1 | 1 |
| Pattern Matching | 1 | 1 |
| Real-time Analysis | 0 | 0 |
| Search | 0 | 0 |
| Search/Traversal | 0 | 0 |
| Simulation | 2 | 1 |
| Sparse Representation | 0 | 1 |
| String Processing | 2 | 2 |

## Implementation Notes

### Common Patterns Observed

### Early 2025 Themes
- **Mathematical simulations**: Day 1 demonstrates circular arithmetic challenges with edge case complexity
- **Boundary detection**: Division-based algorithms for detecting state transitions (dial crossing zero)
- **Signed arithmetic benefits**: Using `i32` over `u32` simplifies wrap-around and negative value handling
- **String pattern matching**: Day 2 showcases string manipulation and repetition detection algorithms
- **Greedy selection**: Day 3 introduces optimal k-digit selection with tie-breaking strategies
- **Grid-based problems**: Day 4 introduces 2D grid processing with Mission 6 infrastructure reuse
- **Cellular automaton patterns**: Day 4 demonstrates neighbor-counting and iterative erosion algorithms
- **Range/interval problems**: Day 5 introduces interval merging for overlapping range problems
- **Input scale awareness**: Day 5 demonstrates why checking input magnitude is critical before choosing algorithms
- **Parsing simplification**: Day 6 shows how `split_whitespace()` can eliminate complex column-index math
- **Mission composition**: Day 7 demonstrates integrator approach - composing Mission 6 (Grid) + Mission 8 (Graph trait)
- **Memoization for exponential problems**: Day 7 Part 2 shows HashMap caching reducing O(2^n) to O(n)
- **Union-Find patterns**: Day 8 showcases Mission 10 for connectivity tracking with path compression
- **3D geometry**: Day 8 introduces spatial distance calculations in 3D coordinate space
- **Computational geometry**: Day 9 introduces point-in-polygon ray casting and geometric constraint optimization
- **Sparse representation strategies**: Day 9 demonstrates HashSet boundaries for massive coordinate spaces
- **AABB sampling optimization**: Day 9 shows strategic point checking vs exhaustive tile iteration
- **Algorithm evolution through failure**: Day 9 documents complete journey from memory crashes to timeout to success
- **Integer Linear Programming**: Day 10 introduces constraint optimization with specialized solvers
- **Problem type recognition**: Day 10 shows importance of identifying NP-hard problems early (search vs solver)
- **Floating-point precision**: Day 10 demonstrates rounding vs truncation when converting solver results
- **Native dependency avoidance**: Day 10 chooses pure Rust solver over faster but compilation-problematic alternatives
- **State-based memoization**: Day 11 shows composite state keys for tracking position + history in path counting
- **Bitmask techniques**: Day 11 demonstrates compact set representation using bit flags (2 required → 4 states)
- **Graph path counting**: Day 11 introduces DFS with memoization for counting paths in DAGs
- **Performance through state representation**: Day 11 shows how proper state design prevents exponential blowup (549T paths)
- **Mission trait constraints**: Day 11 reveals when library abstractions don't fit (Copy requirement) and how to adapt

### Rust-Specific Considerations

- **Day 1**: Demonstrates signed integer arithmetic advantages for circular problems, comprehensive error handling with `anyhow::Result`, and the importance of special case handling for boundary conditions (position 0)
- **Day 2**: Showcases string slicing with `split_at()`, pattern repetition detection using divisibility checks, and functional iteration with `filter()`/`sum()` for range processing
- **Day 3**: Highlights greedy iteration patterns, `.find()` for first-match semantics in tie-breaking, and `u64` for large number results
- **Day 4**: Demonstrates Mission 6 Grid integration, `neighbors_8_bounded()` for automatic boundary handling, and iterative state modification with in-place mutation
- **Day 5**: Showcases importance of checking input scale before choosing algorithms, interval merging for huge ranges, and the difference between enumeration vs mathematical counting
- **Day 6**: Demonstrates `split_whitespace()` for natural token alignment, `char_indices().filter()` for pattern finding, `filter_map()` with `and_then()` for chained Option processing, and iterative refactoring from 640→212 lines
- **Day 7**: Showcases Mission 6+8 composition (Grid + Graph trait), BFS with VecDeque/HashSet, memoized DFS with HashMap caching, pattern matching from Ch19.1 (match, if let, while let, tuple destructuring), and the integrator philosophy (compose validated components, add custom optimizations)
- **Day 8**: Demonstrates Mission 10 Union-Find integration for connectivity problems, `partial_cmp()` for floating-point sorting, iterator patterns with `enumerate()` for bounded loops, component counting vs member enumeration API differences, the importance of reading problem statements carefully ("examine N pairs" vs "make N connections"), and **performance optimization via squared distances** (12% speedup by eliminating `sqrt()` and using `i64` instead of `f64` - monotonicity preservation means sorting by d² gives same order as d)
- **Day 9**: Demonstrates **when NOT to materialize grids** (37GB crash teaches sparse representation), `HashSet<(i64, i64)>` for boundary-only storage (~589K tiles vs 9.36B), Bresenham's line algorithm for polygon edge generation, ray casting point-in-polygon with edge crossing counter, **AABB sampling pattern** with adaptive rate `max(10, dim/100)` reducing billions of checks to thousands, **algorithm evolution documentation** (4 failed approaches before success), Mission applicability limits (Grid perfect for small examples, geometric algorithms for massive sparse spaces), and the critical lesson that **checking input scale first prevents wasted implementation effort**
- **Day 10**: Demonstrates **problem type recognition** (Part 1 = GF(2) linear algebra, Part 2 = NP-hard ILP requiring specialized solvers), **Gaussian elimination over binary field** with XOR operations and free variable enumeration, **pure Rust ILP solver** via `good_lp` crate with minilp feature (avoiding native dependency issues), **floating-point precision handling** with `.round()` instead of truncation (critical fix changing 16754→16757), **algorithm evolution through 6 failed attempts** (backtracking timeout, greedy suboptimal, BFS exponential space, A* inadmissible heuristic, Z3 compilation failure, finally ILP success), **state space analysis** revealing why search fails (machine 3: 194^9 ≈ 10^20 states), **community solution adaptation** (Tom Wilkinson's ILP approach), and the lesson that **specialized algorithms beat general search for constraint optimization** - see [[../examples/day10_solution_analysis]] for complete journey with performance comparisons and mathematical formulations
- **Day 11**: Demonstrates **state-based memoization** with composite keys `(node, visited_mask)`, **bitmask state representation** for tracking visited required nodes (2 required → 4 states per node), **HashMap<String, Vec<String>>** for adjacency list storage, **Mission 8 Graph trait limitation** (Copy constraint blocks String nodes), **performance debugging** (naive v1 timeout → optimized v2 instant), and the lesson that **proper state representation prevents exponential blowup** (549 trillion paths computed via memo, not enumeration)

---

## Adding New Days

To add a new day to this summary:

1. **Read the problem statement**
2. **Identify the core algorithm type** for each part
3. **Add entry following the format above**
4. **Update the distribution table**
5. **Note any new patterns or Rust learning opportunities**
6. **⚠️ CRITICAL: Verify Rust-specific claims against actual implementation code**

### Documentation Quality Lesson Learned

**Always inspect actual code before documenting patterns.**  This highlights the importance of **evidence-based documentation** over **assumption-based documentation**.

**Verification Checklist**:

- [ ] Read the actual Rust implementation file
- [ ] Document patterns that are **actually present** in the code
- [ ] Note deliberate trade-offs (e.g., performance vs functional style)
- [ ] Compare claimed patterns against `grep`/search results in codebase

### Template for New Days

```markdown
### Day X: [Problem Title]
**Title**: [Problem Title]  
**Part 1 Type**: [Category]  
**Part 1 Description**: [Brief description]  
**Part 2 Type**: [Category]  
**Part 2 Description**: [Brief description]  
**Key Concepts**: [Relevant programming concepts]
```

---

*Last Updated: December 11, 2025*
*Days Implemented: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11*
*Days Available: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12*

---
*Tags: #aoc #2025 #problem-analysis #patterns #algorithm-learning*

*Links: [[day01]] | [[day02]] | [[day03]] | [[day04]] | [[day05]] | [[day06]] | [[day07]] | [[day08]] | [[day09]] | [[day10]] | [[day11]] | [[../examples/day01_debugging_analysis]] | [[../examples/day10_solution_analysis]] | [[../../../zettelkasten/AoC Patterns MOC]] | [[../../../zettelkasten/AoC Integration]]*
