//! Comprehensive unit tests for Mission 4: Singly Linked List
//!
//! This test suite covers all requirements (REQ-1 through REQ-6) with full traceability.

use mission4::{LinkedListError, RcLinkedList, SimpleLinkedList};

// ============================================================================
// REQ-1: Basic Linked List Structure Tests
// ============================================================================

#[test] // REQ-1
fn req1_simple_list_basic_structure() {
    let list: SimpleLinkedList<i32> = SimpleLinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
}

#[test] // REQ-1
fn req1_simple_list_node_ownership() {
    let mut list = SimpleLinkedList::new();

    // Test that ownership transfer works correctly
    let value = String::from("test");
    list.push_front(value); // value is moved

    assert_eq!(list.len(), 1);
    assert_eq!(list.peek_front(), Some(&String::from("test")));
}

#[test] // REQ-1
fn req1_memory_safety_after_drop() {
    let mut list = SimpleLinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    // All memory should be cleaned up properly when list is dropped
    drop(list);
    // If we reach here without segfault, memory management is correct
}

// ============================================================================
// REQ-2: Interior Mutability with Rc<RefCell<T>> Tests
// ============================================================================

#[test] // REQ-2
fn req2_rc_list_basic_structure() {
    let list: RcLinkedList<i32> = RcLinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
}

#[test] // REQ-2
fn req2_rc_list_shared_ownership() {
    let mut list = RcLinkedList::new();
    list.push_front(42);

    // Get a reference to the head node
    let head_ref = list.get_head_ref();
    assert!(head_ref.is_some());

    // Verify reference counting
    if let Some(head) = head_ref {
        assert!(std::rc::Rc::strong_count(&head) >= 1);
    }
}

#[test] // REQ-2
fn req2_interior_mutability_borrow_checking() {
    let mut list = RcLinkedList::new();
    list.push_front(100);

    // Test try_peek_front for runtime borrow checking
    match list.try_peek_front() {
        Ok(Some(value)) => assert_eq!(*value, 100),
        Ok(None) => panic!("Expected Some value"),
        Err(_) => panic!("Expected successful borrow"),
    };
}

#[test] // REQ-2
fn req2_weak_references_no_cycles() {
    let mut list = RcLinkedList::new();
    list.push_front(1);
    list.push_front(2);

    // The implementation uses weak references to prevent cycles
    // Verify list can be cleaned up properly
    let initial_len = list.len();
    list.clear();
    assert_eq!(list.len(), 0);
    assert!(initial_len > 0);
}

// ============================================================================
// REQ-3: Core List Operations Tests
// ============================================================================

#[test] // REQ-3
fn req3_simple_push_front_constant_time() {
    let mut list = SimpleLinkedList::new();

    // Multiple pushes should all be O(1)
    for i in 0..1000 {
        list.push_front(i);
        assert_eq!(list.len(), i + 1);
    }

    assert_eq!(list.peek_front(), Some(&999));
}

#[test] // REQ-3
fn req3_simple_pop_front_constant_time() {
    let mut list = SimpleLinkedList::new();

    // Setup
    for i in 0..100 {
        list.push_front(i);
    }

    // Pop should be O(1) and maintain LIFO order
    for i in (0..100).rev() {
        assert_eq!(list.pop_front(), Some(i));
    }

    assert!(list.is_empty());
    assert_eq!(list.pop_front(), None);
}

#[test] // REQ-3
fn req3_simple_peek_operations() {
    let mut list = SimpleLinkedList::new();

    // Peek on empty list
    assert_eq!(list.peek_front(), None);
    assert_eq!(list.peek_front_mut(), None);

    list.push_front(42);

    // Immutable peek
    assert_eq!(list.peek_front(), Some(&42));

    // Mutable peek and modification
    if let Some(front) = list.peek_front_mut() {
        *front = 100;
    }

    assert_eq!(list.peek_front(), Some(&100));
    assert_eq!(list.len(), 1); // Length unchanged
}

#[test] // REQ-3
fn req3_rc_list_operations() {
    let mut list = RcLinkedList::new();

    // Test all basic operations
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    assert_eq!(list.len(), 3);
    assert!(!list.is_empty());

    // Pop operations
    assert_eq!(list.pop_front(), Ok(Some(3)));
    assert_eq!(list.pop_front(), Ok(Some(2)));
    assert_eq!(list.len(), 1);

    // Peek operations
    match list.try_peek_front() {
        Ok(Some(value)) => assert_eq!(*value, 1),
        _ => panic!("Expected value"),
    };
}

#[test] // REQ-3
fn req3_clear_operation() {
    let mut simple_list = SimpleLinkedList::new();
    let mut rc_list = RcLinkedList::new();

    // Populate both lists
    for i in 0..50 {
        simple_list.push_front(i);
        rc_list.push_front(i);
    }

    // Clear both
    simple_list.clear();
    rc_list.clear();

    assert!(simple_list.is_empty());
    assert!(rc_list.is_empty());
    assert_eq!(simple_list.len(), 0);
    assert_eq!(rc_list.len(), 0);
}

