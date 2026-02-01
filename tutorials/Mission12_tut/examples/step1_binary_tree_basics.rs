//! # Step 1: Binary Tree Fundamentals
//!
//! This tutorial covers the foundational concepts of binary trees in Rust.
//!
//! ## Tree Terminology
//!
//! ```text
//!           1          <- Root (no parent)
//!          / \
//!         2   3        <- Children of node 1
//!        / \   \
//!       4   5   6      <- Leaves (no children)
//!      /
//!     7                <- Leaf, depth=3
//! ```
//!
//! - **Root**: The topmost node (no parent)
//! - **Parent**: A node with children below it
//! - **Child**: A node directly below another node
//! - **Leaf**: A node with no children
//! - **Height**: Longest path from node to any leaf (height of tree = height of root)
//! - **Depth**: Distance from root to a node (root has depth 0)
//! - **Subtree**: A node and all its descendants
//!
//! ## Key Rust Pattern: `Option<Box<Node<T>>>`
//!
//! Binary trees are recursive structures. In Rust, we use:
//! - `Box<T>` for heap allocation (required for recursive types)
//! - `Option<T>` to represent "no child" (None) vs "has child" (Some)
//!
//! This pattern appears everywhere in tree implementations!

// =============================================================================
// Node Definition
// =============================================================================

