# Advent of Code 2024 - Problem Summary

This document provides a categorized overview of all Advent of Code 2024 problems, organized by day with problem types for both parts.

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
- **Mathematical**: Arithmetic calculations, formulas, geometric problems
- **Number Theory**: Divisor sums, highly composite numbers, sieve algorithms, multiplicative functions
- **Optimization**: Finding minimum/maximum values
- **Parsing**: Escape sequence parsing, character-level analysis
- **Pattern Matching**: Regular expressions, string validation, substring detection
- **Real-time Analysis**: Temporal scoring, moment-by-moment leader tracking, time-dependent calculations
- **Search**: Informed search algorithms, A* search, heuristics, state space exploration
- **Search/Traversal**: Finding positions, tracking states
- **Simulation**: State tracking, following instructions step-by-step
- **String Processing**: Character manipulation, parsing, pattern matching

---

## Day-by-Day Summary

### Day 1: Historian Hysteria
**Title**: Historian Hysteria  
**Part 1 Type**: Data Structures + Mathematical  
**Part 1 Description**: Calculate total distance between two lists by pairing smallest elements after sorting  
**Part 2 Type**: Data Structures + Mathematical  
**Part 2 Description**: Calculate similarity score by multiplying each left number by its frequency in right list  
**Key Concepts**: List processing, sorting algorithms, frequency counting with HashMap, iterator chains  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Two-part escalation (distance metric → similarity metric)
- **Data Structure**: Vec for sorting, HashMap for frequency counting
- **Complexity**: Part 1: O(n log n) sorting, Part 2: O(n) frequency map construction + O(n) scoring
- **AoC Theme**: Classic "process pairs of lists" with different metrics

**🦀 Rust Conversion Highlights**:
- **From Python manual loops** → **Functional iterator chains** (`zip`, `map`, `fold`, `sum`)
- **From exception handling** → **Explicit `Result<T, E>` with detailed error context**
- **From dynamic typing** → **Compile-time type safety with pattern matching**
- **From multiple data copies** → **In-place sorting and zero-cost abstractions**

**Performance**: Python→Rust identical results, significantly improved safety and performance

### Day 2: Red-Nosed Reports
**Title**: Red-Nosed Reports  
**Part 1 Type**: Conditional Logic + Mathematical  
**Part 1 Description**: Count "safe" reports where levels are all increasing/decreasing by 1-3  
**Part 2 Type**: Conditional Logic + Optimization  
**Part 2 Description**: Count safe reports allowing removal of one problematic level (Problem Dampener)  
**Key Concepts**: Range validation, monotonicity checking, brute force optimization, error tolerance  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Safety validation with escalating tolerance (strict rules → dampener allowance)
- **Data Structure**: Vec for sequences, sliding window comparisons
- **Complexity**: Part 1: O(n) per report, Part 2: O(n²) brute force (try removing each element)
- **AoC Theme**: "Safety analysis" with classic Part 2 tolerance mechanism

**🦀 Rust Conversion Highlights**:
- **From nested loops** → **Iterator windows with imperative early-return validation**
- **From manual bounds checking** → **Range-based validation with `contains()`**
- **From list slicing** → **`to_vec()` + `remove()` for element removal simulation**
- **From Python's functional `all()`** → **Manual state tracking with `Option<bool>` for performance**

**Performance**: Efficient O(n) validation for Part 1, acceptable O(n²) brute force for Part 2 with small input size

### Day 3: Mull It Over
**Title**: Mull It Over  
**Part 1 Type**: Pattern Matching + String Processing  
**Part 1 Description**: Parse corrupted memory for valid `mul(X,Y)` instructions and sum multiplication results  
**Part 2 Type**: Pattern Matching + Conditional Logic  
**Part 2 Description**: Handle `do()` and `don't()` conditional statements to enable/disable multiplication processing  
**Key Concepts**: Regular expressions, pattern validation, state machine logic, instruction parsing, escape sequence handling  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Instruction parsing with escalation (simple pattern matching → stateful conditional processing)
- **Data Structure**: Regex for pattern matching, Vec for instruction sequences, enum for instruction types
- **Complexity**: Part 1: O(n) regex scanning, Part 2: O(n) single-pass state machine with enable/disable tracking
- **AoC Theme**: "Corrupted memory parsing" with classic Part 2 conditional complexity (stateless → stateful processing)

**🦀 Rust Conversion Highlights**:
- **From Python regex groups** → **Rust regex `Captures` with explicit error handling**
- **From dynamic instruction types** → **Type-safe `enum Instruction` with pattern matching**
- **From manual string parsing** → **`anyhow::Context` for detailed parse error reporting**
- **From implicit state tracking** → **Explicit `enabled` boolean with clear state transitions**

**Performance**: Single-pass O(n) regex processing for both parts, efficient state machine for conditional logic

### Day 4: Ceres Search
**Title**: Ceres Search  
**Part 1 Type**: String Processing + Search/Traversal  
**Part 1 Description**: Find all occurrences of "XMAS" in a 2D word search grid, searching in all 8 directions (horizontal, vertical, diagonal, forwards and backwards)  
**Part 2 Type**: Pattern Matching + Search/Traversal  
**Part 2 Description**: Find X-shaped "MAS" patterns where two "MAS" words intersect at their 'A' in an X formation  
**Key Concepts**: 2D grid traversal, directional search algorithms, bounds checking, pattern recognition, grid parsing  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Grid-based word search with escalation (linear word search → geometric pattern matching)
- **Data Structure**: `Vec<Vec<char>>` for 2D grid, direction vectors for 8-directional search, coordinate arithmetic
- **Complexity**: Part 1: O(n*m*8*4) for each position checking 8 directions for 4-char word, Part 2: O(n*m) single pass checking X patterns at each interior position
- **AoC Theme**: "Word search puzzle" with classic Part 2 geometric complexity (1D word patterns → 2D shape patterns)

**🦀 Rust Conversion Highlights**:
- **From direction-specific loops** → **Unified directional vector approach** with `[(row_delta, col_delta); 8]` array
- **From string concatenation** → **Character-by-character matching** with zero allocations
- **From separate forward/backward searches** → **Single unified search** detecting patterns in all orientations
- **From hardcoded pattern checks** → **Logical pattern decomposition** using diagonal validation

**Python vs Rust Comparison**:
- **Python**: Direction-specific approach (horizontal, vertical, diagonal loops), string building with `''.join()`, explicit pattern enumeration for X-MAS (4 separate conditions)
- **Rust**: Mathematical direction vectors, zero-allocation character matching, logical diagonal decomposition for X-MAS patterns
- **Algorithm Philosophy**: Python more literal/explicit, Rust more mathematical/elegant

**Performance**: Rust eliminates string allocation overhead, provides compile-time bounds checking, and uses efficient directional traversal

**🏗️ Mission 6 Refactoring Example**:
- **[[../examples/day04_wm6.rs]]** - Alternative implementation using Mission 6 grid utilities
- **Code Reduction**: 280 lines → 160 lines (43% reduction)
- **Safety Improvement**: Manual bounds checking → Automatic safety guarantees
- **Architectural Benefits**: Problem-specific code → Reusable foundational library
- **Semantic Clarity**: `(row + dy, col + dx)` → `coord.step(Direction::NorthEast)`
- **Validation**: Identical results (Part 1: 2554, Part 2: 1916) with improved maintainability
- **V-Cycle Demonstration**: Shows how foundational libraries dramatically simplify complex algorithms
- **Documentation**: Complete analysis in [[../examples/README_day04_wm6.md]] and [[../../../zettelkasten/aoc2024-day4-mission6-example]]

### Day 5: Print Queue
**Title**: Print Queue  
**Part 1 Type**: Graph Algorithms + Conditional Logic  
**Part 1 Description**: Validate page ordering sequences against dependency rules, sum middle pages of correctly ordered updates  
**Part 2 Type**: Graph Algorithms + Optimization  
**Part 2 Description**: Fix incorrectly ordered sequences using topological sorting, sum middle pages of fixed sequences  
**Key Concepts**: Dependency graphs, topological sorting, cycle detection, Kahn's algorithm, adaptive algorithms, rule-based validation  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Dependency ordering with escalation (validation → repair with topological sorting)
- **Data Structure**: Mission 7 `Graph<T>` for dependencies, HashMap for bidirectional node mapping, adjacency lists for Mission 8 algorithms
- **Complexity**: Part 1: O(n²) per sequence validation, Part 2: O(V + E) Kahn's algorithm or O(n²) rule-based fallback
- **AoC Theme**: "Print queue dependency management" with classic Part 2 repair mechanism (validate → fix using graph theory)

**🦀 Rust Conversion Highlights**:
- **From manual graph construction** → **Mission 7 `Graph<T>` with type-safe node management**
- **From custom cycle detection** → **Mission 8 `has_cycle()` with proven algorithms**
- **From hardcoded topological sort** → **Kahn's algorithm with adaptive fallback for cyclic graphs**
- **From error-prone dependency tracking** → **Bidirectional HashMap mapping between page numbers and NodeId**

**Real-World Complexity Handling**:
- **Global rules**: 1,176 ordering relationships with cycles detected by Mission 8
- **Adaptive algorithm**: Falls back from optimal Kahn's to rule-based bubble sort when cycles exist
- **Production robustness**: Handles edge cases gracefully rather than failing hard
- **Mission integration validation**: Cycle detection provides valuable insights while allowing solution to continue

**🏗️ Mission 7 + Mission 8 Integration**:
- **[[../examples/day05_real_missions.rs]]** - Real Mission integration implementation
- **[[../examples/day05_real_missions_clean.rs]]** - Comment-free version for walkthroughs
- **[[../examples/DAY05_REAL_MISSIONS_WALKTHROUGH.md]]** - Complete architectural analysis
- **Code Reduction**: ~40% reduction through foundational library integration
- **Safety Improvement**: Automatic bounds checking and validation through Mission APIs
- **Architectural Benefits**: Graph theory abstraction → competitive programming application
- **Mission Validation**: Real cycle detection identifies data complexity (49 nodes, 1,176 edges, density 0.500)
- **Results**: Part 1: 4,872, Part 2: 5,564 (real AoC dataset with cycle handling)
- **V-Cycle Demonstration**: Requirements → Mission APIs → adaptive implementation → validation

**Performance**: Efficient O(V + E) when no cycles, graceful O(n²) fallback when cycles detected, demonstrates production-quality error handling

**Python vs Rust Comparison**:
- **Python Approach**: Simple rule-based bubble sort for both validation and fixing (~30 lines total)
- **Algorithm Validation**: Python solution uses identical bubble sort logic as our Rust fallback method
- **Philosophy Difference**: Python optimizes for simplicity and speed, Rust Missions invest in architectural depth and reusability
- **Cycle Handling**: Python implicitly handles cycles through bubble sort, Rust explicitly detects cycles with Mission 8 then adapts algorithm
- **Educational Value**: Python gets correct results efficiently, Rust provides graph insights (cycle detection, density analysis, foundational libraries)
- **Code Complexity**: Python ~30 lines pragmatic solution vs Rust ~200 lines with Mission integration, type safety, and architectural benefits

### Day 6: Guard Gallivant
**Title**: Guard Gallivant  
**Part 1 Type**: Simulation + Search/Traversal  
**Part 1 Description**: Simulate guard patrol with obstacle avoidance, count distinct positions visited before exiting lab bounds  
**Part 2 Type**: Simulation + Optimization  
**Part 2 Description**: Find positions where placing obstacles creates infinite patrol loops (Part 1: 5551, Part 2: 1939)  
**Key Concepts**: Guard state tracking, bounds checking, loop detection, collision avoidance, infinite loop analysis, position-based obstacle testing

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Simulation with escalation (linear patrol simulation → infinite loop detection through obstacle placement)
- **Data Structure**: Mission 6 `Grid<char>` for lab map, Mission 6 `Coord` for positions, Mission 6 `Direction` with rotation methods, Mission 5 `HashSet<GuardState>` for loop detection
- **Complexity**: Part 1: O(W×H) visits each position at most once, Part 2: O(P×W×H) where P is path length—tests obstacle at each path position
- **AoC Theme**: "Guard patrol simulation" with classic Part 2 infinite loop challenge (finite simulation → cycle detection optimization)

**🦀 Rust Conversion Highlights**:
- **From manual coordinate arithmetic** → **Mission 6 `Coord.step(Direction)` with automatic bounds checking and overflow protection**
- **From hardcoded direction arrays** → **Mission 6 `Direction` enum with `rotate_90_clockwise()` method for type-safe rotations**
- **From manual grid indexing** → **Mission 6 `Grid<char>` with safe `.get()` access preventing buffer overflows**
- **From tuple-based state tracking** → **Type-safe `GuardState` struct with `position` and `direction` fields for clear state management**
- **From nested loops with manual exit conditions** → **Mission 5 `HashSet` for O(1) loop detection using state-based cycle identification**

**🏗️ Mission 6 + Mission 5 Integration Benefits**:
- **[[../examples/day06_comprehensive_walkthrough.rs]]** - Complete walkthrough with Mission integration analysis
- **[[DAY06_PYTHON_VS_RUST_ANALYSIS]]** - Comprehensive comparison with Python reference solutions  
- **Code Quality**: Type safety eliminates entire bug classes (bounds errors, direction confusion, state tracking mistakes)
- **Safety Improvement**: All coordinate operations bounds-checked automatically, no possibility of array access violations
- **Architectural Benefits**: Leverages proven V-Cycle data structures for competitive programming application
- **Mission Integration**: `Grid<T>`, `Coord`, `Direction` from Mission 6 + `HashSet` collections from Mission 5 = complete solution framework
- **Results**: Part 1: 5551, Part 2: 1939 with comprehensive test coverage (8 unit tests) and zero clippy warnings
- **Educational Value**: Demonstrates how foundational libraries make complex algorithms more reliable and maintainable

**Real-World Complexity Handling**:
- **State Management**: `GuardState` struct encapsulates position + direction for comprehensive loop detection
- **Bounds Safety**: Mission 6 `Coord.step()` prevents coordinate underflow/overflow through `Option<Coord>` returns  
- **Error Handling**: Comprehensive `anyhow::Result` throughout parsing with detailed error context
- **Performance**: O(1) loop detection through `HashSet<GuardState>` vs naive O(n²) position checking approaches

**Performance**: Mission-leveraged implementation achieves optimal algorithmic complexity while maintaining safety guarantees

### Day 7: Bridge Repair
**Title**: Bridge Repair  
**Part 1 Type**: Brute Force + Mathematical  
**Part 1 Description**: Find calibration equations solvable with + and * operators (left-to-right evaluation), sum valid test values  
**Part 2 Type**: Brute Force + Combinatorial Optimization  
**Part 2 Description**: Add concatenation || operator to expand solution space, sum all valid equations (Sample: 3749 → 11387, Real: 20665830408335 → 354060705047464)  
**Key Concepts**: Expression evaluation, operator combination generation, left-to-right precedence, brute force search, TDD methodology, string concatenation as mathematical operation  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Operator insertion with escalation (2-operator brute force → 3-operator exponential search)
- **Data Structure**: Vec for equations, enum for type-safe operators, explicit left-to-right evaluation engine
- **Complexity**: Part 1: O(2^(N-1)) per equation, Part 2: O(3^(N-1)) per equation where N = number count
- **AoC Theme**: "Missing operators" with classic Part 2 expansion (limited operators → additional concatenation complexity)

**🦀 Rust Conversion Highlights**:
- **From dynamic algorithm** → **Structured TDD approach** with 5 implementation phases (32 comprehensive tests)
- **From iterative state building** → **Systematic combination generation** using base conversion mathematics
- **From implicit operator precedence** → **Explicit left-to-right evaluation** with mathematical precision
- **From string-based concatenation** → **Type-safe `Operator` enum** with pattern matching and overflow handling
- **From ad-hoc parsing** → **Comprehensive error handling** with `anyhow::Result` and detailed error context

**🏗️ TDD Implementation Excellence**:
- **[[DAY07_SOLUTION_OUTLINE.md]]** - Complete TDD planning document with 5-phase breakdown
- **[[DAY07_TDD_COMPLETION_REPORT.md]]** - Success metrics and architectural analysis
- **Test Coverage**: 32/32 tests pass covering all edge cases, parsing, evaluation, combinations, and integration
- **Code Quality**: Zero clippy warnings, comprehensive error handling, production-ready structure
- **Phase Structure**: Data structures → Expression evaluation → Combination generation → Equation validation → Solution integration
- **Results**: Sample (3749, 11387) and real puzzle answers (20665830408335, 354060705047464) with perfect accuracy
- **Educational Value**: Demonstrates professional TDD methodology applied to competitive programming

**Real-World Complexity Handling**:
- **Combination Explosion**: Efficiently handles 2^(N-1) and 3^(N-1) operator combinations using mathematical base conversion
- **Left-to-Right Evaluation**: Proper precedence handling where `81 + 40 * 27 = (81+40)*27 = 3267`, not standard `81 + (40*27)`
- **String Concatenation**: `15 || 6 = 156` implemented as format + parse with overflow protection
- **Expression Safety**: All arithmetic operations checked for validity, graceful error handling for invalid combinations

**Python vs Rust Comparison**:
- **Algorithm Philosophy**: 
  - **Python**: Dynamic state building approach—maintains `possibles` list, adds new values iteratively, prunes values > test_value for performance
  - **Rust**: Systematic combination enumeration—generates all operator combinations upfront, evaluates each possibility deterministically
- **Implementation Style**: 
  - **Python**: ~40 lines, imperative loops with list building, `numbers.pop(0)` for sequential processing
  - **Rust**: ~530 lines with 32 tests, structured TDD approach, type-safe enum operators, comprehensive error handling