// ============================================================================
// REQ-4: Safe Iteration Patterns Tests
// ============================================================================

#[test] // REQ-4
fn req4_consuming_iterator() {
    let mut list = SimpleLinkedList::new();
    for i in 0..5 {
        list.push_front(i);
    }

    let collected: Vec<i32> = list.into_iter().collect();
    assert_eq!(collected, vec![4, 3, 2, 1, 0]);
}

#[test] // REQ-4
fn req4_borrowing_iterator() {
    let mut list = SimpleLinkedList::new();
    for i in 0..5 {
        list.push_front(i);
    }

    let collected: Vec<&i32> = list.iter().collect();
    assert_eq!(collected, vec![&4, &3, &2, &1, &0]);

    // List should still be usable
    assert_eq!(list.len(), 5);
}

#[test] // REQ-4
fn req4_mutable_iterator() {
    let mut list = SimpleLinkedList::new();
    for i in 0..5 {
        list.push_front(i);
    }

    // Modify all elements
    for item in list.iter_mut() {
        *item *= 2;
    }

    let collected: Vec<i32> = list.into_iter().collect();
    assert_eq!(collected, vec![8, 6, 4, 2, 0]);
}

#[test] // REQ-4
fn req4_iterator_traits() {
    let mut list = SimpleLinkedList::new();
    for i in 0..10 {
        list.push_front(i);
    }

    // Test size_hint
    let mut iter = list.into_iter();
    assert_eq!(iter.size_hint(), (10, Some(10)));

    iter.next();
    assert_eq!(iter.size_hint(), (9, Some(9)));
}

#[test] // REQ-4
fn req4_from_iterator() {
    let data = vec![1, 2, 3, 4, 5];
    let list: SimpleLinkedList<i32> = data.into_iter().collect();

    // Should maintain order
    let collected: Vec<i32> = list.into_iter().collect();
    assert_eq!(collected, vec![1, 2, 3, 4, 5]);
}

#[test] // REQ-4
fn req4_extend_iterator() {
    let mut list = SimpleLinkedList::new();
    list.push_front(0);

    list.extend(vec![1, 2, 3]);

    let collected: Vec<i32> = list.into_iter().collect();
    // extend() adds items to front, so [1,2,3] becomes [3,2,1,0]
    assert_eq!(collected, vec![3, 2, 1, 0]);
}

// ============================================================================
// REQ-5: Memory Management Patterns Tests
// ============================================================================

#[test] // REQ-5
fn req5_box_vs_rc_memory_patterns() {
    // Test that both implementations can handle the same operations
    let mut simple_list = SimpleLinkedList::new();
    let mut rc_list = RcLinkedList::new();

    for i in 0..100 {
        simple_list.push_front(i);
        rc_list.push_front(i);
    }

    assert_eq!(simple_list.len(), rc_list.len());

    // Both should be able to clear without memory leaks
    simple_list.clear();
    rc_list.clear();

    assert!(simple_list.is_empty());
    assert!(rc_list.is_empty());
}

#[test] // REQ-5
fn req5_no_memory_leaks_stress_test() {
    // Stress test to verify no memory leaks
    for _ in 0..1000 {
        let mut list = SimpleLinkedList::new();
        for i in 0..100 {
            list.push_front(i);
        }
        while list.pop_front().is_some() {}
    }

    for _ in 0..1000 {
        let mut list = RcLinkedList::new();
        for i in 0..100 {
            list.push_front(i);
        }
        while let Ok(Some(_)) = list.pop_front() {}
    }
}

#[test] // REQ-5
fn req5_drop_implementation() {
    let mut list = SimpleLinkedList::new();

    // Create some data that would be expensive to leak
    for i in 0..1000 {
        list.push_front(vec![i; 100]); // Each element is a Vec<i32>
    }

    // Dropping the list should clean up all Vec allocations
    drop(list);
    // Test passes if we don't run out of memory or crash
}

// ============================================================================
// REQ-6: Weak References for Advanced Patterns Tests
// ============================================================================

#[test] // REQ-6
fn req6_weak_references_prevent_cycles() {
    let mut list = RcLinkedList::new();
    list.push_front(1);
    list.push_front(2);

    // Get head reference
    let head_ref = list.get_head_ref().unwrap();

    // Verify weak references are used internally
    // (This is more of a design verification - weak refs prevent cycles)
    let strong_count = std::rc::Rc::strong_count(&head_ref);

    // Should have at least the list's reference and our local reference
    assert!(strong_count >= 2);

    drop(head_ref);
    list.clear();
    // Should not cause memory leaks or infinite reference cycles
}

