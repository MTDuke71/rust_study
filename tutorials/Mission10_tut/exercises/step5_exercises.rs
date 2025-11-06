//! Step 5 Exercises: Real-World Applications
//!
//! Practice problems for applying Union-Find to real-world scenarios.
//! These exercises reinforce the concepts from step5_applications.rs
//!
//! Solutions provided at the bottom of the file (try before looking!)

use std::collections::HashMap;

// ============================================================================
// Exercise 1: Percolation Simulation
// ============================================================================

/// **Exercise 1: Percolation Simulation**
///
/// Implement a percolation system using Union-Find.
///
/// **Problem**: Given an N×N grid, sites are initially blocked. Open sites
/// randomly. The system "percolates" when there's a path of open sites from
/// the top row to the bottom row.
///
/// **Your Task**:
/// 1. Create a `Percolation` struct with N×N grid
/// 2. Implement `open(row, col)` to open a site
/// 3. Implement `percolates()` to check if system percolates
/// 4. Use virtual top and bottom nodes for efficiency
///
/// **Hint**: Create virtual nodes at indices N*N (top) and N*N+1 (bottom).
/// Connect all top-row sites to virtual top, all bottom-row sites to virtual bottom.
/// System percolates when virtual top and bottom are connected.
///
/// **Example**:
/// ```text
/// 3×3 grid:
/// ⬛⬜⬛     Open (0,1)
/// ⬛⬜⬛     Open (1,1)
/// ⬛⬜⬛     Open (2,1)
/// System percolates! (top to bottom path exists)
/// ```
#[allow(dead_code)]
struct Percolation {
    // TODO: Add fields
    // Hint: Need grid size, open/blocked status, Union-Find structure
}

#[allow(dead_code)]
impl Percolation {
    fn new(n: usize) -> Self {
        // TODO: Initialize N×N grid, create Union-Find with N*N + 2 elements
        // (extra 2 for virtual top and bottom nodes)
        todo!()
    }

    fn open(&mut self, row: usize, col: usize) {
        // TODO: 
        // 1. Mark site as open
        // 2. Union with adjacent open sites (up, down, left, right)
        // 3. If top row, union with virtual top
        // 4. If bottom row, union with virtual bottom
        todo!()
    }

    fn is_open(&self, row: usize, col: usize) -> bool {
        // TODO: Check if site is open
        todo!()
    }

    fn percolates(&mut self) -> bool {
        // TODO: Check if virtual top and virtual bottom are connected
        todo!()
    }
}

// ============================================================================
// Exercise 2: Dynamic Connectivity with Deletions
// ============================================================================

/// **Exercise 2: Union-Find with Edge Deletion Support**
///
/// Standard Union-Find doesn't support edge deletion efficiently.
/// Implement a version that can delete edges.
///
/// **Your Task**:
/// 1. Track all edges in a separate data structure
/// 2. When deleting an edge, rebuild Union-Find from remaining edges
/// 3. Optimize by only rebuilding affected components (harder!)
///
/// **Example**:
/// ```text
/// Add edge 0-1, 1-2, 2-3  → All connected
/// Delete edge 1-2         → {0,1} and {2,3} now separate
/// Query connected(0, 3)   → false
/// ```
#[allow(dead_code)]
struct DynamicUnionFind {
    // TODO: Add fields
    // Hint: Store all edges, rebuild UF on deletion
}

#[allow(dead_code)]
impl DynamicUnionFind {
    fn new(n: usize) -> Self {
        // TODO: Initialize
        todo!()
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        // TODO: Add edge and update Union-Find
        todo!()
    }

    fn remove_edge(&mut self, u: usize, v: usize) {
        // TODO: Remove edge and rebuild Union-Find
        todo!()
    }

    fn connected(&mut self, u: usize, v: usize) -> bool {
        // TODO: Check connectivity
        todo!()
    }
}

// ============================================================================
// Exercise 3: Least Common Ancestor (LCA) using Union-Find
// ============================================================================

/// **Exercise 3: Offline LCA using Tarjan's Algorithm**
///
/// Find the Least Common Ancestor (LCA) of node pairs in a tree using Union-Find.
/// This is an "offline" algorithm (all queries known in advance).
///
/// **Algorithm**:
/// 1. Process tree in post-order DFS
/// 2. After processing subtree, union node with parent
/// 3. When both nodes of a query are processed, their LCA is the root of one's set
///
/// **Your Task**:
/// Implement Tarjan's offline LCA algorithm
///
/// **Example**:
/// ```text
/// Tree:       1
///            / \
///           2   3
///          / \
///         4   5
///
/// Query LCA(4, 5) → 2
/// Query LCA(4, 3) → 1
/// ```
#[allow(dead_code)]
struct LCAFinder {
    // TODO: Add fields for tree structure, Union-Find, queries
}

#[allow(dead_code)]
impl LCAFinder {
    fn new(n: usize) -> Self {
        // TODO: Initialize
        todo!()
    }

    fn add_edge(&mut self, parent: usize, child: usize) {
        // TODO: Build tree structure
        todo!()
    }

    fn add_query(&mut self, u: usize, v: usize) {
        // TODO: Store LCA query
        todo!()
    }

