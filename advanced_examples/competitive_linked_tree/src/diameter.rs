use crate::tree::{Tree, NodeId};
use mission2::LinkedQueue;

/// Tree diameter calculator using two-pass BFS with LinkedQueue
/// 
/// # Requirements Satisfied: REQ-B1, REQ-B4, REQ-D1, REQ-D5
/// 
/// Uses LinkedQueue for dynamic growth during BFS traversal, perfect for
/// trees where queue size is unknown and varies between different BFS runs.
pub struct TreeDiameter {
    tree: Tree,
}

/// BFS result containing farthest node and distance information
/// 
/// # Requirements Satisfied: REQ-B1, REQ-B3
#[derive(Debug, Clone)]
pub struct BfsResult {
    /// Farthest node from the starting node
    pub farthest_node: NodeId,
    /// Distance to the farthest node
    pub distance: usize,
    /// Parent pointers for path reconstruction
    pub parents: Vec<Option<NodeId>>,
}

impl TreeDiameter {
    /// Create a new tree diameter calculator
    /// 
    /// # Requirements Satisfied: REQ-T1, REQ-T2, REQ-T4
    pub fn from_edges(n: usize, edges: Vec<(NodeId, NodeId)>) -> Option<Self> {
        Tree::from_edges(n, edges).map(|tree| Self { tree })
    }
    
    /// Create from existing tree
    pub fn new(tree: Tree) -> Self {
        Self { tree }
    }
    
    /// Find tree diameter using two-pass BFS algorithm
    /// 
    /// # Requirements Satisfied: REQ-D1, REQ-D2, REQ-D5
    /// 
    /// # Algorithm
    /// 1. Run BFS from arbitrary node (node 0) to find farthest node A
    /// 2. Run BFS from node A to find farthest node B and distance
    /// 3. Distance from A to B is the diameter, path A->B is a diameter path
    /// 
    /// # Returns
    /// - Some((diameter, path)) if tree is non-empty
    /// - None if tree is empty
    /// 
    /// # Time Complexity
    /// O(n) where n is number of nodes (two BFS traversals)
    pub fn find_diameter(&self) -> Option<(usize, Vec<NodeId>)> {
        if self.tree.is_empty() {
            return None; // REQ-D4
        }
        
        if self.tree.size() == 1 {
            return Some((0, vec![0])); // REQ-D4: Single node
        }
        
        // First BFS: Find farthest node from node 0 (REQ-D1)
        let first_result = self.bfs_farthest(0);
        
        // Second BFS: Find farthest node from the result of first BFS (REQ-D1)
        let second_result = self.bfs_farthest(first_result.farthest_node);
        
        // Diameter is the distance from second BFS (REQ-D2)
        let diameter = second_result.distance;
        
        // Reconstruct path from first_result.farthest_node to second_result.farthest_node (REQ-D3)
        let path = self.reconstruct_path(
            &second_result.parents,
            first_result.farthest_node,
            second_result.farthest_node,
        );
        
        Some((diameter, path))
    }
    
    /// Run BFS to find the farthest node from a given start node
    /// 
    /// # Requirements Satisfied: REQ-B1, REQ-B2, REQ-B3, REQ-B5
    /// 
    /// Uses LinkedQueue which grows dynamically as we discover more nodes.
    /// Perfect for trees where we don't know the BFS queue size in advance.
    fn bfs_farthest(&self, start: NodeId) -> BfsResult {
        let n = self.tree.size();
        
        // Initialize data structures
        let mut queue = LinkedQueue::new(); // REQ-B2: Dynamic growth
        let mut visited = vec![false; n];
        let mut distances = vec![0; n];
        let mut parents = vec![None; n];
        
        // Start BFS (REQ-B1)
        queue.enqueue(start);
        visited[start] = true;
        distances[start] = 0;
        parents[start] = None;
        
        let mut farthest_node = start;
        let mut max_distance = 0;
        
        // BFS main loop (REQ-B1, REQ-B5)
        while let Some(current) = queue.dequeue() {
            let current_distance = distances[current];
            
            // Update farthest node if current is farther (REQ-B3)
            if current_distance > max_distance {
                max_distance = current_distance;
                farthest_node = current;
            }
            
            // Explore neighbors
            for neighbor in self.tree.neighbors(current) {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    distances[neighbor] = current_distance + 1;
                    parents[neighbor] = Some(current);
                    
                    // LinkedQueue grows automatically - no capacity concerns (REQ-B2)
                    queue.enqueue(neighbor);
                }
            }
        }
        
