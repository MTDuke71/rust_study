# AoC Patterns - Map of Contents

**Comprehensive guide to Advent of Code problem-solving patterns, strategies, and implementation techniques**

---

## 🎯 **Problem Classification System**

### **Input Processing Patterns**
- **[[Text Parsing Patterns]]** - Line-by-line, regex, and custom parsers
- **[[Number Processing]]** - Integer parsing, range handling, mathematical operations
- **[[Grid Input Handling]]** - 2D data ingestion and coordinate systems
- **[[Multi-Format Parsing]]** - JSON, CSV, custom delimiter handling
- **[[Stream Processing]]** - Large input handling and memory efficiency

### **Data Structure Selection**
- **[[AoC Collection Problems]]** - When to use each collection type
- **[[Graph Representation]]** - Adjacency lists, matrices, and edge lists
- **[[Tree Structures]]** - Binary trees, n-ary trees, and traversal patterns
- **[[Queue vs Stack Usage]]** - BFS, DFS, and state management
- **[[Coordinate Systems]]** - 2D/3D grids, hex grids, and navigation

---

## 🔍 **Algorithm Pattern Library**

### **Search Algorithms**
- **[[BFS Patterns]]** - Shortest path, level-order, and state exploration
- **[[DFS Patterns]]** - Recursive exploration, backtracking, and cycle detection
- **[[A* Search]]** - Heuristic search for optimal pathfinding
  - **[[Chebyshev Distance]]** - Chessboard/8-connected distance metric
  - **[[Manhattan Distance]]** - 4-connected grid distance
  - **[[Euclidean Distance]]** - Continuous space distance
- **[[Binary Search]]** - Sorted array search and optimization problems
- **[[Two-Pointer Techniques]]** - Array manipulation and meeting problems

### **Dynamic Programming**
- **[[Memoization Patterns]]** - Top-down DP with HashMap caching
- **[[Tabulation Patterns]]** - Bottom-up DP with Vec storage
- **[[State Space Analysis]]** - Identifying DP subproblems and transitions
- **[[Optimization Problems]]** - Min/max cost paths and resource allocation
- **[[Counting Problems]]** - Ways to achieve goals and combinatorics

### **Graph Algorithms**
- **[[Connected Components]]** - Union-Find and flood-fill techniques
- **[[Shortest Path]]** - Dijkstra, Floyd-Warshall, and BFS variants
- **[[Cycle Detection]]** - Directed and undirected cycle finding
- **[[Topological Sort]]** - Dependency resolution and ordering
- **[[Minimum Spanning Tree]]** - Kruskal's and Prim's algorithms

### **String Algorithms**
- **[[Pattern Matching]]** - KMP, Boyer-Moore, and regex approaches
  - **Quick Reference**: [[../../tutorials/Mission5_tut/REGEX_QUICK_REFERENCE|Regex Quick Reference]] - Comprehensive regex guide with AoC examples
- **[[String Manipulation]]** - Rotation, palindromes, and transformation
- **[[Parsing Techniques]]** - Recursive descent and state machine parsing
- **[[Text Processing]]** - Tokenization, normalization, and analysis
- **[[Encoding/Decoding]]** - Caesar cipher, Base64, and custom encodings
- **Stack-Based Validation** - [[../../advanced_examples/Brackets_Ext/README_EXTENDED|Brackets Extended]] - Configurable bracket matching with multi-error reporting

---

## 📅 **Year-Specific Pattern Analysis**

### **AoC 2015 Patterns**
- **[[../aoc2015/Problem_Statements/summary]]** - Common themes and solution approaches
- **[[../aoc2015/discussions/README]]** - Year-specific implementation notes
- **[[2015 Day Summaries]]** - Quick reference for each day's pattern
- **[[2015 Performance Notes]]** - Optimization lessons learned

### **AoC 2020 Patterns**
- **[[2020 Problem Analysis]]** - Parsing complexity and algorithm variety
- **[[Assembly Simulation]]** - Virtual machine and instruction processing
- **[[Rule Engine Patterns]]** - Complex validation and constraint solving
- **[[Set Theory Applications]]** - Intersection, union, and membership problems

### **Cross-Year Patterns**
- **[[Recurring Themes]]** - Patterns that appear across multiple years
- **[[Difficulty Progression]]** - How complexity builds through December
- **[[Common Gotchas]]** - Frequent pitfalls and debugging strategies
- **[[Evolution Analysis]]** - How AoC problems have changed over time

---

## 🛠️ **Implementation Strategies**

### **Code Organization**
- **[[Solution Structure]]** - Consistent organization for AoC solutions
- **[[Testing Patterns]]** - Unit tests, integration tests, and validation
- **[[Input Management]]** - File handling and test data organization
- **[[Performance Measurement]]** - Benchmarking and optimization tracking
- **[[Debug Strategies]]** - Visualization and step-through techniques