- **Performance Optimization**:
  - **Python**: Early pruning with `if v <= test_value` to reduce search space, efficient for typical inputs
  - **Rust**: Brute force all combinations but with zero-allocation evaluation, optimal for correctness verification
- **Error Handling**: 
  - **Python**: Assumes valid input, potential runtime failures on malformed data
  - **Rust**: Comprehensive `Result<T>` throughout, detailed error context, graceful failure handling
- **Code Philosophy**:
  - **Python**: Optimize for simplicity and speed (~40 lines total for both parts)
  - **Rust**: Invest in architecture, safety, and educational value (530+ lines with comprehensive test suite)
- **Concatenation Implementation**:
  - **Python**: `int(f"{p}{curr}")` - direct string formatting and conversion
  - **Rust**: `format!("{}{}", left, right).parse().unwrap_or(0)` - explicit error handling for edge cases
- **Results**: Both achieve identical answers, Python optimizes for competitive speed, Rust for production quality and learning

**Educational Insights**:
- **TDD Effectiveness**: 32 tests caught multiple edge cases during implementation, proving TDD value for algorithmic problems
- **Type Safety Benefits**: `Operator` enum eliminated entire bug class (operator confusion, precedence errors)
- **Architecture Investment**: Rust's structured approach creates reusable patterns for similar combination problems
- **Algorithm Clarity**: Explicit combination generation makes algorithm behavior transparent and debuggable

### Day 8: Resonant Collinearity
**Title**: Resonant Collinearity  
**Part 1 Type**: Mathematical + Pattern Matching  
**Part 1 Description**: For each unordered pair of same-frequency antennas, extrapolate displacement to place two antinodes where one antenna is twice as far from the antinode as the other (`p1 - (p2 - p1)` and `p2 + (p2 - p1)` if in bounds).  
**Part 2 Type**: Mathematical + Search/Traversal  
**Part 2 Description**: Harmonic resonance saturates every grid position collinear with ≥2 same-frequency antennas using primitive step ray casting in both directions; antennas themselves count when a frequency appears ≥2 times.  
**Key Concepts**: Frequency grouping, vector displacement, gcd normalization (primitive direction), bidirectional ray casting, geometric pattern detection, mission-based grid abstraction

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Geometric extrapolation → harmonic line saturation (ratio constraint removed in Part 2)
- **Data Structures**: Mission 6 `Grid<char>` & `Coord`, `HashMap<char, Vec<Coord>>` for grouping, `HashSet<Coord>` for uniqueness
- **Part 1 Complexity**: O(k²) per frequency group (unordered pairs); each pair produces ≤2 antinodes
- **Part 2 Complexity**: O(k² * L) where L ≤ max(width,height)
- **Correctness Pillar**: GCD normalization guarantees capturing all lattice points (no skipped intermediate harmonics)

**🦀 Rust Conversion Highlights**:
- Mission 6 abstractions eliminate manual bounds/index checks
- Primitive step derivation (`dx/g, dy/g`) prevents missed interior points
- Clean separation of concerns (`solve_part1` / `solve_part2`)
- HashSet dedup yields exact unique antinode counts

**Real-World Complexity Handling**:
- Dense clusters safely deduped
- Overlapping antenna-as-antinodes require no extra branching
- Arbitrary slopes (e.g. 5/7) fully traversed due to primitive direction

**Python vs Rust Comparison**:
- Grouping identical (`defaultdict(list)` vs `HashMap<char, Vec<Coord>>`)
- Part 1 logic equivalent pair displacement extrapolation
- Part 2 divergence: Python steps with full displacement (risks skipping harmonic points when gcd>1, relies on overlapping coverage from all pairs); Rust normalizes to primitive vector
- Rust leverages mission abstractions + explicit error handling; Python relies on tuple arithmetic and implicit assumptions
- Both achieve completeness: Python via overlapping pair coverage, Rust via primitive direction precision

**Educational Insights**:
- Introduces reusable primitive direction normalization for geometry/visibility problems
- Demonstrates correctness-first design vs minimal pragmatic approach
- Reinforces value of foundational spatial types (`Coord`, `Grid`) for algorithm clarity

**Results**: Sample (14, 34) – Puzzle (271, 994)

**Mission Integration Benefit**: Grid & coordinate hashing reduce incidental complexity, focusing effort on geometric reasoning.

### Day 9: Disk Fragmenter
**Title**: Disk Fragmenter  
**Part 1 Type**: Simulation + Data Structures  
**Part 1 Description**: Interpret the dense disk map, treat digits as alternating file and gap lengths, then repeatedly pull the rightmost file blocks into the leftmost gaps to remove every hole before computing the checksum.  
**Part 2 Type**: Simulation + Optimization  
**Part 2 Description**: Move *whole* files exactly once, scanning from the largest ID downward and sliding each file into the earliest gap to its left that can contain it; recompute the checksum after every relocation.  
**Key Concepts**: Dense disk decoding, block-level simulation, Mission 5 dictionary reuse for file metadata, gap tracking, checksum reduction, visualization hooks (`examples/day09_visualization.rs`).

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Memory compaction escalation (single-block sliding → whole-file relocation with greedy nearest-gap fit).
- **Data Structure**: `Vec<Option<u32>>` for block occupancy, `Dictionary<usize, FileInfo>` from Mission 5 for O(1) file metadata, `Vec<GapSegment>` for sorted free-space spans.
- **Complexity**: Part 1 runs in O(total_blocks) by converging two pointers from both ends; Part 2 is O(F·G) where F is file count and G is number of gaps scanned per file (worst-case O(F²) but bounded by problem input sizes).
- **AoC Theme**: Disk defragmentation with progressively stricter movement rules (block shuffling → contiguous file moves).

**🦀 Rust Conversion Highlights**:
- **Structured Parsing**: `parse_disk` enforces digit-only input, pre-allocates block vectors, and records file/gap metadata as the dense map is decoded.
- **Safe Compaction**: `compact_blocks` implements the two-pointer pull without reallocations, while `compact_whole_files` reuses dictionary metadata to update positions after every move.
- **Checksum Accuracy**: Dedicated `checksum` helper mirrors the puzzle requirement (`position * file_id`) and doubles as a regression oracle for the visualization example.
- **Error Context**: `anyhow::Context` pinpoints malformed digits or empty inputs, which was easy to miss in the Python reference implementation.

**Mission Integration & Visualization**:
- Mission 5’s `Dictionary` keeps file metadata synchronized during moves—no ad-hoc hash maps needed and type safety prevents stale references.
- `examples/day09_visualization.rs` streams every intermediate block state (pre/post compaction) so the disk behavior can be inspected or animated, mirroring the narrative screenshots in the puzzle statement.

**Python vs Rust Comparison**:
- Python solution (`2024py/solutions/day09.py`) mutates per-block vectors and stores gaps as indices, but relies on implicit assumptions (e.g., trailing gaps at the end) and lacks strong typing.
- Rust tracks both files and gaps explicitly, uses dedicated structs (`FileInfo`, `GapSegment`), and updates metadata atomically, avoiding subtle off-by-one errors during whole-file moves.
- Visualization tooling in Rust doubles as an integration test for both compilation targets (solver + example), whereas Python depends on print-based debugging.

**Results & Takeaways**:
- Sample checksum: Part 1 = 1928, Part 2 = 2858; puzzle input answers match official AoC totals (6519155389266, 6547228115826).
- Demonstrates how mission libraries make even 1D simulations safer, and how adding an `examples/` visualizer can convert puzzle mechanics into reusable teaching material.

### Day 10: Hoof It

**Title**: Hoof It  
**Part 1 Type**: Graph Algorithms + Search/Traversal  
**Part 1 Description**: Find hiking trail trailheads (height 0), use BFS to count reachable summits (height 9) where each step increases height by exactly 1 in 4 cardinal directions  
**Part 2 Type**: Graph Algorithms + Search/Traversal  
**Part 2 Description**: Count all distinct hiking trails (paths) from each trailhead to any summit, where trails can share positions but form unique routes  
**Key Concepts**: Topographic map parsing, breadth-first search for reachability, depth-first search for path counting, Mission 6 Grid/Coord abstraction, Mission 8 Graph trait integration, height-based traversal constraints

**🧩 Algorithm Analysis**:

- **Problem Pattern**: Graph traversal escalation (reachable nodes → distinct path counting)
- **Data Structure**: Mission 6 `Grid<Option<u32>>` for topographic heights, Mission 6 `Coord` for positions, Mission 8 `Graph` trait with `bfs()`, custom DFS for path enumeration
- **Complexity**: Part 1: O(V+E) BFS per trailhead, Part 2: O(V+E) DFS with exponential branching for path counting (bounded by small grid sizes)
- **AoC Theme**: "Hiking trail analysis" with classic Part 2 escalation (unique destinations → all distinct paths)

**🦀 Rust Conversion Highlights**:

- **From manual grid management** → **Mission 6 `Grid<Option<u32>>`** with automatic bounds checking and safe access
- **From tuple positions `(usize, usize)`** → **Mission 6 `Coord` type** with x/y semantics preventing coordinate confusion
- **From manual direction arrays** → **Mission 6 `coord.neighbors_4()` iterator** encapsulating 4-directional movement
- **From manual bounds checking** → **`grid.in_bounds(coord)` and `grid.get(coord)`** providing safety guarantees
- **From nested loops** → **`grid.enumerate()` functional iteration** over (Coord, &T) pairs
- **From custom BFS implementation** → **Mission 8 generic `bfs()` algorithm** working via Graph trait
- **From Python's `list.pop(0)` O(n)** → **Mission 8's `VecDeque` O(1)** queue operations in BFS

**🏗️ Mission 6 + Mission 8 Composition Benefits**:

- **[[../examples/aoc_day10_hiking.rs]]** - Mission 8 example demonstrating Graph trait with AoC problem (303 lines, manual grid)
- **[[../src/solver/day10.rs]]** - Production solver refactored to use Mission 6 Grid + Coord infrastructure (~160 lines)
- **Code Quality**: Type safety eliminates coordinate bugs (tuple x/y swapping impossible with Coord)
- **Code Reduction**: ~10 lines shorter through Mission 6 integration, eliminated manual bounds checking and direction handling
- **Safety Improvement**: All grid access bounds-checked automatically, `Option<Coord>` returns prevent overflow/underflow
- **Architectural Benefits**: Demonstrates mission composition—Grid (Mission 6) + Graph/BFS (Mission 8) = complete topographic solution
- **Clippy Clean**: Zero warnings after removing redundant `.into_iter()` (neighbors_4() already returns iterator, not Vec)
- **Test Coverage**: 12 comprehensive tests covering parsing, graph operations, edge cases, and both parts
- **Results**: Sample (36, 81), Puzzle (512, 1045) with comprehensive validation

**Real-World Complexity Handling**:

- **Type Safety**: `Option<u32>` for heights (Some(0..=9) passable, None impassable '.') prevents treating impassable as valid
- **Graph Abstraction**: TopoMap implements Graph trait making Mission 8's generic algorithms available
- **Algorithm Selection**: BFS for Part 1 (unique destinations), custom DFS for Part 2 (all paths)—demonstrates when to use vs extend mission libraries
- **Iterator Efficiency**: Mission 6's `neighbors_4()` returns iterator (not Vec), more memory efficient than collecting

**Python vs Rust Comparison**:

- **Algorithm Philosophy**:
  - **Python**: Pragmatic BFS using `list.pop(0)` (O(n) but acceptable for small inputs), single `get_score()` function reused for both parts with set deduplication
  - **Rust**: Structured approach with Mission 8 BFS (O(1) VecDeque), separate algorithms for Part 1 (BFS reachability) vs Part 2 (DFS path counting)
- **Implementation Style**:
  - **Python**: ~50 lines, manual bounds checking every neighbor, tuple positions `(y, x)`, direction array `[(0,1), (1,0), (0,-1), (-1,0)]`
  - **Rust**: ~160 lines with 12 tests, Mission 6 Grid/Coord abstractions, Mission 8 Graph trait, type-safe height validation
- **Performance**:
  - **Python**: O(n) queue operations acceptable for racing at midnight, no visited set in BFS (revisits nodes)
  - **Rust**: O(1) queue operations via VecDeque, HashSet visited tracking in Mission 8 BFS prevents redundant work
- **Error Handling**:
  - **Python**: Assumes all inputs are digits, IndexError possible on malformed input
  - **Rust**: Validates characters, handles '.' explicitly, comprehensive error context with `anyhow::Result`
- **Code Philosophy**:
  - **Python**: Optimize for midnight leaderboard speed (~10-15 minutes to implement)
  - **Rust**: Invest in learning, architecture, and production quality (demonstrates mission composition benefits)

**Educational Insights**:

- **Mission Composition**: Day 10 demonstrates practical benefits of foundational libraries—Grid handles 2D storage, Coord prevents coordinate errors, Graph enables generic BFS
- **Type-Driven Design**: `Coord` type is self-documenting vs ambiguous tuples, compiler enforces correct usage
- **Iterator Design**: Mission 6's `neighbors_4()` returning iterator (not Vec) teaches efficient API design
- **Graph Trait Power**: Generic `bfs<G: Graph>(graph, start)` works on any Graph implementation—topographic maps, abstract graphs, trees
- **Algorithm Selection**: Shows when to use mission libraries (BFS for Part 1) vs custom code (DFS for Part 2 path counting)
- **V-Cycle Validation**: Initial manual implementation (commit d082003) proves algorithm correctness, refactoring (commit c6b2283) proves missions compose properly, tests validate functional equivalence

**Mission Integration Benefit**:

- Grid & Coord reduce manual bounds checking and coordinate arithmetic
- Graph trait enables battle-tested algorithms from Mission 8
- Composition pattern: build generic components, compose for domain problems
- Less code, better type safety, clearer intent—exactly what mission system designed for

### Day 11: Plutonian Pebbles

**Title**: Plutonian Pebbles
**Part 1 Type**: Simulation + Mathematical
**Part 1 Description**: Simulate stone transformations for 25 blinks using three rules: 0→1, even-digit split, else multiply by 2024. Count total stones after all blinks.
**Part 2 Type**: Optimization + Mathematical
**Part 2 Description**: Scale to 75 blinks using memoization—exponential growth makes naive simulation impossible (would take days + terabytes RAM).
**Key Concepts**: Exponential growth, dynamic programming, memoization with (stone, blinks_remaining) cache key, digit counting optimization, integer arithmetic, cache efficiency analysis

**🧩 Algorithm Analysis**:

- **Problem Pattern**: Simulation escalation (manageable naive approach → requires optimization through memoization)
- **Data Structure**: `Vec<u64>` for naive simulation (Part 1), `HashMap<(u64, usize), usize>` for memoization cache (Part 2), math-based digit counting using `log10()`
- **Complexity**:
  - Part 1 Naive: O(S^B) where S ≈ 2 (average split factor), B = 25 blinks → ~187K stones
  - Part 2 Memoized: O(U × B) where U = unique (stone, depth) states ≈ 130K entries
  - Math optimization: Eliminated heap allocations from string conversion/parsing
- **AoC Theme**: "Stone transformation simulation" with classic Part 2 optimization challenge (exponential growth → dynamic programming)

**🦀 Rust Conversion Highlights**:

- **From string-based digit counting** → **Math-based `(n as f64).log10().floor() + 1`** eliminating heap allocations
- **From string parsing for splits** → **Integer arithmetic** `stone / 10^(digits/2)` and `stone % 10^(digits/2)`
- **From nested loops** → **Recursive memoization** with HashMap cache for O(1) lookups
- **From manual state tracking** → **Structured cache key `(u64, usize)`** tuple for clear semantics
- **From ad-hoc optimization** → **Systematic TDD approach** with 18 comprehensive tests including performance analysis

**🏗️ Optimization Journey**:

- **[[../examples/day11_cache_analysis.md]]** - 300+ line comprehensive analysis documenting:
  - Performance comparison: Naive 25 blinks (20ms) vs memoized (711µs) = 28× speedup
  - Cache efficiency: 130K entries for 223 trillion stones (1 trillion× memory reduction)
  - Traced execution example with [0,1,2] for 5 blinks showing cache reuse patterns
  - Educational deep dive into when/why memoization is necessary
- **Python Comparison**: Discovered Python reference solution used `log10()` for digit counting, adopted math-based approach
- **Final Optimization**: Refactored from string-based (`to_string().len()`, string parsing) to integer arithmetic (log10, division/modulo)

**Real-World Complexity Handling**:

- **Exponential Growth**: Naive simulation at 40 blinks takes 10.6s with 233MB RAM, impossible for 75 blinks
- **Memoization Strategy**: Cache key `(stone_value, blinks_remaining)` captures complete state for reuse
- **Cache Reuse**: Same stone values at different depths share computation (e.g., stone 1 at depth 2 reused 84 times)
- **Math Optimization**: `log10()` eliminates String allocations, division/modulo replaces string slicing and parsing
- **Performance Validation**: Dedicated tests comparing naive vs memoized approaches with timing instrumentation

**Transformation Rules**:

1. **Rule 1**: Stone with value 0 → Stone with value 1
2. **Rule 2**: Stone with even number of digits → Split into two stones (left half, right half)
   - Example: 1234 → 12, 34
   - Implementation: `let divisor = 10^(digits/2); left = stone / divisor; right = stone % divisor`
3. **Rule 3**: Otherwise → Stone with value multiplied by 2024

**Educational Insights**:

- **When to Memoize**: Time/memory complexity analysis (naive hits time wall at ~40 blinks, memory wall at ~50)
- **Cache Design**: Choosing cache keys that capture complete state for optimal reuse
- **Algorithm Selection**: Naive simulation acceptable for small inputs (≤25 blinks), memoization essential for scale
- **Optimization Patterns**: Comparing string-based vs math-based approaches for performance improvements
- **Test-Driven Learning**: Performance tests validate optimization benefits with concrete measurements
- **Helper Functions**: `count_stones_with_trace()` and `count_with_cache_stats()` provide educational instrumentation (marked `#[allow(dead_code)]` for test-only usage)

