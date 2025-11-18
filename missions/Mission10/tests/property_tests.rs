//! Property-based tests for Union-Find data structure using QuickCheck
//!
//! These tests verify mathematical properties that should hold for any valid
//! sequence of Union-Find operations, regardless of the specific input values.

use mission10::UnionFind;
use quickcheck::TestResult;
use quickcheck_macros::quickcheck;
use std::collections::HashSet;

// Custom types for QuickCheck to generate reasonable test inputs
#[derive(Clone, Debug)]
struct UnionFindSize(usize);

impl quickcheck::Arbitrary for UnionFindSize {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        // Generate sizes between 1 and 50 to keep tests reasonable
        UnionFindSize(usize::arbitrary(g) % 50 + 1)
    }
}

#[derive(Clone, Debug)]
struct ElementPair {
    size: usize,
    element1: usize,
    element2: usize,
}

impl quickcheck::Arbitrary for ElementPair {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let size = usize::arbitrary(g) % 50 + 1;
        let element1 = usize::arbitrary(g) % size;
        let element2 = usize::arbitrary(g) % size;
        ElementPair {
            size,
            element1,
            element2,
        }
    }
}

#[derive(Clone, Debug)]
struct UnionSequence {
    size: usize,
    operations: Vec<(usize, usize)>,
}

impl quickcheck::Arbitrary for UnionSequence {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let size = usize::arbitrary(g) % 20 + 2; // 2-21 elements
        let num_ops = usize::arbitrary(g) % (size * 2); // 0 to 2*size operations

        let mut operations = Vec::new();
        for _ in 0..num_ops {
            let a = usize::arbitrary(g) % size;
            let b = usize::arbitrary(g) % size;
            operations.push((a, b));
        }

        UnionSequence { size, operations }
    }
}

/// Property: Union operation is commutative
///
/// For any elements x and y, union(x, y) should produce the same result as union(y, x)
/// in terms of the final connectivity of all elements.
#[quickcheck]
fn prop_union_is_commutative(pair: ElementPair) -> TestResult {
    let ElementPair {
        size,
        element1,
        element2,
    } = pair;

    // Skip if elements are the same (trivial case)
    if element1 == element2 {
        return TestResult::discard();
    }

    // Create two identical Union-Find structures
    let mut uf1 = UnionFind::new(size);
    let mut uf2 = UnionFind::new(size);

    // Apply union in different orders
    let _ = uf1.union(element1, element2);
    let _ = uf2.union(element2, element1);

    // Check that all connectivity relationships are identical
    for i in 0..size {
        for j in 0..size {
            let connected1 = uf1.connected(i, j).unwrap_or(false);
            let connected2 = uf2.connected(i, j).unwrap_or(false);
            if connected1 != connected2 {
                return TestResult::failed();
            }
        }
    }

    // Check that component counts are identical
    TestResult::from_bool(uf1.count() == uf2.count())
}

/// Property: Find operation is idempotent
///
/// Calling find(x) multiple times should always return the same result,
/// and should not change the structure's connectivity.
#[quickcheck]
fn prop_find_is_idempotent(pair: ElementPair) -> TestResult {
    let ElementPair {
        size,
        element1,
        element2,
    } = pair;

    let mut uf = UnionFind::new(size);

    // Perform some unions to create non-trivial structure
    let _ = uf.union(element1, element2);
    if size > 2 {
        let _ = uf.union(0, size - 1);
    }

    // Test idempotency for each element
    for element in 0..size {
        // Record state before find operations
        let initial_count = uf.count();
        let mut connectivity_before = Vec::new();
        for i in 0..size {
            for j in 0..size {
                connectivity_before.push(uf.connected(i, j).unwrap_or(false));
            }
        }

        // Call find multiple times
        let first_find = uf.find(element);
        let second_find = uf.find(element);
        let third_find = uf.find(element);

        // All find results should be identical
        if !(first_find == second_find && second_find == third_find) {
            return TestResult::failed();
        }

        // Structure should be unchanged
        if uf.count() != initial_count {
            return TestResult::failed();
        }

        // Connectivity should be unchanged
        let mut connectivity_after = Vec::new();
        for i in 0..size {
            for j in 0..size {
                connectivity_after.push(uf.connected(i, j).unwrap_or(false));
            }
        }

        if connectivity_before != connectivity_after {
            return TestResult::failed();
        }
    }

    TestResult::passed()
}

/// Property: Connectivity is transitive
///
/// If connected(x, y) and connected(y, z) are both true, then connected(x, z) must be true.
/// This is a fundamental property of equivalence relations.
#[quickcheck]
fn prop_transitive_connectivity(seq: UnionSequence) -> TestResult {
    let UnionSequence { size, operations } = seq;

    if size < 3 {
        return TestResult::discard();
    }

    let mut uf = UnionFind::new(size);

    // Apply all union operations
    for (a, b) in operations {
        let _ = uf.union(a, b);
    }

    // Check transitivity for all triples
    for x in 0..size {
        for y in 0..size {
            for z in 0..size {
                let xy = uf.connected(x, y).unwrap_or(false);
                let yz = uf.connected(y, z).unwrap_or(false);
                let xz = uf.connected(x, z).unwrap_or(false);

                // If x-y and y-z are connected, then x-z must be connected
                if xy && yz && !xz {
                    return TestResult::failed();
                }
            }
        }
    }

    TestResult::passed()
}

