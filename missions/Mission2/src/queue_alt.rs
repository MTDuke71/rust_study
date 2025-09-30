//! # Alternative Queue Implementation from Original Specification
//!
//! This module provides the original queue implementations exactly as specified in the README.md
//! V-Cycle documentation. These implementations prioritize simplicity and educational clarity
//! over production features, making them ideal for learning core concepts.
//!
//! ## Design Philosophy
//!
//! The alternative implementations follow the principle of "minimum viable implementation" -
//! they contain exactly what's needed to satisfy the requirements without additional features.
//! This makes them excellent for:
//!
//! - Understanding fundamental queue concepts
//! - Learning basic Rust ownership patterns
//! - Prototyping and experimentation
//! - Educational purposes and teaching
//!
//! ## Key Differences from Main Implementation
//!
//! - Simpler API surface with fewer constructors
//! - Minimal error handling focused on core cases
//! - Direct unsafe usage for educational purposes
//! - Compact codebase prioritizing readability
//! - Basic documentation sufficient for understanding

use std::ptr::NonNull;

// ==========================
// Ring Buffer Queue (Alternative Implementation)
// ==========================

/// A fixed-capacity FIFO queue implemented using a circular buffer.
///
/// This implementation uses a `Vec<Option<T>>` to store elements with wrap-around indexing.
/// When the queue reaches capacity, further enqueue operations return an error containing
/// the value that couldn't be inserted.
///
/// # Design
///
/// The ring buffer maintains three key pieces of state:
/// - `head`: Index of the next element to dequeue
/// - `tail`: Index where the next element will be enqueued  
/// - `len`: Current number of elements (for O(1) length queries)
///
/// The buffer uses modular arithmetic for wrap-around: `(index + 1) % capacity`.
/// Elements are stored as `Option<T>` to allow safe removal using `.take()`.
///
/// # Performance Characteristics
///
/// - **enqueue**: O(1) - direct index assignment with modular arithmetic
/// - **dequeue**: O(1) - direct index access with `.take()`
/// - **peek**: O(1) - direct index access with `.as_ref()`
/// - **Space**: O(capacity) - fixed allocation, no reallocation after construction
///
/// # Memory Layout
///
/// ```text
/// Circular Buffer: [Some(A), None, None, Some(B)]
///                       ^              ^
///                     head           tail
/// ```
///
/// # Examples
///
/// ```rust
/// use mission2::queue_alt::RingBufferQueue;
///
/// let mut queue = RingBufferQueue::with_capacity(3);
///
/// // Enqueue elements
/// assert!(queue.enqueue("first").is_ok());
/// assert!(queue.enqueue("second").is_ok());
/// assert!(queue.enqueue("third").is_ok());
/// assert!(queue.is_full());
///
/// // Queue is full - enqueue returns error with value
/// assert_eq!(queue.enqueue("overflow"), Err("overflow"));
///
/// // Dequeue in FIFO order
/// assert_eq!(queue.dequeue(), Some("first"));
/// assert_eq!(queue.dequeue(), Some("second"));
///
/// // Now we can enqueue again (wrap-around)
/// assert!(queue.enqueue("fourth").is_ok());
/// assert_eq!(queue.dequeue(), Some("third"));
/// assert_eq!(queue.dequeue(), Some("fourth"));
/// assert!(queue.is_empty());
/// ```
///
/// # Use Cases
///
/// Ideal for scenarios where:
/// - Maximum queue size is known in advance
/// - Memory usage must be predictable and bounded
/// - Cache performance is critical (contiguous memory)
/// - Backpressure handling is needed (full queue returns error)
///
/// Common applications:
/// - BFS algorithms on bounded grids
/// - Producer-consumer systems with flow control
/// - Real-time systems requiring deterministic memory usage
/// - High-frequency trading systems needing minimal latency
#[derive(Debug)]
pub struct RingBufferQueue<T> {
    /// Fixed-size buffer storing elements as Option<T> for safe removal
    buf: Vec<Option<T>>,
    /// Index of the next element to dequeue (front of queue)
    head: usize,
    /// Index where the next element will be enqueued (back of queue) 
    tail: usize,
    /// Current number of elements in the queue
    len: usize,
}

