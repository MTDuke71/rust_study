 //! # Step 3: Tree Traversal Algorithms
//!
//! This tutorial covers the four classic ways to visit every node in a tree,
//! each recursively and iteratively, plus practical applications: serializing
//! a tree to a string and reconstructing one from its traversals.
//!
//! ## The Four Traversal Orders
//!
//! Using this tree from Step 2 (`BinarySearchTree` built from
//! `[8, 3, 10, 1, 6, 14, 4, 7, 13]`):
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
//! | Order      | Visit sequence          | Result                              |
//! |------------|--------------------------|--------------------------------------|
//! | Pre-order  | root, left, right        | `8 3 1 6 4 7 10 14 13`                |
//! | In-order   | left, root, right        | `1 3 4 6 7 8 10 13 14` (sorted, BST!) |
//! | Post-order | left, right, root        | `1 4 7 6 3 13 14 10 8`                |
//! | Level-order| top row to bottom row     | `[8] [3,10] [1,6,14] [4,7,13]`        |
//!
//! **In-order is special for a BST**: because left < parent < right at every
//! node, visiting left-root-right always yields values in sorted order.
//! Pre-order and post-order don't depend on the BST property at all — they
//! work identically on *any* binary tree, which is why Step 3's
//! "reconstruct from traversals" exercise doesn't need BST ordering either.
//!
//! ## Why Iterative Versions?
//!
//! Recursive traversal is what the call stack does implicitly: each
//! recursive call pushes a frame, each return pops one. The iterative
//! versions below make that stack **explicit** with a `Vec` (LIFO) or
//! `VecDeque` (FIFO for level-order). Two reasons this matters in practice:
//!
//! - A tree with 100,000 nodes in a straight line (degenerate/unbalanced,
//!   see Step 2) recurses 100,000 frames deep → stack overflow. An explicit
//!   `Vec`-based stack lives on the heap and has no such limit.
//! - Seeing the stack as data (not hidden call frames) is what makes
//!   post-order's "two-stack trick" click — you can't see that trick by
//!   staring at the recursive version.

use std::collections::VecDeque;

// =============================================================================
// Node & BinarySearchTree (same shape as Step 2)
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

/// A Binary Search Tree, extended in this step with traversal algorithms.
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

    fn count_nodes(node: &Option<Box<Node<T>>>) -> usize {
        match node {
            None => 0,
            Some(n) => 1 + Self::count_nodes(&n.left) + Self::count_nodes(&n.right),
        }
    }
}

impl<T: Ord> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// RECURSIVE TRAVERSALS
// =============================================================================

impl<T: Ord> BinarySearchTree<T> {
    /// Pre-order: root, left, right. Time O(n), Space O(h) (call stack).
    ///
    /// Common use: copying/serializing a tree (visit root before children,
    /// so a parser reading left-to-right can rebuild top-down).
    pub fn pre_order(&self) -> Vec<&T> {
        let mut result = Vec::with_capacity(self.size);
        Self::pre_order_recursive(&self.root, &mut result);
        result
    }

    fn pre_order_recursive<'a>(node: &'a Option<Box<Node<T>>>, result: &mut Vec<&'a T>) {
        if let Some(current) = node {
            result.push(&current.value);
            Self::pre_order_recursive(&current.left, result);
            Self::pre_order_recursive(&current.right, result);
        }
    }

    /// In-order: left, root, right. Time O(n), Space O(h).
    ///
    /// For a BST specifically: produces values in sorted order.
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

    /// Post-order: left, right, root. Time O(n), Space O(h).
    ///
    /// Common use: deleting/freeing a tree (visit children before the
    /// parent, so nothing is freed out from under a still-needed node) or
    /// evaluating an expression tree (operands before operator).
    pub fn post_order(&self) -> Vec<&T> {
        let mut result = Vec::with_capacity(self.size);
        Self::post_order_recursive(&self.root, &mut result);
        result
    }

    fn post_order_recursive<'a>(node: &'a Option<Box<Node<T>>>, result: &mut Vec<&'a T>) {
        if let Some(current) = node {
            Self::post_order_recursive(&current.left, result);
            Self::post_order_recursive(&current.right, result);
            result.push(&current.value);
        }
    }
}

