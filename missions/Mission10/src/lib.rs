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
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, serde::Deserialize))]
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
    /// Uses iterative path compression to flatten the tree structure during traversal,
    /// making future queries faster. This approach is more efficient than recursive
    /// path compression and avoids potential stack overflow on deep trees.
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

        // Iterative path compression for optimal performance
        // First, find the root
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }

        // Then, compress the path by making all nodes point to root
        let mut current = x;
        while current != root {
            let next = self.parent[current];
            self.parent[current] = root;
            current = next;
        }

        Ok(root)
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

    /// Resets all sets to singleton state (each element in its own set)
    ///
    /// # Requirements Satisfied: REQ-2 Extensions
    ///
    /// # Complexity
    ///
    /// Time: O(n), Space: O(1)
    ///
    /// # Example
    ///
    /// ```
    /// use mission10::UnionFind;
    ///
    /// # fn main() -> Result<(), String> {
    /// let mut uf = UnionFind::new(5);
    /// uf.union(0, 1)?;
    /// uf.union(2, 3)?;
    /// assert_eq!(uf.count(), 3);
    ///
    /// uf.clear();
    /// assert_eq!(uf.count(), 5); // Back to original state
    /// assert!(!uf.connected(0, 1)?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn clear(&mut self) {
        let n = self.parent.len();
        for i in 0..n {
            self.parent[i] = i;
            self.rank[i] = 0;
            self.size[i] = 1;
        }
        self.count = n;
    }

    /// Reinitializes the structure with a new size
    ///
    /// # Requirements Satisfied: REQ-2 Extensions
    ///
    /// # Arguments
    ///
    /// * `n` - New number of elements
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
    /// let mut uf = UnionFind::new(5);
    /// assert_eq!(uf.len(), 5);
    ///
    /// uf.reset(10);
    /// assert_eq!(uf.len(), 10);
    /// assert_eq!(uf.count(), 10);
    /// ```
    pub fn reset(&mut self, n: usize) {
        self.parent = (0..n).collect();
        self.rank = vec![0; n];
        self.size = vec![1; n];
        self.count = n;
    }

    /// Returns an iterator over all connected components
    ///
    /// Each component is represented as a vector of element indices.
    ///
    /// # Complexity
    ///
    /// Time: O(n⋅α(n)), Space: O(n)
    ///
    /// # Example
    ///
    /// ```
    /// use mission10::UnionFind;
    ///
    /// # fn main() -> Result<(), String> {
    /// let mut uf = UnionFind::new(6);
    /// uf.union(0, 1)?;
    /// uf.union(2, 3)?;
    /// // Components: {0,1}, {2,3}, {4}, {5}
    ///
    /// let components: Vec<Vec<usize>> = uf.components().collect();
    /// assert_eq!(components.len(), 4);
    /// # Ok(())
    /// # }
    /// ```
    pub fn components(&mut self) -> ComponentIter {
        let mut component_map: std::collections::HashMap<usize, Vec<usize>> = 
            std::collections::HashMap::new();

        // Group elements by their root
        for i in 0..self.parent.len() {
            let root = self.find(i).unwrap(); // Safe because i is always valid
            component_map.entry(root).or_default().push(i);
        }

        ComponentIter {
            components: component_map.into_values().collect(),
            current: 0,
        }
    }

    /// Returns an iterator over all members of the set containing the given element
    ///
    /// # Arguments
    ///
    /// * `element` - Element whose set members to iterate over
    ///
    /// # Returns
    ///
    /// * `Ok(MemberIter)` - Iterator over set members
    /// * `Err(msg)` - If element is out of bounds
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
    /// let members: Vec<usize> = uf.members(0)?.collect();
    /// assert_eq!(members.len(), 3); // {0, 1, 2}
    /// # Ok(())
    /// # }
    /// ```
    pub fn members(&mut self, element: usize) -> Result<MemberIter, String> {
        self.validate_index(element)?;
        let root = self.find(element)?;
        
        let mut members = Vec::new();
        for i in 0..self.parent.len() {
            if self.find(i).unwrap() == root {
                members.push(i);
            }
        }

        Ok(MemberIter {
            members,
            current: 0,
        })
    }
}

/// Iterator over connected components
pub struct ComponentIter {
    components: Vec<Vec<usize>>,
    current: usize,
}