/// A node in a binary tree.
///
/// The recursive structure `Option<Box<Node<T>>>` is idiomatic Rust:
/// - `None` = no child (leaf on that side)
/// - `Some(Box<Node<T>>)` = has a child node (heap-allocated)
#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    /// Create a new leaf node (no children).
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    /// Create a node with specified children.
    fn with_children(value: T, left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Self {
        Node { value, left, right }
    }

    /// Check if this node is a leaf (no children).
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

// =============================================================================
// BinaryTree Wrapper
// =============================================================================

/// A binary tree structure.
///
/// Wrapping the root in a struct allows for tree-level operations
/// and handles the empty tree case gracefully.
#[derive(Debug)]
struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> BinaryTree<T> {
    /// Create an empty tree.
    fn new() -> Self {
        BinaryTree { root: None }
    }

    /// Create a tree with a root node.
    fn with_root(root: Node<T>) -> Self {
        BinaryTree {
            root: Some(Box::new(root)),
        }
    }

    /// Check if tree is empty.
    fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

// =============================================================================
// Exercise 1: Calculate Tree Height
// =============================================================================

impl<T> BinaryTree<T> {
    /// Calculate the height of the tree.
    ///
    /// Height = longest path from root to any leaf.
    /// - Empty tree has height 0 (some definitions use -1)
    /// - Single node tree has height 1
    ///
    /// ## Algorithm (Recursive)
    /// ```text
    /// height(node) = 0                           if node is None
    /// height(node) = 1 + max(height(left), height(right))  otherwise
    /// ```
    ///
    /// ## Complexity
    /// - Time: O(n) - visit every node once
    /// - Space: O(h) - recursion stack depth = height
    fn height(&self) -> usize {
        Self::height_recursive(self.root.as_deref())
    }

    fn height_recursive(node: Option<&Node<T>>) -> usize {
        match node {
            None => 0,
            Some(n) => {
                let left_height = Self::height_recursive(n.left.as_deref());
                let right_height = Self::height_recursive(n.right.as_deref());
                1 + left_height.max(right_height)
            }
        }
    }
}

// =============================================================================
// Exercise 2: Count Total Nodes
// =============================================================================

impl<T> BinaryTree<T> {
    /// Count total number of nodes in the tree.
    ///
    /// ## Algorithm (Recursive)
    /// ```text
    /// count(node) = 0                                    if node is None
    /// count(node) = 1 + count(left) + count(right)       otherwise
    /// ```
    ///
    /// ## Complexity
    /// - Time: O(n) - visit every node once
    /// - Space: O(h) - recursion stack depth
    fn count(&self) -> usize {
        Self::count_recursive(self.root.as_deref())
    }

    fn count_recursive(node: Option<&Node<T>>) -> usize {
        match node {
            None => 0,
            Some(n) => {
                1 + Self::count_recursive(n.left.as_deref())
                    + Self::count_recursive(n.right.as_deref())
            }
        }
    }

    /// Count only leaf nodes.
    fn count_leaves(&self) -> usize {
        Self::count_leaves_recursive(self.root.as_deref())
    }

    fn count_leaves_recursive(node: Option<&Node<T>>) -> usize {
        match node {
            None => 0,
            Some(n) if n.is_leaf() => 1,
            Some(n) => {
                Self::count_leaves_recursive(n.left.as_deref())
                    + Self::count_leaves_recursive(n.right.as_deref())
            }
        }
    }
}

// =============================================================================
// Exercise 3: Find Maximum Value
// =============================================================================

impl<T: Ord + Clone> BinaryTree<T> {
    /// Find the maximum value in the tree.
    ///
    /// Note: This is NOT a Binary Search Tree optimization.
    /// We must check every node (brute force).
    ///
    /// ## Complexity
    /// - Time: O(n) - must visit all nodes
    /// - Space: O(h) - recursion depth
    fn max(&self) -> Option<T> {
        Self::max_recursive(self.root.as_deref())
    }

    fn max_recursive(node: Option<&Node<T>>) -> Option<T> {
        match node {
            None => None,
            Some(n) => {
                let left_max = Self::max_recursive(n.left.as_deref());
                let right_max = Self::max_recursive(n.right.as_deref());

                // Compare current value with both subtree maximums
                let mut result = n.value.clone();

                if let Some(left) = left_max {
                    if left > result {
                        result = left;
                    }
                }

                if let Some(right) = right_max {
                    if right > result {
                        result = right;
                    }
                }

                Some(result)
            }
        }
    }

    /// Find minimum value (similar pattern).
    fn min(&self) -> Option<T> {
        Self::min_recursive(self.root.as_deref())
    }

    fn min_recursive(node: Option<&Node<T>>) -> Option<T> {
        match node {
            None => None,
            Some(n) => {
                let left_min = Self::min_recursive(n.left.as_deref());
                let right_min = Self::min_recursive(n.right.as_deref());

                let mut result = n.value.clone();

                if let Some(left) = left_min {
                    if left < result {
                        result = left;
                    }
                }

                if let Some(right) = right_min {
                    if right < result {
                        result = right;
                    }
                }

                Some(result)
            }
        }
    }
}

// =============================================================================
// Exercise 4: Check if Tree is Symmetric
// =============================================================================

impl<T: PartialEq> BinaryTree<T> {
    /// Check if the tree is symmetric (mirror image of itself).
    ///
    /// ```text
    /// Symmetric:           Not Symmetric:
    ///       1                    1
    ///      / \                  / \
    ///     2   2                2   2
    ///    / \ / \                \   \
    ///   3  4 4  3                3   3
    /// ```
    ///
    /// ## Algorithm
    /// Two subtrees are mirrors if:
    /// 1. Both are None, OR
    /// 2. Root values are equal AND left mirrors right's right AND right mirrors left's left
    ///
    /// ## Complexity
    /// - Time: O(n) - visit each node once
    /// - Space: O(h) - recursion depth
    fn is_symmetric(&self) -> bool {
        match &self.root {
            None => true, // Empty tree is symmetric
            Some(root) => Self::is_mirror(root.left.as_deref(), root.right.as_deref()),
        }
    }

    fn is_mirror(left: Option<&Node<T>>, right: Option<&Node<T>>) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(l), Some(r)) => {
                l.value == r.value
                    && Self::is_mirror(l.left.as_deref(), r.right.as_deref())
                    && Self::is_mirror(l.right.as_deref(), r.left.as_deref())
            }
            _ => false, // One is None, other is Some
        }
    }
}

// =============================================================================
// Tree Building Helpers
// =============================================================================

