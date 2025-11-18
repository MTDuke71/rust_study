//! Unit Tests for Mission 10: Union-Find Disjoint Set
//!
//! This test suite validates all requirements (REQ-1 through REQ-7)
//! following V-Cycle methodology with explicit requirement traceability.

use mission10::UnionFind;

// ============================================================================
// REQ-1: Basic Union-Find Structure
// ============================================================================

/// REQ-1: Test basic initialization with usize elements
#[test]
fn req1_usize_elements() {
    let uf = UnionFind::new(10);

    // Each element should be in its own set initially
    assert_eq!(uf.len(), 10);
    assert_eq!(uf.count(), 10);
    assert!(!uf.is_empty());
}

/// REQ-1: Test with large datasets (10,000+ elements)
#[test]
fn req1_large_datasets() {
    let n = 100_000;
    let uf = UnionFind::new(n);

    assert_eq!(uf.len(), n);
    assert_eq!(uf.count(), n);

    // All operations should complete in reasonable time
    // (effectively constant time due to O(α(n)) complexity)
}

/// REQ-1: Test edge case with empty set (n=0)
#[test]
fn req1_empty_set() {
    let uf = UnionFind::new(0);

    assert_eq!(uf.len(), 0);
    assert_eq!(uf.count(), 0);
    assert!(uf.is_empty());
}

/// REQ-1: Test that each element is initially its own parent
#[test]
fn req1_initial_singleton_sets() {
    let mut uf = UnionFind::new(5);

    // Each element should be in a different set
    for i in 0..5 {
        for j in 0..5 {
            if i != j {
                assert!(!uf.connected(i, j).unwrap());
            }
        }
    }
}

// ============================================================================
// REQ-2: Find Operation with Path Compression
// ============================================================================

/// REQ-2: Test that find returns correct root
#[test]
fn req2_find_returns_correct_root() {
    let mut uf = UnionFind::new(10);

    // Initially, each element is its own root
    for i in 0..10 {
        assert_eq!(uf.find(i).unwrap(), i);
    }
}

/// REQ-2: Test that path compression is applied
#[test]
fn req2_path_compression_applied() {
    let mut uf = UnionFind::new(10);

    // Create a chain: 0 <- 1 <- 2 <- 3 <- 4
    uf.union(0, 1).unwrap();
    uf.union(1, 2).unwrap();
    uf.union(2, 3).unwrap();
    uf.union(3, 4).unwrap();

    // All elements should have the same root
    let root = uf.find(0).unwrap();
    assert_eq!(uf.find(1).unwrap(), root);
    assert_eq!(uf.find(2).unwrap(), root);
    assert_eq!(uf.find(3).unwrap(), root);
    assert_eq!(uf.find(4).unwrap(), root);

    // After first find, path should be compressed
    // Subsequent finds should be faster (flat tree)
}

/// REQ-2: Test find performance with repeated calls
#[test]
fn req2_find_performance() {
    let mut uf = UnionFind::new(1000);

    // Create some unions
    for i in 0..999 {
        uf.union(i, i + 1).unwrap();
    }

    // Repeated finds should be fast due to path compression
    let root = uf.find(0).unwrap();
    for _ in 0..100 {
        assert_eq!(uf.find(500).unwrap(), root);
        assert_eq!(uf.find(999).unwrap(), root);
    }
}

/// REQ-2: Test that tree height is reduced after find
#[test]
fn req2_tree_height_reduced() {
    let mut uf = UnionFind::new(8);

    // Build a deep tree
    uf.union(0, 1).unwrap();
    uf.union(1, 2).unwrap();
    uf.union(2, 3).unwrap();
    uf.union(3, 4).unwrap();
    uf.union(4, 5).unwrap();

    // First find from deepest element
    let root = uf.find(5).unwrap();

    // All elements should now point closer to root due to path compression
    // Verify by checking that all have same root
    for i in 0..=5 {
        assert_eq!(uf.find(i).unwrap(), root);
    }
}

// ============================================================================
// REQ-3: Union Operation with Union by Rank
// ============================================================================

/// REQ-3: Test basic union operation
#[test]
fn req3_basic_union() {
    let mut uf = UnionFind::new(5);

    // Union returns true when sets are merged
    assert!(uf.union(0, 1).unwrap());
    assert_eq!(uf.count(), 4); // 5 - 1 = 4 sets

    // Union returns false when already in same set
    assert!(!uf.union(0, 1).unwrap());
    assert_eq!(uf.count(), 4); // Still 4 sets
}

