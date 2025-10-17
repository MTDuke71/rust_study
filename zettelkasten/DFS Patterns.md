# 🌲 DFS Patterns - Depth-First Search

**Deep exploration with backtracking for exhaustive search and path finding**

## 🎯 Core Concept

**Depth-First Search (DFS)** is a graph/grid traversal algorithm that explores as far as possible along each branch before backtracking. It uses a **stack** (LIFO) data structure or recursion.

**Key Properties:**
- ✅ **Explores deeply** before exploring widely
- ✅ **Memory efficient** - O(depth) space
- ✅ **Natural for backtracking** problems
- ✅ **Complete** with cycle detection
- ❌ **Does NOT guarantee shortest path**

**When to Use DFS:**
- Exhaustive search needed (find all paths/solutions)
- Backtracking problems (puzzles, constraints)
- Path existence check (any path, not shortest)
- Topological sorting
- Cycle detection
- Memory constrained (uses less memory than BFS)

---

## 📐 DFS Algorithm Templates

### **Recursive DFS (Most Common)**
```rust
use std::collections::HashSet;

fn dfs_recursive(
    grid: &[Vec<bool>],
    current: Point,
    visited: &mut HashSet<Point>,
) {
    // Mark as visited
    visited.insert(current);
    
    // Process current node
    println!("Visiting: {:?}", current);
    
    // Explore neighbors recursively
    for neighbor in get_neighbors(current, grid) {
        if !visited.contains(&neighbor) {
            dfs_recursive(grid, neighbor, visited);
        }
    }
    
    // Backtracking happens automatically on return
}

// Usage:
let mut visited = HashSet::new();
dfs_recursive(&grid, start, &mut visited);
```

### **Iterative DFS with Explicit Stack**
```rust
fn dfs_iterative(grid: &[Vec<bool>], start: Point) {
    let mut stack = vec![start];
    let mut visited = HashSet::new();
    
    while let Some(current) = stack.pop() {
        if visited.contains(&current) {
            continue;
        }
        
        visited.insert(current);
        
        // Process current node
        println!("Visiting: {:?}", current);
        
        // Add neighbors to stack
        for neighbor in get_neighbors(current, grid) {
            if !visited.contains(&neighbor) {
                stack.push(neighbor);
            }
        }
    }
}
```

### **DFS Path Finding**
```rust
fn dfs_find_path(
    grid: &[Vec<bool>],
    current: Point,
    goal: Point,
    visited: &mut HashSet<Point>,
    path: &mut Vec<Point>,
) -> bool {
    // Add current to path
    path.push(current);
    visited.insert(current);
    
    // Check if goal reached
    if current == goal {
        return true;
    }
    
    // Try all neighbors
    for neighbor in get_neighbors(current, grid) {
        if !visited.contains(&neighbor) {
            if dfs_find_path(grid, neighbor, goal, visited, path) {
                return true;  // Found a path!
            }
        }
    }
    
    // Backtrack: remove current from path
    path.pop();
    false
}

// Usage:
let mut visited = HashSet::new();
let mut path = Vec::new();
if dfs_find_path(&grid, start, goal, &mut visited, &mut path) {
    println!("Path found: {:?}", path);
}
```

### **DFS Find All Paths**
```rust
fn dfs_all_paths(
    grid: &[Vec<bool>],
    current: Point,
    goal: Point,
    visited: &mut HashSet<Point>,
    path: &mut Vec<Point>,
    all_paths: &mut Vec<Vec<Point>>,
) {
    path.push(current);
    visited.insert(current);
    
    if current == goal {
        // Found a path - save it
        all_paths.push(path.clone());
    } else {
        // Continue exploring
        for neighbor in get_neighbors(current, grid) {
            if !visited.contains(&neighbor) {
                dfs_all_paths(grid, neighbor, goal, visited, path, all_paths);
            }
        }
    }
    
    // Backtrack
    path.pop();
    visited.remove(&current);
}

// Usage:
let mut visited = HashSet::new();
let mut path = Vec::new();
let mut all_paths = Vec::new();
dfs_all_paths(&grid, start, goal, &mut visited, &mut path, &mut all_paths);
```

---

## 🎨 Common DFS Patterns

