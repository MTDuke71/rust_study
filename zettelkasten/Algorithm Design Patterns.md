# Algorithm Design Patterns

*Foundational strategies for solving computational problems efficiently*

---

## Overview

Algorithm design patterns are reusable strategies for solving classes of computational problems. Unlike data structure patterns (which focus on *storage*), algorithm patterns focus on *process* - the systematic approaches to transforming inputs into outputs efficiently.

**Key Principle**: Match the problem structure to the algorithm pattern, not the other way around.

---

## 🎯 Pattern Categories

### **1. Divide and Conquer**

Split problem into independent subproblems, solve recursively, combine results.

```rust
// Classic example: Merge Sort
fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 { return; }
    
    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);  // Divide
    merge_sort(&mut arr[mid..]);  // Divide
    
    // Conquer: merge sorted halves
    let merged = merge(&arr[..mid], &arr[mid..]);
    arr.copy_from_slice(&merged);
}
```

**When to Use**:
- Problem can be split into independent parts
- Subproblems have same structure as original
- Combining solutions is straightforward

**Examples in This Workspace**:
- [[Binary Search]] - O(log n) lookup
- [[Quick Sort]] - Average O(n log n) sorting
- [[Merge Sort]] - Guaranteed O(n log n) with stability

---

### **2. Greedy Algorithms**

Make locally optimal choice at each step, hoping for global optimum.

```rust
// Example: Coin change (when greedy works)
fn min_coins_greedy(amount: u32, denominations: &[u32]) -> Vec<u32> {
    let mut remaining = amount;
    let mut result = Vec::new();
    
    // Sort denominations descending
    let mut sorted = denominations.to_vec();
    sorted.sort_by(|a, b| b.cmp(a));
    
    for &coin in &sorted {
        while remaining >= coin {
            result.push(coin);
            remaining -= coin;
        }
    }
    result
}
```

**When to Use**:
- Problem has **greedy choice property**: local optimum leads to global optimum
- Problem has **optimal substructure**: optimal solution contains optimal solutions to subproblems

**⚠️ Caution**: Greedy doesn't always work! Coin change fails for denominations like [1, 3, 4] when amount=6.

**Examples in This Workspace**:
- [[mission-9]] - Dijkstra's algorithm (greedy on shortest known distance)
- [[kahns-topological-sort]] - Process nodes with zero in-degree first
- AoC interval scheduling problems

---

### **3. Dynamic Programming (DP)**

Break problem into overlapping subproblems, cache results to avoid recomputation.

```rust
// Example: Fibonacci with memoization
fn fib_memo(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 { return n; }
    
    if let Some(&result) = cache.get(&n) {
        return result;  // Cached!
    }
    
    let result = fib_memo(n - 1, cache) + fib_memo(n - 2, cache);
    cache.insert(n, result);
    result
}

// Or tabulation (bottom-up)
fn fib_table(n: usize) -> u64 {
    if n <= 1 { return n as u64; }
    
    let mut dp = vec![0u64; n + 1];
    dp[1] = 1;
    
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n]
}
```

**When to Use**:
- **Overlapping subproblems**: Same subproblem solved multiple times
- **Optimal substructure**: Optimal solution built from optimal subsolutions

**Top-Down (Memoization) vs Bottom-Up (Tabulation)**:
| Aspect | Memoization | Tabulation |
|--------|-------------|------------|
| Approach | Recursive + cache | Iterative + array |
| Order | Solves what's needed | Solves all subproblems |
| Space | Often less (only needed) | Fixed (all states) |
| Debugging | Harder (recursion) | Easier (iteration) |

**Examples in This Workspace**:
- [[AoC Patterns MOC]] - Many AoC problems use DP
- Path counting problems
- Knapsack-style optimization

---

### **4. Backtracking**

Explore all possibilities systematically, abandoning paths that can't lead to solution.

