//! FIFO Queue implementations: Ring Buffer and Linked Queue
//!
//! This module provides two complementary queue implementations, each optimized
//! for different use cases while maintaining the same FIFO interface.

use std::ptr::NonNull;

// ==========================
// Ring Buffer Queue
// ==========================

/// A fixed-capacity FIFO queue implemented with a ring buffer.
///
/// This queue uses a contiguous `Vec<Option<T>>` with wrap-around indexing to provide
/// excellent cache locality and predictable memory usage. Perfect for scenarios where
/// you know an upper bound on queue size (e.g., BFS frontiers on bounded grids).
///
/// # Memory Layout
///
/// The queue maintains three indices:
/// - `head`: Index of the next element to dequeue
/// - `tail`: Index where the next element will be enqueued  
/// - `len`: Current number of elements
///
/// The buffer wraps around using modular arithmetic: `(index + 1) % capacity`.
///
/// # Examples
///
/// ```rust
/// use mission2::RingBufferQueue;
///
/// let mut queue = RingBufferQueue::with_capacity(3);
///
/// // Fill the queue
/// assert!(queue.enqueue(1).is_ok());
/// assert!(queue.enqueue(2).is_ok());
/// assert!(queue.enqueue(3).is_ok());
/// assert!(queue.is_full());
///
/// // Try to overfill - returns the value back
/// assert_eq!(queue.enqueue(4), Err(4));
///
/// // FIFO dequeue
/// assert_eq!(queue.dequeue(), Some(1));
/// assert_eq!(queue.dequeue(), Some(2));
///
/// // Now we can enqueue again (wrap-around)
/// assert!(queue.enqueue(4).is_ok());
/// ```
///
/// # Performance
///
/// - **enqueue**: O(1) - simple array assignment and index increment
/// - **dequeue**: O(1) - take from array and index increment  
/// - **peek**: O(1) - array access without modification
/// - **Space**: Fixed at construction, no reallocations
///
/// # Requirements Satisfied
/// - **REQ-R1**: Fixed capacity with backpressure via Err(value)
/// - **REQ-R2**: Efficient wrap-around using modular arithmetic
/// - **REQ-R3**: Contiguous Vec<Option<T>> with no reallocation
#[derive(Debug)]
pub struct RingBufferQueue<T> {
    /// Internal storage using Option<T> to allow moving values out safely
    buf: Vec<Option<T>>,
    /// Index of next element to dequeue (front of queue)
    head: usize,
    /// Index where next element will be enqueued (back of queue)
    tail: usize,
    /// Current number of elements in the queue
    len: usize,
}

impl<T> RingBufferQueue<T> {
    /// Creates a new ring buffer queue with the specified capacity.
    ///
    /// # Arguments
    /// * `cap` - Fixed capacity for the queue (must be > 0)
    ///
    /// # Panics
    /// Panics if capacity is 0, as this would create an unusable queue.
    ///
    /// # Examples
    /// ```rust
    /// use mission2::RingBufferQueue;
    ///
    /// let queue: RingBufferQueue<i32> = RingBufferQueue::with_capacity(10);
    /// assert_eq!(queue.capacity(), 10);
    /// assert!(queue.is_empty());
    /// ```
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
    /// ```rust
    /// use mission2::RingBufferQueue;
    ///
    /// let mut queue = RingBufferQueue::with_capacity(5);
    /// assert_eq!(queue.len(), 0);
    /// queue.enqueue("hello").unwrap();
    /// assert_eq!(queue.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the queue contains no elements.
    ///
    /// # Examples
    /// ```rust
    /// use mission2::RingBufferQueue;
    ///
    /// let mut queue = RingBufferQueue::with_capacity(5);
    /// assert!(queue.is_empty());
    /// queue.enqueue(1).unwrap();
    /// assert!(!queue.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the maximum capacity of the queue.
    ///
    /// # Examples
    /// ```rust
    /// use mission2::RingBufferQueue;
    ///
    /// let queue: RingBufferQueue<i32> = RingBufferQueue::with_capacity(10);
    /// assert_eq!(queue.capacity(), 10);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    /// Returns true if the queue is at maximum capacity.
    ///
    /// # Examples
    /// ```rust
    /// use mission2::RingBufferQueue;
    ///
    /// let mut queue = RingBufferQueue::with_capacity(2);
    /// assert!(!queue.is_full());
    /// queue.enqueue(1).unwrap();
    /// queue.enqueue(2).unwrap();
    /// assert!(queue.is_full());
    /// ```
    #[inline]
    pub fn is_full(&self) -> bool {
        self.len == self.capacity()
    }

