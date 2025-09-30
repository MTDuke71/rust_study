// Alternative Queue Tests from Original Specification
// This matches the test cases described in the README.md

use mission2::queue_alt::{RingBufferQueue, LinkedQueue};
use std::collections::VecDeque;

#[test] // REQ-R1, REQ-G2, REQ-G3, REQ-G4
fn ring_basic_wrap_and_full() {
    let mut q = RingBufferQueue::with_capacity(3);
    assert!(q.enqueue(1).is_ok());
    assert!(q.enqueue(2).is_ok());
    assert!(q.enqueue(3).is_ok()); // should be ok with capacity 3
    assert!(q.is_full());
    assert!(q.enqueue(4).is_err()); // now it's full
    assert_eq!(q.dequeue(), Some(1));
    assert!(q.enqueue(4).is_ok());  // wrap
    assert_eq!(q.peek(), Some(&2));
    assert_eq!(q.dequeue(), Some(2));
    assert_eq!(q.dequeue(), Some(3));
    assert_eq!(q.dequeue(), Some(4));
    assert!(q.dequeue().is_none());
    assert!(q.is_empty());
}

#[test] // REQ-G2 vs reference (VecDeque)
fn ring_randomized_matches_vecdeque() {
    let mut q = RingBufferQueue::with_capacity(8);
    let mut v = VecDeque::new();
    
    // simple pseudo-random pattern - fix overflow
    for i in 0u32..1_000 {
        let pseudo_random = i.wrapping_mul(2654435761u32) % 7;
        if pseudo_random != 0 {
            // enqueue
            let x = (i as i32) % 100 - 50;
            if q.enqueue(x).is_ok() {
                if v.len() < q.capacity() { 
                    v.push_back(x); 
                }
            }
        } else {
            // dequeue
            assert_eq!(q.dequeue(), v.pop_front());
        }
    }
}

#[test] // REQ-L2, REQ-L3
fn linked_basic_fifo() {
    let mut q = LinkedQueue::new();
    assert!(q.is_empty());
    q.enqueue("a");
    q.enqueue("b");
    q.enqueue("c");
    assert_eq!(q.len(), 3);
    assert_eq!(q.peek(), Some(&"a"));
    assert_eq!(q.dequeue(), Some("a"));
    assert_eq!(q.dequeue(), Some("b"));
    assert_eq!(q.dequeue(), Some("c"));
    assert!(q.dequeue().is_none());
    assert!(q.is_empty());
}

#[test] // REQ-G2 vs reference for linked
fn linked_matches_vecdeque() {
    let mut q = LinkedQueue::new();
    let mut v = VecDeque::new();
    
    for i in 0..5000 {
        if (i & 3) != 0 {
            q.enqueue(i);
            v.push_back(i);
        } else {
            assert_eq!(q.dequeue(), v.pop_front());
        }
    }
    
    while let Some(x) = v.pop_front() {
        assert_eq!(q.dequeue(), Some(x));
    }
}

#[test] // REQ-G3: no panics on edge cases
fn edges_empty_and_singleton() {
    let mut r = RingBufferQueue::with_capacity(1);
    assert!(r.dequeue().is_none());
    assert!(r.enqueue(1).is_ok());
    assert!(r.enqueue(2).is_err());
    assert_eq!(r.dequeue(), Some(1));
    assert!(r.dequeue().is_none());

    let mut l: LinkedQueue<i32> = LinkedQueue::new();
    assert!(l.dequeue().is_none());
    l.enqueue(10);
    assert_eq!(l.dequeue(), Some(10));
    assert!(l.dequeue().is_none());
}

#[test] // Additional comprehensive test
fn comprehensive_fifo_behavior() {
    // Test ring buffer
    let mut ring = RingBufferQueue::with_capacity(4);
    
    // Test interleaved operations
    for i in 0..20 {
        if i % 3 == 0 && !ring.is_full() {
            assert!(ring.enqueue(i).is_ok());
        } else if !ring.is_empty() {
            ring.dequeue();
        }
    }
    
    // Test linked queue unlimited growth
    let mut linked = LinkedQueue::new();
    
    // Enqueue many items
    for i in 0..1000 {
        linked.enqueue(i);
    }
    assert_eq!(linked.len(), 1000);
    
    // Verify FIFO order
    for i in 0..1000 {
        assert_eq!(linked.dequeue(), Some(i));
    }
    assert!(linked.is_empty());
}