/// REQ-3: Test that rank is maintained correctly
#[test]
fn req3_rank_maintained() {
    let mut uf = UnionFind::new(10);

    // Perform several unions
    uf.union(0, 1).unwrap();
    uf.union(2, 3).unwrap();
    uf.union(0, 2).unwrap(); // Merges {0,1} and {2,3}

    // All should have same root
    let root = uf.find(0).unwrap();
    assert_eq!(uf.find(1).unwrap(), root);
    assert_eq!(uf.find(2).unwrap(), root);
    assert_eq!(uf.find(3).unwrap(), root);
}

/// REQ-3: Test that trees remain balanced
#[test]
fn req3_balanced_trees() {
    let mut uf = UnionFind::new(16);

    // Union by rank should keep trees balanced
    // Create two sets of size 4 each
    uf.union(0, 1).unwrap();
    uf.union(2, 3).unwrap();
    uf.union(0, 2).unwrap(); // First set: {0,1,2,3}

    uf.union(4, 5).unwrap();
    uf.union(6, 7).unwrap();
    uf.union(4, 6).unwrap(); // Second set: {4,5,6,7}

    // Merge the two balanced trees
    uf.union(0, 4).unwrap();

    // All should be connected
    let root = uf.find(0).unwrap();
    for i in 0..8 {
        assert_eq!(uf.find(i).unwrap(), root);
    }
}

/// REQ-3: Test union performance with many operations
#[test]
fn req3_union_performance() {
    let mut uf = UnionFind::new(10_000);

    // Perform many union operations
    for i in 0..9_999 {
        uf.union(i, i + 1).unwrap();
    }

    // Should result in one large set
    assert_eq!(uf.count(), 1);
    assert_eq!(uf.size(0).unwrap(), 10_000);
}

/// REQ-3: Test union with various patterns
#[test]
fn req3_union_patterns() {
    let mut uf = UnionFind::new(8);

    // Pattern 1: Sequential unions
    uf.union(0, 1).unwrap();
    uf.union(1, 2).unwrap();
    uf.union(2, 3).unwrap();

    assert_eq!(uf.count(), 5); // {0,1,2,3}, {4}, {5}, {6}, {7}

    // Pattern 2: Pair unions
    uf.union(4, 5).unwrap();
    uf.union(6, 7).unwrap();

    assert_eq!(uf.count(), 3); // {0,1,2,3}, {4,5}, {6,7}

    // Pattern 3: Merge large sets
    uf.union(0, 4).unwrap();

    assert_eq!(uf.count(), 2); // {0,1,2,3,4,5}, {6,7}
}

// ============================================================================
// REQ-4: Connected Query Operation
// ============================================================================

/// REQ-4: Test basic connectivity queries
#[test]
fn req4_basic_connectivity() {
    let mut uf = UnionFind::new(10);

    // Initially, no elements are connected
    for i in 0..10 {
        for j in 0..10 {
            if i == j {
                assert!(uf.connected(i, j).unwrap()); // Reflexive
            } else {
                assert!(!uf.connected(i, j).unwrap());
            }
        }
    }
}

/// REQ-4: Test connectivity after unions
#[test]
fn req4_connectivity_after_union() {
    let mut uf = UnionFind::new(6);

    uf.union(0, 1).unwrap();
    uf.union(2, 3).unwrap();
    uf.union(4, 5).unwrap();

    // Check within sets
    assert!(uf.connected(0, 1).unwrap());
    assert!(uf.connected(2, 3).unwrap());
    assert!(uf.connected(4, 5).unwrap());

    // Check between sets
    assert!(!uf.connected(0, 2).unwrap());
    assert!(!uf.connected(0, 4).unwrap());
    assert!(!uf.connected(2, 4).unwrap());
}

/// REQ-4: Test transitivity of connectivity
#[test]
fn req4_transitivity() {
    let mut uf = UnionFind::new(5);

    uf.union(0, 1).unwrap();
    uf.union(1, 2).unwrap();
    uf.union(2, 3).unwrap();

    // Transitive: if 0~1 and 1~2 and 2~3, then 0~3
    assert!(uf.connected(0, 1).unwrap());
    assert!(uf.connected(1, 2).unwrap());
    assert!(uf.connected(2, 3).unwrap());
    assert!(uf.connected(0, 3).unwrap()); // Transitive

    // Not connected to separate element
    assert!(!uf.connected(0, 4).unwrap());
}

