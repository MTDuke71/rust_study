//! # Step 7: Real-World Applications & Production Systems
//!
//! This tutorial demonstrates how to build production-ready pathfinding systems
//! that can be deployed in real-world applications. We'll cover:
//!
//! - Command-line interface design
//! - File I/O for graph data (JSON, CSV)
//! - Performance monitoring and metrics
//! - Batch processing capabilities
//! - Error handling for production systems
//! - Integration patterns for different use cases
//!
//! ## Learning Objectives
//!
//! By the end of this tutorial, you will understand:
//! 1. How to design CLIs for pathfinding applications
//! 2. Graph data serialization and deserialization
//! 3. Production-grade error handling
//! 4. Performance metrics collection and reporting
//! 5. Batch processing for multiple queries
//! 6. Real-world integration patterns

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use std::time::Instant;

// ============================================================================
// Part 1: Graph Data Structures for Serialization
// ============================================================================

/// Graph data format for JSON serialization
///
/// This format is easy to read/write and supports optional coordinate data
/// for heuristic-based algorithms.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphData {
    /// Number of nodes in the graph
    pub nodes: usize,
    /// List of edges with weights
    pub edges: Vec<EdgeData>,
    /// Optional coordinate data for nodes (enables A* heuristics)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<HashMap<u32, Coordinate>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EdgeData {
    pub from: u32,
    pub to: u32,
    pub weight: f64,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

// ============================================================================
// Part 2: Simple Weighted Graph Implementation
// ============================================================================

/// Production-ready weighted graph with efficient storage
#[derive(Debug, Clone)]
pub struct SimpleWeightedGraph {
    adjacency_list: Vec<Vec<(u32, f64)>>,
}

impl SimpleWeightedGraph {
    pub fn new(node_count: usize) -> Self {
        SimpleWeightedGraph {
            adjacency_list: vec![Vec::new(); node_count],
        }
    }

    pub fn add_edge(&mut self, from: u32, to: u32, weight: f64) -> Result<(), String> {
        if from as usize >= self.adjacency_list.len() {
            return Err(format!("Node {} out of bounds", from));
        }
        self.adjacency_list[from as usize].push((to, weight));
        Ok(())
    }

    pub fn neighbors(&self, node: u32) -> &[(u32, f64)] {
        &self.adjacency_list[node as usize]
    }

    pub fn node_count(&self) -> usize {
        self.adjacency_list.len()
    }
}

// ============================================================================
// Part 3: File I/O Operations
// ============================================================================

/// Load graph from JSON file
///
/// Example JSON format:
/// ```json
/// {
///   "nodes": 5,
///   "edges": [
///     {"from": 0, "to": 1, "weight": 1.0},
///     {"from": 1, "to": 2, "weight": 1.5}
///   ],
///   "coordinates": {
///     "0": {"x": 0.0, "y": 0.0},
///     "1": {"x": 1.0, "y": 0.0}
///   }
/// }
/// ```
pub fn load_graph_from_json(
    path: &Path,
) -> io::Result<(SimpleWeightedGraph, Option<HashMap<u32, Coordinate>>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let data: GraphData = serde_json::from_reader(reader)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut graph = SimpleWeightedGraph::new(data.nodes);
    for edge in data.edges {
        graph
            .add_edge(edge.from, edge.to, edge.weight)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    }

    Ok((graph, data.coordinates))
}

/// Save graph to JSON file
pub fn save_graph_to_json(
    path: &Path,
    graph: &SimpleWeightedGraph,
    coordinates: Option<&HashMap<u32, Coordinate>>,
) -> io::Result<()> {
    let mut edges = Vec::new();
    for node in 0..graph.node_count() {
        for &(neighbor, weight) in graph.neighbors(node as u32) {
            edges.push(EdgeData {
                from: node as u32,
                to: neighbor,
                weight,
            });
        }
    }

    let data = GraphData {
        nodes: graph.node_count(),
        edges,
        coordinates: coordinates.cloned(),
    };

    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, &data).map_err(io::Error::other)
}

/// Load batch queries from CSV file
///
/// CSV format: start,goal
/// Example:
/// ```csv
/// 0,10
/// 5,15
/// 20,30
/// ```
pub fn load_queries_from_csv(path: &Path) -> io::Result<Vec<(u32, u32)>> {
    let file = File::open(path)?;
    let mut reader = csv::Reader::from_reader(file);
    let mut queries = Vec::new();

    for result in reader.records() {
        let record = result?;
        if record.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "CSV must have at least 2 columns (start, goal)",
            ));
        }

        let start: u32 = record[0]
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid start node"))?;
        let goal: u32 = record[1]
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid goal node"))?;

        queries.push((start, goal));
    }

    Ok(queries)
}

// ============================================================================
// Part 4: Dijkstra Implementation with Metrics
// ============================================================================

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
struct State {
    cost: f64,
    node: u32,
}