impl Iterator for ComponentIter {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.components.len() {
            let component = self.components[self.current].clone();
            self.current += 1;
            Some(component)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.components.len() - self.current;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for ComponentIter {}

/// Iterator over members of a specific set
pub struct MemberIter {
    members: Vec<usize>,
    current: usize,
}

impl Iterator for MemberIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.members.len() {
            let member = self.members[self.current];
            self.current += 1;
            Some(member)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.members.len() - self.current;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for MemberIter {}

/// Weighted Union-Find variant that supports distance/weight queries
///
/// # Requirements Satisfied: REQ-8
///
/// This variant maintains weights on edges and can answer distance queries
/// between elements in the same component.
#[cfg(feature = "weighted")]
pub struct WeightedUnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    /// Distance from each node to its parent
    dist: Vec<i64>,
    count: usize,
}

#[cfg(feature = "weighted")]
impl WeightedUnionFind {
    /// Creates a new weighted Union-Find structure
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            dist: vec![0; n],
            count: n,
        }
    }

    /// Finds the root of element x and compresses paths with distance updates
    /// 
    /// Note: WeightedUnionFind uses recursive path compression because it must
    /// correctly update distances during compression. This is more complex than
    /// the basic Union-Find case.
    pub fn find(&mut self, x: usize) -> Result<usize, String> {
        if x >= self.parent.len() {
            return Err(format!("Index {} out of bounds", x));
        }

        if self.parent[x] != x {
            let old_parent = self.parent[x];
            self.parent[x] = self.find(old_parent)?;
            // Update distance to root by adding parent's distance to root
            self.dist[x] += self.dist[old_parent];
        }
        Ok(self.parent[x])
    }

    /// Unions two elements with a specified weight/distance
    ///
    /// The weight represents the distance from x to y.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut wuf = WeightedUnionFind::new(5);
    /// wuf.weighted_union(0, 1, 10)?; // Distance from 0 to 1 is 10
    /// wuf.weighted_union(1, 2, 5)?;  // Distance from 1 to 2 is 5
    /// 
    /// // Now distance from 0 to 2 should be 15
    /// assert_eq!(wuf.distance(0, 2)?, Some(15));
    /// ```
    pub fn weighted_union(&mut self, x: usize, y: usize, weight: i64) -> Result<bool, String> {
        let root_x = self.find(x)?;
        let root_y = self.find(y)?;

        if root_x == root_y {
            return Ok(false); // Already connected
        }

        // Calculate the weight needed between roots
        let weight_needed = weight + self.dist[x] - self.dist[y];

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
            self.dist[root_x] = -weight_needed;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
            self.dist[root_y] = weight_needed;
        } else {
            self.parent[root_y] = root_x;
            self.dist[root_y] = weight_needed;
            self.rank[root_x] += 1;
        }

        self.count -= 1;
        Ok(true)
    }

    /// Returns the distance between two elements, if they are connected
    pub fn distance(&mut self, x: usize, y: usize) -> Result<Option<i64>, String> {
        let root_x = self.find(x)?;
        let root_y = self.find(y)?;

        if root_x != root_y {
            Ok(None) // Not connected
        } else {
            Ok(Some(self.dist[y] - self.dist[x]))
        }
    }
}

/// Union-Find with undo support
///
/// # Requirements Satisfied: REQ-9
///
/// This variant maintains a history of operations that can be undone.
pub struct UndoUnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
    count: usize,
    /// History of operations for undo functionality
    history: Vec<UndoOperation>,
    operation_count: u64,
}

#[derive(Debug, Clone)]
struct UndoOperation {
    op_type: OperationType,
    x_parent: usize,
    y_parent: usize,
    x_rank: usize,
    y_rank: usize,
    x_size: usize,
    y_size: usize,
    old_count: usize,
}

#[derive(Debug, Clone)]
enum OperationType {
    Union { x: usize, y: usize },
}

