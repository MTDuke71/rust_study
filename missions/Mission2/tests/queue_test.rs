use mission2::queue::{RingBufferQueue, LinkedQueue};
use std::collections::VecDeque;

/// REQ-R1, REQ-G2, REQ-G3, REQ-G4: Ring buffer wrap-around and capacity tests
#[test]
fn ring_basic_wrap_and_full() {
    let mut q = RingBufferQueue::with_capacity(3);
    
    // Test basic enqueue/dequeue
    assert!(q.enqueue(1).is_ok());
    assert!(q.enqueue(2).is_ok());
    assert_eq!(q.len(), 2);
    assert!(!q.is_full());
    
    // Fill to capacity
    assert!(q.enqueue(3).is_ok());
    assert!(q.is_full());
    assert_eq!(q.len(), 3);
    
    // Test full queue behavior
    assert_eq!(q.enqueue(4), Err(4)); // Should return the value back
    assert_eq!(q.len(), 3); // Length unchanged
    
    // Test FIFO order and wrap-around
    assert_eq!(q.dequeue(), Some(1)); // FIFO: first in, first out
    assert!(!q.is_full());
    assert!(q.enqueue(4).is_ok()); // Now there's space (wrap-around)
    
    assert_eq!(q.peek(), Some(&2));   // Should see next element
    assert_eq!(q.dequeue(), Some(2)); // Continue FIFO
    assert_eq!(q.dequeue(), Some(3));
    assert_eq!(q.dequeue(), Some(4)); // The wrapped element
    assert!(q.dequeue().is_none());   // Empty
    assert!(q.is_empty());
}

/// REQ-G2: Ring buffer behavior matches reference implementation (VecDeque)
#[test]
fn ring_randomized_matches_vecdeque() {
    let mut q = RingBufferQueue::with_capacity(8);
    let mut v = VecDeque::new();
    
    // Pseudo-random pattern for comprehensive testing
    for i in 0..10_000u32 {
        if (i.wrapping_mul(2654435761u32)) % 7 != 0 {
            // Enqueue operation (~85% of operations)
            let x = (i as i32) % 100 - 50; // Values from -50 to 49
            
            let ring_result = q.enqueue(x);
            if v.len() < q.capacity() {
                // VecDeque has space, ring buffer should too
                assert!(ring_result.is_ok(), "Ring buffer should accept value when VecDeque has space");
                v.push_back(x);
            } else {
                // Both should be "full" (VecDeque simulating capacity limit)
                assert_eq!(ring_result, Err(x), "Ring buffer should reject when at capacity");
            }
        } else {
            // Dequeue operation (~15% of operations)
            let ring_result = q.dequeue();
            let vec_result = v.pop_front();
            assert_eq!(ring_result, vec_result, "Ring buffer and VecDeque should match");
        }
        
        // Invariant: lengths should always match
        assert_eq!(q.len(), v.len(), "Lengths should always match");
        
        // Peek should also match
        assert_eq!(q.peek(), v.front(), "Peek should match VecDeque front");
    }
    
    // Final drain - everything should match
    while let Some(expected) = v.pop_front() {
        assert_eq!(q.dequeue(), Some(expected), "Final drain should match");
    }
    assert!(q.is_empty());
}

/// REQ-L2, REQ-L3: Linked queue basic FIFO operations
#[test]
fn linked_basic_fifo() {
    let mut q = LinkedQueue::new();
    assert!(q.is_empty());
    assert_eq!(q.len(), 0);
    
    // Test single element
    q.enqueue("a");
    assert_eq!(q.len(), 1);
    assert!(!q.is_empty());
    assert_eq!(q.peek(), Some(&"a"));
    
    // Test multiple elements
    q.enqueue("b");
    q.enqueue("c");
    assert_eq!(q.len(), 3);
    assert_eq!(q.peek(), Some(&"a")); // Should still be first
    
    // Test FIFO dequeue order
    assert_eq!(q.dequeue(), Some("a"));
    assert_eq!(q.dequeue(), Some("b"));
    assert_eq!(q.dequeue(), Some("c"));
    assert!(q.dequeue().is_none());
    assert!(q.is_empty());
    assert_eq!(q.len(), 0);
}