/// Build a tree from a level-order array representation.
///
/// Uses `Option<T>` where `None` represents missing nodes.
///
/// Example: `[Some(1), Some(2), Some(3), None, Some(4)]`
/// ```text
///       1
///      / \
///     2   3
///      \
///       4
/// ```
fn build_from_level_order<T: Clone>(values: &[Option<T>]) -> BinaryTree<T> {
    if values.is_empty() || values[0].is_none() {
        return BinaryTree::new();
    }

    // Create all nodes first
    let mut nodes: Vec<Option<Box<Node<T>>>> = values
        .iter()
        .map(|v| v.as_ref().map(|val| Box::new(Node::new(val.clone()))))
        .collect();

    // Link nodes together (iterate from back to avoid borrow issues)
    // For node at index i: left child at 2i+1, right child at 2i+2
    // Process in reverse so children are ready before parent
    for i in (0..nodes.len()).rev() {
        let left_idx = 2 * i + 1;
        let right_idx = 2 * i + 2;

        // Take children first (they're further in the vec, already processed)
        let left_child = if left_idx < nodes.len() {
            nodes[left_idx].take()
        } else {
            None
        };
        let right_child = if right_idx < nodes.len() {
            nodes[right_idx].take()
        } else {
            None
        };

        // Now assign to parent
        if let Some(ref mut node) = nodes[i] {
            node.left = left_child;
            node.right = right_child;
        }
    }

    BinaryTree {
        root: nodes.into_iter().next().flatten(),
    }
}

// =============================================================================
// Manual Tree Construction Demo
// =============================================================================

/// Build a sample tree manually to understand the structure.
///
/// Creates:
/// ```text
///        5
///       / \
///      3   8
///     / \   \
///    1   4   9
/// ```
fn build_sample_tree() -> BinaryTree<i32> {
    // Build from leaves up (bottom-up construction)
    let node1 = Node::new(1);
    let node4 = Node::new(4);
    let node9 = Node::new(9);

    let node3 = Node::with_children(3, Some(Box::new(node1)), Some(Box::new(node4)));
    let node8 = Node::with_children(8, None, Some(Box::new(node9)));

    let root = Node::with_children(5, Some(Box::new(node3)), Some(Box::new(node8)));

    BinaryTree::with_root(root)
}

/// Build a symmetric tree for testing.
///
/// Creates:
/// ```text
///        1
///       / \
///      2   2
///     / \ / \
///    3  4 4  3
/// ```
fn build_symmetric_tree() -> BinaryTree<i32> {
    let left_left = Node::new(3);
    let left_right = Node::new(4);
    let right_left = Node::new(4);
    let right_right = Node::new(3);

    let left = Node::with_children(2, Some(Box::new(left_left)), Some(Box::new(left_right)));
    let right = Node::with_children(2, Some(Box::new(right_left)), Some(Box::new(right_right)));

    let root = Node::with_children(1, Some(Box::new(left)), Some(Box::new(right)));

    BinaryTree::with_root(root)
}

// =============================================================================
// Main - Demonstration
// =============================================================================

