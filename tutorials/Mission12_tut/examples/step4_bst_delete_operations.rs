 //! # Step 4: BST Delete & Advanced Operations
//!
//! Step 3 covered *reading* a tree (traversals). Step 4 covers *mutating*
//! it safely (delete) and answering structural questions about it
//! (successor/predecessor, LCA, path sums, equality) — all using the same
//! BST ordering property, so none of it needs parent pointers.
//!
//! ## The Three Delete Cases
//!
//! Using the Step 2/3 sample tree (inserted `8,3,10,1,6,14,4,7,13`):
//!
//! ```text
//!         8
//!        / \
//!       3   10
//!      / \    \
//!     1   6    14
//!        / \   /
//!       4   7 13
//! ```
//!
//! | Case              | Example  | Fix                                        |
//! |--------------------|----------|---------------------------------------------|
//! | 1: Leaf            | delete 1 | Just remove it — nothing points to it.       |
//! | 2: One child        | delete 14 | Splice the child up into the parent's slot.  |
//! | 3: Two children     | delete 3 | Can't just remove it — two subtrees would need two slots. Replace its *value* with its in-order successor's value, then delete that successor (which has at most one child, so it's Case 1 or 2). |
//!
//! **Why the in-order successor?** It's the smallest value greater than the
//! deleted node — the leftmost node of the right subtree. Promoting it into
//! the deleted node's spot keeps every value in its correct position
//! relative to everything else (left < it < right still holds everywhere).
//! The in-order *predecessor* (rightmost of the left subtree) works exactly
//! as well; this tutorial picks the successor by convention.
//!
//! ## Successor / Predecessor Without Parent Pointers
//!
//! "What's the next value after X, in sorted order?" has two cases:
//! - X has a right subtree → answer is that subtree's minimum.
//! - X has no right subtree → answer is the nearest ancestor for which X
//!   is in the *left* subtree. Finding that ancestor needs a walk up from
//!   X — normally requiring parent pointers.
//!
//! The trick: search from the **root** instead of from X, remembering the
//! last node where the search went left (that's exactly "the nearest
//! ancestor whose left subtree contains X"). One O(h) walk, no parent
//! pointers, and it even works if X isn't in the tree at all — it finds
//! where X *would* split the tree.
//!
//! ## LCA the BST Way
//!
//! For a general binary tree, LCA needs two root-to-node paths and a
//! comparison. A BST's ordering property shortcuts this: walk down from
//! the root; the first node that *splits* p and q (one on each side, or
//! equal to either) is the LCA — everything before that split had both
//! targets on the same side, so it can't be the lowest common ancestor.

use std::ops::Add;

// =============================================================================
// Node & BinarySearchTree (same shape as Steps 2-3)
// =============================================================================

/// A node in a Binary Search Tree.
#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

/// A Binary Search Tree, extended in this step with delete and structural
/// queries (successor, predecessor, LCA, paths, equality).
#[derive(Debug)]
pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Insert a value, maintaining the BST property (see Step 2 for details).
    pub fn insert(&mut self, value: T) {
        if Self::insert_recursive(&mut self.root, value) {
            self.size += 1;
        }
    }

    fn insert_recursive(node: &mut Option<Box<Node<T>>>, value: T) -> bool {
        match node {
            None => {
                *node = Some(Box::new(Node::new(value)));
                true
            }
            Some(current) => {
                if value < current.value {
                    Self::insert_recursive(&mut current.left, value)
                } else if value > current.value {
                    Self::insert_recursive(&mut current.right, value)
                } else {
                    current.value = value;
                    false
                }
            }
        }
    }

    /// In-order traversal (see Step 3), used here to sanity-check that
    /// delete keeps the BST property intact.
    pub fn in_order(&self) -> Vec<&T> {
        let mut result = Vec::with_capacity(self.size);
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
}

impl<T: Ord> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// DELETE (all three cases)
// =============================================================================

impl<T: Ord> BinarySearchTree<T> {
    /// Remove `value` from the tree, if present. Returns whether it was found.
    pub fn delete(&mut self, value: &T) -> bool {
        let (new_root, deleted) = Self::delete_recursive(self.root.take(), value);
        self.root = new_root;
        if deleted {
            self.size -= 1;
        }
        deleted
    }

