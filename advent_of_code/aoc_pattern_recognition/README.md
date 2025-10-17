# 🎯 AoC Pattern Recognition

**Algorithm pattern trainer and competitive programming utilities for Advent of Code mastery**

This library teaches **pattern recognition** - the most critical skill for fast, consistent AoC success. Instead of solving problems from scratch, learn to identify problem types and apply proven solution templates.

---

## 📚 **What's Inside**

### **Core Pattern Libraries**

#### 🔲 **Grid Patterns** (`src/grid_patterns.rs`)
2D spatial problem solutions - the most common AoC pattern category
- **BFS/DFS traversal** - Shortest paths, flood fill, maze solving
- **Coordinate systems** - Grid navigation, neighbor detection
- **Pathfinding algorithms** - A*, Dijkstra's for weighted graphs
- **Spatial transformations** - Rotations, reflections, translations

**Typical AoC Problems**: Maze navigation, cellular automata, map exploration, grid-based puzzles

#### 📝 **Parsing Patterns** (`src/parsing_patterns.rs`)
Fast, robust input transformation - foundation of competitive speed
- **Regex-based parsing** - Pattern matching for structured inputs
- **Custom parsers** - Manual parsing for complex formats
- **Instruction parsing** - Command-based problem inputs
- **Data extraction** - Numbers, coordinates, structured data

**Typical AoC Problems**: Following instructions, parsing logs, extracting coordinates, processing commands

#### 🗃️ **State Management Patterns** (`src/state_patterns.rs`)
Complex state tracking and optimization strategies
- **Finite State Machines** - Rule-based transitions
- **Memoization** - Caching for recursive problems
- **State tracking** - Complex multi-variable optimization
- **Optimization patterns** - Dynamic programming, greedy algorithms

**Typical AoC Problems**: Assembly simulation, game state management, optimization challenges, recursive sequences

#### 🎓 **Pattern Trainer** (`src/pattern_trainer.rs`)
Interactive learning system for rapid problem classification
- **Problem classification** - Identify pattern types from descriptions
- **Template selection** - Choose optimal solution approach
- **Performance metrics** - Track recognition speed and accuracy
- **Progressive difficulty** - Build intuition through practice

---

## 🚀 **Quick Start**

### **Run Interactive Pattern Trainer**
```bash
cargo run --example pattern_trainer_demo
```
Learn to classify AoC problems by type in <2 minutes per problem.

### **Explore Pattern Examples**
```bash
# Grid navigation and pathfinding
cargo run --example grid_patterns_demo

# Input parsing techniques  
cargo run --example parsing_patterns_demo

# State management strategies
cargo run --example state_patterns_demo

# Real AoC problem applications
cargo run --example real_aoc_solutions
```

### **Use in Your Solutions**
```rust
use aoc_pattern_recognition::{
    grid_patterns::{Grid, Coord, GridPattern},
    parsing_patterns::InstructionParser,
    state_patterns::MemoizationCache,
};

// Example: Grid-based shortest path problem
let grid = Grid::from_string("..#.\n.#..\n..#.");
let start = Coord { x: 0, y: 0 };
let end = Coord { x: 3, y: 2 };
// Apply BFS/DFS algorithm

// Example: Parse structured input
let parser = InstructionParser;
let instruction = parser.parse_line("move 3 from 2 to 1")?;

// Example: Memoized recursive solution
let mut cache = MemoizationCache::new();
fn solve(n: i32, cache: &mut MemoizationCache) -> i32 {
    // Memoized computation
}
```

---

## 📊 **Pattern Recognition Training**

### **The Core Skill**
> **Fast AoC solvers don't solve problems from scratch - they recognize patterns and apply templates.**

### **Training Workflow**
```
1. Read problem description (30 seconds)
   ↓
2. Identify pattern category (Grid? Parsing? State?)
   ↓  
3. Select solution template (BFS? Regex? Memoization?)
   ↓
4. Adapt template to specific problem (10-12 minutes)
   ↓
5. Submit solution (Target: <15 minutes total)
```

### **Pattern Categories by Frequency**
| Pattern | AoC Frequency | Example Problems |
|---------|---------------|------------------|
| **Grid Traversal** | ~30% | Mazes, pathfinding, cellular automata |
| **Parsing/String** | ~25% | Input transformation, text processing |
| **State Management** | ~20% | Simulation, FSMs, assembly execution |
| **Hash/Counting** | ~15% | Frequency analysis, duplicate detection |
| **Graph Algorithms** | ~10% | Connectivity, topological sort, trees |