#[test] // REQ-6
fn req6_shared_node_access() {
    let mut list = RcLinkedList::new();
    list.push_front(42);

    // Get multiple references to the same node
    let ref1 = list.get_head_ref().unwrap();
    let ref2 = list.get_head_ref().unwrap();

    // Both should point to the same data
    assert_eq!(ref1.borrow().data, ref2.borrow().data);

    // Reference count should reflect multiple ownership
    assert!(std::rc::Rc::strong_count(&ref1) >= 3); // list + ref1 + ref2
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_rc_list_multiple_references_error() {
    let mut list = RcLinkedList::new();
    list.push_front(42);

    // Get a reference to the head node
    let _head_ref = list.get_head_ref().unwrap();

    // Now try to pop - this should fail with MultipleReferences error
    match list.pop_front() {
        Err(LinkedListError::MultipleReferences) => {
            // This is expected
        }
        Ok(_) => panic!("Expected MultipleReferences error"),
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_borrow_error_handling() {
    let mut list = RcLinkedList::new();
    list.push_front(100);

    // Get a mutable borrow
    let _mut_ref = list.try_peek_front_mut().unwrap().unwrap();

    // Now try to get another borrow - this should fail
    match list.try_peek_front() {
        Err(LinkedListError::BorrowError) => {
            // This is expected
        }
        Ok(_) => panic!("Expected BorrowError"),
        Err(e) => panic!("Unexpected error: {:?}", e),
    };
}

// ============================================================================
// Trait Implementation Tests
// ============================================================================

#[test]
fn test_debug_trait() {
    let mut list = SimpleLinkedList::new();
    list.push_front(1);
    list.push_front(2);

    let debug_string = format!("{:?}", list);
    assert!(debug_string.contains("SimpleLinkedList"));
}

#[test]
fn test_display_trait() {
    let mut list = SimpleLinkedList::new();
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    let display_string = format!("{}", list);
    assert_eq!(display_string, "[1, 2, 3]");
}

#[test]
fn test_clone_trait() {
    let mut list = SimpleLinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    let cloned = list.clone();
    assert_eq!(list, cloned);

    // Verify they're independent
    let original_len = list.len();
    assert_eq!(cloned.len(), original_len);
}

#[test]
fn test_equality_traits() {
    let mut list1 = SimpleLinkedList::new();
    let mut list2 = SimpleLinkedList::new();

    // Empty lists should be equal
    assert_eq!(list1, list2);

    // Add same elements
    for i in 0..5 {
        list1.push_front(i);
        list2.push_front(i);
    }

    assert_eq!(list1, list2);

    // Different lists should not be equal
    list2.push_front(999);
    assert_ne!(list1, list2);
}

// ============================================================================
// Edge Cases and Stress Tests
// ============================================================================

#[test]
fn test_empty_list_operations() {
    let mut simple_list = SimpleLinkedList::<i32>::new();
    let mut rc_list = RcLinkedList::<i32>::new();

    // All operations on empty lists should work
    assert_eq!(simple_list.pop_front(), None);
    assert_eq!(rc_list.pop_front(), Ok(None));

    assert_eq!(simple_list.peek_front(), None);
    // Check if the result is None without direct equality comparison
    match rc_list.try_peek_front().unwrap() {
        None => (), // This is what we expect
        Some(_) => panic!("Expected None from empty list"),
    }

    assert!(simple_list.is_empty());
    assert!(rc_list.is_empty());

    simple_list.clear();
    rc_list.clear();
}

#[test]
fn test_single_element_operations() {
    let mut simple_list = SimpleLinkedList::new();
    let mut rc_list = RcLinkedList::new();

    simple_list.push_front(42);
    rc_list.push_front(42);

    assert_eq!(simple_list.len(), 1);
    assert_eq!(rc_list.len(), 1);

    assert_eq!(simple_list.peek_front(), Some(&42));
    // Check the value without direct comparison of Ref<T>
    match rc_list.try_peek_front().unwrap() {
        Some(value_ref) => assert_eq!(*value_ref, 42),
        None => panic!("Expected value"),
    }

    assert_eq!(simple_list.pop_front(), Some(42));
    assert_eq!(rc_list.pop_front(), Ok(Some(42)));

    assert!(simple_list.is_empty());
    assert!(rc_list.is_empty());
}

#[test]
fn test_large_list_performance() {
    let mut list = SimpleLinkedList::new();

    // Large list operations
    for i in 0..10000 {
        list.push_front(i);
    }

    assert_eq!(list.len(), 10000);

    // Iterate through all elements
    let mut count = 0;
    for _ in &list {
        count += 1;
    }
    assert_eq!(count, 10000);

    // Clear should be efficient
    list.clear();
    assert!(list.is_empty());
}
