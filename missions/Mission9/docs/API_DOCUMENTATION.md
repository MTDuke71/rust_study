# Mission 9 API Documentation

## Overview

Mission 9 provides a comprehensive pathfinding library with multiple algorithms, optimization strategies, and utilities for graph-based navigation problems.

## Core Traits

### `Pathfinder` Trait

The main trait for pathfinding algorithms operating on weighted graphs.

```rust
pub trait Pathfinder {
    fn find_path(
        &self,
        graph: &impl WeightedGraph,
        start: u32,
        goal: u32,
    ) -> Result<PathResult, PathfindingError>;
}
```

**Implemented by:**
- `DijkstraPathfinder`
- `AstarPathfinder`

**Example:**
```rust
use mission9::*;

// Create graph
let mut graph = SimpleWeightedGraph::new(5);
graph.add_edge(0, 1, 1.0)?;
graph.add_edge(1, 2, 1.0)?;

// Use Dijkstra
let pathfinder = DijkstraPathfinder::new();
let result = pathfinder.find_path(&graph, 0, 2)?;

println!("Path cost: {}", result.cost);
println!("Path: {:?}", result.path);
```

### `WeightedGraph` Trait

Trait for graph data structures with weighted edges.

```rust
pub trait WeightedGraph {
    fn node_count(&self) -> usize;
    fn neighbors(&self, node: u32) -> Vec<(u32, f64)>;
    fn has_edge(&self, from: u32, to: u32) -> bool;
    fn edge_weight(&self, from: u32, to: u32) -> Option<f64>;
}
```

**Implemented by:**
- `SimpleWeightedGraph`

### `Heuristic` Trait

Trait for A* heuristic functions.

```rust
pub trait Heuristic {
    fn estimate(&self, from: u32, to: u32, context: &HeuristicContext) -> f64;
    
    fn is_admissible(&self) -> bool {
        true // Default: assume admissible
    }
}
```

**Implemented by:**
- `ManhattanHeuristic`
- `EuclideanHeuristic`
- `ChebyshevHeuristic`
- `ZeroHeuristic`
- `WeightedCombinationHeuristic`

## Algorithm Implementations

### Dijkstra's Algorithm

Classic shortest path algorithm with guaranteed optimality.

```rust
pub struct DijkstraPathfinder {
    // Internal state
}

impl DijkstraPathfinder {
    pub fn new() -> Self;
}
```

**Characteristics:**
- **Time Complexity**: O((V + E) log V)
- **Space Complexity**: O(V)
- **Optimality**: Always finds optimal path
- **Requirements**: Non-negative edge weights

**Example:**
```rust
use mission9::*;

let pathfinder = DijkstraPathfinder::new();
let result = pathfinder.find_path(&graph, start, goal)?;

// Access results
println!("Cost: {}", result.cost);
println!("Nodes explored: {}", result.nodes_explored);
println!("Time: {:?}", result.search_time);
```

### A* Algorithm

Informed search using heuristics for faster pathfinding.

```rust
pub struct AstarPathfinder<H: Heuristic> {
    heuristic: H,
}

impl<H: Heuristic> AstarPathfinder<H> {
    pub fn new(heuristic: H) -> Self;
}
```

**Characteristics:**
- **Time Complexity**: O(b^d) where b is branching factor, d is depth
- **Space Complexity**: O(b^d)
- **Optimality**: Guaranteed with admissible heuristic
- **Requirements**: Non-negative weights, heuristic function

**Example:**
```rust
use mission9::*;

// Create with built-in heuristic
let pathfinder = AstarPathfinder::new(EuclideanHeuristic);

// Or create context for coordinate-based heuristics
let mut context = HeuristicContext::new();
context.add_coordinate(0, 0.0, 0.0);
context.add_coordinate(1, 1.0, 0.0);

let result = pathfinder.find_path_with_context(&graph, start, goal, &context)?;
```

### Bidirectional Dijkstra

Searches from both start and goal simultaneously.

```rust
pub struct BidirectionalDijkstra {
    // Internal state
}

impl BidirectionalDijkstra {
    pub fn new() -> Self;
    
    pub fn find_path<N>(
        &mut self,
        start: (usize, usize),
        goal: (usize, usize),
        neighbors: N,
    ) -> Option<(u32, Vec<(usize, usize)>)>
    where
        N: Fn((usize, usize)) -> Vec<((usize, usize), u32)>;
}
```