### **Pattern 1: Flood Fill (Region Coloring)**
```rust
fn flood_fill(
    grid: &mut [Vec<char>],
    start: Point,
    old_char: char,
    new_char: char,
) {
    let height = grid.len();
    let width = grid[0].len();
    
    fn fill(
        grid: &mut [Vec<char>],
        pos: Point,
        old_char: char,
        new_char: char,
        width: usize,
        height: usize,
    ) {
        // Bounds check
        if pos.x < 0 || pos.x >= width as i32 
            || pos.y < 0 || pos.y >= height as i32 {
            return;
        }
        
        let x = pos.x as usize;
        let y = pos.y as usize;
        
        // Check if this cell needs filling
        if grid[y][x] != old_char {
            return;
        }
        
        // Fill this cell
        grid[y][x] = new_char;
        
        // Recursively fill neighbors
        fill(grid, Point::new(pos.x + 1, pos.y), old_char, new_char, width, height);
        fill(grid, Point::new(pos.x - 1, pos.y), old_char, new_char, width, height);
        fill(grid, Point::new(pos.x, pos.y + 1), old_char, new_char, width, height);
        fill(grid, Point::new(pos.x, pos.y - 1), old_char, new_char, width, height);
    }
    
    fill(grid, start, old_char, new_char, width, height);
}

// Use case: Paint bucket tool, region marking
```

### **Pattern 2: Connected Components**
```rust
fn count_connected_components(grid: &[Vec<bool>]) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut visited = vec![vec![false; width]; height];
    let mut count = 0;
    
    fn dfs(
        grid: &[Vec<bool>],
        pos: Point,
        visited: &mut [Vec<bool>],
        width: usize,
        height: usize,
    ) {
        let x = pos.x as usize;
        let y = pos.y as usize;
        
        if visited[y][x] {
            return;
        }
        
        visited[y][x] = true;
        
        for neighbor in get_neighbors(pos, grid) {
            let nx = neighbor.x as usize;
            let ny = neighbor.y as usize;
            
            if grid[ny][nx] && !visited[ny][nx] {
                dfs(grid, neighbor, visited, width, height);
            }
        }
    }
    
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] && !visited[y][x] {
                dfs(grid, Point::new(x as i32, y as i32), &mut visited, width, height);
                count += 1;
            }
        }
    }
    
    count
}

// Use case: Count islands, count separate regions
```

### **Pattern 3: Backtracking (N-Queens, Sudoku)**
```rust
fn solve_sudoku(board: &mut [Vec<u8>]) -> bool {
    fn is_valid(board: &[Vec<u8>], row: usize, col: usize, num: u8) -> bool {
        // Check row
        if board[row].contains(&num) {
            return false;
        }
        
        // Check column
        if board.iter().any(|r| r[col] == num) {
            return false;
        }
        
        // Check 3x3 box
        let box_row = (row / 3) * 3;
        let box_col = (col / 3) * 3;
        for r in box_row..box_row + 3 {
            for c in box_col..box_col + 3 {
                if board[r][c] == num {
                    return false;
                }
            }
        }
        
        true
    }
    
    fn backtrack(board: &mut [Vec<u8>], row: usize, col: usize) -> bool {
        if row == 9 {
            return true;  // Solved!
        }
        
        let (next_row, next_col) = if col == 8 {
            (row + 1, 0)
        } else {
            (row, col + 1)
        };
        
        if board[row][col] != 0 {
            return backtrack(board, next_row, next_col);
        }
        
        // Try each number 1-9
        for num in 1..=9 {
            if is_valid(board, row, col, num) {
                board[row][col] = num;  // Place
                
                if backtrack(board, next_row, next_col) {
                    return true;  // Solution found
                }
                
                board[row][col] = 0;  // Backtrack
            }
        }
        
        false  // No solution
    }
    
    backtrack(board, 0, 0)
}

// Use case: Constraint satisfaction problems
```

### **Pattern 4: Cycle Detection**
```rust
#[derive(Clone, Copy, PartialEq)]
enum VisitState {
    Unvisited,
    Visiting,   // Currently in DFS stack
    Visited,    // Completed processing
}

fn has_cycle(graph: &HashMap<usize, Vec<usize>>) -> bool {
    let mut state = HashMap::new();
    
    fn dfs(
        node: usize,
        graph: &HashMap<usize, Vec<usize>>,
        state: &mut HashMap<usize, VisitState>,
    ) -> bool {
        state.insert(node, VisitState::Visiting);
        
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                match state.get(&neighbor) {
                    Some(VisitState::Visiting) => return true,  // Cycle!
                    Some(VisitState::Visited) => continue,
                    _ => {
                        if dfs(neighbor, graph, state) {
                            return true;
                        }
                    }
                }
            }
        }
        
        state.insert(node, VisitState::Visited);
        false
    }
    
    for &node in graph.keys() {
        if !state.contains_key(&node) {
            if dfs(node, graph, &mut state) {
                return true;
            }
        }
    }
    
    false
}

// Use case: Detect circular dependencies
```

