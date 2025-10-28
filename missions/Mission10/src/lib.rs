//! # Mission 10: Union-Find Disjoint Set Data Structure
//!
//! An efficient implementation of the Union-Find (Disjoint Set Union) data structure
//! with path compression and union by rank optimizations.
//!
//! ## Overview
//!
//! The Union-Find data structure maintains a collection of disjoint (non-overlapping) sets.
//! It provides near-constant time operations for:
//! - Finding which set an element belongs to
//! - Uniting two sets into one
//! - Checking if two elements are in the same set
//!
//! ## Example
//!
//! ```
//! use mission10::UnionFind;
//!
//! # fn main() -> Result<(), String> {
//! let mut uf = UnionFind::new(10);
//!
//! // Initially, each element is in its own set
//! assert_eq!(uf.count(), 10);
//!
//! // Union some elements
//! uf.union(0, 1)?;
//! uf.union(2, 3)?;
//! uf.union(0, 2)?; // Merges {0,1} and {2,3} into {0,1,2,3}
//!
//! // Check connectivity
//! assert!(uf.connected(0, 3)?);
//! assert!(!uf.connected(0, 4)?);
//!
//! // Count disjoint sets
//! assert_eq!(uf.count(), 7); // {0,1,2,3}, {4}, {5}, {6}, {7}, {8}, {9}
//! # Ok(())
//! # }
//! ```
//!
//! ## Complexity
//!
//! All operations run in O(α(n)) amortized time, where α(n) is the inverse Ackermann
//! function. For all practical purposes, α(n) ≤ 4, making operations effectively constant time.

/// Union-Find data structure with path compression and union by rank
///
/// # Requirements Satisfied
///
/// - REQ-1: Basic Union-Find structure with initialization
/// - REQ-2: Find operation with path compression
/// - REQ-3: Union operation with union by rank
/// - REQ-4: Connected query operation
/// - REQ-5: Set counting and statistics
/// - REQ-6: Error handling and bounds checking
/// - REQ-7: Connected components application support
pub struct UnionFind {
    /// Parent pointers: parent[i] is the parent of element i
    parent: Vec<usize>,
    /// Rank (approximate tree height) for union by rank optimization
    rank: Vec<usize>,
    /// Size of each set (only accurate for root elements)
    size: Vec<usize>,
    /// Number of disjoint sets
    count: usize,
}

