# Tree Algorithms

Key patterns and techniques for solving tree-based interview problems.

## Core Problem Patterns

- **Traversal**: preorder, inorder, postorder, level-order (BFS)
- **Depth / height / diameter**: recursive DFS with aggregated return values
- **Path-based queries**: root-to-leaf paths, path sums, lowest common ancestor (LCA)
- **Structural checks**: balanced tree, symmetric tree, valid BST
- **Modification / construction**: build from traversals or lists, insert/delete operations

## Typical Techniques

- Recursive DFS with `Option<Rc<RefCell<Node>>>` in Rust
- Iterative traversals using explicit stacks or queues
- Encoding/decoding trees for serialization problems
- Using parent pointers vs storing paths on the stack
- Binary lifting / Euler tour for advanced LCA queries

---
*Links: [[10-common-interview-problems]] [[tree-data-structures]] [[mission-10]] [[union-find-algorithm]]*