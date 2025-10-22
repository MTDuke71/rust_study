# Binary Heap Data Structure

**Tags**: #data-structures #heap #priority-queue #algorithms #binary-tree #rust #performance  
**Created**: October 22, 2025  
**Links**: [[Priority Queue Patterns]] | [[BFS Patterns]] | [[Collections MOC]] | [[Mission 9 Tutorial]]

---

## Overview

A **Binary Heap** is a complete binary tree that satisfies the heap property. It's the underlying data structure for priority queues and heap sort algorithms.

## Core Properties

### 1. **Complete Binary Tree**
- All levels filled except possibly the last
- Last level filled from left to right
- Height is always ⌊log₂(n)⌋

### 2. **Heap Property**
- **Max-Heap**: Parent ≥ all children (largest at root)
- **Min-Heap**: Parent ≤ all children (smallest at root)

## Array Representation

Binary heaps are typically stored in arrays for efficiency:

```rust
// For node at index i:
parent(i) = (i - 1) / 2
left_child(i) = 2 * i + 1
right_child(i) = 2 * i + 2

// Example max-heap: [16, 14, 10, 8, 7, 9, 3, 2, 4, 1]
//        16
//      /    \
//    14      10
//   / \     / \
//  8   7   9   3
// / \ /
//2  4 1
```

## Time Complexities

| Operation | Time Complexity | Description |
|-----------|----------------|-------------|
| Insert | O(log n) | Add element, bubble up |
| Extract-Min/Max | O(log n) | Remove root, bubble down |
| Peek | O(1) | View root element |
| Build Heap | O(n) | From unsorted array |
| Heapify | O(log n) | Restore heap property |

## Key Algorithms

### Insert (Bubble Up)
1. Add element to end of array
2. Compare with parent
3. Swap if heap property violated
4. Repeat until property satisfied

### Extract-Min/Max (Bubble Down)
1. Save root element (to return)
2. Move last element to root
3. Compare with children
4. Swap with appropriate child if needed
5. Repeat until property satisfied

## Rust Implementation Patterns

### Max-Heap (Rust's BinaryHeap)
```rust
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(5);
heap.push(1);
heap.push(8);
assert_eq!(heap.pop(), Some(8)); // Largest first
```

### Min-Heap via Reverse Ordering
```rust
// Method 1: Use Reverse wrapper
use std::cmp::Reverse;
heap.push(Reverse(5));

// Method 2: Custom ordering (our approach)
impl Ord for PriorityItem<T, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority) // Reversed!
    }
}
```

## Applications

### 1. **Priority Queues**
- Dijkstra's Algorithm
- A* Pathfinding
- Huffman Coding
- Task Scheduling

### 2. **Sorting**
- Heap Sort (O(n log n))
- K largest/smallest elements

### 3. **Graph Algorithms**
- Prim's MST
- Dijkstra's shortest path

## Advantages

✅ **Efficient Operations**: O(log n) for main operations  
✅ **Space Efficient**: Array-based, no pointer overhead  
✅ **Cache Friendly**: Sequential memory access  
✅ **Simple Implementation**: Straightforward algorithms  

## Disadvantages

❌ **No Search**: O(n) to find arbitrary element  
❌ **No Decrease-Key**: Expensive to update priorities  
❌ **Fixed Size**: Array-based implementations may need resizing  

## Heap vs Other Structures

| Structure | Insert | Extract-Min | Peek | Find |
|-----------|---------|-------------|------|------|
| Binary Heap | O(log n) | O(log n) | O(1) | O(n) |
| Sorted Array | O(n) | O(1) | O(1) | O(log n) |
| Linked List | O(1) | O(n) | O(n) | O(n) |
| BST | O(log n) | O(log n) | O(log n) | O(log n) |

## Pathfinding Context

In pathfinding algorithms like Dijkstra and A*:

```rust
// We need min-heap behavior for cost-based exploration
let mut frontier = BinaryHeap::new();

// Process lowest-cost nodes first
while let Some(current) = frontier.pop() {
    // Explore neighbors, add to frontier with updated costs
}
```

## Advanced Variations

### 1. **d-ary Heap**
- Each node has d children instead of 2
- Better for decrease-key operations
- Trade-off between insert/extract performance

### 2. **Fibonacci Heap**
- Better amortized complexity for decrease-key
- More complex implementation
- Used in advanced graph algorithms

### 3. **Pairing Heap**
- Simpler than Fibonacci heap
- Good practical performance
- Self-adjusting structure

## Implementation Tips

### Memory Layout
```rust
// Efficient array-based storage
struct BinaryHeap<T> {
    data: Vec<T>,
}

// Index calculations
fn parent(i: usize) -> usize { (i - 1) / 2 }
fn left_child(i: usize) -> usize { 2 * i + 1 }
fn right_child(i: usize) -> usize { 2 * i + 2 }
```

### Common Pitfalls
- **Off-by-one errors** in index calculations
- **Forgetting heap property** during modifications  
- **Wrong comparison direction** for min vs max heap
- **Not handling empty heap** edge cases

## Floating-Point Considerations

When using floating-point priorities:

```rust
// Problem: f64 doesn't implement Ord due to NaN
// Solution: OrderedFloat wrapper
#[derive(PartialEq)]
struct OrderedFloat(f64);

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}
```

## Performance Characteristics

### Best Case
- **Insert**: O(1) when added element maintains heap property
- **Extract**: O(1) when heap has only one element

### Worst Case  
- **Insert**: O(log n) when element bubbles to root
- **Extract**: O(log n) when replacement bubbles to leaf

### Average Case
- Both operations: O(log n) expected

## Related Concepts

- [[Priority Queue Patterns]] - High-level usage patterns
- [[A-Star-Algorithm-Deep-Dive]] - Pathfinding application
- [[Collections MOC]] - Other data structure options
- [[BFS Patterns]] - Breadth-first search with heaps

## Example Problems

1. **K Largest Elements**: Use min-heap of size K
2. **Merge K Sorted Arrays**: Use min-heap for efficient merging
3. **Median from Data Stream**: Use two heaps (max + min)
4. **Task Scheduler**: Priority-based task ordering

## Code Examples

See practical implementations in:
- `tutorials/Mission9_tut/examples/step1_priority_queue_foundation.rs`
- `missions/Mission8/src/lib.rs` (BFS/DFS with heaps)

---

*This Zettel is part of my algorithmic knowledge system. It connects heap theory with practical Rust implementations used in pathfinding tutorials.*