impl<T> RingBufferQueue<T> {
    /// Creates a new ring buffer queue with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `cap` - The maximum number of elements the queue can hold. Must be greater than 0.
    ///
    /// # Panics
    ///
    /// Panics if `cap` is 0, as a zero-capacity queue is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission2::queue_alt::RingBufferQueue;
    ///
    /// let mut queue = RingBufferQueue::<i32>::with_capacity(5);
    /// assert_eq!(queue.capacity(), 5);
    /// assert!(queue.is_empty());
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(capacity) - must initialize the internal vector with None values.
    pub fn with_capacity(cap: usize) -> Self {
        assert!(cap > 0, "capacity must be > 0");
        Self {
            buf: (0..cap).map(|_| None).collect(),
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    /// Returns the current number of elements in the queue.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission2::queue_alt::RingBufferQueue;
    ///
    /// let mut queue = RingBufferQueue::with_capacity(5);
    /// assert_eq!(queue.len(), 0);
    ///
    /// queue.enqueue(42);
    /// assert_eq!(queue.len(), 1);
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(1) - length is maintained as queue operations occur.
    #[inline] 
    pub fn len(&self) -> usize { 
        self.len 
    }
    
    /// Returns `true` if the queue contains no elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission2::queue_alt::RingBufferQueue;
    ///
    /// let mut queue = RingBufferQueue::with_capacity(3);
    /// assert!(queue.is_empty());
    ///
    /// queue.enqueue("item");
    /// assert!(!queue.is_empty());
    ///
    /// queue.dequeue();
    /// assert!(queue.is_empty());
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(1) - simple length comparison.
    #[inline] 
    pub fn is_empty(&self) -> bool { 
        self.len == 0 
    }
    
    /// Returns the maximum number of elements the queue can hold.
    ///
    /// The capacity is fixed at construction time and never changes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission2::queue_alt::RingBufferQueue;
    ///
    /// let queue = RingBufferQueue::<i32>::with_capacity(100);
    /// assert_eq!(queue.capacity(), 100);
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(1) - returns the length of the internal buffer.
    #[inline] 
    pub fn capacity(&self) -> usize { 
        self.buf.len() 
    }
    
    /// Returns `true` if the queue has reached its maximum capacity.
    ///
    /// When full, `enqueue` operations will return `Err(value)` instead of adding the element.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission2::queue_alt::RingBufferQueue;
    ///
    /// let mut queue = RingBufferQueue::with_capacity(2);
    /// assert!(!queue.is_full());
    ///
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    /// assert!(queue.is_full());
    ///
    /// // Further enqueues will fail
    /// assert_eq!(queue.enqueue(3), Err(3));
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(1) - compares current length with capacity.
    #[inline] 
    pub fn is_full(&self) -> bool { 
        self.len == self.capacity() 
    }

    /// Adds an element to the back of the queue.
    ///
    /// If the queue is at capacity, returns `Err(x)` with the value that couldn't be inserted.
    /// This provides backpressure - the caller can decide how to handle a full queue.
    ///
    /// # Arguments
    ///
    /// * `x` - The element to add to the queue
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the element was successfully added
    /// * `Err(x)` if the queue was full, returning the original value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission2::queue_alt::RingBufferQueue;
    ///
    /// let mut queue = RingBufferQueue::with_capacity(2);
    ///
    /// // Successful enqueues
    /// assert!(queue.enqueue("first").is_ok());
    /// assert!(queue.enqueue("second").is_ok());
    ///
    /// // Queue is now full
    /// assert!(queue.is_full());
    /// assert_eq!(queue.enqueue("third"), Err("third"));
    ///
    /// // Make space and try again
    /// queue.dequeue();
    /// assert!(queue.enqueue("third").is_ok());
    /// ```
    ///
    /// # Implementation Details
    ///
    /// Uses modular arithmetic for wrap-around: `(tail + 1) % capacity`.
    /// The `debug_assert!` ensures the target slot is empty in debug builds.
    ///
    /// # Time Complexity
    ///
    /// O(1) - direct array assignment with modular arithmetic.
    pub fn enqueue(&mut self, x: T) -> Result<(), T> {
        if self.is_full() {
            return Err(x);
        }
        let cap = self.capacity();
        debug_assert!(self.buf[self.tail].is_none());
        self.buf[self.tail] = Some(x);
        self.tail = (self.tail + 1) % cap;
        self.len += 1;
        Ok(())
    }

    /// Removes and returns the element at the front of the queue.
    ///
    /// Returns `None` if the queue is empty. Elements are returned in FIFO order.
    ///
    /// # Returns
    ///
    /// * `Some(element)` if the queue had elements
    /// * `None` if the queue was empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission2::queue_alt::RingBufferQueue;
    ///
    /// let mut queue = RingBufferQueue::with_capacity(3);
    /// assert_eq!(queue.dequeue(), None);
    ///
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    /// queue.enqueue(3);
    ///
    /// // FIFO order
    /// assert_eq!(queue.dequeue(), Some(1));
    /// assert_eq!(queue.dequeue(), Some(2));
    /// assert_eq!(queue.dequeue(), Some(3));
    /// assert_eq!(queue.dequeue(), None);
    /// ```
    ///
    /// # Implementation Details
    ///
    /// Uses `Option::take()` to move the value out and leave `None` in its place.
    /// This avoids any copying and properly handles ownership transfer.
    ///
    /// # Time Complexity
    ///
    /// O(1) - direct array access with modular arithmetic.
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let cap = self.capacity();
        let val = self.buf[self.head].take();
        self.head = (self.head + 1) % cap;
        self.len -= 1;
        val
    }

    /// Returns a reference to the element at the front of the queue without removing it.
    ///
    /// Returns `None` if the queue is empty.
    ///
    /// # Returns
    ///
    /// * `Some(&element)` if the queue has elements
    /// * `None` if the queue is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission2::queue_alt::RingBufferQueue;
    ///
    /// let mut queue = RingBufferQueue::with_capacity(3);
    /// assert_eq!(queue.peek(), None);
    ///
    /// queue.enqueue("hello");
    /// queue.enqueue("world");
    ///
    /// // Peek doesn't modify the queue
    /// assert_eq!(queue.peek(), Some(&"hello"));
    /// assert_eq!(queue.len(), 2);
    ///
    /// // Dequeue returns the same element peek showed
    /// assert_eq!(queue.dequeue(), Some("hello"));
    /// assert_eq!(queue.peek(), Some(&"world"));
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(1) - direct array access without modification.
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() { 
            None 
        } else { 
            self.buf[self.head].as_ref() 
        }
    }
}