        BfsResult {
            farthest_node,
            distance: max_distance,
            parents,
        }
    }
    
    /// Reconstruct path using parent pointers
    /// 
    /// # Requirements Satisfied: REQ-D3
    fn reconstruct_path(
        &self,
        parents: &[Option<NodeId>],
        start: NodeId,
        end: NodeId,
    ) -> Vec<NodeId> {
        let mut path = Vec::new();
        let mut current = end;
        
        // Trace back from end to start
        loop {
            path.push(current);
            
            if current == start {
                break;
            }
            
            if let Some(parent) = parents[current] {
                current = parent;
            } else {
                // Should not happen in connected tree
                break;
            }
        }
        
        // Reverse to get path from start to end
        path.reverse();
        path
    }
    
    /// Get tree size for analysis
    pub fn size(&self) -> usize {
        self.tree.size()
    }
    
    /// Find tree center (node(s) that minimize maximum distance to any other node)
    /// 
    /// Bonus feature demonstrating additional BFS applications.
    /// The center is always on the diameter path.
    pub fn find_center(&self) -> Option<Vec<NodeId>> {
        if let Some((diameter, path)) = self.find_diameter() {
            if path.len() == 1 {
                return Some(path); // Single node is the center
            }
            
            let center_idx = diameter / 2;
            if diameter % 2 == 0 {
                // Even diameter: single center
                Some(vec![path[center_idx]])
            } else {
                // Odd diameter: two centers
                Some(vec![path[center_idx], path[center_idx + 1]])
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // REQ-D1, REQ-D5
    fn req_d1_d5_line_graph_diameter() {
        // Line graph: 0-1-2-3-4, diameter should be 4 (0 to 4)
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
        let diameter_finder = TreeDiameter::from_edges(5, edges).expect("Valid tree");
        
        let (diameter, path) = diameter_finder.find_diameter().expect("Should find diameter");
        
        assert_eq!(diameter, 4);
        assert_eq!(path.len(), 5); // Path includes both endpoints
        assert!(path == vec![0, 1, 2, 3, 4] || path == vec![4, 3, 2, 1, 0]);
    }

    #[test] // REQ-D1, REQ-D5  
    fn req_d1_d5_star_graph_diameter() {
        // Star graph: center=0, leaves=1,2,3,4. Diameter=2 (any leaf to any other leaf)
        let edges = vec![(0, 1), (0, 2), (0, 3), (0, 4)];
        let diameter_finder = TreeDiameter::from_edges(5, edges).expect("Valid tree");
        
        let (diameter, path) = diameter_finder.find_diameter().expect("Should find diameter");
        
        assert_eq!(diameter, 2);
        assert_eq!(path.len(), 3); // leaf -> center -> leaf
        assert_eq!(path[1], 0); // Center should be in the middle
    }

    #[test] // REQ-D4
    fn req_d4_edge_cases() {
        // Empty tree
        let diameter_finder = TreeDiameter::from_edges(0, vec![]).expect("Valid empty tree");
        assert!(diameter_finder.find_diameter().is_none());
        
        // Single node
        let diameter_finder = TreeDiameter::from_edges(1, vec![]).expect("Valid single node");
        let (diameter, path) = diameter_finder.find_diameter().expect("Should handle single node");
        assert_eq!(diameter, 0);
        assert_eq!(path, vec![0]);
    }

    #[test] // REQ-B1, REQ-B3
    fn req_b1_b3_bfs_correctness() {
        // Create tree: 0-1-2-3
        //                 |
        //                 4
        let edges = vec![(0, 1), (1, 2), (2, 3), (1, 4)];
        let diameter_finder = TreeDiameter::from_edges(5, edges).expect("Valid tree");
        
        // BFS from node 0 should find node 3 as farthest (distance 3)
        let result = diameter_finder.bfs_farthest(0);
        assert_eq!(result.distance, 3);
        assert_eq!(result.farthest_node, 3);
    }

    #[test] // REQ-D2, REQ-D3
    fn req_d2_d3_path_reconstruction() {
        // Create binary tree:
        //       1
        //      / \
        //     0   2
        //        / \
        //       3   4
        let edges = vec![(1, 0), (1, 2), (2, 3), (2, 4)];
        let diameter_finder = TreeDiameter::from_edges(5, edges).expect("Valid tree");
        
        let (diameter, path) = diameter_finder.find_diameter().expect("Should find diameter");
        
        // Diameter should be from 0 to 3 or 0 to 4 (distance 3)
        assert_eq!(diameter, 3);
        assert_eq!(path.len(), 4);
        
        // Verify path is contiguous
        for window in path.windows(2) {
            let curr = window[0];
            let next = window[1];
            let neighbors: Vec<_> = diameter_finder.tree.neighbors(curr).collect();
            assert!(neighbors.contains(&next), "Path should be contiguous");
        }
    }

    #[test] // REQ-B2
    fn req_b2_dynamic_queue_growth() {
        // Create large balanced tree to test LinkedQueue growth
        // Tree structure: 0 connected to 1,2,3,4, each connected to 2 leaves
        let mut edges = vec![(0, 1), (0, 2), (0, 3), (0, 4)];
        let mut node_id = 5;
        
        for parent in 1..=4 {
            edges.push((parent, node_id));
            edges.push((parent, node_id + 1));
            node_id += 2;
        }
        
        // Total: 13 nodes, BFS queue will grow and shrink dynamically
        let diameter_finder = TreeDiameter::from_edges(13, edges).expect("Valid tree");
        
        let (diameter, path) = diameter_finder.find_diameter().expect("Should find diameter");
        
        // Diameter should be 4 (from one leaf to another through center)
        assert_eq!(diameter, 4);
        assert_eq!(path.len(), 5);
    }

    #[test] // REQ-B5
    fn req_b5_large_tree_performance() {
        // Create deep tree to test LinkedQueue with larger sizes
        let n = 1000;
        let mut edges = Vec::new();
        
        // Create line graph 0-1-2-...-999
        for i in 0..n-1 {
            edges.push((i, i + 1));
        }
        
        let diameter_finder = TreeDiameter::from_edges(n, edges).expect("Valid tree");
        
        let (diameter, path) = diameter_finder.find_diameter().expect("Should find diameter");
        
        assert_eq!(diameter, n - 1); // Distance from 0 to 999
        assert_eq!(path.len(), n);   // Path includes all nodes
    }

    #[test] // Bonus: tree center
    fn tree_center_finding() {
        // Line graph: 0-1-2-3-4, center should be node 2
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
        let diameter_finder = TreeDiameter::from_edges(5, edges).expect("Valid tree");
        
        let centers = diameter_finder.find_center().expect("Should find center");
        assert_eq!(centers, vec![2]);
        
        // Even length line: 0-1-2-3, centers should be nodes 1 and 2
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let diameter_finder = TreeDiameter::from_edges(4, edges).expect("Valid tree");
        
        let centers = diameter_finder.find_center().expect("Should find centers");
        assert_eq!(centers.len(), 2);
        assert!(centers.contains(&1));
        assert!(centers.contains(&2));
    }
}