// =============================================================================
// ITERATIVE TRAVERSALS (explicit Vec-as-stack)
// =============================================================================

impl<T: Ord> BinarySearchTree<T> {
    /// Pre-order, iterative. Push right before left so left pops first.
    pub fn pre_order_iterative(&self) -> Vec<&T> {
        let mut result = Vec::with_capacity(self.size);
        let mut stack: Vec<&Node<T>> = Vec::new();
        if let Some(root) = &self.root {
            stack.push(root);
        }
        while let Some(node) = stack.pop() {
            result.push(&node.value);
            if let Some(right) = &node.right {
                stack.push(right);
            }
            if let Some(left) = &node.left {
                stack.push(left);
            }
        }
        result
    }

    /// In-order, iterative. Walk left as far as possible (pushing along the
    /// way), then visit, then step right and repeat.
    pub fn in_order_iterative(&self) -> Vec<&T> {
        let mut result = Vec::with_capacity(self.size);
        let mut stack: Vec<&Node<T>> = Vec::new();
        let mut current = self.root.as_deref();

        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                stack.push(node);
                current = node.left.as_deref();
            }
            if let Some(node) = stack.pop() {
                result.push(&node.value);
                current = node.right.as_deref();
            }
        }
        result
    }

    /// Post-order, iterative, via the "two-stack trick": produce
    /// root-right-left order on `stack2` (a mirrored pre-order), then pop
    /// `stack2` — which reverses it into left-right-root.
    pub fn post_order_iterative(&self) -> Vec<&T> {
        let mut stack1: Vec<&Node<T>> = Vec::new();
        let mut stack2: Vec<&Node<T>> = Vec::new();
        if let Some(root) = &self.root {
            stack1.push(root);
        }

        while let Some(node) = stack1.pop() {
            stack2.push(node);
            if let Some(left) = &node.left {
                stack1.push(left);
            }
            if let Some(right) = &node.right {
                stack1.push(right);
            }
        }

        let mut result = Vec::with_capacity(self.size);
        while let Some(node) = stack2.pop() {
            result.push(&node.value);
        }
        result
    }
}

// =============================================================================
// LEVEL-ORDER (BFS with VecDeque)
// =============================================================================

impl<T: Ord> BinarySearchTree<T> {
    /// Level-order (breadth-first): one inner `Vec` per depth level.
    /// Time O(n), Space O(w) where w = max width of the tree.
    pub fn level_order(&self) -> Vec<Vec<&T>> {
        let mut result = Vec::new();
        let mut queue: VecDeque<&Node<T>> = VecDeque::new();
        if let Some(root) = &self.root {
            queue.push_back(root);
        }

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut level = Vec::with_capacity(level_size);
            for _ in 0..level_size {
                // level_size was captured before this loop, so pop_front
                // only ever drains the nodes that belong to *this* level —
                // children pushed inside the loop wait for the next round.
                let node = queue.pop_front().expect("level_size matches queue length");
                level.push(&node.value);
                if let Some(left) = &node.left {
                    queue.push_back(left);
                }
                if let Some(right) = &node.right {
                    queue.push_back(right);
                }
            }
            result.push(level);
        }
        result
    }

    /// Zigzag level-order: alternate levels read left-to-right, then
    /// right-to-left. Same BFS as `level_order`, just reverse odd levels.
    pub fn zigzag_level_order(&self) -> Vec<Vec<&T>> {
        let mut levels = self.level_order();
        for (depth, level) in levels.iter_mut().enumerate() {
            if depth % 2 == 1 {
                level.reverse();
            }
        }
        levels
    }
}

