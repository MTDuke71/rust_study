# Mission 10: Union-Find Disjoint Set Data Structure

**Mission Duration**: November 2-8, 2025 (7 days)

**Objective**: Implement a high-performance Union-Find (Disjoint Set Union - DSU) data structure with path compression and union by rank optimizations, following V-Cycle methodology.

---

## 🎯 Mission Overview

The Union-Find data structure (also known as Disjoint Set Union) is a fundamental data structure that efficiently maintains a partition of a set into disjoint subsets. It supports two primary operations:
- **Find**: Determine which subset a particular element belongs to
- **Union**: Join two subsets into a single subset

This data structure is crucial for algorithms involving connected components, cycle detection, network connectivity, Kruskal's minimum spanning tree algorithm, and many other graph-related problems.

---

## 📋 Requirements Specification

### REQ-1: Basic Union-Find Structure
- Create a `UnionFind` struct that maintains `n` disjoint sets
- Each element is initially in its own set
- Support creation with `new(size: usize)` constructor
- Use vector-based parent tracking
- **Complexity**: O(n) initialization

### REQ-2: Find Operation with Path Compression
- Implement `find(&mut self, x: usize) -> Result<usize, String>` operation
- Return the representative (root) of the set containing element `x`
- Apply path compression optimization during traversal
- Flatten the tree structure to speed up future queries
- **Complexity**: O(α(n)) amortized (inverse Ackermann function, effectively constant)

### REQ-3: Union Operation with Union by Rank
- Implement `union(&mut self, x: usize, y: usize) -> Result<bool, String>` operation
- Join the sets containing elements `x` and `y`
- Return `true` if sets were merged, `false` if already in same set
- Use union by rank to keep trees balanced
- Attach smaller tree under the root of the larger tree
- **Complexity**: O(α(n)) amortized

### REQ-4: Connected Query Operation
- Implement `connected(&mut self, x: usize, y: usize) -> Result<bool, String>` operation
- Determine if two elements are in the same set
- Utilize the find operation for efficiency
- **Complexity**: O(α(n)) amortized

### REQ-5: Set Counting and Statistics
- Track the number of disjoint sets with `count: usize` field
- Implement `count(&self) -> usize` to return current number of sets
- Implement `size(&mut self, x: usize) -> Result<usize, String>` to get size of set containing `x`
- Update count dynamically during union operations
- **Complexity**: O(1) for count(), O(α(n)) for size()

### REQ-6: Error Handling and Bounds Checking
- Validate element indices are within bounds [0, n)
- Return descriptive error messages for invalid operations
- Use `Result` type for operations that can fail
- Handle edge cases gracefully (empty sets, single element, etc.)