    fn find_lcas(&mut self) -> Vec<usize> {
        // TODO: Run Tarjan's algorithm, return LCAs for all queries
        todo!()
    }
}

// ============================================================================
// Exercise 4: Maze Generation
// ============================================================================

/// **Exercise 4: Random Maze Generation using Union-Find**
///
/// Generate a random maze where every cell is reachable from every other cell.
///
/// **Algorithm**:
/// 1. Start with N×M grid, all walls present (cells disconnected)
/// 2. Pick random wall between two cells
/// 3. If cells are in different components, remove wall and union them
/// 4. Repeat until all cells are connected
///
/// **Your Task**:
/// Implement maze generator using Union-Find
///
/// **Example**:
/// ```text
/// 5×5 maze:
/// ┌─┬─┬─┬─┬─┐
/// │     │   │
/// ├─┼ ┼ ┼ ┼ │
/// │ │ │   │ │
/// ├ ┼─┼─┼ ┼ │
/// │       │ │
/// ├─┼─┼─┼ ┼ │
/// │ │     │ │
/// └─┴─┴─┴─┴─┘
/// ```
#[allow(dead_code)]
struct MazeGenerator {
    // TODO: Add fields for grid dimensions, Union-Find, walls
}

#[allow(dead_code)]
impl MazeGenerator {
    fn new(rows: usize, cols: usize) -> Self {
        // TODO: Initialize grid and Union-Find
        todo!()
    }

    fn generate(&mut self) {
        // TODO: Randomly remove walls until all cells connected
        todo!()
    }

    fn display(&self) {
        // TODO: Print maze using box-drawing characters
        todo!()
    }
}

// ============================================================================
// Exercise 5: Number of Islands (LeetCode 200)
// ============================================================================

/// **Exercise 5: Count Islands using Union-Find**
///
/// Given a 2D grid of '1's (land) and '0's (water), count the number of islands.
/// An island is surrounded by water and formed by connecting adjacent lands
/// horizontally or vertically.
///
/// **Your Task**:
/// Implement `num_islands` using Union-Find
///
/// **Example**:
/// ```text
/// Grid:
/// 1 1 0 0 0
/// 1 1 0 0 0
/// 0 0 1 0 0
/// 0 0 0 1 1
///
/// Output: 3 islands
/// ```
fn num_islands(grid: Vec<Vec<char>>) -> usize {
    // TODO: Implement using Union-Find
    // Hint: Treat water as separate entities, count components of land
    let _ = grid;
    todo!()
}

// ============================================================================
// Exercise 6: Accounts Merge (LeetCode 721)
// ============================================================================

/// **Exercise 6: Merge Accounts with Common Emails**
///
/// Given a list of accounts where each account has a name and list of emails,
/// merge accounts that share at least one email.
///
/// **Your Task**:
/// Implement account merging using Union-Find
///
/// **Example**:
/// ```text
/// Input:
/// accounts = [
///   ["John", "john@mail.com", "john_work@mail.com"],
///   ["John", "john@mail.com", "john_home@mail.com"],
///   ["Mary", "mary@mail.com"]
/// ]
///
/// Output:
/// [
///   ["John", "john@mail.com", "john_home@mail.com", "john_work@mail.com"],
///   ["Mary", "mary@mail.com"]
/// ]
/// ```
fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    // TODO: Implement using Union-Find
    // Hint: Map emails to account indices, union accounts with shared emails
    let _ = accounts;
    todo!()
}

// ============================================================================
// Exercise 7: Redundant Connection (LeetCode 684)
// ============================================================================

/// **Exercise 7: Find Redundant Connection**
///
/// Given a graph that started as a tree with N nodes (0 to N-1) and N edges,
/// one additional edge was added. Find the edge that can be removed to make
/// it a tree again. If multiple answers exist, return the last occurring edge.
///
/// **Your Task**:
/// Use Union-Find to detect the edge that creates a cycle
///
/// **Example**:
/// ```text
/// Input: edges = [[1,2], [1,3], [2,3]]
/// Output: [2,3]  (this edge creates the cycle)
/// ```
fn find_redundant_connection(edges: Vec<(usize, usize)>) -> (usize, usize) {
    // TODO: Implement using Union-Find
    // Hint: The first edge that connects already-connected nodes is redundant
    let _ = edges;
    todo!()
}

// ============================================================================
// SOLUTIONS (Try exercises first before looking!)
// ============================================================================

#[cfg(test)]
mod solutions {
    use super::*;

    // Solution structure for Union-Find (for solutions)
    struct UnionFind {
        parent: Vec<usize>,
        rank: Vec<usize>,
    }

    impl UnionFind {
        fn new(n: usize) -> Self {
            UnionFind {
                parent: (0..n).collect(),
                rank: vec![0; n],
            }
        }

        fn find(&mut self, x: usize) -> usize {
            if self.parent[x] != x {
                self.parent[x] = self.find(self.parent[x]);
            }
            self.parent[x]
        }

