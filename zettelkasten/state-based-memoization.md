# State-Based Memoization

**Core Concept**: Caching computed results based on state representation to avoid redundant computation in recursive algorithms. The key insight is choosing the right state representation - too simple misses optimization opportunities, too complex wastes memory.

## State Representation Spectrum

### Simple State (Single Variable)
**When to use**: Current position/node is sufficient to determine subproblem result
**Example**: Count paths from node to target in DAG (Day 11 Part 1)

```rust
// State = current node only
fn count_paths(
    current: &str,
    target: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if current == target {
        return 1;
    }
    
    // Check cache - state is just the node
    if let Some(&cached) = memo.get(current) {
        return cached;
    }
    
    let count = graph.get(current)
        .map(|neighbors| {
            neighbors.iter()
                .map(|next| count_paths(next, target, graph, memo))
                .sum()
        })
        .unwrap_or(0);
    
    memo.insert(current.to_string(), count);
    count
}
```

**Complexity**: O(V + E) - each node computed once
**Memory**: O(V) - one entry per node

### Composite State (Position + History)
**When to use**: Subproblem result depends on both current position AND path history/constraints
**Example**: Count paths visiting required nodes in specific order (Day 11 Part 2)

```rust
// State = (current node, set of visited required nodes)
fn count_paths_with_requirements(
    current: &str,
    target: &str,
    visited_mask: usize,  // Bitmask: which required nodes visited
    required: &[&str],
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<(String, usize), usize>,
) -> usize {
    if current == target {
        // Check if all required nodes visited (all bits set)
        let all_visited = (1 << required.len()) - 1;
        return if visited_mask == all_visited { 1 } else { 0 };
    }
    
    // Check cache - state is (node, visited_set)
    let state = (current.to_string(), visited_mask);
    if let Some(&cached) = memo.get(&state) {
        return cached;
    }
    
    // Update visited mask if current is a required node
    let mut new_mask = visited_mask;
    for (i, &req) in required.iter().enumerate() {
        if current == req {
            new_mask |= 1 << i;  // Set bit i
        }
    }
    
    let count = graph.get(current)
        .map(|neighbors| {
            neighbors.iter()
                .map(|next| count_paths_with_requirements(
                    next, target, new_mask, required, graph, memo
                ))
                .sum()
        })
        .unwrap_or(0);
    
    memo.insert(state, count);
    count
}
```

**Complexity**: O(V × 2^R) where R = number of required nodes
- For 2 required: O(V × 4) - effectively linear
- For 5 required: O(V × 32) - still manageable

**Memory**: O(V × 2^R) - one entry per (node, visited_set) combination

## Real-World Examples from AoC 2025

### Day 7: Tachyon Manifold (Simple Memoization)
**Problem**: Count quantum timelines (paths) from start to bottom edge in grid
**State**: Just the current coordinate
**Why simple works**: Path count from a position doesn't depend on how we got there

```rust
fn count_timelines_memoized(
    pos: Coord,
    grid: &Grid<Cell>,
    memo: &mut HashMap<Coord, usize>,
) -> usize {
    // Exit: reached bottom edge
    if pos.row as usize >= grid.height() - 1 {
        return 1;
    }
    
    // Check cache
    if let Some(&cached) = memo.get(&pos) {
        return cached;
    }
    
    let cell = grid[pos];
    let count = match cell {
        Cell::Empty => {
            // Continue downward
            if let Some(next) = try_move_in_direction(&grid, pos, Direction::South) {
                count_timelines_memoized(next, grid, memo)
            } else {
                1  // Exit at bottom
            }
        }
        Cell::Splitter => {
            // Quantum split: explore both left and right paths
            let mut total = 0;
            for dir in [Direction::West, Direction::East] {
                if let Some(next) = try_move_in_direction(&grid, pos, dir) {
                    total += count_timelines_memoized(next, grid, memo);
                }
            }
            total
        }
    };
    
    memo.insert(pos, count);
    count
}
```

**Result**: 390,684,413,472,684 timelines computed instantly
**Key insight**: Position alone determines timeline count - no need to track history

### Day 11: Network Routing (Composite State Memoization)
**Problem**: Count paths visiting BOTH required nodes (any order)
**State**: (current_node, visited_required_nodes_bitmask)
**Why composite needed**: Path count depends on which required nodes we've already visited

**Initial Naive Approach** (❌ Timeout):
```rust
fn count_naive(
    current: &str,
    path: &mut Vec<String>,  // Track full path - EXPENSIVE!
    required: &[&str],
) -> usize {
    path.push(current.to_string());
    
    if current == target {
        let has_all = required.iter().all(|&r| path.contains(r));
        path.pop();
        return if has_all { 1 } else { 0 };
    }
    
    // No memoization → explores every path completely
    let count = graph[current].iter()
        .map(|next| count_naive(next, path, required))
        .sum();
    
    path.pop();
    count
}
```
**Problem**: O(2^n) - must enumerate 549 trillion paths, times out after 10 seconds

**Optimized Approach** (✅ Instant):
```rust
// State includes which required nodes visited (bitmask)
fn count_optimized(
    current: &str,
    visited_mask: usize,
    required: &[&str],
    memo: &mut HashMap<(String, usize), usize>,
) -> usize {
    if current == target {
        let all_visited = (1 << required.len()) - 1;
        return if visited_mask == all_visited { 1 } else { 0 };
    }
    
    // Composite state: (node, visited_set)
    let state = (current.to_string(), visited_mask);
    if let Some(&cached) = memo.get(&state) {
        return cached;  // Reuse computation!
    }
    
    let mut new_mask = visited_mask;
    for (i, &req) in required.iter().enumerate() {
        if current == req {
            new_mask |= 1 << i;
        }
    }
    
    let count = graph[current].iter()
        .map(|next| count_optimized(next, new_mask, required, memo))
        .sum();
    
    memo.insert(state, count);
    count
}
```
**Result**: 549,705,036,748,518 paths computed instantly via memoization
**Key insight**: Only 4 states per node (00, 01, 10, 11) for 2 required nodes