    /// Adds an element to the back of the queue.
    ///
    /// Returns `Ok(())` if successful, or `Err(x)` if the queue is full,
    /// returning the value that couldn't be enqueued.
    ///
    /// # Arguments
    /// * `x` - The value to enqueue (ownership is transferred)
    ///
    /// # Examples
    /// ```rust
    /// use mission2::RingBufferQueue;
    ///
    /// let mut queue = RingBufferQueue::with_capacity(2);
    /// assert!(queue.enqueue(1).is_ok());
    /// assert!(queue.enqueue(2).is_ok());
    /// assert_eq!(queue.enqueue(3), Err(3)); // Full!
    /// ```
    ///
    /// # Requirements
    /// - **REQ-G1**: FIFO API implementation
    /// - **REQ-G4**: O(1) amortized complexity
    /// - **REQ-R1**: Returns Err(value) when full (no reallocation)
    pub fn enqueue(&mut self, x: T) -> Result<(), T> {
        if self.is_full() {
            return Err(x);
        }
        let cap = self.capacity();
        debug_assert!(self.buf[self.tail].is_none());
        self.buf[self.tail] = Some(x);
        self.tail = (self.tail + 1) % cap; // REQ-R2: Wrap-around
        self.len += 1;
        Ok(())
    }

    /// Removes and returns the element from the front of the queue.
    ///
    /// Returns `Some(value)` if the queue is not empty, or `None` if empty.
    ///
    /// # Examples
    /// ```rust
    /// use mission2::RingBufferQueue;
    ///
    /// let mut queue = RingBufferQueue::with_capacity(3);
    /// queue.enqueue(1).unwrap();
    /// queue.enqueue(2).unwrap();
    /// assert_eq!(queue.dequeue(), Some(1)); // FIFO order
    /// assert_eq!(queue.dequeue(), Some(2));
    /// assert_eq!(queue.dequeue(), None); // Empty
    /// ```
    ///
    /// # Requirements
    /// - **REQ-G1**: FIFO API implementation
    /// - **REQ-G2**: Maintains FIFO ordering
    /// - **REQ-G4**: O(1) complexity
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let cap = self.capacity();
        let val = self.buf[self.head].take(); // Move out, leave None
        self.head = (self.head + 1) % cap; // REQ-R2: Wrap-around
        self.len -= 1;
        val
    }

    /// Returns a reference to the front element without removing it.
    ///
    /// Returns `Some(&value)` if the queue is not empty, or `None` if empty.
    ///
    /// # Examples
    /// ```rust
    /// use mission2::RingBufferQueue;
    ///
    /// let mut queue = RingBufferQueue::with_capacity(3);
    /// assert_eq!(queue.peek(), None);
    /// queue.enqueue("hello").unwrap();
    /// assert_eq!(queue.peek(), Some(&"hello"));
    /// assert_eq!(queue.len(), 1); // peek doesn't remove
    /// ```
    ///
    /// # Requirements
    /// - **REQ-G1**: FIFO API implementation
    /// - **REQ-G4**: O(1) complexity
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.buf[self.head].as_ref()
        }
    }

    /// Adds an element to the back of the queue, overwriting the oldest element if full.
    ///
    /// Unlike `enqueue()`, this method never fails - if the queue is full, it overwrites
    /// the oldest element and returns it. This is useful for fixed-size buffers where
    /// you always want to keep the most recent N elements.
    ///
    /// When the queue is not full, this behaves identically to `enqueue().unwrap()`.
    /// When the queue is full, it overwrites the oldest element and advances both
    /// head and tail pointers to maintain the circular buffer invariants.
    ///
    /// # Returns
    /// - `None` if the queue had space (normal enqueue)
    /// - `Some(old_value)` if the queue was full and the oldest value was overwritten
    ///
    /// # Examples
    /// ```rust
    /// use mission2::RingBufferQueue;
    ///
    /// let mut queue = RingBufferQueue::with_capacity(3);
    /// 
    /// // Normal enqueue when not full
    /// assert_eq!(queue.enqueue_overwrite(1), None);
    /// assert_eq!(queue.enqueue_overwrite(2), None);
    /// assert_eq!(queue.enqueue_overwrite(3), None);
    /// assert_eq!(queue.len(), 3);
    /// 
    /// // Overwrite when full
    /// assert_eq!(queue.enqueue_overwrite(4), Some(1)); // Overwrote 1
    /// assert_eq!(queue.enqueue_overwrite(5), Some(2)); // Overwrote 2
    /// assert_eq!(queue.len(), 3); // Still full
    /// 
    /// // Queue now contains [3, 4, 5] in FIFO order
    /// assert_eq!(queue.dequeue(), Some(3));
    /// assert_eq!(queue.dequeue(), Some(4));
    /// assert_eq!(queue.dequeue(), Some(5));
    /// ```
    ///
    /// # Use Cases
    /// - Recent message history (keep last N messages)
    /// - Sensor data buffering (rolling window of readings)
    /// - Game frame history (replay/analysis buffer)
    /// - Log rotation (circular log buffer)
    ///
    /// # Requirements
    /// - **REQ-G1**: FIFO API extension
    /// - **REQ-G2**: Maintains FIFO ordering of remaining elements
    /// - **REQ-G4**: O(1) complexity
    /// - **REQ-R2**: Wrap-around behavior for full queue
    pub fn enqueue_overwrite(&mut self, x: T) -> Option<T> {
        let cap = self.capacity();
        
        if self.is_full() {
            // Overwrite the oldest element (at head position)
            let old_value = self.buf[self.head].replace(x);
            // Move both head and tail forward to maintain circular buffer
            self.head = (self.head + 1) % cap;
            self.tail = (self.tail + 1) % cap;
            // Length stays the same (still full)
            old_value
        } else {
            // Normal enqueue - queue has space
            self.buf[self.tail] = Some(x);
            self.tail = (self.tail + 1) % cap;
            self.len += 1;
            None
        }
    }
}

