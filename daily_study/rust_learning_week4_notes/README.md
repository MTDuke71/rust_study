# Week 4: Applied Problem Solving (Days 22-28)

**Focus**: Grids, Navigation, Pathfinding, Queues, and String Parsing

---

## 📚 **Week Overview**

Week 4 applies all previous concepts to solve complex, real-world problems. This week focuses on algorithms commonly used in competitive programming (Advent of Code), game development, and practical software engineering.

### **Core Themes**
- 🗺️ **Grid Systems** - 2D data structures and coordinate systems
- 🧭 **Navigation** - Direction handling and movement patterns
- 🌊 **Flood Fill** - Area exploration algorithms
- 🔍 **BFS/DFS** - Graph traversal and pathfinding
- 📜 **String Parsing** - Input processing and validation
- ⚡ **Priority Queues** - Dijkstra and A* algorithms

---

## 🗓️ **Daily Breakdown**

### [[daily-study/Day22]] or [[ds-day22]] - 2D Grid Fundamentals
**Topics**: Grid representation, coordinate systems, indexing
- Row-major vs column-major layout
- Boundary checking patterns
- Grid initialization strategies

### [[daily-study/Day23]] or [[ds-day23]] - Direction and Navigation
**Topics**: Cardinal directions, movement vectors, rotation
- Direction enums (North, South, East, West)
- Applying movement deltas
- 90-degree rotations

### [[daily-study/Day24]] or [[ds-day24]] - Flood Fill Algorithms
**Topics**: Recursive and iterative flood fill, connected components
- Stack-based flood fill (DFS)
- Queue-based flood fill (BFS)
- Counting regions and area calculation

### [[Day25]] - Breadth-First Search (BFS)
**Topics**: Level-order traversal, shortest path, distance calculation
- Queue-based BFS implementation
- Visited set management
- Distance tracking

### [[Day26]] - Depth-First Search (DFS)
**Topics**: Recursive exploration, backtracking, cycle detection
- Stack-based (iterative) DFS
- Recursive DFS patterns
- Topological sorting

### [[Day27]] - Priority Queues and Dijkstra
**Topics**: `BinaryHeap`, weighted graphs, shortest path algorithms
- Min-heap vs max-heap
- Dijkstra's algorithm implementation
- A* heuristic search

### [[Day28]] - String Parsing and Validation
**Topics**: Input processing, regex patterns, custom parsers
- Splitting and tokenization
- Validation with pattern matching
- Building custom parsers

---

## 🎯 **Learning Objectives**

By the end of Week 4, you should be able to:
- ✅ Implement 2D grid systems with efficient indexing
- ✅ Navigate grids with direction enums and movement patterns
- ✅ Apply flood fill for area exploration
- ✅ Implement BFS for shortest path problems
- ✅ Use DFS for deep exploration and backtracking
- ✅ Solve weighted graph problems with Dijkstra/A*
- ✅ Parse and validate complex input formats

---

## 🔗 **Related Missions**

### **Mission 6: Grid Systems and Pathfinding**
**Connection**: Core 2D data structures and algorithms
- [[mission-6]] - Complete grid implementation
- BFS/DFS implementations on grids

### **Mission 2: Ring Buffer Queue**
**Connection**: Efficient queue for BFS algorithms
- [[../../missions/Mission2/README|Mission2 README]] - Circular buffer design
- Performance-optimized BFS

### **Mission 3: Search Algorithms**
**Connection**: Binary search and search patterns
- [[../../missions/Mission3/README|Mission3 README]] - Search algorithm catalog

---

## 📖 **Rust Book Integration**

Week 4 applies concepts from earlier chapters:
- **[[../../rust_book/Ch8/README|Chapter 8]]** - Strings and collections in parsing
- **[[../../rust_book/Ch10/README|Chapter 10]]** - Generic grid and algorithm implementations
- **[[../../rust_book/Ch18/README|Chapter 18]]** - Patterns in input validation

---

## 🎮 **Advent of Code Applications**

Week 4 patterns are ESSENTIAL for AoC success:

### **Grid Problems** (40% of AoC)
- Maze navigation
- Conway's Game of Life
- Warehouse robot pathfinding
- See [[../../advent_of_code/aoc2015/README|AoC 2015 Days 3, 18]]

### **Graph Problems** (30% of AoC)
- Shortest path problems
- Network traversal
- Connectivity analysis

### **Parsing Problems** (20% of AoC)
- Input format processing
- Validation and error handling
- Building data structures from text

---

## 🚀 **Running Week 4 Examples**

All Day files contain complete runnable examples with AoC-style problems:

```powershell
# Run individual day
.\scripts\run_md.bat daily_study\rust_learning_week4_notes\Day24.md

# Run all Week 4 examples
Get-ChildItem daily_study\rust_learning_week4_notes\Day*.md | 
    ForEach-Object { .\run_markdown_code.ps1 $_.FullName }
```

---

## 🔗 **Navigation**

- **⬅️ [[../rust_learning_week3_notes/README|Week 3: Abstractions]]** - Previous week
- **📚 [[../README|Daily Study Home]]** - All weeks overview
- **🗺️ [[../../zettelkasten/Daily Study MOC]]** - Complete study navigation
- **📅 [[../../MONTHLY_CALENDAR]]** - 30-day learning plan

---

## 🎓 **Key Takeaways**

Week 4 demonstrates **algorithms in action**:

> **"Data structures + algorithms = problem-solving power"**

### **Algorithm Patterns:**

**Grid Navigation Pattern:**
```rust
const DIRS: [(i32, i32); 4] = [(0,1), (1,0), (0,-1), (-1,0)];
for (dr, dc) in DIRS {
    let nr = row + dr;
    let nc = col + dc;
    if grid.in_bounds(nr, nc) { /* explore */ }
}
```

**BFS Template:**
```rust
let mut queue = VecDeque::new();
let mut visited = HashSet::new();
queue.push_back(start);
while let Some(current) = queue.pop_front() {
    for neighbor in get_neighbors(current) {
        if !visited.contains(&neighbor) {
            visited.insert(neighbor);
            queue.push_back(neighbor);
        }
    }
}
```

**DFS Template:**
```rust
fn dfs(node: Node, visited: &mut HashSet<Node>) {
    if visited.contains(&node) { return; }
    visited.insert(node);
    for neighbor in get_neighbors(node) {
        dfs(neighbor, visited);
    }
}
```

### **Performance Considerations:**
- ✅ **Grid indexing**: O(1) with proper layout
- ✅ **BFS shortest path**: O(V + E) with queue
- ✅ **DFS exploration**: O(V + E) with stack/recursion
- ✅ **Dijkstra**: O((V + E) log V) with binary heap

### **Real-World Applications:**
- 🎮 **Game Dev** - Pathfinding, FOV, procedural generation
- 🗺️ **Mapping** - Route planning, area calculation
- 🌐 **Networks** - Graph analysis, connectivity
- 🧩 **Puzzles** - AoC, competitive programming

Master these patterns, and you're ready for any algorithmic challenge! 🚀

---

*Tags: #week4 #grids #bfs #dfs #pathfinding #algorithms #aoc #daily-study*