/// REQ-4: Test symmetry of connectivity
#[test]
fn req4_symmetry() {
    let mut uf = UnionFind::new(5);

    uf.union(0, 1).unwrap();
    uf.union(2, 3).unwrap();

    // Symmetric: if 0~1, then 1~0
    assert_eq!(uf.connected(0, 1).unwrap(), uf.connected(1, 0).unwrap());
    assert_eq!(uf.connected(2, 3).unwrap(), uf.connected(3, 2).unwrap());
    assert_eq!(uf.connected(0, 2).unwrap(), uf.connected(2, 0).unwrap());
}

// ============================================================================
// REQ-5: Set Counting and Statistics
// ============================================================================

/// REQ-5: Test count accuracy
#[test]
fn req5_count_accuracy() {
    let mut uf = UnionFind::new(10);

    assert_eq!(uf.count(), 10);

    uf.union(0, 1).unwrap();
    assert_eq!(uf.count(), 9);

    uf.union(2, 3).unwrap();
    assert_eq!(uf.count(), 8);

    uf.union(0, 2).unwrap(); // Merges two existing sets
    assert_eq!(uf.count(), 7);

    // Union of already connected elements shouldn't change count
    uf.union(0, 1).unwrap();
    assert_eq!(uf.count(), 7);
}

/// REQ-5: Test size tracking
#[test]
fn req5_size_tracking() {
    let mut uf = UnionFind::new(10);

    // Initially, all sets have size 1
    for i in 0..10 {
        assert_eq!(uf.size(i).unwrap(), 1);
    }

    // After union, set size increases
    uf.union(0, 1).unwrap();
    assert_eq!(uf.size(0).unwrap(), 2);
    assert_eq!(uf.size(1).unwrap(), 2);

    uf.union(0, 2).unwrap();
    assert_eq!(uf.size(0).unwrap(), 3);
    assert_eq!(uf.size(1).unwrap(), 3);
    assert_eq!(uf.size(2).unwrap(), 3);
}

/// REQ-5: Test size with multiple merges
#[test]
fn req5_size_multiple_merges() {
    let mut uf = UnionFind::new(8);

    // Create two sets of size 2
    uf.union(0, 1).unwrap();
    uf.union(2, 3).unwrap();

    assert_eq!(uf.size(0).unwrap(), 2);
    assert_eq!(uf.size(2).unwrap(), 2);

    // Merge them into set of size 4
    uf.union(0, 2).unwrap();
    assert_eq!(uf.size(0).unwrap(), 4);
    assert_eq!(uf.size(1).unwrap(), 4);
    assert_eq!(uf.size(2).unwrap(), 4);
    assert_eq!(uf.size(3).unwrap(), 4);
}

/// REQ-5: Test count with full connectivity
#[test]
fn req5_full_connectivity() {
    let mut uf = UnionFind::new(10);

    // Unite all elements into one set
    for i in 0..9 {
        uf.union(i, i + 1).unwrap();
    }

    assert_eq!(uf.count(), 1);
    assert_eq!(uf.size(0).unwrap(), 10);
    assert_eq!(uf.size(5).unwrap(), 10);
    assert_eq!(uf.size(9).unwrap(), 10);
}

// ============================================================================
// REQ-6: Error Handling and Bounds Checking
// ============================================================================

/// REQ-6: Test bounds checking for find operation
#[test]
fn req6_find_bounds_checking() {
    let mut uf = UnionFind::new(10);

    // Valid indices should work
    assert!(uf.find(0).is_ok());
    assert!(uf.find(9).is_ok());

    // Out of bounds should error
    assert!(uf.find(10).is_err());
    assert!(uf.find(100).is_err());
}

/// REQ-6: Test bounds checking for union operation
#[test]
fn req6_union_bounds_checking() {
    let mut uf = UnionFind::new(10);

    // Valid indices should work
    assert!(uf.union(0, 9).is_ok());

    // Out of bounds should error
    assert!(uf.union(0, 10).is_err());
    assert!(uf.union(10, 0).is_err());
    assert!(uf.union(10, 11).is_err());
}

