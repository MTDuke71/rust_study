/// Graph loading utilities for various file formats
///
/// REQ-7: Support for loading graphs from JSON, CSV, and other formats

use crate::error::PathfindingError;
use crate::graph::{SimpleWeightedGraph, WeightedGraph};
use crate::heuristic::HeuristicContext;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Graph representation for JSON serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    /// Number of nodes in the graph
    pub nodes: u32,
    /// List of edges (from, to, weight)
    pub edges: Vec<EdgeData>,
    /// Optional node coordinates for heuristic calculations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<HashMap<u32, Coordinate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeData {
    pub from: u32,
    pub to: u32,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

/// Load a graph from a JSON file
///
/// # Format
/// ```json
/// {
///   "nodes": 4,
///   "edges": [
///     {"from": 0, "to": 1, "weight": 2.0},
///     {"from": 1, "to": 2, "weight": 3.0}
///   ],
///   "coordinates": {
///     "0": {"x": 0.0, "y": 0.0},
///     "1": {"x": 1.0, "y": 0.0}
///   }
/// }
/// ```
pub fn load_graph_json(
    path: &Path,
) -> Result<(SimpleWeightedGraph, HeuristicContext), PathfindingError> {
    let file = File::open(path).map_err(|e| {
        PathfindingError::InvalidInput(format!("Failed to open file {}: {}", path.display(), e))
    })?;

    let data: GraphData = serde_json::from_reader(file).map_err(|e| {
        PathfindingError::InvalidInput(format!("Failed to parse JSON: {}", e))
    })?;

    let mut graph = SimpleWeightedGraph::new();
    let mut context = HeuristicContext::new();

    // Add edges to graph
    for edge in data.edges {
        graph.add_edge(edge.from, edge.to, edge.weight);
    }

    // Add coordinates if available
    if let Some(coords) = data.coordinates {
        for (node, coord) in coords {
            context.add_coordinates(node, coord.x, coord.y);
        }
    }

    Ok((graph, context))
}

/// Save a graph to a JSON file
pub fn save_graph_json(
    path: &Path,
    graph: &SimpleWeightedGraph,
    context: Option<&HeuristicContext>,
) -> Result<(), PathfindingError> {
    let mut edges = Vec::new();

    for node in 0..graph.node_count() {
        for (neighbor, weight) in graph.neighbors(node as u32) {
            edges.push(EdgeData {
                from: node as u32,
                to: neighbor,
                weight,
            });
        }
    }

    let coordinates = context.and_then(|ctx| {
        let mut coords = HashMap::new();
        for node in 0..graph.node_count() {
            if let Some((x, y)) = ctx.get_coordinates(node as u32) {
                coords.insert(node as u32, Coordinate { x, y });
            }
        }
        if coords.is_empty() {
            None
        } else {
            Some(coords)
        }
    });

    let data = GraphData {
        nodes: graph.node_count() as u32,
        edges,
        coordinates,
    };

    let file = File::create(path).map_err(|e| {
        PathfindingError::InvalidInput(format!("Failed to create file {}: {}", path.display(), e))
    })?;

    serde_json::to_writer_pretty(file, &data).map_err(|e| {
        PathfindingError::InvalidInput(format!("Failed to write JSON: {}", e))
    })?;

    Ok(())
}

/// Load a graph from a CSV file
///
/// # Format
/// CSV with columns: from,to,weight
/// ```csv
/// from,to,weight
/// 0,1,2.0
/// 1,2,3.0
/// ```
pub fn load_graph_csv(path: &Path) -> Result<SimpleWeightedGraph, PathfindingError> {
    let file = File::open(path).map_err(|e| {
        PathfindingError::InvalidInput(format!("Failed to open file {}: {}", path.display(), e))
    })?;

    let mut reader = csv::Reader::from_reader(file);
    let mut graph = SimpleWeightedGraph::new();

    for result in reader.records() {
        let record = result.map_err(|e| {
            PathfindingError::InvalidInput(format!("Failed to read CSV record: {}", e))
        })?;

        if record.len() < 3 {
            return Err(PathfindingError::InvalidInput(
                "CSV must have at least 3 columns: from,to,weight".to_string(),
            ));
        }

        let from: u32 = record[0].parse().map_err(|_| {
            PathfindingError::InvalidInput(format!("Invalid from node: {}", &record[0]))
        })?;

        let to: u32 = record[1].parse().map_err(|_| {
            PathfindingError::InvalidInput(format!("Invalid to node: {}", &record[1]))
        })?;

        let weight: f64 = record[2].parse().map_err(|_| {
            PathfindingError::InvalidInput(format!("Invalid weight: {}", &record[2]))
        })?;

        graph.add_edge(from, to, weight);
    }

    Ok(graph)
}

