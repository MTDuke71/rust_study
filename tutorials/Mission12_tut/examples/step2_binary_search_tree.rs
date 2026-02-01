//! # Step 2: Binary Search Tree (BST)
//!
//! This tutorial covers Binary Search Trees, a fundamental ordered data structure.
//!
//! ## BST Ordering Property
//!
//! For every node in a BST:
//! - **All values in left subtree < node's value**
//! - **All values in right subtree > node's value**
//!
//! ```text
//!         8
//!        / \
//!       3   10         3 < 8 < 10
//!      / \    \
//!     1   6    14      1 < 3 < 6 and 10 < 14
//!        / \   /
//!       4   7 13       4 < 6 < 7 and 13 < 14
//! ```
//!
//! ## Why BSTs?
//!
//! - **Search**: O(log n) average, O(n) worst case
//! - **Insert**: O(log n) average, O(n) worst case
//! - **Delete**: O(log n) average, O(n) worst case
//! - **In-order traversal produces sorted sequence**
//!
//! ## Worst Case: Degenerate Tree
//!
//! ```text
//! Inserting [1,2,3,4,5] produces a linked list:
//!     1
//!      \
//!       2
//!        \
//!         3    <- O(n) operations!
//!          \
//!           4
//!            \
//!             5
//! ```
//!
//! Balanced trees (AVL, Red-Black) solve this → Step 4!

// =============================================================================
// Node Definition
// =============================================================================

/// A node in a Binary Search Tree.
#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    /// Create a new leaf node.
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    /// Check if this node is a leaf.
    #[allow(dead_code)]
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

// =============================================================================
// Binary Search Tree
// =============================================================================