// ==========================
// Linked Queue (Alternative Implementation with unsafe)
// ==========================

/// A node in the linked queue containing an element and a pointer to the next node.
///
/// Each node owns its successor through `Option<Box<Node<T>>>`, creating a chain
/// of owned heap allocations. When a node is dropped, it automatically drops
/// its successor, creating recursive cleanup.
///
/// # Memory Layout
///
/// ```text
/// Node A -> Node B -> Node C -> None
///   |         |         |
/// elem_a    elem_b    elem_c
/// ```
#[derive(Debug)]
struct Node<T> {
    /// The element stored in this node
    elem: T,
    /// Owned pointer to the next node, or None if this is the tail
    next: Option<Box<Node<T>>>,
}

/// An unbounded FIFO queue implemented as a singly-linked list.
///
/// This implementation uses a head pointer for dequeue operations and a raw tail pointer
/// for O(1) enqueue operations. The tail pointer is managed carefully to ensure safety
/// while maintaining performance.
///
/// # Design Philosophy
///
/// This implementation demonstrates the classic queue design with explicit pointer
/// management. It uses a small amount of `unsafe` code to maintain a tail pointer,
/// showing how unsafe can be used judiciously for performance while maintaining
/// overall safety guarantees.
///
/// # Safety Invariants
///
/// The implementation maintains these critical invariants:
/// 1. `tail` always points to a valid node when `head` is Some
/// 2. `tail` is None if and only if `head` is None (empty queue)
/// 3. The node pointed to by `tail` is always owned by the linked structure
/// 4. `tail.next` is always None (tail is the last node)
///
/// # Performance Characteristics
///
/// - **enqueue**: O(1) - append to tail using raw pointer
/// - **dequeue**: O(1) - remove from head with ownership transfer  
/// - **peek**: O(1) - access head element without modification
/// - **Space**: O(n) - one heap allocation per element
///
/// # Memory Allocation Pattern
///
/// Each enqueue allocates a new `Box<Node<T>>`. Each dequeue deallocates the head node.
/// Memory usage grows and shrinks dynamically with queue size.
///
/// # Examples
///
/// ```rust
/// use mission2::queue_alt::LinkedQueue;
///
/// let mut queue = LinkedQueue::new();
/// assert!(queue.is_empty());
///
/// // Unlimited growth
/// for i in 0..1000 {
///     queue.enqueue(i);
/// }
/// assert_eq!(queue.len(), 1000);
///
/// // FIFO ordering maintained
/// for i in 0..1000 {
///     assert_eq!(queue.dequeue(), Some(i));
/// }
/// assert!(queue.is_empty());
/// ```
///
/// # Use Cases
///
/// Ideal for scenarios where:
/// - Queue size is unknown or highly variable
/// - Memory should be allocated only as needed
/// - Unbounded growth is required
/// - Occasional allocation overhead is acceptable
///
/// Common applications:
/// - Message passing between threads
/// - Event processing systems
/// - Task scheduling queues
/// - Parser token streams
/// - Any algorithm requiring dynamic FIFO storage
#[derive(Debug, Default)]
pub struct LinkedQueue<T> {
    /// Owned pointer to the first node (front of queue)
    head: Option<Box<Node<T>>>,
    /// Raw pointer to the last node (back of queue) for O(1) enqueue
    tail: Option<NonNull<Node<T>>>,
    /// Current number of elements in the queue
    len: usize,
}