**Python vs Rust Comparison**:

- **Algorithm Philosophy**:
  - **Python**: Math-based from start using `floor(log(stone, 10)) + 1` for digit counting
  - **Rust**: Evolved from string-based (initial) → math-based (optimized after Python comparison)
- **Implementation Style**:
  - **Python**: Recursive memoization with `@cache` decorator, math-based digit operations
  - **Rust**: Manual HashMap cache management, comprehensive test suite (18 tests), educational instrumentation
- **Performance**:
  - Both use memoization for Part 2 (75 blinks)
  - Rust's final math-based approach eliminates heap allocations from String operations
  - Both achieve optimal algorithmic complexity
- **Code Philosophy**:
  - **Python**: Pragmatic racing solution leveraging standard library (`@cache`, `math.log`)
  - **Rust**: Educational investment with performance analysis, traced examples, comprehensive documentation

**Results**:

- Sample: Part 1 = 55,312 (25 blinks)
- Puzzle: Part 1 = 187,738 (25 blinks), Part 2 = 223,767,210,249,237 (75 blinks)
- Cache stats: 129,787 unique (stone, depth) combinations for Part 2

**Test Coverage**: 18 tests including:

- Parsing validation (5 tests)
- Transformation rules (3 tests)
- Simulation correctness (4 tests)
- Memoization correctness (2 tests)
- Performance analysis (2 ignored tests: naive vs memoized comparison, cache stats)
- Integration tests (2 tests: Part 1 and Part 2 answers)

**Mission Integration Benefit**:

- Demonstrates optimization patterns applicable to other exponential growth problems
- Shows progression from naive → optimized approaches with measurable performance gains
- Educational documentation with traced examples makes complex memoization accessible
- Test infrastructure validates both correctness and performance characteristics

### Day 12: Garden Groups

**Title**: Garden Groups
**Part 1 Type**: Graph Algorithms + Mathematical
**Part 1 Description**: Calculate fencing cost for garden regions where cost = area × perimeter. Use flood fill to detect connected regions of same plant type, sum all region costs.
**Part 2 Type**: Graph Algorithms + Mathematical
**Part 2 Description**: Calculate bulk discount cost where cost = area × number_of_sides. Count sides using corner detection algorithm (sides = corners for polygons).
**Key Concepts**: Flood fill region detection, connected components, perimeter calculation, corner counting algorithm, Mission 6 Grid/FloodFill integration, geometric insight (sides = corners)

**🧩 Algorithm Analysis**:

- **Problem Pattern**: Region analysis escalation (perimeter-based cost → geometric side counting)
- **Data Structure**: Mission 6 `Grid<char>` for garden map, Mission 6 `FloodFill::analyze_region_4()` for connected components, `HashSet<Coord>` for visited tracking and region membership
- **Complexity**:
  - Part 1: O(W×H) single grid scan with flood fill, perimeter computed during BFS
  - Part 2: O(W×H) for flood fill + O(R×8) for corner checking (R = region size, 8 neighbor checks per cell)
  - Generic abstraction: Single `calculate_total_cost<F>()` function eliminates duplication
- **AoC Theme**: "Garden fencing optimization" with classic Part 2 geometric complexity (perimeter → side counting)

**🦀 Rust Conversion Highlights**:

- **From manual flood fill** → **Mission 6 `FloodFill::analyze_region_4()`** providing area, perimeter, and coordinates in one call
- **From separate region detection passes** → **Generic `calculate_total_cost<F>()` function** accepting closure for different cost calculations (area × perimeter vs area × sides)
- **From string/rotation side detection** → **Mathematical corner counting** using geometric theorem (sides = corners)
- **From manual bounds checking** → **Mission 6 `Grid` and `Coord`** with automatic safety guarantees
- **From ad-hoc validation** → **Defense in depth** with validation in both `parse_grid()` and `Grid::from_vec2d()`

**🏗️ Mission 6 Integration Benefits**:

- **Code Reduction**: Generic function eliminates ~30 lines of duplicate code between Part 1 and Part 2
- **Safety Improvement**: All grid operations bounds-checked automatically through Mission 6 APIs
- **Metadata for Free**: `FloodFill` provides area, perimeter, and coordinates—no manual computation needed
- **Cache-Friendly**: Row-major grid scanning (y outer, x inner loop) matches memory layout
- **Type Safety**: `Coord` type prevents x/y confusion, `Grid` indexing prevents buffer overflows

**Corner Counting Algorithm** (Part 2):

For each cell in a region, check 4 corner positions (top-left, top-right, bottom-left, bottom-right):

- **Outer corner (convex)**: Neither orthogonal neighbor exists (e.g., !North && !West)
- **Inner corner (concave)**: Both orthogonal neighbors exist but diagonal is missing (e.g., North && West && !NorthWest)

**Key Insight**: Number of sides equals number of corners for any polygon (including those with holes). This eliminates need for complex edge tracing or rotation algorithms.

**Real-World Complexity Handling**:

- **Nested Regions**: Inner corner detection handles regions with holes (e.g., B regions inside A region)
- **Irregular Shapes**: Corner algorithm works for any shape—rectangles, L-shapes, E-shapes, nested structures
- **Validation Redundancy**: `parse_grid()` validates dimensions even though `Grid::from_vec2d()` also checks—provides better error messages ("Row 2 has length 5, expected 6")
- **Code Reuse**: Generic function with closures demonstrates functional programming patterns for algorithm families

**Python vs Rust Comparison**:

- **Algorithm Philosophy**:
  - **Python Part 2**: String manipulation + grid rotation approach—creates padded grid, counts edge segments using string split trick, rotates 90° for vertical edges
  - **Rust Part 2**: Mathematical corner counting—leverages geometric theorem (sides = corners), HashSet lookups for neighbor checks
- **Implementation Style**:
  - **Python**: ~120 lines, manual DFS flood fill, string concatenation for edge detection, grid rotation with `zip(*grid[::-1])`
  - **Rust**: ~190 lines with 22 tests, Mission 6 flood fill, HashSet-based corner detection, generic cost function
- **Memory Efficiency**:
  - **Python**: Creates padded grid per region, allocates rotated grid, string operations
  - **Rust**: Single HashSet for region membership, no grid allocations, direct coordinate checks
- **Code Organization**:
  - **Python**: Pre-indexes coordinates by plant type, processes regions grouped by type
  - **Rust**: Single pass through grid, processes regions as discovered, generic abstraction for cost calculations
- **Side Detection**:
  - **Python**: Clever but memory-intensive—builds strings representing edge transitions, counts contiguous segments
  - **Rust**: Mathematically elegant—counts corners directly, leverages polygon property

**Educational Insights**:

- **Geometric Theorems**: Sides = corners is a fundamental polygon property applicable to many problems
- **Mission Composition**: Grid storage + FloodFill algorithm = complete region detection framework
- **Functional Patterns**: Generic functions with closures enable code reuse across similar algorithms
- **Corner Detection**: Distinguishing outer (convex) vs inner (concave) corners handles complex shapes including holes
- **Algorithm Selection**: Python's rotation trick is creative for small grids; Rust's corner counting scales better and requires less memory

**Results**:

- Sample: Part 1 = 140, Part 2 = 80 (simple 4×4 example)
- Sample: Part 1 = 1,930, Part 2 = 1,206 (large 10×10 example)
- Puzzle: Part 1 = 1,450,816, Part 2 = 865,662

**Test Coverage**: 22 tests including:

- Parsing validation (11 tests: empty input, single cell, jagged grids, trailing newlines, various characters)
- Part 1 integration (5 tests: simple example, OXOXO example, large example, edge cases)
- Part 2 integration (6 tests: simple, OXOXO, E-shape, nested regions, large example, edge cases)
- All tests validate expected area, perimeter, sides, and total costs from problem statement

**Mission Integration Benefit**:

- Demonstrates foundational library reuse for competitive programming (Grid + FloodFill)
- Shows functional programming patterns (generic functions with closures) for algorithm families
- Validates that Mission 6 components handle real AoC complexity (nested regions, irregular shapes)
- Proves value of defense-in-depth validation (better error messages through layered checks)

### Day 13: Claw Contraption

**Title**: Claw Contraption
**Part 1 Type**: Mathematical + Optimization
**Part 1 Description**: Solve claw machine puzzles to win prizes. Each machine has two buttons (A costs 3 tokens, B costs 1 token) that move the claw by specific X/Y amounts. Find the minimum tokens to reach the prize position, or determine if it's impossible.
**Part 2 Type**: Mathematical + Optimization
**Part 2 Description**: Same puzzle but add 10 trillion to each prize coordinate, requiring linear algebra solution (brute force impossible).
**Key Concepts**: Linear algebra, Cramer's rule, 2×2 system of equations, integer solution validation, modular arithmetic for divisibility checking

**🧩 Algorithm Analysis**:

- **Problem Pattern**: Mathematical optimization with escalation (small search space → enormous coordinates requiring analytical solution)
- **Data Structure**: `ClawMachine` struct with button displacements (dx, dy) and prize coordinates, no complex data structures needed—pure mathematics
- **Complexity**: O(1) per machine—Cramer's rule provides direct closed-form solution
- **AoC Theme**: "Claw machine optimization" with classic Part 2 scale-up that eliminates brute force (100×100 search → 10 trillion offset)

**🦀 Rust Conversion Highlights**:

- **From brute force search** → **Cramer's rule closed-form solution** eliminating nested loops entirely
- **From float division with `.is_integer()`** → **Integer modulo `% det != 0`** for exact divisibility checking (avoids floating-point precision issues)
- **From duplicated Part 1/Part 2 logic** → **Shared `total_tokens(&machines)` helper** accepting machine slice for code reuse
- **From string-based line splitting** → **Line-based grouping with `.lines()` iterator** handling both LF and CRLF line endings
- **From implicit error handling** → **Comprehensive `anyhow::Result`** with detailed error context for parsing failures

**Linear Algebra Solution**:

Given system of equations:

```
a * Ax + b * Bx = Px
a * Ay + b * By = Py
```

Cramer's rule solution:

- `det = Ax * By - Bx * Ay` (determinant)
- `a = (Px * By - Bx * Py) / det`
- `b = (Ax * Py - Px * Ay) / det`

Validity checks:

1. `det ≠ 0` (non-parallel lines, unique solution exists)
2. `a_num % det == 0 && b_num % det == 0` (integer solution)
3. `a ≥ 0 && b ≥ 0` (non-negative button presses)

**Key Insight**: Linear algebra guarantees at most one solution—no need to search for "minimum cost" among multiple solutions.

**Real-World Complexity Handling**:

- **Integer Precision**: Using `i64` throughout avoids floating-point precision loss for 10 trillion-scale coordinates
- **Determinant Zero Handling**: Parallel lines (no unique solution) gracefully handled as unsolvable
- **Cross-Platform Parsing**: `.lines()` iterator handles both Unix LF and Windows CRLF line endings automatically
- **Code Reuse**: `total_tokens()` function shared between Part 1 and Part 2, only prize offset differs

**Python vs Rust Comparison**:

- **Algorithm Approach**:
  - **Python**: Uses substitution method (solve for `a` in terms of `b`, substitute), float division with `.is_integer()` check
  - **Rust**: Uses Cramer's rule directly, integer modulo for divisibility check, no floating-point operations
- **Implementation Style**:
  - **Python**: ~45 lines, compact regex parsing, commented Z3 solver code for alternative approach
  - **Rust**: ~170 lines with 11 tests, structured parsing with error handling, type-safe `ClawMachine` struct
- **Integer Checking**:
  - **Python**: `float(a).is_integer()` relies on floating-point representation
  - **Rust**: `a_num % det != 0` uses exact integer arithmetic, no precision concerns
- **Code Reuse**:
  - **Python**: Separate `part1()` and `part2()` functions with duplicated solve logic
  - **Rust**: Shared `total_tokens()` function, Part 2 only adds offset before calling same solver
- **Error Handling**:
  - **Python**: Assumes valid input format, potential parse failures on malformed data
  - **Rust**: Comprehensive `Result<T>` throughout with detailed error context

**Educational Insights**:

- **Linear Algebra Application**: Real-world use of Cramer's rule for 2×2 systems—common in geometry, physics, and optimization problems
- **Scale-Driven Algorithm Selection**: Part 1 could use brute force (100×100 = 10K iterations), Part 2 forces O(1) analytical approach
- **Integer vs Float Precision**: Demonstrates why integer arithmetic is preferred when dealing with large coordinates (floating-point would lose precision at 10 trillion scale)
- **Uniqueness Guarantee**: Understanding that 2×2 linear systems have at most one solution (assuming non-parallel lines) eliminates unnecessary optimization logic

**Results**:

- Sample: Part 1 = 480 (machines 1 and 3 solvable, costs 280 + 200)
- Puzzle: Part 1 = 25,751, Part 2 = 108,528,956,728,655

**Test Coverage**: 11 tests including:

- Parsing validation (4 tests: button lines, prize lines, full machine, complete input)
- Individual machine solving (4 tests: solvable machine 1, unsolvable machine 2, solvable machine 3, unsolvable machine 4)
- Part 1 integration (1 test: example total 480)
- Part 2 behavior (2 tests: machine 2 becomes solvable, machine 4 becomes solvable with offset)

**Mission Integration Benefit**:

- Demonstrates that not all AoC problems benefit from mission libraries—Day 13 is pure mathematics with minimal data structure needs
- Shows value of choosing right tool: analytical solution over brute force when problem structure allows
- Validates that clean code organization (`ClawMachine` struct, `solve_machine()` function, `total_tokens()` helper) can make mathematical solutions readable and maintainable

---

### Day 14: Restroom Redoubt

**Title**: Restroom Redoubt
**Part 1 Type**: Simulation + Mathematical
**Part 1 Description**: Simulate 500 robots moving on a 101×103 grid with wraparound for 100 seconds. Each robot has position (px, py) and velocity (vx, vy). Calculate safety factor by multiplying quadrant counts (excluding robots on midlines).
**Part 2 Type**: Pattern Matching + Optimization
**Part 2 Description**: Find the timestep when robots form a Christmas tree pattern by detecting when all robots occupy unique positions (no overlaps).
**Key Concepts**: Modular arithmetic with `rem_euclid()`, quadrant classification, parallel processing with rayon, pattern detection via uniqueness, Mission 6 Grid integration

**🧩 Algorithm Analysis**:

- **Problem Pattern**: Simulation with escalation (simple position calculation → pattern detection across time)
- **Data Structure**: `Robot` struct with position/velocity, Mission 6 `Grid<usize>` for visualization, `HashSet<(i32, i32)>` for uniqueness checking
- **Complexity**: Part 1: O(N) robots × O(1) position calculation; Part 2: O(T×N) where T = timesteps until pattern found (~6516)
- **AoC Theme**: "Robot simulation" with classic Part 2 pattern search (calculate metric → find special configuration)

**🦀 Rust Conversion Highlights**:

- **From manual modulo** → **`rem_euclid()` for proper wraparound** (handles negative velocities correctly)
- **From sequential processing** → **Rayon parallelization** with `par_iter()` for large-scale performance testing
- **From implicit bounds** → **Mission 6 `Grid<T>` and `Coord`** for safe spatial operations
- **From ad-hoc pattern detection** → **Type-safe uniqueness check** with HashSet membership testing
- **From single solution** → **Educational parallel example** (day14_rayon_learning.rs) demonstrating 6 rayon concepts

**Rayon Parallelization Analysis**:

| **Dataset Size** | **Serial** | **Parallel** | **Speedup** | **Winner** |
|------------------|------------|--------------|-------------|------------|
| 5 items          | 400 ns     | 96.6 µs      | 0.004x      | Serial (241x faster) |
| 1,000 robots     | 37.8 µs    | 733 µs       | 0.05x       | Serial (19x faster) |
| 1,000,000 items  | 5.01 ms    | 1.41 ms      | **3.54x**   | **Parallel** |

**Key Finding**: Parallel overhead ~100µs; only beneficial when total work > 1ms. Demonstrates real-world performance trade-offs for data parallelism.

**Real-World Complexity Handling**:

- **Negative Velocity Wraparound**: `rem_euclid()` correctly handles negative velocities where standard `%` fails (e.g., `-3 % 101 = -3` vs `rem_euclid() = 98`)
- **Quadrant Classification**: Uses `Ordering::cmp()` for clean three-way comparisons, skips robots on midlines
- **Pattern Detection**: Christmas tree identified by uniqueness property—all 500 robots at distinct positions (insight from community after brute force visualization)
- **Performance Instrumentation**: Dedicated `examples/day14_rayon_learning.rs` teaches parallel patterns with timing measurements

**Python vs Rust Comparison**:

- **Algorithm Approach**:
  - **Python**: Direct computation avoiding simulation loop—`(x + 100 * (vx + width)) % width` calculates final position immediately for Part 1
  - **Rust**: Position calculation method `position_at(seconds)` enables both direct computation and iterative simulation
- **Part 2 Discovery**:
  - **Python**: Comments reveal process—printed all frames, noticed 10,403-frame pattern, found 101-frame sub-pattern with clustering, manual inspection found answer; key insight: "no robot stands in the same spot" when forming tree
  - **Rust**: Implemented `has_pattern()` checking for unique positions, validates Python's discovery with type-safe HashSet membership
- **Performance Optimization**:
  - **Python**: Efficient single-pass per timestep with early exit when duplicate position found (`valid = False; break`)
  - **Rust**: Both serial implementation matching Python's approach AND parallel variants for educational comparison
- **Code Philosophy**:
  - **Python**: ~55 lines pragmatic solution, comments document discovery process, relies on visual inspection + community insight
  - **Rust**: ~260 lines with comprehensive rayon tutorial (181 lines), transforms discovered pattern into programmatic validation, educational infrastructure for parallel processing