impl UndoUnionFind {
    /// Creates a new Union-Find structure with undo support
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
            count: n,
            history: Vec::new(),
            operation_count: 0,
        }
    }

    /// Find with path compression (read-only version for undo compatibility)
    pub fn find(&self, mut x: usize) -> Result<usize, String> {
        if x >= self.parent.len() {
            return Err(format!("Index {} out of bounds", x));
        }

        while x != self.parent[x] {
            x = self.parent[x];
        }
        Ok(x)
    }

    /// Union operation with history tracking
    pub fn union(&mut self, x: usize, y: usize) -> Result<bool, String> {
        let root_x = self.find(x)?;
        let root_y = self.find(y)?;

        // Save state before operation
        let old_state = UndoOperation {
            op_type: OperationType::Union { 
                x: root_x, 
                y: root_y
            },
            x_parent: self.parent[root_x],
            y_parent: self.parent[root_y],
            x_rank: self.rank[root_x],
            y_rank: self.rank[root_y],
            x_size: self.size[root_x],
            y_size: self.size[root_y],
            old_count: self.count,
        };

        if root_x == root_y {
            self.history.push(old_state);
            self.operation_count += 1;
            return Ok(false);
        }

        // Perform union by rank
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
            self.rank[root_x] += 1;
        }

        self.count -= 1;
        self.history.push(old_state);
        self.operation_count += 1;
        Ok(true)
    }

    /// Undo the last operation
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - If an operation was undone
    /// * `Ok(false)` - If there are no operations to undo
    ///
    /// # Example
    ///
    /// ```
    /// use mission10::UndoUnionFind;
    ///
    /// # fn main() -> Result<(), String> {
    /// let mut uf = UndoUnionFind::new(5);
    /// uf.union(0, 1)?;
    /// assert_eq!(uf.count(), 4);
    ///
    /// assert!(uf.undo()?); // Undo the union
    /// assert_eq!(uf.count(), 5); // Back to original state
    /// # Ok(())
    /// # }
    /// ```
    pub fn undo(&mut self) -> Result<bool, String> {
        if let Some(op) = self.history.pop() {
            match op.op_type {
                OperationType::Union { x, y } => {
                    // Restore previous state
                    self.parent[x] = op.x_parent;
                    self.parent[y] = op.y_parent;
                    self.rank[x] = op.x_rank;
                    self.rank[y] = op.y_rank;
                    self.size[x] = op.x_size;
                    self.size[y] = op.y_size;
                    self.count = op.old_count;
                }
            }
            self.operation_count -= 1;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Returns the number of operations performed
    pub fn operation_count(&self) -> u64 {
        self.operation_count
    }

    /// Returns the number of disjoint sets
    pub fn count(&self) -> usize {
        self.count
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

    #[test]
    fn req2_additional_operations() {
        let mut uf = UnionFind::new(5);
        
        // Test len() and is_empty()
        assert_eq!(uf.len(), 5);
        assert!(!uf.is_empty());

        // Test clear()
        uf.union(0, 1).unwrap();
        uf.union(2, 3).unwrap();
        assert_eq!(uf.count(), 3);
        
        uf.clear();
        assert_eq!(uf.count(), 5);
        assert!(!uf.connected(0, 1).unwrap());

        // Test reset()
        uf.reset(10);
        assert_eq!(uf.len(), 10);
        assert_eq!(uf.count(), 10);
    }

    #[test]
    fn test_components_iterator() {
        let mut uf = UnionFind::new(6);
        uf.union(0, 1).unwrap();
        uf.union(2, 3).unwrap();
        // Components: {0,1}, {2,3}, {4}, {5}

        let components: Vec<Vec<usize>> = uf.components().collect();
        assert_eq!(components.len(), 4);
        
        // Check that each component has correct size
        let mut sizes: Vec<usize> = components.iter().map(|c| c.len()).collect();
        sizes.sort();
        assert_eq!(sizes, vec![1, 1, 2, 2]);
    }

    #[test]
    fn test_members_iterator() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1).unwrap();
        uf.union(1, 2).unwrap();

        let members: Vec<usize> = uf.members(0).unwrap().collect();
        assert_eq!(members.len(), 3);

        // Check that all members are in the same set
        for &member in &members {
            assert!(uf.connected(0, member).unwrap());
        }
    }

    #[test]
    fn test_undo_union_find() {
        let mut uf = UndoUnionFind::new(5);
        
        // Initial state
        assert_eq!(uf.count(), 5);
        assert_eq!(uf.operation_count(), 0);

        // Perform some operations
        assert!(uf.union(0, 1).unwrap());
        assert_eq!(uf.count(), 4);
        assert_eq!(uf.operation_count(), 1);

        assert!(uf.union(2, 3).unwrap());
        assert_eq!(uf.count(), 3);
        assert_eq!(uf.operation_count(), 2);

        // Undo last operation
        assert!(uf.undo().unwrap());
        assert_eq!(uf.count(), 4);
        assert_eq!(uf.operation_count(), 1);

        // Undo first operation
        assert!(uf.undo().unwrap());
        assert_eq!(uf.count(), 5);
        assert_eq!(uf.operation_count(), 0);

        // Can't undo anymore
        assert!(!uf.undo().unwrap());
    }

    #[cfg(feature = "weighted")]
    #[test]
    fn test_weighted_union_find() {
        let mut wuf = WeightedUnionFind::new(4);
        
        // Create weighted connections
        wuf.weighted_union(0, 1, 10).unwrap();
        wuf.weighted_union(1, 2, 5).unwrap();
        
        // Test distances
        assert_eq!(wuf.distance(0, 1).unwrap(), Some(10));
        assert_eq!(wuf.distance(0, 2).unwrap(), Some(15)); // 10 + 5
        assert_eq!(wuf.distance(1, 2).unwrap(), Some(5));
        assert_eq!(wuf.distance(0, 3).unwrap(), None); // Not connected
    }
}