fn main() {
    println!("=== Step 1: Binary Tree Fundamentals ===\n");

    // Demo 1: Manual tree construction
    println!("--- Demo 1: Manual Tree Construction ---");
    let tree = build_sample_tree();
    println!("Sample tree structure:");
    println!("       5       ");
    println!("      / \\      ");
    println!("     3   8     ");
    println!("    / \\   \\    ");
    println!("   1   4   9   ");
    println!();

    // Demo 2: Tree height
    println!("--- Demo 2: Tree Height ---");
    println!("Height of tree: {}", tree.height());
    println!("(Longest path: 5 -> 3 -> 1, length = 3)");
    println!();

    // Demo 3: Node count
    println!("--- Demo 3: Node Count ---");
    println!("Total nodes: {}", tree.count());
    println!("Leaf nodes: {}", tree.count_leaves());
    println!();

    // Demo 4: Find max/min
    println!("--- Demo 4: Find Max/Min ---");
    println!("Maximum value: {:?}", tree.max());
    println!("Minimum value: {:?}", tree.min());
    println!();

    // Demo 5: Symmetry check
    println!("--- Demo 5: Symmetry Check ---");
    println!("Sample tree symmetric? {}", tree.is_symmetric());

    let symmetric = build_symmetric_tree();
    println!("\nSymmetric tree structure:");
    println!("       1       ");
    println!("      / \\      ");
    println!("     2   2     ");
    println!("    / \\ / \\    ");
    println!("   3  4 4  3   ");
    println!("Symmetric tree symmetric? {}", symmetric.is_symmetric());
    println!();

    // Demo 6: Empty tree edge cases
    println!("--- Demo 6: Empty Tree Edge Cases ---");
    let empty: BinaryTree<i32> = BinaryTree::new();
    println!("Empty tree is_empty? {}", empty.is_empty());
    println!("Empty tree height: {}", empty.height());
    println!("Empty tree count: {}", empty.count());
    println!("Empty tree max: {:?}", empty.max());
    println!("Empty tree symmetric? {}", empty.is_symmetric());
    println!();

    // Demo 7: Single node tree
    println!("--- Demo 7: Single Node Tree ---");
    let single = BinaryTree::with_root(Node::new(42));
    println!("Single node height: {}", single.height());
    println!("Single node count: {}", single.count());
    println!("Single node leaves: {}", single.count_leaves());
    println!("Single node symmetric? {}", single.is_symmetric());
    println!();

    // Demo 8: Build from level-order array
    println!("--- Demo 8: Build from Level-Order Array ---");
    let values = vec![Some(1), Some(2), Some(3), None, Some(4), Some(5), None];
    println!("Input: [Some(1), Some(2), Some(3), None, Some(4), Some(5), None]");
    let from_array = build_from_level_order(&values);
    println!("Tree structure:");
    println!("       1       ");
    println!("      / \\      ");
    println!("     2   3     ");
    println!("      \\ /      ");
    println!("      4 5      ");
    println!("Height: {}", from_array.height());
    println!("Count: {}", from_array.count());
    println!();

    println!("=== Step 1 Complete! ===");
    println!("\nKey Takeaways:");
    println!("1. Binary trees use `Option<Box<Node<T>>>` for recursive structure");
    println!("2. Recursive algorithms naturally match tree structure");
    println!("3. Height and count use O(n) time, O(h) space");
    println!("4. Symmetry check compares left subtree with mirror of right");
    println!("\nNext: Step 2 - Binary Search Tree with ordering property!");
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height_empty() {
        let tree: BinaryTree<i32> = BinaryTree::new();
        assert_eq!(tree.height(), 0);
    }

    #[test]
    fn test_height_single_node() {
        let tree = BinaryTree::with_root(Node::new(1));
        assert_eq!(tree.height(), 1);
    }

    #[test]
    fn test_height_sample_tree() {
        let tree = build_sample_tree();
        assert_eq!(tree.height(), 3);
    }

    #[test]
    fn test_count_empty() {
        let tree: BinaryTree<i32> = BinaryTree::new();
        assert_eq!(tree.count(), 0);
    }

    #[test]
    fn test_count_sample_tree() {
        let tree = build_sample_tree();
        assert_eq!(tree.count(), 6);
    }

    #[test]
    fn test_count_leaves() {
        let tree = build_sample_tree();
        assert_eq!(tree.count_leaves(), 3); // 1, 4, 9
    }

    #[test]
    fn test_max_sample_tree() {
        let tree = build_sample_tree();
        assert_eq!(tree.max(), Some(9));
    }

    #[test]
    fn test_min_sample_tree() {
        let tree = build_sample_tree();
        assert_eq!(tree.min(), Some(1));
    }

    #[test]
    fn test_max_empty() {
        let tree: BinaryTree<i32> = BinaryTree::new();
        assert_eq!(tree.max(), None);
    }

    #[test]
    fn test_symmetric_empty() {
        let tree: BinaryTree<i32> = BinaryTree::new();
        assert!(tree.is_symmetric());
    }

    #[test]
    fn test_symmetric_single() {
        let tree = BinaryTree::with_root(Node::new(1));
        assert!(tree.is_symmetric());
    }

    #[test]
    fn test_symmetric_true() {
        let tree = build_symmetric_tree();
        assert!(tree.is_symmetric());
    }

    #[test]
    fn test_symmetric_false() {
        let tree = build_sample_tree();
        assert!(!tree.is_symmetric());
    }

    #[test]
    fn test_is_leaf() {
        let leaf = Node::new(1);
        assert!(leaf.is_leaf());

        let not_leaf = Node::with_children(1, Some(Box::new(Node::new(2))), None);
        assert!(!not_leaf.is_leaf());
    }

    #[test]
    fn test_build_from_level_order() {
        let values = vec![Some(1), Some(2), Some(3)];
        let tree = build_from_level_order(&values);
        assert_eq!(tree.count(), 3);
        assert_eq!(tree.height(), 2);
    }

    #[test]
    fn test_build_from_level_order_empty() {
        let values: Vec<Option<i32>> = vec![];
        let tree = build_from_level_order(&values);
        assert!(tree.is_empty());
    }
}