        fn union(&mut self, x: usize, y: usize) -> bool {
            let root_x = self.find(x);
            let root_y = self.find(y);

            if root_x == root_y {
                return false;
            }

            match self.rank[root_x].cmp(&self.rank[root_y]) {
                std::cmp::Ordering::Less => self.parent[root_x] = root_y,
                std::cmp::Ordering::Greater => self.parent[root_y] = root_x,
                std::cmp::Ordering::Equal => {
                    self.parent[root_y] = root_x;
                    self.rank[root_x] += 1;
                }
            }
            true
        }

        fn connected(&mut self, x: usize, y: usize) -> bool {
            self.find(x) == self.find(y)
        }
    }

    // Solution 1: Percolation
    struct PercolationSolution {
        n: usize,
        open_sites: Vec<bool>,
        uf: UnionFind,
        virtual_top: usize,
        virtual_bottom: usize,
    }

    impl PercolationSolution {
        fn new(n: usize) -> Self {
            let total = n * n + 2; // +2 for virtual nodes
            PercolationSolution {
                n,
                open_sites: vec![false; n * n],
                uf: UnionFind::new(total),
                virtual_top: n * n,
                virtual_bottom: n * n + 1,
            }
        }

        fn index(&self, row: usize, col: usize) -> usize {
            row * self.n + col
        }

        fn open(&mut self, row: usize, col: usize) {
            let idx = self.index(row, col);
            self.open_sites[idx] = true;

            // Connect to virtual top if top row
            if row == 0 {
                self.uf.union(idx, self.virtual_top);
            }

            // Connect to virtual bottom if bottom row
            if row == self.n - 1 {
                self.uf.union(idx, self.virtual_bottom);
            }

            // Union with adjacent open sites
            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            for (dr, dc) in directions.iter() {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;

                if new_row >= 0 && new_row < self.n as i32 
                   && new_col >= 0 && new_col < self.n as i32 {
                    let neighbor = self.index(new_row as usize, new_col as usize);
                    if self.open_sites[neighbor] {
                        self.uf.union(idx, neighbor);
                    }
                }
            }
        }

        fn percolates(&mut self) -> bool {
            self.uf.connected(self.virtual_top, self.virtual_bottom)
        }
    }

    // Solution 5: Number of Islands
    fn num_islands_solution(grid: Vec<Vec<char>>) -> usize {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }

        let rows = grid.len();
        let cols = grid[0].len();
        let mut uf = UnionFind::new(rows * cols);
        let mut water_count = 0;

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '0' {
                    water_count += 1;
                } else {
                    // Union with right neighbor
                    if c + 1 < cols && grid[r][c + 1] == '1' {
                        uf.union(r * cols + c, r * cols + c + 1);
                    }
                    // Union with bottom neighbor
                    if r + 1 < rows && grid[r + 1][c] == '1' {
                        uf.union(r * cols + c, (r + 1) * cols + c);
                    }
                }
            }
        }

        // Count unique components (subtract water)
        let mut roots = std::collections::HashSet::new();
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '1' {
                    roots.insert(uf.find(r * cols + c));
                }
            }
        }
        roots.len()
    }

    // Solution 7: Redundant Connection
    fn find_redundant_connection_solution(edges: Vec<(usize, usize)>) -> (usize, usize) {
        let n = edges.len();
        let mut uf = UnionFind::new(n + 1);

        for &(u, v) in &edges {
            if uf.connected(u, v) {
                return (u, v); // This edge creates a cycle
            }
            uf.union(u, v);
        }

        (0, 0) // Should never reach here
    }

    #[test]
    fn test_percolation() {
        let mut perc = PercolationSolution::new(3);
        assert!(!perc.percolates());

        perc.open(0, 1);
        perc.open(1, 1);
        perc.open(2, 1);

        assert!(perc.percolates());
    }

    #[test]
    fn test_num_islands() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        assert_eq!(num_islands_solution(grid), 3);
    }

    #[test]
    fn test_redundant_connection() {
        let edges = vec![(1, 2), (1, 3), (2, 3)];
        assert_eq!(find_redundant_connection_solution(edges), (2, 3));
    }
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  Step 5: Real-World Applications - Exercises            ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    println!("This file contains 7 exercises for practicing Union-Find applications.\n");

    println!("Exercises:");
    println!("  1. Percolation Simulation");
    println!("  2. Dynamic Connectivity with Deletions");
    println!("  3. Least Common Ancestor (Tarjan's Algorithm)");
    println!("  4. Maze Generation");
    println!("  5. Number of Islands (LeetCode 200)");
    println!("  6. Accounts Merge (LeetCode 721)");
    println!("  7. Redundant Connection (LeetCode 684)\n");

    println!("Instructions:");
    println!("  1. Read each exercise description");
    println!("  2. Implement the TODO sections");
    println!("  3. Run tests: cargo test");
    println!("  4. Check solutions at bottom (try first!)\n");

    println!("Difficulty:");
    println!("  ⭐     - Exercises 5, 7 (straightforward application)");
    println!("  ⭐⭐   - Exercises 1, 4 (require creativity)");
    println!("  ⭐⭐⭐ - Exercises 2, 3, 6 (advanced techniques)\n");

    println!("For detailed solutions, see the `solutions` module at the bottom.");
}