**Educational Value - Rayon Learning**:

Created `examples/day14_rayon_learning.rs` demonstrating:
1. **Basic `par_iter()`** - Just add 'par_' prefix for parallelization
2. **`reduce()` pattern** - Parallel aggregation (sum, product, tuple reduction)
3. **`into_par_iter()`** - Consuming parallel iterators
4. **`find_first()`** - Early exit searches with deterministic ordering
5. **Performance trade-offs** - When parallel helps vs hurts (overhead analysis)
6. **`par_extend()`** - Parallel collection building

**Mission 6 Integration Benefits**:

- `Grid<T>` for robot position visualization (`build_grid()` helper)
- `Coord` type prevents x/y coordinate confusion (never used)
- Type-safe spatial operations (prepared for future grid-based solutions)

**Results**: Part 1 = 217,132,650, Part 2 = 6,516 (timestep when Christmas tree forms)

**Test Coverage**: 5 comprehensive tests:
- Parsing validation (robot position/velocity extraction)
- Movement simulation (wraparound behavior with small 11×7 example)
- Example integration (12 robots, safety factor = 12)
- Serial vs parallel equivalence (functional correctness of rayon implementation)
- Large dataset performance (demonstrates 19x overhead for 1k robots)

---

### Day 15: Warehouse Woes

**Title**: Warehouse Woes
**Part 1 Type**: Simulation + Search/Traversal
**Part 1 Description**: Robot pushes single-width boxes (`O`) in a warehouse grid following movement commands (`^v<>`). Boxes form 1D chains that shift together when space is found. Calculate GPS sum (100 × row + col) of all final box positions.
**Part 2 Type**: Simulation + Graph Algorithms
**Part 2 Description**: Transform warehouse to double-width (walls `##`, boxes `[]`, robot stays 1-wide). Boxes form 2D overlapping structures requiring recursive collection to move multiple boxes simultaneously. Calculate GPS sum using left bracket positions.
**Key Concepts**: Grid simulation, box-pushing mechanics, recursive traversal, HashSet deduplication, check-then-execute pattern, Mission 6 Grid integration

**🧩 Algorithm Analysis**:

- **Problem Pattern**: Simulation with dimensional escalation (1D box chains → 2D overlapping box structures)
- **Data Structure**: Mission 6 `Grid<Tile>` for warehouse state, `HashSet<Coord>` for vertical box collection (Part 2), enum `Tile` for cell types, `Coord` for type-safe positions
- **Complexity**:
  - Part 1: O(M × C) where M = moves, C = chain length (scan to find empty space)
  - Part 2 Horizontal: Similar to Part 1, find far edge and recurse
  - Part 2 Vertical: O(M × B) where B = boxes in overlapping structure (recursive collection with memoization via HashSet)
- **AoC Theme**: "Robot simulation" with classic Part 2 complexity escalation (simple chains → complex overlapping structures)

**🦀 Rust Conversion Highlights**:

- **From Python's tuple positions** → **Type-safe `Coord` from Mission 6** (prevents x/y confusion)
- **From manual bounds checking** → **`grid.in_bounds()` and `grid.get()`/`grid.get_mut()` with `Option` returns**
- **From nested function calls** → **Explicit enum `Direction` with `delta()` method for directional offsets**
- **From Python sets for tracking** → **Rust `HashSet<Coord>` with `contains()` for duplicate prevention**
- **From dynamic typing** → **Strongly typed `enum Tile` with exhaustive pattern matching**

**Algorithm Deep Dive - Part 1 Optimization Journey**:

The solution evolved through three approaches:

1. **Initial Buggy Approach** (commit a09d182): Only moved first and last box in chain
   - Bug: `. O O O @>` became `O . . O @` (middle boxes vanished)
   - Issue: Cleared first position, placed box at empty, ignored intermediate boxes

2. **Over-Engineered Fix** (commit 4bb6c60): Tracked all boxes, moved each in reverse order
   - Correct but complex: Vec of all box positions, iterate reverse to avoid overwrites
   - Unnecessary work: Updated every box position (`O` → `O` for middle boxes)

3. **Elegant Solution** (current): Two-position update only
   - Key insight: Middle boxes don't change state (`O` → `O`), no update needed!
   - Algorithm: Scan for empty space → place box at empty → clear first position
   - **Check-Then-Execute Pattern**: Validate entire chain before any modifications (atomic success/failure)

**Algorithm Deep Dive - Part 2 Vertical Complexity**:

Part 2 introduces **2D overlapping boxes** that require sophisticated handling:

**Python's BFS Collection Approach** (`get_adjs_and_edges`):
```python
queue = [(y, x)]
while queue:
    y, x = queue.pop(0)  # BFS traversal
    if (y, x) in adjs:
        continue  # Skip visited
    adjs.add((y, x))
    
    # Check above/below for connected boxes
    if grid[ny][nx] == "[":
        queue.append((ny, nx))
        queue.append((ny, nx + 1))  # Both halves
```
- Uses BFS with set-based deduplication
- Collects ALL affected positions (edges and adjacent cells)
- Sorts by distance before moving (`sorted_coords`)

**Rust's Recursive Collection Approach** (`collect_boxes_vertical`):
```rust
fn collect_boxes_vertical(
    grid: &Grid<Tile>, 
    box_left: Coord,  // Track by left bracket position
    dir: Direction, 
    boxes: &mut HashSet<Coord>
) -> bool {
    if boxes.contains(&box_left) {
        return true;  // Already visited
    }
    boxes.insert(box_left);
    
    // Recurse on both halves' neighbors
    // Returns false if any path hits a wall
}
```
- Uses recursion with HashSet deduplication
- Returns `bool` for validation (wall detection)
- Collects only box positions (not all cells)

**Why HashSet is Critical**:
```
  []         Box A
 [][]        Box B, Box C
  @^
```
When robot pushes up from Box B:
- Check above-left: finds `[` of Box A → recurse
- Check above-right: finds `]` of Box A → recurse **again**
- Without HashSet: Box A added twice, moved twice! 🐛
- With HashSet: Second encounter skipped, Box A moves once ✅

**The Two-Phase Execute Pattern**:

Both languages use **collect-then-execute**:

**Phase 1 - Collection** (No grid modifications):
- Scan all affected boxes recursively/iteratively
- Validate no walls block the move
- Build complete set of boxes to move

**Phase 2 - Execution** (Atomic modification):
- Sort boxes by distance (furthest first)
- Move each box to new position
- Clear old positions
- **Critical**: Distance sorting prevents overwriting unmoved boxes

**Key Architectural Differences**:

| **Aspect** | **Python** | **Rust** |
|------------|------------|----------|
| **Traversal** | BFS with `list.pop(0)` (O(n)) | Recursion with call stack |
| **Collection** | All cells (`adjs` set) + edge markers | Box positions only (left brackets) |
| **Sorting** | `sorted()` with key function | `sort_by_key()` with `Reverse()` |
| **Grid Updates** | Direct list indexing | `grid.get_mut()` with `Option` |
| **Coordinate Type** | Tuples `(y, x)` | Type-safe `Coord{x, y}` |
| **Validation** | Count blocked edges | Boolean return from recursion |

**Mission 6 Integration Benefits**:

- **`Grid<Tile>`**: Type-safe 2D storage with bounds checking
- **`Coord`**: Prevents x/y confusion (never wrote `grid[x][y]` by mistake)
- **`in_bounds()`**: Eliminates manual boundary checks
- **`get()`/`get_mut()`**: Returns `Option` for safe access patterns
- **`from_vec2d()`**: Clean grid construction from parsed input

**Results**: Part 1 = 1,465,152, Part 2 = 1,511,259

**Test Coverage**: 3 comprehensive tests:
- Small example (8×8 grid, simple movements, GPS = 2,028)
- Large example Part 1 (10×10 grid, complex chains, GPS = 10,092)
- Large example Part 2 (widened grid, vertical overlaps, GPS = 9,021)

**Code Organization**:
- **Part 1**: `try_move()` handles 1D box chains with endpoint updates
- **Part 2**: `try_move_wide()` dispatches to horizontal (recursive scan) vs vertical (collect-then-execute)
- **Separation of Concerns**: `try_move*()` handles box logic, `simulate_robot*()` handles robot tile updates
- **Helper Functions**: `widen_grid()` transforms Part 1 → Part 2, `calculate_gps_sum()` shared scoring

**Educational Insights**:

1. **Simplification Through Understanding**: The Part 1 optimization journey shows how deep problem understanding leads to simpler code (tracking all boxes → updating two positions)

2. **Check-Then-Execute Pattern**: Validate entire operation before any modifications ensures atomic success/failure (no partial states, no rollback needed)

3. **Deduplication is Critical**: Part 2's overlapping boxes prove why HashSet tracking prevents double-processing in graph-like traversals

4. **Distance-Based Ordering**: Moving furthest boxes first prevents overwriting unmoved boxes (common pattern in grid simulations)

5. **Mission Library Composition**: Grid + Coord infrastructure eliminates entire bug classes (bounds errors, coordinate confusion)

**Python vs Rust Philosophy**:

- **Python**: ~130 lines, BFS with list-based queue, tuple coordinates, dynamic typing, direct grid indexing
- **Rust**: ~520 lines (including comprehensive tests), recursive collection, type-safe Coord/Tile, explicit error handling, Mission 6 integration
- **Both Correct**: Python optimizes for brevity and rapid development; Rust optimizes for safety, educational clarity, and integration with mission libraries
- **Key Difference**: Python's pragmatic approach vs Rust's educational infrastructure with comprehensive type safety

---

### Day 16: Reindeer Maze

**Title**: Reindeer Maze  
**Part 1 Type**: Graph Algorithms + Search  
**Part 1 Description**: Maze pathfinding where moving forward costs 1 point and rotating 90° costs 1000 points. Find minimum cost path from start (facing East) to end.  
**Part 2 Type**: Graph Algorithms + Optimization  
**Part 2 Description**: Count all distinct tiles that are part of at least one optimal path through the maze (backtracking from optimal end states).  (Determined from running the python solution there are 8 best routes)
**Key Concepts**: Dijkstra's algorithm with compound state, BFS with cost tracking, state space design (position + direction), priority queues, backtracking for path reconstruction, Mission 6 Grid integration

**🧩 Algorithm Analysis**:

- **Problem Pattern**: Pathfinding with escalation (shortest path → all optimal paths)
- **Data Structure**: 
  - **Rust**: Mission 6 `Grid<char>` for maze, `BinaryHeap` for Dijkstra priority queue, `HashMap<State, usize>` for distances, `State` struct containing `(position, direction)`, enum `Direction` with delta/rotation methods
  - **Python**: 2D list for grid, plain list as BFS queue with `pop(0)`, dict for `visited[((y, x), direction)]`, tuple-based state tracking
- **Complexity**: 
  - **Rust Part 1**: O(V log V + E) Dijkstra with min-heap, V = positions × 4 directions, E = 3 transitions per state (forward + 2 rotations)
  - **Rust Part 2**: Additional O(V + E) backtracking from optimal end states
  - **Python**: O(V + E) BFS with full history tracking per route, larger memory footprint but simpler implementation
- **AoC Theme**: "Maze navigation with rotation costs" featuring classic Part 2 enumeration (find one path → find all paths)

**🦀 Rust Conversion Highlights**:

- **From tuple state `((y, x), direction)`** → **Type-safe `struct State { pos: (usize, usize), dir: Direction }` with Hash + Eq**
- **From list-based BFS queue** → **`BinaryHeap<Node>` for O(log n) push/pop with custom `Ord` for min-heap behavior**
- **From direction indices 0-3** → **Explicit `enum Direction { North, East, South, West }` with `delta()`, `rotate_cw()`, `rotate_ccw()` methods**
- **From direct grid indexing** → **Mission 6 `Grid<char>` with `grid[pos]` and `in_bounds(pos.into())` for safe access**
- **From full history tracking** → **Distance map only for Part 1, backtracking for Part 2 path reconstruction (memory efficient)**

**Key Algorithmic Differences**:

**State Space Design**:
- **Critical Insight**: State must be `(position, direction)` not just position!
- Why: Same position facing different directions has different future paths and costs
- Example: Reaching `(5,5)` from North requires different rotations than from East

**Python's BFS Approach with History**:
```python
queue = [(start, [start], 0, 0)]  # position, history, score, direction
while queue:
    (y, x), history, curr_score, curr_dir = queue.pop(0)
    
    # Key: Store full path history with each state
    # Enables immediate Part 2 solution by filtering routes by min_score
    if (y, x) == end:
        routes.append((history, curr_score))
```
- Tracks complete path with every queue node
- Simple Part 2: filter routes by minimum score, union all positions
- Memory cost: O(P × L) where P = paths explored, L = average path length
- Time cost: O(n) per `pop(0)` for list-based queue

**Rust's Dijkstra Approach with Backtracking**:
```rust
// Part 1: Track only best distance to each state
let mut distances = HashMap<State, usize>::new();
heap.push(Node { cost: 0, state: start_state });

// Part 2: Backtrack from optimal end states
let end_states = all_directions
    .filter(|state| distances.get(state) == Some(&min_cost));

for state in end_states {
    // Reconstruct paths by checking predecessors
    if prev_cost + edge_cost == current_cost {
        queue.push(prev_state);  // Valid predecessor
    }
}
```
- Stores only best cost to each state (no path history)
- Separate backtracking phase for Part 2
- Memory cost: O(V) where V = position × direction states
- Time cost: O(log n) per heap operation via `BinaryHeap`

**Visited State Pruning**:

**Python's Subtle Optimization**:
```python
# For Part 1 only (faster):
if visited[((y, x), curr_dir)] <= curr_score:
    continue  # Skip if we've seen better or equal

# For both parts (slower but necessary):
if visited[((y, x), curr_dir)] < curr_score:
    continue  # Skip only if strictly better exists
```
- Uses `<` instead of `<=` to allow multiple paths with same cost
- Critical for Part 2: same-cost paths must be explored
- Python author documented this explicitly in comments!

**Rust's Dijkstra Pruning**:
```rust
if let Some(&best) = distances.get(&state) {
    if cost > best {
        continue;  // Skip if we've found strictly better
    }
}
// Updates distance if cost == best (implicit allow-same-cost)
distances.insert(state, cost);
```
- Allows equal-cost paths implicitly through HashMap updates
- Dijkstra naturally handles this via min-heap ordering

**Transition Generation**:

**Python** (4 directions, skip opposite):
```python
dirs = [(0, 1), (-1, 0), (0, -1), (1, 0)]  # E, N, W, S
for _dir, (dy, dx) in enumerate(dirs):
    if (curr_dir + 2) % 4 == _dir:
        continue  # Skip 180° turn (impossible)
    
    if _dir == curr_dir:
        # Move forward (+1)
    else:
        # Rotate (+1000)
```
- Rotation implicit: try different direction from same position
- Clever opposite check: `(curr_dir + 2) % 4`

**Rust** (explicit transitions):
```rust
fn get_neighbors(state: State) -> Vec<(State, usize)> {
    vec![
        (move_forward(state), 1),          // Same direction
        (rotate_cw(state), 1000),          // 90° clockwise
        (rotate_ccw(state), 1000),         // 90° counterclockwise
    ]
}
```
- Explicit 3 transitions per state
- Type-safe `Direction` enum with rotation methods
- No 180° turn needed (only CW/CCW from any position)

**Mission 6 Integration**:

**Grid Operations**:
- `Grid::new(rows, cols, '#')` - Create maze with wall default
- `grid[(r, c)] = ch` - Index by tuple (Mission 6 `impl Index`)
- `grid.in_bounds(pos.into())` - Safe bounds checking with `Coord` conversion
- `grid[pos]` - Direct character access (bounds checked via Index)

**Type Safety Benefits**:
- `Coord` type prevents x/y confusion (never wrote `grid[x][y]` by mistake)
- Direction enum eliminates magic numbers (0=East, 1=North, etc.)
- State struct ensures position and direction always paired

**Results**: Part 1 = 92,432, Part 2 = 458

**Test Coverage**: 4 comprehensive tests:
- Example 1 Part 1 (small 15×15 maze, score = 7,036)
- Example 2 Part 1 (large 17×17 maze, score = 11,048)
- Example 1 Part 2 (45 tiles on optimal paths)
- Example 2 Part 2 (64 tiles on optimal paths)

**Code Organization**:
- **`State` struct**: Encapsulates `(position, direction)` with Hash/Eq for HashMap keys
- **`Direction` enum**: Type-safe directions with `delta()`, `rotate_cw()`, `rotate_ccw()` methods
- **`Node` struct**: Priority queue wrapper with custom `Ord` for min-heap (reverse ordering)
- **`parse_maze()`**: Grid construction with start/end detection
- **`get_neighbors()`**: Transition generation (forward or rotate)
- **`dijkstra()`**: Single-source shortest path with all-directions tracking
- **`part1()`**: Extract minimum cost to end
- **`part2()`**: Backtrack from optimal end states to count tiles

**Educational Insights**:

1. **Compound State Space**: Rotation costs require state = `(position, direction)`, not just position. This expands graph size but enables correct cost modeling.

2. **Algorithm Choice Matters**: 
   - **Python's BFS with history**: Simple, unified approach for both parts, higher memory cost
   - **Rust's Dijkstra with backtracking**: Optimal complexity, two-phase approach, memory efficient

3. **Same-Cost Path Handling**: Part 2 requires exploring all paths with minimum cost, not just the first one found. Python explicitly documents this with `<` vs `<=` comparison; Rust handles implicitly via HashMap updates.

4. **Backtracking Pattern**: When Part 1 asks "find best" and Part 2 asks "find all best", backtracking from optimal solutions is often more efficient than tracking all paths upfront.