impl<T> LinkedQueue<T> {
    /// Creates a new empty linked queue.
    ///
    /// The queue starts with no allocated memory and will grow as elements are added.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission2::queue_alt::LinkedQueue;
    ///
    /// let mut queue: LinkedQueue<String> = LinkedQueue::new();
    /// assert!(queue.is_empty());
    /// assert_eq!(queue.len(), 0);
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(1) - no allocation or initialization required.
    #[inline] 
    pub fn new() -> Self { 
        Self { 
            head: None, 
            tail: None, 
            len: 0 
        } 
    }
    
    /// Returns the current number of elements in the queue.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission2::queue_alt::LinkedQueue;
    ///
    /// let mut queue = LinkedQueue::new();
    /// assert_eq!(queue.len(), 0);
    ///
    /// queue.enqueue("first");
    /// queue.enqueue("second");
    /// assert_eq!(queue.len(), 2);
    ///
    /// queue.dequeue();
    /// assert_eq!(queue.len(), 1);
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(1) - length is maintained as queue operations occur.
    #[inline] 
    pub fn len(&self) -> usize { 
        self.len 
    }
    
    /// Returns `true` if the queue contains no elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission2::queue_alt::LinkedQueue;
    ///
    /// let mut queue = LinkedQueue::new();
    /// assert!(queue.is_empty());
    ///
    /// queue.enqueue(42);
    /// assert!(!queue.is_empty());
    ///
    /// queue.dequeue();
    /// assert!(queue.is_empty());
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(1) - checks if head pointer is None.
    #[inline] 
    pub fn is_empty(&self) -> bool { 
        self.head.is_none() 
    }

