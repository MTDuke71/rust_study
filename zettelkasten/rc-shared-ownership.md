# Rc<T> - Reference Counted Smart Pointer

*Type: Concept Note*  
*Created: 2025-11-13*  
*Status: Active*

## Overview

`Rc<T>` (Reference Counted) is a smart pointer that enables multiple ownership of the same data through reference counting. It keeps track of the number of references to a value and only cleans up the value when the last reference goes out of scope.

**Key Characteristics**:
- **Shared Ownership**: Multiple owners can hold references to the same data
- **Reference Counting**: Tracks number of active references at runtime
- **Single-Threaded Only**: Not thread-safe (use `Arc<T>` for concurrent contexts)
- **Immutable by Default**: All references are immutable unless combined with `RefCell<T>`
- **Heap Allocation**: Data stored on heap with reference count metadata

## When to Use Rc<T>

### ✅ Perfect Use Cases

1. **Graph Data Structures**: Multiple nodes reference the same node
2. **Shared Configuration**: Multiple components read same config
3. **Observer Patterns**: Multiple observers reference same subject
4. **Tree Structures**: Nodes with multiple parents (DAGs)
5. **Caching**: Multiple consumers share cached data

### ❌ When NOT to Use Rc<T>

1. **Single Ownership Sufficient**: Use `Box<T>` instead
2. **Multi-threaded Context**: Use `Arc<T>` instead
3. **Need Mutation**: Combine with `RefCell<T>` for interior mutability
4. **Performance Critical**: Runtime overhead from reference counting
5. **Circular References Risk**: Can cause memory leaks if not careful

## Core API

```rust
use std::rc::Rc;

// Creation
let rc1 = Rc::new(5);
let rc2 = Rc::clone(&rc1);  // Increment reference count (NOT deep copy)

// Reference counting
let count = Rc::strong_count(&rc1);  // Get current reference count
let weak_count = Rc::weak_count(&rc1);  // Get weak reference count

// Weak references (break cycles)
let weak = Rc::downgrade(&rc1);  // Create weak reference
if let Some(strong) = weak.upgrade() {  // Try to get strong reference
    // Use strong reference
}

// Getting inner value (requires unique ownership)
if let Ok(value) = Rc::try_unwrap(rc1) {
    // Now own the value directly
}
```

## Graph Problem Examples

### Example 1: AoC-Style Graph with Shared Nodes

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct GraphNode {
    id: usize,
    value: i32,
    neighbors: Vec<Rc<RefCell<GraphNode>>>,
}

impl GraphNode {
    fn new(id: usize, value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(GraphNode {
            id,
            value,
            neighbors: Vec::new(),
        }))
    }
    
    fn add_neighbor(&mut self, neighbor: Rc<RefCell<GraphNode>>) {
        self.neighbors.push(neighbor);
    }
}

// AoC-style usage: Building a graph from input
fn build_graph_from_edges(edges: &[(usize, usize, i32)]) -> Vec<Rc<RefCell<GraphNode>>> {
    use std::collections::HashMap;
    
    let mut nodes: HashMap<usize, Rc<RefCell<GraphNode>>> = HashMap::new();
    
    // Create all nodes
    for &(from, to, _) in edges {
        nodes.entry(from).or_insert_with(|| GraphNode::new(from, 0));
        nodes.entry(to).or_insert_with(|| GraphNode::new(to, 0));
    }
    
    // Add edges (shared ownership allows same node in multiple neighbor lists)
    for &(from, to, weight) in edges {
        let from_node = nodes.get(&from).unwrap().clone();
        let to_node = nodes.get(&to).unwrap().clone();
        from_node.borrow_mut().add_neighbor(to_node);
    }
    
    nodes.into_values().collect()
}

// Example: BFS traversal with shared nodes
fn bfs_traversal(start: Rc<RefCell<GraphNode>>) {
    use std::collections::{VecDeque, HashSet};
    
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_back(start);
    visited.insert(Rc::as_ptr(&queue[0]) as usize);  // Use raw pointer for visited check
    
    while let Some(current) = queue.pop_front() {
        let current_borrowed = current.borrow();
        println!("Visiting node {} with value {}", current_borrowed.id, current_borrowed.value);
        
        for neighbor in &current_borrowed.neighbors {
            let ptr = Rc::as_ptr(neighbor) as usize;
            if visited.insert(ptr) {
                queue.push_back(neighbor.clone());  // Rc::clone increments reference count
            }
        }
    }
}
```

### Example 2: Tree with Multiple Parents (DAG)

```rust
use std::rc::Rc;