### **Pattern 5: Topological Sort**
```rust
fn topological_sort(graph: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    
    fn dfs(
        node: usize,
        graph: &HashMap<usize, Vec<usize>>,
        visited: &mut HashSet<usize>,
        stack: &mut Vec<usize>,
    ) {
        visited.insert(node);
        
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    dfs(neighbor, graph, visited, stack);
                }
            }
        }
        
        stack.push(node);  // Add to stack after all descendants
    }
    
    for &node in graph.keys() {
        if !visited.contains(&node) {
            dfs(node, graph, &mut visited, &mut stack);
        }
    }
    
    stack.reverse();  // Reverse for topological order
    stack
}

// Use case: Task scheduling, build systems
```

---

## 🎮 AoC DFS Patterns

### **AoC Pattern 1: Find All Solutions**
```rust
// Generate all valid combinations
fn generate_combinations(
    items: &[i32],
    target: i32,
    current: &mut Vec<i32>,
    all: &mut Vec<Vec<i32>>,
) {
    if current.iter().sum::<i32>() == target {
        all.push(current.clone());
        return;
    }
    
    for &item in items {
        if current.iter().sum::<i32>() + item <= target {
            current.push(item);
            generate_combinations(items, target, current, all);
            current.pop();  // Backtrack
        }
    }
}
```

### **AoC Pattern 2: Path with Constraints**
```rust
// Find path avoiding certain cells
fn find_valid_path(
    grid: &[Vec<char>],
    current: Point,
    goal: Point,
    visited: &mut HashSet<Point>,
    path: &mut Vec<Point>,
) -> bool {
    path.push(current);
    visited.insert(current);
    
    if current == goal {
        return true;
    }
    
    for neighbor in get_neighbors(current, grid) {
        let x = neighbor.x as usize;
        let y = neighbor.y as usize;
        
        // Check constraints
        if grid[y][x] != '#' && !visited.contains(&neighbor) {
            if find_valid_path(grid, neighbor, goal, visited, path) {
                return true;
            }
        }
    }
    
    path.pop();
    false
}
```

### **AoC Pattern 3: Maximum Path Value**
```rust
// Find path with maximum collected value
fn max_path_value(
    grid: &[Vec<i32>],
    current: Point,
    goal: Point,
    visited: &mut HashSet<Point>,
    current_value: i32,
    max_value: &mut i32,
) {
    visited.insert(current);
    let x = current.x as usize;
    let y = current.y as usize;
    let new_value = current_value + grid[y][x];
    
    if current == goal {
        *max_value = (*max_value).max(new_value);
    } else {
        for neighbor in get_neighbors(current, grid) {
            if !visited.contains(&neighbor) {
                max_path_value(grid, neighbor, goal, visited, new_value, max_value);
            }
        }
    }
    
    visited.remove(&current);  // Backtrack
}
```

---

## 📊 DFS vs BFS Comparison

| Aspect | DFS | [[BFS Patterns\|BFS]] |
|--------|-----|-----|
| **Data Structure** | Stack (LIFO) or recursion | Queue (FIFO) |
| **Exploration** | Deep first, backtrack | Layer by layer |
| **Shortest Path** | ❌ No | ✅ Yes (unweighted) |
| **Memory** | O(depth) - **low** | O(width) - high |
| **Completeness** | ✅ Yes (with cycle detection) | ✅ Yes |
| **Use Case** | Exhaustive search, backtracking | Shortest path, levels |
| **Path Found** | Any path (may not be shortest) | Shortest path |

---

## ⚡ Performance Considerations

### **1. Stack Overflow Risk**
```rust
// ❌ Deep recursion can overflow stack
fn deep_dfs_recursive(n: usize) {
    if n > 0 {
        deep_dfs_recursive(n - 1);  // May overflow for large n
    }
}

// ✅ Use iterative DFS for deep graphs
fn deep_dfs_iterative(start: usize, max: usize) {
    let mut stack = vec![start];
    
    while let Some(current) = stack.pop() {
        if current < max {
            stack.push(current + 1);
        }
    }
}
```

