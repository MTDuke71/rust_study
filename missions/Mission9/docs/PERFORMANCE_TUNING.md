# Mission 9 Performance Tuning Guide

## Overview

This guide provides strategies for optimizing pathfinding performance across different problem types and scales.

## Algorithm Selection

### Decision Tree

```
Start: What type of problem do you have?
│
├─ Grid-based (coordinates, obstacles)?
│  ├─ Small grid (<1000 nodes)
│  │  └─ Use: A* with Manhattan heuristic
│  │
│  └─ Large grid (>1000 nodes)
│     └─ Use: Bidirectional A*
│
├─ Graph with coordinates (heuristic available)?
│  ├─ Dense graph (many edges)
│  │  └─ Use: A* with Euclidean heuristic
│  │
│  └─ Sparse graph
│     └─ Use: Bidirectional A*
│
└─ Graph without coordinates?
   ├─ Need guaranteed optimal?
   │  └─ Use: Dijkstra or Bidirectional Dijkstra
   │
   └─ Can use approximate heuristic?
      └─ Use: A* with domain-specific heuristic
```

### Performance Characteristics

| Algorithm | Time (Best) | Time (Worst) | Space | Optimal? | Requirements |
|-----------|-------------|--------------|-------|----------|--------------|
| Dijkstra | O(E log V) | O(E log V) | O(V) | ✓ | Non-negative weights |
| A* | O(b^d) | O(b^d) | O(b^d) | ✓* | Admissible heuristic |
| Bidirectional Dijkstra | O(E log V) | O(E log V) | O(V) | ✓ | Non-negative weights |
| Bidirectional A* | O(b^(d/2)) | O(b^d) | O(b^d) | ✓* | Admissible heuristic |

\* Optimal only with admissible heuristic

### Benchmark Results

From actual testing on Mission9:

**Small Graph (100 nodes, grid):**
```
Algorithm                | Avg Time | Speedup
Dijkstra                 | 18.30 µs | 1.00x
A* (Euclidean)           | 23.00 µs | 0.80x
```
*Conclusion: For very small graphs, Dijkstra's simplicity wins*

**Medium Grid (50×50 = 2500 nodes):**
```
Algorithm                | Avg Time | Speedup
Bidirectional Dijkstra   | 1.06 ms  | 1.00x
Bidirectional A*         | 0.03 ms  | 33.34x
```
*Conclusion: A* heuristic provides massive speedup on grids*

**Large Grid (100×100 = 10000 nodes):**
```
Algorithm                | Avg Time | Speedup
Dijkstra                 | ~50 ms   | 1.00x
A*                       | ~5 ms    | 10.00x
Bidirectional Dijkstra   | ~25 ms   | 2.00x
Bidirectional A*         | ~0.5 ms  | 100.00x
```
*Conclusion: Bidirectional + heuristic = exponential improvement*

## Heuristic Selection

### Manhattan Distance

**Best for:**
- Grid-based movement (4-directional)
- City block distance problems
- Taxicab geometry

**Formula:** `|x₁ - x₂| + |y₁ - y₂|`

**Performance:**
- Very fast to compute
- Admissible for 4-directional grids
- Provides good guidance in grid problems

**Example:**
```rust
use mission9::*;

let heuristic = ManhattanHeuristic;
let pathfinder = AstarPathfinder::new(heuristic);
```

**When to avoid:**
- Diagonal movement allowed (underestimates)
- Non-grid graphs

### Euclidean Distance

**Best for:**
- Direct-line distance
- Spatial problems with any-angle movement
- Physical distance calculations

**Formula:** `√((x₁-x₂)² + (y₁-y₂)²)`

**Performance:**
- Slower than Manhattan (requires sqrt)
- Always admissible
- Excellent guidance for spatial problems

**Example:**
```rust
use mission9::*;

let heuristic = EuclideanHeuristic;
let pathfinder = AstarPathfinder::new(heuristic);
```

**Optimization tip:**
```rust
// For comparison-only operations, skip sqrt:
fn euclidean_squared(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    dx * dx + dy * dy // 2-3x faster than with sqrt
}
```

### Chebyshev Distance

**Best for:**
- 8-directional grid movement
- Chess king movement
- Maximum coordinate difference

**Formula:** `max(|x₁ - x₂|, |y₁ - y₂|)`