impl Eq for State {}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.node == other.node
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Pathfinding result with comprehensive metrics
#[derive(Debug, Clone)]
pub struct PathResult {
    pub path: Vec<u32>,
    pub cost: f64,
    pub nodes_explored: usize,
    pub search_time_ms: f64,
}

/// Dijkstra's algorithm with performance metrics collection
pub fn dijkstra_with_metrics(
    graph: &SimpleWeightedGraph,
    start: u32,
    goal: u32,
) -> Option<PathResult> {
    let start_time = Instant::now();

    let mut heap = BinaryHeap::new();
    let mut distances = vec![f64::INFINITY; graph.node_count()];
    let mut predecessors = vec![None; graph.node_count()];
    let mut nodes_explored = 0;

    distances[start as usize] = 0.0;
    heap.push(State {
        cost: 0.0,
        node: start,
    });

    while let Some(State { cost, node }) = heap.pop() {
        nodes_explored += 1;

        // Goal reached
        if node == goal {
            let elapsed = start_time.elapsed();
            let path = reconstruct_path(&predecessors, start, goal);
            return Some(PathResult {
                path,
                cost,
                nodes_explored,
                search_time_ms: elapsed.as_secs_f64() * 1000.0,
            });
        }

        // Skip if we've found a better path
        if cost > distances[node as usize] {
            continue;
        }

        // Explore neighbors
        for &(neighbor, weight) in graph.neighbors(node) {
            let new_cost = cost + weight;
            if new_cost < distances[neighbor as usize] {
                distances[neighbor as usize] = new_cost;
                predecessors[neighbor as usize] = Some(node);
                heap.push(State {
                    cost: new_cost,
                    node: neighbor,
                });
            }
        }
    }

    None // No path found
}

fn reconstruct_path(predecessors: &[Option<u32>], start: u32, goal: u32) -> Vec<u32> {
    let mut path = vec![goal];
    let mut current = goal;

    while current != start {
        if let Some(pred) = predecessors[current as usize] {
            path.push(pred);
            current = pred;
        } else {
            break;
        }
    }

    path.reverse();
    path
}

// ============================================================================
// Part 5: Batch Processing
// ============================================================================

/// Process multiple pathfinding queries and collect statistics
pub struct BatchProcessor {
    graph: SimpleWeightedGraph,
}

impl BatchProcessor {
    pub fn new(graph: SimpleWeightedGraph) -> Self {
        BatchProcessor { graph }
    }

    /// Process batch of queries and return results
    pub fn process_batch(&self, queries: &[(u32, u32)]) -> Vec<Option<PathResult>> {
        queries
            .iter()
            .map(|&(start, goal)| dijkstra_with_metrics(&self.graph, start, goal))
            .collect()
    }

    /// Process batch and save results to CSV
    pub fn process_and_save(&self, queries: &[(u32, u32)], output_path: &Path) -> io::Result<()> {
        let results = self.process_batch(queries);

        let file = File::create(output_path)?;
        let mut writer = csv::Writer::from_writer(file);

        // Write header
        writer.write_record([
            "start",
            "goal",
            "cost",
            "path_length",
            "nodes_explored",
            "time_ms",
        ])?;

        // Write results
        for (i, result) in results.iter().enumerate() {
            let (start, goal) = queries[i];
            if let Some(path_result) = result {
                writer.write_record(&[
                    start.to_string(),
                    goal.to_string(),
                    format!("{:.2}", path_result.cost),
                    path_result.path.len().to_string(),
                    path_result.nodes_explored.to_string(),
                    format!("{:.2}", path_result.search_time_ms),
                ])?;
            } else {
                writer.write_record(&[
                    start.to_string(),
                    goal.to_string(),
                    "NO_PATH".to_string(),
                    "0".to_string(),
                    "0".to_string(),
                    "0.00".to_string(),
                ])?;
            }
        }

        writer.flush()?;
        Ok(())
    }
}

// ============================================================================
// Part 6: Performance Monitoring
// ============================================================================

/// Performance statistics aggregator
pub struct PerformanceMonitor {
    total_queries: usize,
    successful_queries: usize,
    total_time_ms: f64,
    total_nodes_explored: usize,
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        PerformanceMonitor {
            total_queries: 0,
            successful_queries: 0,
            total_time_ms: 0.0,
            total_nodes_explored: 0,
        }
    }

    pub fn record(&mut self, result: Option<&PathResult>) {
        self.total_queries += 1;
        if let Some(r) = result {
            self.successful_queries += 1;
            self.total_time_ms += r.search_time_ms;
            self.total_nodes_explored += r.nodes_explored;
        }
    }

    pub fn print_summary(&self) {
        println!("\n=== Performance Summary ===");
        println!("Total queries: {}", self.total_queries);
        println!("Successful: {}", self.successful_queries);
        println!(
            "Success rate: {:.1}%",
            (self.successful_queries as f64 / self.total_queries as f64) * 100.0
        );

        if self.successful_queries > 0 {
            println!(
                "Avg time per query: {:.2} ms",
                self.total_time_ms / self.successful_queries as f64
            );
            println!(
                "Avg nodes explored: {:.0}",
                self.total_nodes_explored as f64 / self.successful_queries as f64
            );
        }
        println!("Total time: {:.2} ms", self.total_time_ms);
    }
}