/// Property: Set count decreases or stays the same after union operations
///
/// Each union operation should either decrease the number of disjoint sets by 1
/// (if the elements were in different sets) or leave it unchanged (if they were
/// already in the same set).
#[quickcheck]
fn prop_union_count_decreases_or_same(seq: UnionSequence) -> TestResult {
    let UnionSequence { size, operations } = seq;

    let mut uf = UnionFind::new(size);
    let mut current_count = uf.count();

    for (a, b) in operations {
        let were_connected = uf.connected(a, b).unwrap_or(true); // Assume connected if error

        // Perform union
        if uf.union(a, b).is_ok() {
            let new_count = uf.count();

            if were_connected {
                // If elements were already connected, count should stay the same
                if new_count != current_count {
                    return TestResult::failed();
                }
            } else {
                // If elements were not connected, count should decrease by 1
                if new_count != current_count - 1 {
                    return TestResult::failed();
                }
            }

            current_count = new_count;
        }
    }

    // Final count should be between 1 and original size
    TestResult::from_bool(current_count >= 1 && current_count <= size)
}

/// Property: Connected is symmetric
///
/// For any elements x and y, connected(x, y) should equal connected(y, x).
/// This tests the symmetry property of the equivalence relation.
#[quickcheck]
fn prop_connected_is_symmetric(seq: UnionSequence) -> TestResult {
    let UnionSequence { size, operations } = seq;

    let mut uf = UnionFind::new(size);

    // Apply all union operations
    for (a, b) in operations {
        let _ = uf.union(a, b);
    }

    // Check symmetry for all pairs
    for x in 0..size {
        for y in 0..size {
            let xy = uf.connected(x, y).unwrap_or(false);
            let yx = uf.connected(y, x).unwrap_or(false);

            if xy != yx {
                return TestResult::failed();
            }
        }
    }

    TestResult::passed()
}

/// Property: Connected is reflexive
///
/// For any element x, connected(x, x) should always be true.
/// Every element is connected to itself.
#[quickcheck]
fn prop_connected_is_reflexive(uf_size: UnionFindSize) -> TestResult {
    let UnionFindSize(size) = uf_size;
    let mut uf = UnionFind::new(size);

    // Check reflexivity for all elements
    for x in 0..size {
        if !uf.connected(x, x).unwrap_or(false) {
            return TestResult::failed();
        }
    }

    TestResult::passed()
}

/// Property: Size of component is consistent
///
/// The size of a component containing element x should equal the number of elements
/// that are connected to x.
#[quickcheck]
fn prop_component_size_consistency(seq: UnionSequence) -> TestResult {
    let UnionSequence { size, operations } = seq;

    let mut uf = UnionFind::new(size);

    // Apply all union operations
    for (a, b) in operations {
        let _ = uf.union(a, b);
    }

    // Check size consistency for each element
    for x in 0..size {
        if let Ok(reported_size) = uf.size(x) {
            // Count actual number of elements connected to x
            let mut actual_size = 0;
            for y in 0..size {
                if uf.connected(x, y).unwrap_or(false) {
                    actual_size += 1;
                }
            }

            if reported_size != actual_size {
                return TestResult::failed();
            }
        }
    }

    TestResult::passed()
}

/// Property: Total elements in all components equals original size
///
/// The sum of sizes of all disjoint components should equal the total number of elements.
#[quickcheck]
fn prop_total_elements_conservation(seq: UnionSequence) -> TestResult {
    let UnionSequence { size, operations } = seq;

    let mut uf = UnionFind::new(size);

    // Apply all union operations
    for (a, b) in operations {
        let _ = uf.union(a, b);
    }

    // Find all unique component roots
    let mut roots = HashSet::new();
    for i in 0..size {
        if let Ok(root) = uf.find(i) {
            roots.insert(root);
        }
    }

    // Sum sizes of all components
    let mut total_size = 0;
    for &root in &roots {
        if let Ok(component_size) = uf.size(root) {
            total_size += component_size;
        }
    }

    // Total should equal original size
    TestResult::from_bool(total_size == size)
}

/// Property: Union operations preserve existing connectivity
///
/// If two elements were connected before a union operation, they should remain
/// connected after any union operation.
#[quickcheck]
fn prop_union_preserves_connectivity(seq: UnionSequence) -> TestResult {
    let UnionSequence { size, operations } = seq;

    if operations.is_empty() {
        return TestResult::discard();
    }

    let mut uf = UnionFind::new(size);

    // Apply operations one by one, checking preservation
    for &(a, b) in operations.iter() {
        // Record all current connections
        let mut connections = Vec::new();
        for x in 0..size {
            for y in 0..size {
                if uf.connected(x, y).unwrap_or(false) {
                    connections.push((x, y));
                }
            }
        }

        // Apply the union operation
        let _ = uf.union(a, b);

        // Check that all previous connections are preserved
        for (x, y) in connections {
            if !uf.connected(x, y).unwrap_or(false) {
                return TestResult::failed();
            }
        }
    }

    TestResult::passed()
}