/// REQ-G2: Linked queue behavior matches reference implementation
#[test]
fn linked_matches_vecdeque() {
    let mut q = LinkedQueue::new();
    let mut v = VecDeque::new();
    
    for i in 0..5000 {
        if (i & 3) != 0 {
            // Enqueue operation (~75% of operations)
            q.enqueue(i);
            v.push_back(i);
        } else {
            // Dequeue operation (~25% of operations)
            assert_eq!(q.dequeue(), v.pop_front());
        }
        
        // Invariants
        assert_eq!(q.len(), v.len());
        assert_eq!(q.is_empty(), v.is_empty());
        assert_eq!(q.peek(), v.front());
    }
    
    // Final drain
    while let Some(x) = v.pop_front() {
        assert_eq!(q.dequeue(), Some(x));
    }
    assert!(q.is_empty());
}

/// REQ-G3: No panics on edge cases (empty operations, single element, etc.)
#[test]
fn edges_empty_and_singleton() {
    // Ring buffer edge cases
    let mut r = RingBufferQueue::with_capacity(1);
    
    // Empty operations
    assert!(r.dequeue().is_none());
    assert!(r.peek().is_none());
    assert!(r.is_empty());
    assert!(!r.is_full());
    
    // Single element capacity
    assert!(r.enqueue(1).is_ok());
    assert!(r.is_full());
    assert_eq!(r.len(), 1);
    assert_eq!(r.peek(), Some(&1));
    
    // Full queue operations
    assert_eq!(r.enqueue(2), Err(2)); // Should return value
    assert_eq!(r.len(), 1); // Unchanged
    
    // Dequeue and re-enqueue
    assert_eq!(r.dequeue(), Some(1));
    assert!(r.is_empty());
    assert!(r.enqueue(3).is_ok());
    assert_eq!(r.dequeue(), Some(3));
    assert!(r.dequeue().is_none());
    
    // Linked queue edge cases
    let mut l: LinkedQueue<i32> = LinkedQueue::new();
    
    // Empty operations
    assert!(l.dequeue().is_none());
    assert!(l.peek().is_none());
    assert!(l.is_empty());
    
    // Single element
    l.enqueue(10);
    assert_eq!(l.len(), 1);
    assert!(!l.is_empty());
    assert_eq!(l.peek(), Some(&10));
    assert_eq!(l.dequeue(), Some(10));
    assert!(l.is_empty());
    assert!(l.dequeue().is_none());
}

/// REQ-R2: Test wrap-around behavior extensively
#[test]
fn ring_extensive_wraparound() {
    let mut q = RingBufferQueue::with_capacity(4);
    
    // Fill queue
    for i in 0..4 {
        assert!(q.enqueue(i).is_ok());
    }
    assert!(q.is_full());
    
    // Test wraparound by partially draining and refilling
    assert_eq!(q.dequeue(), Some(0));
    assert_eq!(q.dequeue(), Some(1));
    assert_eq!(q.len(), 2);
    
    // Refill (this should wrap around)
    assert!(q.enqueue(4).is_ok());
    assert!(q.enqueue(5).is_ok());
    assert!(q.is_full());
    
    // Verify order is maintained
    assert_eq!(q.dequeue(), Some(2)); // Original element
    assert_eq!(q.dequeue(), Some(3)); // Original element
    assert_eq!(q.dequeue(), Some(4)); // Wrapped element
    assert_eq!(q.dequeue(), Some(5)); // Wrapped element
    assert!(q.is_empty());
    
    // Test multiple complete cycles
    for cycle in 0..3 {
        for i in 0..4 {
            assert!(q.enqueue(cycle * 4 + i).is_ok());
        }
        assert!(q.is_full());
        
        for i in 0..4 {
            assert_eq!(q.dequeue(), Some(cycle * 4 + i));
        }
        assert!(q.is_empty());
    }
}

