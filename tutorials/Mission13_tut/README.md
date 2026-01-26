# Mission 13 Tutorial: Heaps & Priority Queues

**Tutorial Focus**: Binary heaps, priority queues, heap operations, and applications

**Zettelkasten**: [[mission-13]] | [[heap-data-structure]] | [[priority-queue-patterns]]

---

## 📚 Overview

This tutorial explores **heaps** and **priority queues** - fundamental data structures for efficiently managing elements by priority. Heaps are complete binary trees that maintain heap order, enabling O(log n) insertions and O(1) access to the highest/lowest priority element.

**Key Concepts**:
- Min-heap and max-heap properties
- Array-based heap representation
- Heap operations: insert, extract, heapify
- Priority queue applications (Dijkstra, scheduling, event systems)
- Heap sort algorithm

**Why Heaps Matter**:
- **Pathfinding**: Dijkstra's algorithm uses min-heap for shortest paths
- **Scheduling**: Process schedulers, task queues
- **Data Streams**: Finding median, top-K elements
- **AoC Applications**: Many pathfinding and optimization problems

---

## 🎯 Learning Path (10 Steps)

### **Day 1: Binary Heap Basics** (Step 1)
**File**: `examples/step1_binary_heap_basics.rs`

**Topics**:
- Complete binary tree concept
- Heap property (min-heap vs max-heap)
- Array representation (parent at i, children at 2i+1, 2i+2)
- Heap invariant verification

**Implementation**:
```rust
pub struct BinaryHeap<T> {
    data: Vec<T>,
    // Min-heap or max-heap (determined by comparator)
}
```

**Operations**:
- `new()` - Create empty heap
- `len()`, `is_empty()` - Size queries
- `peek()` - View min/max element (O(1))
- `is_valid_heap()` - Verify heap property

**Learning Outcomes**:
- ✅ Understand heap as array-based tree
- ✅ Know parent/child index formulas
- ✅ Distinguish min-heap from max-heap

---

### **Day 2: Heap Insertion** (Step 2)
**File**: `examples/step2_heap_insertion.rs`

**Topics**:
- Bubble-up (sift-up) algorithm
- Maintaining heap property on insert
- Time complexity analysis (O(log n))

**Operations**:
- `push(value)` - Insert new element
- `bubble_up(index)` - Restore heap property upward

**Example**:
```rust
// Insert 3 into min-heap [1, 5, 10, 8, 12]
//        1
//      /   \
//     5     10
//    / \
//   8  12
// 
// After push(3):
//        1
//      /   \
//     3     10      ← 3 bubbled up past 5
//    / \   /
//   8  12 5
```

**Learning Outcomes**:
- ✅ Implement bubble-up/sift-up algorithm
- ✅ Understand why insertion is O(log n)
- ✅ Visualize heap structure changes

---

### **Day 3: Heap Extraction** (Step 3)
**File**: `examples/step3_heap_extraction.rs`

**Topics**:
- Extract min/max (remove root)
- Bubble-down (sift-down) algorithm
- Replacing root with last element

**Operations**:
- `pop()` - Remove and return min/max element
- `bubble_down(index)` - Restore heap property downward

**Algorithm**:
```
1. Save root value (the min/max)
2. Move last element to root
3. Bubble down: swap with smaller child until heap property restored
4. Return saved root value
```

**Example**:
```rust
// Extract min from [1, 3, 10, 8, 12, 5]
//        1           Remove 1, move 5 to root
//      /   \
//     3     10             5            Bubble down
//    / \   /             /   \
//   8  12 5      →      3     10
//                      / \
//                     8  12
```

**Learning Outcomes**:
- ✅ Implement bubble-down/sift-down algorithm
- ✅ Choose correct child to swap with
- ✅ Understand extraction complexity O(log n)

---

### **Day 4: Heapify & Heap Sort** (Step 4)
**File**: `examples/step4_heapify_and_sort.rs`

**Topics**:
- Build heap from unsorted array (heapify)
- Floyd's algorithm (bottom-up heapify) - O(n)
- Heap sort algorithm - O(n log n)

**Operations**:
- `from_vec(vec)` - Build heap from existing data
- `heapify()` - Convert array to heap in-place
- `heap_sort(vec)` - Sort using heap

**Heapify Algorithm (Bottom-Up)**:
```rust
// Start from last non-leaf node, bubble down each
for i in (0..n/2).rev() {
    bubble_down(i);
}
// O(n) time - better than n insertions (O(n log n))
```