---

## 🔗 **Integration with Workspace**

### **Mission Connections**
This library applies concepts from workspace missions:
- **Mission1 (Stack)** - DFS traversal, bracket validation, parsing
- **Mission2 (Queue)** - BFS pathfinding, level-order processing
- **Mission3 (Search)** - Binary search, optimization problems  
- **Mission5 (HashMap)** - Frequency counting, fast lookups, caching
- **Mission6 (Grids)** - 2D navigation, pathfinding, spatial algorithms

### **Learning Track Integration**
- **Daily Study** - Algorithm patterns (Week 4-5)
- **Rust Book** - Ch13 Iterators, Ch10 Generics (template patterns)
- **AoC 2015 Solutions** - Real-world pattern applications in [../aoc2015/](../aoc2015/README.md)

### **Related Resources**
- **[Advanced Examples](../../advanced_examples/README.md)** - Competitive programming utilities
- **[AoC 2015](../aoc2015/README.md)** - Historical solutions using these patterns
- **[Mission6](../../missions/Mission6/README.md)** - Grid and graph implementations

---

## 🧪 **Testing & Validation**

### **Run Tests**
```bash
cargo test
```

### **Run Benchmarks**
```bash
cargo bench
```

### **Test Coverage**
- **Unit tests** - Individual pattern implementations in `tests/`
- **Integration tests** - Complete problem-solving workflows  
- **Performance benchmarks** - Algorithm complexity validation

---

## 📖 **Documentation Standards**

All code follows workspace documentation standards:
- **[RUST_DOCUMENTATION_STANDARDS.md](../../.github/RUST_DOCUMENTATION_STANDARDS.md)** - Module and function docs
- **[RUST_TEST_DOCUMENTATION_STANDARDS.md](../../.github/RUST_TEST_DOCUMENTATION_STANDARDS.md)** - Test organization

Every pattern includes:
- **Time complexity** analysis (Big-O)
- **Space complexity** analysis  
- **Use case guidance** - When to apply this pattern
- **Working examples** - Copy-paste ready code
- **Common pitfalls** - What to watch out for

---

## 🎯 **Requirements Fulfilled**

This library satisfies the following design requirements:
- **REQ-P1**: Grid navigation and manipulation patterns
- **REQ-P2**: Input parsing and transformation patterns
- **REQ-P3**: State management and optimization patterns  
- **REQ-P4**: Performance-aware pattern implementations
- **REQ-P5**: Pattern recognition training and validation

---

## 🏆 **Success Metrics**

After training with this library, you should achieve:
- ✅ **Problem classification** in <2 minutes per problem
- ✅ **Template selection** confidence for all major categories
- ✅ **Solution implementation** in <15 minutes (competitive standard)
- ✅ **Code quality** - Readable, maintainable, reusable patterns
- ✅ **Performance awareness** - Optimal algorithm selection

---

## 🔄 **Examples Directory**

See [examples/README.md](examples/README.md) for detailed demonstrations of each pattern category with real AoC problem applications.

---

## 📝 **Navigation**

### **Zettelkasten Core**
- [[zettel-index]] - Master knowledge graph entry point
- [[AoC Patterns MOC]] - Algorithm pattern catalog
- [[Missions Overview]] - V-Cycle mission implementations

### **Related Projects**
- [AoC 2015 Solutions](../aoc2015/README.md) - Historical problems using these patterns
- [Advanced Examples](../../advanced_examples/README.md) - Competitive programming utilities
- [Mission6](../../missions/Mission6/README.md) - Grid and graph implementations
- [Mission5](../../missions/Mission5/README.md) - HashMap for frequency counting

### **Learning Resources**
- [[Daily Study MOC]] - Progressive algorithm learning
- [[Collections MOC]] - Data structure patterns
- [[Performance Optimization]] - Algorithm complexity analysis

---

*Tags: #aoc-patterns #pattern-recognition #competitive-programming #grid-traversal #parsing #state-management #algorithms #training #bfs #dfs #pathfinding*

---

**🎯 Master pattern recognition to achieve consistent AoC success through systematic preparation and proven algorithmic frameworks.**