// ============================================================================
// Part 7: Example Applications
// ============================================================================

/// Example 1: Basic pathfinding with metrics
fn example_basic_pathfinding() {
    println!("\n=== Example 1: Basic Pathfinding ===");

    // Create a simple graph
    let mut graph = SimpleWeightedGraph::new(5);
    graph.add_edge(0, 1, 1.0).unwrap();
    graph.add_edge(1, 2, 1.5).unwrap();
    graph.add_edge(2, 3, 1.0).unwrap();
    graph.add_edge(0, 3, 10.0).unwrap(); // Longer alternative path

    // Find path
    if let Some(result) = dijkstra_with_metrics(&graph, 0, 3) {
        println!("Path found: {:?}", result.path);
        println!("Cost: {:.2}", result.cost);
        println!("Nodes explored: {}", result.nodes_explored);
        println!("Search time: {:.2} ms", result.search_time_ms);
    }
}

/// Example 2: File I/O demonstration
fn example_file_io() -> io::Result<()> {
    println!("\n=== Example 2: File I/O ===");

    // Create and save a graph
    let mut graph = SimpleWeightedGraph::new(4);
    graph
        .add_edge(0, 1, 1.0)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    graph
        .add_edge(1, 2, 1.0)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    graph
        .add_edge(2, 3, 1.0)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut coords = HashMap::new();
    coords.insert(0, Coordinate { x: 0.0, y: 0.0 });
    coords.insert(1, Coordinate { x: 1.0, y: 0.0 });
    coords.insert(2, Coordinate { x: 2.0, y: 0.0 });
    coords.insert(3, Coordinate { x: 3.0, y: 0.0 });

    save_graph_to_json(Path::new("example_graph.json"), &graph, Some(&coords))?;
    println!("Graph saved to example_graph.json");

    // Load it back
    let (loaded_graph, loaded_coords) = load_graph_from_json(Path::new("example_graph.json"))?;
    println!("Graph loaded: {} nodes", loaded_graph.node_count());
    if let Some(coords) = loaded_coords {
        println!("Coordinates loaded: {} nodes", coords.len());
    }

    Ok(())
}

/// Example 3: Batch processing
fn example_batch_processing() -> io::Result<()> {
    println!("\n=== Example 3: Batch Processing ===");

    // Create test graph
    let mut graph = SimpleWeightedGraph::new(10);
    for i in 0..9 {
        graph
            .add_edge(i, i + 1, 1.0)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    }

    // Create batch queries
    let queries = vec![(0, 5), (2, 8), (0, 9), (3, 7)];

    // Process batch
    let processor = BatchProcessor::new(graph);
    let mut monitor = PerformanceMonitor::new();

    for &(start, goal) in &queries {
        let result = dijkstra_with_metrics(&processor.graph, start, goal);
        monitor.record(result.as_ref());

        if let Some(r) = result {
            println!(
                "Query ({} → {}): cost={:.2}, time={:.2}ms",
                start, goal, r.cost, r.search_time_ms
            );
        }
    }

    monitor.print_summary();

    // Save to CSV
    processor.process_and_save(&queries, Path::new("batch_results.csv"))?;
    println!("\nResults saved to batch_results.csv");

    Ok(())
}

/// Example 4: Production error handling
fn example_error_handling() {
    println!("\n=== Example 4: Error Handling ===");

    let graph = SimpleWeightedGraph::new(3);

    // Test various error conditions
    match dijkstra_with_metrics(&graph, 0, 5) {
        Some(_) => println!("Path found (unexpected)"),
        None => println!("✓ Handled out-of-bounds goal gracefully"),
    }

    // Test disconnected graph
    let mut disconnected = SimpleWeightedGraph::new(4);
    disconnected.add_edge(0, 1, 1.0).unwrap();
    disconnected.add_edge(2, 3, 1.0).unwrap();

    match dijkstra_with_metrics(&disconnected, 0, 3) {
        Some(_) => println!("Path found (unexpected)"),
        None => println!("✓ Handled disconnected graph gracefully"),
    }
}

// ============================================================================
// Main Demo
// ============================================================================