### REQ-7: Connected Components Application
- Implement graph connectivity detection
- Provide `from_edges` constructor for graph initialization
- Support cycle detection in undirected graphs
- Enable minimum spanning tree algorithms (Kruskal's)

---

## 🏗️ V-Cycle Development Process

### Phase 1: Requirements Analysis (Day 1 - Nov 2)
**Deliverables**:
- [x] Complete requirements specification (this document)
- [x] Define public API surface
- [x] Establish performance targets
- [x] Create test plan outline

**Activities**:
```bash
# Initialize mission structure
cd missions/Mission10
cargo init --lib

# Define initial API in lib.rs
# Create TODO.md with 7-day plan
```

### Phase 2: Design (Day 2 - Nov 3)
**Deliverables**:
- [ ] Data structure design (parent vector, rank vector)
- [ ] Algorithm pseudocode for find and union operations
- [ ] Optimization strategies (path compression, union by rank)
- [ ] Error handling strategy

**Design Decisions**:
- **Representation**: Use `Vec<usize>` for parent tracking (index = element, value = parent)
- **Rank**: Use `Vec<usize>` for tree heights
- **Mutability**: Find requires `&mut self` due to path compression
- **Generic vs Concrete**: Start with `usize` indices, consider generic keys later

### Phase 3: Implementation (Days 3-5 - Nov 4-6)
**Day 3 (Nov 4)**: Core Operations
- [ ] Implement `UnionFind::new()`
- [ ] Implement basic `find()` with path compression
- [ ] Write unit tests for REQ-1 and REQ-2

**Day 4 (Nov 5)**: Union and Connectivity
- [ ] Implement `union()` with union by rank
- [ ] Implement `connected()` query
- [ ] Write unit tests for REQ-3 and REQ-4

**Day 5 (Nov 6)**: Statistics and Applications
- [ ] Implement set counting and size tracking
- [ ] Add error handling and validation
- [ ] Create connected components examples
- [ ] Write unit tests for REQ-5, REQ-6, REQ-7

### Phase 4: Testing & Validation (Day 6 - Nov 7)
**Deliverables**:
- [ ] Complete unit test suite (all REQ tests passing)
- [ ] Integration tests for graph algorithms
- [ ] Performance benchmarks
- [ ] Edge case validation

**Test Categories**:
- Unit tests: Each requirement validated independently
- Property tests: Invariants maintained (transitivity, reflexivity)
- Performance tests: Verify O(α(n)) complexity
- Edge cases: Empty sets, single element, full connectivity

### Phase 5: Documentation (Day 7 - Nov 8)
**Deliverables**:
- [ ] Comprehensive API documentation with examples
- [ ] Algorithm explanation and complexity analysis
- [ ] Usage examples and common patterns
- [ ] Tutorial integration with Mission10_tut

---

## 🧪 Test Plan

### Unit Tests (tests/unit_tests.rs)

```rust
#[test]
fn req1_basic_initialization() {
    // Test UnionFind creation and initial state
    // Each element should be in its own set
}

#[test]
fn req2_find_with_path_compression() {
    // Test find operation correctness
    // Verify path compression occurs
}

#[test]
fn req3_union_by_rank() {
    // Test union operation merges sets correctly
    // Verify union by rank optimization
}

#[test]
fn req4_connected_query() {
    // Test connectivity detection
    // Verify correct results after unions
}

#[test]
fn req5_set_counting() {
    // Test count() accuracy
    // Verify count decreases with unions
}

#[test]
fn req5_set_size() {
    // Test size() for individual sets
    // Verify size increases with unions
}

#[test]
fn req6_bounds_checking() {
    // Test error handling for out-of-bounds
    // Verify descriptive error messages
}

#[test]
fn req7_connected_components() {
    // Test graph connectivity detection
    // Verify cycle detection
}

#[test]
fn test_edge_case_single_element() {
    // Test with n=1
}

#[test]
fn test_edge_case_full_connectivity() {
    // Test when all elements are united
}
```

### Integration Tests (tests/integration_tests.rs)

```rust
#[test]
fn test_graph_connectivity() {
    // Build graph from edges
    // Verify connected components
}

#[test]
fn test_cycle_detection() {
    // Use union-find for cycle detection
    // Test on various graphs
}

#[test]
fn test_kruskal_mst_support() {
    // Demonstrate minimum spanning tree use case
}
```

### Performance Benchmarks (benches/performance.rs)

```rust
// Benchmark find operations
fn bench_find_operations(c: &mut Criterion) { ... }

// Benchmark union operations
fn bench_union_operations(c: &mut Criterion) { ... }

// Benchmark worst-case scenarios
fn bench_worst_case(c: &mut Criterion) { ... }
```

---

## 📊 Complexity Analysis

| Operation | Time Complexity | Space Complexity |
|-----------|----------------|------------------|
| `new(n)` | O(n) | O(n) |
| `find(x)` | O(α(n))* | O(1) |
| `union(x, y)` | O(α(n))* | O(1) |
| `connected(x, y)` | O(α(n))* | O(1) |
| `count()` | O(1) | O(1) |
| `size(x)` | O(α(n))* | O(1) |

*α(n) is the inverse Ackermann function, which grows extremely slowly. For all practical values of n, α(n) ≤ 4, making it effectively constant time.

**Space Complexity**: O(n) total for storing parent and rank arrays.

---

## 🎮 Usage Examples

### Example 1: Basic Usage

```rust
use mission10::UnionFind;

fn main() -> Result<(), String> {
    let mut uf = UnionFind::new(10);
    
    // Initially 10 disjoint sets
    println!("Initial sets: {}", uf.count()); // 10
    
    // Union some elements
    uf.union(0, 1)?;
    uf.union(2, 3)?;
    uf.union(0, 2)?; // Merges {0,1} and {2,3}
    
    println!("After unions: {}", uf.count()); // 7
    
    // Check connectivity
    println!("0 and 3 connected: {}", uf.connected(0, 3)?); // true
    println!("0 and 4 connected: {}", uf.connected(0, 4)?); // false
    
    Ok(())
}
```

### Example 2: Network Connectivity

```rust
use mission10::UnionFind;

fn main() -> Result<(), String> {
    let mut network = UnionFind::new(6); // 6 computers
    
    // Connect computers with cables
    network.union(0, 1)?; // Cable 1-2
    network.union(1, 2)?; // Cable 2-3
    network.union(3, 4)?; // Cable 4-5
    
    // Check if computers can communicate
    if network.connected(0, 2)? {
        println!("Computer 1 can reach Computer 3");
    }
    
    if !network.connected(0, 5)? {
        println!("Computer 1 cannot reach Computer 6");
        println!("Need to add cable!");
    }
    
    // Add cable to connect networks
    network.union(2, 3)?;
    
    println!("Connected components: {}", network.count());
    
    Ok(())
}
```

### Example 3: Cycle Detection

```rust
use mission10::UnionFind;

fn has_cycle(edges: &[(usize, usize)], n: usize) -> Result<bool, String> {
    let mut uf = UnionFind::new(n);
    
    for &(u, v) in edges {
        // If u and v are already connected, adding this edge creates a cycle
        if uf.connected(u, v)? {
            return Ok(true);
        }
        uf.union(u, v)?;
    }
    
    Ok(false)
}

fn main() -> Result<(), String> {
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 0)]; // Forms a cycle
    
    if has_cycle(&edges, 4)? {
        println!("Graph contains a cycle");
    }
    
    Ok(())
}
```

---

## 🚀 Running the Mission

```bash
# Build the library
cargo build

# Run all tests
cargo test

# Run specific requirement tests
cargo test req1_basic_initialization
cargo test req2_find_with_path_compression

# Run benchmarks
cargo bench

# Generate documentation
cargo doc --open

# Run examples
cargo run --example demo
cargo run --example connected_components
cargo run --example network_connectivity
```

---

## 📚 Learning Resources

### Background Reading
- **CLRS**: "Introduction to Algorithms" - Chapter on Disjoint Sets
- **Competitive Programming**: Union-Find is fundamental for many graph problems
- **Real-world Applications**: Network connectivity, image processing, Kruskal's MST

### Related Data Structures
- Disjoint Set Forests
- Weighted Union-Find (for additional optimizations)
- Persistent Union-Find

### Algorithms Using Union-Find
- Kruskal's Minimum Spanning Tree
- Cycle detection in undirected graphs
- Least common ancestor (offline)
- Connected components in dynamic graphs

---

## 🔗 Integration

### Mission 10 Tutorial (Mission10_tut)
- **Step 1**: Basic Union-Find without optimizations
- **Step 2**: Path compression optimization
- **Step 3**: Union by rank optimization
- **Step 4**: Combined optimizations and analysis
- **Step 5**: Applications (MST, cycle detection)
- **Step 6**: Advanced variants (weighted, persistent)
- **Step 7**: Real-world problem solving

### Alignment with Daily Study (Week 6)
- **Day 36**: Module organization for Union-Find API
- **Day 37**: Crate structure and library design
- **Day 38**: Feature flags for optimization variants
- **Day 39**: Workspace integration
- **Day 40**: Documentation and publishing preparation
- **Day 41**: Dependencies for benchmarking (criterion)
- **Day 42**: Complete project organization

---

## 🎯 Success Criteria

### Functional Requirements
- [ ] All 7 requirements (REQ-1 through REQ-7) implemented
- [ ] All unit tests passing
- [ ] All integration tests passing
- [ ] Examples demonstrate all major use cases

### Quality Requirements
- [ ] Zero clippy warnings with `-D warnings`
- [ ] All public APIs documented with rustdoc
- [ ] Code coverage > 90%
- [ ] Performance benchmarks show O(α(n)) behavior

### Documentation Requirements
- [ ] API documentation complete
- [ ] Usage examples clear and runnable
- [ ] Complexity analysis documented
- [ ] Integration with tutorial complete

---

## 📝 Daily Progress Tracking

### Day 1 (Nov 2) - Setup ✅
- [x] Mission structure created
- [x] Requirements defined
- [x] Test plan outlined
- [x] Initial API design

### Day 2 (Nov 3) - Design
- [ ] Algorithm design complete
- [ ] Data structure finalized
- [ ] Error handling strategy defined

### Day 3 (Nov 4) - Core Implementation
- [ ] REQ-1: Basic structure
- [ ] REQ-2: Find with path compression
- [ ] Unit tests for REQ-1, REQ-2

### Day 4 (Nov 5) - Union Operations
- [ ] REQ-3: Union by rank
- [ ] REQ-4: Connected query
- [ ] Unit tests for REQ-3, REQ-4

### Day 5 (Nov 6) - Advanced Features
- [ ] REQ-5: Statistics
- [ ] REQ-6: Error handling
- [ ] REQ-7: Applications
- [ ] Unit tests for REQ-5, REQ-6, REQ-7

### Day 6 (Nov 7) - Testing & Validation
- [ ] All tests passing
- [ ] Benchmarks complete
- [ ] Edge cases covered

### Day 7 (Nov 8) - Documentation
- [ ] API docs complete
- [ ] Examples polished
- [ ] Tutorial integration
- [ ] Mission complete! 🎉

---

## 🔗 Related Concepts

- [[../../zettelkasten/union-find-data-structure|Union-Find in Zettelkasten]]
- [[../../zettelkasten/disjoint-sets|Disjoint Sets Theory]]
- [[../../zettelkasten/path-compression|Path Compression Optimization]]
- [[../../zettelkasten/union-by-rank|Union by Rank Optimization]]
- [[../../zettelkasten/kruskals-algorithm|Kruskal's MST Algorithm]]

---

*Mission Start: November 2, 2025*
*Mission End: November 8, 2025*
*Status: Planning Phase*

*Navigation: [[../../missions/README.md|Missions Overview]] | [[../../tutorials/Mission10_tut/README.md|Mission 10 Tutorial]]*
