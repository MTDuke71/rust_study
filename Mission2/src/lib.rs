//! # Mission 2: FIFO Queue Implementations
//!
//! This crate provides two high-performance FIFO (First-In-First-Out) queue implementations
//! designed for different use cases in competitive programming and systems development.
//!
//! ## Implementations
//!
//! ### 🔄 **RingBufferQueue<T>**
//! - **Fixed capacity** with contiguous memory layout
//! - **Cache-friendly** with no pointer chasing
//! - **O(1) operations** with predictable performance
//! - **Perfect for bounded problems** (e.g., BFS on grids)
//!
//! ### 🔗 **LinkedQueue<T>**
//! - **Dynamic size** growing as needed
//! - **No capacity limits** beyond available memory
//! - **O(1) operations** with ownership transfer
//! - **Ideal for unbounded problems** with unknown queue size
//!
//! ## Quick Start
//!
//! ```rust
//! use mission2::queue::{RingBufferQueue, LinkedQueue};
//!
//! // Ring buffer queue (fixed capacity)
//! let mut ring = RingBufferQueue::with_capacity(3);
//! ring.enqueue(1).unwrap();
//! ring.enqueue(2).unwrap();
//! assert_eq!(ring.dequeue(), Some(1)); // FIFO order
//!
//! // Linked queue (dynamic size)
//! let mut linked = LinkedQueue::new();
//! linked.enqueue("hello");
//! linked.enqueue("world");
//! assert_eq!(linked.dequeue(), Some("hello"));
//! ```
//!
//! ## Requirements Fulfilled
//!
//! This implementation satisfies all V-Cycle requirements:
//!
//! ### Global Requirements (Both Implementations)
//! - **REQ-G1**: Complete FIFO API (enqueue, dequeue, peek, len, is_empty)
//! - **REQ-G2**: Strict FIFO ordering preservation
//! - **REQ-G3**: Memory safety without panics or use-after-free
//! - **REQ-G4**: O(1) amortized complexity for all operations
//! - **REQ-G5**: Deterministic behavior across platforms
//!
//! ### Ring Buffer Specific
//! - **REQ-R1**: Fixed capacity with backpressure (returns Err on full)
//! - **REQ-R2**: Efficient wrap-around indexing without branches
//! - **REQ-R3**: Contiguous memory with no reallocation
//!
//! ### Linked Queue Specific
//! - **REQ-L1**: Singly-linked node structure with head/tail
//! - **REQ-L2**: True O(1) operations with pointer manipulation
//! - **REQ-L3**: Value ownership transfer without copying
//!
//! ## Performance Characteristics
//!
//! | Operation | RingBufferQueue | LinkedQueue |
//! |-----------|-----------------|-------------|
//! | enqueue   | O(1)           | O(1)        |
//! | dequeue   | O(1)           | O(1)        |
//! | peek      | O(1)           | O(1)        |
//! | Space     | Fixed          | Dynamic     |
//! | Cache     | Excellent      | Moderate    |
//!
//! ## Usage Patterns
//!
//! ### When to Use RingBufferQueue
//! - BFS on bounded grids or graphs
//! - Producer-consumer with known capacity
//! - Real-time systems requiring predictable memory
//! - High-frequency operations needing cache efficiency
//!
//! ### When to Use LinkedQueue
//! - Unknown or highly variable queue sizes
//! - Memory-constrained environments (allocate as needed)
//! - Algorithms with unpredictable growth patterns
//! - When occasional allocation overhead is acceptable

pub mod queue;
pub mod queue_alt; // Alternative implementations from original specification

// Re-export main types for convenient access
pub use queue::{RingBufferQueue, LinkedQueue};