    fn delete_recursive(node: Option<Box<Node<T>>>, value: &T) -> (Option<Box<Node<T>>>, bool) {
        let Some(mut current) = node else {
            return (None, false); // Value not found.
        };

        if *value < current.value {
            let (new_left, deleted) = Self::delete_recursive(current.left.take(), value);
            current.left = new_left;
            return (Some(current), deleted);
        }
        if *value > current.value {
            let (new_right, deleted) = Self::delete_recursive(current.right.take(), value);
            current.right = new_right;
            return (Some(current), deleted);
        }

        // *value == current.value: this is the node to delete.
        match (current.left.take(), current.right.take()) {
            // Case 1: leaf — nothing replaces it.
            (None, None) => (None, true),
            // Case 2: one child — the child takes this node's place.
            (Some(left), None) => (Some(left), true),
            (None, Some(right)) => (Some(right), true),
            // Case 3: two children — promote the in-order successor's value,
            // then remove that successor from the right subtree (it has no
            // left child, so removing it is always Case 1 or Case 2).
            (Some(left), Some(right)) => {
                let mut right_subtree = Some(right);
                let successor_value = Self::extract_min(&mut right_subtree);
                current.value = successor_value;
                current.left = Some(left);
                current.right = right_subtree;
                (Some(current), true)
            }
        }
    }

    /// Remove and return the minimum (leftmost) value from a subtree,
    /// splicing that node's right child (if any) into its place in-place.
    fn extract_min(subtree: &mut Option<Box<Node<T>>>) -> T {
        let node = subtree.as_mut().expect("extract_min called on an empty subtree");
        if node.left.is_some() {
            Self::extract_min(&mut node.left)
        } else {
            // No left child: this node *is* the minimum. Its right subtree
            // (which may or may not exist) replaces it.
            let minimum = subtree.take().expect("checked Some above");
            *subtree = minimum.right;
            minimum.value
        }
    }
}

// =============================================================================
// SUCCESSOR / PREDECESSOR
// =============================================================================

impl<T: Ord> BinarySearchTree<T> {
    /// The next larger value in sorted order (in-order successor of `value`).
    /// Works even if `value` isn't in the tree — it finds where `value`
    /// would split the sequence.
    pub fn successor(&self, value: &T) -> Option<&T> {
        let mut current = self.root.as_deref();
        let mut candidate: Option<&Node<T>> = None;

        while let Some(node) = current {
            if *value < node.value {
                candidate = Some(node); // Search went left: remember this split point.
                current = node.left.as_deref();
            } else if *value > node.value {
                current = node.right.as_deref();
            } else {
                // Found `value` itself: successor is the min of its right
                // subtree, if any — otherwise fall back to `candidate`.
                if let Some(right) = &node.right {
                    return Some(Self::min_value(right));
                }
                break;
            }
        }
        candidate.map(|n| &n.value)
    }

    /// The next smaller value in sorted order (in-order predecessor), the
    /// mirror image of [`successor`](Self::successor).
    pub fn predecessor(&self, value: &T) -> Option<&T> {
        let mut current = self.root.as_deref();
        let mut candidate: Option<&Node<T>> = None;

        while let Some(node) = current {
            if *value > node.value {
                candidate = Some(node); // Search went right: remember this split point.
                current = node.right.as_deref();
            } else if *value < node.value {
                current = node.left.as_deref();
            } else {
                if let Some(left) = &node.left {
                    return Some(Self::max_value(left));
                }
                break;
            }
        }
        candidate.map(|n| &n.value)
    }

    fn min_value(node: &Node<T>) -> &T {
        let mut current = node;
        while let Some(left) = &current.left {
            current = left;
        }
        &current.value
    }

    fn max_value(node: &Node<T>) -> &T {
        let mut current = node;
        while let Some(right) = &current.right {
            current = right;
        }
        &current.value
    }
}

// =============================================================================
// LOWEST COMMON ANCESTOR
// =============================================================================

