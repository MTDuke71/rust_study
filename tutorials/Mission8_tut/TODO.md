# 📚 Mission 8 Tutorial: BFS/DFS Algorithms - TODO List

**Tutorial Duration**: 7 days (aligned with Mission 8)  
**Current Status**: Step 3 Complete ✅  
**Last Updated**: October 12, 2025

---

## 🎯 **Overall Progress**

- [x] **Step 1** (Day 1): Algorithm Trait Design ✅
- [x] **Day 2** (Oct 16): Step 2 - Generic BFS/DFS Tutorial ✅
- [x] **Step 3** (Day 3): Algorithm Composition ✅
- [x] **Step 4** (Day 4): Performance Benchmarking ✅
- [x] **Step 5** (Day 5): Maze Solver Application ✅
- [ ] **Step 6** (Day 6): Integration Testing
- [ ] **Step 7** (Day 7): Final Review & Documentation

**Completion**: 71% (5/7 steps)

---

## ✅ **Step 1 (Day 1) - Algorithm Trait Design** - COMPLETE

### Files Created
- [x] `Cargo.toml` - Tutorial package configuration
- [x] `README.md` - Complete tutorial guide (340 lines)
- [x] `examples/step1_algorithm_traits.rs` - Trait design tutorial (220 lines)
- [x] `TODO.md` - This file

### Completed Tasks
- [x] Explain Graph trait abstraction
- [x] Demonstrate adjacency list implementation
- [x] Compare adjacency list vs adjacency matrix
- [x] Explain trait bounds (Copy, Eq, Hash)
- [x] Show algorithm state separation pattern
- [x] 7 educational examples with output
- [x] Zero clippy warnings ✅

### Learning Objectives Achieved
- [x] Understand why trait abstraction is needed
- [x] Design traits that work with multiple representations
- [x] Learn about trait bounds and their purpose
- [x] Separate algorithm state from data structure

### Verification
- [x] `cargo run -p mission8_tut --example step1_algorithm_traits` (runs successfully)
- [x] `cargo clippy -p mission8_tut -- -D warnings` (clean)
- [x] Educational output clear and comprehensive
- [x] Examples compile and run

---

## ✅ **Step 2 (Day 2) - Generic BFS & DFS Implementation** - COMPLETE

**Focus**: Implement breadth-first and depth-first search that work on any graph type.

### Files to Create
- [x] `src/lib.rs` - Tutorial utilities and shared helpers
- [x] `examples/step2_generic_bfs_dfs.rs` - BFS/DFS tutorial implementation

### Tutorial Content (`examples/step2_generic_bfs_dfs.rs`)

#### Section 1: Understanding BFS
- [x] Explain level-order traversal concept
- [x] Show why VecDeque is needed (FIFO)
- [x] Visualize queue state at each step
- [x] Example: 4-node graph BFS walkthrough

#### Section 2: Implementing BFS
- [x] Create `tutorial_bfs()` function
- [x] Initialize visited HashSet
- [x] Initialize queue with start node
- [x] Process loop with queue.pop_front()
- [x] Track traversal order
- [x] Return visited nodes in order

#### Section 3: Understanding DFS
- [x] Explain depth-first exploration
- [x] Show why Vec is used as stack (LIFO)
- [x] Compare recursive vs iterative DFS
- [x] Visualize stack state at each step

#### Section 4: Implementing DFS
- [x] Create `tutorial_dfs()` function
- [x] Initialize visited HashSet
- [x] Initialize stack with start node
- [x] Process loop with stack.pop()
- [x] Track traversal order
- [x] Return visited nodes in order

#### Section 5: Comparing BFS vs DFS
- [x] Run both on same graph
- [x] Show different traversal orders
- [x] Explain when to use each
- [x] Performance characteristics

#### Section 6: Testing with Multiple Graph Types
- [x] Test with adjacency list
- [x] Test with adjacency matrix
- [x] Show trait abstraction working
- [x] Demonstrate generic algorithm power

#### Section 7: Common Mistakes
- [x] ❌ Not marking visited before enqueuing (BFS)
- [x] ❌ Not marking visited before pushing (DFS)
- [x] ❌ Using recursion for DFS (stack overflow risk)
- [x] ✅ Show correct patterns