## Choosing the Right State Representation

### Decision Framework

| **Question** | **Simple State** | **Composite State** |
|-------------|------------------|---------------------|
| Does result depend only on current position? | ✅ Yes | ❌ No - also depends on history |
| Can same position be visited with different constraints? | ❌ No | ✅ Yes - visited set varies |
| Is history size bounded and small? | N/A | ✅ Yes (use bitmask) |
| Is history size unbounded or large? | N/A | ❌ No (use different approach) |

### Bitmask Techniques for Small Sets

**When to use**: Tracking 2-5 items (visited nodes, collected keys, unlocked doors)

```rust
// Bitmask operations
let mut mask = 0usize;

// Set bit i (mark item i as visited)
mask |= 1 << i;

// Check bit i (test if item i visited)
let visited = (mask & (1 << i)) != 0;

// Check all bits set (all items visited)
let all_visited = (1 << count) - 1;
let complete = mask == all_visited;

// Count set bits (items visited)
let num_visited = mask.count_ones();
```

**Example states for 3 required nodes**:
```
0b000 (0): None visited
0b001 (1): Node 0 visited
0b010 (2): Node 1 visited
0b011 (3): Nodes 0,1 visited
0b100 (4): Node 2 visited
0b101 (5): Nodes 0,2 visited
0b110 (6): Nodes 1,2 visited
0b111 (7): All visited ✓
```

## Performance Impact

### Day 11 Part 2 Comparison

| **Approach** | **State** | **Memo Size** | **Time** | **Result** |
|-------------|----------|---------------|----------|------------|
| **Naive (no memo)** | Full path | N/A | Timeout (10s+) | ❌ |
| **Optimized (composite)** | (node, mask) | V × 4 entries | <1s | ✅ 549T paths |

**Space-time tradeoff**: 
- Naive: O(1) space, O(2^n) time → Unusable
- Memoized: O(V × 2^R) space, O(V × 2^R) time → Practical

### Exponential → Linear Reduction

**Without memoization**:
```
Path count = 2^(branching factor × depth)
For highly connected graph: ~549,000,000,000,000 paths
Enumeration impossible
```

**With state-based memoization**:
```
State count = V × 2^R
For Day 11: ~500 nodes × 4 states = ~2,000 states
Each state computed once, reused many times
```

## Common Pitfalls

### ❌ Mistake 1: State Too Simple
```rust
// BUG: Ignores that paths with different visited sets are different subproblems
fn count_wrong(node: &str, memo: &mut HashMap<String, usize>) -> usize {
    // This assumes all paths from 'node' have same count
    // But paths visiting different required nodes have different counts!
}
```
**Symptom**: Wrong answer (undercounting or overcounting)
**Fix**: Include constraint state in memo key

### ❌ Mistake 2: State Too Complex
```rust
// SLOW: Full path in state is overkill
fn count_slow(
    node: &str, 
    path: Vec<String>,  // Entire path history - too much!
    memo: &mut HashMap<(String, Vec<String>), usize>
) -> usize {
    // Vec hashing is expensive, memory usage explodes
}
```
**Symptom**: High memory usage, slow HashMap operations
**Fix**: Encode only necessary constraint info (bitmask, not full path)

### ❌ Mistake 3: Forgetting to Update State
```rust
// BUG: Doesn't update visited_mask when passing required node
let count = neighbors.iter()
    .map(|next| count_paths(next, visited_mask, ...))  // Same mask!
    .sum();
```
**Symptom**: Wrong answer (doesn't recognize required nodes as visited)
**Fix**: Update state before recursive call

## Related Patterns

- **[[dynamic-programming]]** - Broader category including state-based memoization
- **[[bitmask-dp]]** - Using bitmasks to encode state in DP problems
- **[[graph-path-counting]]** - Specific application domain for these techniques
- **[[memoization-vs-tabulation]]** - Top-down (memo) vs bottom-up (DP table) approaches

## When NOT to Use Memoization

1. **State space too large**: If state count > 10^6, memoization may use too much memory
2. **No overlapping subproblems**: If each state computed once, memoization adds overhead
3. **State comparison expensive**: If state is complex structure, HashMap lookups slow
4. **Better algorithm exists**: Sometimes mathematical formula or greedy works better

## References

- **AoC 2025 Day 7** - [[advent_of_code/aoc2025/Problem_Statements/day07]] - Simple position-based memoization
- **AoC 2025 Day 11** - [[advent_of_code/aoc2025/Problem_Statements/day11]] - Composite state with bitmask
- **Daily Notes** - [[Daily Notes/2025-12-11]] - Performance debugging journey (timeout → optimization)

---

*Tags: #algorithms #memoization #dynamic-programming #state-representation #bitmask #graph-algorithms #performance-optimization #aoc*

*Links: [[graph-path-counting]] | [[bitmask-dp]] | [[dynamic-programming]] | [[performance-debugging]] | [[advent_of_code/aoc2025/Problem_Statements/day07]] | [[advent_of_code/aoc2025/Problem_Statements/day11]]*