5. **Mission Integration Value**: Mission 6 Grid + Coord eliminates coordinate confusion, bounds errors, and provides clean indexing patterns for maze problems.

**Python vs Rust Comparison**:

| **Aspect** | **Python** | **Rust** |
|------------|------------|----------|
| **Algorithm** | BFS with full history | Dijkstra with backtracking |
| **Queue** | `list.pop(0)` O(n) | `BinaryHeap` O(log n) |
| **State** | Tuple `((y, x), dir)` | `struct State { pos, dir }` |
| **History** | Tracked per route | Reconstructed in Part 2 |
| **Memory** | O(P × L) paths × length | O(V) states only |
| **Visited Check** | `< curr_score` explicitly | `> best` implicitly |
| **Transitions** | Loop over 4 directions | 3 explicit transitions |
| **Grid Access** | `grid[ny][nx]` direct | `grid[pos]` with bounds check |
| **LOC** | ~68 lines unified solution | ~355 lines with 4 tests |

**Both Correct**: Python optimizes for midnight racing with simple BFS and full history tracking (~68 lines). Rust optimizes for educational clarity with Dijkstra, type-safe state management, Mission 6 integration, and comprehensive test coverage (~355 lines). Python's pragmatic approach gets correct answer efficiently; Rust's structured approach provides deeper algorithmic insights and reusable patterns.

**Key Learning**: When rotation costs matter, state space must include direction. When Part 2 asks for "all optimal", ensure your algorithm explores equal-cost paths, not just first-found paths.

### Day 17: Chronospatial Computer

**Title**: Chronospatial Computer  
**Part 1 Type**: Simulation + Mathematical  
**Part 1 Description**: Implement a 3-bit virtual machine with 3 registers (A, B, C) and 8 opcodes, execute the program and return comma-separated output.  
**Part 2 Type**: Search + Optimization  
**Part 2 Description**: Find the lowest value of register A that makes the program output itself (quine challenge).  
**Key Concepts**: Virtual machine implementation, instruction pointer management, opcode dispatch, combo operands, recursive backtracking, mathematical optimization, bit manipulation, quine programming

**🧩 Algorithm Analysis**:

- **Problem Pattern**: VM simulation with escalation (run program → reverse engineer program to output itself)
- **Data Structure**: 
  - **Rust**: `Vec<u8>` for program, `i64` for registers, recursive function with backtracking for Part 2
  - **Python**: `list` for program/registers, iterative search with smart jumps for Part 2
- **Complexity**: 
  - **Part 1**: O(P) where P = program length (linear execution until halt)
  - **Rust Part 2**: O(8^N) recursive search with early pruning, N = program length in digits
  - **Python Part 2**: O(K × P) where K = iterations needed, P = program execution cost per iteration
- **AoC Theme**: "Virtual machine challenge" with classic quine problem (execute code → code generates itself)

**Virtual Machine Implementation**:

**8 Opcodes** (3-bit computer):
| Opcode | Mnemonic | Operation | Description |
|--------|----------|-----------|-------------|
| **0** | `adv` | `A >>= combo` | Divide A by 2^combo (arithmetic right shift) |
| **1** | `bxl` | `B ^= literal` | XOR B with literal operand |
| **2** | `bst` | `B = combo & 7` | B = combo mod 8 (keep lowest 3 bits) |
| **3** | `jnz` | Jump if A≠0 | Set ip to operand if A is non-zero |
| **4** | `bxc` | `B ^= C` | XOR B with C (operand ignored) |
| **5** | `out` | Output `combo & 7` | Print combo mod 8 to output |
| **6** | `bdv` | `B = A >> combo` | Like adv but stores in B |
| **7** | `cdv` | `C = A >> combo` | Like adv but stores in C |

**Combo Operands**:
- `0-3`: Literal values 0, 1, 2, 3
- `4`: Value of register A
- `5`: Value of register B
- `6`: Value of register C
- `7`: Reserved (invalid)

**Key VM Characteristics**:
- **Instruction size**: 2 bytes (opcode + operand)
- **Instruction pointer**: Advances by 2 each step (except `jnz` jumps)
- **Halt condition**: IP exceeds program length
- **Output format**: Comma-separated single digits (0-7)

**🦀 Rust Conversion Highlights**:

- **From Python method** → **Standalone function with explicit parameters** (`run_program(a, b, c, program)`)
- **From nested match/case** → **Rust `match` with helper closure** for combo operand resolution
- **From integer division** → **Right shift operators** (`A >>= combo` more idiomatic for powers of 2)
- **From list append** → **`Vec::push()`** for output accumulation
- **From iterative search** → **Recursive backtracking** with early pruning for Part 2

**Part 2 Algorithm Comparison**:

**Shared Insight**: Both solutions exploit the fact that each program iteration divides A by 8 (opcode 0 with shift), meaning **each output digit is determined by 3 bits of A**.

**Python's Iterative Jump Strategy**:
```python
# Calculate min/max possible values mathematically
A_max = sum(7 * 8**i for i in range(len(program)))     # Max possible
A_min = sum(7 * 8**i for i in range(len(program) - 1)) # Min possible

# Start from minimum and jump smartly
A = A_min + 1
while True:
    result = run_program([A, 0, 0], program)
    
    if result == program:
        return A
    
    # Find first mismatched position from end
    for i in range(len(result) - 1, -1, -1):
        if result[i] != program[i]:
            # Jump by 8^i (skip all values that won't fix this digit)
            A += 8**i
            break
```

**Strategy**: 
- Forward search from calculated minimum
- When output mismatches at position i, add 8^i to skip invalid values
- Leverages mathematical structure: position i corresponds to bits [3i, 3i+1, 3i+2]
- Iterative with smart increments (not brute force)

**Rust's Recursive Backtracking**:
```rust
fn find_a(program, target_pos, current_a, reg_b, reg_c) -> Option<i64> {
    if target_pos > program.len() {
        // Verify complete program generates itself
        let output = execute_program(current_a, reg_b, reg_c, program);
        return if output == program { Some(current_a) } else { None };
    }
    
    // Try all 3-bit values (0-7) for this position
    for bits in 0..8 {
        let test_a = (current_a << 3) | bits;  // Add 3 bits
        let output = execute_program(test_a, reg_b, reg_c, program);
        
        // Check if last target_pos digits match
        if output.len() >= target_pos 
           && output[output.len() - target_pos..] == program[program.len() - target_pos..] {
            // Recurse to build next 3 bits
            if let Some(result) = find_a(program, target_pos + 1, test_a, reg_b, reg_c) {
                return Some(result);
            }
        }
    }
    
    None  // Backtrack
}

// Start: build from last digit backwards
find_a(&program, 1, 0, reg_b, reg_c)
```

**Strategy**:
- Backward construction from last digit to first
- Build A as `current_a << 3 | bits` (shift left 3, add new bits)
- Early pruning: if partial output doesn't match, don't recurse deeper
- Depth-first search with backtracking
- Guaranteed to find smallest A (tries 0-7 in order)

**Algorithm Philosophy Comparison**:

| **Aspect** | **Python Iterative** | **Rust Recursive** |
|------------|----------------------|-------------------|
| **Direction** | Forward (start → end) | Backward (end → start) |
| **State** | Single A value with jumps | Stack of partial As |
| **Pruning** | Skip 8^i values on mismatch | Backtrack if partial match fails |
| **Memory** | O(1) | O(N) call stack depth |
| **Search Space** | Guided jumps | Exhaustive with pruning |
| **Conceptual Model** | "Climb towards answer" | "Build answer digit by digit" |

**Both Correct**: Python leverages mathematical insight for efficient forward search (~30 lines), Rust uses classic recursive backtracking for systematic construction (~80 lines for Part 2). Python optimizes for competitive speed; Rust demonstrates algorithmic pattern applicable to constraint-solving problems.

**🏗️ Code Organization**:

**Rust Structure**:
- `parse_input()`: Extract registers and program from text with `anyhow::Result` error handling
- `execute_program()`: VM implementation with closure for combo operand resolution
- `solve_part1()`: Direct program execution and output formatting
- `solve_part2()`: Recursive `find_a()` helper for backtracking search
- **Test Coverage**: 7 comprehensive tests validating parsing, VM opcodes, Part 1 example, and specific VM behaviors

**Python Structure**:
- `run_program()`: VM as method with match/case for opcodes
- `part1()`: Parse, execute, format output
- `part2()`: Mathematical bounds calculation + iterative jump search
- Minimal structure optimized for competitive programming

**Educational Insights**:

1. **3-Bit Computer Pattern**: All operations constrained to 3-bit values (0-7), output via `& 7`, modulo via `& 7`, demonstrating bitwise efficiency over `% 8`.

2. **Combo Operand Design**: Dual-mode operands (literal vs register) handled elegantly via closure/function mapping operand → value.

3. **Quine Algorithm**: Part 2 requires understanding program structure—each loop iteration consumes 3 bits via right shift, enabling digit-by-digit construction.

4. **Right Shift vs Division**: `A >>= n` equivalent to `A /= 2^n` but more efficient and clearer for power-of-2 operations (VM arithmetic).

5. **Search Strategy Trade-offs**: Python's iterative jump approach requires mathematical insight but is memory-efficient; Rust's recursive backtracking is more general-purpose but uses call stack. Both valid, different optimization goals.

6. **Early Pruning**: Both solutions avoid brute force by eliminating entire ranges—Python via 8^i jumps, Rust via partial match validation before recursion.

7. **Bit Manipulation**: Building A via `(current << 3) | bits` constructs number 3 bits at a time, leveraging binary structure of the problem.

**Rust-Specific Patterns**:

- **Closure for combo operands**: Captures registers by value, provides clean operand → value mapping
- **Pattern matching on opcode**: Rust match exhaustiveness ensures all opcodes handled
- **Right shift operations**: `>>=` and `>>` idiomatic for power-of-2 divisions
- **Bit operations**: `& 7` for mod 8, `<< 3` for multiply by 8, `| bits` for bit setting
- **Recursive Option return**: `Option<i64>` cleanly represents success/failure without exceptions
- **Vec<u8> for output**: Efficient push for building output sequence

**Test Coverage**:
- `test_parse_input()`: Validates input parsing with registers and program
- `test_part1_example()`: Verifies complete Part 1 execution (729 → "4,6,3,5,6,3,5,2,1,0")
- `test_execute_c_set()`: Tests `bst` opcode (register C → register B)
- `test_execute_output()`: Tests `out` opcode with multiple outputs
- `test_execute_adv_out()`: Tests compound behavior (adv + out + jnz loop)
- `test_execute_bxl()`: Tests `bxl` XOR operation (B XOR literal)
- `test_execute_bxc()`: Tests `bxc` XOR operation (B XOR C)

**Results**: Part 1 = [actual output], Part 2 = [lowest A value]

**Key Learning**: Virtual machine problems benefit from clean opcode dispatch (match/case), combo operand resolution via functions/closures, and understanding bit-level program structure for quine challenges. Part 2 search strategies vary (forward jumps vs backward construction) but both exploit the 3-bits-per-output pattern for efficiency.
---

### Day 18: RAM Run

**Title**: RAM Run  
**Part 1 Type**: Graph Algorithms + Simulation  
**Part 1 Description**: Find shortest path through 71×71 memory grid from (0,0) to (70,70) after 1024 bytes have corrupted specific coordinates.  
**Part 2 Type**: Search + Optimization  
**Part 2 Description**: Binary search to find first byte coordinate that blocks all paths from start to exit (Part 1: 282, Part 2: 64,29).  
**Key Concepts**: Grid pathfinding, BFS shortest path, binary search, coordinate parsing, Mission 6 Grid integration, Mission 8 Graph trait implementation, library composition

**🧩 Algorithm Analysis**:

- **Problem Pattern**: Pathfinding with escalation (find shortest path after fixed corruption → find first blocking byte via binary search)
- **Data Structure**:
  - **Rust**: Mission 6 `Grid<bool>` for corruption tracking, `Coord` for type-safe positions, Mission 8 `Graph` trait on custom `MemorySpace`, `shortest_path()` algorithm
  - **Python**: `networkx.Graph` with explicit node/edge construction OR 2D list with custom BFS fallback
- **Complexity**:
  - **Part 1**: O(W×H) BFS pathfinding where W=H=71 (explores up to 5,041 positions)
  - **Part 2**: O(log N × W×H) binary search over N=3,450 bytes, each testing path existence
- **AoC Theme**: "Memory corruption pathfinding" with classic binary search optimization (simulate fixed corruption → find blocking threshold)

**Mission Integration Excellence**:

This problem perfectly demonstrates the **integrator philosophy**—building solutions by composing validated foundational libraries rather than reimplementing algorithms:

**Mission 6 Benefits**:
- `Grid<bool>`: Automatic bounds checking, no manual array indexing
- `Coord`: Type-safe coordinates preventing x/y confusion
- `in_bounds()`: Built-in validation, no manual range checks

**Mission 8 Benefits**:
- `Graph` trait: Standard interface enabling generic algorithms
- `shortest_path()`: BFS implementation with parent tracking for path reconstruction
- Returns `Result<Vec<Coord>, GraphError>`: Type-safe error handling

**Implementation Focus**: Zero lines of custom BFS code, zero lines of manual grid indexing—entire solution focused on problem-specific logic (corruption simulation, binary search strategy).

**🦀 Rust Conversion Highlights**:

**From Python networkx Library**:
```python
# Python builds explicit graph structure
self.graph = ntx.Graph()
for y in range(self.size):
    for x in range(self.size):
        if self.grid[y][x] == ".":
            self.graph.add_node((x, y))
            for dx, dy in [(0,1), (1,0), (0,-1), (-1,0)]:
                nx, ny = x + dx, y + dy
                if 0 <= nx < self.size and 0 <= ny < self.size and self.grid[ny][nx] == ".":
                    self.graph.add_edge((x, y), (nx, ny))

# Find path
path = ntx.shortest_path(self.graph, start, end)
```

**To Rust Mission Composition**:
```rust
// Rust implements Graph trait directly on MemorySpace
impl Graph for MemorySpace {
    type Node = Coord;
    
    fn neighbors(&self, node: Self::Node) -> Vec<Self::Node> {
        // 4-directional movement with safe checking
        // Returns only valid, non-corrupted neighbors
    }
    
    fn contains(&self, node: Self::Node) -> bool {
        node.x < self.width && node.y < self.height
    }
    
    fn nodes(&self) -> Vec<Self::Node> {
        // All grid coordinates
    }
}

// Find path - zero-copy, no graph construction needed
match shortest_path(&memory, start, goal) {
    Ok(path) => path.len() - 1,  // Steps = nodes - 1
    Err(_) => bail!("No path found"),
}
```

**Key Differences**:
- **Python**: Builds explicit graph structure with `add_node()`/`add_edge()`, consumes memory proportional to edges
- **Rust**: Implements trait methods, zero-copy adapter pattern, computes neighbors on-demand
- **Python**: Uses networkx library for graph operations OR custom BFS with `list.pop(0)` (O(n) per pop)
- **Rust**: Mission 8 provides `shortest_path()` using efficient `VecDeque` (O(1) per pop)

**Part 2 Binary Search - Algorithm Evolution**:

**Python's Learning Journey** (documented in code comments):
```python
# Original approach: test every corrupted coord (slow)
for cx, cy in self.corrupted[self.corrupted_length:]:
    self.add_corrupted(cx, cy)
    steps = self.get_shortest_path_steps()
    if steps == -1:
        return f"{cx},{cy}"  # Found blocker!

# Learned from Reddit: binary search over byte count
```

**Educational Insight**: Python developer documented their optimization journey—started with sequential testing (~3,450 iterations), learned binary search pattern from community, applied it to reduce to ~12 iterations. This shows competitive programming learning process.

**Rust Implementation** (binary search from start):
```rust
let mut left = 0;
let mut right = bytes.len();  // 3,450 bytes

while left < right {
    let mid = (left + right) / 2;
    
    // Simulate first 'mid' bytes falling
    let mut memory = MemorySpace::new(width, height);
    for coord in bytes.iter().take(mid) {
        memory.corrupt(*coord);
    }
    
    // Test if path still exists
    if shortest_path(&memory, start, goal).is_ok() {
        left = mid + 1;  // Path exists, try more bytes
    } else {
        right = mid;     // Path blocked, try fewer bytes
    }
}

// left-1 is last byte where path existed
// left is first blocking byte
let blocking_byte = bytes[left - 1];
```

**Algorithm Philosophy**:
- Python: Documented learning/optimization process in production code (shows growth)
- Rust: Applied binary search pattern immediately (demonstrates algorithmic thinking)
- Both correct, Python's comments provide educational value about problem-solving journey

**Library Strategy Comparison**:

| **Aspect** | **Python (networkx)** | **Rust (Missions)** |
|------------|----------------------|---------------------|
| **Graph Building** | Explicit `add_node()`/`add_edge()` | Implicit trait implementation |
| **Memory Model** | Materialized graph structure | On-demand neighbor computation |
| **Dynamic Updates** | `remove_node()` for Part 2 (original) | Rebuild grid (binary search) |
| **Algorithm Access** | `shortest_path()` library function | `shortest_path()` mission function |
| **Trade-offs** | Batteries-included, more memory | Trait-based, zero-copy, extensible |

**Python Alternative - Custom BFS**:
```python
# Fallback when networkx disabled
queue = [(start, 0)]  # (position, length)
seen = set()
while queue:
    pos, length = queue.pop(0)  # O(n) operation!
    if pos == end:
        return length
    if pos in seen:
        continue
    seen.add(pos)
    # ... add neighbors
```

**Performance Note**: Python's `list.pop(0)` is O(n) because lists are arrays requiring element shifting. Rust's `VecDeque.pop_front()` is O(1) using ring buffer. For competitive programming at midnight, Python's simplicity wins; for education, understanding data structure impact matters.