// ==========================
// Linked Queue (safe tail pointer)
// ==========================

/// Internal node structure for the linked queue.
///
/// Each node owns the next node in the chain, creating a singly-linked structure
/// where ownership flows from head to tail.
#[derive(Debug)]
struct Node<T> {
    /// The data stored in this node
    elem: T,
    /// Ownership of the next node (None for tail node)
    next: Option<Box<Node<T>>>,
}

/// A dynamically-sized FIFO queue implemented with a singly-linked list.
///
/// This queue can grow to any size limited only by available memory. It uses
/// head and tail pointers for O(1) enqueue and dequeue operations, making it
/// ideal for scenarios where queue size is unpredictable.
///
/// # Memory Layout
///
/// The queue maintains:
/// - `head`: Ownership of the first node (for dequeue)
/// - `tail`: Raw pointer to the last node (for enqueue)
/// - `len`: Current number of elements
///
/// # Safety
///
/// The tail pointer is managed carefully to ensure memory safety:
/// - It's only dereferenced when we know it's valid (queue not empty)
/// - It's always updated consistently with the head pointer
/// - No unsafe code escapes this module's boundaries
///
/// # Examples
///
/// ```rust
/// use mission2::LinkedQueue;
///
/// let mut queue = LinkedQueue::new();
///
/// // Enqueue elements
/// queue.enqueue("first");
/// queue.enqueue("second");
/// queue.enqueue("third");
///
/// // FIFO dequeue
/// assert_eq!(queue.dequeue(), Some("first"));
/// assert_eq!(queue.dequeue(), Some("second"));
/// assert_eq!(queue.len(), 1);
///
/// // Can keep growing
/// let mut int_queue = LinkedQueue::new();
/// for i in 0..1000 {
///     int_queue.enqueue(i);
/// }
/// assert_eq!(int_queue.len(), 1000);
/// ```
///
/// # Performance
///
/// - **enqueue**: O(1) - allocate new node and update tail
/// - **dequeue**: O(1) - move head ownership and deallocate
/// - **peek**: O(1) - dereference head pointer
/// - **Space**: Dynamic, one allocation per element
///
/// # Requirements Satisfied
/// - **REQ-L1**: Singly-linked nodes with head/tail pointers
/// - **REQ-L2**: True O(1) operations with pointer manipulation
/// - **REQ-L3**: Value ownership transfer without copying
#[derive(Debug, Default)]
pub struct LinkedQueue<T> {
    /// Ownership of the first node in the queue
    head: Option<Box<Node<T>>>,
    /// Raw pointer to the last node (for O(1) enqueue)
    tail: Option<NonNull<Node<T>>>,
    /// Current number of elements in the queue
    len: usize,
}