#[derive(Debug)]
struct DagNode {
    value: String,
    children: Vec<Rc<DagNode>>,
}

impl DagNode {
    fn new(value: String) -> Rc<Self> {
        Rc::new(DagNode {
            value,
            children: Vec::new(),
        })
    }
}

// Example: Dependency graph (like package dependencies)
fn build_dependency_graph() -> Rc<DagNode> {
    // Shared dependency - multiple packages depend on it
    let shared_dep = DagNode::new("logging_utils".to_string());
    
    // Package A depends on shared_dep
    let pkg_a = Rc::new(DagNode {
        value: "package_a".to_string(),
        children: vec![Rc::clone(&shared_dep)],
    });
    
    // Package B also depends on shared_dep
    let pkg_b = Rc::new(DagNode {
        value: "package_b".to_string(),
        children: vec![Rc::clone(&shared_dep)],
    });
    
    // Root package depends on both A and B
    Rc::new(DagNode {
        value: "root".to_string(),
        children: vec![pkg_a, pkg_b],
    })
    
    // Note: shared_dep has reference count of 3 (pkg_a, pkg_b, and original binding)
}
```

### Example 3: AoC Day 5 2024 - Graph Dependency Management

```rust
use std::rc::Rc;
use std::collections::HashMap;

// Real AoC 2024 Day 5 pattern: Page ordering rules
#[derive(Debug, Clone)]
struct Page {
    number: i32,
    must_come_before: Vec<Rc<Page>>,  // Shared references to other pages
}

fn build_ordering_graph(rules: &[(i32, i32)]) -> HashMap<i32, Rc<Page>> {
    let mut pages: HashMap<i32, Rc<Page>> = HashMap::new();
    
    // Create all page nodes
    for &(before, after) in rules {
        pages.entry(before).or_insert_with(|| {
            Rc::new(Page {
                number: before,
                must_come_before: Vec::new(),
            })
        });
        pages.entry(after).or_insert_with(|| {
            Rc::new(Page {
                number: after,
                must_come_before: Vec::new(),
            })
        });
    }
    
    // Note: In real implementation, we'd need RefCell to modify must_come_before
    // This demonstrates the pattern where Rc<RefCell<T>> is needed for graph construction
    
    pages
}
```

## Rc<T> with RefCell<T> - Interior Mutability Pattern

When you need shared ownership AND mutation:

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct MutableGraph {
    nodes: Vec<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
struct Node {
    id: usize,
    visited: bool,
    neighbors: Vec<Rc<RefCell<Node>>>,
}

impl MutableGraph {
    fn mark_visited(&self, node_id: usize) {
        if let Some(node) = self.nodes.get(node_id) {
            node.borrow_mut().visited = true;  // Interior mutability!
        }
    }
    
    fn dfs(&self, start_id: usize) {
        fn dfs_helper(node: &Rc<RefCell<Node>>) {
            let mut borrowed = node.borrow_mut();
            if borrowed.visited {
                return;
            }
            borrowed.visited = true;
            println!("Visiting node {}", borrowed.id);
            
            // Clone neighbors before recursion to avoid borrow conflicts
            let neighbors = borrowed.neighbors.clone();
            drop(borrowed);  // Release borrow before recursion
            
            for neighbor in neighbors {
                dfs_helper(&neighbor);
            }
        }
        
        if let Some(start) = self.nodes.get(start_id) {
            dfs_helper(start);
        }
    }
}
```

## Performance Considerations

### Runtime Overhead
```rust
use std::rc::Rc;

let data = vec![1, 2, 3, 4, 5];

// Rc has overhead for reference counting
let rc1 = Rc::new(data);
let rc2 = Rc::clone(&rc1);  // Increments counter (atomic operation in Arc)
let rc3 = Rc::clone(&rc1);  // Increments again

// Each clone is O(1) but has runtime cost
// Each drop decrements counter

println!("Strong count: {}", Rc::strong_count(&rc1));  // 3
```