/// A Binary Search Tree that maintains the ordering property:
/// left < parent < right for all nodes.
#[derive(Debug, Clone)]
pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Ord> BinarySearchTree<T> {
    /// Create an empty BST.
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            size: 0,
        }
    }

    /// Check if the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Get the number of nodes in the tree.
    pub fn len(&self) -> usize {
        self.size
    }

    // =========================================================================
    // INSERT Operation
    // =========================================================================

    /// Insert a value into the BST.
    ///
    /// Maintains BST property by recursively finding the correct position:
    /// - If value < current node: insert in left subtree
    /// - If value > current node: insert in right subtree
    /// - If value == current node: replace (or could skip/return false)
    ///
    /// # Time Complexity
    /// - Average: O(log n) for balanced tree
    /// - Worst: O(n) for degenerate tree
    ///
    /// # Example
    /// ```
    /// let mut bst = BinarySearchTree::new();
    /// bst.insert(5);
    /// bst.insert(3);
    /// bst.insert(7);
    /// ```
    pub fn insert(&mut self, value: T) {
        if Self::insert_recursive(&mut self.root, value) {
            self.size += 1;
        }
    }

    /// Recursive helper for insert.
    /// Returns true if a new node was inserted, false if value already existed.
    fn insert_recursive(node: &mut Option<Box<Node<T>>>, value: T) -> bool {
        match node {
            None => {
                // Found the insertion point - create new leaf node
                *node = Some(Box::new(Node::new(value)));
                true
            }
            Some(current) => {
                // Compare and recurse into appropriate subtree
                if value < current.value {
                    Self::insert_recursive(&mut current.left, value)
                } else if value > current.value {
                    Self::insert_recursive(&mut current.right, value)
                } else {
                    // Value already exists - replace it (alternative: return false)
                    current.value = value;
                    false
                }
            }
        }
    }

    // =========================================================================
    // SEARCH Operations
    // =========================================================================

    /// Search for a value in the BST.
    ///
    /// Returns true if value exists, false otherwise.
    ///
    /// # Time Complexity
    /// - Average: O(log n)
    /// - Worst: O(n)
    ///
    /// # Example
    /// ```
    /// let mut bst = BinarySearchTree::new();
    /// bst.insert(5);
    /// assert!(bst.contains(&5));
    /// assert!(!bst.contains(&10));
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        Self::search_recursive(&self.root, value)
    }

    /// Recursive helper for search.
    fn search_recursive(node: &Option<Box<Node<T>>>, value: &T) -> bool {
        match node {
            None => false, // Not found
            Some(current) => {
                if value < &current.value {
                    Self::search_recursive(&current.left, value)
                } else if value > &current.value {
                    Self::search_recursive(&current.right, value)
                } else {
                    true // Found it!
                }
            }
        }
    }

    /// Find a reference to a value in the BST.
    ///
    /// Returns Some(&T) if found, None otherwise.
    pub fn find(&self, value: &T) -> Option<&T> {
        Self::find_recursive(&self.root, value)
    }

    /// Recursive helper for find.
    fn find_recursive<'a>(node: &'a Option<Box<Node<T>>>, value: &T) -> Option<&'a T> {
        match node {
            None => None,
            Some(current) => {
                if value < &current.value {
                    Self::find_recursive(&current.left, value)
                } else if value > &current.value {
                    Self::find_recursive(&current.right, value)
                } else {
                    Some(&current.value)
                }
            }
        }
    }

    // =========================================================================
    // MIN/MAX Operations
    // =========================================================================

    /// Find the minimum value in the tree.
    ///
    /// The minimum is always the leftmost node.
    ///
    /// # Time Complexity: O(h) where h is height
    pub fn min(&self) -> Option<&T> {
        Self::min_recursive(&self.root)
    }

    fn min_recursive(node: &Option<Box<Node<T>>>) -> Option<&T> {
        match node {
            None => None,
            Some(current) => {
                if current.left.is_none() {
                    Some(&current.value)
                } else {
                    Self::min_recursive(&current.left)
                }
            }
        }
    }

    /// Find the maximum value in the tree.
    ///
    /// The maximum is always the rightmost node.
    ///
    /// # Time Complexity: O(h) where h is height
    pub fn max(&self) -> Option<&T> {
        Self::max_recursive(&self.root)
    }

    fn max_recursive(node: &Option<Box<Node<T>>>) -> Option<&T> {
        match node {
            None => None,
            Some(current) => {
                if current.right.is_none() {
                    Some(&current.value)
                } else {
                    Self::max_recursive(&current.right)
                }
            }
        }
    }

    // =========================================================================
    // DELETE Operation (Advanced - 3 Cases)
    // =========================================================================

    /// Delete a value from the BST.
    ///
    /// Returns true if value was found and deleted, false otherwise.
    ///
    /// # Three Cases:
    /// 1. **Leaf node**: Simply remove it
    /// 2. **One child**: Replace node with its child
    /// 3. **Two children**: Replace with in-order successor (min of right subtree)
    ///
    /// # Time Complexity: O(h) where h is height
    pub fn delete(&mut self, value: &T) -> bool {
        let deleted = Self::delete_recursive(&mut self.root, value);
        if deleted {
            self.size -= 1;
        }
        deleted
    }

    /// Recursive helper for delete.
    fn delete_recursive(node: &mut Option<Box<Node<T>>>, value: &T) -> bool {
        match node {
            None => false, // Value not found
            Some(current) => {
                if value < &current.value {
                    // Search in left subtree
                    Self::delete_recursive(&mut current.left, value)
                } else if value > &current.value {
                    // Search in right subtree
                    Self::delete_recursive(&mut current.right, value)
                } else {
                    // Found the node to delete!
                    Self::delete_node(node);
                    true
                }
            }
        }
    }

    /// Delete the node, handling three cases.
    fn delete_node(node: &mut Option<Box<Node<T>>>) {
        let current = node.as_mut().unwrap();

        match (&mut current.left, &mut current.right) {
            // Case 1: Leaf node (no children)
            (None, None) => {
                *node = None;
            }

            // Case 2a: Only right child
            (None, Some(_)) => {
                *node = current.right.take();
            }

            // Case 2b: Only left child
            (Some(_), None) => {
                *node = current.left.take();
            }

            // Case 3: Two children
            // Replace with in-order successor (min of right subtree)
            (Some(_), Some(_)) => {
                // Find minimum value in right subtree
                let successor_value = Self::extract_min(&mut current.right);
                current.value = successor_value;
            }
        }
    }

    /// Extract the minimum value from a subtree (used in deletion).
    /// Removes and returns the minimum value.
    fn extract_min(node: &mut Option<Box<Node<T>>>) -> T {
        match node {
            None => panic!("extract_min called on None"),
            Some(current) => {
                if current.left.is_none() {
                    // This is the minimum - extract it
                    let min_node = node.take().unwrap();
                    *node = min_node.right;
                    min_node.value
                } else {
                    // Keep searching left
                    Self::extract_min(&mut current.left)
                }
            }
        }
    }

    // =========================================================================
    // VALIDATION
    // =========================================================================

    /// Validate that this tree satisfies the BST property.
    ///
    /// Useful for testing after complex operations.
    pub fn is_valid_bst(&self) -> bool {
        Self::is_valid_recursive(&self.root, None, None)
    }

    /// Recursive BST validation with bounds checking.
    fn is_valid_recursive(
        node: &Option<Box<Node<T>>>,
        min_bound: Option<&T>,
        max_bound: Option<&T>,
    ) -> bool {
        match node {
            None => true, // Empty tree is valid
            Some(current) => {
                // Check current node against bounds
                if let Some(min) = min_bound {
                    if current.value <= *min {
                        return false;
                    }
                }
                if let Some(max) = max_bound {
                    if current.value >= *max {
                        return false;
                    }
                }

                // Recursively validate subtrees with updated bounds
                Self::is_valid_recursive(&current.left, min_bound, Some(&current.value))
                    && Self::is_valid_recursive(&current.right, Some(&current.value), max_bound)
            }
        }
    }

    // =========================================================================
    // TRAVERSAL (In-Order gives sorted sequence)
    // =========================================================================

    /// In-order traversal: left -> root -> right
    /// For a BST, this produces values in sorted order!
    pub fn in_order(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::in_order_recursive(&self.root, &mut result);
        result
    }

    fn in_order_recursive<'a>(node: &'a Option<Box<Node<T>>>, result: &mut Vec<&'a T>) {
        if let Some(current) = node {
            Self::in_order_recursive(&current.left, result);
            result.push(&current.value);
            Self::in_order_recursive(&current.right, result);
        }
    }

    // =========================================================================
    // DEBUG VISUALIZATION
    // =========================================================================

    /// Print tree structure (rotated 90° - read from left to right as top to bottom)
    ///
    /// Example output:
    /// ```text
    ///         14
    ///             13
    ///     10
    /// 8
    ///         7
    ///     6
    ///         4
    ///     3
    ///         1
    /// ```
    pub fn print_tree(&self)
    where
        T: std::fmt::Display,
    {
        Self::print_tree_recursive(&self.root, 0, "Root");
    }

    fn print_tree_recursive(node: &Option<Box<Node<T>>>, depth: usize, prefix: &str)
    where
        T: std::fmt::Display,
    {
        match node {
            None => {
                println!("{}(empty)", "    ".repeat(depth));
            }
            Some(current) => {
                // Print right subtree first (appears at top when rotated)
                Self::print_tree_recursive(&current.right, depth + 1, "R");

                // Print current node
                println!("{}{}: {}", "    ".repeat(depth), prefix, current.value);

                // Print left subtree (appears at bottom when rotated)
                Self::print_tree_recursive(&current.left, depth + 1, "L");
            }
        }
    }
}