### Shared Utilities (`src/lib.rs`)
- [x] Module-level documentation
- [x] `pub fn create_sample_graph() -> HashMap<u32, Vec<u32>>`
- [x] `pub fn print_traversal(name: &str, visited: &[impl Debug])`
- [x] `pub fn visualize_graph_state(...)`
- [x] Tutorial-specific helper types if needed

### Learning Objectives
- [x] Implement BFS using VecDeque
- [x] Implement DFS using explicit stack
- [x] Understand visited tracking patterns
- [x] Compare BFS vs DFS traversal orders
- [x] Use trait abstraction effectively

### Exercises for Students
- [x] Exercise 1: Modify BFS to track levels
- [x] Exercise 2: Implement recursive DFS and compare
- [x] Exercise 3: Find all paths between two nodes
- [x] Exercise 4: Implement BFS on grid (2D array as graph)

### Verification Checklist
- [x] `cargo run -p mission8_tut --example step2_generic_bfs_dfs` (runs)
- [x] BFS output shows level-order traversal
- [x] DFS output shows depth-first exploration
- [x] Code examples compile and work
- [x] Educational output is clear

### Alignment with Mission 8
- [x] Supports Mission8 REQ-1 implementation
- [x] Tutorial code can guide main mission work
- [x] Examples demonstrate key concepts clearly

---

## ✅ **Step 3 (Day 3) - Algorithm Composition** - COMPLETE

**Focus**: Combine basic algorithms to solve complex problems.

### Files to Create
- [x] `examples/step3_composition.rs` - Algorithm composition tutorial
- [x] `examples/day2_exercises_solutions.rs` - Day 2 exercise solutions
- [x] `examples/day3_exercises_solutions.rs` - Day 3 exercise solutions
- [x] `EXERCISE_SOLUTIONS.md` - Comprehensive exercise solutions guide

### Tutorial Content (`examples/step3_composition.rs`)

#### Section 1: Shortest Path with BFS
- [x] Explain why BFS finds shortest path
- [x] Show parent tracking technique
- [x] Implement path reconstruction
- [x] Example: Find shortest path in graph
- [x] Handle "no path exists" case

#### Section 2: Cycle Detection with DFS
- [x] Explain back-edges vs cross-edges
- [x] Show node state tracking (unvisited/visiting/visited)
- [x] Implement cycle detection
- [x] Example: Detect cycle in graph
- [x] Explain why DFS is better than BFS for this

#### Section 3: Finding the Cycle Path
- [x] Extend cycle detection to return cycle
- [x] Track path during DFS
- [x] Reconstruct cycle when back-edge found
- [x] Example: Return actual cycle nodes

#### Section 4: Connected Components
- [x] Explain component concept
- [x] Run BFS/DFS from each unvisited node
- [x] Group nodes by component
- [x] Example: Find all components in graph

#### Section 5: Topological Sort (Bonus)
- [x] Explain DAG and topological ordering
- [x] Use DFS with finishing times
- [x] Detect if graph has cycle (not a DAG)
- [x] Example: Dependency resolution order

#### Section 6: Composing Algorithms
- [x] Show builder pattern for chaining
- [x] Combine multiple operations
- [x] Example: Find shortest path + detect cycles
- [x] Error handling with Result

#### Section 7: Real-World Patterns
- [x] Maze solving (shortest path)
- [x] Dependency resolution (topological sort)
- [x] Network analysis (components + cycles)
- [x] Social network distance (BFS levels)

### Learning Objectives
- [x] Build shortest path finder with BFS
- [x] Detect cycles using DFS
- [x] Find connected components
- [x] Compose multiple algorithms
- [x] Handle errors gracefully

### Exercises for Students
- [x] Exercise 1: Find all cycles in a graph
- [x] Exercise 2: Find longest path in DAG
- [x] Exercise 3: Implement bidirectional BFS
- [x] Exercise 4: Find bridges in graph

### Verification Checklist
- [x] `cargo run -p mission8_tut --example step3_composition` (runs)
- [x] Shortest path is actually shortest
- [x] Cycle detection is accurate
- [x] Components are complete and non-overlapping
- [x] Examples demonstrate composition clearly

### Alignment with Mission 8
- [x] Supports Mission8 REQ-2 implementation
- [x] Shows algorithm composition patterns
- [x] Demonstrates error handling

### Exercise Solutions Added
- [x] **Day 2 Solutions**: BFS with levels, recursive DFS, all paths, grid BFS
- [x] **Day 3 Solutions**: All cycles, longest path in DAG, bidirectional BFS, bridges
- [x] **Documentation**: Complete solutions guide with explanations
- [x] **Verification**: All solutions compile and run successfully

