# Competitive Programming: Tree Diameter with LinkedQueue

## 🎯 Project Overview

This project demonstrates the **optimal use case for LinkedQueue** in competitive programming: **Tree diameter calculation using multi-source BFS** where queue size is unbounded and dynamic growth is essential for unknown problem sizes.

## 🏆 Problem: Tree Diameter (Longest Path in Tree)

**Classic competitive programming problem** found in contests like:
- Codeforces (Tree problems, DP on trees)
- AtCoder (Graph algorithms)
- USACO (Tree diameter, tree DP)
- ACM ICPC (Graph theory challenges)

**Given**: Unweighted tree with n nodes and n-1 edges  
**Find**: Longest simple path between any two nodes (diameter)

## 🔗 Algorithm: Two-Pass BFS

1. **First BFS**: Start from any node, find the farthest node from it
2. **Second BFS**: Start from that farthest node, find the farthest node from it  
3. **Result**: The distance from step 2 is the tree diameter

## 💡 Why LinkedQueue is Perfect Here

| **Advantage** | **Benefit** |
|---------------|-------------|
| **Unbounded Problem** | Tree size unknown, queue grows dynamically |
| **Variable Queue Size** | Different BFS runs have different queue patterns |
| **Memory Efficiency** | Allocate only as needed, no wasted capacity |
| **No Capacity Limits** | Can handle trees of any size within memory |
| **Two Separate BFS** | Each gets fresh queue without capacity concerns |

## 📋 V-Cycle Requirements & Traceability

### Tree Construction Requirements
- **REQ-T1**: Build adjacency list from edge list input ✅ `tree.rs:Tree::from_edges()`
- **REQ-T2**: Support undirected tree representation ✅ `tree.rs:Tree::add_edge()`
- **REQ-T3**: Handle nodes numbered 0 to n-1 ✅ `tree.rs:NodeId` type alias
- **REQ-T4**: Validate tree properties (n-1 edges, connected) ✅ `tree.rs:Tree::from_edges()` validation

### BFS Algorithm Requirements
- **REQ-B1**: Implement BFS with distance and parent tracking ✅ `diameter.rs:bfs_farthest()`
- **REQ-B2**: Use LinkedQueue for unbounded growth during traversal ✅ `diameter.rs` queue usage
- **REQ-B3**: Find farthest node from any given starting node ✅ `diameter.rs:bfs_farthest()`
- **REQ-B4**: Support multiple independent BFS runs ✅ Two-pass algorithm
- **REQ-B5**: Handle trees of arbitrary size (1 to 10^6+ nodes) ✅ Dynamic allocation

### Diameter Calculation Requirements
- **REQ-D1**: Execute two-pass BFS algorithm correctly ✅ `diameter.rs:find_diameter()`
- **REQ-D2**: Return diameter length and actual path ✅ `(usize, Vec<NodeId>)` return
- **REQ-D3**: Reconstruct path using parent pointers ✅ `diameter.rs:reconstruct_path()`
- **REQ-D4**: Handle edge cases (single node, empty tree) ✅ Special case handling
- **REQ-D5**: Guarantee correctness for any tree structure ✅ Algorithm correctness

## 🚀 Quick Start

```rust
use competitive_linked_tree::TreeDiameter;

// Create tree with edges: 0-1, 1-2, 2-3, 3-4 (line graph)
let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
let diameter_finder = TreeDiameter::from_edges(5, edges).expect("Valid tree");

match diameter_finder.find_diameter() {
    Some((diameter, path)) => {
        println!("Diameter: {}", diameter);
        println!("Path: {:?}", path);
    }
    None => println!("Empty tree"),
}
```

## 🔧 Running the Examples

```powershell
# Run main demo
cargo run --package competitive_linked_tree

# Run detailed analysis demo  
cargo run --package competitive_linked_tree --example tree_diameter_demo

# Run all tests
cargo test --package competitive_linked_tree

# Run integration tests with data files
cargo test --package competitive_linked_tree integration_test
```

## 📊 Performance Characteristics

| **Metric** | **LinkedQueue** | **std::VecDeque** | **std::collections::VecDeque** |
|------------|-----------------|-------------------|--------------------------------|
| **Memory Growth** | Dynamic O(n) | Dynamic O(n) | Dynamic O(n) |
| **Allocation Pattern** | As needed | Exponential | Exponential |
| **Memory Overhead** | Minimal | Some slack | Some slack |
| **Operation Complexity** | O(1) | Amortized O(1) | Amortized O(1) |
| **Memory Fragmentation** | Linked nodes | Contiguous | Contiguous |
| **Peak Memory Usage** | Exactly needed | May overshoot | May overshoot |

## 🎮 Competitive Programming Applications

This pattern applies to many tree/graph contest problems:

### Tree Problems
- **Tree diameter** (this project)
- **Tree center finding**
- **Longest path in tree**
- **Tree DP problems**