### Memory Overhead
- **Reference Count Storage**: Extra word(s) for strong/weak counts
- **Heap Allocation**: Data always on heap
- **Indirection**: Pointer dereference to access data

## Common Patterns

### Pattern 1: Shared Configuration

```rust
use std::rc::Rc;

struct Config {
    max_connections: usize,
    timeout_ms: u64,
}

struct Service {
    config: Rc<Config>,
}

impl Service {
    fn new(config: Rc<Config>) -> Self {
        Service { config }
    }
}

// Multiple services share same config
let config = Rc::new(Config {
    max_connections: 100,
    timeout_ms: 5000,
});

let service1 = Service::new(Rc::clone(&config));
let service2 = Service::new(Rc::clone(&config));
// config has reference count of 3
```

### Pattern 2: Observer Pattern

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct Subject {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
}

trait Observer {
    fn update(&mut self, data: i32);
}

impl Subject {
    fn attach(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }
    
    fn notify(&self, data: i32) {
        for observer in &self.observers {
            observer.borrow_mut().update(data);
        }
    }
}
```

### Pattern 3: Breaking Reference Cycles with Weak<T>

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,  // Weak reference to prevent cycle
    children: RefCell<Vec<Rc<Node>>>,
}

fn create_tree() -> Rc<Node> {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    // Set parent as weak reference to avoid cycle
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    
    branch
}
```

## AoC Integration Examples

### Pattern: Graph Traversal with Shared Nodes

Applicable to:
- **AoC 2024 Day 5**: Page ordering dependency graph
- **AoC 2015 Day 9**: Traveling salesman with city graph
- **AoC 2023 Day 10**: Pipe maze graph representation
- **Any problem with**: Multiple paths to same node, shared state

```rust
// Generic pattern for AoC graph problems
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

struct GraphSolver<T> {
    nodes: HashMap<String, Rc<RefCell<GraphNode<T>>>>,
}

#[derive(Debug)]
struct GraphNode<T> {
    id: String,
    data: T,
    edges: Vec<Rc<RefCell<GraphNode<T>>>>,
}

impl<T> GraphSolver<T> {
    fn add_edge(&mut self, from: &str, to: &str) {
        let from_node = self.nodes.get(from).unwrap().clone();
        let to_node = self.nodes.get(to).unwrap().clone();
        from_node.borrow_mut().edges.push(to_node);
    }
}
```

## Decision Tree: Rc<T> vs Alternatives

```
Need shared ownership?
├─ No → Use Box<T> or direct ownership
└─ Yes → Is it multi-threaded?
    ├─ Yes → Use Arc<T>
    └─ No → Need mutation?
        ├─ No → Use Rc<T>
        └─ Yes → Use Rc<RefCell<T>>
```

## Common Pitfalls

### ❌ Pitfall 1: Reference Cycles

```rust
// BAD: Creates memory leak
use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    next: Option<Rc<RefCell<Node>>>,
}

let a = Rc::new(RefCell::new(Node { next: None }));
let b = Rc::new(RefCell::new(Node { next: Some(Rc::clone(&a)) }));
a.borrow_mut().next = Some(Rc::clone(&b));  // Cycle! Memory leak!

// FIX: Use Weak<T> for one direction
```

### ❌ Pitfall 2: Unnecessary Cloning

```rust
// BAD: Deep clone when Rc::clone would work
let data = Rc::new(vec![1, 2, 3]);
let copy = (*data).clone();  // Expensive deep clone!

// GOOD: Reference count increment
let shared = Rc::clone(&data);  // O(1) operation
```

### ❌ Pitfall 3: Borrow Conflicts with RefCell

```rust
// BAD: Simultaneous mutable and immutable borrows
let node = Rc::new(RefCell::new(Node { value: 5 }));
let mut_borrow = node.borrow_mut();
let immut_borrow = node.borrow();  // PANIC! Already mutably borrowed

// FIX: Drop mut_borrow before immutable borrow
```