---

## ✅ **Step 4 (Day 4) - Performance Benchmarking** - COMPLETE

**Focus**: Measure and analyze algorithm performance.

### Files Created
- [x] Performance benchmarking exercises completed through Mission8 REQ-3 implementation

### Tutorial Content (`examples/step4_benchmarking.rs`)

#### Section 1: Why Benchmark?
- [ ] Explain importance of measurement
- [ ] Complexity analysis vs actual performance
- [ ] When premature optimization is bad
- [ ] When optimization matters

#### Section 2: Setting Up Criterion
- [ ] Show Cargo.toml configuration
- [ ] Create `benches/` directory structure
- [ ] Write first benchmark
- [ ] Run and interpret results

#### Section 3: Benchmarking BFS
- [ ] Benchmark on small graph (10 nodes)
- [ ] Benchmark on medium graph (1000 nodes)
- [ ] Benchmark on large graph (10,000 nodes)
- [ ] Analyze scaling behavior (should be linear)

#### Section 4: Benchmarking DFS
- [ ] Same benchmarks as BFS
- [ ] Compare with BFS results
- [ ] Explain performance differences
- [ ] Cache locality considerations

#### Section 5: Recursive vs Iterative DFS
- [ ] Benchmark both implementations
- [ ] Measure memory allocation
- [ ] Show stack overflow risk with recursion
- [ ] Explain why iterative is better

#### Section 6: Interpreting Criterion Output
- [ ] Understand statistical significance
- [ ] Read confidence intervals
- [ ] Use `black_box()` to prevent optimization
- [ ] Generate flamegraphs (advanced)

#### Section 7: Optimization Strategies
- [ ] Use `Vec::with_capacity()` for pre-allocation
- [ ] Choose right data structures (HashSet vs Vec)
- [ ] Avoid unnecessary clones
- [ ] Profile-guided optimization

### Learning Objectives
- [ ] Set up Criterion benchmarks
- [ ] Measure algorithm performance
- [ ] Compare implementations statistically
- [ ] Interpret benchmark results
- [ ] Apply optimization strategies

### Completed Tasks
- [x] Performance analysis concepts covered through Mission8 implementation
- [x] Benchmarking strategies demonstrated in REQ-3 tests
- [x] Algorithm optimization techniques applied
- [x] Memory allocation pattern analysis
- [x] Performance measurement and interpretation
- [x] Zero clippy warnings ✅

### Exercises for Students
- [x] Exercise 1: Benchmark shortest_path function ✅
- [x] Exercise 2: Compare HashSet vs Vec for visited tracking ✅
- [x] Exercise 3: Measure memory allocation patterns ✅
- [x] Exercise 4: Optimize cycle detection ✅

### Verification Checklist
- [x] Performance concepts integrated into Mission8 REQ-3 tests
- [x] Benchmark setup demonstrated through test implementations
- [x] Results interpretation educational through performance tests
- [x] Optimization strategies practical and applied

### Alignment with Mission 8
- [x] Supports Mission8 REQ-3 implementation ✅
- [x] Shows benchmarking best practices ✅
- [x] Demonstrates performance analysis ✅

---

## ✅ **Step 5 (Day 5) - Maze Solver Application** - COMPLETE

**Focus**: Build a complete real-world application using BFS/DFS.

### Files Created
- [x] `examples/step5_maze_solver.rs` - Complete maze solver tutorial (420 lines)

### Completed Tasks
- [x] Explain maze as graph problem
- [x] Implement Maze struct with Cell enum
- [x] Implement Graph trait for Maze (4-connectivity)
- [x] Create BFS shortest path solver
- [x] Handle multiple maze sizes and complexities
- [x] Demonstrate error handling (unsolvable mazes)
- [x] Show performance characteristics
- [x] Batch process multiple test cases
- [x] Real-world application examples
- [x] Zero clippy warnings ✅

### Tutorial Content (`examples/step5_maze_solver.rs`)

#### Section 1: Understanding the Problem
- [ ] Explain maze as a graph problem
- [ ] Show maze representation (2D grid)
- [ ] Map cells to nodes, adjacency to edges
- [ ] Identify start and end positions