### Graph BFS
- **Unknown graph size**, dynamic exploration
- **Multi-source BFS** from multiple starting points
- **Level-order problems** with unknown depth

### Parsing Problems
- **Tree building from input** with unknown structure
- **Dynamic graph construction**

## 📁 Project Structure

```
competitive_linked_tree/
├── Cargo.toml           # Project configuration
├── src/
│   ├── lib.rs          # Main module with documentation
│   ├── main.rs         # Demo application
│   ├── tree.rs         # Tree data structure and adjacency list
│   └── diameter.rs     # Two-pass BFS using LinkedQueue
├── examples/
│   └── tree_diameter_demo.rs  # Detailed analysis and comparisons
├── tests/
│   ├── integration_test.rs     # End-to-end testing
│   └── data/                   # Test tree files
│       ├── line_graph.txt
│       ├── star_graph.txt
│       └── binary_tree.txt
└── README.md           # This file
```

## 🔬 Testing Strategy

### Unit Tests (in src/)
- **Requirements-based naming**: `req_t1_t3_tree_creation()`, `req_d1_d5_line_graph_diameter()`
- **Individual component testing**: Tree construction, BFS algorithm, path reconstruction
- **Edge case coverage**: Empty trees, single nodes, various tree shapes

### Integration Tests (tests/)
- **File-based test data**: Standard competitive programming input format
- **End-to-end validation**: Parse → Build Tree → Find Diameter → Verify
- **Scaling tests**: LinkedQueue behavior with different tree sizes

### Example Programs
- **Demo applications**: Real-world tree diameter scenarios
- **Performance analysis**: LinkedQueue vs alternatives
- **Educational content**: Why unbounded problems benefit from dynamic growth

## 💡 When to Use LinkedQueue

### ✅ Perfect For:
- **Tree/graph problems** with unknown size
- **Queue size varies dramatically** between runs
- **Memory efficiency** more important than cache performance
- **Multiple independent BFS** traversals needed
- **Input size not bounded** or very large

### ❌ Consider RingBufferQueue Instead:
- **Grid/matrix problems** with known bounds
- **Single BFS run** with predictable queue size
- **Cache performance is critical**
- **Memory allocation overhead** must be minimized

## 🧮 Time Complexity

- **Tree Construction**: O(n) where n = number of nodes
- **Single BFS**: O(n + m) where m = number of edges (m = n-1 for trees)
- **Diameter Algorithm**: O(n) since we do two BFS runs
- **Space Complexity**: O(n) for adjacency list + queue + visited array

## 🎯 Competitive Programming Tips

1. **Two-pass BFS**: Standard algorithm for tree diameter
2. **LinkedQueue adapts**: No need to estimate capacity
3. **Tree input format**: Usually n followed by n-1 edges
4. **Handle edge cases**: Empty trees, single nodes
5. **Path reconstruction**: Store parent pointers during BFS

## 📚 Learning Outcomes

After working with this project, you'll understand:

- **When dynamic queues outperform fixed-capacity queues**
- **Memory efficiency vs cache efficiency trade-offs**
- **Two-pass BFS algorithm for tree diameter**
- **LinkedQueue benefits for unknown problem sizes**
- **V-Cycle methodology for complex algorithm development**

## 🔗 Related Projects

- **Mission2**: Core RingBufferQueue and LinkedQueue implementations
- **competitive_ring_bfs**: Demonstrates RingBufferQueue for bounded grid problems
- **Brackets_Basic/Brackets_Ext**: Stack-based parsing with different capacity strategies

---

*This project follows the **V-Cycle development methodology** with complete traceability from requirements through implementation to validation.*
---

## 🔗 Related Resources & Navigation

### 📚 Zettelkasten Navigation
- **[[zettel-index]]** - Main knowledge base entry point
- **[[Missions Overview]]** - V-Cycle project implementations
- **[[Collections MOC]]** - Data structures and algorithms

### 🎯 LinkedQueue Resources
- **[[mission-2]]** - Queue V-Cycle implementation
- **[[Ring Buffer Overwriting Semantics]]** - Capacity management strategies
- [[../../missions/Mission2/IMPLEMENTATION_SUMMARY|Implementation Summary]] - Mission2 complete details
- [[../../missions/Mission2/ALTERNATIVE_COMPARISON|Alternative Comparison]] - Queue approaches

### 🌲 Tree Algorithms & BFS
- Tree diameter algorithm - Two-pass BFS approach
- Unbounded queue growth - LinkedQueue benefits
- Multi-source BFS - Multiple starting points
- Competitive programming patterns

### 📖 Related Examples
- [[Q_A]] - Implementation discussion and decisions
- [[../competitive_ring_bfs/README|Ring BFS Example]] - RingBufferQueue use case
- [[../competitive_ring_bfs/Q_A|Ring BFS Discussion]] - DFS vs BFS data structures

### 🏷️ Tags
*Tags: #linkedqueue #bfs #tree-diameter #competitive-programming #unbounded-growth #v-cycle #requirements*