    /// Adds an element to the back of the queue.
    ///
    /// This operation always succeeds as the linked queue has no capacity limit.
    /// A new node is allocated on the heap to store the element.
    ///
    /// # Arguments
    ///
    /// * `x` - The element to add to the queue
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission2::queue_alt::LinkedQueue;
    ///
    /// let mut queue = LinkedQueue::new();
    /// queue.enqueue("first");
    /// queue.enqueue("second");
    /// queue.enqueue("third");
    ///
    /// assert_eq!(queue.len(), 3);
    /// assert_eq!(queue.peek(), Some(&"first"));
    /// ```
    ///
    /// # Implementation Details
    ///
    /// Creates a new `Box<Node<T>>` and updates the tail pointer using unsafe code.
    /// The unsafe block is carefully designed to maintain the safety invariants:
    /// - The tail pointer always refers to a node we own
    /// - The pointed-to node is always the actual tail of the list
    ///
    /// # Safety
    ///
    /// The unsafe code is sound because:
    /// - `tail_ptr` was created from a `Box<Node<T>>` we still own
    /// - We only mutate the `next` field of the node we own
    /// - The node remains owned by the linked structure throughout
    ///
    /// # Time Complexity
    ///
    /// O(1) - single heap allocation and pointer manipulation.
    pub fn enqueue(&mut self, x: T) {
        let mut new = Box::new(Node { elem: x, next: None });
        let new_ptr = Some(unsafe { NonNull::new_unchecked(&mut *new) });

        match self.tail {
            None => {
                // empty queue - new node becomes both head and tail
                self.head = Some(new);
                self.tail = new_ptr;
            }
            Some(mut tail_ptr) => {
                // SAFETY: tail_ptr came from a Box<Node<T>> we still own in the linked structure.
                // The node it points to is guaranteed to be valid and owned by us.
                // We're only modifying the `next` field, which is safe to do.
                unsafe {
                    tail_ptr.as_mut().next = Some(new);
                }
                self.tail = new_ptr;
            }
        }
        self.len += 1;
    }

    /// Removes and returns the element at the front of the queue.
    ///
    /// Returns `None` if the queue is empty. Elements are returned in FIFO order.
    ///
    /// # Returns
    ///
    /// * `Some(element)` if the queue had elements
    /// * `None` if the queue was empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission2::queue_alt::LinkedQueue;
    ///
    /// let mut queue = LinkedQueue::new();
    /// assert_eq!(queue.dequeue(), None);
    ///
    /// queue.enqueue("first");
    /// queue.enqueue("second");
    ///
    /// // FIFO order
    /// assert_eq!(queue.dequeue(), Some("first"));
    /// assert_eq!(queue.dequeue(), Some("second"));
    /// assert_eq!(queue.dequeue(), None);
    /// ```
    ///
    /// # Implementation Details
    ///
    /// Uses `Option::take()` to move the head node out of the Option, then extracts
    /// the element and moves the next node to become the new head. If the queue
    /// becomes empty, the tail pointer is also cleared.
    ///
    /// # Memory Management
    ///
    /// When a node is removed, its `Box<Node<T>>` is automatically deallocated,
    /// freeing both the node structure and its contained element.
    ///
    /// # Time Complexity
    ///
    /// O(1) - single pointer manipulation and deallocation.
    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|mut old_head| {
            self.head = old_head.next.take(); // move next into head
            if self.head.is_none() {
                self.tail = None; // became empty - clear tail pointer
            }
            self.len -= 1;
            old_head.elem
        })
    }

    /// Returns a reference to the element at the front of the queue without removing it.
    ///
    /// Returns `None` if the queue is empty.
    ///
    /// # Returns
    ///
    /// * `Some(&element)` if the queue has elements
    /// * `None` if the queue is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission2::queue_alt::LinkedQueue;
    ///
    /// let mut queue = LinkedQueue::new();
    /// assert_eq!(queue.peek(), None);
    ///
    /// queue.enqueue(100);
    /// queue.enqueue(200);
    ///
    /// // Peek doesn't modify the queue
    /// assert_eq!(queue.peek(), Some(&100));
    /// assert_eq!(queue.len(), 2);
    ///
    /// // Dequeue returns the same element peek showed
    /// assert_eq!(queue.dequeue(), Some(100));
    /// assert_eq!(queue.peek(), Some(&200));
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(1) - direct access to head node element.
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.elem)
    }
}

#[cfg(test)]
mod tests {
    //! # Unit Tests for Alternative Queue Implementations
    //!
    //! This module contains focused unit tests that verify the core functionality
    //! of both queue implementations. The tests are designed to be simple and
    //! educational, demonstrating key behaviors without excessive complexity.
    //!
    //! ## Test Coverage
    //!
    //! - Basic FIFO operations (enqueue, dequeue, peek)
    //! - Empty and full queue edge cases
    //! - Wrap-around behavior for ring buffer
    //! - Large sequence handling for linked queue
    //! - Memory safety and ownership transfer
    //!
    //! ## Test Philosophy
    //!
    //! These tests prioritize clarity and comprehension over exhaustive edge case
    //! coverage. They serve as executable documentation of how the queues work.