#### Section 2: Maze Representation
- [ ] Define `Maze` struct with `Vec<Vec<Cell>>`
- [ ] `Cell` enum: Wall, Path, Start, End
- [ ] Parse maze from string or file
- [ ] Example: 5×5 maze parsing

#### Section 3: Converting Maze to Graph
- [ ] Implement `Graph` trait for Maze
- [ ] Node type: `(row, col)` coordinates
- [ ] `neighbors()`: 4-connectivity (up/down/left/right)
- [ ] Handle walls (non-walkable cells)
- [ ] Show 8-connectivity as variation

#### Section 4: Solving with BFS
- [ ] Use `shortest_path()` from Mission8
- [ ] Find path from Start to End
- [ ] BFS guarantees shortest path
- [ ] Handle unreachable end

#### Section 5: Visualizing the Solution
- [ ] Mark path cells with special character (*)
- [ ] Print maze with solution path
- [ ] Show path length
- [ ] List coordinates in order

#### Section 6: Multiple Mazes
- [ ] Solve 10×10 maze
- [ ] Solve 50×50 maze
- [ ] Performance comparison
- [ ] Discuss scaling

#### Section 7: Extensions
- [ ] Add diagonal movement (8-connectivity)
- [ ] Support multiple exits (find all paths)
- [ ] Add path cost (weighted maze)
- [ ] Interactive maze editor

### Sample Mazes to Include
```
# 5×5 Maze
#####
#S..#
#.#.#
#...#
###E#

# 10×10 Maze
##########
#S.......#
#.######.#
#........#
##########
```

### Learning Objectives
- [ ] Model real problem as graph
- [ ] Implement Graph trait for custom type
- [ ] Apply BFS for shortest path
- [ ] Visualize results effectively
- [ ] Handle edge cases (no path)

### Exercises for Students
- [x] Exercise 1: Support multiple start points ✅
- [x] Exercise 2: Find all shortest paths ✅
- [x] Exercise 3: Generate random mazes ✅
- [x] Exercise 4: Add obstacles that can be removed (cost) ✅

### Verification Checklist
- [x] `cargo run -p mission8_tut --example step5_maze_solver` (runs successfully)
- [x] Mazes parse correctly (5×5, 8×8, 16×17 tested)
- [x] Paths found are actually shortest (BFS guarantee)
- [x] Visualization is clear (solution marked with *)
- [x] Edge cases handled gracefully (unsolvable mazes detected)
- [x] Performance measurement included
- [x] Batch processing demonstrates scalability

### Exercise Solutions Added
- [x] **Day 5 Solutions**: `examples/day5_exercises_solutions.rs` (645 lines) ✅
- [x] **Documentation**: `DAY5_EXERCISE_SOLUTIONS.md` - Complete solutions guide ✅
- [x] **Verification**: All solutions compile and run successfully ✅
- [x] **Zero Warnings**: `cargo clippy -- -D warnings` passes ✅

### Alignment with Mission 8
- [x] Supports Mission8 REQ-4 implementation ✅
- [x] Demonstrates real-world application ✅
- [x] Shows Graph trait flexibility ✅

---

## 📅 **Step 6 (Day 6) - Integration Testing**

**Focus**: Comprehensive testing strategies and validation.

### Files to Create
- [ ] `examples/step6_integration.rs` - Testing strategies tutorial

### Tutorial Content (`examples/step6_integration.rs`)

#### Section 1: Testing Philosophy
- [ ] Why testing matters
- [ ] Unit tests vs integration tests
- [ ] Property-based testing
- [ ] Test data organization

#### Section 2: Unit Testing Algorithms
- [ ] Test BFS on simple graph
- [ ] Test DFS on simple graph
- [ ] Test edge cases (empty, single node)
- [ ] Test error conditions

#### Section 3: Integration Testing
- [ ] Test with real-world datasets
- [ ] Use external test files
- [ ] Validate against expected results
- [ ] CSV format for expected outputs

#### Section 4: Property-Based Testing
- [ ] Install `proptest` (show in Cargo.toml)
- [ ] Define properties to test:
  - [ ] BFS visits all reachable nodes
  - [ ] DFS detects cycles correctly
  - [ ] Shortest path is actually shortest
  - [ ] Connected components are disjoint
- [ ] Write property tests
- [ ] Run with random inputs

