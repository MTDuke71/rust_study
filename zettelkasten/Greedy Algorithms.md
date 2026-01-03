# Greedy Algorithms

**Making locally optimal choices at each step to find a global optimum**

> **Key Insight**: Greedy works when the problem has **optimal substructure** AND the **greedy choice property** - a locally optimal choice leads to a globally optimal solution.

---

## 🎯 **Core Concept**

A greedy algorithm builds a solution piece by piece, always choosing the next piece that offers the most immediate benefit. Unlike dynamic programming, greedy algorithms **never reconsider** previous choices.

### **When Greedy Works**

1. **Greedy Choice Property**: A global optimum can be reached by making locally optimal choices
2. **Optimal Substructure**: An optimal solution contains optimal solutions to subproblems

### **When Greedy Fails**

- **Coin Change** (arbitrary denominations): Greedy may not find minimum coins
- **0/1 Knapsack**: Must consider combinations, not just best value/weight ratio
- **Traveling Salesman**: Nearest neighbor heuristic isn't optimal

---

## 🔧 **Common Greedy Patterns**

### **Pattern 1: Selection with Constraints**

Pick k elements from n to maximize/minimize some value while satisfying constraints.

**AoC 2025 Day 3 Example** - Select 12 digits to form largest number:

```rust
/// Greedy k-digit selection preserving order
/// At each step: pick the largest digit that still leaves enough for remaining picks
fn max_k_digit_joltage(digits: &[u8], k: usize) -> u64 {
    let n = digits.len();
    let mut result: u64 = 0;
    let mut start = 0;
    let mut remaining = k;

    for _ in 0..k {
        // Window: can pick from [start..=(n - remaining)]
        let end = n - remaining;
        
        // Find max digit in window (first occurrence for tie-breaking)
        let max_digit = *digits[start..=end].iter().max().unwrap();
        let idx = digits[start..=end].iter().position(|&d| d == max_digit).unwrap();
        
        result = result * 10 + max_digit as u64;
        start = start + idx + 1;
        remaining -= 1;
    }
    result
}
```

**Critical Insight**: Tie-breaking matters! Pick the **earliest** max digit to preserve maximum flexibility for subsequent picks.

### **Pattern 2: Interval Scheduling**

Select maximum non-overlapping intervals.

```rust
/// Activity selection - pick most activities that don't overlap
fn max_activities(mut activities: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    // Sort by end time (greedy choice: finish earliest)
    activities.sort_by_key(|&(_, end)| end);
    
    let mut selected = vec![];
    let mut last_end = 0;
    
    for (start, end) in activities {
        if start >= last_end {
            selected.push((start, end));
            last_end = end;
        }
    }
    selected
}
```

### **Pattern 3: Huffman-Style Merging**

Repeatedly combine smallest elements.

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