impl<T: Ord> BinarySearchTree<T> {
    /// Lowest Common Ancestor of `p` and `q`, using the BST ordering
    /// property: the first node the search reaches that doesn't have both
    /// values strictly on the same side is the split point — and therefore
    /// the LCA. O(h), no parent pointers needed.
    pub fn lca(&self, p: &T, q: &T) -> Option<&T> {
        let mut current = self.root.as_deref();
        while let Some(node) = current {
            if *p < node.value && *q < node.value {
                current = node.left.as_deref();
            } else if *p > node.value && *q > node.value {
                current = node.right.as_deref();
            } else {
                return Some(&node.value);
            }
        }
        None
    }
}

// =============================================================================
// ROOT-TO-LEAF PATHS AND PATH SUMS
// =============================================================================

impl<T: Ord> BinarySearchTree<T> {
    /// Every root-to-leaf path, each as a `Vec<&T>` in top-to-bottom order.
    pub fn root_to_leaf_paths(&self) -> Vec<Vec<&T>> {
        let mut paths = Vec::new();
        let mut current_path = Vec::new();
        Self::collect_paths(&self.root, &mut current_path, &mut paths);
        paths
    }

    fn collect_paths<'a>(
        node: &'a Option<Box<Node<T>>>,
        current_path: &mut Vec<&'a T>,
        paths: &mut Vec<Vec<&'a T>>,
    ) {
        let Some(current) = node else { return };

        current_path.push(&current.value);
        if current.left.is_none() && current.right.is_none() {
            paths.push(current_path.clone()); // Leaf: record the path so far.
        } else {
            Self::collect_paths(&current.left, current_path, paths);
            Self::collect_paths(&current.right, current_path, paths);
        }
        current_path.pop(); // Backtrack before returning to the caller.
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Copy + Add<Output = T> + Default,
{
    /// The sum of values along each root-to-leaf path, one sum per leaf.
    pub fn leaf_path_sums(&self) -> Vec<T> {
        self.root_to_leaf_paths()
            .into_iter()
            .map(|path| path.into_iter().fold(T::default(), |acc, &v| acc + v))
            .collect()
    }

    /// All leaf path sums added together — "sum of all root-to-leaf paths".
    pub fn sum_of_all_paths(&self) -> T {
        self.leaf_path_sums()
            .into_iter()
            .fold(T::default(), |acc, v| acc + v)
    }

    /// The classic "Path Sum" question: does *any* root-to-leaf path sum
    /// to exactly `target`?
    pub fn has_path_sum(&self, target: T) -> bool {
        self.leaf_path_sums().into_iter().any(|sum| sum == target)
    }
}

// =============================================================================
// TREE EQUALITY
// =============================================================================

impl<T: PartialEq> BinarySearchTree<T> {
    /// Structural equality: same shape *and* same value at every position.
    /// (Two BSTs holding the same values but built via different insertion
    /// orders can have different shapes — this catches that difference.)
    pub fn is_identical(&self, other: &Self) -> bool {
        Self::nodes_identical(&self.root, &other.root)
    }

    fn nodes_identical(a: &Option<Box<Node<T>>>, b: &Option<Box<Node<T>>>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                a.value == b.value
                    && Self::nodes_identical(&a.left, &b.left)
                    && Self::nodes_identical(&a.right, &b.right)
            }
            _ => false,
        }
    }
}

// =============================================================================
// EXAMPLES AND DEMONSTRATIONS
// =============================================================================

fn sample_tree() -> BinarySearchTree<i32> {
    let mut bst = BinarySearchTree::new();
    for value in [8, 3, 10, 1, 6, 14, 4, 7, 13] {
        bst.insert(value);
    }
    bst
}

fn print_row<T: std::fmt::Debug>(label: &str, values: &[T]) {
    println!("  {label}: {values:?}");
}

