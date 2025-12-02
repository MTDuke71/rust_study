# Priority Queue Patterns

*A comprehensive guide to priority queue operations, implementations, and algorithmic applications in Rust.*

## Core Concept

A **priority queue** is an abstract data type where each element has an associated priority, and elements are dequeued in priority order rather than insertion order. Unlike standard queues (FIFO), priority queues serve the highest-priority element first.

### Key Characteristics

- **Ordered Access**: Elements retrieved by priority, not insertion order
- **Efficient Operations**: O(log n) insert and extract operations
- **Flexible Ordering**: Customizable comparison logic via traits
- **Heap-Based**: Typically implemented using binary heaps

## Rust's BinaryHeap<T>

Rust provides `std::collections::BinaryHeap<T>` - a **max-heap** by default.

### Basic Operations

```rust
use std::collections::BinaryHeap;

fn priority_queue_basics() {
    // Create a max-heap
    let mut heap = BinaryHeap::new();
    
    // Insert elements: O(log n)
    heap.push(5);
    heap.push(2);
    heap.push(8);
    heap.push(1);
    
    // Peek at maximum: O(1)
    assert_eq!(heap.peek(), Some(&8));
    
    // Extract maximum: O(log n)
    assert_eq!(heap.pop(), Some(8));
    assert_eq!(heap.pop(), Some(5));
    assert_eq!(heap.pop(), Some(2));
    assert_eq!(heap.pop(), Some(1));
    assert_eq!(heap.pop(), None);
}
```

### Min-Heap Pattern

To create a min-heap, wrap elements in `Reverse`:

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn min_heap_example() {
    let mut min_heap = BinaryHeap::new();
    
    // Wrap in Reverse for min-heap behavior
    min_heap.push(Reverse(5));
    min_heap.push(Reverse(2));
    min_heap.push(Reverse(8));
    min_heap.push(Reverse(1));
    
    // Extracts minimum first
    assert_eq!(min_heap.pop(), Some(Reverse(1)));
    assert_eq!(min_heap.pop(), Some(Reverse(2)));
}
```

## Performance Characteristics

| Operation | Time Complexity | Description |
|-----------|----------------|-------------|
| `push(item)` | O(log n) | Insert element |
| `pop()` | O(log n) | Remove highest priority |
| `peek()` | O(1) | View highest priority |
| `len()` | O(1) | Get size |
| Heapify from Vec | O(n) | Build from collection |

### Space Complexity

- **O(n)** - Linear space for n elements
- **Contiguous memory** - Backed by `Vec<T>`

## Dijkstra's Algorithm Application

Priority queues are essential for Dijkstra's shortest path algorithm:

```rust
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    id: usize,
    distance: u32,
}

// Ord implementation for min-heap by distance
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance) // Reverse for min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra_example(graph: &HashMap<usize, Vec<(usize, u32)>>, start: usize) -> HashMap<usize, u32> {
    let mut distances = HashMap::new();
    let mut pq = BinaryHeap::new();
    
    // Initialize
    distances.insert(start, 0);
    pq.push(Node { id: start, distance: 0 });
    
    while let Some(Node { id, distance }) = pq.pop() {
        // Skip if we've found a better path
        if distance > *distances.get(&id).unwrap_or(&u32::MAX) {
            continue;
        }
        
        // Process neighbors
        if let Some(neighbors) = graph.get(&id) {
            for &(neighbor_id, edge_weight) in neighbors {
                let new_dist = distance + edge_weight;
                let current_dist = *distances.get(&neighbor_id).unwrap_or(&u32::MAX);
                
                if new_dist < current_dist {
                    distances.insert(neighbor_id, new_dist);
                    pq.push(Node { id: neighbor_id, distance: new_dist });
                }
            }
        }
    }
    
    distances
}
```

## A* Algorithm Application

A* uses priority queue with heuristic-based ordering:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AStarNode {
    position: (usize, usize),
    g_score: u32,  // Cost from start
    f_score: u32,  // g_score + heuristic
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Min-heap by f_score
        other.f_score.cmp(&self.f_score)
            .then_with(|| other.g_score.cmp(&self.g_score))
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> u32 {
    ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as u32
}

// A* search would use this in priority queue:
// pq.push(AStarNode {
//     position,
//     g_score,
//     f_score: g_score + manhattan_distance(position, goal),
// });
```