// =============================================================================
// SERIALIZE / DESERIALIZE
// =============================================================================

impl<T: Ord> BinarySearchTree<T> {
    /// Serialize to a comma-separated pre-order string, with `#` marking a
    /// missing child. Pre-order (not in-order) is the encoding of choice
    /// because it visits the root *before* its children — a left-to-right
    /// reader can rebuild top-down without looking ahead.
    ///
    /// Example: the Step 2 sample tree becomes
    /// `"8,3,1,#,#,6,4,#,#,7,#,#,10,#,14,13,#,#,#"`.
    pub fn serialize(&self) -> String
    where
        T: std::fmt::Display,
    {
        let mut parts = Vec::new();
        Self::serialize_recursive(&self.root, &mut parts);
        parts.join(",")
    }

    fn serialize_recursive(node: &Option<Box<Node<T>>>, parts: &mut Vec<String>)
    where
        T: std::fmt::Display,
    {
        match node {
            None => parts.push("#".to_string()),
            Some(current) => {
                parts.push(current.value.to_string());
                Self::serialize_recursive(&current.left, parts);
                Self::serialize_recursive(&current.right, parts);
            }
        }
    }

    /// Deserialize a string produced by [`serialize`](Self::serialize) back
    /// into a tree with the *exact same shape* (not rebuilt via `insert`,
    /// which could rebalance differently — this restores the original
    /// structure node-for-node).
    pub fn deserialize(data: &str) -> Self
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut parts = data.split(',');
        let root = Self::deserialize_recursive(&mut parts);
        let size = Self::count_nodes(&root);
        BinarySearchTree { root, size }
    }

    fn deserialize_recursive<'a, I>(parts: &mut I) -> Option<Box<Node<T>>>
    where
        I: Iterator<Item = &'a str>,
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        match parts.next() {
            None | Some("#") => None,
            Some(token) => {
                let value: T = token.parse().expect("token should parse as T");
                let left = Self::deserialize_recursive(parts);
                let right = Self::deserialize_recursive(parts);
                Some(Box::new(Node { value, left, right }))
            }
        }
    }
}

// =============================================================================
// RECONSTRUCT FROM PRE-ORDER + IN-ORDER
// =============================================================================

impl<T: Ord + Clone> BinarySearchTree<T> {
    /// Rebuild a tree from its pre-order and in-order traversals.
    ///
    /// This is a general binary-tree algorithm — it does **not** rely on
    /// the BST ordering property, only on values being distinct. It works
    /// because:
    /// - Pre-order's first element is always the root.
    /// - That root's position in in-order splits the rest into "everything
    ///   in the left subtree" and "everything in the right subtree".
    /// - The left subtree's size tells you where to split what's left of
    ///   pre-order the same way.
    ///
    /// Requires: `preorder` and `inorder` are traversals of the same tree
    /// with no duplicate values.
    pub fn from_pre_in_order(preorder: &[T], inorder: &[T]) -> Self {
        let root = Self::build_from_pre_in(preorder, inorder);
        let size = Self::count_nodes(&root);
        BinarySearchTree { root, size }
    }