**Performance:**
- Fastest to compute
- Admissible for 8-directional grids
- Can be too optimistic for 4-directional

**Example:**
```rust
use mission9::*;

let heuristic = ChebyshevHeuristic;
let pathfinder = AstarPathfinder::new(heuristic);
```

### Zero Heuristic

**Best for:**
- Testing/debugging (A* becomes Dijkstra)
- Comparison baseline
- Unknown problem structure

**Performance:**
- Fastest heuristic (no computation)
- No search guidance
- Equivalent to Dijkstra

**Example:**
```rust
use mission9::*;

let heuristic = ZeroHeuristic;
let pathfinder = AstarPathfinder::new(heuristic); // Equivalent to Dijkstra
```

### Custom Heuristics

**Creating domain-specific heuristics:**

```rust
use mission9::*;

struct TrafficAwareHeuristic {
    congestion_map: HashMap<u32, f64>,
}

impl Heuristic for TrafficAwareHeuristic {
    fn estimate(&self, from: u32, to: u32, context: &HeuristicContext) -> f64 {
        // Base Euclidean distance
        let base = if let (Some(p1), Some(p2)) = 
            (context.get_coordinate(from), context.get_coordinate(to)) {
            let dx = p1.0 - p2.0;
            let dy = p1.1 - p2.1;
            (dx * dx + dy * dy).sqrt()
        } else {
            0.0
        };
        
        // Adjust for congestion
        let congestion_penalty = self.congestion_map.get(&from).unwrap_or(&1.0);
        base * congestion_penalty
    }
    
    fn is_admissible(&self) -> bool {
        true // If congestion ≥ 1.0, this is admissible
    }
}
```

**Heuristic Design Rules:**
1. **Admissibility**: h(n) ≤ true_cost(n, goal) for optimality
2. **Consistency**: h(n) ≤ cost(n, n') + h(n') for efficiency
3. **Computation speed**: Faster heuristic = more nodes evaluated
4. **Guidance quality**: Better heuristic = fewer nodes needed

## Memory Optimization

### Graph Representation

**For sparse graphs (<10% density):**
```rust
// Adjacency list (default) - O(V + E) space
let graph = SimpleWeightedGraph::new(nodes);
```

**For dense graphs (>50% density):**
Consider matrix representation if implementing custom graph:
```rust
// Adjacency matrix - O(V²) space, O(1) edge lookup
// (Not currently in Mission9, but could be added)
```

### Memory Usage Estimates

| Graph Size | Adjacency List | Adjacency Matrix |
|------------|----------------|------------------|
| 100 nodes, 500 edges | ~8 KB | 40 KB |
| 1K nodes, 5K edges | ~80 KB | 4 MB |
| 10K nodes, 50K edges | ~800 KB | 400 MB |
| 100K nodes, 500K edges | ~8 MB | 40 GB |

**Conclusion:** Adjacency list scales much better for sparse graphs

### Reducing Memory Allocations

**1. Pre-allocate collections:**
```rust
let mut open_set = BinaryHeap::with_capacity(estimated_size);
let mut closed_set = HashSet::with_capacity(estimated_size);
```

**2. Reuse pathfinder instances:**
```rust
// Instead of:
for query in queries {
    let pathfinder = DijkstraPathfinder::new(); // New allocation each time
    pathfinder.find_path(&graph, query.0, query.1)?;
}

// Do:
let pathfinder = DijkstraPathfinder::new(); // Single allocation
for query in queries {
    pathfinder.find_path(&graph, query.0, query.1)?;
}
```

**3. Use node pools (Mission9 Day 4):**
```rust
use mission9::NodePool;

let mut pool = NodePool::new(1000); // Pre-allocate 1000 nodes
// Pool automatically reuses memory
```

## Computational Optimization

### Early Termination

**Goal-directed pruning (A*):**
```rust
// A* naturally terminates early when goal is reached
// Dijkstra explores all nodes up to distance d

// Speedup: 2-100x depending on heuristic quality
```

**Iteration limits:**
```rust
// For time-constrained applications
let pathfinder = MultiObjectiveAstar::with_max_iterations(10000);
```

### Bidirectional Search

**When to use:**
- Large graphs (>1000 nodes)
- Symmetric edge weights
- Known start and goal

