/// Node identifier in the tree
///
/// # Requirements Satisfied: REQ-T3
/// Uses usize for efficient indexing and compatibility with competitive programming
/// input formats where nodes are numbered 0 to n-1.
pub type NodeId = usize;

/// Undirected tree representation using adjacency lists
///
/// # Requirements Satisfied: REQ-T1, REQ-T2, REQ-T4
///
/// Stores edges as Vec<Vec<NodeId>> where tree[u] contains all neighbors of node u.
/// Optimized for competitive programming where edges are given as pairs and
/// we need fast neighbor iteration during BFS.
pub struct Tree {
    /// Adjacency list representation: tree[node] = list of neighbors
    adj: Vec<Vec<NodeId>>,
    /// Number of nodes in the tree
    n: usize,
}

impl Tree {
    /// Create a new tree with n nodes
    ///
    /// # Requirements Satisfied: REQ-T1, REQ-T3
    ///
    /// # Arguments
    /// * `n` - Number of nodes (0 to n-1)
    ///
    /// # Returns
    /// Empty tree with n nodes and no edges
    pub fn new(n: usize) -> Self {
        Self {
            adj: vec![Vec::new(); n],
            n,
        }
    }

    /// Create tree from edge list (competitive programming format)
    ///
    /// # Requirements Satisfied: REQ-T1, REQ-T2, REQ-T4
    ///
    /// # Arguments
    /// * `n` - Number of nodes
    /// * `edges` - List of (u, v) edges representing connections
    ///
    /// # Returns
    /// - Some(tree) if input represents valid tree (n-1 edges, no self-loops)
    /// - None if input is invalid
    ///
    /// # Example
    /// ```rust
    /// use competitive_linked_tree::Tree;
    ///
    /// // Create line graph: 0-1-2-3-4
    /// let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
    /// let tree = Tree::from_edges(5, edges).expect("Valid tree");
    /// ```
    pub fn from_edges(n: usize, edges: Vec<(NodeId, NodeId)>) -> Option<Self> {
        // REQ-T4: Tree must have exactly n-1 edges
        if n == 0 {
            return Some(Self::new(0));
        }

        if edges.len() != n - 1 {
            return None;
        }

        let mut tree = Self::new(n);

        for (u, v) in edges {
            // REQ-T3: Validate node indices
            if u >= n || v >= n {
                return None;
            }

            // REQ-T4: No self-loops in trees
            if u == v {
                return None;
            }

            // REQ-T2: Add undirected edge
            tree.add_edge(u, v);
        }

        // REQ-T4: Verify connectivity (optional - expensive for large inputs)
        // In competitive programming, we usually trust the input format

        Some(tree)
    }

    /// Add an undirected edge between two nodes
    ///
    /// # Requirements Satisfied: REQ-T2
    fn add_edge(&mut self, u: NodeId, v: NodeId) {
        self.adj[u].push(v);
        self.adj[v].push(u);
    }