impl<T: Ord> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// EXAMPLES AND DEMONSTRATIONS
// =============================================================================

fn main() {
    println!("🌳 Step 2: Binary Search Tree\n");

    // Example 1: Basic Operations
    println!("=== Example 1: Insert and Search ===\n");
    let mut bst = BinarySearchTree::new();

    // Insert values
    let values = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
    println!("Inserting values: {:?}", values);
    for value in values {
        bst.insert(value);
    }

    println!("Tree size: {}", bst.len());
    println!("Is valid BST: {}", bst.is_valid_bst());

    // Visualize tree structure
    println!("\nTree structure (rotated 90° - root at left):");
    bst.print_tree();

    // Search operations
    println!("\nSearch operations:");
    println!("  Contains 6? {}", bst.contains(&6));
    println!("  Contains 5? {}", bst.contains(&5));
    println!("  Contains 13? {}", bst.contains(&13));

    // Min/Max
    println!("\nMin/Max:");
    println!("  Minimum: {:?}", bst.min());
    println!("  Maximum: {:?}", bst.max());

    // In-order traversal (sorted!)
    println!("\nIn-order traversal (sorted):");
    let sorted = bst.in_order();
    print!("  ");
    for value in &sorted {
        print!("{} ", value);
    }
    println!();

    // Example 2: Deletion Cases
    println!("\n=== Example 2: Deletion ===\n");

    // Delete leaf node
    println!("Delete 4 (leaf node):");
    bst.delete(&4);
    println!(
        "  Sorted: {:?}",
        bst.in_order().iter().map(|&&v| v).collect::<Vec<_>>()
    );
    println!("\n  Tree after deleting 4:");
    bst.print_tree();

    // Delete node with one child
    println!("\nDelete 14 (one child):");
    bst.delete(&14);
    println!(
        "  Sorted: {:?}",
        bst.in_order().iter().map(|&&v| v).collect::<Vec<_>>()
    );
    println!("\n  Tree after deleting 14:");
    bst.print_tree();

    // Delete node with two children
    println!("\nDelete 3 (two children):");
    bst.delete(&3);
    println!(
        "  Sorted: {:?}",
        bst.in_order().iter().map(|&&v| v).collect::<Vec<_>>()
    );
    println!("\n  Tree after deleting 3:");
    bst.print_tree();
    println!("\n  Tree is still valid BST: {}", bst.is_valid_bst());

    // Example 3: Degenerate Tree (Worst Case)
    println!("\n=== Example 3: Degenerate Tree ===\n");
    let mut degenerate = BinarySearchTree::new();

    println!("Inserting sorted sequence [1,2,3,4,5]:");
    for i in 1..=5 {
        degenerate.insert(i);
    }

    println!("  This creates a linked list (worst case!)");
    println!("  Size: {}", degenerate.len());
    println!("  Height would be O(n) instead of O(log n)");

    println!("\nDegenerate tree structure (all right children!):");
    degenerate.print_tree();

    println!("\n  Solution: Use balanced trees (AVL, Red-Black) → Step 4");

    // Example 4: Building from Array
    println!("\n=== Example 4: Build BST from Array ===\n");

    let array = vec![15, 10, 20, 8, 12, 16, 25];
    let mut bst = BinarySearchTree::new();

    println!("Building from: {:?}", array);
    for value in array {
        bst.insert(value);
    }
    bst.print_tree();
    println!(
        "Sorted sequence: {:?}",
        bst.in_order().iter().map(|&&v| v).collect::<Vec<_>>()
    );

    // Key Takeaways
    println!("\n=== Key Takeaways ===\n");
    println!("✓ BST Property: left < parent < right");
    println!("✓ Insert: O(log n) average, O(n) worst");
    println!("✓ Search: O(log n) average, O(n) worst");
    println!("✓ Delete: 3 cases (leaf, one child, two children)");
    println!("✓ In-order traversal produces sorted sequence");
    println!("✓ Degenerate tree = linked list (bad!)");
    println!("✓ Next: Tree traversals (Step 3)");
    println!("✓ Later: Balanced trees guarantee O(log n) (Step 4)");
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let bst: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(bst.is_empty());
        assert_eq!(bst.len(), 0);
        assert_eq!(bst.min(), None);
        assert_eq!(bst.max(), None);
    }

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(1);
        bst.insert(9);

        assert_eq!(bst.len(), 5);
        assert!(bst.contains(&5));
        assert!(bst.contains(&3));
        assert!(bst.contains(&7));
        assert!(bst.contains(&1));
        assert!(bst.contains(&9));
        assert!(!bst.contains(&4));
    }

    #[test]
    fn test_min_max() {
        let mut bst = BinarySearchTree::new();

        bst.insert(8);
        bst.insert(3);
        bst.insert(10);
        bst.insert(1);
        bst.insert(14);

        assert_eq!(bst.min(), Some(&1));
        assert_eq!(bst.max(), Some(&14));
    }

    #[test]
    fn test_in_order_traversal() {
        let mut bst = BinarySearchTree::new();

        let values = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        for value in values {
            bst.insert(value);
        }

        let sorted: Vec<i32> = bst.in_order().iter().map(|&&v| v).collect();
        assert_eq!(sorted, vec![1, 3, 4, 6, 7, 8, 10, 13, 14]);
    }

    #[test]
    fn test_delete_leaf() {
        let mut bst = BinarySearchTree::new();

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);

        assert!(bst.delete(&3));
        assert!(!bst.contains(&3));
        assert_eq!(bst.len(), 2);
        assert!(bst.is_valid_bst());
    }

    #[test]
    fn test_delete_one_child() {
        let mut bst = BinarySearchTree::new();

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(1);

        assert!(bst.delete(&3)); // Has left child (1)
        assert!(!bst.contains(&3));
        assert!(bst.contains(&1));
        assert!(bst.is_valid_bst());
    }

    #[test]
    fn test_delete_two_children() {
        let mut bst = BinarySearchTree::new();

        let values = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        for value in values {
            bst.insert(value);
        }

        assert!(bst.delete(&3)); // Has two children
        assert!(!bst.contains(&3));
        assert!(bst.is_valid_bst());

        let sorted: Vec<i32> = bst.in_order().iter().map(|&&v| v).collect();
        assert_eq!(sorted, vec![1, 4, 6, 7, 8, 10, 13, 14]);
    }

    #[test]
    fn test_delete_root() {
        let mut bst = BinarySearchTree::new();

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);

        assert!(bst.delete(&5));
        assert!(!bst.contains(&5));
        assert!(bst.is_valid_bst());
    }

    #[test]
    fn test_delete_nonexistent() {
        let mut bst = BinarySearchTree::new();

        bst.insert(5);
        bst.insert(3);

        assert!(!bst.delete(&10));
        assert_eq!(bst.len(), 2);
    }

    #[test]
    fn test_is_valid_bst() {
        let mut bst = BinarySearchTree::new();

        let values = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        for value in values {
            bst.insert(value);
        }

        assert!(bst.is_valid_bst());
    }

    #[test]
    fn test_duplicate_insert() {
        let mut bst = BinarySearchTree::new();

        bst.insert(5);
        bst.insert(5); // Duplicate

        assert_eq!(bst.len(), 1); // Should not increase size
        assert!(bst.contains(&5));
    }

    #[test]
    fn test_find() {
        let mut bst = BinarySearchTree::new();

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);

        assert_eq!(bst.find(&5), Some(&5));
        assert_eq!(bst.find(&3), Some(&3));
        assert_eq!(bst.find(&10), None);
    }
}