/// Save a graph to a CSV file
pub fn save_graph_csv(path: &Path, graph: &SimpleWeightedGraph) -> Result<(), PathfindingError> {
    let file = File::create(path).map_err(|e| {
        PathfindingError::InvalidInput(format!("Failed to create file {}: {}", path.display(), e))
    })?;

    let mut writer = csv::Writer::from_writer(file);

    // Write header
    writer
        .write_record(&["from", "to", "weight"])
        .map_err(|e| PathfindingError::InvalidInput(format!("Failed to write CSV: {}", e)))?;

    // Write edges
    for node in 0..graph.node_count() {
        for (neighbor, weight) in graph.neighbors(node as u32) {
            writer
                .write_record(&[
                    (node as u32).to_string(),
                    neighbor.to_string(),
                    weight.to_string(),
                ])
                .map_err(|e| {
                    PathfindingError::InvalidInput(format!("Failed to write CSV: {}", e))
                })?;
        }
    }

    writer
        .flush()
        .map_err(|e| PathfindingError::InvalidInput(format!("Failed to flush CSV: {}", e)))?;

    Ok(())
}

/// Load batch queries from a CSV file
///
/// # Format
/// CSV with columns: start,goal
/// ```csv
/// start,goal
/// 0,5
/// 1,8
/// ```
pub fn load_queries_csv(path: &Path) -> Result<Vec<(u32, u32)>, PathfindingError> {
    let file = File::open(path).map_err(|e| {
        PathfindingError::InvalidInput(format!("Failed to open file {}: {}", path.display(), e))
    })?;

    let mut reader = csv::Reader::from_reader(file);
    let mut queries = Vec::new();

    for result in reader.records() {
        let record = result.map_err(|e| {
            PathfindingError::InvalidInput(format!("Failed to read CSV record: {}", e))
        })?;

        if record.len() < 2 {
            return Err(PathfindingError::InvalidInput(
                "Query CSV must have at least 2 columns: start,goal".to_string(),
            ));
        }

        let start: u32 = record[0].parse().map_err(|_| {
            PathfindingError::InvalidInput(format!("Invalid start node: {}", &record[0]))
        })?;

        let goal: u32 = record[1].parse().map_err(|_| {
            PathfindingError::InvalidInput(format!("Invalid goal node: {}", &record[1]))
        })?;

        queries.push((start, goal));
    }

    Ok(queries)
}

/// Generate a visualization output file
///
/// Creates a simple DOT format file that can be rendered with Graphviz
pub fn save_visualization(
    path: &Path,
    graph: &SimpleWeightedGraph,
    highlighted_path: Option<&[u32]>,
) -> Result<(), PathfindingError> {
    let mut file = File::create(path).map_err(|e| {
        PathfindingError::InvalidInput(format!("Failed to create file {}: {}", path.display(), e))
    })?;

    writeln!(file, "digraph G {{").map_err(|e| {
        PathfindingError::InvalidInput(format!("Failed to write visualization: {}", e))
    })?;

    writeln!(file, "  rankdir=LR;").map_err(|e| {
        PathfindingError::InvalidInput(format!("Failed to write visualization: {}", e))
    })?;

    writeln!(file, "  node [shape=circle];").map_err(|e| {
        PathfindingError::InvalidInput(format!("Failed to write visualization: {}", e))
    })?;

    // Highlight path nodes if provided
    if let Some(path) = highlighted_path {
        for node in path {
            writeln!(
                file,
                "  {} [style=filled, fillcolor=lightblue];",
                node
            )
            .map_err(|e| {
                PathfindingError::InvalidInput(format!("Failed to write visualization: {}", e))
            })?;
        }
    }

    // Write edges
    for node in 0..graph.node_count() {
        let node_u32 = node as u32;
        for (neighbor, weight) in graph.neighbors(node_u32) {
            let is_path_edge = highlighted_path
                .and_then(|path| {
                    path.windows(2)
                        .find(|w| w[0] == node_u32 && w[1] == neighbor)
                })
                .is_some();

            if is_path_edge {
                writeln!(
                    file,
                    "  {} -> {} [label=\"{:.1}\", color=red, penwidth=2.0];",
                    node_u32, neighbor, weight
                )
                .map_err(|e| {
                    PathfindingError::InvalidInput(format!("Failed to write visualization: {}", e))
                })?;
            } else {
                writeln!(file, "  {} -> {} [label=\"{:.1}\"];", node_u32, neighbor, weight)
                    .map_err(|e| {
                        PathfindingError::InvalidInput(format!(
                            "Failed to write visualization: {}",
                            e
                        ))
                    })?;
            }
        }
    }

    writeln!(file, "}}").map_err(|e| {
        PathfindingError::InvalidInput(format!("Failed to write visualization: {}", e))
    })?;

    Ok(())
}