/// REQ-L1, REQ-L2: Test linked queue with large number of elements
#[test]
fn linked_large_sequence() {
    let mut q = LinkedQueue::new();
    
    // Test growing to large size
    let n = 10_000;
    for i in 0..n {
        q.enqueue(i);
        assert_eq!(q.len(), i + 1);
    }
    
    // Test partial drain
    for i in 0..n/2 {
        assert_eq!(q.dequeue(), Some(i));
        assert_eq!(q.len(), n - i - 1);
    }
    
    // Test mixed operations (grow while draining)
    for i in n..n + n/4 {
        q.enqueue(i);
    }
    
    // Verify remaining order
    for i in n/2..n {
        assert_eq!(q.dequeue(), Some(i));
    }
    for i in n..n + n/4 {
        assert_eq!(q.dequeue(), Some(i));
    }
    assert!(q.is_empty());
}

/// REQ-G4: Performance characteristics test (basic timing)
#[test]
fn performance_characteristics() {
    use std::time::Instant;
    
    const N: usize = 100_000;
    
    // Ring buffer performance
    let start = Instant::now();
    let mut ring = RingBufferQueue::with_capacity(1000);
    for i in 0..N {
        if ring.enqueue(i).is_err() {
            // Drain some elements to make space
            for _ in 0..500 {
                ring.dequeue();
            }
            ring.enqueue(i).unwrap();
        }
    }
    let ring_time = start.elapsed();
    
    // Linked queue performance  
    let start = Instant::now();
    let mut linked = LinkedQueue::new();
    for i in 0..N {
        linked.enqueue(i);
    }
    let linked_time = start.elapsed();
    
    // Both should complete in reasonable time (this is not a precise benchmark)
    assert!(ring_time.as_millis() < 1000, "Ring buffer should be fast");
    assert!(linked_time.as_millis() < 1000, "Linked queue should be fast");
    
    // Cleanup
    while ring.dequeue().is_some() {}
    while linked.dequeue().is_some() {}
}

/// REQ-G5: Determinism test - same operations should produce same results
#[test]
fn determinism_test() {
    // Run the same sequence of operations multiple times
    for run in 0..3 {
        let mut ring = RingBufferQueue::with_capacity(5);
        let mut linked = LinkedQueue::new();
        let mut expected = Vec::new();
        
        // Predefined sequence of operations
        let operations = [
            (true, 1), (true, 2), (false, 0), (true, 3), (true, 4),
            (false, 0), (false, 0), (true, 5), (true, 6), (false, 0),
        ];
        
        for &(is_enqueue, value) in &operations {
            if is_enqueue {
                let _ = ring.enqueue(value);
                linked.enqueue(value);
                expected.push(value);
            } else {
                let ring_result = ring.dequeue();
                let linked_result = linked.dequeue();
                let expected_result = if expected.is_empty() { 
                    None 
                } else { 
                    Some(expected.remove(0)) 
                };
                
                assert_eq!(ring_result, expected_result, "Run {}: Ring buffer determinism", run);
                assert_eq!(linked_result, expected_result, "Run {}: Linked queue determinism", run);
            }
        }
    }
}

/// REQ-G3: Memory safety test with different types
#[test]
fn memory_safety_different_types() {
    // Test with String (heap-allocated)
    let mut ring_str = RingBufferQueue::with_capacity(3);
    let mut linked_str = LinkedQueue::new();
    
    let strings = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];
    
    for s in strings.clone() {
        ring_str.enqueue(s.clone()).ok();
        linked_str.enqueue(s);
    }
    
    for expected in &strings {
        assert_eq!(ring_str.dequeue().as_ref(), Some(expected));
        assert_eq!(linked_str.dequeue().as_ref(), Some(expected));
    }
    
    // Test with Vec (heap-allocated)
    let mut ring_vec = RingBufferQueue::with_capacity(2);
    let mut linked_vec = LinkedQueue::new();
    
    let vecs = vec![vec![1, 2, 3], vec![4, 5, 6]];
    
    for v in vecs.clone() {
        ring_vec.enqueue(v.clone()).ok();
        linked_vec.enqueue(v);
    }
    
    for expected in &vecs {
        assert_eq!(ring_vec.dequeue().as_ref(), Some(expected));
        assert_eq!(linked_vec.dequeue().as_ref(), Some(expected));
    }
}