/// Minimum cost to merge all elements (like rope cutting)
fn min_merge_cost(elements: Vec<i32>) -> i32 {
    let mut heap: BinaryHeap<Reverse<i32>> = elements.into_iter().map(Reverse).collect();
    let mut total_cost = 0;
    
    while heap.len() > 1 {
        let Reverse(a) = heap.pop().unwrap();
        let Reverse(b) = heap.pop().unwrap();
        let merged = a + b;
        total_cost += merged;
        heap.push(Reverse(merged));
    }
    total_cost
}
```

### **Pattern 4: Graph Greedy (Dijkstra/Kruskal)**

Always process the "best" unprocessed element.

**Dijkstra's Algorithm**: Always expand the vertex with smallest known distance

```rust
// See [[Dijkstra Algorithm]] for full implementation
// Key greedy property: once a vertex is finalized, its distance is optimal
```

**Kruskal's MST**: Always add the smallest edge that doesn't create a cycle

```rust
// See [[mission-10]] for Union-Find based implementation
// Key greedy property: smallest safe edge is always part of some MST
```

---

## 📊 **Greedy vs Other Paradigms**

| Aspect | Greedy | Dynamic Programming | Backtracking |
|--------|--------|---------------------|--------------|
| **Approach** | Make best local choice | Try all subproblems | Explore all paths |
| **Revisits choices** | Never | Implicitly (memoization) | Explicitly (backtrack) |
| **Time complexity** | Usually O(n) or O(n log n) | O(n²) to O(n³) typical | Exponential |
| **Space complexity** | Often O(1) | O(n) to O(n²) | O(recursion depth) |
| **Guarantee** | Only if greedy works | Always optimal | Always finds if exists |
| **Use when** | Greedy choice property holds | Overlapping subproblems | Need all solutions |

---

## 🎯 **AoC Applications**

### **Confirmed Greedy Problems**

| Problem | Pattern | Key Insight |
|---------|---------|-------------|
| **AoC 2025 Day 3** | K-selection | Earliest tie-breaking preserves flexibility |
| **Dijkstra variants** | Shortest path | Priority queue ensures optimal expansion order |
| **Interval scheduling** | Activity selection | Sort by end time, greedily select |
| **Huffman coding** | Merging | Always merge smallest first |

### **Looks Greedy But Isn't**

- **Subset sum**: Need DP or backtracking
- **Longest path in DAG**: Need DP (greedy may miss longer paths)
- **Edit distance**: Need DP (local choices affect global)

---

## 🔍 **Proving Greedy Correctness**

### **Exchange Argument**

Show that any optimal solution can be transformed into the greedy solution without losing optimality.

**Example** (Activity Selection):
1. Suppose optimal solution O doesn't include greedy choice g (earliest finish)
2. O must include some activity a that overlaps with g's time
3. Replace a with g in O - still valid (g finishes earlier)
4. New solution is no worse → greedy choice is safe

### **Greedy Stays Ahead**

Show that at every step, greedy solution is at least as good as any other.

**Example** (K-digit Selection):
1. After picking i digits, greedy has chosen largest possible prefix
2. Any other choice has ≤ greedy's prefix value
3. Greedy stays ahead → final result is optimal

---

## 🛠️ **Implementation Tips**

### **Sorting is Often Key**

Most greedy algorithms require sorting first:
- **By end time**: Interval scheduling
- **By ratio**: Fractional knapsack
- **By deadline**: Job scheduling with penalties
- **By weight**: Kruskal's MST

### **Priority Queues for Dynamic Greedy**

When the "best" choice changes as you process:

```rust
use std::collections::BinaryHeap;

// Max-heap by default
let mut heap = BinaryHeap::new();

// Min-heap using Reverse wrapper
use std::cmp::Reverse;
let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
```

### **Watch for Tie-Breaking**

Ties can make or break greedy correctness:
- `max_by_key` returns **last** max → may need custom logic
- Use `.position()` + `.find()` for **first** occurrence
- Document tie-breaking strategy in comments

---

## 🔗 **Related Concepts**

### **Algorithm Connections**

- **[[Dynamic Programming]]** - When greedy fails, DP often works
- **[[Dijkstra Algorithm]]** - Greedy shortest path algorithm
- **[[mission-10]]** - Union-Find for Kruskal's MST (greedy edge selection)
- **[[mission-9]]** - Priority queue for greedy algorithms
- **[[A-Star-Algorithm-Deep-Dive]]** - Greedy best-first with heuristics

### **AoC Resources**

- **[[AoC Patterns MOC]]** - Algorithm pattern library
- **[[../../advent_of_code/aoc2025/Problem_Statements/day03|AoC 2025 Day 3]]** - K-digit greedy selection example
- **[[../../advent_of_code/aoc2025/Problem_Statements/summary|AoC 2025 Summary]]** - Problem type distribution

### **Data Structures for Greedy**

- **[[Priority Queue Patterns]]** - Efficient "get best" operations
- **[[Union-Find]]** - Cycle detection for MST algorithms
- **[[sorting-algorithms]]** - Pre-processing for many greedy solutions

---

## 📚 **Classic Greedy Problems**

1. **Activity Selection** - Max non-overlapping intervals
2. **Huffman Coding** - Optimal prefix-free encoding
3. **Fractional Knapsack** - Max value with weight constraint (items divisible)
4. **Job Scheduling** - Minimize penalties for missed deadlines
5. **Minimum Spanning Tree** - Kruskal's and Prim's algorithms
6. **Dijkstra's Algorithm** - Single-source shortest paths (non-negative weights)
7. **Coin Change** (canonical systems) - Minimum coins for amount
8. **Task Assignment** - Match tasks to workers optimally

---

*Tags: #greedy #algorithms #optimization #aoc #competitive-programming #problem-solving*

*Links: [[AoC Patterns MOC]] | [[Dynamic Programming]] | [[Dijkstra Algorithm]] | [[mission-9]] | [[mission-10]] | [[memoization-comprehensive-guide]]*