## Testing Patterns

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    
    #[test]
    fn test_reference_counting() {
        let data = Rc::new(vec![1, 2, 3]);
        assert_eq!(Rc::strong_count(&data), 1);
        
        let copy1 = Rc::clone(&data);
        assert_eq!(Rc::strong_count(&data), 2);
        
        let copy2 = Rc::clone(&data);
        assert_eq!(Rc::strong_count(&data), 3);
        
        drop(copy1);
        assert_eq!(Rc::strong_count(&data), 2);
    }
    
    #[test]
    fn test_shared_graph_node() {
        let shared = GraphNode::new(1, 42);
        let node1 = GraphNode::new(2, 10);
        let node2 = GraphNode::new(3, 20);
        
        node1.borrow_mut().add_neighbor(Rc::clone(&shared));
        node2.borrow_mut().add_neighbor(Rc::clone(&shared));
        
        // Shared node referenced by both node1 and node2
        assert_eq!(Rc::strong_count(&shared), 3);  // original + 2 neighbors
    }
}
```

## Real-World Mission Integration

### Mission 7 (Graphs) + Rc<T>

```rust
// Using Rc<T> with Mission 7 Graph implementation
use std::rc::Rc;
use std::cell::RefCell;

// Could extend Mission 7 with Rc-based node sharing
struct SharedGraph<T> {
    nodes: Vec<Rc<RefCell<Node<T>>>>,
}

// This enables patterns not possible with Mission 7's ownership model:
// - Multiple edges to same node without duplication
// - Bidirectional references without lifetime complications
// - Dynamic graph modification after construction
```

## Related Concepts

**Smart Pointers**:
- [[box-heap-allocation]] - Single ownership heap allocation
- [[refcell-interior-mutability]] - Interior mutability pattern (often used with Rc)
- [[arc-atomic-rc]] - Thread-safe reference counting
- [[weak-references]] - Breaking reference cycles

**Ownership & Borrowing**:
- [[ownership]] - Rust's ownership system fundamentals
- [[borrowing-rules]] - Borrow checker and lifetime rules
- [[lifetimes]] - Explicit lifetime annotations

**Graph & Data Structures**:
- [[mission-7]] - Graph algorithms and implementations
- [[graph-traversal-patterns]] - BFS, DFS, topological sort
- [[tree-structures]] - Binary trees, tries, DAGs

**AoC Problem Patterns**:
- [[aoc-graph-problems]] - Graph-based AoC challenges
- [[aoc-dependency-resolution]] - Topological sorting problems
- [[aoc-shared-state]] - Problems requiring shared mutable state

**Performance**:
- [[heap-vs-stack]] - Memory allocation strategies
- [[zero-cost-abstractions]] - Rust's performance philosophy
- [[reference-counting-overhead]] - Runtime cost analysis

## Further Reading

- **Rust Book**: Chapter 15.4 - Rc<T>, the Reference Counted Smart Pointer
- **Rust Book**: Chapter 15.5 - RefCell<T> and the Interior Mutability Pattern
- **Rust Book**: Chapter 15.6 - Reference Cycles Can Leak Memory
- **Documentation**: https://doc.rust-lang.org/std/rc/struct.Rc.html
- **Weak References**: https://doc.rust-lang.org/std/rc/struct.Weak.html

## Summary

`Rc<T>` enables **shared ownership** in single-threaded contexts through **reference counting**. It's essential for:
- Graph data structures with shared nodes
- Configuration sharing across components
- Tree structures with multiple parents (DAGs)
- Observer patterns and event systems

**Key Trade-offs**:
- ✅ Enables multiple ownership without lifetime complexity
- ✅ Automatic cleanup when last reference drops
- ✅ Safe sharing in single-threaded contexts
- ❌ Runtime overhead from reference counting
- ❌ Risk of reference cycles causing memory leaks
- ❌ Not thread-safe (use `Arc<T>` instead)

**Best Practice**: Combine with `RefCell<T>` when shared mutable access is needed, and use `Weak<T>` to prevent reference cycles in recursive structures.

---

*Navigation: [[zettel-index]] | [[Smart Pointers MOC]] | [[Rust Book Ch15]]*  
*Tags: #rust #smart-pointers #rc #reference-counting #shared-ownership #graphs #aoc-patterns #memory-management*  
*Related: [[box-heap-allocation]] [[refcell-interior-mutability]] [[mission-7]] [[aoc-graph-problems]]*