    fn build_from_pre_in(preorder: &[T], inorder: &[T]) -> Option<Box<Node<T>>> {
        let (root_value, rest_preorder) = preorder.split_first()?;

        let root_idx = inorder
            .iter()
            .position(|v| v == root_value)
            .expect("preorder and inorder must be traversals of the same tree");
        let (left_inorder, after_root) = inorder.split_at(root_idx);
        let right_inorder = &after_root[1..]; // skip the root value itself

        let left_preorder = &rest_preorder[..left_inorder.len()];
        let right_preorder = &rest_preorder[left_inorder.len()..];

        Some(Box::new(Node {
            value: root_value.clone(),
            left: Self::build_from_pre_in(left_preorder, left_inorder),
            right: Self::build_from_pre_in(right_preorder, right_inorder),
        }))
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
    println!("🌳 Step 3: Tree Traversal Algorithms\n");

    let bst = sample_tree();
    println!("Using the Step 2 sample tree (inserted 8,3,10,1,6,14,4,7,13):");
    println!("        8");
    println!("       / \\");
    println!("      3   10");
    println!("     / \\    \\");
    println!("    1   6    14");
    println!("       / \\   /");
    println!("      4   7 13\n");

    // --- Recursive traversals ---
    println!("=== Recursive Traversals ===");
    print_row("Pre-order  (root,left,right)", &bst.pre_order());
    print_row("In-order   (left,root,right)", &bst.in_order());
    print_row("Post-order (left,right,root)", &bst.post_order());
    println!();

    // --- Iterative traversals should match recursive exactly ---
    println!("=== Iterative Traversals (should match recursive above) ===");
    print_row("Pre-order  iterative", &bst.pre_order_iterative());
    print_row("In-order   iterative", &bst.in_order_iterative());
    print_row("Post-order iterative", &bst.post_order_iterative());
    assert_eq!(bst.pre_order(), bst.pre_order_iterative());
    assert_eq!(bst.in_order(), bst.in_order_iterative());
    assert_eq!(bst.post_order(), bst.post_order_iterative());
    println!("  ✓ Iterative results match recursive results\n");

    // --- Level-order / BFS ---
    println!("=== Level-Order (BFS) ===");
    for (depth, level) in bst.level_order().iter().enumerate() {
        println!("  Depth {depth}: {level:?}");
    }
    println!();

    println!("=== Zigzag Level-Order ===");
    for (depth, level) in bst.zigzag_level_order().iter().enumerate() {
        let direction = if depth % 2 == 0 { "→" } else { "←" };
        println!("  Depth {depth} {direction}: {level:?}");
    }
    println!();

    // --- Serialize / deserialize round-trip ---
    println!("=== Serialize / Deserialize ===");
    let encoded = bst.serialize();
    println!("  Serialized: {encoded}");
    let decoded: BinarySearchTree<i32> = BinarySearchTree::deserialize(&encoded);
    print_row("  Decoded pre-order ", &decoded.pre_order());
    print_row("  Decoded in-order  ", &decoded.in_order());
    assert_eq!(bst.pre_order(), decoded.pre_order());
    assert_eq!(bst.in_order(), decoded.in_order());
    println!("  ✓ Round-trip preserved exact tree shape\n");

    // --- Reconstruct from traversals (classic interview problem) ---
    println!("=== Reconstruct from Pre-order + In-order ===");
    // A different, non-BST-inserted example tree:
    //        3
    //       / \
    //      9  20
    //        /  \
    //       15   7
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    println!("  Given preorder: {preorder:?}");
    println!("  Given inorder:  {inorder:?}");
    let rebuilt = BinarySearchTree::from_pre_in_order(&preorder, &inorder);
    print_row("  Rebuilt pre-order ", &rebuilt.pre_order());
    print_row("  Rebuilt post-order", &rebuilt.post_order());
    assert_eq!(rebuilt.pre_order(), preorder.iter().collect::<Vec<_>>());
    assert_eq!(rebuilt.post_order(), vec![&9, &15, &7, &20, &3]);
    println!("  ✓ Reconstructed tree matches the given traversals");
    println!("  (works without the BST property — only pre/in-order + distinct values)\n");

    // Key Takeaways
    println!("=== Key Takeaways ===\n");
    println!("✓ Pre/in/post-order differ only in *when* the root is visited");
    println!("✓ In-order on a BST always yields sorted output");
    println!("✓ Iterative versions trade implicit call-stack for an explicit Vec/VecDeque");
    println!("✓ Post-order's two-stack trick: mirrored pre-order, then reverse");
    println!("✓ Level-order needs a queue (FIFO), not a stack (LIFO)");
    println!("✓ Pre-order + in-order together uniquely determine a tree's shape");
    println!("✓ Next: BST delete and advanced operations (Step 4)");
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

    #[test]
    fn test_pre_order_recursive() {
        let bst = sample_tree();
        assert_eq!(
            values(bst.pre_order()),
            vec![8, 3, 1, 6, 4, 7, 10, 14, 13]
        );
    }

    #[test]
    fn test_in_order_recursive_is_sorted() {
        let bst = sample_tree();
        assert_eq!(
            values(bst.in_order()),
            vec![1, 3, 4, 6, 7, 8, 10, 13, 14]
        );
    }

    #[test]
    fn test_post_order_recursive() {
        let bst = sample_tree();
        assert_eq!(
            values(bst.post_order()),
            vec![1, 4, 7, 6, 3, 13, 14, 10, 8]
        );
    }

    #[test]
    fn test_iterative_matches_recursive() {
        let bst = sample_tree();
        assert_eq!(bst.pre_order(), bst.pre_order_iterative());
        assert_eq!(bst.in_order(), bst.in_order_iterative());
        assert_eq!(bst.post_order(), bst.post_order_iterative());
    }

    #[test]
    fn test_level_order() {
        let bst = sample_tree();
        let levels: Vec<Vec<i32>> = bst
            .level_order()
            .into_iter()
            .map(values)
            .collect();
        assert_eq!(
            levels,
            vec![
                vec![8],
                vec![3, 10],
                vec![1, 6, 14],
                vec![4, 7, 13],
            ]
        );
    }

    #[test]
    fn test_zigzag_level_order() {
        let bst = sample_tree();
        let levels: Vec<Vec<i32>> = bst
            .zigzag_level_order()
            .into_iter()
            .map(values)
            .collect();
        assert_eq!(
            levels,
            vec![
                vec![8],
                vec![10, 3],
                vec![1, 6, 14],
                vec![13, 7, 4],
            ]
        );
    }

    #[test]
    fn test_serialize_deserialize_round_trip() {
        let bst = sample_tree();
        let encoded = bst.serialize();
        let decoded: BinarySearchTree<i32> = BinarySearchTree::deserialize(&encoded);
        assert_eq!(bst.pre_order(), decoded.pre_order());
        assert_eq!(bst.in_order(), decoded.in_order());
        assert_eq!(bst.len(), decoded.len());
    }

    #[test]
    fn test_serialize_empty_tree() {
        let bst: BinarySearchTree<i32> = BinarySearchTree::new();
        assert_eq!(bst.serialize(), "#");
        let decoded: BinarySearchTree<i32> = BinarySearchTree::deserialize(&bst.serialize());
        assert!(decoded.is_empty());
    }

    #[test]
    fn test_from_pre_in_order() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let rebuilt = BinarySearchTree::from_pre_in_order(&preorder, &inorder);

        assert_eq!(values(rebuilt.pre_order()), preorder);
        assert_eq!(values(rebuilt.in_order()), inorder);
        assert_eq!(values(rebuilt.post_order()), vec![9, 15, 7, 20, 3]);
    }

    #[test]
    fn test_from_pre_in_order_single_node() {
        let rebuilt = BinarySearchTree::from_pre_in_order(&[5], &[5]);
        assert_eq!(values(rebuilt.pre_order()), vec![5]);
        assert_eq!(rebuilt.len(), 1);
    }

    #[test]
    fn test_from_pre_in_order_empty() {
        let rebuilt: BinarySearchTree<i32> = BinarySearchTree::from_pre_in_order(&[], &[]);
        assert!(rebuilt.is_empty());
    }

    #[test]
    fn test_empty_tree_traversals_dont_panic() {
        let bst: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(bst.pre_order().is_empty());
        assert!(bst.in_order().is_empty());
        assert!(bst.post_order().is_empty());
        assert!(bst.pre_order_iterative().is_empty());
        assert!(bst.in_order_iterative().is_empty());
        assert!(bst.post_order_iterative().is_empty());
        assert!(bst.level_order().is_empty());
    }
}