/// Stress test: alternating enqueue/dequeue patterns
#[test]
fn stress_alternating_patterns() {
    // Test that both implementations handle alternating patterns without panics
    // This test acknowledges that ring buffer and linked queue have different behaviors
    
    let mut ring = RingBufferQueue::with_capacity(5);
    let mut linked = LinkedQueue::new();
    
    // Test 1: Simple alternating pattern with capacity management
    for i in 0..50 {
        // Ensure ring buffer has space
        if ring.len() >= ring.capacity() {
            let ring_val = ring.dequeue();
            let linked_val = linked.dequeue();
            // If ring had something, linked should too (FIFO consistency)
            if ring_val.is_some() {
                assert!(linked_val.is_some());
            }
        }
        
        // Now enqueue should succeed
        assert!(ring.enqueue(i).is_ok());
        linked.enqueue(i);
        
        // Occasionally dequeue
        if i % 3 == 2 {
            let ring_val = ring.dequeue();
            let linked_val = linked.dequeue();
            assert_eq!(ring_val, linked_val);
        }
    }
    
    // Test 2: Ring buffer capacity behavior
    let mut ring_capacity_test = RingBufferQueue::with_capacity(3);
    
    // Fill to capacity
    for i in 0..3 {
        assert!(ring_capacity_test.enqueue(i).is_ok());
    }
    assert!(ring_capacity_test.is_full());
    
    // Test overflow
    assert_eq!(ring_capacity_test.enqueue(99), Err(99));
    
    // Make space and verify wrap-around
    assert_eq!(ring_capacity_test.dequeue(), Some(0));
    assert!(ring_capacity_test.enqueue(99).is_ok());
    
    // Test 3: Linked queue unlimited growth
    let mut linked_growth_test = LinkedQueue::new();
    
    for i in 0..1000 {
        linked_growth_test.enqueue(i);
    }
    assert_eq!(linked_growth_test.len(), 1000);
    
    // Verify FIFO order
    for i in 0..1000 {
        assert_eq!(linked_growth_test.dequeue(), Some(i));
    }
    assert!(linked_growth_test.is_empty());
    
    // Test 4: Mixed operations with proper synchronization
    let mut ring_sync = RingBufferQueue::with_capacity(10);
    let mut linked_sync = LinkedQueue::new();
    
    for cycle in 0..20 {
        // Enqueue phase - respect capacity
        let enqueue_count = std::cmp::min(5, ring_sync.capacity() - ring_sync.len());
        for i in 0..enqueue_count {
            let val = cycle * 10 + i;
            assert!(ring_sync.enqueue(val).is_ok());
            linked_sync.enqueue(val);
        }
        
        // Dequeue phase - remove half
        let dequeue_count = ring_sync.len() / 2;
        for _ in 0..dequeue_count {
            let ring_val = ring_sync.dequeue();
            let linked_val = linked_sync.dequeue();
            assert_eq!(ring_val, linked_val);
        }
    }
    
    // Final cleanup
    while let Some(ring_val) = ring.dequeue() {
        let linked_val = linked.dequeue();
        assert_eq!(Some(ring_val), linked_val);
    }
    while let Some(_) = linked.dequeue() {
        // Linked might have more due to capacity constraints during test
    }
    
    // Cleanup other queues
    while ring_capacity_test.dequeue().is_some() {}
    while ring_sync.dequeue().is_some() {}
    while linked_sync.dequeue().is_some() {}
    
    assert!(ring.is_empty());
    assert!(linked.is_empty());
    assert!(ring_sync.is_empty());
    assert!(linked_sync.is_empty());
}