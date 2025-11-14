//! Integration Tests for Mission 10: Union-Find Disjoint Set
//!
//! Tests real-world applications and complex scenarios:
//! - Kruskal's Minimum Spanning Tree algorithm
//! - Cycle detection in graphs
//! - Dynamic connectivity queries
//! - Social network friend circles
//! - Image segmentation
//! - Percolation simulation

use mission10::UnionFind;
use std::cmp::Ordering;

// ============================================================================
// Kruskal's Minimum Spanning Tree Algorithm
// ============================================================================

/// Edge representation for graph algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Edge {
    u: usize,
    v: usize,
    weight: i32,
}

impl Edge {
    fn new(u: usize, v: usize, weight: i32) -> Self {
        Self { u, v, weight }
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Test Kruskal's MST algorithm using Union-Find
#[test]
fn integration_kruskals_mst() {
    // Graph with 6 vertices
    let n = 6;
    let mut edges = vec![
        Edge::new(0, 1, 4),
        Edge::new(0, 2, 3),
        Edge::new(1, 2, 1),
        Edge::new(1, 3, 2),
        Edge::new(2, 3, 4),
        Edge::new(3, 4, 2),
        Edge::new(4, 5, 6),
    ];
    
    // Sort edges by weight (Kruskal's algorithm step 1)
    edges.sort();
    
    let mut uf = UnionFind::new(n);
    let mut mst_edges = Vec::new();
    let mut total_weight = 0;
    
    // Kruskal's algorithm: add edges that don't create cycles
    for edge in edges {
        if !uf.connected(edge.u, edge.v).unwrap() {
            uf.union(edge.u, edge.v).unwrap();
            mst_edges.push(edge);
            total_weight += edge.weight;
        }
    }
    
    // MST should have n-1 edges
    assert_eq!(mst_edges.len(), n - 1);
    
    // All vertices should be in one component
    assert_eq!(uf.count(), 1);
    
    // Verify MST weight (for this specific graph)
    // Expected MST: (1-2):1, (1-3):2, (3-4):2, (0-2):3, (4-5):6 = 14
    assert_eq!(total_weight, 14);
}

/// Test MST with larger graph
#[test]
fn integration_kruskals_mst_large() {
    let n = 10;
    let mut edges = vec![
        Edge::new(0, 1, 5),
        Edge::new(0, 2, 3),
        Edge::new(0, 3, 7),
        Edge::new(1, 2, 2),
        Edge::new(1, 4, 6),
        Edge::new(2, 3, 4),
        Edge::new(2, 4, 5),
        Edge::new(3, 5, 8),
        Edge::new(4, 5, 9),
        Edge::new(4, 6, 7),
        Edge::new(5, 7, 10),
        Edge::new(6, 7, 11),
        Edge::new(6, 8, 3),
        Edge::new(7, 9, 4),
        Edge::new(8, 9, 2),
    ];
    
    edges.sort();
    
    let mut uf = UnionFind::new(n);
    let mut mst_edges = Vec::new();
    
    for edge in edges {
        if !uf.connected(edge.u, edge.v).unwrap() {
            uf.union(edge.u, edge.v).unwrap();
            mst_edges.push(edge);
        }
    }
    
    assert_eq!(mst_edges.len(), n - 1);
    assert_eq!(uf.count(), 1);
}

// ============================================================================
// Cycle Detection in Undirected Graphs
// ============================================================================

/// Detect if adding an edge creates a cycle
#[test]
fn integration_cycle_detection_basic() {
    let mut uf = UnionFind::new(5);
    
    // Graph: 0-1-2-3-4
    let edges = vec![
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 4),
    ];
    
    // No cycles in this tree
    for (u, v) in edges {
        assert!(!uf.connected(u, v).unwrap());
        uf.union(u, v).unwrap();
    }
    
    // Adding edge 4-0 creates a cycle
    assert!(uf.connected(4, 0).unwrap());
}

/// Test cycle detection during graph construction
#[test]
fn integration_cycle_detection_construction() {
    let mut uf = UnionFind::new(6);
    
    let edges = vec![
        (0, 1, false), // No cycle
        (1, 2, false), // No cycle
        (2, 3, false), // No cycle
        (3, 4, false), // No cycle
        (4, 5, false), // No cycle
        (5, 0, true),  // Creates cycle!
        (1, 3, true),  // Would create cycle
    ];
    
    for (u, v, should_cycle) in edges {
        let has_cycle = uf.connected(u, v).unwrap();
        assert_eq!(has_cycle, should_cycle);
        
        if !has_cycle {
            uf.union(u, v).unwrap();
        }
    }
}

/// Test cycle detection in disconnected graph
#[test]
fn integration_cycle_detection_disconnected() {
    let mut uf = UnionFind::new(8);
    
    // Create two separate components
    // Component 1: 0-1-2-3 (cycle)
    uf.union(0, 1).unwrap();
    uf.union(1, 2).unwrap();
    uf.union(2, 3).unwrap();
    
    // Adding 3-0 creates cycle in component 1
    assert!(uf.connected(3, 0).unwrap());
    
    // Component 2: 4-5-6-7 (no cycle yet)
    uf.union(4, 5).unwrap();
    uf.union(5, 6).unwrap();
    uf.union(6, 7).unwrap();
    
    // Adding 7-4 would create cycle in component 2
    assert!(uf.connected(7, 4).unwrap());
    
    // Adding edge between components doesn't create cycle
    assert!(!uf.connected(0, 4).unwrap());
}

// ============================================================================
// Dynamic Connectivity Queries
// ============================================================================

/// Test dynamic connectivity in network
#[test]
fn integration_dynamic_connectivity() {
    let mut uf = UnionFind::new(10);
    
    // Simulate network connections being added over time
    let connections = vec![
        (0, 1),
        (2, 3),
        (4, 5),
        (6, 7),
        (8, 9),
    ];
    
    for (u, v) in connections {
        uf.union(u, v).unwrap();
    }
    
    assert_eq!(uf.count(), 5); // Five separate networks
    
    // Query connectivity
    assert!(uf.connected(0, 1).unwrap());
    assert!(!uf.connected(0, 2).unwrap());
    
    // Merge networks
    uf.union(1, 2).unwrap(); // Connects {0,1} and {2,3}
    assert!(uf.connected(0, 3).unwrap());
    
    uf.union(5, 6).unwrap(); // Connects {4,5} and {6,7}
    assert!(uf.connected(4, 7).unwrap());
    
    assert_eq!(uf.count(), 3); // Three networks now
}

/// Test online connectivity queries
#[test]
fn integration_online_queries() {
    let mut uf = UnionFind::new(100);
    
    // Simulate stream of operations
    let operations = vec![
        ("union", 5, 10),
        ("union", 15, 20),
        ("union", 25, 30),
        ("query", 5, 10),   // Should be true
        ("query", 5, 15),   // Should be false
        ("union", 10, 15),  // Connect first two sets
        ("query", 5, 20),   // Should now be true
    ];
    
    for (op, x, y) in operations {
        match op {
            "union" => {
                uf.union(x, y).unwrap();
            }
            "query" => {
                let connected = uf.connected(x, y).unwrap();
                if x == 5 && (y == 10 || y == 20) {
                    assert!(connected);
                } else if x == 5 && y == 15 {
                    assert!(!connected);
                }
            }
            _ => {}
        }
    }
}

// ============================================================================
// Social Network Friend Circles
// ============================================================================

/// Model social network with friend relationships
#[test]
fn integration_social_network_basic() {
    let n = 10; // 10 people
    let mut uf = UnionFind::new(n);
    
    // Friend relationships (transitive)
    let friendships = vec![
        (0, 1), // Person 0 and 1 are friends
        (1, 2), // Person 1 and 2 are friends (so 0,1,2 are in one circle)
        (3, 4),
        (4, 5), // Circle: {3,4,5}
        (6, 7), // Circle: {6,7}
        // Person 8 and 9 are alone
    ];
    
    for (p1, p2) in friendships {
        uf.union(p1, p2).unwrap();
    }
    
    // Count friend circles
    assert_eq!(uf.count(), 5); // {0,1,2}, {3,4,5}, {6,7}, {8}, {9}
    
    // Query if people are in same friend circle
    assert!(uf.connected(0, 2).unwrap()); // Transitive friendship
    assert!(!uf.connected(0, 3).unwrap()); // Different circles
    
    // Get circle size
    assert_eq!(uf.size(0).unwrap(), 3);
    assert_eq!(uf.size(3).unwrap(), 3);
    assert_eq!(uf.size(8).unwrap(), 1);
}

/// Test finding largest friend circle
#[test]
fn integration_social_network_largest_circle() {
    let n = 20;
    let mut uf = UnionFind::new(n);
    
    // Create various sized circles
    // Circle 1: 5 people
    for i in 0..4 {
        uf.union(i, i + 1).unwrap();
    }
    
    // Circle 2: 8 people
    for i in 5..12 {
        uf.union(i, i + 1).unwrap();
    }
    
    // Circle 3: 3 people
    uf.union(13, 14).unwrap();
    uf.union(14, 15).unwrap();
    
    // Individuals: 16, 17, 18, 19
    
    // Find largest circle
    let mut max_size = 0;
    for i in 0..n {
        let size = uf.size(i).unwrap();
        if size > max_size {
            max_size = size;
        }
    }
    
    assert_eq!(max_size, 8);
}

/// Test mutual friend suggestions
#[test]
fn integration_social_network_suggestions() {
    let n = 8;
    let mut uf = UnionFind::new(n);
    
    // Current friendships
    uf.union(0, 1).unwrap();
    uf.union(1, 2).unwrap();
    uf.union(3, 4).unwrap();
    uf.union(4, 5).unwrap();
    
    // Person 0 and Person 2 are in same circle (mutual friends through 1)
    assert!(uf.connected(0, 2).unwrap());
    
    // Person 0 and Person 5 are not connected (potential new connection)
    assert!(!uf.connected(0, 5).unwrap());
    
    // Simulate friend suggestion accepted
    uf.union(0, 5).unwrap();
    
    // Now both circles are merged
    assert!(uf.connected(2, 4).unwrap());
    assert_eq!(uf.size(0).unwrap(), 6);
}

// ============================================================================
// Image Segmentation
// ============================================================================

/// Segment image pixels by color similarity
#[test]
fn integration_image_segmentation_basic() {
    // 4x4 image represented as flat array
    // 0 0 1 1
    // 0 0 1 1
    // 2 2 2 3
    // 2 2 2 3
    let pixels = vec![
        0, 0, 1, 1,
        0, 0, 1, 1,
        2, 2, 2, 3,
        2, 2, 2, 3,
    ];
    let width = 4;
    let height = 4;
    
    let mut uf = UnionFind::new(pixels.len());
    
    // Union adjacent pixels with same color
    for row in 0..height {
        for col in 0..width {
            let idx = row * width + col;
            let color = pixels[idx];
            
            // Check right neighbor
            if col + 1 < width {
                let right_idx = row * width + (col + 1);
                if pixels[right_idx] == color {
                    uf.union(idx, right_idx).unwrap();
                }
            }
            
            // Check bottom neighbor
            if row + 1 < height {
                let bottom_idx = (row + 1) * width + col;
                if pixels[bottom_idx] == color {
                    uf.union(idx, bottom_idx).unwrap();
                }
            }
        }
    }
    
    // Should have 4 segments (colors 0, 1, 2, 3)
    assert_eq!(uf.count(), 4);
    
    // Verify segment sizes
    assert_eq!(uf.size(0).unwrap(), 4);  // Color 0: top-left 2x2
    assert_eq!(uf.size(2).unwrap(), 4);  // Color 1: top-right 2x2
    assert_eq!(uf.size(8).unwrap(), 6);  // Color 2: bottom-left
    assert_eq!(uf.size(11).unwrap(), 2); // Color 3: bottom-right
}

/// Test image segmentation with noise
#[test]
fn integration_image_segmentation_noise() {
    // 5x5 image with some noise
    let pixels = vec![
        1, 1, 1, 1, 1,
        1, 2, 1, 1, 1,  // Single noise pixel
        1, 1, 1, 1, 1,
        1, 1, 1, 1, 1,
        1, 1, 1, 1, 1,
    ];
    
    let mut uf = UnionFind::new(pixels.len());
    
    // Union adjacent same-colored pixels
    for row in 0..5 {
        for col in 0..5 {
            let idx = row * 5 + col;
            
            if col + 1 < 5 {
                let right = row * 5 + (col + 1);
                if pixels[idx] == pixels[right] {
                    uf.union(idx, right).unwrap();
                }
            }
            
            if row + 1 < 5 {
                let bottom = (row + 1) * 5 + col;
                if pixels[idx] == pixels[bottom] {
                    uf.union(idx, bottom).unwrap();
                }
            }
        }
    }
    
    // Two segments: large region of 1s (24 pixels) and one noise pixel
    assert_eq!(uf.count(), 2);
    assert_eq!(uf.size(0).unwrap(), 24);  // Main region
    assert_eq!(uf.size(6).unwrap(), 1);   // Noise pixel
}

// ============================================================================
// Percolation Simulation
// ============================================================================

/// Test percolation threshold
#[test]
fn integration_percolation_basic() {
    let n = 5; // 5x5 grid
    let grid_size = n * n;
    
    // Use n*n + 2 for virtual top and bottom nodes
    let mut uf = UnionFind::new(grid_size + 2);
    let virtual_top = grid_size;
    let virtual_bottom = grid_size + 1;
    
    // Open some sites (1 = open, 0 = blocked)
    let open_sites = vec![
        1, 0, 0, 0, 1,
        1, 1, 0, 1, 1,
        0, 1, 0, 1, 0,
        0, 1, 1, 1, 0,
        0, 0, 0, 1, 1,
    ];
    
    // Connect virtual top to top row open sites
    for (col, &site) in open_sites.iter().enumerate().take(n) {
        if site == 1 {
            uf.union(col, virtual_top).unwrap();
        }
    }
    
    // Connect virtual bottom to bottom row open sites
    for col in 0..n {
        let idx = (n - 1) * n + col;
        if open_sites[idx] == 1 {
            uf.union(idx, virtual_bottom).unwrap();
        }
    }
    
    // Connect adjacent open sites
    for row in 0..n {
        for col in 0..n {
            let idx = row * n + col;
            
            if open_sites[idx] == 1 {
                // Right neighbor
                if col + 1 < n && open_sites[idx + 1] == 1 {
                    uf.union(idx, idx + 1).unwrap();
                }
                
                // Bottom neighbor
                if row + 1 < n && open_sites[idx + n] == 1 {
                    uf.union(idx, idx + n).unwrap();
                }
            }
        }
    }
    
    // Check if system percolates (top connects to bottom)
    let percolates = uf.connected(virtual_top, virtual_bottom).unwrap();
    assert!(percolates);
}

/// Test non-percolating system
#[test]
fn integration_percolation_no_path() {
    let n = 4;
    let grid_size = n * n;
    
    let mut uf = UnionFind::new(grid_size + 2);
    let virtual_top = grid_size;
    let virtual_bottom = grid_size + 1;
    
    // Blocked middle row prevents percolation
    let open_sites = vec![
        1, 1, 1, 1,
        1, 1, 1, 1,
        0, 0, 0, 0,  // Blocked row
        1, 1, 1, 1,
    ];
    
    // Connect top row
    for (col, &site) in open_sites.iter().enumerate().take(n) {
        if site == 1 {
            uf.union(col, virtual_top).unwrap();
        }
    }
    
    // Connect bottom row
    for col in 0..n {
        let idx = (n - 1) * n + col;
        if open_sites[idx] == 1 {
            uf.union(idx, virtual_bottom).unwrap();
        }
    }
    
    // Connect adjacent open sites
    for row in 0..n {
        for col in 0..n {
            let idx = row * n + col;
            
            if open_sites[idx] == 1 {
                if col + 1 < n && open_sites[idx + 1] == 1 {
                    uf.union(idx, idx + 1).unwrap();
                }
                
                if row + 1 < n && open_sites[idx + n] == 1 {
                    uf.union(idx, idx + n).unwrap();
                }
            }
        }
    }
    
    // Should NOT percolate
    let percolates = uf.connected(virtual_top, virtual_bottom).unwrap();
    assert!(!percolates);
}

// ============================================================================
// Performance Tests with Real-World Scale
// ============================================================================

/// Test with large graph (performance verification)
#[test]
fn integration_large_scale_performance() {
    let n = 10_000;
    let mut uf = UnionFind::new(n);
    
    // Simulate random network connections
    let edges: Vec<(usize, usize)> = (0..5000)
        .map(|i| {
            let u = (i * 7) % n;
            let v = (i * 13) % n;
            (u, v)
        })
        .collect();
    
    // All operations should complete quickly (O(α(n)) ≈ O(1))
    for (u, v) in edges {
        uf.union(u, v).unwrap();
    }
    
    // Verify some connectivity
    assert!(uf.count() < n); // Some connections were made
}

/// Test many queries after initial construction
#[test]
fn integration_query_intensive() {
    let n = 1000;
    let mut uf = UnionFind::new(n);
    
    // Build some structure
    for i in 0..n / 2 {
        uf.union(i * 2, i * 2 + 1).unwrap();
    }
    
    // Perform many connectivity queries
    for i in 0..n / 2 {
        assert!(uf.connected(i * 2, i * 2 + 1).unwrap());
        if i > 0 {
            assert!(!uf.connected(i * 2, (i - 1) * 2).unwrap());
        }
    }
}

/// Test component extraction with moderate size
#[test]
fn integration_component_extraction() {
    let n = 100;
    let mut uf = UnionFind::new(n);
    
    // Create specific component structure
    // 10 components of size 10 each
    for comp in 0..10 {
        let start = comp * 10;
        for i in 0..9 {
            uf.union(start + i, start + i + 1).unwrap();
        }
    }
    
    assert_eq!(uf.count(), 10);
    
    let components: Vec<Vec<usize>> = uf.components().collect();
    assert_eq!(components.len(), 10);
    
    // Each component should have exactly 10 members
    for component in &components {
        assert_eq!(component.len(), 10);
    }
}