**🏗️ Code Organization**:

**Rust Structure**:
- `struct MemorySpace`: Wraps `Grid<bool>` with corruption tracking
- `impl Graph for MemorySpace`: Adapter pattern enabling mission algorithms
- `parse_bytes()`: Coordinate parsing with `anyhow::Result` error context
- `part1()`: Size detection (7×7 vs 71×71), simulation, pathfinding
- `part2()`: Binary search with efficient grid recreation each iteration
- **Test Coverage**: 2 comprehensive tests with example data (7×7, 12 bytes → 22 steps; blocker at 6,1)

**Python Structure**:
- Class-based with mutable state (`self.grid`, `self.graph`, etc.)
- `use_networkx` flag for algorithm switching (demonstration vs production)
- Size auto-detection via input length (clever: 25 coords = example, else real)
- Part 2 shows evolution: commented-out original O(N) + active binary search O(log N)

**Educational Insights**:

1. **Integrator Philosophy**: Rust solution is **pure composition**—Mission 6 + Mission 8 + problem logic. No algorithm implementation, only trait methods and binary search loop. This is the goal: focus on problem, leverage validated components.

2. **Zero-Copy Adapters**: Python builds explicit graph consuming memory; Rust implements trait methods computing neighbors on-demand. Trait pattern enables efficient abstractions.

3. **Binary Search Efficiency**: Reduces 3,450 iterations to ~12 iterations (log₂(3450) ≈ 11.75). Critical for problems with large search spaces.

4. **Type Safety Benefits**: `Coord` type prevents tuple confusion ((x,y) vs (y,x)), `Grid<bool>` eliminates bounds errors, `Result<Vec<Coord>, GraphError>` makes errors explicit.

5. **Library Composition vs Implementation**: Python's networkx is feature-complete batteries-included library; Rust's missions are foundational types requiring trait implementation but enabling custom optimizations and learning.

6. **Learning Documentation**: Python's commented-out code shows problem-solving evolution (original slow approach → community-learned optimization). Valuable for understanding thought process.

7. **Algorithm Selection**: Both solutions ultimately use BFS for pathfinding + binary search for optimization. Core algorithms are universal; library integration patterns differ by language ecosystem.

**Performance Comparison**:

Both solutions achieve correct results efficiently:
- **Part 1**: Both O(W×H) BFS, Rust's `VecDeque` vs Python's `list.pop(0)` or networkx internals
- **Part 2**: Both O(log N × W×H) binary search, Python originally tested all coordinates (O(N × W×H)) before Reddit learning

**Real-World Complexity**:
- Grid: 71×71 = 5,041 positions
- Byte coordinates: 3,450 total
- Part 1 simulates: 1,024 bytes
- Part 2 binary search: ~12 iterations finding first of 2,426 remaining bytes that blocks path

**Key Takeaway**: This problem demonstrates **mission system maturity**—when foundational libraries (Grid, Graph, pathfinding) are production-ready, competitive programming becomes **problem composition** rather than algorithm implementation. Python uses batteries-included networkx; Rust builds lightweight trait adapters. Both approaches valid; Rust's teaches architectural patterns while solving problems.

---

### Day 19: Linen Layout (Detailed Analysis)

**Title**: Linen Layout  
**Part 1 Type**: Pattern Matching + Dynamic Programming  
**Part 1 Description**: Determine which towel designs can be constructed from available pattern pieces by checking if design strings can be built from prefix combinations of available patterns (Result: 360 possible designs)  
**Part 2 Type**: Combinatorial Optimization + Dynamic Programming  
**Part 2 Description**: Count all possible ways to arrange patterns to create each design—same patterns in different orders count as distinct arrangements (Result: 577,474,410,989,846 total arrangements)  
**Key Concepts**: Memoized recursion, substring prefix matching, combinatorial counting, dynamic programming state caching, string slice operations, exponential search pruning

**🧩 Algorithm Analysis**:

- **Problem Pattern**: Pattern composition with escalation (boolean existence check → exhaustive arrangement counting)
- **Data Structure**:
  - **Rust**: `Vec<&str>` for zero-copy pattern storage, `HashMap<&str, bool>` for Part 1 possibility cache, `HashMap<&str, u64>` for Part 2 arrangement count cache, string slices as keys for efficient substring memoization
  - **Python**: List for patterns, `@cache` decorator with position-based recursion, `defaultdict(int)` for manual count memoization
- **Complexity**:
  - **Base Recursion**: O(P × L) per design where P = pattern count, L = design length (tries all patterns at each position)
  - **With Memoization**: O(P × L) amortized where each unique substring computed once then cached
  - **Without Memoization**: Exponential O(P^L) due to overlapping subproblems (infeasible for long designs)
- **AoC Theme**: "Towel pattern matching" with classic Part 2 combinatorial explosion (can make? → how many ways?)

**Algorithm Comparison: Position-Based vs String-Based Recursion**:

**Python Approach** (Position Index):
```python
@cache
def matching_towel(pos):
    if pos == len(design):  # Base case: reached end
        return True
    for towel in towels:
        next_pos = pos + len(towel)
        if next_pos <= len(design) and design[pos:next_pos] == towel:
            if matching_towel(next_pos):
                return True
    return False
```
- **Cache Key**: Integer position in design string
- **Strategy**: Track current index, recursively try patterns from that position
- **Memory**: Cache size = O(L) where L = design length (one entry per position)

**Rust Approach** (Substring Slices):
```rust
fn can_make_recursive<'a>(
    patterns: &[&str],
    remaining: &'a str,
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    if remaining.is_empty() { return true; }  // Base case
    if let Some(&result) = memo.get(remaining) { return result; }
    
    for pattern in patterns {
        if let Some(rest) = remaining.strip_prefix(pattern) {
            if can_make_recursive(patterns, rest, memo) {
                memo.insert(remaining, true);
                return true;
            }
        }
    }
    memo.insert(remaining, false);
    false
}
```
- **Cache Key**: String slice (`&str`) representing remaining substring
- **Strategy**: Track remaining suffix, use `strip_prefix()` for pattern matching
- **Memory**: Cache size = O(U) where U = unique substrings encountered (potentially L² worst case, typically much smaller)

**Trade-offs**:
- **Python**: Simpler cache key (integer), O(L) guaranteed cache size, substring creation on each check (`design[pos:next_pos]`)
- **Rust**: Zero-copy string slices (no allocation), elegant `strip_prefix()` API, larger potential cache but still efficient in practice
- **Performance**: Both achieve similar O(P × L) amortized complexity with memoization; Rust avoids substring allocations through borrowing

**Part 2 Extension Pattern**:

Both languages extend identical recursion structure from boolean (can make?) to counting (how many ways?):

**Boolean → Counting Transformation**:
```rust
// Part 1: Return true if ANY pattern works
for pattern in patterns {
    if let Some(rest) = remaining.strip_prefix(pattern) {
        if can_make_recursive(patterns, rest, memo) {
            return true;  // Early exit on first success
        }
    }
}

// Part 2: SUM arrangements for ALL patterns
let mut total_ways = 0;
for pattern in patterns {
    if let Some(rest) = remaining.strip_prefix(pattern) {
        total_ways += count_ways_recursive(patterns, rest, memo);  // Accumulate all paths
    }
}
return total_ways;
```

This transformation from "find one solution" to "count all solutions" is a classic dynamic programming pattern—the recursive structure remains identical, only the reduction operation changes (boolean OR → integer SUM).

**🦀 Rust Conversion Highlights**:

**From Python decorator caching**:
```python
@cache
def matching_towel(pos):
    # Automatic memoization by functools
```

**To Rust manual HashMap**:
```rust
fn can_make_recursive<'a>(
    patterns: &[&str],
    remaining: &'a str,
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    if let Some(&result) = memo.get(remaining) {
        return result;  // Cache hit
    }
    // ... compute result ...
    memo.insert(remaining, result);
    result
}
```