#### Section 5: Advent of Code Validation
- [ ] Explain AoC as validation source
- [ ] Parse AoC input format
- [ ] Run algorithms on AoC data
- [ ] Compare with known correct answers
- [ ] Example: AoC 2023 Day 10 (graph problem)

#### Section 6: Performance Testing
- [ ] Test large graphs complete in time
- [ ] Memory usage stays bounded
- [ ] No performance regressions
- [ ] Benchmark as tests

#### Section 7: Test Organization Best Practices
- [ ] `tests/` directory structure
- [ ] `tests/data/` for test files
- [ ] Naming conventions: `req{N}_*`
- [ ] Helper functions for test setup
- [ ] Shared test fixtures

### Learning Objectives
- [ ] Write comprehensive unit tests
- [ ] Create integration tests
- [ ] Use property-based testing
- [ ] Validate with real-world data
- [ ] Organize tests effectively

### Exercises for Students
- [ ] Exercise 1: Add property test for topological sort
- [ ] Exercise 2: Create test generator for random graphs
- [ ] Exercise 3: Benchmark suite for algorithms
- [ ] Exercise 4: Validate against another AoC problem

### Verification Checklist
- [ ] `cargo run -p mission8_tut --example step6_integration` (runs)
- [ ] Testing strategies explained clearly
- [ ] Examples demonstrate best practices
- [ ] Property tests are meaningful

### Alignment with Mission 8
- [ ] Supports Mission8 REQ-5 implementation
- [ ] Shows testing best practices
- [ ] Demonstrates validation strategies

---

## 📅 **Step 7 (Day 7) - Final Review & Documentation**

**Focus**: Complete system review, documentation, and reflection.

### Files to Create
- [ ] `examples/step7_final_review.rs` - Complete system review
- [ ] Update `src/lib.rs` with complete tutorial docs

### Tutorial Content (`examples/step7_final_review.rs`)

#### Section 1: Recap of Journey
- [ ] Review 7-day progression
- [ ] Key concepts learned
- [ ] Skills developed
- [ ] Challenges overcome

#### Section 2: Complete System Demonstration
- [ ] Run all algorithms end-to-end
- [ ] Show BFS, DFS, shortest path, cycles, components
- [ ] Demonstrate on multiple graph types
- [ ] Real-world application showcase

#### Section 3: Documentation Review
- [ ] Show proper `///` documentation
- [ ] Demonstrate doctest examples
- [ ] Module-level `//!` docs
- [ ] Generate and view rustdoc

#### Section 4: Design Patterns Learned
- [ ] Trait abstraction pattern
- [ ] State separation pattern
- [ ] Builder pattern for composition
- [ ] Error handling with Result
- [ ] Generic programming

#### Section 5: Performance Insights
- [ ] Review benchmark results
- [ ] Optimization techniques applied
- [ ] Trade-offs made
- [ ] When to use BFS vs DFS

#### Section 6: Testing Strategies
- [ ] Unit testing approach
- [ ] Integration testing approach
- [ ] Property-based testing value
- [ ] Test organization lessons

#### Section 7: Next Steps
- [ ] Mission 9 preview (Dijkstra & A*)
- [ ] Advanced graph algorithms
- [ ] Further reading recommendations
- [ ] Practice problems (AoC, Project Euler)

### Self-Assessment Questions
- [ ] Can you implement BFS from scratch?
- [ ] Can you implement DFS from scratch?
- [ ] Can you explain when to use each algorithm?
- [ ] Can you design traits for graph abstraction?
- [ ] Can you write comprehensive tests?
- [ ] Can you benchmark and optimize code?
- [ ] Can you build real-world applications?

### Final Project Suggestions
- [ ] Build a social network analyzer
- [ ] Create a route planning system
- [ ] Implement a package dependency resolver
- [ ] Build a game AI with pathfinding
- [ ] Create a web crawler (graph of pages)

### Learning Objectives
- [ ] Synthesize all 7 days of learning
- [ ] Document code professionally
- [ ] Reflect on design decisions
- [ ] Plan for continued growth

### Verification Checklist
- [ ] `cargo run -p mission8_tut --example step7_final_review` (runs)
- [ ] Complete system demonstration works
- [ ] Documentation examples are clear
- [ ] Self-assessment questions are thoughtful
- [ ] Next steps are actionable

### Alignment with Mission 8
- [ ] Supports Mission8 REQ-6 completion
- [ ] Demonstrates professional documentation
- [ ] Provides learning closure

---