## Custom Priority Ordering

Implement `Ord` for custom priority logic:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
struct Task {
    name: String,
    priority: u8,
    urgency: u8,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority first, then urgency
        self.priority.cmp(&other.priority)
            .then_with(|| self.urgency.cmp(&other.urgency))
            .then_with(|| self.name.cmp(&other.name))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
```

## Binary Heap Implementation Details

### Internal Structure

```
Binary Heap as Array:
       10
      /  \
     7    9
    / \  / \
   3  5 8  2

Array: [10, 7, 9, 3, 5, 8, 2]
Index:  0   1  2  3  4  5  6

Parent of i: (i - 1) / 2
Left child:  2*i + 1
Right child: 2*i + 2
```

### Heap Property

- **Max-Heap**: Parent ≥ all children
- **Min-Heap**: Parent ≤ all children

### Heapify Operations

1. **Bubble Up**: After insertion, restore heap property upward
2. **Bubble Down**: After removal, restore heap property downward

## Common Patterns

### 1. Top-K Elements Pattern

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn top_k_elements(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut min_heap = BinaryHeap::new();
    
    for num in nums {
        min_heap.push(Reverse(num));
        if min_heap.len() > k {
            min_heap.pop();
        }
    }
    
    min_heap.into_iter().map(|Reverse(n)| n).collect()
}
```

### 2. Merge K Sorted Lists Pattern

```rust
#[derive(Debug, Eq, PartialEq)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val) // Min-heap
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Use BinaryHeap to merge k sorted lists efficiently
```

### 3. Event Scheduling Pattern

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Event {
    time: u64,
    event_type: EventType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EventType {
    Start,
    End,
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.time.cmp(&self.time) // Min-heap by time
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
```

## Mission 9 Integration

Priority queues are fundamental to Mission 9's pathfinding algorithms:

### Dijkstra Implementation (Mission 9 Day 2)

- **Use Case**: Shortest path in weighted graphs
- **Priority**: Distance from source
- **Pattern**: Min-heap with distance tracking

### A* Implementation (Mission 9 Day 3)

- **Use Case**: Optimal pathfinding with heuristics
- **Priority**: f(n) = g(n) + h(n)
- **Pattern**: Min-heap with combined cost

### Performance Optimization (Mission 9 Day 4)

- **Closed Set**: Track visited nodes separately
- **Early Termination**: Stop when goal reached
- **Tie Breaking**: Secondary ordering for equal priorities

## AoC Applications

Priority queues appear in multiple Advent of Code problems:

### Day 15 (2021): Chiton

- **Problem**: Lowest risk path through grid
- **Solution**: Dijkstra with BinaryHeap
- **Key**: Each cell's risk as edge weight

### Day 23 (2021): Amphipod

- **Problem**: Minimum energy to organize amphipods
- **Solution**: A* with custom state
- **Key**: Complex state space with heuristic

## Best Practices

### ✅ Do

- Use `BinaryHeap` for priority-based access
- Wrap in `Reverse` for min-heap behavior
- Implement `Ord` carefully for custom types
- Consider separate closed/visited set for graphs
- Profile performance for large datasets

### ❌ Don't

- Don't assume stable ordering for equal priorities
- Don't mutate elements after insertion
- Don't forget to handle duplicate states in graph algorithms
- Don't use for simple FIFO queues (use `VecDeque`)

## Performance Considerations

### When to Use Priority Queue

- ✅ Need access to min/max element repeatedly
- ✅ Dijkstra, A*, or greedy algorithms
- ✅ Event-driven simulation
- ✅ Top-K problems
- ✅ Merge operations

### When NOT to Use

- ❌ Simple FIFO queue (`VecDeque` is better)
- ❌ Need stable ordering
- ❌ Frequent arbitrary element access
- ❌ All elements have same priority

## Testing Patterns

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    
    #[test]
    fn test_max_heap_ordering() {
        let mut heap = BinaryHeap::from(vec![3, 1, 4, 1, 5, 9]);
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(5));
    }
    
    #[test]
    fn test_min_heap_ordering() {
        let mut heap = BinaryHeap::from(vec![
            Reverse(3), Reverse(1), Reverse(4), 
            Reverse(1), Reverse(5), Reverse(9)
        ]);
        assert_eq!(heap.pop(), Some(Reverse(1)));
        assert_eq!(heap.pop(), Some(Reverse(1)));
    }
    
    #[test]
    fn test_custom_priority() {
        #[derive(Debug, Eq, PartialEq)]
        struct Item { priority: u32, value: String }
        
        impl Ord for Item {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.priority.cmp(&other.priority)
            }
        }
        
        impl PartialOrd for Item {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        
        let mut heap = BinaryHeap::new();
        heap.push(Item { priority: 5, value: "low".to_string() });
        heap.push(Item { priority: 10, value: "high".to_string() });
        
        let item = heap.pop().unwrap();
        assert_eq!(item.priority, 10);
    }
}
```

## Common Pitfalls

### 1. Max-Heap Default

```rust
// ❌ Wrong: Expects min-heap but gets max
let mut heap = BinaryHeap::new();
heap.push(5);
heap.push(2);
assert_eq!(heap.pop(), Some(5)); // Gets 5, not 2!

// ✅ Correct: Use Reverse for min-heap
let mut heap = BinaryHeap::new();
heap.push(Reverse(5));
heap.push(Reverse(2));
assert_eq!(heap.pop(), Some(Reverse(2)));
```

### 2. Duplicate State Handling

```rust
// ❌ Wrong: Processes same node multiple times
while let Some(node) = pq.pop() {
    process(node); // May process duplicates
}

// ✅ Correct: Track visited nodes
let mut visited = HashSet::new();
while let Some(node) = pq.pop() {
    if !visited.insert(node.id) {
        continue; // Skip already visited
    }
    process(node);
}
```

### 3. Stale Priority Values

```rust
// ❌ Wrong: Priority may be stale
pq.push(Node { id: 1, cost: 10 });
// Later find better path...
pq.push(Node { id: 1, cost: 5 }); // Both in queue!

// ✅ Correct: Check current best on pop
while let Some(node) = pq.pop() {
    if node.cost > best_cost[&node.id] {
        continue; // Skip stale entry
    }
    // Process with current best cost
}
```

## Related Concepts

### Data Structures

- [[Binary Heap Data Structure]] - Underlying implementation
- [[graph-algorithms]] - Primary use case
- [[Rust Collections MOC]] - Collection type overview

### Algorithms

- [[Dijkstra Algorithm]] - Shortest path with priority queue
- [[missions/mission-9]] - Mission 9 pathfinding implementations
- [[BFS Patterns]] - Comparison with breadth-first search

### Learning Resources

- [[daily-study/Day26]] - Queue implementation study
- [[Mission 9 Tutorial]] - Tutorial progression
- [[Daily Notes/2025-10-23]] - Pathfinding fundamentals

## Mission Integration

### Mission 2

- Queue fundamentals and FIFO semantics
- Contrast with priority-based ordering

### Mission 9

- **Day 2**: Dijkstra with BinaryHeap
- **Day 3**: A* heuristic-based priority
- **Day 4**: Performance optimization patterns

### Mission 10

- Union-Find vs Priority Queue comparison
- When to use each data structure

## Further Reading

### Official Documentation

- [std::collections::BinaryHeap](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html)
- [std::cmp::Reverse](https://doc.rust-lang.org/std/cmp/struct.Reverse.html)

### Academic References

- Introduction to Algorithms (CLRS) - Chapter 6: Heapsort
- Algorithm Design Manual - Priority Queues

### Implementation Examples

- Mission 9 Dijkstra implementation
- Mission 9 A* implementation
- AoC 2021 Day 15, Day 23 solutions

---

## Related Resources

- [[Binary Heap Data Structure]] - Implementation details and heap operations
- [[graph-algorithms]] - Algorithms using priority queues (Dijkstra, A*, Prim's)
- [[missions/mission-9]] - Mission 9 pathfinding with priority queue patterns
- [[Mission 9 Tutorial]] - Tutorial progression teaching priority queues
- [[Dijkstra Algorithm]] - Shortest path algorithm using priority queues
- [[daily-study/Day26]] - Daily study on queue implementations and patterns
- [[Daily Notes/2025-10-23]] - Pathfinding fundamentals and priority queue usage
- [[Rust Collections MOC]] - Overview of Rust collection types
- [[Algorithms MOC]] - Algorithm patterns and implementations

*Tags: #data-structures #priority-queue #binary-heap #algorithms #dijkstra #a-star #pathfinding #mission9 #graph-algorithms #rust-collections #performance-optimization*