impl<T> LinkedQueue<T> {
    /// Creates a new empty linked queue.
    ///
    /// # Examples
    /// ```rust
    /// use mission2::LinkedQueue;
    ///
    /// let queue: LinkedQueue<i32> = LinkedQueue::new();
    /// assert!(queue.is_empty());
    /// assert_eq!(queue.len(), 0);
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    /// Returns the current number of elements in the queue.
    ///
    /// # Examples
    /// ```rust
    /// use mission2::LinkedQueue;
    ///
    /// let mut queue = LinkedQueue::new();
    /// assert_eq!(queue.len(), 0);
    /// queue.enqueue("hello");
    /// assert_eq!(queue.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the queue contains no elements.
    ///
    /// # Examples
    /// ```rust
    /// use mission2::LinkedQueue;
    ///
    /// let mut queue = LinkedQueue::new();
    /// assert!(queue.is_empty());
    /// queue.enqueue(1);
    /// assert!(!queue.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Adds an element to the back of the queue.
    ///
    /// This operation always succeeds and takes ownership of the value.
    /// Memory is allocated for a new node to store the element.
    ///
    /// # Arguments
    /// * `x` - The value to enqueue (ownership is transferred)
    ///
    /// # Examples
    /// ```rust
    /// use mission2::LinkedQueue;
    ///
    /// let mut queue = LinkedQueue::new();
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    /// queue.enqueue(3);
    /// assert_eq!(queue.len(), 3);
    /// ```
    ///
    /// # Requirements
    /// - **REQ-G1**: FIFO API implementation
    /// - **REQ-G4**: O(1) complexity
    /// - **REQ-L2**: O(1) append to tail
    /// - **REQ-L3**: Ownership transfer into node
    pub fn enqueue(&mut self, x: T) {
        let mut new = Box::new(Node { elem: x, next: None });
        let new_ptr = Some(unsafe { NonNull::new_unchecked(&mut *new) });

        match self.tail {
            None => {
                // Empty queue - both head and tail point to new node
                self.head = Some(new);
                self.tail = new_ptr;
            }
            Some(mut tail_ptr) => {
                // Non-empty queue - link tail to new node and update tail
                // SAFETY: tail_ptr came from a Box<Node<T>> we still own
                unsafe {
                    tail_ptr.as_mut().next = Some(new);
                }
                self.tail = new_ptr;
            }
        }
        self.len += 1;
    }

    /// Removes and returns the element from the front of the queue.
    ///
    /// Returns `Some(value)` if the queue is not empty, or `None` if empty.
    /// The returned value is moved out of the queue (ownership transfer).
    ///
    /// # Examples
    /// ```rust
    /// use mission2::LinkedQueue;
    ///
    /// let mut queue = LinkedQueue::new();
    /// queue.enqueue("first");
    /// queue.enqueue("second");
    /// assert_eq!(queue.dequeue(), Some("first")); // FIFO order
    /// assert_eq!(queue.dequeue(), Some("second"));
    /// assert_eq!(queue.dequeue(), None); // Empty
    /// ```
    ///
    /// # Requirements
    /// - **REQ-G1**: FIFO API implementation
    /// - **REQ-G2**: Maintains FIFO ordering
    /// - **REQ-G4**: O(1) complexity
    /// - **REQ-L2**: O(1) pop from head
    /// - **REQ-L3**: Ownership transfer out of node
    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|mut old_head| {
            self.head = old_head.next.take(); // Move next into head
            if self.head.is_none() {
                self.tail = None; // Queue became empty
            }
            self.len -= 1;
            old_head.elem // Move element out
        })
    }

    /// Returns a reference to the front element without removing it.
    ///
    /// Returns `Some(&value)` if the queue is not empty, or `None` if empty.
    ///
    /// # Examples
    /// ```rust
    /// use mission2::LinkedQueue;
    ///
    /// let mut queue = LinkedQueue::new();
    /// assert_eq!(queue.peek(), None);
    /// queue.enqueue("hello");
    /// assert_eq!(queue.peek(), Some(&"hello"));
    /// assert_eq!(queue.len(), 1); // peek doesn't remove
    /// ```
    ///
    /// # Requirements
    /// - **REQ-G1**: FIFO API implementation
    /// - **REQ-G4**: O(1) complexity
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

// Memory safety for LinkedQueue - implement Send and Sync safely
unsafe impl<T: Send> Send for LinkedQueue<T> {}
unsafe impl<T: Sync> Sync for LinkedQueue<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_buffer_basic_operations() {
        let mut queue = RingBufferQueue::with_capacity(3);
        