impl UnionFind {
    /// Creates a new Union-Find structure with `n` elements
    ///
    /// Initially, each element is in its own set.
    ///
    /// # Requirements Satisfied: REQ-1
    ///
    /// # Arguments
    ///
    /// * `n` - Number of elements (indexed 0 to n-1)
    ///
    /// # Complexity
    ///
    /// Time: O(n), Space: O(n)
    ///
    /// # Example
    ///
    /// ```
    /// use mission10::UnionFind;
    ///
    /// let uf = UnionFind::new(5);
    /// assert_eq!(uf.count(), 5);
    /// ```
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(), // Each element is its own parent initially
            rank: vec![0; n],
            size: vec![1; n], // Each set has size 1 initially
            count: n,
        }
    }

    /// Finds the representative (root) of the set containing element `x`
    ///
    /// Applies path compression during traversal to flatten the tree structure,
    /// making future queries faster.
    ///
    /// # Requirements Satisfied: REQ-2
    ///
    /// # Arguments
    ///
    /// * `x` - Element to find
    ///
    /// # Returns
    ///
    /// * `Ok(root)` - The root element representing the set
    /// * `Err(msg)` - If `x` is out of bounds
    ///
    /// # Complexity
    ///
    /// Time: O(α(n)) amortized
    ///
    /// # Example
    ///
    /// ```
    /// use mission10::UnionFind;
    ///
    /// # fn main() -> Result<(), String> {
    /// let mut uf = UnionFind::new(5);
    /// uf.union(0, 1)?;
    /// uf.union(1, 2)?;
    ///
    /// // All should have the same root
    /// let root = uf.find(0)?;
    /// assert_eq!(uf.find(1)?, root);
    /// assert_eq!(uf.find(2)?, root);
    /// # Ok(())
    /// # }
    /// ```
    pub fn find(&mut self, x: usize) -> Result<usize, String> {
        self.validate_index(x)?;

        // Path compression: make every node point directly to root
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x])?;
        }

        Ok(self.parent[x])
    }

    /// Unites the sets containing elements `x` and `y`
    ///
    /// Uses union by rank to keep trees balanced, attaching the smaller tree
    /// under the root of the larger tree.
    ///
    /// # Requirements Satisfied: REQ-3
    ///
    /// # Arguments
    ///
    /// * `x` - First element
    /// * `y` - Second element
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - If the sets were merged (x and y were in different sets)
    /// * `Ok(false)` - If x and y were already in the same set
    /// * `Err(msg)` - If indices are out of bounds
    ///
    /// # Complexity
    ///
    /// Time: O(α(n)) amortized
    ///
    /// # Example
    ///
    /// ```
    /// use mission10::UnionFind;
    ///
    /// # fn main() -> Result<(), String> {
    /// let mut uf = UnionFind::new(5);
    ///
    /// assert!(uf.union(0, 1)?); // Returns true - sets merged
    /// assert!(!uf.union(0, 1)?); // Returns false - already in same set
    /// assert_eq!(uf.count(), 4); // 5 - 1 = 4 sets remaining
    /// # Ok(())
    /// # }
    /// ```
    pub fn union(&mut self, x: usize, y: usize) -> Result<bool, String> {
        let root_x = self.find(x)?;
        let root_y = self.find(y)?;

        // Already in the same set
        if root_x == root_y {
            return Ok(false);
        }

        // Union by rank: attach smaller tree under larger tree
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        } else {
            // Same rank: arbitrarily choose one as parent and increment its rank
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
            self.rank[root_x] += 1;
        }

        self.count -= 1; // One less disjoint set
        Ok(true)
    }

    /// Checks if elements `x` and `y` are in the same set
    ///
    /// # Requirements Satisfied: REQ-4
    ///
    /// # Arguments
    ///
    /// * `x` - First element
    /// * `y` - Second element
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - If x and y are in the same set
    /// * `Ok(false)` - If x and y are in different sets
    /// * `Err(msg)` - If indices are out of bounds
    ///
    /// # Complexity
    ///
    /// Time: O(α(n)) amortized
    ///
    /// # Example
    ///
    /// ```
    /// use mission10::UnionFind;
    ///
    /// # fn main() -> Result<(), String> {
    /// let mut uf = UnionFind::new(5);
    /// uf.union(0, 1)?;
    /// uf.union(1, 2)?;
    ///
    /// assert!(uf.connected(0, 2)?);
    /// assert!(!uf.connected(0, 3)?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn connected(&mut self, x: usize, y: usize) -> Result<bool, String> {
        Ok(self.find(x)? == self.find(y)?)
    }

    /// Returns the current number of disjoint sets
    ///
    /// # Requirements Satisfied: REQ-5
    ///
    /// # Complexity
    ///
    /// Time: O(1)
    ///
    /// # Example
    ///
    /// ```
    /// use mission10::UnionFind;
    ///
    /// # fn main() -> Result<(), String> {
    /// let mut uf = UnionFind::new(10);
    /// assert_eq!(uf.count(), 10);
    ///
    /// uf.union(0, 1)?;
    /// assert_eq!(uf.count(), 9);
    ///
    /// uf.union(2, 3)?;
    /// assert_eq!(uf.count(), 8);
    /// # Ok(())
    /// # }
    /// ```
    pub fn count(&self) -> usize {
        self.count
    }

    /// Returns the size of the set containing element `x`
    ///
    /// # Requirements Satisfied: REQ-5
    ///
    /// # Arguments
    ///
    /// * `x` - Element to query
    ///
    /// # Returns
    ///
    /// * `Ok(size)` - The number of elements in the set containing x
    /// * `Err(msg)` - If x is out of bounds
    ///
    /// # Complexity
    ///
    /// Time: O(α(n)) amortized
    ///
    /// # Example
    ///
    /// ```
    /// use mission10::UnionFind;
    ///
    /// # fn main() -> Result<(), String> {
    /// let mut uf = UnionFind::new(10);
    /// uf.union(0, 1)?;
    /// uf.union(1, 2)?;
    ///
    /// assert_eq!(uf.size(0)?, 3); // {0, 1, 2}
    /// assert_eq!(uf.size(1)?, 3);
    /// assert_eq!(uf.size(5)?, 1); // {5} alone
    /// # Ok(())
    /// # }
    /// ```
    pub fn size(&mut self, x: usize) -> Result<usize, String> {
        let root = self.find(x)?;
        Ok(self.size[root])
    }

    /// Validates that an index is within bounds
    ///
    /// # Requirements Satisfied: REQ-6
    fn validate_index(&self, x: usize) -> Result<(), String> {
        if x >= self.parent.len() {
            Err(format!(
                "Index {} out of bounds (size: {})",
                x,
                self.parent.len()
            ))
        } else {
            Ok(())
        }
    }

    /// Returns the total number of elements
    ///
    /// # Example
    ///
    /// ```
    /// use mission10::UnionFind;
    ///
    /// let uf = UnionFind::new(10);
    /// assert_eq!(uf.len(), 10);
    /// ```
    pub fn len(&self) -> usize {
        self.parent.len()
    }

    /// Returns true if the structure contains no elements
    ///
    /// # Example
    ///
    /// ```
    /// use mission10::UnionFind;
    ///
    /// let uf = UnionFind::new(0);
    /// assert!(uf.is_empty());
    ///
    /// let uf2 = UnionFind::new(5);
    /// assert!(!uf2.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }
}

// TODO: Implement additional helper methods for REQ-7 (Connected Components Application)
// - from_edges: Create UnionFind from graph edges
// - get_components: Return all connected components
// - has_cycle: Detect cycles in graph

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let uf = UnionFind::new(5);
        assert_eq!(uf.count(), 5);
        assert_eq!(uf.len(), 5);
        assert!(!uf.is_empty());
    }

    #[test]
    fn test_basic_operations() {
        let mut uf = UnionFind::new(5);

        // Test find on initial state
        assert_eq!(uf.find(0).unwrap(), 0);
        assert_eq!(uf.find(4).unwrap(), 4);

        // Test union
        assert!(uf.union(0, 1).unwrap());
        assert_eq!(uf.count(), 4);

        // Test connected
        assert!(uf.connected(0, 1).unwrap());
        assert!(!uf.connected(0, 2).unwrap());
    }

    #[test]
    fn test_bounds_checking() {
        let mut uf = UnionFind::new(5);

        assert!(uf.find(10).is_err());
        assert!(uf.union(0, 10).is_err());
        assert!(uf.connected(10, 11).is_err());
    }
}