**Heap Sort**:
```rust
// 1. Heapify array (O(n))
// 2. Repeatedly extract max (O(n log n))
// Result: sorted array
```

**Learning Outcomes**:
- ✅ Build heap efficiently in O(n)
- ✅ Implement heap sort
- ✅ Compare heap sort vs quick sort vs merge sort

---

### **Day 5: Priority Queue** (Step 5)
**File**: `examples/step5_priority_queue.rs`

**Topics**:
- Priority queue abstraction
- Custom priority types
- Decrease-key operation
- Merging heaps

**Implementation**:
```rust
pub struct PriorityQueue<T, P> {
    heap: BinaryHeap<(P, T)>,  // (priority, value) pairs
}

// Methods:
// - push_with_priority(item, priority)
// - pop_highest_priority() -> Option<T>
// - update_priority(item, new_priority)  // Decrease-key
```

**Use Cases**:
- Task scheduling (priority-based)
- Event-driven simulation
- Dijkstra's algorithm (next node selection)

**Learning Outcomes**:
- ✅ Separate value from priority
- ✅ Implement custom ordering with Ord trait
- ✅ Handle priority updates efficiently

---

### **Day 6: Graph Algorithms with Heaps** (Step 6)
**File**: `examples/step6_dijkstra.rs`

**Topics**:
- Dijkstra's shortest path algorithm
- Using priority queue for node selection
- Integration with Mission 8 graphs

**Algorithm**:
```rust
fn dijkstra(graph: &Graph, start: Node) -> HashMap<Node, Distance> {
    let mut pq = PriorityQueue::new();
    let mut distances = HashMap::new();
    
    pq.push_with_priority(start, 0);
    
    while let Some(node) = pq.pop_highest_priority() {
        for neighbor in graph.neighbors(node) {
            let new_dist = distances[&node] + edge_weight;
            if new_dist < distances.get(&neighbor).unwrap_or(&INFINITY) {
                distances.insert(neighbor, new_dist);
                pq.push_with_priority(neighbor, new_dist);
            }
        }
    }
    distances
}
```

**Applications**:
- Shortest paths in weighted graphs
- Network routing
- AoC pathfinding problems

**Learning Outcomes**:
- ✅ Implement Dijkstra's algorithm
- ✅ Integrate heap with Mission 8 graphs
- ✅ Understand why heap is O(log n) vs naive O(n)

---

### **Day 7: Advanced Heap Patterns** (Step 7)
**File**: `examples/step7_advanced_patterns.rs`

**Topics**:
- Median maintenance (dual-heap pattern)
- K-largest/smallest elements
- Heap-based event scheduling
- Merging K sorted arrays

**Pattern 1: Median Maintenance**:
```rust
pub struct RunningMedian {
    max_heap: BinaryHeap<i32>,  // Lower half (max at top)
    min_heap: BinaryHeap<i32>,  // Upper half (min at top)
}
// Always maintain: max_heap.len() == min_heap.len() or max_heap.len() + 1
// Median = max_heap.peek()
```

**Pattern 2: Top-K Elements**:
```rust
pub struct TopK<T> {
    heap: BinaryHeap<T>,
    k: usize,
}
// Keep min-heap of size K
// If new element > heap.peek(), replace
```

**Pattern 3: Event Scheduler**:
```rust
pub struct EventScheduler {
    events: PriorityQueue<Event, Timestamp>,
}
// Pop events in timestamp order
```

**Learning Outcomes**:
- ✅ Use dual heaps for dynamic median
- ✅ Solve top-K problems efficiently
- ✅ Build event-driven systems

---

### **Day 8: D-ary Heaps & Variants** (Step 8)
**File**: `examples/step8_dary_heaps.rs`

**Topics**:
- Generalize binary heap to d-ary heap (d children per node)
- Min-max heap (track both min and max)
- Leftist heap (mergeable heaps)
- Trade-offs: d-ary vs binary

**D-ary Heap**:
```rust
pub struct DAryHeap<T> {
    data: Vec<T>,
    d: usize,  // Number of children per node
}

impl<T: Ord> DAryHeap<T> {
    fn parent(&self, i: usize) -> usize {
        (i - 1) / self.d
    }
    
    fn first_child(&self, i: usize) -> usize {
        i * self.d + 1
    }
    
    fn children(&self, i: usize) -> Range<usize> {
        let first = self.first_child(i);
        first..(first + self.d).min(self.data.len())
    }
}
```