```rust
// Example: N-Queens
fn solve_n_queens(n: usize) -> Vec<Vec<usize>> {
    let mut solutions = Vec::new();
    let mut board = vec![0; n];  // board[row] = column of queen
    
    fn backtrack(row: usize, board: &mut Vec<usize>, solutions: &mut Vec<Vec<usize>>) {
        if row == board.len() {
            solutions.push(board.clone());  // Found valid solution!
            return;
        }
        
        for col in 0..board.len() {
            if is_safe(row, col, board) {
                board[row] = col;
                backtrack(row + 1, board, solutions);
                // Implicit backtrack: next iteration overwrites board[row]
            }
        }
    }
    
    backtrack(0, &mut board, &mut solutions);
    solutions
}
```

**When to Use**:
- Need to find ALL solutions (or first valid solution)
- Solution is built incrementally
- Can detect invalid partial solutions early (pruning)

**Examples in This Workspace**:
- Sudoku solvers
- Maze pathfinding (all paths)
- Constraint satisfaction problems

---

### **5. Graph Traversal Patterns**

Systematic exploration of graph structures.

#### **BFS (Breadth-First Search)**
```rust
fn bfs(graph: &Graph, start: NodeId) -> Vec<NodeId> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut order = Vec::new();
    
    queue.push_back(start);
    visited.insert(start);
    
    while let Some(node) = queue.pop_front() {
        order.push(node);
        
        for neighbor in graph.neighbors(node) {
            if visited.insert(neighbor) {
                queue.push_back(neighbor);
            }
        }
    }
    order
}
```

**Use BFS for**: Shortest path (unweighted), level-order, minimum steps

#### **DFS (Depth-First Search)**
```rust
fn dfs(graph: &Graph, start: NodeId) -> Vec<NodeId> {
    let mut visited = HashSet::new();
    let mut stack = vec![start];
    let mut order = Vec::new();
    
    while let Some(node) = stack.pop() {
        if visited.insert(node) {
            order.push(node);
            
            for neighbor in graph.neighbors(node) {
                if !visited.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }
    }
    order
}
```

**Use DFS for**: Cycle detection, topological sort, connected components, path existence

**Examples in This Workspace**:
- [[mission-7]] - Graph foundations
- [[mission-8]] - BFS/DFS implementations
- [[graph-algorithms]] - Comprehensive reference

---

### **6. Two Pointers / Sliding Window**

Process sequences with coordinated pointer movement.

```rust
// Two pointers: Find pair summing to target in sorted array
fn two_sum_sorted(arr: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    
    while left < right {
        let sum = arr[left] + arr[right];
        match sum.cmp(&target) {
            Ordering::Equal => return Some((left, right)),
            Ordering::Less => left += 1,
            Ordering::Greater => right -= 1,
        }
    }
    None
}

// Sliding window: Maximum sum subarray of size k
fn max_sum_window(arr: &[i32], k: usize) -> i32 {
    let mut window_sum: i32 = arr[..k].iter().sum();
    let mut max_sum = window_sum;
    
    for i in k..arr.len() {
        window_sum += arr[i] - arr[i - k];  // Slide window
        max_sum = max_sum.max(window_sum);
    }
    max_sum
}
```

**When to Use**:
- Sequential data with ordered property
- Finding pairs, subarrays, or subsequences
- O(n) instead of O(n²) nested loops

---

### **7. Union-Find (Disjoint Sets)**

Efficiently track connected components with near-constant operations.

```rust
// See Mission 10 for complete implementation
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);  // Path compression
        }
        self.parent[x]
    }
    
    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        
        if px != py {
            // Union by rank
            match self.rank[px].cmp(&self.rank[py]) {
                Ordering::Less => self.parent[px] = py,
                Ordering::Greater => self.parent[py] = px,
                Ordering::Equal => {
                    self.parent[py] = px;
                    self.rank[px] += 1;
                }
            }
        }
    }
}
```

**When to Use**:
- Dynamic connectivity queries
- Kruskal's MST algorithm
- Connected components in streaming data

**Examples in This Workspace**:
- [[mission-10]] - Complete Union-Find implementation
- [[union-find-algorithm]] - Deep dive analysis

---