        // Test empty queue
        assert!(queue.is_empty());
        assert!(!queue.is_full());
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.capacity(), 3);
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.peek(), None);

        // Test enqueue
        assert!(queue.enqueue(1).is_ok());
        assert_eq!(queue.len(), 1);
        assert!(!queue.is_empty());
        assert_eq!(queue.peek(), Some(&1));

        // Fill queue
        assert!(queue.enqueue(2).is_ok());
        assert!(queue.enqueue(3).is_ok());
        assert!(queue.is_full());
        assert_eq!(queue.len(), 3);

        // Test full queue
        assert_eq!(queue.enqueue(4), Err(4));
        assert_eq!(queue.len(), 3);

        // Test dequeue FIFO order
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.len(), 1);
        assert!(!queue.is_full());

        // Test wrap-around
        assert!(queue.enqueue(4).is_ok());
        assert!(queue.enqueue(5).is_ok());
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), Some(5));
        assert!(queue.is_empty());
    }

    #[test]
    fn linked_queue_basic_operations() {
        let mut queue = LinkedQueue::new();
        
        // Test empty queue
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.peek(), None);

        // Test enqueue
        queue.enqueue(1);
        assert_eq!(queue.len(), 1);
        assert!(!queue.is_empty());
        assert_eq!(queue.peek(), Some(&1));

        // Test multiple enqueue
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.len(), 3);

        // Test dequeue FIFO order
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.len(), 1);

        // Test mixed operations
        queue.enqueue(4);
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
        assert!(queue.is_empty());
    }

    #[test]
    fn ring_buffer_wrap_around() {
        let mut queue = RingBufferQueue::with_capacity(2);
        
        // Fill and empty multiple times to test wrap-around
        for cycle in 0..3 {
            assert!(queue.enqueue(cycle * 2).is_ok());
            assert!(queue.enqueue(cycle * 2 + 1).is_ok());
            assert!(queue.is_full());
            
            assert_eq!(queue.dequeue(), Some(cycle * 2));
            assert_eq!(queue.dequeue(), Some(cycle * 2 + 1));
            assert!(queue.is_empty());
        }
    }

    #[test]
    fn linked_queue_large_sequence() {
        let mut queue = LinkedQueue::new();
        
        // Test with larger sequence to ensure memory management works
        for i in 0..100 {
            queue.enqueue(i);
        }
        assert_eq!(queue.len(), 100);
        
        for i in 0..50 {
            assert_eq!(queue.dequeue(), Some(i));
        }
        assert_eq!(queue.len(), 50);
        
        // Add more while some remain
        for i in 100..150 {
            queue.enqueue(i);
        }
        assert_eq!(queue.len(), 100);
        
        // Drain remaining
        for i in 50..100 {
            assert_eq!(queue.dequeue(), Some(i));
        }
        for i in 100..150 {
            assert_eq!(queue.dequeue(), Some(i));
        }
        assert!(queue.is_empty());
    }

    #[test]
    fn ring_buffer_enqueue_overwrite_basic() {
        let mut queue = RingBufferQueue::with_capacity(3);
        
        // Test normal enqueue when not full
        assert_eq!(queue.enqueue_overwrite(1), None);
        assert_eq!(queue.enqueue_overwrite(2), None);
        assert_eq!(queue.enqueue_overwrite(3), None);
        assert_eq!(queue.len(), 3);
        assert!(queue.is_full());
        
        // Test overwrite when full
        assert_eq!(queue.enqueue_overwrite(4), Some(1)); // Overwrote 1
        assert_eq!(queue.len(), 3); // Still full
        assert_eq!(queue.enqueue_overwrite(5), Some(2)); // Overwrote 2
        assert_eq!(queue.enqueue_overwrite(6), Some(3)); // Overwrote 3
        
        // Verify FIFO order of remaining elements
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), Some(5));
        assert_eq!(queue.dequeue(), Some(6));
        assert!(queue.is_empty());
    }

    #[test]
    fn ring_buffer_enqueue_overwrite_single_capacity() {
        let mut queue = RingBufferQueue::with_capacity(1);
        
        // First element goes in normally
        assert_eq!(queue.enqueue_overwrite(1), None);
        assert_eq!(queue.len(), 1);
        assert!(queue.is_full());
        
        // Subsequent elements overwrite
        assert_eq!(queue.enqueue_overwrite(2), Some(1));
        assert_eq!(queue.enqueue_overwrite(3), Some(2));
        assert_eq!(queue.enqueue_overwrite(4), Some(3));
        
        // Only the last element remains
        assert_eq!(queue.dequeue(), Some(4));
        assert!(queue.is_empty());
    }

    #[test]
    fn ring_buffer_enqueue_overwrite_mixed_operations() {
        let mut queue = RingBufferQueue::with_capacity(4);
        
        // Fill queue normally
        assert_eq!(queue.enqueue_overwrite(1), None);
        assert_eq!(queue.enqueue_overwrite(2), None);
        assert_eq!(queue.enqueue_overwrite(3), None);
        assert_eq!(queue.enqueue_overwrite(4), None);
        
        // Dequeue some elements
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.len(), 2);
        
        // Add more elements (should not overwrite since space available)
        assert_eq!(queue.enqueue_overwrite(5), None);
        assert_eq!(queue.enqueue_overwrite(6), None);
        assert!(queue.is_full());
        
        // Now overwrite again
        assert_eq!(queue.enqueue_overwrite(7), Some(3)); // Overwrote 3
        assert_eq!(queue.enqueue_overwrite(8), Some(4)); // Overwrote 4
        
        // Verify final order
        assert_eq!(queue.dequeue(), Some(5));
        assert_eq!(queue.dequeue(), Some(6));
        assert_eq!(queue.dequeue(), Some(7));
        assert_eq!(queue.dequeue(), Some(8));
        assert!(queue.is_empty());
    }

    #[test]
    fn ring_buffer_enqueue_overwrite_wrap_around() {
        let mut queue = RingBufferQueue::with_capacity(3);
        
        // Fill queue completely
        assert_eq!(queue.enqueue_overwrite(1), None);
        assert_eq!(queue.enqueue_overwrite(2), None);
        assert_eq!(queue.enqueue_overwrite(3), None);
        assert!(queue.is_full());
        
        // Now test multiple overwrite cycles to ensure wrap-around works
        assert_eq!(queue.enqueue_overwrite(4), Some(1)); // [2, 3, 4]
        assert_eq!(queue.enqueue_overwrite(5), Some(2)); // [3, 4, 5]
        assert_eq!(queue.enqueue_overwrite(6), Some(3)); // [4, 5, 6]
        
        // Verify the queue wrapped around correctly
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), Some(5));
        assert_eq!(queue.dequeue(), Some(6));
        assert!(queue.is_empty());
        
        // Test wrap-around after partial operations
        queue.enqueue_overwrite(10);
        queue.enqueue_overwrite(20);
        assert_eq!(queue.dequeue(), Some(10)); // Partial drain
        
        // Fill again
        queue.enqueue_overwrite(30);
        queue.enqueue_overwrite(40); // Should fill without overwrite
        assert_eq!(queue.len(), 3);
        
        // Now overwrite should happen
        assert_eq!(queue.enqueue_overwrite(50), Some(20)); // Overwrote 20
        assert_eq!(queue.dequeue(), Some(30));
        assert_eq!(queue.dequeue(), Some(40));
        assert_eq!(queue.dequeue(), Some(50));
    }

    #[test]
    fn ring_buffer_enqueue_overwrite_maintains_peek() {
        let mut queue = RingBufferQueue::with_capacity(2);
        
        // Fill queue
        queue.enqueue_overwrite(1);
        queue.enqueue_overwrite(2);
        assert_eq!(queue.peek(), Some(&1));
        
        // Overwrite - peek should now show the new front
        assert_eq!(queue.enqueue_overwrite(3), Some(1));
        assert_eq!(queue.peek(), Some(&2)); // 2 is now at front
        
        assert_eq!(queue.enqueue_overwrite(4), Some(2));
        assert_eq!(queue.peek(), Some(&3)); // 3 is now at front
        
        // Verify by dequeuing
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
    }

    #[test]
    fn ring_buffer_enqueue_overwrite_type_safety() {
        // Test with different types to ensure generic implementation works
        let mut string_queue = RingBufferQueue::with_capacity(2);
        assert_eq!(string_queue.enqueue_overwrite("first".to_string()), None);
        assert_eq!(string_queue.enqueue_overwrite("second".to_string()), None);
        assert_eq!(string_queue.enqueue_overwrite("third".to_string()), Some("first".to_string()));
        
        let mut option_queue = RingBufferQueue::with_capacity(2);
        assert_eq!(option_queue.enqueue_overwrite(Some(1)), None);
        assert_eq!(option_queue.enqueue_overwrite(None), None);
        assert_eq!(option_queue.enqueue_overwrite(Some(2)), Some(Some(1)));
    }
}