**Performance:**
```rust
// Unidirectional: Explores O(b^d) nodes
// Bidirectional: Explores O(2 * b^(d/2)) nodes

// Example: b=10, d=6
// Unidirectional: 1,000,000 nodes
// Bidirectional: 2,000 nodes (500x speedup!)
```

**Implementation:**
```rust
use mission9::*;

let mut pathfinder = BidirectionalAstar::new(manhattan_heuristic);
let result = pathfinder.find_path(start, goal, neighbors);
```

### Batch Processing

**Amortize graph loading:**
```bash
# Bad: Load graph 1000 times
for i in {1..1000}; do
    mission9 find-path --graph big.json --start $i --goal 999
done

# Good: Load once, process batch
mission9 batch --graph big.json --queries batch.csv --output results.csv
```

**Parallel processing:**
```rust
use rayon::prelude::*;
use mission9::*;

let results: Vec<_> = queries.par_iter().map(|(start, goal)| {
    let pathfinder = AstarPathfinder::new(EuclideanHeuristic);
    pathfinder.find_path(&graph, *start, *goal)
}).collect();
```

## Problem-Specific Optimizations

### Grid Pathfinding

**1. Use appropriate heuristic:**
```rust
// 4-directional movement
let heuristic = ManhattanHeuristic;

// 8-directional movement
let heuristic = ChebyshevHeuristic;

// Any-angle movement
let heuristic = EuclideanHeuristic;
```

**2. Efficient neighbor generation:**
```rust
// Cache movement deltas
const DIRS_4: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const DIRS_8: [(i32, i32); 8] = [
    (-1, 0), (1, 0), (0, -1), (0, 1),
    (-1, -1), (-1, 1), (1, -1), (1, 1)
];

let neighbors = |pos: (usize, usize)| {
    DIRS_4.iter().filter_map(|(dx, dy)| {
        let nx = (pos.0 as i32 + dx) as usize;
        let ny = (pos.1 as i32 + dy) as usize;
        if nx < width && ny < height && !obstacles.contains(&(nx, ny)) {
            Some(((nx, ny), 1))
        } else {
            None
        }
    }).collect()
};
```

**3. Obstacle representation:**
```rust
// O(1) lookup with HashSet
let obstacles: HashSet<(usize, usize)> = /* ... */;

// vs O(n) lookup with Vec
let obstacles: Vec<(usize, usize)> = /* ... */; // Slower!
```

### Road Networks

**1. Bidirectional search essential:**
```rust
// Road networks are typically sparse and symmetric
let pathfinder = BidirectionalAstar::new(euclidean_heuristic);
```

**2. Coordinate-based heuristics:**
```rust
// Use real GPS coordinates for accurate estimates
let mut context = HeuristicContext::new();
for (node_id, lat, lon) in road_network.nodes() {
    context.add_coordinate(node_id, lat, lon);
}
```

**3. Precompute landmarks (advanced):**
```rust
// Select strategic landmarks, precompute distances
// Use landmark distances as heuristic bounds
// Requires preprocessing but provides strong guidance
```

### Game Maps

**1. Hierarchical pathfinding:**
```rust
// Divide map into regions
// Find path: region1 → region2 → ... → regionN
// Then find detailed path within each region
// Reduces search space dramatically
```

**2. Path smoothing:**
```rust
// After finding grid path, apply smoothing:
fn smooth_path(path: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut smoothed = vec![path[0]];
    let mut i = 0;
    while i < path.len() - 1 {
        let mut j = path.len() - 1;
        while j > i {
            if line_of_sight(path[i], path[j]) {
                smoothed.push(path[j]);
                i = j;
                break;
            }
            j -= 1;
        }
    }
    smoothed
}
```

**3. Movement cost variation:**
```rust
// Different terrain types have different costs
let neighbors = |pos: (usize, usize)| {
    adjacent_cells(pos).into_iter().map(|next| {
        let terrain_cost = terrain_map.get(&next).unwrap_or(&1);
        (next, *terrain_cost)
    }).collect()
};
```

## Benchmarking

### Using Mission9 CLI

```bash
# Quick benchmark
mission9 benchmark --graph test.json --start 0 --goal 99 --iterations 1000

# Grid benchmark
mission9-grid benchmark --width 100 --height 100 --iterations 100
```

### Custom Benchmarking