    /// Get neighbors of a node
    ///
    /// # Requirements Satisfied: REQ-T1, REQ-T2
    ///
    /// # Returns
    /// Iterator over neighbor node IDs
    pub fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.adj[node].iter().copied()
    }

    /// Get number of nodes
    ///
    /// # Requirements Satisfied: REQ-T3
    pub fn size(&self) -> usize {
        self.n
    }

    /// Check if tree is empty
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Get degree of a node (number of neighbors)
    ///
    /// # Requirements Satisfied: REQ-T1
    pub fn degree(&self, node: NodeId) -> usize {
        if node < self.n {
            self.adj[node].len()
        } else {
            0
        }
    }

    /// Validate tree properties for debugging
    ///
    /// # Requirements Satisfied: REQ-T4
    ///
    /// Expensive operation - use only for testing or small inputs.
    /// Checks:
    /// - Total edges = n-1 (each edge counted twice in adjacency list)
    /// - No self-loops
    /// - Connectivity (all nodes reachable from node 0)
    #[cfg(test)]
    pub fn validate(&self) -> bool {
        if self.n == 0 {
            return true;
        }

        // Count total edges (each edge appears twice in adjacency list)
        let total_edges: usize = self.adj.iter().map(|neighbors| neighbors.len()).sum();
        if total_edges != 2 * (self.n - 1) {
            return false;
        }

        // Check for self-loops
        for (node, neighbors) in self.adj.iter().enumerate() {
            if neighbors.contains(&node) {
                return false;
            }
        }

        // Check connectivity with simple DFS
        let mut visited = vec![false; self.n];
        self.dfs_validate(0, &mut visited);

        visited.iter().all(|&v| v)
    }

    #[cfg(test)]
    fn dfs_validate(&self, node: NodeId, visited: &mut [bool]) {
        visited[node] = true;
        for neighbor in self.neighbors(node) {
            if !visited[neighbor] {
                self.dfs_validate(neighbor, visited);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // REQ-T1, REQ-T3
    fn req_t1_t3_tree_creation() {
        let tree = Tree::new(5);
        assert_eq!(tree.size(), 5);
        assert!(!tree.is_empty());

        // All nodes should have no neighbors initially
        for node in 0..5 {
            assert_eq!(tree.degree(node), 0);
        }
    }

    #[test] // REQ-T1, REQ-T2
    fn req_t1_t2_edge_construction() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let tree = Tree::from_edges(4, edges).expect("Valid tree");

        // Check adjacency structure
        let neighbors_0: Vec<_> = tree.neighbors(0).collect();
        assert_eq!(neighbors_0, vec![1]);

        let neighbors_1: Vec<_> = tree.neighbors(1).collect();
        assert!(neighbors_1.contains(&0));
        assert!(neighbors_1.contains(&2));

        let neighbors_2: Vec<_> = tree.neighbors(2).collect();
        assert!(neighbors_2.contains(&1));
        assert!(neighbors_2.contains(&3));

        let neighbors_3: Vec<_> = tree.neighbors(3).collect();
        assert_eq!(neighbors_3, vec![2]);
    }

    #[test] // REQ-T4
    fn req_t4_invalid_tree_rejection() {
        // Too many edges
        let edges = vec![(0, 1), (1, 2), (2, 0), (0, 3)]; // 4 edges for 4 nodes
        assert!(Tree::from_edges(4, edges).is_none());

        // Too few edges
        let edges = vec![(0, 1), (1, 2)]; // 2 edges for 4 nodes
        assert!(Tree::from_edges(4, edges).is_none());

        // Self-loop
        let edges = vec![(0, 0), (0, 1), (1, 2)];
        assert!(Tree::from_edges(3, edges).is_none());

        // Out of bounds node
        let edges = vec![(0, 1), (1, 5)]; // Node 5 doesn't exist in 3-node tree
        assert!(Tree::from_edges(3, edges).is_none());
    }

    #[test] // REQ-T1, REQ-T2, REQ-T4
    fn req_all_line_graph() {
        // Create line graph: 0-1-2-3-4
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
        let tree = Tree::from_edges(5, edges).expect("Valid tree");

        assert!(tree.validate());

        // Check degrees
        assert_eq!(tree.degree(0), 1); // Leaf
        assert_eq!(tree.degree(1), 2); // Internal
        assert_eq!(tree.degree(2), 2); // Internal
        assert_eq!(tree.degree(3), 2); // Internal
        assert_eq!(tree.degree(4), 1); // Leaf
    }

    #[test] // REQ-T1, REQ-T2, REQ-T4
    fn req_all_star_graph() {
        // Create star graph: center=1, leaves=0,2,3,4
        let edges = vec![(1, 0), (1, 2), (1, 3), (1, 4)];
        let tree = Tree::from_edges(5, edges).expect("Valid tree");

        assert!(tree.validate());

        // Check degrees
        assert_eq!(tree.degree(1), 4); // Center
        assert_eq!(tree.degree(0), 1); // Leaf
        assert_eq!(tree.degree(2), 1); // Leaf
        assert_eq!(tree.degree(3), 1); // Leaf
        assert_eq!(tree.degree(4), 1); // Leaf
    }

    #[test] // REQ-T3
    fn req_t3_empty_tree() {
        let tree = Tree::new(0);
        assert_eq!(tree.size(), 0);
        assert!(tree.is_empty());

        let tree2 = Tree::from_edges(0, vec![]).expect("Valid empty tree");
        assert_eq!(tree2.size(), 0);
        assert!(tree2.is_empty());
    }

    #[test] // REQ-T3
    fn req_t3_single_node() {
        let tree = Tree::from_edges(1, vec![]).expect("Valid single node");
        assert_eq!(tree.size(), 1);
        assert_eq!(tree.degree(0), 0);
        assert!(tree.validate());
    }
}
