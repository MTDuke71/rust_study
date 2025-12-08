# Union-Find Patterns and Applications

**Tags:** #aoc #competitive-programming #union-find #patterns #problem-solving #graph #connectivity  
**Created:** 2025-12-08  
**Related:** [[union-find-algorithm]], [[mission-10]], [[aoc-day-08]], [[graph-algorithms]], [[AoC Patterns MOC]]

---

## 🎯 Overview

This note catalogs **common problem patterns** that map to Union-Find (Disjoint Set Union) data structures, with practical examples from Advent of Code and competitive programming. For algorithmic details, see [[union-find-algorithm]].

**When to recognize Union-Find**: Problems involving **connectivity**, **grouping**, **equivalence relations**, or **dynamic set merging**.

---

## 🧩 Core Pattern Recognition

### Pattern 1: **Connectivity Queries**

**Problem signals:**
- "Are X and Y connected?"
- "How many separate groups/components exist?"
- "Can you reach Y from X?"
- Dynamic edge additions with connectivity checks

**Examples:**
- AoC 2025 Day 8: Junction box circuits
- Network connectivity (routers, cities, friends)
- Social network friend circles

**Template:**
```rust
let mut uf = UnionFind::new(n);

// Process connections
for (a, b) in connections {
    uf.union(a, b);
}

// Query connectivity
if uf.connected(x, y) {
    println!("Connected!");
}

// Count components
println!("Groups: {}", uf.count());
```

**AoC 2025 Day 8 Application:**
```rust
// Connect 1000 closest junction box pairs
let mut uf = UnionFind::new(boxes.len());
for (box_a, box_b, _dist) in closest_pairs.iter().take(1000) {
    if uf.find(*box_a) != uf.find(*box_b) {
        uf.union(*box_a, *box_b);
    }
}

// Find circuit sizes
let sizes: Vec<usize> = uf.components()
    .map(|comp| comp.len())
    .collect();
```

---

### Pattern 2: **Cycle Detection in Undirected Graphs**

**Problem signals:**
- "Does adding this edge create a cycle?"
- "Is this graph a forest (acyclic)?"
- Building spanning trees

**Key insight:** Edge `(u,v)` creates a cycle **if and only if** `u` and `v` are already connected.

**Template:**
```rust
fn has_cycle(edges: &[(usize, usize)], n: usize) -> bool {
    let mut uf = UnionFind::new(n);
    
    for &(u, v) in edges {
        if uf.connected(u, v) {
            return true; // cycle detected!
        }
        uf.union(u, v);
    }
    false
}
```

**Application: Kruskal's MST**
```rust
// Build minimum spanning tree
edges.sort_by_key(|e| e.weight);
let mut uf = UnionFind::new(n);
let mut mst = Vec::new();

for edge in edges {
    if uf.union(edge.u, edge.v) { // union returns false if already connected
        mst.push(edge); // safe to add - no cycle
        if mst.len() == n - 1 { break; } // MST complete
    }
}
```

---

### Pattern 3: **Connected Component Counting**

**Problem signals:**
- "How many separate islands/regions?"
- "Count distinct groups"
- "Number of independent networks"

**Template:**
```rust
fn count_components(edges: &[(usize, usize)], n: usize) -> usize {
    let mut uf = UnionFind::new(n);
    
    for &(u, v) in edges {
        uf.union(u, v);
    }
    
    uf.count() // number of disjoint sets
}
```

**2D Grid variant (islands):**
```rust
// Count islands in grid (connected '1's, separated by '0's)
fn count_islands(grid: &[Vec<char>]) -> usize {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut uf = UnionFind::new(rows * cols);
    
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '1' {
                let idx = r * cols + c;
                // Union with right neighbor
                if c + 1 < cols && grid[r][c+1] == '1' {
                    uf.union(idx, idx + 1);
                }
                // Union with bottom neighbor
                if r + 1 < rows && grid[r+1][c] == '1' {
                    uf.union(idx, idx + cols);
                }
            }
        }
    }
    
    // Count unique roots of '1' cells
    (0..rows*cols)
        .filter(|&i| grid[i/cols][i%cols] == '1' && uf.find(i) == i)
        .count()
}
```