**Lifetime Management**: Rust's `<'a>` lifetime parameter ensures string slice keys in HashMap remain valid—cache borrows from input string without ownership transfer.

**From substring equality checks**:
```python
if design[pos:next_pos] == towel:  # String slice allocation
```

**To zero-copy prefix matching**:
```rust
if let Some(rest) = remaining.strip_prefix(pattern) {
    // No allocation: 'rest' borrows from 'remaining'
}
```

**Pattern Matching Excellence**: `strip_prefix()` returns `Option<&str>` enabling both validation and extraction in single operation—if pattern matches, get remaining suffix; if not, None.

**Flexible Input Parsing**:
```rust
let (patterns_str, designs_str) = if parts.len() == 1 {
    // Handle single-newline format (actual puzzle input)
    if let Some(first_newline) = input.find('\n') {
        (&input[..first_newline], &input[first_newline + 1..])
    } else {
        (input, "")
    }
} else {
    // Handle double-newline format (example format)
    (parts[0], parts[1])
};
```
**Robust Parsing**: Handles both `\n\n` (example format) and `\n` (puzzle format) automatically by testing split result length.

**Python vs Rust Philosophy**:

| Aspect | Python | Rust |
|--------|--------|------|
| **Memoization** | `@cache` decorator automatic | Manual `HashMap` management |
| **Cache Key** | Position integer (simple) | String slice (zero-copy) |
| **Recursion Style** | Index-based tracking | Suffix-based slicing |
| **Pattern Matching** | Substring equality | `strip_prefix()` API |
| **Memory Strategy** | O(L) cache guaranteed | O(U) cache (unique substrings) |
| **Code Length** | ~50 lines total | ~200 lines with tests |
| **Sorting** | Patterns sorted by length | No sorting needed |
| **Print Debugging** | Extensive `print()` statements | Clean implementation |

**Educational Value**:

**Python Strengths**:
- Decorator caching eliminates boilerplate
- Print debugging shows algorithm execution (`print(pos)`, `print(towel)`)
- Position-based approach intuitive for beginners
- Sorting patterns by length (potential optimization, though not always beneficial)

**Rust Strengths**:
- Lifetime system prevents cache invalidation bugs
- Zero-copy string operations through borrowing
- Type-safe pattern matching with Option unwrapping
- Comprehensive test suite (5 tests) validates correctness
- Explicit memoization control for educational transparency

**Performance Characteristics**:

Both solutions achieve similar performance with memoization:
- **Example (8 designs)**: Sub-millisecond
- **Puzzle (400 designs, 400+ patterns)**: Sub-second
- **Memoization Critical**: Without cache, Part 2 infeasible (exponential blowup)

**Key Algorithmic Insight**: This problem demonstrates why memoization is essential for recursive problems with overlapping subproblems. Without caching, the same suffix would be recomputed exponentially many times as different pattern prefixes lead to identical suffixes.

**Test Coverage**:

Rust implementation includes comprehensive tests:
1. **Parse validation**: 8 patterns, 8 designs extracted correctly
2. **Individual design checking**: 6 possible, 2 impossible validated
3. **Part 1 integration**: Total count matches example (6)
4. **Arrangement counting**: Individual designs match expected combinations
5. **Part 2 integration**: Total arrangements match example (16)

**Code Organization**:

- **Parsing**: `parse_input()` with flexible format handling
- **Part 1 Logic**: `can_make_design()` wrapper → `can_make_recursive()` with boolean cache
- **Part 2 Logic**: `count_ways_to_make()` wrapper → `count_ways_recursive()` with count cache
- **Clean Separation**: Public API (`solve_part1`, `solve_part2`) delegates to internal recursive helpers

**Real-World Complexity Handling**:

- **Empty String Base Case**: Both solutions correctly return 1 for empty design (zero patterns needed = one way to arrange nothing)
- **Pattern Order Independence**: Rust doesn't sort patterns (unlike Python); both approaches work regardless of pattern order
- **Large Number Handling**: Uses `u64` for arrangement counts (up to 18 quintillion) safely handling Part 2's 577 trillion result

**Mission Integration Opportunities**:

While this problem doesn't require Mission libraries, it could benefit from:
- **Mission 5 HashMap**: Already using `std::collections::HashMap` which Mission 5 demonstrates
- **String Processing Patterns**: Common AoC pattern for parsing and substring operations
- **Memoization Patterns**: Reusable pattern for future dynamic programming problems

**Results**: Part 1 = 360 possible designs, Part 2 = 577,474,410,989,846 total arrangements

---

## Problem Type Distribution (Available Days)

| Category | Part 1 Count | Part 2 Count |
|----------|--------------|--------------|
| Advanced Pattern Matching | 0 | 0 |
| Brute Force | 1 | 1 |
| Cellular Automaton | 0 | 0 |
| Combinatorial Optimization | 1 | 1 |
| Conditional Logic | 1 | 2 |
| Cryptographic | 0 | 0 |
| Data Structures | 2 | 2 |
| Encoding | 0 | 0 |
| Graph Algorithms | 5 | 5 |
| Greedy Algorithms | 0 | 0 |
| Mathematical | 9 | 6 |
| Number Theory | 0 | 0 |
| Optimization | 1 | 10 |
| Parsing | 0 | 0 |
| Pattern Matching | 3 | 3 |
| Real-time Analysis | 0 | 0 |
| Search | 0 | 2 |
| Search/Traversal | 5 | 5 |
| Simulation | 7 | 4 |
| String Processing | 2 | 0 |

## Implementation Notes

### Common Patterns Observed

1. **Input Parsing**: Structured text parsing with error handling (Day 1: whitespace-separated integers, Day 2: space-separated levels per line, Day 3: regex pattern extraction from corrupted memory)
2. **Two-Part Escalation**: Part 2 transforms Part 1's approach (Day 1: distance → similarity, Day 2: strict safety → tolerance mechanism, Day 3: stateless parsing → stateful conditional processing)
3. **List Processing**: Sort and pair operations (Day 1: smallest-to-smallest pairing), sequence validation (Day 2: monotonicity checking)
4. **Frequency Analysis**: Count occurrences for scoring (Day 1: HashMap frequency counting)
5. **Functional Pipelines**: Iterator chains for data transformation (Day 1: parsing → unzipping → processing, Day 2: windows → validation → counting)
6. **Safety Validation**: Range and monotonicity checking (Day 2: difference bounds + direction consistency)
7. **Brute Force Optimization**: Try all possibilities when constraints relax (Day 2: Problem Dampener trying each removal)
8. **Pattern Matching**: Regular expressions for instruction parsing (Day 3: `mul(X,Y)`, `do()`, `don't()` extraction from noisy input), geometric pattern recognition (Day 4: X-shaped MAS patterns)
9. **State Machine Logic**: Conditional instruction processing (Day 3: enable/disable state tracking across instruction sequence)
10. **Grid Processing**: 2D array parsing and traversal (Day 4: character grid with bounds validation, directional search algorithms)
11. **Directional Search**: Multi-directional pattern detection (Day 4: 8-direction word search using coordinate arithmetic and direction vectors)
12. **Dependency Graphs**: Topological ordering and cycle detection (Day 5: page ordering rules with Mission 7 Graph representation and Mission 8 algorithms)
13. **Adaptive Algorithms**: Algorithm selection based on data characteristics (Day 5: Kahn's algorithm for acyclic graphs, rule-based sorting for cyclic graphs)
14. **Foundational Library Integration**: Mission utilities demonstrate architectural benefits (Day 4: Mission 6 - 43% code reduction, Day 5: Mission 7+8 - 40% code reduction with cycle detection)
15. **Guard/Agent Simulation**: State-based entity movement with obstacle avoidance (Day 6: guard patrol with turn-right collision handling, position tracking with bounds checking)
16. **Loop Detection**: Infinite cycle identification in simulations (Day 6: HashSet-based state tracking for O(1) loop detection vs naive position-only approaches)
17. **Obstacle Placement Optimization**: Brute force testing of environmental modifications (Day 6: testing obstacle at each path position to create infinite loops, P×W×H complexity pattern)
18. **Combination Generation**: Systematic enumeration of operator combinations using base conversion mathematics (Day 7: 2^(N-1) and 3^(N-1) combinations for expression evaluation)
19. **Custom Precedence Evaluation**: Non-standard operator precedence with left-to-right evaluation (Day 7: `81 + 40 * 27 = (81+40)*27 = 3267` vs standard `81 + (40*27)`)
20. **TDD Methodology**: Professional test-driven development for competitive programming (Day 7: 5-phase implementation with 32 comprehensive tests covering edge cases)
21. **Expression Parsing and Evaluation**: Mathematical expression processing with custom operators (Day 7: addition, multiplication, concatenation with overflow handling)
22. **Exponential Search Algorithms**: Brute force approaches with exponential complexity but manageable input constraints (Day 7: O(3^(N-1)) acceptable for typical AoC input sizes)
23. **Vector Normalization**: GCD-based primitive direction extraction for complete lattice coverage (Day 8: prevents skipped harmonic points)
24. **Primitive Ray Casting**: Bidirectional traversal using minimal step vectors for geometric saturation (Day 8: harmonic resonance lines)
25. **Geometric Resonance Saturation**: Transition from discrete extrapolation to full line filling via normalized direction (Day 8: resonance propagation)
26. **Disk Compaction & Gap Management**: Dense disk decoding, dual-pointer compaction, and mission-backed metadata tracking for relocations (Day 9)
27. **Height-Constrained Graph Traversal**: Graph navigation with edge constraints based on vertex properties (Day 10: topographic hiking trails with +1 height increment requirement)
28. **Reachability vs Path Enumeration**: Contrasting BFS for unique destination counting with DFS for complete path enumeration (Day 10: Part 1 counts reachable summits, Part 2 counts all distinct routes)
29. **Exponential Growth Simulation**: Stone/entity multiplication with transformation rules leading to exponential count increases (Day 11: stone transformations with 2× average growth per blink)
30. **Dynamic Programming with Memoization**: HashMap-based caching of recursive subproblems using composite state keys (Day 11: `(stone_value, blinks_remaining) → count` for O(U×B) vs naive O(S^B))
31. **Math-Based Optimization**: Replacing string operations with integer arithmetic for performance (Day 11: `log10()` for digit counting, division/modulo for splitting vs string conversion/parsing)
32. **Cache Efficiency Analysis**: Measuring memoization effectiveness through cache size vs theoretical state space (Day 11: 130K cache entries for 223 trillion stones = 1 trillion× reduction)
33. **Algorithmic Complexity Thresholds**: Identifying breakpoints where naive approaches become infeasible and optimization is required (Day 11: naive feasible ≤25 blinks, memoization required for 75)
34. **Connected Component Analysis**: Region detection using flood fill for connected cells sharing properties (Day 12: garden plots grouped by plant type with 4-connectivity)
35. **Corner Counting for Polygon Sides**: Geometric algorithm leveraging the property that sides = corners for any polygon, detecting both outer (convex) and inner (concave) corners (Day 12: eliminates need for edge tracing or rotation)
36. **Generic Higher-Order Functions**: Code reuse through generic functions accepting closures for algorithm family variations (Day 12: single `calculate_total_cost<F>()` for different cost formulas eliminates duplication)
37. **Region Metadata Extraction**: Single-pass flood fill providing multiple metrics (area, perimeter, coordinates) avoiding redundant traversals (Day 12: Mission 6 FloodFill returns complete region info)
38. **Linear Algebra for Constraint Solving**: Cramer's rule for 2×2 systems providing O(1) closed-form solutions instead of brute force search (Day 13: claw machine button press combinations)
39. **Scale-Driven Algorithm Selection**: Problem constraints that force analytical solutions over brute force approaches (Day 13: Part 2 adds 10 trillion offset making search impossible)
40. **Integer Precision for Large Coordinates**: Using integer modulo for divisibility checking instead of floating-point `.is_integer()` to avoid precision loss at large scales (Day 13: 10 trillion-scale coordinates)
41. **Modular Arithmetic with Wraparound**: Using `rem_euclid()` for proper modulo with negative numbers in grid simulation (Day 14: robot movement with negative velocities wrapping correctly)
42. **Parallel Performance Analysis**: Empirical measurement of parallel overhead vs benefit, identifying when parallelization helps (Day 14: 3.54x speedup at 1M items, 19x overhead at 1k items)
43. **Pattern Detection via Uniqueness**: Finding special configurations by testing constraint satisfaction (Day 14: Christmas tree when all robots occupy unique positions)
44. **Data Parallelism with Rayon**: Converting sequential iterators to parallel with minimal code changes for CPU-bound workloads (Day 14: educational example with 6 core rayon patterns)
45. **Tuple-Based Parallel Reduction**: Mapping items to tuple components and reducing by adding corresponding fields for multiple counters (Day 14: quadrant counting with `(q1, q2, q3, q4)` tuple reduction)
46. **Check-Then-Execute Pattern**: Validate entire operation atomically before any modifications, eliminating need for rollback or partial state handling (Day 15: scan for empty space before moving any boxes)
47. **Box-Chain Simulation**: 1D chain pushing where intermediate elements don't need updates if their state remains unchanged (Day 15 Part 1: only update endpoints of box chain)
48. **Recursive Collection with Deduplication**: Using HashSet to prevent double-processing in overlapping graph-like structures (Day 15 Part 2: vertical box pushing where boxes share edges)
49. **Distance-Based Ordering for Cascading Updates**: Sort entities by distance from change source to prevent overwriting unmoved elements (Day 15: move furthest boxes first in chain/structure)
50. **Dimensional Escalation in Simulations**: Increasing complexity from 1D chains to 2D overlapping structures requiring fundamentally different algorithms (Day 15: simple chains → recursive collection)
51. **Compound State Space Design**: Expanding state representation beyond position to include direction/orientation when rotation costs exist (Day 16: state = `(position, direction)` not just position, enables correct pathfinding with rotation penalties)
52. **Dijkstra with Priority Queues**: Using `BinaryHeap` for optimal shortest path with O(log n) operations, custom `Ord` implementation for min-heap behavior (Day 16: Dijkstra's algorithm for weighted pathfinding)
53. **Backtracking for Path Reconstruction**: Two-phase approach where Phase 1 finds optimal distance, Phase 2 reconstructs all paths achieving that distance (Day 16: Part 1 finds minimum cost, Part 2 backtracks from optimal end states)
54. **Same-Cost Path Exploration**: Algorithm design allowing multiple paths with equal cost to be explored, not just first-found path (Day 16: critical for Part 2 "all optimal paths" enumeration, Python uses `<` not `<=` for visited check)
55. **Transition Generation Patterns**: Explicit enumeration of valid state transitions with associated costs (Day 16: 3 transitions per state—move forward +1, rotate CW +1000, rotate CCW +1000)
56. **Virtual Machine Implementation**: Building interpreters with instruction pointer, opcode dispatch, and register management (Day 17: 3-bit computer with 8 opcodes, combo operand resolution, halt detection)
57. **Combo Operand Design**: Dual-mode operands mapping to literals or register values via function/closure (Day 17: operands 0-3 are literals, 4-6 are registers A/B/C)
58. **Quine Problem Solving**: Reverse engineering programs to output themselves by exploiting structural patterns (Day 17: each output digit determined by 3 bits of register A)
59. **Recursive Backtracking vs Iterative Search**: Comparing depth-first backward construction with forward jump-based search for constraint satisfaction (Day 17: Rust builds A digit-by-digit recursively, Python uses mathematical jumps of 8^i)
60. **Bit-Level Program Analysis**: Understanding how bit manipulation drives program output for optimization (Day 17: A >>= 3 each loop, enabling 3-bit-per-digit construction strategy)
61. **Binary Search Over Simulation State**: Efficient threshold detection by binary searching over parameter space and testing simulation outcome at each point (Day 18: O(log N) search over byte count, testing pathfinding at each midpoint to find first blocking coordinate)
62. **Zero-Copy Graph Adapters**: Implementing Graph trait directly on existing data structures for algorithm composition without memory overhead (Day 18: `impl Graph for MemorySpace` enables Mission 8 shortest_path() with zero-copy neighbor computation)
63. **On-Demand Neighbor Generation**: Computing graph edges dynamically via trait methods rather than materializing explicit graph structure (Day 18: `neighbors()` checks 4 directions with corruption validation, no pre-built adjacency list)
64. **Integrator Philosophy Application**: Building complete solutions through pure library composition without algorithm implementation (Day 18: Mission 6 Grid + Mission 8 Graph trait + problem logic = pathfinding solution with zero custom BFS code)
65. **Algorithm Reuse via Traits**: Enabling generic algorithms through trait implementation on custom types (Day 18: custom MemorySpace implements Graph trait → gains access to Mission 8 pathfinding algorithms)
66. **String Slice Memoization**: Using borrowed string slices as HashMap keys for zero-copy caching in recursive string processing (Day 19: `HashMap<&str, bool>` and `HashMap<&str, u64>` with lifetime parameters)
67. **Substring-Based Recursion**: Pattern matching via suffix recursion using `strip_prefix()` for prefix validation and remainder extraction in single operation (Day 19: check pattern, get remaining substring, zero allocations)
68. **Boolean-to-Counting DP Transformation**: Extending existence-check algorithms to exhaustive counting by changing reduction operator (Day 19: Part 1 early-exit OR → Part 2 accumulative SUM with identical recursion structure)
69. **Manual HashMap Memoization**: Explicit cache management with lifetime tracking for educational transparency vs decorator-based automatic caching (Day 19: demonstrates cache hit/miss mechanics, entry insertion timing, borrowing constraints)
70. **Overlapping Subproblem Identification**: Recognizing when memoization transforms exponential O(P^L) complexity to linear O(P×L) through cache reuse (Day 19: same suffix reached via different prefix paths)
71. **Lifetime-Parametric Recursion**: Using `<'a>` lifetime parameters ensuring borrowed cache keys remain valid throughout recursion (Day 19: cache borrows from input string without ownership, prevents dangling references)
72. **Flexible Format Parsing**: Robust input handling detecting format variations automatically via split result testing (Day 19: handles both `\n\n` example format and `\n` puzzle format without hardcoding)

### Rust-Specific Considerations

- **Day 1**: Excellent introduction to functional error handling with `Result<T, E>`, iterator combinators (`zip`, `fold`, `sum`), and pattern matching for safe parsing. Demonstrates HashMap construction with functional approach vs Python's Counter.
- **Day 2**: Showcases iterator windows for sliding comparisons, early-return imperative validation (vs Python's `all()`/`any()` functional style), and `to_vec()` + `remove()` for element removal simulation. Demonstrates performance-focused approach with manual state tracking vs functional boolean aggregation.
- **Day 3**: Highlights regex integration with `regex` crate, type-safe instruction parsing using `enum` with pattern matching, comprehensive error context with `anyhow::Context`, and efficient single-pass state machine implementation. Shows Rust's strength in pattern validation and stateful processing with zero-cost abstractions.
- **Day 4**: Demonstrates 2D grid processing with comprehensive bounds checking, mathematical approach to directional search using coordinate vectors, and zero-allocation character matching vs Python's string concatenation approach. Showcases Rust's compile-time safety for array indexing and elegant pattern decomposition for geometric shapes. **Mission 6 Alternative**: Illustrates how foundational libraries can dramatically simplify competitive programming solutions—280-line manual implementation reduced to 160 lines with automatic safety guarantees, proving that good architecture improves both productivity and correctness.
- **Day 5**: Exemplifies sophisticated graph algorithm integration using **actual Mission 7 + Mission 8 APIs** (not mocks). Demonstrates bidirectional HashMap mapping for type-safe node management, adaptive algorithm selection based on Mission 8 cycle detection, and production-quality error handling with graceful degradation. Shows how foundational libraries enable focus on problem logic rather than low-level graph implementation. **Mission Integration**: Proves concrete benefits—40% code reduction, automatic cycle detection, proven algorithms, and real-world complexity handling for graphs with cycles (49 nodes, 1,176 edges).
- **Day 6**: Demonstrates comprehensive Mission 6 + Mission 5 integration for simulation-based problems. Showcases type-safe coordinate operations with automatic bounds checking, enum-based direction management with rotation methods, and efficient state-based loop detection using HashSet collections. **Mission Integration**: Eliminates entire bug classes through type safety—coordinate arithmetic errors, direction confusion, bounds violations—while maintaining optimal algorithmic complexity. Shows how foundational libraries make complex simulations both safer and more maintainable (8 comprehensive unit tests, zero clippy warnings).
- **Day 7**: Exemplifies professional TDD methodology applied to competitive programming with comprehensive test-driven development (32 tests). Demonstrates type-safe `Operator` enum with pattern matching for mathematical operations, explicit left-to-right evaluation engine vs standard precedence, and systematic combination generation using base conversion mathematics. **TDD Excellence**: 5-phase implementation approach (data structures → evaluation → combinations → validation → integration) with complete edge case coverage. Shows `anyhow::Result` throughout for production-quality error handling, zero-allocation evaluation for performance, and structured approach to brute force algorithms. **Architecture Investment**: 530+ lines with comprehensive test suite vs Python's 40-line pragmatic solution—demonstrates Rust's strength in creating maintainable, verifiable, and educationally valuable competitive programming solutions.
- **Day 8**: Demonstrates geometric correctness via primitive vector normalization (`gcd`) ensuring complete harmonic line saturation. Highlights mission abstraction benefits (safe `Grid` operations, hashed `Coord`) and contrasts completeness-oriented Rust approach with Python's brevity that risks skipped intermediate points. Establishes reusable pattern for line-of-sight / visibility algorithms with mathematical rigor applied to competitive programming.
- **Day 9**: Highlights dense-disk simulations built atop Mission 5 collections—`Dictionary` keeps file metadata synchronized during block and whole-file moves while gap segments stay sorted for O(1) lookup. Dedicated checksum helper doubles as regression oracle, and `examples/day09_visualization.rs` streams every intermediate compaction state so the algorithm can be inspected visually. Emphasizes how mission libraries plus instrumentation (examples/ tooling) turn a puzzle solver into a teaching asset.
- **Day 10**: Exemplifies mission composition at its finest—Mission 6 `Grid<Option<u32>>` + `Coord` type + `neighbors_4()` iterator combined with Mission 8 `Graph` trait + generic `bfs()` algorithm. Demonstrates complete refactoring journey: initial manual implementation (d082003) proves algorithm correctness, Mission 6 refactoring (c6b2283) reduces code by ~10 lines while eliminating entire bug classes (coordinate confusion, bounds errors). **Key Learning**: `Coord` type prevents tuple x/y swapping, `grid.in_bounds()` eliminates manual checks, `neighbors_4()` returns iterator (not Vec) for efficiency, Graph trait enables generic algorithms. Shows BFS for Part 1 reachability vs custom DFS for Part 2 path counting—demonstrates when to use vs extend mission libraries. **Python Comparison**: Python's 50-line pragmatic solution uses O(n) `list.pop(0)` and tuple positions; Rust's 160-line structured solution uses O(1) VecDeque, type-safe Coord, and comprehensive validation. Both correct, different optimization goals (midnight racing vs production learning). **V-Cycle Validation**: Tests prove functional equivalence (12/12 pass), answers match (512/1045), zero clippy warnings after iterator refinement.
- **Day 11**: Showcases optimization journey from string-based to math-based approaches. Demonstrates dynamic programming with `HashMap<(u64, usize), usize>` memoization cache using composite state keys for O(1) lookups. Highlights performance analysis through dedicated tests comparing naive O(S^B) vs memoized O(U×B) approaches. **Math Optimization**: Evolved from `to_string().len()` to `log10()` for digit counting, from string parsing to integer arithmetic (`division/modulo`) for splitting stones—eliminates heap allocations while maintaining correctness. **Educational Infrastructure**: `count_stones_with_trace()` and `count_with_cache_stats()` test helpers (marked `#[allow(dead_code)]`) provide instrumentation for understanding memoization mechanics. **Test-Driven Analysis**: 18 comprehensive tests including performance validation (naive vs memoized timing), cache efficiency measurement (130K entries for 223T stones), and traced execution examples. **Python Comparison**: Python used math-based approach from start with `@cache` decorator; Rust's manual cache management provides deeper understanding of memoization mechanics. Shows when optimization transitions from optional (Part 1 ≤25 blinks) to essential (Part 2 = 75 blinks), with concrete performance metrics validating the necessity.
- **Day 12**: Exemplifies Mission 6 integration for region-based problems with flood fill for connected component detection. Demonstrates generic higher-order functions with closures—`calculate_total_cost<F>()` eliminates ~30 lines of duplication while accepting different cost formulas (area × perimeter vs area × sides). **Corner Counting Algorithm**: Mathematical approach leveraging geometric theorem (sides = corners for polygons), distinguishing outer corners (!N && !W) from inner corners (N && W && !NW), handling complex shapes including nested regions with holes. **Mission Integration**: `FloodFill::analyze_region_4()` provides area, perimeter, and coordinates in single call; `Grid<char>` + `Coord` eliminate manual bounds checking; row-major scanning (y outer, x inner) matches memory layout for cache efficiency. **Defense in Depth**: Validation in both `parse_grid()` and `Grid::from_vec2d()` provides better error messages despite redundancy. **Test Coverage**: 22 comprehensive tests covering parsing edge cases (11), Part 1 integration (5), Part 2 geometric shapes (6). **Python Comparison**: Python's creative string manipulation + grid rotation approach (creates padded grid, counts edge segments, rotates 90° for vertical edges) vs Rust's mathematical corner counting (HashSet lookups, no grid allocations, leverages polygon property). Shows functional programming patterns (closures for algorithm families) and foundational library composition (Grid + FloodFill = complete region detection framework).
- **Day 13**: Demonstrates pure mathematical problem solving with minimal data structure complexity. Showcases Cramer's rule implementation for 2×2 linear systems with integer arithmetic avoiding floating-point precision issues. **Algorithm Choice**: Direct closed-form solution instead of brute force—O(1) vs O(N²) complexity; Part 2's 10 trillion offset makes brute force literally impossible. **Integer Precision**: Uses `i64` throughout and modulo checks (`a_num % det != 0`) instead of floating-point `.is_integer()`, critical for large-scale coordinates. **Cross-Platform Parsing**: `.lines()` iterator handles both LF and CRLF line endings automatically, fixing initial parsing failure on Windows. **Code Organization**: Clean separation with `ClawMachine` struct, dedicated parse functions with error context, and shared `total_tokens()` helper between parts. **Test Coverage**: 11 comprehensive tests covering parsing, individual machine solving (solvable and unsolvable cases), and Part 2 behavior changes. **Key Learning**: Not all AoC problems need complex data structures or mission libraries—recognizing when mathematical analysis is the right tool shows problem-solving maturity.
- **Day 14**: Showcases **rayon data parallelism** for robot simulation with comprehensive educational infrastructure. Demonstrates `rem_euclid()` for proper modulo with negative numbers (critical for wraparound), quadrant classification with `Ordering::cmp()`, and pattern detection via uniqueness constraint (HashSet checking for collision-free positioning). **Rayon Excellence**: Created `examples/day14_rayon_learning.rs` (181 lines) teaching 6 core patterns—`par_iter()`, `reduce()`, `into_par_iter()`, `find_first()`, overhead analysis, `par_extend()`—with empirical performance data showing 3.54x speedup at 1M items vs 19x overhead at 1k items (parallel overhead ~100µs). **Tuple Reduction Pattern**: Demonstrates elegant multi-counter aggregation by mapping items to tuples `(q1, q2, q3, q4)` and reducing component-wise for parallel quadrant counting. **Python Comparison**: Python's pragmatic ~55-line solution with direct position calculation and manual pattern discovery (printing frames, visual inspection) vs Rust's ~260-line educational approach with programmatic validation, comprehensive parallel variants, and performance instrumentation. **Mission 6 Integration**: Prepared `Grid<T>` for visualization and spatial operations, demonstrating readiness for grid-based algorithms. **Key Insight**: Python documented discovery process in comments ("printed all frames, noticed patterns, found answer via community insight"), Rust formalized the insight into type-safe validation logic with parallel alternatives. **Test Coverage**: 5 tests validating parsing, wraparound simulation, serial/parallel equivalence, and large-scale performance characteristics.
- **Day 15**: Demonstrates **Mission 6 Grid mastery** for complex box-pushing simulations with sophisticated algorithm evolution. Showcases **check-then-execute pattern** where entire operation is validated before any modifications (atomic success/failure, no rollback needed). **Optimization Journey**: Evolved from buggy endpoint-only updates → over-engineered full-chain tracking → elegant two-position updates (recognizing middle boxes don't change state). **Part 2 Complexity**: Handles 2D overlapping box structures requiring recursive collection with HashSet deduplication to prevent double-processing. **Key Algorithms**: Part 1 uses simple scan-for-empty pattern (O(C) chain length); Part 2 horizontal uses recursive edge-finding; Part 2 vertical uses collect-then-execute with distance-based sorting to prevent overwriting unmoved boxes. **Mission Integration Benefits**: `Grid<Tile>` eliminates bounds errors, `Coord` type prevents x/y confusion, `in_bounds()`/`get()`/`get_mut()` provide safe access patterns. **Python Comparison**: Python's BFS with `list.pop(0)` (O(n) per pop) and tuple coordinates vs Rust's recursion with call stack and type-safe Coord; Python collects all cells vs Rust collects box positions only; both use collect-then-execute for Part 2 vertical but different traversal strategies. **Code Organization**: Clean separation with `try_move*()` for box logic, `simulate_robot*()` for robot tile management, `widen_grid()` for Part 2 transformation. **Educational Value**: Shows how deep problem understanding leads to simpler code (tracking all boxes → updating two positions); demonstrates when HashSet deduplication is critical (overlapping structures); proves distance-based ordering prevents overwrite bugs. **Test Coverage**: 3 comprehensive tests covering small example, large Part 1 chains, and large Part 2 overlapping structures. **Results**: Part 1 = 1,465,152, Part 2 = 1,511,259
- **Day 16**: Exemplifies **Dijkstra's algorithm implementation** with Mission 6 Grid integration and compound state space design. Showcases **type-safe state representation** using `struct State { pos: (usize, usize), dir: Direction }` with `Hash + Eq` for HashMap keys, and `enum Direction` with methods (`delta()`, `rotate_cw()`, `rotate_ccw()`) eliminating magic numbers. **Algorithm Choice**: Rust's Dijkstra with `BinaryHeap<Node>` and custom `Ord` for min-heap (O(log n) operations) vs Python's BFS with `list.pop(0)` (O(n) per operation). **Two-Phase Pattern**: Phase 1 stores only best distances in `HashMap<State, usize>` (memory efficient O(V) where V = positions × 4 directions); Phase 2 backtracks from optimal end states to reconstruct all paths. **Mission 6 Benefits**: `Grid<char>` with safe indexing (`grid[pos]`), `in_bounds(pos.into())` with `Coord` conversion, type-safe coordinate handling preventing x/y confusion. **State Space Design**: Critical insight that rotation costs require `(position, direction)` state not just position—same location facing different directions has different future costs. **Transition Modeling**: Explicit 3-transition pattern (move forward +1, rotate CW +1000, rotate CCW +1000) vs Python's implicit rotation through direction loop. **Same-Cost Path Handling**: Rust's HashMap updates implicitly allow equal-cost paths; Python explicitly uses `<` not `<=` for visited check (documented in comments). **Code Organization**: Clean separation with `State`/`Direction`/`Node` structs, `parse_maze()`, `get_neighbors()`, `dijkstra()`, separate `part1()`/`part2()` functions. **Educational Value**: Demonstrates when state space expansion is necessary (rotation costs), showcases priority queue patterns with custom ordering, proves backtracking efficiency for "find best" → "find all best" escalations. **Test Coverage**: 4 comprehensive tests covering two different maze sizes for both parts (15×15 with score 7,036, 17×17 with score 11,048). **Results**: Part 1 = 92,432, Part 2 = 458. **Python Comparison**: Python's ~68-line unified BFS with full history tracking vs Rust's ~355-line Dijkstra with backtracking—Python optimizes for midnight racing brevity, Rust optimizes for algorithmic clarity and type safety.
- **Day 17**: Demonstrates **virtual machine implementation** with clean opcode dispatch and bit manipulation patterns. Showcases **closure for combo operand resolution** capturing registers by value for dual-mode operand handling (literals 0-3 vs registers 4-6). **Part 1 VM Design**: Instruction pointer with 2-byte instructions (opcode + operand), `match` statement for exhaustive opcode handling, right shift operators (`>>=`, `>>`) idiomatic for power-of-2 divisions, bit operations (`& 7` for mod 8) replacing modulo. **Part 2 Quine Algorithm**: Recursive backtracking building A digit-by-digit backward (3 bits at a time via `current << 3 | bits`), early pruning via partial output validation before deeper recursion, `Option<i64>` cleanly representing search success/failure. **Python Comparison**: Python uses iterative forward search with mathematical jump strategy (`A += 8^i` when position i mismatches) vs Rust's depth-first backward construction—both exploit 3-bits-per-output structure but different search directions. **Algorithm Trade-offs**: Python's O(1) memory iterative jumps vs Rust's O(N) call stack recursion; Python optimizes for competitive speed (~30 lines Part 2), Rust demonstrates general backtracking pattern (~80 lines Part 2). **Key Insight**: Understanding bit-level program structure (A >>= 3 each loop) enables intelligent search strategies instead of brute force. **Test Coverage**: 7 comprehensive tests validating parsing, all 8 opcode behaviors, Part 1 execution, and VM edge cases. **Educational Value**: Shows how closure capture simplifies operand resolution, demonstrates recursive backtracking with Option returns, proves bit manipulation efficiency over arithmetic for power-of-2 operations. **Code Organization**: Standalone `execute_program()` function (not method), recursive `find_a()` helper with explicit parameters, `anyhow::Result` error handling throughout parsing.
- **Day 18**: Exemplifies **integrator philosophy** through pure mission composition—demonstrating how to build complex solutions by combining validated foundational libraries without reimplementation. Showcases **Mission 6 Grid<bool> integration** for memory space corruption tracking with automatic bounds checking, **Mission 8 Graph trait implementation** adapting custom `MemorySpace` struct for generic pathfinding algorithms, **Mission 8 shortest_path() usage** providing BFS pathfinding without custom algorithm code. **Part 1 Algorithm**: Simulate 1024 falling bytes corrupting 71×71 grid positions, implement `Graph::neighbors()` for 4-directional safe movement validation (`is_safe()` checks corruption + bounds), call `shortest_path(start, goal)` returning `Result<Vec<Coord>, GraphError>` with path length. **Part 2 Binary Search**: Efficient O(log N) search over byte count (range 0 to 3450 bytes) testing path existence at each midpoint—when path exists try more bytes (left = mid + 1), when blocked try fewer (right = mid), converges to first blocking byte coordinate. **Mission Composition Benefits**: No manual BFS queue management, no custom grid indexing with bounds checks, no coordinate arithmetic errors—focus entirely on problem logic (corruption simulation, binary search strategy). **Python Comparison**: Python uses `networkx.Graph` with `add_node()`/`add_edge()` building explicit graph structure and `shortest_path()` library call OR custom BFS with `list.pop(0)` fallback; Rust implements lightweight `Graph` trait directly on grid enabling zero-copy algorithm composition. **Algorithm Philosophy**: Python's original Part 2 tested every corrupted coordinate sequentially (slow, learned binary search from Reddit community); Rust implements binary search from start demonstrating algorithmic thinking. **Library Strategy**: Python's networkx provides batteries-included graph operations with `remove_node()` for dynamic updates; Rust's mission system provides foundational types requiring trait implementation but enabling custom optimizations. **Test Coverage**: 2 comprehensive tests validating both parts with example data (7×7 grid, 12 bytes → 22 steps, first blocker at 6,1). **Educational Value**: Perfect demonstration of integrator approach—leveraging Mission 6 coordinate safety + Mission 8 graph algorithms = clean solution focusing on problem-specific logic (corruption tracking, binary search). **Code Organization**: Clean separation with `MemorySpace` struct, `Graph` trait implementation with `neighbors()`/`contains()`/`nodes()` methods, `parse_bytes()` helper with error context, `anyhow::Result` throughout. **Results**: Part 1 = 282 steps, Part 2 = byte at (64,29) blocks path.
- **Day 19**: Demonstrates **lifetime-parametric recursion with string slice memoization** for pattern composition problems. Showcases **zero-copy prefix matching** using `strip_prefix()` returning `Option<&str>` for simultaneous validation and remainder extraction—no substring allocations. **Lifetime Management**: Manual `HashMap<&'a str, bool/u64>` with explicit `<'a>` lifetime ensuring cache keys (borrowed from input) remain valid throughout recursion, preventing dangling references. **Substring vs Position Recursion**: Rust's suffix-based approach (`remaining: &str` → `strip_prefix()` → recurse on rest) vs Python's index-based (`pos: int` → slice design → recurse on next_pos); both O(P×L) but different memory profiles—Rust O(U) unique substrings vs Python O(L) positions. **Boolean-to-Counting DP Pattern**: Identical recursive structure for Part 1 (existence check with early-exit OR) and Part 2 (exhaustive counting with accumulative SUM)—demonstrates classic DP transformation where only reduction operator changes. **Cache Strategy Trade-offs**: Manual HashMap management (explicit insert/lookup, lifetime tracking) provides educational transparency vs Python's `@cache` decorator (automatic, hidden mechanics); both achieve equivalent performance with memoization. **Educational Transparency**: Explicit memoization demonstrates cache mechanics—when to check, when to insert, how lifetimes prevent invalidation—valuable for understanding DP vs production convenience. **Flexible Parsing**: Robust format handling detecting `\n\n` (example) vs `\n` (puzzle) by testing split result length, avoiding hardcoded assumptions. **Complexity Transformation**: Shows why memoization is critical—exponential O(P^L) explosion (overlapping subproblems recomputed) → linear O(P×L) efficiency (each unique substring computed once). **String Slice Excellence**: `&str` keys in HashMap enable zero-copy caching—cache entries borrow from input string without allocation or ownership transfer, demonstrating Rust's memory efficiency. **Test-Driven Validation**: 5 comprehensive tests covering parsing correctness, individual design checks (6 possible/2 impossible validated), Part 1 totals (6), arrangement counting accuracy, Part 2 totals (16)—proves algorithm correctness and memoization benefits. **Results**: Part 1 = 360 possible designs, Part 2 = 577,474,410,989,846 arrangements.

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

**Always inspect actual code before documenting patterns.** During Day 2 documentation, incorrect claims were made about using `all()`/`any()` functions when the implementation actually used imperative loops with early returns. This highlights the importance of **evidence-based documentation** over **assumption-based documentation**.

**Verification Checklist**:

- [ ] Read the actual Rust implementation file
- [ ] Document patterns that are **actually present** in the code
- [ ] Note deliberate trade-offs (e.g., performance vs functional style)
- [ ] Compare claimed patterns against `grep`/search results in codebase

### Template for New Days

**Scope**: This template applies to **all AoC years** (2015-2024+). The comprehensive format established here provides consistent, educational documentation across all solved problems.

**Two-Part Structure**: Each day has both a **concise bullet point** (in the main list) and a **detailed analysis section** (after all bullet points, before "Problem Type Distribution" section).

#### Part 1: Concise Bullet Point (for main list)

Add to the bullet list in chronological order (e.g., after Day 18, before Day 20):

```markdown
- **Day X**: [One-sentence summary of problem]. **[Key Technique 1]**: [brief description]. **[Key Technique 2]**: [brief description]. **Mission Integration**: [if applicable]. **Python Comparison**: [key difference]. **Test Coverage**: [N tests covering X, Y, Z]. **Results**: Part 1 = X, Part 2 = Y.
```

#### Part 2: Detailed Analysis Section (comprehensive format)

Add in chronological order after all concise bullet points, before "## Problem Type Distribution":

```markdown
### Day X: [Problem Title] (Detailed Analysis)

**Title**: [Problem Title]  
**Part 1 Type**: [Category from distribution table]  
**Part 1 Description**: [What Part 1 asks you to solve with result]  
**Part 2 Type**: [Category from distribution table]  
**Part 2 Description**: [What Part 2 asks you to solve with result]  
**Key Concepts**: [Comma-separated list of relevant programming concepts]

**🧩 Algorithm Analysis**:

- **Problem Pattern**: [High-level algorithmic pattern - e.g., "Dynamic programming with state caching"]
- **Data Structure**:
  - **Rust**: [Actual Rust data structures used - e.g., "HashMap<State, Cost>", "Vec<T>", "Grid<char>"]
  - **Python**: [Python equivalent or comparison]
- **Complexity**:
  - **Time**: O(?) with explanation
  - **Space**: O(?) with explanation
  - **Critical Insight**: [Why this complexity matters for the problem]
- **AoC Theme**: [How problem fits AoC pattern - e.g., "Classic Part 2 escalation from counting to optimization"]

**Algorithm Comparison: [Rust Approach] vs [Python Approach]**:

**Python Approach** ([Description]):
```python
[Key Python code snippet showing algorithm structure]
```
- **Strategy**: [How Python solves it]
- **Trade-offs**: [Python-specific considerations]

**Rust Approach** ([Description]):
```rust
[Key Rust code snippet showing algorithm structure]
```
- **Strategy**: [How Rust solves it]
- **Trade-offs**: [Rust-specific considerations]

**🦀 Rust Conversion Highlights**:

**[Pattern 1 Name]**:
```python
[Python version]
```
```rust
[Rust version]
```
[Explanation of conversion, Rust advantages]

**[Pattern 2 Name]**:
[Similar structure for each major conversion point]

**Python vs Rust Philosophy**:

| Aspect | Python | Rust |
|--------|--------|------|
| **[Aspect 1]** | [Python approach] | [Rust approach] |
| **[Aspect 2]** | [Python approach] | [Rust approach] |
| **Code Length** | ~X lines | ~Y lines with tests |

**Educational Value**:

**Python Strengths**:
- [What Python does well for this problem]
- [Learning opportunities from Python approach]

**Rust Strengths**:
- [What Rust does well for this problem]
- [Learning opportunities from Rust approach]

**Performance Characteristics**:

- **Small Input**: [Performance notes]
- **Large Input**: [Performance notes]
- **Critical Optimization**: [What makes the solution efficient]

**Key Algorithmic Insight**: [The core learning from solving this problem]

**Test Coverage**:

Rust implementation includes comprehensive tests:
1. **[Test Category 1]**: [What it validates]
2. **[Test Category 2]**: [What it validates]
[... continue for all test categories]

**Code Organization**:

- **[Module/Function 1]**: [Responsibility]
- **[Module/Function 2]**: [Responsibility]
- **Clean Separation**: [Architecture notes]

**Mission Integration Opportunities** (if applicable):

- **Mission X**: [How it could be used or was used]
- **Pattern Recognition**: [Connection to mission patterns]

**Results**: Part 1 = [answer], Part 2 = [answer]
```

#### Required Updates After Adding Day:

1. **Update Distribution Table**: Increment counts for relevant categories (Part 1/Part 2 columns)
2. **Add to Common Patterns Observed**: Add 1-3 new patterns (numbered sequentially, currently at #72)
3. **Add to Rust-Specific Considerations**: Add comprehensive bullet point with **bold keywords** (chronologically after previous entries)
4. **Update "Last Updated" date** and **"Days Implemented"** list at bottom
5. **Add tags** if introducing new concepts (at very bottom, e.g., `#dynamic-programming`, `#memoization`)
6. **Verify against actual code** ✅ (use grep/read_file to confirm claims - evidence-based documentation)

#### Cross-Year Considerations:

When documenting solutions from previous years (2015-2023):
- **Python solutions may not exist** - focus on Rust-specific patterns and compare to common algorithmic approaches
- **Complexity evolution** - Earlier years (2015-2017) tend to be simpler; later years (2020-2023) more complex
- **Pattern recognition** - Note when a pattern appears across multiple years (e.g., "This BFS pattern similar to 2024 Day 18")
- **Mission integration opportunities** - Identify where mission libraries could simplify older solutions
- **Historical context** - Mention if problem introduces a pattern that becomes common in later years

---

*Last Updated: December 18, 2025*
*Days Implemented: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19*
*Days Available: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25*

---
*Tags: #aoc #2024 #problem-analysis #patterns #rust-conversion #algorithm-learning #mission6-integration #mission8-integration #foundational-libraries #flood-fill #corner-counting #generic-functions #linear-algebra #cramers-rule #rayon #data-parallelism #dijkstra #state-space-search #virtual-machine #quine #bit-manipulation #binary-search #graph-adapters #pathfinding #integrator-philosophy*
*Links: [[../../../zettelkasten/AoC Patterns MOC]] | [[../../../zettelkasten/AoC Integration]] | [[../../../zettelkasten/aoc2024-day4-mission6-example]] | [[../../../zettelkasten/missions/mission-6]] | [[../../../zettelkasten/missions/mission-8]] | [[../../../zettelkasten/rayon-parallel-iterators]] | [[../../../zettelkasten/dijkstra-algorithm]] | [[../../../zettelkasten/virtual-machine-patterns]] | [[../../../zettelkasten/recursive-backtracking]] | [[../../../zettelkasten/binary-search-patterns]] | [[../../../zettelkasten/graph-trait-adapters]]*
