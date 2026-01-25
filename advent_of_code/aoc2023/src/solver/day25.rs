use std::collections::{HashMap, HashSet, VecDeque};

/// Day 25: Snowverload
/// 
/// Find the three edges that, when removed, split the graph into two components.
/// This is a minimum cut problem - we need to find 3 edges whose removal maximally
/// disconnects the graph.
///
/// # Approach
/// 
/// 1. Build an undirected graph from the input
/// 2. Use BFS to find "bridge" edges that carry lots of flow between components
/// 3. Try removing combinations of high-flow edges
/// 4. Check if removal creates exactly 2 components
/// 
/// # Algorithm: Edge Betweenness
/// 
/// For each edge, count how many shortest paths use it. The 3 edges with highest
/// betweenness are likely the cut edges.

pub fn solve_part1(input: &str) -> usize {
    let graph = parse_input(input);
    
    // Find the 3 edges to cut using edge betweenness
    let cut_edges = find_min_cut(&graph);
    
    // Remove the cut edges and count components
    let (size1, size2) = count_components_after_cut(&graph, &cut_edges);
    
    size1 * size2
}

pub fn solve_part2(_input: &str) -> &'static str {
    "Merry Christmas! 🎄"
}

type Graph = HashMap<String, HashSet<String>>;
type Edge = (String, String);

fn parse_input(input: &str) -> Graph {
    let mut graph: Graph = HashMap::new();
    
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        let (from, to_list) = line.split_once(": ").unwrap();
        let from = from.to_string();
        
        for to in to_list.split_whitespace() {
            let to = to.to_string();
            
            // Add bidirectional edge
            graph.entry(from.clone())
                .or_insert_with(HashSet::new)
                .insert(to.clone());
            
            graph.entry(to.clone())
                .or_insert_with(HashSet::new)
                .insert(from.clone());
        }
    }
    
    graph
}

/// Find edges with highest betweenness (used in many shortest paths)
fn find_min_cut(graph: &Graph) -> Vec<Edge> {
    let mut edge_counts: HashMap<Edge, usize> = HashMap::new();
    
    // Sample BFS from multiple nodes to find frequently-used edges
    let nodes: Vec<_> = graph.keys().collect();
    let sample_size = nodes.len().min(50); // Sample to avoid O(n²) complexity
    
    for i in 0..sample_size {
        let start = nodes[i].clone();
        let paths = bfs_count_paths(graph, &start);
        
        // Track which edges are used in paths from this start node
        for (edge, count) in paths {
            *edge_counts.entry(normalize_edge(edge)).or_insert(0) += count;
        }
    }
    
    // Sort edges by usage count
    let mut edges: Vec<_> = edge_counts.into_iter().collect();
    edges.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
    
    // Return top 3 edges
    edges.into_iter().take(3).map(|(edge, _)| edge).collect()
}

/// BFS that counts how many times each edge is used in shortest paths
fn bfs_count_paths(graph: &Graph, start: &str) -> HashMap<Edge, usize> {
    let mut edge_usage: HashMap<Edge, usize> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = VecDeque::new();
    
    visited.insert(start.to_string());
    queue.push_back(start.to_string());
    
    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&current) {
            for next in neighbors {
                if !visited.contains(next) {
                    visited.insert(next.clone());
                    queue.push_back(next.clone());
                    
                    // Count this edge usage
                    let edge = (current.clone(), next.clone());
                    *edge_usage.entry(edge).or_insert(0) += 1;
                }
            }
        }
    }
    
    edge_usage
}

/// Normalize edge so (a,b) and (b,a) are treated as the same
fn normalize_edge(edge: Edge) -> Edge {
    let (a, b) = edge;
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

/// Count component sizes after removing cut edges
fn count_components_after_cut(graph: &Graph, cut_edges: &[Edge]) -> (usize, usize) {
    // Build graph without cut edges
    let mut modified_graph: Graph = graph.clone();
    
    for (a, b) in cut_edges {
        if let Some(neighbors) = modified_graph.get_mut(a) {
            neighbors.remove(b);
        }
        if let Some(neighbors) = modified_graph.get_mut(b) {
            neighbors.remove(a);
        }
    }
    
    // Find connected components
    let mut visited: HashSet<String> = HashSet::new();
    let mut component_sizes: Vec<usize> = Vec::new();
    
    for node in modified_graph.keys() {
        if !visited.contains(node) {
            let size = component_bfs(&modified_graph, node, &mut visited);
            component_sizes.push(size);
        }
    }
    
    assert_eq!(component_sizes.len(), 2, "Should have exactly 2 components");
    (component_sizes[0], component_sizes[1])
}

/// BFS to find all nodes in a component
fn component_bfs(graph: &Graph, start: &str, visited: &mut HashSet<String>) -> usize {
    let mut queue: VecDeque<String> = VecDeque::new();
    let mut size = 0;
    
    visited.insert(start.to_string());
    queue.push_back(start.to_string());
    size += 1;
    
    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&current) {
            for next in neighbors {
                if !visited.contains(next) {
                    visited.insert(next.clone());
                    queue.push_back(next.clone());
                    size += 1;
                }
            }
        }
    }
    
    size
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_parse_input() {
        let graph = parse_input(EXAMPLE);
        
        // Check bidirectional edges
        assert!(graph.get("jqt").unwrap().contains("rhn"));
        assert!(graph.get("rhn").unwrap().contains("jqt"));
        
        // Check all nodes are present
        assert!(graph.contains_key("jqt"));
        assert!(graph.contains_key("cmg"));
        assert!(graph.contains_key("nvd"));
    }

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE);
        assert_eq!(result, 54); // 9 * 6
    }

    #[test]
    fn test_part2() {
        let result = solve_part2("");
        assert!(result.contains("Christmas"));
    }
}