**Characteristics:**
- **Time Complexity**: O((V + E) log V), but typically faster than unidirectional
- **Space Complexity**: O(V)
- **Optimality**: Always finds optimal path
- **Best for**: Grid-based problems, symmetric graphs

**Example:**
```rust
use mission9::*;

let mut pathfinder = BidirectionalDijkstra::new();

// Define neighbor function for grid
let neighbors = |pos: (usize, usize)| {
    let mut result = Vec::new();
    if pos.0 > 0 {
        result.push(((pos.0 - 1, pos.1), 1));
    }
    // Add more directions...
    result
};

if let Some((cost, path)) = pathfinder.find_path((0, 0), (10, 10), neighbors) {
    println!("Found path with cost: {}", cost);
}
```

### Bidirectional A*

Bidirectional search with heuristic guidance.

```rust
pub struct BidirectionalAstar<H> {
    heuristic: H,
}

impl<H> BidirectionalAstar<H>
where
    H: Fn((usize, usize), (usize, usize)) -> u32,
{
    pub fn new(heuristic: H) -> Self;
    
    pub fn find_path<N>(
        &mut self,
        start: (usize, usize),
        goal: (usize, usize),
        neighbors: N,
    ) -> Option<(u32, Vec<(usize, usize)>)>
    where
        N: Fn((usize, usize)) -> Vec<((usize, usize), u32)>;
}
```

**Characteristics:**
- **Time Complexity**: Better than unidirectional A* in practice
- **Space Complexity**: O(V)
- **Optimality**: Guaranteed with admissible heuristic
- **Best for**: Large grids with good heuristics

**Example:**
```rust
use mission9::*;

// Manhattan distance heuristic
let heuristic = |from: (usize, usize), to: (usize, usize)| {
    (from.0.abs_diff(to.0) + from.1.abs_diff(to.1)) as u32
};

let mut pathfinder = BidirectionalAstar::new(heuristic);
let result = pathfinder.find_path((0, 0), (50, 50), neighbors);
```

## Multi-Objective Pathfinding

### Multi-Objective A*

Finds Pareto-optimal paths considering multiple objectives.

```rust
pub struct MultiObjectiveAstar {
    max_iterations: usize,
}

impl MultiObjectiveAstar {
    pub fn new() -> Self;
    
    pub fn with_max_iterations(max_iterations: usize) -> Self;
    
    pub fn find_pareto_paths(
        &self,
        graph: &impl WeightedGraph,
        start: u32,
        goal: u32,
        objectives: &[ObjectiveFunction],
    ) -> Result<Vec<PathSolution>, PathfindingError>;
}
```

**Example:**
```rust
use mission9::*;

let pathfinder = MultiObjectiveAstar::new();

// Define objectives: minimize cost, minimize time, maximize safety
let objectives = vec![
    ObjectiveFunction::Cost,
    ObjectiveFunction::Time,
    ObjectiveFunction::Safety,
];

let solutions = pathfinder.find_pareto_paths(&graph, start, goal, &objectives)?;

for (i, solution) in solutions.iter().enumerate() {
    println!("Solution {}: Cost={}, Time={}, Safety={}",
             i, solution.objectives[0], solution.objectives[1], solution.objectives[2]);
}
```

### Constrained A*

Pathfinding with user-defined constraints.

```rust
pub struct ConstrainedAstar {
    max_iterations: usize,
}

impl ConstrainedAstar {
    pub fn new() -> Self;
    
    pub fn find_path(
        &self,
        graph: &impl WeightedGraph,
        start: u32,
        goal: u32,
        constraints: &[PathConstraint],
    ) -> Result<PathResult, PathfindingError>;
}
```

**Available Constraints:**
```rust
pub enum PathConstraint {
    ForbiddenZone(Vec<u32>),           // Avoid specific nodes
    MaxCost(f64),                       // Path cost limit
    MaxLength(usize),                   // Path length limit
    NoCycle,                            // Prevent revisiting nodes
    Waypoint(Vec<u32>),                 // Must pass through nodes
}
```

**Example:**
```rust
use mission9::*;

let pathfinder = ConstrainedAstar::new();

// Create constraints
let constraints = vec![
    PathConstraint::ForbiddenZone(vec![5, 6, 7]), // Avoid nodes 5, 6, 7
    PathConstraint::MaxCost(20.0),                 // Total cost ≤ 20
    PathConstraint::Waypoint(vec![10]),            // Must pass through node 10
];

let result = pathfinder.find_path(&graph, start, goal, &constraints)?;
```

## Data Structures