/// Generate a sample graph for testing
pub fn generate_graph(
    graph_type: &str,
    nodes: usize,
) -> Result<(SimpleWeightedGraph, HeuristicContext), PathfindingError> {
    let mut graph = SimpleWeightedGraph::new();
    let mut context = HeuristicContext::new();

    match graph_type {
        "grid" => {
            // Generate a square grid
            let size = (nodes as f64).sqrt().ceil() as usize;

            for y in 0..size {
                for x in 0..size {
                    let node = (y * size + x) as u32;
                    context.add_coordinates(node, x as f64, y as f64);

                    // Add horizontal edges
                    if x < size - 1 {
                        let right = (y * size + (x + 1)) as u32;
                        graph.add_undirected_edge(node, right, 1.0);
                    }

                    // Add vertical edges
                    if y < size - 1 {
                        let below = ((y + 1) * size + x) as u32;
                        graph.add_undirected_edge(node, below, 1.0);
                    }
                }
            }
        }
        "random" => {
            // Generate random graph with approximate density
            use std::collections::HashSet;
            let mut rng = simple_rng(12345);
            let edges_count = nodes * 2; // Approximate density

            let mut added_edges = HashSet::new();

            for _ in 0..edges_count {
                let from = (rng.next() % nodes) as u32;
                let to = (rng.next() % nodes) as u32;

                if from != to && !added_edges.contains(&(from, to)) {
                    let weight = 1.0 + (rng.next() % 10) as f64;
                    graph.add_edge(from, to, weight);
                    added_edges.insert((from, to));
                }
            }
        }
        "tree" => {
            // Generate a tree (each node connected to parent)
            for i in 1..nodes {
                let parent = ((i - 1) / 2) as u32; // Binary tree structure
                let weight = 1.0 + (i % 5) as f64;
                graph.add_undirected_edge(parent, i as u32, weight);
            }
        }
        "complete" => {
            // Generate a complete graph (all nodes connected)
            for i in 0..nodes {
                for j in (i + 1)..nodes {
                    let weight = 1.0 + ((i + j) % 10) as f64;
                    graph.add_undirected_edge(i as u32, j as u32, weight);
                }
            }
        }
        _ => {
            return Err(PathfindingError::InvalidInput(format!(
                "Unknown graph type: {}",
                graph_type
            )))
        }
    }

    Ok((graph, context))
}

// Simple pseudo-random number generator for deterministic graph generation
struct SimpleRng {
    state: u64,
}

fn simple_rng(seed: u64) -> SimpleRng {
    SimpleRng { state: seed }
}

impl SimpleRng {
    fn next(&mut self) -> usize {
        self.state = self.state.wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.state >> 32) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_save_json() {
        let mut graph = SimpleWeightedGraph::new();
        graph.add_edge(0, 1, 2.0);
        graph.add_edge(1, 2, 3.0);

        let mut context = HeuristicContext::new();
        context.add_coordinates(0, 0.0, 0.0);
        context.add_coordinates(1, 1.0, 0.0);

        let mut temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Save
        save_graph_json(path, &graph, Some(&context)).unwrap();

        // Load
        let (loaded_graph, loaded_context) = load_graph_json(path).unwrap();

        assert_eq!(loaded_graph.node_count(), graph.node_count());
        assert_eq!(loaded_graph.edge_count(), graph.edge_count());
        assert_eq!(loaded_context.get_coordinates(0), Some((0.0, 0.0)));
        assert_eq!(loaded_context.get_coordinates(1), Some((1.0, 0.0)));
    }

    #[test]
    fn test_load_save_csv() {
        let mut graph = SimpleWeightedGraph::new();
        graph.add_edge(0, 1, 2.0);
        graph.add_edge(1, 2, 3.0);

        let mut temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Save
        save_graph_csv(path, &graph).unwrap();

        // Load
        let loaded_graph = load_graph_csv(path).unwrap();

        assert_eq!(loaded_graph.node_count(), graph.node_count());
        assert_eq!(loaded_graph.edge_count(), graph.edge_count());
    }

    #[test]
    fn test_generate_grid_graph() {
        let (graph, context) = generate_graph("grid", 9).unwrap();

        // 3x3 grid should have 9 nodes
        assert!(graph.node_count() >= 9);

        // Check that coordinates are set
        assert!(context.get_coordinates(0).is_some());
    }

    #[test]
    fn test_generate_random_graph() {
        let (graph, _) = generate_graph("random", 10).unwrap();
        assert!(graph.node_count() >= 10);
        assert!(graph.edge_count() > 0);
    }
}