---

### Pattern 4: **Equivalence Classes / Grouping**

**Problem signals:**
- "Group items with same property"
- "Merge equivalent sets"
- "Transitivity: if A=B and B=C, then A=C"

**Example: Email account merging**
```rust
// Given: [["john@a.com", "john@b.com"], ["john@b.com", "john@c.com"]]
// Output: Same person owns all three emails

fn merge_accounts(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut email_to_id = HashMap::new();
    let mut uf = UnionFind::new(accounts.len());
    
    // Map emails to account IDs
    for (id, account) in accounts.iter().enumerate() {
        for email in &account[1..] { // skip name
            if let Some(&prev_id) = email_to_id.get(email) {
                uf.union(id, prev_id); // merge accounts
            }
            email_to_id.insert(email.clone(), id);
        }
    }
    
    // Group emails by root account
    let mut components: HashMap<usize, Vec<String>> = HashMap::new();
    for (email, id) in email_to_id {
        let root = uf.find(id);
        components.entry(root).or_default().push(email);
    }
    
    components.into_values()
        .map(|mut emails| {
            emails.sort();
            emails
        })
        .collect()
}
```

---

### Pattern 5: **Partial/Progressive Connectivity**

**Problem signals:**
- "After processing K connections, what's the state?"
- "How many connections needed to reach N components?"
- "Process connections in order, tracking intermediate states"

**AoC 2025 Day 8 demonstrates this:**
```rust
// Part 1: After 1000 connections, count components
fn solve_part1(boxes: &[JunctionBox]) -> usize {
    let mut pairs = get_all_pairs(boxes);
    pairs.sort_by_key(|&(_, _, dist_sq)| dist_sq); // by distance
    
    let mut uf = UnionFind::new(boxes.len());
    
    // Examine first 1000 pairs (might make fewer connections if redundant)
    for (i, &(a, b, _)) in pairs.iter().enumerate() {
        if i >= 1000 { break; }
        uf.union(a, b); // may skip if already connected
    }
    
    // Extract component sizes
    let mut sizes: Vec<usize> = uf.components()
        .map(|c| c.len())
        .collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    
    sizes[0] * sizes[1] * sizes[2] // product of 3 largest
}

// Part 2: Continue until fully connected (1 component)
fn solve_part2(boxes: &[JunctionBox]) -> (usize, usize) {
    let mut pairs = get_all_pairs(boxes);
    pairs.sort_by_key(|&(_, _, dist_sq)| dist_sq);
    
    let mut uf = UnionFind::new(boxes.len());
    let mut num_components = boxes.len();
    
    for &(a, b, _) in &pairs {
        if uf.find(a) != uf.find(b) {
            uf.union(a, b);
            num_components -= 1;
            
            if num_components == 1 {
                // Return last connection for Part 2 answer
                return (boxes[a].x, boxes[b].x);
            }
        }
    }
    unreachable!()
}
```

**Key pattern**: Track `num_components` manually, decrement on successful union, stop at target.

---

### Pattern 6: **Weighted Union-Find** (Extension)

**When regular Union-Find isn't enough:**
- "What's the distance/difference between X and Y?"
- "Maintain relationships: A is 5 units bigger than B"
- Disjoint Set with values