**Trade-offs**:
| Heap Type | Insert | Extract Min | Memory | Best For |
|-----------|--------|-------------|--------|----------|
| Binary (d=2) | O(log₂ n) | O(log₂ n) | Compact | General use |
| 4-ary (d=4) | O(log₄ n) | O(4·log₄ n) | Cache-friendly | Few extracts |
| 8-ary (d=8) | O(log₈ n) | O(8·log₈ n) | Very cache-friendly | Insert-heavy |

**Min-Max Heap**:
```rust
// Even levels: min-heap property
// Odd levels: max-heap property
// Allows O(1) access to both min AND max!

pub struct MinMaxHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> MinMaxHeap<T> {
    pub fn peek_min(&self) -> Option<&T> {
        self.data.first()  // Root is always min
    }
    
    pub fn peek_max(&self) -> Option<&T> {
        // Max is one of root's children
        match self.data.len() {
            0 => None,
            1 => Some(&self.data[0]),
            2 => Some(&self.data[1]),
            _ => Some(self.data[1..3].iter().max().unwrap()),
        }
    }
}
```

**Learning Outcomes**:
- ✅ Generalize heap to arbitrary branching factor
- ✅ Understand cache effects (d-ary)
- ✅ Implement double-ended priority queue (min-max heap)

---

### **Day 9: Fibonacci Heaps** (Step 9)
**File**: `examples/step9_fibonacci_heap.rs`

**Topics**:
- Fibonacci heap structure (forest of trees)
- Lazy operations (defer work until necessary)
- Decrease-key in O(1) amortized
- Applications: Dijkstra, Prim's MST improvements

**Why Fibonacci Heaps?**
| Operation | Binary Heap | Fibonacci Heap |
|-----------|-------------|----------------|
| Insert | O(log n) | **O(1)** amortized |
| Find-Min | O(1) | O(1) |
| Extract-Min | O(log n) | O(log n) amortized |
| Decrease-Key | O(log n) | **O(1)** amortized |
| Merge | O(n) | **O(1)** |

**Structure** (Conceptual - Complex Implementation):
```rust
// Forest of min-heap-ordered trees
// Each node:
struct FibNode<T> {
    value: T,
    degree: usize,     // Number of children
    marked: bool,      // For cascading cuts
    parent: Option<*mut FibNode<T>>,
    child: Option<*mut FibNode<T>>,
    left: *mut FibNode<T>,   // Circular doubly-linked list
    right: *mut FibNode<T>>,
}

pub struct FibonacciHeap<T> {
    min: Option<*mut FibNode<T>>,
    size: usize,
}
```

**Key Operations**:
- **Insert**: Add new tree to root list (O(1))
- **Extract-Min**: Remove min, consolidate trees of same degree (O(log n) amortized)
- **Decrease-Key**: Cut from parent if violates heap order, cascading cuts (O(1) amortized)

**When to Use**:
- ✅ Dijkstra/Prim with many decrease-key operations
- ✅ Theoretical improvements (complex to implement!)
- ❌ Practical use: binary heap often faster due to constants

**Learning Outcomes**:
- ✅ Understand amortized analysis
- ✅ Know when Fibonacci heaps improve complexity
- ✅ Appreciate trade-offs (theory vs practice)

---

### **Day 10: External Memory & Disk-Based Heaps** (Step 10)
**File**: `examples/step10_external_heaps.rs`

