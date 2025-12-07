# Advent of Code 2025 - Problem Summary

This document provides a categorized overview of all Advent of Code 2025 problems, organized by day with problem types for both parts.

## Problem Categories

- **Advanced Pattern Matching**: Complex pattern constraints, non-overlapping patterns
- **Brute Force**: Exhaustive search through solution space
- **Cellular Automaton**: Conway's Game of Life, state evolution, neighbor counting, grid simulation
- **Combinatorial Optimization**: Subset sum, container packing, constrained combination enumeration
- **Conditional Logic**: Property-based filtering, range-based matching, rule-based comparisons
- **Cryptographic**: Hash functions, encryption, cryptographic puzzles
- **Data Structures**: Working with arrays, lists, sets, maps
- **Encoding**: String encoding, character escaping
- **Graph Algorithms**: Graph traversal, shortest path, connectivity analysis
- **Greedy Algorithms**: Optimal greedy strategies, reverse optimization, exploiting problem structure
- **Grid Processing**: 2D grid parsing, coordinate-based access, neighbor queries, spatial reasoning
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

## Problem Type Distribution (Available Days)

| Category | Part 1 Count | Part 2 Count |
|----------|--------------|--------------|
| Advanced Pattern Matching | 0 | 0 |
| Brute Force | 0 | 0 |
| Cellular Automaton | 1 | 1 |
| Combinatorial Optimization | 0 | 0 |
| Conditional Logic | 0 | 0 |
| Range Containment | 1 | 0 |
| Interval Merging | 0 | 1 |
| Cryptographic | 0 | 0 |
| Data Structures | 0 | 0 |
| Encoding | 0 | 0 |
| Graph Algorithms | 0 | 0 |
| Greedy Algorithms | 1 | 1 |
| Grid Processing | 1 | 0 |
| Iterative Erosion | 0 | 1 |
| Mathematical | 1 | 1 |
| Number Theory | 0 | 0 |
| Optimization | 0 | 0 |
| Parsing | 0 | 0 |
| Pattern Matching | 1 | 1 |
| Real-time Analysis | 0 | 0 |
| Search | 0 | 0 |
| Search/Traversal | 0 | 0 |
| Simulation | 1 | 1 |
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
- **Part 2 escalation pattern**: All days follow classic AoC pattern of Part 2 expanding the problem scope

### Rust-Specific Considerations

- **Day 1**: Demonstrates signed integer arithmetic advantages for circular problems, comprehensive error handling with `anyhow::Result`, and the importance of special case handling for boundary conditions (position 0)
- **Day 2**: Showcases string slicing with `split_at()`, pattern repetition detection using divisibility checks, and functional iteration with `filter()`/`sum()` for range processing
- **Day 3**: Highlights greedy iteration patterns, `.find()` for first-match semantics in tie-breaking, and `u64` for large number results
- **Day 4**: Demonstrates Mission 6 Grid integration, `neighbors_8_bounded()` for automatic boundary handling, and iterative state modification with in-place mutation
- **Day 5**: Showcases importance of checking input scale before choosing algorithms, interval merging for huge ranges, and the difference between enumeration vs mathematical counting

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

*Last Updated: December 5, 2025*
*Days Implemented: 1, 2, 3, 4, 5*
*Days Available: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12*

---
*Tags: #aoc #2025 #problem-analysis #patterns #algorithm-learning*

*Links: [[day01]] | [[day02]] | [[day03]] | [[day04]] | [[day05]] | [[../examples/day01_debugging_analysis]] | [[../../../zettelkasten/AoC Patterns MOC]] | [[../../../zettelkasten/AoC Integration]]*