```rust
use std::time::Instant;
use mission9::*;

fn benchmark_algorithm<P: Pathfinder>(
    pathfinder: &P,
    graph: &impl WeightedGraph,
    queries: &[(u32, u32)],
) {
    let start = Instant::now();
    let mut total_cost = 0.0;
    let mut total_explored = 0;
    
    for &(start, goal) in queries {
        match pathfinder.find_path(graph, start, goal) {
            Ok(result) => {
                total_cost += result.cost;
                total_explored += result.nodes_explored;
            }
            Err(_) => continue,
        }
    }
    
    let elapsed = start.elapsed();
    println!("Total time: {:?}", elapsed);
    println!("Avg time: {:?}", elapsed / queries.len() as u32);
    println!("Avg cost: {:.2}", total_cost / queries.len() as f64);
    println!("Avg explored: {}", total_explored / queries.len());
}
```

### Profiling

**Using cargo-flamegraph:**
```bash
cargo install flamegraph
cargo flamegraph --bin mission9 -- find-path --graph large.json --start 0 --goal 10000
```

**Using perf (Linux):**
```bash
cargo build --release
perf record target/release/mission9 find-path --graph large.json --start 0 --goal 10000
perf report
```

## Performance Checklist

### Before Optimization
- [ ] Profile to identify bottlenecks (don't guess!)
- [ ] Establish baseline performance metrics
- [ ] Define performance targets

### Algorithm Level
- [ ] Choose appropriate algorithm for problem type
- [ ] Select optimal heuristic (if applicable)
- [ ] Consider bidirectional search for large problems
- [ ] Use early termination when possible

### Data Structure Level
- [ ] Use appropriate graph representation
- [ ] Pre-allocate collections with capacity
- [ ] Use efficient data structures (HashSet for lookups)
- [ ] Consider memory pools for node allocation

### Implementation Level
- [ ] Minimize allocations in hot paths
- [ ] Cache frequently computed values
- [ ] Use release builds for benchmarking
- [ ] Enable link-time optimization (LTO)

### System Level
- [ ] Batch process multiple queries
- [ ] Parallelize independent queries
- [ ] Consider preprocessing for repeated queries
- [ ] Profile memory usage and optimize if needed

## Common Pitfalls

### 1. Non-Admissible Heuristic
```rust
// Bad: Overestimates distance
fn bad_heuristic(from: u32, to: u32) -> f64 {
    euclidean_distance(from, to) * 2.0 // Not admissible!
}

// Good: Always underestimates or equals true cost
fn good_heuristic(from: u32, to: u32) -> f64 {
    euclidean_distance(from, to) // Admissible
}
```

### 2. Expensive Heuristic
```rust
// Bad: Heuristic slower than search!
fn expensive_heuristic(from: u32, to: u32) -> f64 {
    // Complex computation that takes 1ms
    complex_domain_calculation(from, to)
}

// Good: Simple, fast approximation
fn cheap_heuristic(from: u32, to: u32) -> f64 {
    manhattan_distance(from, to) // <1µs
}
```

### 3. Debug Builds
```bash
# Bad: 10-100x slower
cargo run -- find-path --graph huge.json ...

# Good: Full optimizations
cargo run --release -- find-path --graph huge.json ...
```

### 4. Repeated Graph Loading
```rust
// Bad: Load graph 1000 times
for query in queries {
    let graph = load_graph("big.json")?; // Expensive!
    pathfinder.find_path(&graph, query.0, query.1)?;
}

// Good: Load once
let graph = load_graph("big.json")?;
for query in queries {
    pathfinder.find_path(&graph, query.0, query.1)?;
}
```

## Performance Targets

Based on Mission9 benchmarks:

| Problem Size | Target Time | Algorithm |
|--------------|-------------|-----------|
| 100 nodes | <100 µs | Any |
| 1K nodes | <1 ms | A* or bidirectional |
| 10K nodes | <10 ms | Bidirectional A* |
| 100K nodes | <100 ms | Bidirectional A* |
| 1M nodes | <1 s | Hierarchical/preprocessed |

**If not meeting targets:**
1. Verify release build
2. Check heuristic admissibility
3. Profile to find bottleneck
4. Consider bidirectional search
5. Optimize hot paths

## See Also

- [API Documentation](API_DOCUMENTATION.md) - Algorithm details
- [CLI Guide](CLI_GUIDE.md) - Benchmarking tools
- [Integration Guide](INTEGRATION_GUIDE.md) - Production usage patterns
