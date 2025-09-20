# Day 14 · Mission 2 (Queue: Ring Buffer + Linked)

## Key Points
- Ring Buffer:
  - Fixed capacity, uses `Vec<Option<T>>`.
  - Wrap-around with `(i+1) % capacity`.
  - Enqueue returns `Err(value)` if full.
- Linked Queue:
  - Singly linked nodes, O(1) enqueue/dequeue.
  - `Box<Node<T>>` owns nodes; raw pointer for tail.
- Requirements Testing:
  - FIFO order (REQ-G2).
  - O(1) complexity (REQ-G4).
  - No panics, no aliasing issues.

## Example
```rust
let mut q = RingBufferQueue::with_capacity(3);
q.enqueue(1).unwrap();
q.enqueue(2).unwrap();
assert_eq!(q.dequeue(), Some(1));
```

## Takeaway
Mission 2 reinforces ownership + borrowing in dynamic structures, and introduces safe low-level patterns useful for AoC (BFS frontiers, job queues).