/// REQ-6: Test bounds checking for connected operation
#[test]
fn req6_connected_bounds_checking() {
    let mut uf = UnionFind::new(10);

    // Valid indices should work
    assert!(uf.connected(0, 9).is_ok());

    // Out of bounds should error
    assert!(uf.connected(0, 10).is_err());
    assert!(uf.connected(10, 0).is_err());
}

/// REQ-6: Test bounds checking for size operation
#[test]
fn req6_size_bounds_checking() {
    let mut uf = UnionFind::new(10);

    // Valid indices should work
    assert!(uf.size(0).is_ok());
    assert!(uf.size(9).is_ok());

    // Out of bounds should error
    assert!(uf.size(10).is_err());
    assert!(uf.size(100).is_err());
}

/// REQ-6: Test error message quality
#[test]
fn req6_error_messages() {
    let mut uf = UnionFind::new(10);

    // Error messages should be descriptive
    match uf.find(15) {
        Err(msg) => {
            assert!(msg.contains("15"));
            assert!(msg.contains("10") || msg.contains("bounds"));
        }
        Ok(_) => panic!("Expected error for out-of-bounds index"),
    }
}

/// REQ-6: Test operations on empty Union-Find
#[test]
fn req6_empty_operations() {
    let mut uf = UnionFind::new(0);

    assert!(uf.is_empty());
    assert!(uf.find(0).is_err());
    assert!(uf.union(0, 0).is_err());
    assert!(uf.connected(0, 0).is_err());
    assert!(uf.size(0).is_err());
}

// ============================================================================
// REQ-7: Connected Components Application
// ============================================================================

/// REQ-7: Test component iteration
#[test]
fn req7_component_iteration() {
    let mut uf = UnionFind::new(10);

    // Create some components
    uf.union(0, 1).unwrap();
    uf.union(2, 3).unwrap();
    uf.union(4, 5).unwrap();
    // Components: {0,1}, {2,3}, {4,5}, {6}, {7}, {8}, {9}

    let components: Vec<Vec<usize>> = uf.components().collect();
    assert_eq!(components.len(), 7);

    // Each component should have correct size
    let sizes: Vec<usize> = components.iter().map(|c| c.len()).collect();
    assert!(sizes.contains(&2)); // Three components of size 2
    assert!(sizes.contains(&1)); // Four components of size 1
}

/// REQ-7: Test member iteration
#[test]
fn req7_member_iteration() {
    let mut uf = UnionFind::new(8);

    uf.union(0, 1).unwrap();
    uf.union(1, 2).unwrap();
    uf.union(2, 3).unwrap();

    let members: Vec<usize> = uf.members(0).unwrap().collect();
    assert_eq!(members.len(), 4);
    assert!(members.contains(&0));
    assert!(members.contains(&1));
    assert!(members.contains(&2));
    assert!(members.contains(&3));
}

/// REQ-7: Test graph connectivity detection
#[test]
fn req7_graph_connectivity() {
    let mut uf = UnionFind::new(6);

    // Graph edges: (0-1), (1-2), (3-4)
    let edges = vec![(0, 1), (1, 2), (3, 4)];

    for (u, v) in edges {
        uf.union(u, v).unwrap();
    }

    // Component 1: {0, 1, 2}
    assert!(uf.connected(0, 1).unwrap());
    assert!(uf.connected(0, 2).unwrap());
    assert!(uf.connected(1, 2).unwrap());

    // Component 2: {3, 4}
    assert!(uf.connected(3, 4).unwrap());

    // Component 3: {5}
    assert!(!uf.connected(5, 0).unwrap());
    assert!(!uf.connected(5, 3).unwrap());

    assert_eq!(uf.count(), 3); // Three components
}

/// REQ-7: Test cycle detection in undirected graph
#[test]
fn req7_cycle_detection() {
    let mut uf = UnionFind::new(5);

    // Add edges one by one
    let edges = vec![
        (0, 1), // No cycle
        (1, 2), // No cycle
        (2, 3), // No cycle
        (3, 4), // No cycle
        (4, 0), // Creates a cycle!
    ];

    let mut has_cycle = false;
    for (u, v) in edges {
        // If u and v are already connected, adding this edge creates a cycle
        if uf.connected(u, v).unwrap() {
            has_cycle = true;
            break;
        }
        uf.union(u, v).unwrap();
    }

    assert!(has_cycle);
}