fn main() -> io::Result<()> {
    println!("=================================================");
    println!("Step 7: Real-World Applications & Production Systems");
    println!("=================================================");

    // Run all examples
    example_basic_pathfinding();
    example_file_io()?;
    example_batch_processing()?;
    example_error_handling();

    println!("\n=== Tutorial Complete! ===");
    println!("\nKey Takeaways:");
    println!("1. Production systems need comprehensive metrics");
    println!("2. File I/O enables data persistence and sharing");
    println!("3. Batch processing improves efficiency");
    println!("4. Error handling is critical for reliability");
    println!("5. Performance monitoring helps identify bottlenecks");

    println!("\n📚 Next Steps:");
    println!("- Integrate with Mission9's dual-CLI system");
    println!("- Add visualization output (DOT format)");
    println!("- Implement A* with heuristics");
    println!("- Add more graph formats (GraphML, etc.)");
    println!("- Build REST API wrapper");

    // Clean up example files
    //let _ = std::fs::remove_file("example_graph.json");
    //let _ = std::fs::remove_file("batch_results.csv");

    Ok(())
}

// ============================================================================
// Exercises
// ============================================================================

#[cfg(test)]
mod exercises {
    use super::*;

    /// Exercise 1: Add CSV graph loading
    ///
    /// TODO: Implement a function that loads graphs from CSV format:
    /// ```csv
    /// from,to,weight
    /// 0,1,1.0
    /// 1,2,1.5
    /// ```
    #[test]
    fn exercise1_csv_graph_loading() {
        // YOUR CODE HERE
        // Hint: Use csv::Reader and parse each row into edges
    }

    /// Exercise 2: Add graph generation utilities
    ///
    /// TODO: Implement functions to generate common graph types:
    /// - Grid graph (for game maps)
    /// - Random graph (for testing)
    /// - Complete graph (all nodes connected)
    #[test]
    fn exercise2_graph_generation() {
        // YOUR CODE HERE
        // Hint: For grid, connect adjacent cells
        // For random, use rand crate to add edges
    }

    /// Exercise 3: Add A* with heuristics
    ///
    /// TODO: Extend the system to support A* algorithm with coordinates
    #[test]
    fn exercise3_astar_integration() {
        // YOUR CODE HERE
        // Hint: Calculate Euclidean distance from coordinates
        // Modify heap to use f(n) = g(n) + h(n)
    }

    /// Exercise 4: Add visualization output
    ///
    /// TODO: Generate DOT format for Graphviz visualization
    /// Output should highlight the found path
    #[test]
    fn exercise4_visualization() {
        // YOUR CODE HERE
        // Hint: DOT format: digraph { 0 -> 1 [label="1.0"]; }
        // Color path edges differently
    }

    /// Exercise 5: CLI argument parsing
    ///
    /// TODO: Use clap to parse command-line arguments:
    /// - Input graph file
    /// - Start/goal nodes
    /// - Output format (text/json/csv)
    #[test]
    fn exercise5_cli_design() {
        // YOUR CODE HERE
        // Hint: Use clap's derive macros for clean argument parsing
    }
}

// ============================================================================
// Solutions (Commented Out)
// ============================================================================

/*
/// Solution to Exercise 1: CSV Graph Loading
pub fn load_graph_from_csv(path: &Path) -> io::Result<SimpleWeightedGraph> {
    let file = File::open(path)?;
    let mut reader = csv::Reader::from_reader(file);

    // First pass: count nodes
    let mut max_node = 0u32;
    for result in reader.records() {
        let record = result?;
        let from: u32 = record[0].parse().unwrap();
        let to: u32 = record[1].parse().unwrap();
        max_node = max_node.max(from).max(to);
    }

    // Second pass: build graph
    let mut graph = SimpleWeightedGraph::new((max_node + 1) as usize);
    let file = File::open(path)?;
    let mut reader = csv::Reader::from_reader(file);

    for result in reader.records() {
        let record = result?;
        let from: u32 = record[0].parse().unwrap();
        let to: u32 = record[1].parse().unwrap();
        let weight: f64 = record[2].parse().unwrap();
        graph.add_edge(from, to, weight)?;
    }

    Ok(graph)
}

/// Solution to Exercise 2: Grid Graph Generation
pub fn generate_grid_graph(width: usize, height: usize) -> SimpleWeightedGraph {
    let mut graph = SimpleWeightedGraph::new(width * height);

    for y in 0..height {
        for x in 0..width {
            let node = (y * width + x) as u32;

            // Right neighbor
            if x + 1 < width {
                let right = (y * width + (x + 1)) as u32;
                graph.add_edge(node, right, 1.0).unwrap();
                graph.add_edge(right, node, 1.0).unwrap();
            }

            // Down neighbor
            if y + 1 < height {
                let down = ((y + 1) * width + x) as u32;
                graph.add_edge(node, down, 1.0).unwrap();
                graph.add_edge(down, node, 1.0).unwrap();
            }
        }
    }

    graph
}
*/