fn main() {
    println!("🌳 Step 4: BST Delete & Advanced Operations\n");

    println!("Using the Step 2/3 sample tree (inserted 8,3,10,1,6,14,4,7,13):");
    println!("        8");
    println!("       / \\");
    println!("      3   10");
    println!("     / \\    \\");
    println!("    1   6    14");
    println!("       / \\   /");
    println!("      4   7 13\n");

    // --- Delete: Case 1 (leaf) ---
    println!("=== Delete Case 1: Leaf (delete 1) ===");
    let mut bst = sample_tree();
    print_row("Before", &bst.in_order());
    assert!(bst.delete(&1));
    print_row("After ", &bst.in_order());
    assert_eq!(bst.len(), 8);
    println!("  ✓ 1 removed, everything else still sorted\n");

    // --- Delete: Case 2 (one child) ---
    println!("=== Delete Case 2: One Child (delete 14, whose only child is 13) ===");
    let mut bst = sample_tree();
    print_row("Before", &bst.in_order());
    assert!(bst.delete(&14));
    print_row("After ", &bst.in_order());
    print_row("Pre-order after (13 spliced into 14's old slot)", &{
        let mut result = Vec::new();
        fn preorder<'a>(node: &'a Option<Box<Node<i32>>>, out: &mut Vec<&'a i32>) {
            if let Some(n) = node {
                out.push(&n.value);
                preorder(&n.left, out);
                preorder(&n.right, out);
            }
        }
        preorder(&bst.root, &mut result);
        result
    });
    println!("  ✓ 13 took 14's place directly (no successor search needed)\n");

    // --- Delete: Case 3 (two children) ---
    println!("=== Delete Case 3: Two Children (delete 3, whose children are 1 and 6) ===");
    let mut bst = sample_tree();
    print_row("Before", &bst.in_order());
    assert!(bst.delete(&3));
    print_row("After ", &bst.in_order());
    assert!(!bst.in_order().contains(&&3));
    println!("  ✓ 3 replaced by its in-order successor (4), tree stays sorted\n");

    // --- Delete: value not present ---
    println!("=== Delete: Value Not Present ===");
    let mut bst = sample_tree();
    assert!(!bst.delete(&99));
    assert_eq!(bst.len(), 9);
    println!("  ✓ delete(99) returns false, tree untouched\n");

    // --- Successor / Predecessor ---
    println!("=== Successor / Predecessor ===");
    let bst = sample_tree();
    println!("  successor(6)   = {:?}  (6 has a right subtree → its min)", bst.successor(&6));
    println!("  successor(7)   = {:?}  (7 has none → nearest ancestor via left)", bst.successor(&7));
    println!("  successor(14)  = {:?}  (14 is the maximum → no successor)", bst.successor(&14));
    println!("  predecessor(6) = {:?}  (6 has a left subtree → its max)", bst.predecessor(&6));
    println!("  predecessor(7) = {:?}  (7 has none → nearest ancestor via right)", bst.predecessor(&7));
    println!("  predecessor(1) = {:?}  (1 is the minimum → no predecessor)", bst.predecessor(&1));
    assert_eq!(bst.successor(&6), Some(&7));
    assert_eq!(bst.successor(&7), Some(&8));
    assert_eq!(bst.successor(&14), None);
    assert_eq!(bst.predecessor(&6), Some(&4));
    assert_eq!(bst.predecessor(&7), Some(&6));
    assert_eq!(bst.predecessor(&1), None);
    println!();

    // --- Lowest Common Ancestor ---
    println!("=== Lowest Common Ancestor ===");
    println!("  lca(1, 7)   = {:?}  (split at the root's left child)", bst.lca(&1, &7));
    println!("  lca(4, 7)   = {:?}  (siblings under 6)", bst.lca(&4, &7));
    println!("  lca(13, 14) = {:?}  (14 is 13's parent — an ancestor is its own LCA)", bst.lca(&13, &14));
    assert_eq!(bst.lca(&1, &7), Some(&3));
    assert_eq!(bst.lca(&4, &7), Some(&6));
    assert_eq!(bst.lca(&13, &14), Some(&14));
    println!();

    // --- Root-to-leaf paths and path sums ---
    println!("=== Root-to-Leaf Paths & Path Sums ===");
    for path in bst.root_to_leaf_paths() {
        println!("  Path: {path:?}");
    }
    print_row("Leaf path sums   ", &bst.leaf_path_sums());
    println!("  Sum of all paths : {}", bst.sum_of_all_paths());
    println!("  has_path_sum(21) : {}  (8+3+6+4)", bst.has_path_sum(21));
    println!("  has_path_sum(100): {}", bst.has_path_sum(100));
    assert_eq!(bst.leaf_path_sums(), vec![12, 21, 24, 45]);
    assert_eq!(bst.sum_of_all_paths(), 102);
    assert!(bst.has_path_sum(21));
    assert!(!bst.has_path_sum(100));
    println!();

    // --- Tree equality ---
    println!("=== Tree Equality ===");
    let same_shape = sample_tree();
    let mut different_values = sample_tree();
    different_values.delete(&13);
    different_values.insert(99);
    println!("  identical to itself-shape twin : {}", bst.is_identical(&same_shape));
    println!("  identical after swapping a leaf: {}", bst.is_identical(&different_values));
    assert!(bst.is_identical(&same_shape));
    assert!(!bst.is_identical(&different_values));
    println!();

    // Key Takeaways
    println!("=== Key Takeaways ===\n");
    println!("✓ Delete has 3 cases: leaf (drop), one child (splice up), two children (promote successor)");
    println!("✓ The in-order successor of a two-children node always has ≤1 child — deleting it recurses into an easier case");
    println!("✓ Successor/predecessor need no parent pointers: search from the root, remember the last turn");
    println!("✓ BST LCA is just 'walk down until p and q split' — O(h), no path storage needed");
    println!("✓ Root-to-leaf path problems are backtracking: push on the way down, pop on the way back up");
    println!("✓ Structural equality must compare shape AND values — same values, different insertion order can differ in shape");
    println!("✓ Next: Balanced trees — AVL rotations (Step 5)");
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn values<'a>(refs: Vec<&'a i32>) -> Vec<i32> {
        refs.into_iter().copied().collect()
    }

    // --- Delete ---

    #[test]
    fn test_delete_leaf() {
        let mut bst = sample_tree();
        assert!(bst.delete(&1));
        assert_eq!(bst.len(), 8);
        assert_eq!(
            values(bst.in_order()),
            vec![3, 4, 6, 7, 8, 10, 13, 14]
        );
    }

    #[test]
    fn test_delete_one_child() {
        let mut bst = sample_tree();
        assert!(bst.delete(&14)); // 14's only child is 13.
        assert_eq!(bst.len(), 8);
        assert_eq!(
            values(bst.in_order()),
            vec![1, 3, 4, 6, 7, 8, 10, 13]
        );
        // 13 should have taken 14's place directly under 10.
        let preorder = bst.in_order(); // sorted check is enough to confirm shape validity
        assert!(!preorder.contains(&&14));
    }

    #[test]
    fn test_delete_two_children() {
        let mut bst = sample_tree();
        assert!(bst.delete(&3)); // 3's children are 1 and 6.
        assert_eq!(bst.len(), 8);
        assert_eq!(
            values(bst.in_order()),
            vec![1, 4, 6, 7, 8, 10, 13, 14]
        );
    }

    #[test]
    fn test_delete_root_with_two_children() {
        let mut bst = sample_tree();
        assert!(bst.delete(&8)); // Root has two children.
        assert_eq!(bst.len(), 8);
        assert_eq!(
            values(bst.in_order()),
            vec![1, 3, 4, 6, 7, 10, 13, 14]
        );
    }

    #[test]
    fn test_delete_nonexistent_value() {
        let mut bst = sample_tree();
        assert!(!bst.delete(&99));
        assert_eq!(bst.len(), 9);
    }

    #[test]
    fn test_delete_all_values_leaves_empty_tree() {
        let mut bst = sample_tree();
        for value in [8, 3, 10, 1, 6, 14, 4, 7, 13] {
            assert!(bst.delete(&value));
        }
        assert!(bst.is_empty());
        assert_eq!(bst.len(), 0);
    }

    #[test]
    fn test_delete_from_empty_tree() {
        let mut bst: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(!bst.delete(&5));
    }

    // --- Successor / predecessor ---

    #[test]
    fn test_successor_with_right_subtree() {
        let bst = sample_tree();
        assert_eq!(bst.successor(&6), Some(&7));
        assert_eq!(bst.successor(&8), Some(&10));
    }

    #[test]
    fn test_successor_without_right_subtree() {
        let bst = sample_tree();
        assert_eq!(bst.successor(&7), Some(&8));
        assert_eq!(bst.successor(&4), Some(&6));
    }

    #[test]
    fn test_successor_of_maximum_is_none() {
        let bst = sample_tree();
        assert_eq!(bst.successor(&14), None);
    }

    #[test]
    fn test_predecessor_with_left_subtree() {
        let bst = sample_tree();
        assert_eq!(bst.predecessor(&6), Some(&4));
        assert_eq!(bst.predecessor(&10), Some(&8));
    }

    #[test]
    fn test_predecessor_without_left_subtree() {
        let bst = sample_tree();
        assert_eq!(bst.predecessor(&7), Some(&6));
    }

    #[test]
    fn test_predecessor_of_minimum_is_none() {
        let bst = sample_tree();
        assert_eq!(bst.predecessor(&1), None);
    }

    #[test]
    fn test_successor_predecessor_of_value_not_in_tree() {
        let bst = sample_tree();
        // 5 is not in the tree; it would sit between 4 and 6.
        assert_eq!(bst.successor(&5), Some(&6));
        assert_eq!(bst.predecessor(&5), Some(&4));
    }

    // --- LCA ---

    #[test]
    fn test_lca_split_at_ancestor() {
        let bst = sample_tree();
        assert_eq!(bst.lca(&1, &7), Some(&3));
        assert_eq!(bst.lca(&4, &7), Some(&6));
    }

    #[test]
    fn test_lca_one_node_is_ancestor_of_other() {
        let bst = sample_tree();
        assert_eq!(bst.lca(&13, &14), Some(&14));
    }

    #[test]
    fn test_lca_of_root_with_anything_is_root() {
        let bst = sample_tree();
        assert_eq!(bst.lca(&8, &13), Some(&8));
    }

    // --- Paths and path sums ---

    #[test]
    fn test_root_to_leaf_paths() {
        let bst = sample_tree();
        let paths: Vec<Vec<i32>> = bst.root_to_leaf_paths().into_iter().map(values).collect();
        assert_eq!(
            paths,
            vec![
                vec![8, 3, 1],
                vec![8, 3, 6, 4],
                vec![8, 3, 6, 7],
                vec![8, 10, 14, 13],
            ]
        );
    }

    #[test]
    fn test_leaf_path_sums() {
        let bst = sample_tree();
        assert_eq!(bst.leaf_path_sums(), vec![12, 21, 24, 45]);
    }

    #[test]
    fn test_sum_of_all_paths() {
        let bst = sample_tree();
        assert_eq!(bst.sum_of_all_paths(), 102);
    }

    #[test]
    fn test_has_path_sum() {
        let bst = sample_tree();
        assert!(bst.has_path_sum(21));
        assert!(bst.has_path_sum(45));
        assert!(!bst.has_path_sum(100));
    }

    #[test]
    fn test_single_node_tree_paths() {
        let mut bst = BinarySearchTree::new();
        bst.insert(42);
        assert_eq!(
            bst.root_to_leaf_paths().into_iter().map(values).collect::<Vec<_>>(),
            vec![vec![42]]
        );
        assert_eq!(bst.leaf_path_sums(), vec![42]);
    }

    // --- Equality ---

    #[test]
    fn test_is_identical_same_shape_and_values() {
        let a = sample_tree();
        let b = sample_tree();
        assert!(a.is_identical(&b));
    }

    #[test]
    fn test_is_identical_different_values() {
        let a = sample_tree();
        let mut b = sample_tree();
        b.delete(&13);
        b.insert(99);
        assert!(!a.is_identical(&b));
    }

    #[test]
    fn test_is_identical_different_shape_same_values_via_different_insert_order() {
        // Inserting in a different order can produce a different shape
        // even with an overlapping value set.
        let mut a = BinarySearchTree::new();
        for value in [5, 3, 8] {
            a.insert(value);
        }
        let mut b = BinarySearchTree::new();
        for value in [3, 5, 8] {
            b.insert(value);
        }
        assert!(!a.is_identical(&b));
    }

    #[test]
    fn test_is_identical_empty_trees() {
        let a: BinarySearchTree<i32> = BinarySearchTree::new();
        let b: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(a.is_identical(&b));
    }
}