## 🔄 Pattern Selection Guide

| Problem Type | Primary Pattern | Why |
|--------------|-----------------|-----|
| Sorted array search | Binary Search | O(log n) vs O(n) |
| Shortest path (unweighted) | BFS | Guarantees minimum steps |
| Shortest path (weighted, non-negative) | Dijkstra (Greedy) | Optimal with priority queue |
| All pairs shortest | Floyd-Warshall (DP) | O(V³) but handles negatives |
| Optimization with constraints | DP | Optimal substructure + overlap |
| Find all valid configurations | Backtracking | Systematic exploration |
| Component connectivity | Union-Find | Near O(1) per operation |
| Dependency ordering | Topological Sort (Kahn's) | Handles DAGs correctly |

---

## 🧩 Combining Patterns

Real problems often require pattern combinations:

### **A* = Dijkstra + Heuristic**
```rust
// A* combines greedy (heuristic) with DP-like (optimal path tracking)
fn a_star(graph: &Graph, start: Node, goal: Node, heuristic: fn(Node) -> Cost) {
    // f(n) = g(n) + h(n)
    // g(n) = actual cost from start (Dijkstra-like)
    // h(n) = estimated cost to goal (heuristic/greedy)
}
```

### **BFS + Memoization**
For shortest path with state beyond just position:
```rust
// State = (position, keys_collected)
// Memoize to avoid revisiting same (position, keys) combinations
```

**Examples in This Workspace**:
- [[mission-9]] - A* and Dijkstra integration
- [[A-Star-Algorithm-Deep-Dive|A* Algorithm]] - Detailed heuristic analysis

---

## 📊 Complexity Quick Reference

| Pattern | Time | Space | Notes |
|---------|------|-------|-------|
| Binary Search | O(log n) | O(1) | Requires sorted input |
| Merge Sort | O(n log n) | O(n) | Stable, predictable |
| Dijkstra | O(E log V) | O(V) | Non-negative weights |
| BFS/DFS | O(V + E) | O(V) | Graph traversal |
| DP (typical) | O(n²) to O(n³) | O(n) to O(n²) | State-dependent |
| Union-Find | O(α(n)) ≈ O(1) | O(n) | With path compression |
| Backtracking | O(b^d) | O(d) | b=branching, d=depth |

---

## 🦀 Rust-Specific Considerations

### **Ownership in Recursive Algorithms**
```rust
// Use references or indices instead of moving values
fn dfs(graph: &Graph, node: usize, visited: &mut HashSet<usize>) {
    // Graph is borrowed, not moved
}
```

### **Iterator Patterns**
```rust
// Prefer iterators for functional algorithm expression
let sum: i32 = (1..=n).filter(|x| x % 2 == 0).sum();
```

### **Entry API for Memoization**
```rust
// Efficient cache management
let result = *cache.entry(key).or_insert_with(|| expensive_compute(key));
```

---

## Related Notes

**Foundations**:
- [[Big-O Analysis]] - Complexity analysis fundamentals
- [[Algorithm Analysis]] - Performance measurement techniques
- [[Data Structure Patterns]] - Storage-focused patterns

**Implementations**:
- [[graph-algorithms]] - Graph-specific algorithm details
- [[mission-7]] - Graph fundamentals
- [[mission-8]] - BFS/DFS implementations
- [[mission-9]] - Pathfinding algorithms
- [[mission-10]] - Union-Find implementation
- [[kahns-topological-sort]] - Dependency ordering

**Applications**:
- [[AoC Patterns MOC]] - Problem-solving pattern recognition
- [[Performance Analysis]] - Real-world optimization

---

*Tags: #algorithms #design-patterns #problem-solving #optimization #complexity #foundations*

*Links: [[Big-O Analysis]] | [[Algorithm Analysis]] | [[graph-algorithms]] | [[mission-9]] | [[mission-10]] | [[kahns-topological-sort]] | [[union-find-algorithm]] | [[AoC Patterns MOC]] | [[Data Structure Patterns]] | [[zettel-index]]*