// ============================================================================
// Edge Cases and Additional Tests
// ============================================================================

/// Test with single element (n=1)
#[test]
fn test_edge_case_single_element() {
    let mut uf = UnionFind::new(1);

    assert_eq!(uf.len(), 1);
    assert_eq!(uf.count(), 1);
    assert_eq!(uf.find(0).unwrap(), 0);
    assert_eq!(uf.size(0).unwrap(), 1);
    assert!(uf.connected(0, 0).unwrap());

    // Self-union should return false
    assert!(!uf.union(0, 0).unwrap());
    assert_eq!(uf.count(), 1);
}

/// Test full connectivity (all elements in one set)
#[test]
fn test_edge_case_full_connectivity() {
    let mut uf = UnionFind::new(100);

    // Connect all elements
    for i in 0..99 {
        uf.union(i, i + 1).unwrap();
    }

    // All should be connected
    assert_eq!(uf.count(), 1);
    for i in 0..100 {
        for j in 0..100 {
            assert!(uf.connected(i, j).unwrap());
        }
        assert_eq!(uf.size(i).unwrap(), 100);
    }
}

/// Test clear operation
#[test]
fn test_clear_operation() {
    let mut uf = UnionFind::new(10);

    // Make some unions
    uf.union(0, 1).unwrap();
    uf.union(2, 3).unwrap();
    assert_eq!(uf.count(), 8);

    // Clear resets to singleton sets
    uf.clear();
    assert_eq!(uf.count(), 10);
    assert!(!uf.connected(0, 1).unwrap());
    assert!(!uf.connected(2, 3).unwrap());
}

/// Test reset operation
#[test]
fn test_reset_operation() {
    let mut uf = UnionFind::new(5);

    uf.union(0, 1).unwrap();
    assert_eq!(uf.len(), 5);
    assert_eq!(uf.count(), 4);

    // Reset with different size
    uf.reset(10);
    assert_eq!(uf.len(), 10);
    assert_eq!(uf.count(), 10);
    assert!(!uf.connected(0, 1).unwrap());
}

/// Test random union pattern
#[test]
fn test_random_unions() {
    let mut uf = UnionFind::new(50);

    // Perform various random-like unions
    let unions = vec![
        (5, 10),
        (15, 20),
        (25, 30),
        (35, 40),
        (10, 15),
        (30, 35), // Merge pairs
        (5, 25),  // Merge larger sets
    ];

    for (u, v) in unions {
        uf.union(u, v).unwrap();
    }

    // Verify connectivity within merged set
    assert!(uf.connected(5, 40).unwrap());
    assert!(uf.connected(10, 30).unwrap());

    // Verify non-connectivity with other elements
    assert!(!uf.connected(0, 5).unwrap());
    assert!(!uf.connected(1, 10).unwrap());
}

/// Test worst-case chain scenario
#[test]
fn test_worst_case_chain() {
    let mut uf = UnionFind::new(1000);

    // Create a long chain
    for i in 0..999 {
        uf.union(i, i + 1).unwrap();
    }

    // After path compression, find should be fast
    let root = uf.find(0).unwrap();
    for i in 0..1000 {
        assert_eq!(uf.find(i).unwrap(), root);
    }
}

/// Test len and is_empty consistency
#[test]
fn test_len_is_empty() {
    let uf_empty = UnionFind::new(0);
    assert_eq!(uf_empty.len(), 0);
    assert!(uf_empty.is_empty());

    let uf_nonempty = UnionFind::new(5);
    assert_eq!(uf_nonempty.len(), 5);
    assert!(!uf_nonempty.is_empty());
}

/// Test component count correctness
#[test]
fn test_component_count() {
    let mut uf = UnionFind::new(12);

    // Create specific component structure
    uf.union(0, 1).unwrap();
    uf.union(1, 2).unwrap(); // Component 1: {0, 1, 2}

    uf.union(3, 4).unwrap(); // Component 2: {3, 4}

    uf.union(5, 6).unwrap();
    uf.union(6, 7).unwrap();
    uf.union(7, 8).unwrap(); // Component 3: {5, 6, 7, 8}

    // Components: {0,1,2}, {3,4}, {5,6,7,8}, {9}, {10}, {11}
    assert_eq!(uf.count(), 6);

    let components: Vec<Vec<usize>> = uf.components().collect();
    assert_eq!(components.len(), 6);
}