### `SimpleWeightedGraph`

Basic adjacency list graph implementation.

```rust
pub struct SimpleWeightedGraph {
    // Internal storage
}

impl SimpleWeightedGraph {
    pub fn new(node_count: usize) -> Self;
    pub fn add_edge(&mut self, from: u32, to: u32, weight: f64) -> Result<(), PathfindingError>;
    pub fn remove_edge(&mut self, from: u32, to: u32) -> Result<(), PathfindingError>;
}
```

**Example:**
```rust
use mission9::*;

let mut graph = SimpleWeightedGraph::new(5);
graph.add_edge(0, 1, 1.0)?;
graph.add_edge(1, 2, 2.0)?;
graph.add_edge(2, 3, 1.5)?;
graph.add_edge(3, 4, 1.0)?;

// Query graph
assert_eq!(graph.node_count(), 5);
assert_eq!(graph.edge_weight(0, 1), Some(1.0));
```

### `PathResult`

Result of a pathfinding operation.

```rust
pub struct PathResult {
    pub path: Vec<u32>,
    pub cost: f64,
    pub nodes_explored: usize,
    pub search_time: Duration,
}
```

### `HeuristicContext`

Context for coordinate-based heuristics.

```rust
pub struct HeuristicContext {
    coordinates: HashMap<u32, (f64, f64)>,
}

impl HeuristicContext {
    pub fn new() -> Self;
    pub fn add_coordinate(&mut self, node: u32, x: f64, y: f64);
    pub fn get_coordinate(&self, node: u32) -> Option<(f64, f64)>;
}
```

## Built-in Heuristics

### Manhattan Distance

```rust
pub struct ManhattanHeuristic;

impl Heuristic for ManhattanHeuristic {
    fn estimate(&self, from: u32, to: u32, context: &HeuristicContext) -> f64 {
        // |x1 - x2| + |y1 - y2|
    }
}
```

**Best for:** Grid-based movement (4-directional)

### Euclidean Distance

```rust
pub struct EuclideanHeuristic;

impl Heuristic for EuclideanHeuristic {
    fn estimate(&self, from: u32, to: u32, context: &HeuristicContext) -> f64 {
        // sqrt((x1-x2)^2 + (y1-y2)^2)
    }
}
```

**Best for:** Direct-line distance, any-angle movement

### Chebyshev Distance

```rust
pub struct ChebyshevHeuristic;

impl Heuristic for ChebyshevHeuristic {
    fn estimate(&self, from: u32, to: u32, context: &HeuristicContext) -> f64 {
        // max(|x1 - x2|, |y1 - y2|)
    }
}
```

**Best for:** 8-directional grid movement

### Zero Heuristic

```rust
pub struct ZeroHeuristic;

impl Heuristic for ZeroHeuristic {
    fn estimate(&self, _from: u32, _to: u32, _context: &HeuristicContext) -> f64 {
        0.0 // Makes A* equivalent to Dijkstra
    }
}
```

**Best for:** Testing, comparison with Dijkstra

### Weighted Combination

```rust
pub struct WeightedCombinationHeuristic {
    heuristics: Vec<(Box<dyn Heuristic>, f64)>,
}

impl WeightedCombinationHeuristic {
    pub fn new() -> Self;
    pub fn add_heuristic(&mut self, heuristic: Box<dyn Heuristic>, weight: f64);
}
```

**Example:**
```rust
let mut combined = WeightedCombinationHeuristic::new();
combined.add_heuristic(Box::new(EuclideanHeuristic), 0.7);
combined.add_heuristic(Box::new(ManhattanHeuristic), 0.3);

let pathfinder = AstarPathfinder::new(combined);
```

## Error Handling

### `PathfindingError`

```rust
pub enum PathfindingError {
    NoPath {
        start: u32,
        goal: u32,
        nodes_explored: usize,
    },
    InvalidNode(u32),
    InvalidEdge {
        from: u32,
        to: u32,
    },
    NegativeWeight(f64),
    Timeout {
        max_iterations: usize,
        nodes_explored: usize,
    },
    InvalidInput(String),
}
```

**Example:**
```rust
use mission9::*;

match pathfinder.find_path(&graph, start, goal) {
    Ok(result) => println!("Path found: {:?}", result.path),
    Err(PathfindingError::NoPath { start, goal, nodes_explored }) => {
        eprintln!("No path from {} to {} (explored {} nodes)", start, goal, nodes_explored);
    }
    Err(PathfindingError::InvalidNode(node)) => {
        eprintln!("Node {} does not exist", node);
    }
    Err(e) => eprintln!("Error: {:?}", e),
}
```