**Topics**:
- External sorting with heaps (data doesn't fit in RAM)
- K-way merge for large datasets
- Memory-mapped files
- Streaming merge sort

**Problem**: Sort 100GB of data with 4GB RAM

**Solution: K-way Merge with Heap**:
```rust
pub struct ExternalHeapSort {
    chunk_size: usize,  // Fit in memory
    temp_dir: PathBuf,
}

impl ExternalHeapSort {
    pub fn sort_file(&self, input: &Path, output: &Path) -> io::Result<()> {
        // Phase 1: Sort chunks that fit in memory
        let sorted_chunks = self.sort_chunks(input)?;
        
        // Phase 2: K-way merge using min-heap
        self.kway_merge(&sorted_chunks, output)?;
        
        Ok(())
    }
    
    fn kway_merge(&self, chunks: &[PathBuf], output: &Path) -> io::Result<()> {
        let mut heap = BinaryHeap::new();
        let mut readers: Vec<_> = chunks.iter()
            .map(|p| BufReader::new(File::open(p).unwrap()))
            .collect();
        
        // Initialize heap with first element from each chunk
        for (i, reader) in readers.iter_mut().enumerate() {
            if let Some(value) = read_next(reader)? {
                heap.push(Reverse((value, i)));  // Min-heap
            }
        }
        
        let mut writer = BufWriter::new(File::create(output)?);
        
        while let Some(Reverse((value, chunk_id))) = heap.pop() {
            write_value(&mut writer, value)?;
            
            // Read next from same chunk
            if let Some(next) = read_next(&mut readers[chunk_id])? {
                heap.push(Reverse((next, chunk_id)));
            }
        }
        
        Ok(())
    }
}
```

**Applications**:
- Database external sorting
- Log file processing
- Large-scale data analytics
- AoC when input files are massive

**Learning Outcomes**:
- ✅ Handle datasets larger than RAM
- ✅ Use heap for k-way merge
- ✅ Understand I/O complexity vs computational complexity

---

## 🔗 Integration Points

### **Mission 8 (Graphs)**
- Dijkstra's algorithm uses priority queue
- A* pathfinding with heuristic priorities
- Minimum spanning tree (Prim's algorithm)

### **AoC Applications**
- **Pathfinding**: Many days require shortest path (heap-based Dijkstra)
- **Scheduling**: Task ordering, dependency resolution
- **Data Streams**: Top-K problems, median finding

### **Rust Book**
- **Ch8 (Collections)**: BinaryHeap in standard library
- **Ch10 (Generics)**: Generic heap implementation
- **Ch13 (Iterators)**: Heap-based iterators

---

## 📊 Performance Characteristics

| Operation | Time Complexity | Space Complexity |
|-----------|----------------|------------------|
| `push()` | O(log n) | O(1) |
| `pop()` | O(log n) | O(1) |
| `peek()` | O(1) | O(1) |
| `from_vec()` (heapify) | O(n) | O(1) extra |
| Heap sort | O(n log n) | O(1) |
| Dijkstra (with heap) | O((V+E) log V) | O(V) |

---

## 🎓 Learning Objectives

By completing this tutorial, you will:

### Core Concepts
- ✅ Understand binary heap structure and properties
- ✅ Know array-based tree representation
- ✅ Distinguish min-heap from max-heap
- ✅ Implement bubble-up and bubble-down algorithms

### Operations
- ✅ Insert elements in O(log n)
- ✅ Extract min/max in O(log n)
- ✅ Build heap from array in O(n)
- ✅ Implement heap sort

### Applications
- ✅ Use priority queues for scheduling
- ✅ Implement Dijkstra's algorithm
- ✅ Solve median maintenance problem
- ✅ Handle top-K element queries

### Advanced Patterns
- ✅ Dual-heap median tracking
- ✅ Event-driven systems
- ✅ Merging sorted streams

---

## 🚀 Getting Started

```bash
# Run examples in sequence
cd tutorials/Mission13_tut

cargo run --example step1_binary_heap_basics
cargo run --example step2_heap_insertion
cargo run --example step3_heap_extraction
cargo run --example step4_heapify_and_sort
cargo run --example step5_priority_queue
cargo run --example step6_dijkstra
cargo run --example step7_advanced_patterns

# Run all tests
cargo test

# Run benchmarks
cargo bench
```

---

## 📚 References

**Standard Library**:
- `std::collections::BinaryHeap` - Production-ready min/max heap
- Compare tutorial implementation to std behavior

**AoC Problems Using Heaps**:
- Pathfinding problems (many days)
- Resource scheduling
- Priority-based simulations

**Mathematical Background**:
- Complete binary trees
- Heap property invariants
- Floyd's heapify algorithm complexity

---

## 🎯 Next Steps

After completing this tutorial:
- [ ] **Mission 13**: Formal V-Cycle implementation with REQ-IDs
- [ ] Apply to AoC pathfinding problems
- [ ] Combine with Mission 8 for graph algorithms
- [ ] Explore concurrent priority queues (Mission 14)

---

*Zettelkasten Integration*:
- [[heap-data-structure]] - Core concepts
- [[priority-queue-patterns]] - Applications
- [[dijkstra-algorithm]] - Shortest path
- [[mission-8]] - Graph integration
- [[mission-14]] - Concurrent heaps

*Created: 2026-01-25*  
*Part of: Mission Track - Data Structures & Algorithms*