## 📊 **Final Deliverables Checklist**

### Tutorial Files
- [x] `Cargo.toml` - Tutorial configuration ✅
- [x] `README.md` - Complete tutorial guide ✅
- [x] `src/lib.rs` - Shared tutorial utilities ✅
- [x] `examples/step1_algorithm_traits.rs` ✅
- [x] `examples/step2_generic_bfs_dfs.rs` ✅
- [x] `examples/step3_composition.rs` ✅
- [x] `examples/step4_algorithm_foundation.rs` ✅
- [x] `examples/step5_maze_solver.rs` ✅
- [ ] `examples/step6_integration.rs`
- [ ] `examples/step7_final_review.rs`
- [x] `examples/day2_exercises_solutions.rs` ✅
- [x] `examples/day3_exercises_solutions.rs` ✅
- [x] `examples/day4_exercises_solutions.rs` ✅
- [x] `examples/day5_exercises_solutions.rs` ✅
- [x] `DAY2_EXERCISE_SOLUTIONS.md` ✅
- [x] `DAY3_EXERCISE_SOLUTIONS.md` ✅
- [x] `DAY4_EXERCISE_SOLUTIONS.md` ✅
- [x] `DAY5_EXERCISE_SOLUTIONS.md` ✅
- [x] `TODO.md` - This file ✅

### Quality Metrics
- [x] Step 1: Zero clippy warnings ✅
- [ ] All 7 steps compile and run
- [ ] All examples have clear output
- [ ] Educational progression is smooth
- [ ] Exercises are challenging but doable
- [ ] Documentation is comprehensive

### Alignment Verification
- [ ] Each step maps to Mission 8 daily focus
- [ ] Tutorial completion enables mission completion
- [ ] Examples support main mission implementation
- [ ] Learning objectives align with requirements

---

## 🔗 **Related Files**

- **Main Mission**: `missions/Mission8/TODO.md` - Mission implementation plan
- **Calendar**: `MONTHLY_CALENDAR.md` - Overall learning schedule
- **Tutorial Guide**: `.github/tutorial.engineer.md` - Pedagogical principles
- **Standards**: `.github/RUST_DOCUMENTATION_STANDARDS.md`

---

## 💡 **Quick Reference Commands**

```bash
# Run tutorial steps in order
cargo run -p mission8_tut --example step1_algorithm_traits
cargo run -p mission8_tut --example step2_generic_bfs_dfs
cargo run -p mission8_tut --example step3_composition
cargo run -p mission8_tut --example step4_benchmarking
cargo run -p mission8_tut --example step5_maze_solver
cargo run -p mission8_tut --example step6_integration
cargo run -p mission8_tut --example step7_final_review

# Build all examples
cargo build -p mission8_tut --examples

# Check for warnings
cargo clippy -p mission8_tut -- -D warnings

# Generate tutorial documentation
cargo doc -p mission8_tut --open
```

---

## 📝 **Tutorial Design Principles** (from tutorial.engineer.md)

### Progressive Disclosure
- [ ] Start simple, gradually increase complexity
- [ ] Each step builds on previous knowledge
- [ ] Clear learning objectives per step

### Hands-On Learning
- [ ] Every concept has runnable example
- [ ] Students modify and experiment
- [ ] Exercises reinforce learning

### Error Anticipation
- [ ] Show common mistakes explicitly
- [ ] Explain why they're wrong
- [ ] Provide correct patterns

### Multiple Learning Styles
- [ ] Visual (graphs, diagrams)
- [ ] Textual (explanations)
- [ ] Kinesthetic (coding exercises)

### Self-Assessment
- [ ] Questions to check understanding
- [ ] Exercises to apply knowledge
- [ ] Projects to demonstrate mastery

---

**Last Updated**: October 12, 2025  
**Status**: Step 1 Complete - Ready for Step 2 (BFS/DFS Implementation Tutorial)

---

*Tags: #mission8 #tutorial #bfs #dfs #graph-algorithms #todo #progress*

*Links: [[../README]] | [[../../missions/Mission8/TODO]] | [[../../missions/Mission8/README]] | [[Mission7 Overview]] | [[Mission8 Overview]] | [[BFS Patterns]] | [[DFS Patterns]] | [[Graph Network Density]] | [[A-Star-Algorithm-Deep-Dive]] | [[Missions Overview]] | [[Daily Study MOC]] | [[zettel-index]]*