**Template (with distances):**
```rust
struct WeightedUnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    dist: Vec<i64>,  // distance from node to parent
}

impl WeightedUnionFind {
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let orig_parent = self.parent[x];
            self.parent[x] = self.find(orig_parent);
            self.dist[x] += self.dist[orig_parent]; // path compression updates distance
        }
        self.parent[x]
    }
    
    // Returns false if creates inconsistency
    fn union(&mut self, x: usize, y: usize, diff: i64) -> bool {
        // diff represents: weight[y] = weight[x] + diff
        let root_x = self.find(x);
        let root_y = self.find(y);
        
        if root_x == root_y {
            // Check consistency
            return self.dist[y] - self.dist[x] == diff;
        }
        
        self.parent[root_y] = root_x;
        self.dist[root_y] = self.dist[x] + diff - self.dist[y];
        true
    }
}
```

---

## 🚀 Performance Optimization: Squared Distances

**Pattern:** When only **relative ordering** matters (not absolute values)

**AoC 2025 Day 8 optimization study:**

### Original (with sqrt):
```rust
fn calculate_distance(a: &JunctionBox, b: &JunctionBox) -> f64 {
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    let dz = (a.z - b.z) as f64;
    (dx*dx + dy*dy + dz*dz).sqrt() // 👈 expensive!
}
```

### Optimized (squared distance):
```rust
fn calculate_distance_squared(a: &JunctionBox, b: &JunctionBox) -> i64 {
    let dx = (a.x - b.x) as i64;
    let dy = (a.y - b.y) as i64;
    let dz = (a.z - b.z) as i64;
    dx*dx + dy*dy + dz*dz // 👈 no sqrt!
}
```

**Benchmark results (1000 boxes, 499,500 pairs):**
- Part 1: 24.31ms → 21.64ms (**11% faster**)
- Part 2: 24.32ms → 21.58ms (**11% faster**)

**Why it works:**
- **Monotonicity preserved**: If `a² < b²` then `a < b` (for positive numbers)
- Sorting by `d²` gives **same ordering** as sorting by `d`
- Eliminates 499,500 expensive `sqrt()` calls (10-20x slower than multiply)
- `i64` arithmetic often faster than `f64` on modern CPUs

**When to apply:**
- ✅ Need relative ordering (sorting, min/max, comparisons)
- ✅ Distance-based greedy algorithms (closest pair, MST)
- ❌ Need actual distance values (sum of distances, threshold checks)

**Bonus:** Can use cleaner `sort_by_key(|&(_, _, d)| d)` instead of `sort_by(|a,b| a.2.partial_cmp(&b.2))`

---

## 🎯 Problem-Solving Checklist

**Recognize Union-Find when you see:**

- ✅ **Connectivity**: "Are these connected?" "Can you reach B from A?"
- ✅ **Grouping**: "How many groups?" "Which group is X in?"
- ✅ **Merging**: "Combine these sets" "Merge equivalence classes"
- ✅ **Dynamic edges**: "Add connection" "Process edges in order"
- ✅ **Cycle detection**: "Does this create a cycle?" (undirected graphs only)
- ✅ **MST problems**: Kruskal's algorithm uses Union-Find for cycle detection
- ✅ **Equivalence relations**: Transitivity (A~B, B~C ⟹ A~C)

**Implementation checklist:**

1. **Initialize**: `UnionFind::new(n)` with n elements
2. **Process connections**: Call `union(a, b)` for each relationship
3. **Query as needed**:
   - Connectivity: `connected(x, y)` or `find(x) == find(y)`
   - Component count: `count()`
   - Component membership: `find(x)` (returns root/representative)
   - Component sizes: Maintain separate `size` array or use `components()` iterator

4. **Optimize if needed**:
   - Path compression (usually built-in)
   - Union by rank/size (usually built-in)
   - For distance problems: Use squared distances when only ordering matters

---

## 📚 AoC Examples

### AoC 2025 Day 8: Junction Box Connectivity

**Problem:** Connect junction boxes by Euclidean distance using greedy selection
- **Part 1:** Examine 1000 closest pairs, find product of 3 largest circuits
- **Part 2:** Continue until all boxes form one circuit, return last connection