/// Property: Find returns valid element indices
#[quickcheck]
fn prop_find_returns_valid_indices(uf_size: UnionFindSize) -> TestResult {
    let UnionFindSize(size) = uf_size;
    let mut uf = UnionFind::new(size);

    for element in 0..size {
        if let Ok(root) = uf.find(element) {
            if root >= size {
                return TestResult::failed();
            }
        } else {
            return TestResult::failed();
        }
    }

    TestResult::passed()
}

/// Property: Count is always positive and at most the original size
#[quickcheck]
fn prop_count_bounds(seq: UnionSequence) -> TestResult {
    let UnionSequence { size, operations } = seq;

    let mut uf = UnionFind::new(size);

    // Initial count should equal size
    if uf.count() != size {
        return TestResult::failed();
    }

    // Apply operations and check bounds
    for (a, b) in operations {
        let _ = uf.union(a, b);
        let count = uf.count();

        if count < 1 || count > size {
            return TestResult::failed();
        }
    }

    TestResult::passed()
}

#[cfg(test)]
mod manual_property_tests {
    use super::*;

    /// Manually test union commutativity with specific examples
    #[test]
    fn test_union_commutativity_manual() {
        let mut uf1 = UnionFind::new(5);
        let mut uf2 = UnionFind::new(5);

        // Apply union in different orders
        let _ = uf1.union(1, 3);
        let _ = uf2.union(3, 1);

        // Check that connectivity is the same
        for i in 0..5 {
            for j in 0..5 {
                assert_eq!(
                    uf1.connected(i, j).unwrap_or(false),
                    uf2.connected(i, j).unwrap_or(false)
                );
            }
        }

        assert_eq!(uf1.count(), uf2.count());
    }

    /// Manually test find idempotency
    #[test]
    fn test_find_idempotency_manual() {
        let mut uf = UnionFind::new(4);
        let _ = uf.union(0, 2);

        // Test idempotency for each element
        for element in 0..4 {
            let first_find = uf.find(element);
            let second_find = uf.find(element);
            let third_find = uf.find(element);

            assert_eq!(first_find, second_find);
            assert_eq!(second_find, third_find);
        }
    }

    /// Manually test transitivity
    #[test]
    fn test_transitivity_manual() {
        let mut uf = UnionFind::new(5);
        let _ = uf.union(0, 1);
        let _ = uf.union(1, 2);
        let _ = uf.union(3, 4);

        // Test transitivity: if 0-1 and 1-2, then 0-2
        assert!(uf.connected(0, 1).unwrap());
        assert!(uf.connected(1, 2).unwrap());
        assert!(uf.connected(0, 2).unwrap()); // Should be transitive

        // Test non-connected elements
        assert!(!uf.connected(0, 3).unwrap());
        assert!(!uf.connected(2, 4).unwrap());
    }

    /// Manually test count property
    #[test]
    fn test_count_property_manual() {
        let mut uf = UnionFind::new(3);
        let initial_count = uf.count();
        assert_eq!(initial_count, 3);

        // Union different elements - count should decrease
        let were_connected = uf.connected(0, 1).unwrap();
        assert!(!were_connected);

        let _ = uf.union(0, 1);
        assert_eq!(uf.count(), initial_count - 1);

        // Union same component - count should stay same
        let count_before = uf.count();
        let _ = uf.union(0, 1); // Same elements, already connected
        assert_eq!(uf.count(), count_before);
    }

    /// Test reflexivity manually
    #[test]
    fn test_reflexivity_manual() {
        let mut uf = UnionFind::new(5);

        // Every element should be connected to itself
        for x in 0..5 {
            assert!(uf.connected(x, x).unwrap());
        }

        // Even after unions, reflexivity should hold
        let _ = uf.union(0, 2);
        let _ = uf.union(1, 3);

        for x in 0..5 {
            assert!(uf.connected(x, x).unwrap());
        }
    }

    /// Test that component sizes are consistent
    #[test]
    fn test_component_size_consistency_manual() {
        let mut uf = UnionFind::new(5);
        let _ = uf.union(0, 1);
        let _ = uf.union(1, 2);

        // Component containing 0, 1, 2 should have size 3
        assert_eq!(uf.size(0).unwrap(), 3);
        assert_eq!(uf.size(1).unwrap(), 3);
        assert_eq!(uf.size(2).unwrap(), 3);

        // Components containing 3, 4 should have size 1
        assert_eq!(uf.size(3).unwrap(), 1);
        assert_eq!(uf.size(4).unwrap(), 1);
    }
}