### **Rust-Specific Techniques**
- **[[Iterator Patterns]]** - Functional approaches to problem solving
- **[[Error Handling]]** - Result types and graceful failure handling
- **[[Memory Management]]** - Ownership patterns for complex data structures
- **[[Trait Usage]]** - Custom traits for problem domain modeling
- **[[Macro Patterns]]** - Code generation for repetitive problems

### **Performance Optimization**
- **[[Algorithmic Optimization]]** - Choosing better algorithms
- **[[Data Structure Tuning]]** - Collection selection and configuration
- **[[Memory Optimization]]** - Allocation reduction and cache efficiency
- **[[Parallel Processing]]** - Rayon integration for embarrassingly parallel problems
- **[[Profiling Techniques]]** - Finding and fixing performance bottlenecks

---

## 🎲 **Problem Categories**

### **Simulation Problems**
- **[[Game of Life Variants]]** - Cellular automata and state evolution
- **[[Physics Simulation]]** - Particle systems and collision detection
- **[[Process Simulation]]** - Assembly execution and virtual machines
- **[[Growth Simulation]]** - Population dynamics and exponential processes
- **[[Time-Based Simulation]]** - Event scheduling and temporal logic

### **Optimization Problems**
- **[[Shortest Path Finding]]** - Navigation and route optimization
- **[[Resource Allocation]]** - Knapsack variants and constraint satisfaction
- **[[Scheduling Problems]]** - Task ordering and deadline management
- **[[Packing Problems]]** - 2D/3D space utilization and fitting
- **[[Network Flow]]** - Maximum flow and minimum cut problems

### **Combinatorial Problems**
- **[[Permutation Generation]]** - All arrangements and ordering problems
- **[[Subset Generation]]** - Power sets and combination enumeration
- **[[Constraint Satisfaction]]** - Sudoku-style and rule-based solving
- **[[Game Theory]]** - Optimal play and minimax strategies
- **[[Counting Problems]]** - Combinatorics and mathematical sequences

### **Parsing and Interpretation**
- **[[Expression Evaluation]]** - Mathematical and logical expression parsing
- **[[Command Processing]]** - Instruction parsing and execution
- **[[Grammar Parsing]]** - Formal language processing and validation
- **[[Data Format Parsing]]** - Custom format interpretation and conversion
- **[[Protocol Implementation]]** - Message parsing and state machines

---

## 🔗 **Integration Points**

### **Learning Track Connections**
- **[[../Mission2/README]]** - Queue implementations for BFS problems
- **[[../Mission5/README]]** - HashMap usage in frequency and caching problems
- **[[../Mission6/README]]** - Grid algorithms and 2D problem solving
- **[[../daily_study/rust_learning_week2_notes/Day10]]** - HashMap patterns in practice
- **[[AoC Collection Problems]]** - Collection selection for competitive programming

### **External Resources**
- **[[AoC Community Solutions]]** - Analysis of popular solution approaches
- **[[Algorithm Visualization]]** - Tools for understanding complex algorithms
- **[[Competitive Programming]]** - Broader CP techniques applicable to AoC
- **[[Mathematical Foundations]]** - Number theory and discrete math for AoC
- **[[Complexity Analysis]]** - Big-O analysis for AoC solution evaluation

### **Workspace Integration**
- **[[../aoc_pattern_recognition/README]]** - Automated pattern detection tools
- **[[../aoc_scaffold_templates_with_tests/README]]** - Template generation system
- **[[../advanced_examples/README]]** - Production-ready implementations of core patterns (Stack, Queue, BFS, Tree algorithms)
- **[[Performance Optimization Guide]]** - General optimization strategies
- **[[Tutorial Engineering]]** - Creating educational content for patterns

---

## 🚀 **Quick Reference**

### **By Difficulty Level**
- **Easy (Days 1-5)**: Basic parsing, simple algorithms, direct implementation
- **Medium (Days 6-15)**: Moderate algorithms, data structure selection, optimization
- **Hard (Days 16-25)**: Complex algorithms, advanced optimization, mathematical insight

### **By Time Complexity**
- **O(n) Solutions**: Linear processing, single-pass algorithms
- **O(n log n) Solutions**: Sorting-based, tree operations, efficient search
- **O(n²) Solutions**: Nested loops, pairwise comparisons, simple DP
- **Exponential Solutions**: Backtracking, brute force with pruning

### **By Common Patterns**
- **Grid Navigation**: 2D movement, pathfinding, cellular automata
- **Tree Processing**: Parsing, traversal, structural analysis
- **Number Theory**: Modular arithmetic, prime numbers, mathematical sequences
- **String Processing**: Parsing, pattern matching, transformation
- **Simulation**: State machines, iterative processes, complex rules

---

*Tags: #aoc #patterns #algorithms #competitive-programming #problem-solving #reference #navigation #moc*
*Links: [[AoC Collection Problems]] | [[Performance Optimization Guide]] | [[Rust Collections MOC]] | [[../aoc2015/README]] | [[../Mission5/README]]*