### **2. Tail Recursion Optimization**
```rust
// Rust doesn't guarantee tail call optimization
// For very deep recursion, use iterative approach
```

### **3. Memory for Visited Set**
```rust
// For grids: use Vec<Vec<bool>> instead of HashSet
let mut visited = vec![vec![false; width]; height];
```

---

## 🎯 Real-World Applications

1. **Maze Generation** - Generate random mazes with DFS
2. **Puzzle Solving** - Sudoku, N-Queens, crossword
3. **File System Traversal** - Recursively list directories
4. **Expression Evaluation** - Parse and evaluate expressions
5. **Game Tree Search** - Chess, checkers (with alpha-beta pruning)
6. **Compiler Design** - Syntax tree traversal
7. **Dependency Resolution** - Topological sort for build systems

---

## 🚫 Common Pitfalls

### **Pitfall 1: Forgetting to Backtrack**
```rust
// ❌ Wrong: No backtracking
fn wrong_dfs(grid: &[Vec<bool>], current: Point, visited: &mut HashSet<Point>) {
    visited.insert(current);
    // ... explore ...
    // Missing: visited.remove(&current);
}

// ✅ Correct: Backtrack when needed
fn correct_dfs(grid: &[Vec<bool>], current: Point, visited: &mut HashSet<Point>) {
    visited.insert(current);
    // ... explore ...
    visited.remove(&current);  // Backtrack for all-paths search
}
```

### **Pitfall 2: Infinite Recursion**
```rust
// ❌ No base case or cycle detection
fn infinite_dfs(graph: &HashMap<usize, Vec<usize>>, node: usize) {
    for &neighbor in &graph[&node] {
        infinite_dfs(graph, neighbor);  // Will loop if cycle exists!
    }
}

// ✅ Add visited tracking
fn safe_dfs(
    graph: &HashMap<usize, Vec<usize>>,
    node: usize,
    visited: &mut HashSet<usize>,
) {
    visited.insert(node);
    for &neighbor in &graph[&node] {
        if !visited.contains(&neighbor) {
            safe_dfs(graph, neighbor, visited);
        }
    }
}
```

### **Pitfall 3: Stack Overflow**
```rust
// For very deep graphs (>10000 nodes), use iterative DFS
// Rust's default stack size may not handle extreme recursion
```

---

## 🔗 Connected Concepts

### **Related Zettelkasten Pages**
- [[BFS Patterns]] - Breadth-first alternative
- [[A-Star-Algorithm-Deep-Dive]] - Heuristic-guided search
- [[Mission7 Overview]] - Graph algorithms including DFS
- [[Week 4 Overview]] - Day 24 covers DFS for region detection
- [[directed-vs-undirected-graphs]] - Understanding graph types for DFS

### **Related Algorithms**
- **Backtracking**: Subset problems, constraint satisfaction
- **Topological Sort**: Dependency resolution
- **Strongly Connected Components**: Tarjan's algorithm (uses DFS)
- **Minimum Spanning Tree**: Kruskal's algorithm

---

## 💡 Key Takeaways

1. **DFS = Go Deep**: Explore paths fully before backtracking
2. **Memory Efficient**: O(depth) space vs BFS's O(width)
3. **Natural for Backtracking**: Recursive structure suits constraint problems
4. **No Shortest Path**: First path found may not be optimal
5. **Stack or Recursion**: Two equivalent implementations
6. **Cycle Detection Essential**: Prevent infinite loops

**When to Use DFS:**
```
✅ Need to find ALL paths/solutions
✅ Backtracking problems (Sudoku, N-Queens)
✅ Exhaustive search required
✅ Memory constrained (deep but narrow graphs)
✅ Topological sorting
✅ Cycle detection
✅ Connected components

❌ Need shortest path → Use BFS or A*
❌ Wide graphs → Use BFS (better memory)
❌ Level-order traversal → Use BFS
❌ Very deep graphs → Use iterative DFS
```

**DFS Philosophy:**
> "Sometimes you need to go deep to find what you're looking for. DFS commits to each path fully before trying another - perfect for exploration and backtracking." 🌲

---

*Tags: #dfs #depth-first-search #backtracking #recursion #exhaustive-search #graph-algorithms #tree-traversal #aoc-patterns*

*Links: [[zettel-index]] | [[BFS Patterns]] | [[A-Star-Algorithm-Deep-Dive]] | [[Mission7 Overview]] | [[Week 4 Overview]]*