**Pattern recognition:**
- Connectivity tracking → Union-Find
- Greedy edge selection by distance → Sort pairs
- Progressive connection → Track component count
- Need circuit sizes → `components()` iterator

**Key insights:**
- "Examine 1000 pairs" ≠ "Make 1000 connections" (some already connected)
- Result: Examined 1000, made 698 connections (302 redundant)
- Performance: Squared distances 12% faster than sqrt

**Mission 10 validation:** Perfect real-world demonstration of DSU's power

---

## 🔗 Related Patterns

- **DFS/BFS for connectivity**: Use when graph is static and single-query
- **Union-Find**: Use when graph is dynamic or many connectivity queries
- **Tarjan's SCC**: Use for directed graph strongly connected components
- **Topological sort**: Use for DAG ordering problems

**Union-Find vs DFS comparison:**

| Scenario | Union-Find | DFS/BFS |
|----------|-----------|---------|
| Static graph, one query | Overkill | ✅ Better |
| Static graph, many queries | ✅ Better (after O(n) setup) | Slow (O(n) per query) |
| Dynamic edges (online) | ✅ Better (O(α(n)) per edge) | Slow (rebuild each time) |
| Directed graph connectivity | Limited | ✅ Better |
| Undirected connectivity | ✅ Better | Works but slower |
| Cycle detection (undirected) | ✅ Better | Works |
| Cycle detection (directed) | ❌ Can't do | ✅ Better |

---

## 💡 Common Pitfalls

1. **Using with directed graphs**: Union-Find only works for **undirected** connectivity
   - For directed: Use DFS/BFS or Tarjan's algorithm

2. **Forgetting to call `find()` before comparing**:
   ```rust
   // ❌ Wrong - compares indices, not roots
   if parent[x] == parent[y] { ... }
   
   // ✅ Correct - compares roots
   if uf.find(x) == uf.find(y) { ... }
   ```

3. **Modifying during iteration over components**:
   ```rust
   // ❌ Problematic - modifying uf while iterating
   for component in uf.components() {
       uf.union(component[0], something); // BAD!
   }
   
   // ✅ Collect first, then modify
   let components: Vec<_> = uf.components().collect();
   for component in components {
       uf.union(component[0], something);
   }
   ```

4. **Not handling `union` return value** when cycle detection matters:
   ```rust
   // ❌ Ignores whether edge created cycle
   uf.union(u, v);
   
   // ✅ Check if union was successful (no cycle)
   if uf.union(u, v) {
       mst.push(edge); // safe to add
   }
   ```

5. **Assuming absolute distance values when only ordering matters**:
   ```rust
   // ❌ Unnecessary sqrt() calls
   let dist = ((dx*dx + dy*dy + dz*dz) as f64).sqrt();
   pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
   
   // ✅ Use squared distances for 10-20% speedup
   let dist_sq = dx*dx + dy*dy + dz*dz; // i64
   pairs.sort_by_key(|&(_, _, d)| d); // cleaner too!
   ```

---

## 🎓 Learning Resources

- **Theory**: [[union-find-algorithm]] - Detailed algorithm explanation
- **Implementation**: [[mission-10]] - Production-ready Rust implementation
- **Practice**: [[aoc-day-08]] - Real problem walkthrough
- **Patterns**: [[AoC Patterns MOC]] - Broader problem-solving patterns

**Next steps:**
1. Review [[union-find-algorithm]] for implementation details
2. Solve AoC 2025 Day 8 using [[mission-10]]
3. Practice on: LeetCode 547, 684, 685, 721, 990, 1319
4. Apply to Kruskal's MST algorithm

---

**Related Links:**
- [[union-find-algorithm]]
- [[mission-10]]
- [[aoc-day-08]]
- [[graph-algorithms]]
- [[AoC Patterns MOC]]
- [[competitive-programming-patterns]]
- [[performance-optimization-patterns]]

---

*Tags: #aoc #union-find #patterns #competitive-programming #connectivity #data-structures #optimization*