## File I/O Utilities

### Loading Graphs

```rust
pub fn load_graph_json(path: &str) -> Result<(SimpleWeightedGraph, HeuristicContext), PathfindingError>;
pub fn load_graph_csv(path: &str) -> Result<SimpleWeightedGraph, PathfindingError>;
```

**Example:**
```rust
use mission9::graph_loader::*;

// Load JSON with coordinates
let (graph, context) = load_graph_json("graph.json")?;

// Load CSV (edges only)
let graph = load_graph_csv("edges.csv")?;
```

### Saving Graphs

```rust
pub fn save_graph_json(
    graph: &SimpleWeightedGraph,
    path: &str,
    coordinates: Option<&HashMap<u32, Coordinate>>,
) -> Result<(), PathfindingError>;

pub fn save_graph_csv(
    graph: &SimpleWeightedGraph,
    path: &str,
) -> Result<(), PathfindingError>;
```

### Visualization

```rust
pub fn save_visualization(
    graph: &SimpleWeightedGraph,
    path: &str,
    result_path: Option<&[u32]>,
    coordinates: Option<&HashMap<u32, Coordinate>>,
) -> Result<(), PathfindingError>;
```

**Example:**
```rust
use mission9::graph_loader::*;

let result = pathfinder.find_path(&graph, start, goal)?;
save_visualization(&graph, "output.dot", Some(&result.path), Some(&coords))?;

// Render with Graphviz:
// dot -Tpng output.dot -o output.png
```

### Graph Generation

```rust
pub fn generate_graph(
    graph_type: &str,
    nodes: usize,
) -> Result<(SimpleWeightedGraph, Option<HashMap<u32, Coordinate>>), PathfindingError>;
```

**Supported types:**
- `"grid"` - Square grid with bidirectional edges
- `"random"` - Random edges with deterministic PRNG
- `"tree"` - Binary tree structure
- `"complete"` - Fully connected graph

**Example:**
```rust
use mission9::graph_loader::*;

let (graph, coords) = generate_graph("grid", 100)?;
```

## Complete Example

```rust
use mission9::*;
use mission9::graph_loader::*;

fn main() -> Result<(), PathfindingError> {
    // Generate test graph
    let (graph, coords) = generate_graph("grid", 100)?;
    
    // Save for later use
    save_graph_json(&graph, "test_graph.json", coords.as_ref())?;
    
    // Create context from coordinates
    let mut context = HeuristicContext::new();
    if let Some(ref coords) = coords {
        for (&node, &coord) in coords.iter() {
            context.add_coordinate(node, coord.x, coord.y);
        }
    }
    
    // Compare Dijkstra vs A*
    let dijkstra = DijkstraPathfinder::new();
    let astar = AstarPathfinder::new(EuclideanHeuristic);
    
    let start = 0;
    let goal = 99;
    
    let d_result = dijkstra.find_path(&graph, start, goal)?;
    println!("Dijkstra: cost={}, explored={}, time={:?}",
             d_result.cost, d_result.nodes_explored, d_result.search_time);
    
    let a_result = astar.find_path_with_context(&graph, start, goal, &context)?;
    println!("A*: cost={}, explored={}, time={:?}",
             a_result.cost, a_result.nodes_explored, a_result.search_time);
    
    // Generate visualization
    save_visualization(&graph, "path.dot", Some(&a_result.path), coords.as_ref())?;
    
    Ok(())
}
```

## Thread Safety

All pathfinding algorithms are `Send` but not `Sync`. For concurrent pathfinding:

```rust
use std::thread;
use mission9::*;

let graph = /* ... */;
let queries = vec![(0, 10), (5, 15), (20, 30)];

let handles: Vec<_> = queries.into_iter().map(|(start, goal)| {
    let graph = graph.clone(); // Clone graph for each thread
    thread::spawn(move || {
        let pathfinder = DijkstraPathfinder::new();
        pathfinder.find_path(&graph, start, goal)
    })
}).collect();

for handle in handles {
    let result = handle.join().unwrap()?;
    println!("Path cost: {}", result.cost);
}
```

## See Also

- [CLI Guide](CLI_GUIDE.md) - Command-line usage
- [Performance Tuning Guide](PERFORMANCE_TUNING.md) - Optimization strategies
- [Integration Guide](INTEGRATION_GUIDE.md) - Using Mission9 in your projects