    use super::*;

    /// Tests fundamental ring buffer operations: enqueue, dequeue, peek, and state queries.
    ///
    /// Verifies that:
    /// - Empty queue behaves correctly
    /// - Elements can be added and removed
    /// - FIFO ordering is maintained
    /// - Length and emptiness are tracked correctly
    /// - Peek doesn't modify the queue
    #[test]
    fn ring_buffer_basic_operations() {
        let mut queue = RingBufferQueue::with_capacity(3);
        
        // Test empty queue state
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.capacity(), 3);
        assert!(queue.dequeue().is_none());
        assert!(queue.peek().is_none());
        
        // Test enqueue operations
        assert!(queue.enqueue(1).is_ok());
        assert!(queue.enqueue(2).is_ok());
        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 2);
        
        // Test peek functionality
        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.len(), 2); // peek doesn't change length
        
        // Test dequeue operations and FIFO ordering
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert!(queue.is_empty());
        assert!(queue.dequeue().is_none());
    }

    /// Tests ring buffer wrap-around behavior and capacity management.
    ///
    /// Verifies that:
    /// - Queue correctly reaches full capacity
    /// - Enqueue fails when full and returns the value
    /// - Wrap-around indexing works correctly
    /// - Elements maintain FIFO order through wrap-around
    #[test]
    fn ring_buffer_wrap_around() {
        let mut queue = RingBufferQueue::with_capacity(3);
        
        // Fill to capacity
        assert!(queue.enqueue(1).is_ok());
        assert!(queue.enqueue(2).is_ok());
        assert!(queue.enqueue(3).is_ok());
        assert!(queue.is_full());
        
        // Test overflow behavior - should return the value that couldn't be inserted
        assert_eq!(queue.enqueue(4), Err(4));
        
        // Create space and test wrap-around
        assert_eq!(queue.dequeue(), Some(1));
        assert!(queue.enqueue(4).is_ok());
        
        // Verify FIFO order is maintained through wrap-around
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
        assert!(queue.is_empty());
    }

    /// Tests fundamental linked queue operations: enqueue, dequeue, peek, and state queries.
    ///
    /// Verifies that:
    /// - Empty queue behaves correctly
    /// - Elements can be added without capacity limits
    /// - FIFO ordering is maintained
    /// - Length tracking works correctly
    /// - Peek provides access without modification
    #[test]
    fn linked_queue_basic_operations() {
        let mut queue = LinkedQueue::new();
        
        // Test empty queue state
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        assert!(queue.dequeue().is_none());
        assert!(queue.peek().is_none());
        
        // Test enqueue operations - no capacity limit
        queue.enqueue("first");
        queue.enqueue("second");
        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 2);
        
        // Test peek functionality
        assert_eq!(queue.peek(), Some(&"first"));
        assert_eq!(queue.len(), 2); // peek doesn't change length
        
        // Test dequeue operations and FIFO ordering
        assert_eq!(queue.dequeue(), Some("first"));
        assert_eq!(queue.dequeue(), Some("second"));
        assert!(queue.is_empty());
        assert!(queue.dequeue().is_none());
    }

    /// Tests linked queue performance with large sequences to verify scalability.
    ///
    /// Verifies that:
    /// - Queue can handle large numbers of elements
    /// - FIFO ordering is maintained across many operations
    /// - Memory management works correctly for many allocations
    /// - Performance remains acceptable for substantial workloads
    #[test]
    fn linked_queue_large_sequence() {
        let mut queue = LinkedQueue::new();
        
        // Enqueue a large number of elements
        for i in 0..1000 {
            queue.enqueue(i);
        }
        assert_eq!(queue.len(), 1000);
        
        // Dequeue all elements and verify FIFO order
        for i in 0..1000 {
            assert_eq!(queue.dequeue(), Some(i));
        }
        assert!(queue.is_empty());
    }
}