#[test] // Performance characteristic validation
fn performance_characteristics() {
    // Ring buffer: O(1) operations with fixed capacity
    let mut ring = RingBufferQueue::with_capacity(1000);
    
    // Fill completely
    for i in 0..1000 {
        assert!(ring.enqueue(i).is_ok());
    }
    assert!(ring.is_full());
    
    // Empty completely
    for i in 0..1000 {
        assert_eq!(ring.dequeue(), Some(i));
    }
    assert!(ring.is_empty());
    
    // Linked queue: O(1) operations with dynamic growth
    let mut linked = LinkedQueue::new();
    
    // Test that it can grow beyond any fixed size
    for i in 0..10000 {
        linked.enqueue(i);
    }
    assert_eq!(linked.len(), 10000);
    
    // Verify order maintained
    for i in 0..10000 {
        assert_eq!(linked.dequeue(), Some(i));
    }
    assert!(linked.is_empty());
}

#[test] // Stress test: alternating patterns
fn stress_alternating_patterns() {
    let mut ring = RingBufferQueue::with_capacity(5);
    let mut linked = LinkedQueue::new();
    let mut ring_values = Vec::new();
    let mut linked_values = Vec::new();
    
    // Complex pattern of operations
    for i in 0..100 {
        // Try to enqueue
        if i % 4 != 3 {
            if ring.enqueue(i).is_ok() {
                ring_values.push(i);
            }
            linked.enqueue(i);
            linked_values.push(i);
        }
        
        // Try to dequeue occasionally
        if i % 7 == 0 {
            if let Some(val) = ring.dequeue() {
                assert_eq!(Some(val), ring_values.first().copied());
                if !ring_values.is_empty() {
                    ring_values.remove(0);
                }
            }
            
            if let Some(val) = linked.dequeue() {
                assert_eq!(Some(val), linked_values.first().copied());
                if !linked_values.is_empty() {
                    linked_values.remove(0);
                }
            }
        }
    }
    
    // Clean up - both should maintain FIFO order
    while let Some(val) = ring.dequeue() {
        assert_eq!(Some(val), ring_values.first().copied());
        if !ring_values.is_empty() {
            ring_values.remove(0);
        }
    }
    
    while let Some(val) = linked.dequeue() {
        assert_eq!(Some(val), linked_values.first().copied());
        if !linked_values.is_empty() {
            linked_values.remove(0);
        }
    }
    
    assert!(ring_values.is_empty());
    assert!(linked_values.is_empty());
    assert!(ring.is_empty());
    assert!(linked.is_empty());
}

#[test] // Memory safety with different types
fn memory_safety_different_types() {
    // Test with String (heap-allocated)
    let mut string_queue = LinkedQueue::new();
    string_queue.enqueue("Hello".to_string());
    string_queue.enqueue("World".to_string());
    
    assert_eq!(string_queue.dequeue(), Some("Hello".to_string()));
    assert_eq!(string_queue.dequeue(), Some("World".to_string()));
    
    // Test with Vec (owned data)
    let mut vec_queue = RingBufferQueue::with_capacity(3);
    assert!(vec_queue.enqueue(vec![1, 2, 3]).is_ok());
    assert!(vec_queue.enqueue(vec![4, 5, 6]).is_ok());
    
    assert_eq!(vec_queue.dequeue(), Some(vec![1, 2, 3]));
    assert_eq!(vec_queue.dequeue(), Some(vec![4, 5, 6]));
}

#[test] // Deterministic behavior
fn determinism_test() {
    for _ in 0..10 {
        let mut ring = RingBufferQueue::with_capacity(4);
        let mut linked = LinkedQueue::new();
        
        // Same sequence of operations should produce same results
        let sequence = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        
        for &val in &sequence {
            // Ring buffer might reject due to capacity
            let ring_result = ring.enqueue(val);
            
            // Linked queue always accepts
            linked.enqueue(val);
            
            // Both should have deterministic behavior
            if ring_result.is_ok() {
                assert_eq!(ring.peek().is_some(), true);
            }
            assert_eq!(linked.peek().is_some(), true);
        }
        
        // Dequeue order should be deterministic
        let mut ring_order = Vec::new();
        let mut linked_order = Vec::new();
        
        while let Some(val) = ring.dequeue() {
            ring_order.push(val);
        }
        
        while let Some(val) = linked.dequeue() {
            linked_order.push(val);
        }
        
        // Both should maintain FIFO order for what they accepted
        for window in ring_order.windows(2) {
            assert!(window[0] < window[1]);
        }
        
        for window in linked_order.windows(2) {
            assert!(window[0] < window[1]);
        }
        
        // Linked queue should have all values
        assert_eq!(linked_order, sequence.to_vec());
    }
}