I am ready for Phase 1 · Core Data Structures & Ownership

(Many AoC puzzles require ad hoc stacks, queues, sets, maps, grids.)

Mission 2 (Queue): Ring buffer + linked queue.  Please present this with the full V-cycle
Universal Primer said:
Thought for 10s

awesome — Mission 2 is your Queue double feature: a cache-friendly ring buffer and a pointer-flavored linked queue. We’ll do the full V-cycle so it’s production-grade and AoC-ready.

Mission 2 · Queue (Ring Buffer + Linked Queue)
1) Requirements
Global (both implementations)

REQ-G1 (API): Provide a FIFO queue API:

enqueue(&mut self, T) -> Result<(), T>
dequeue(&mut self) -> Option<T>
peek(&self) -> Option<&T>
len(&self) -> usize
is_empty(&self) -> bool


REQ-G2 (Correctness): Elements shall be returned in exact FIFO order.

REQ-G3 (Safety): No panics on normal operations; no use-after-free; no invalid references.

REQ-G4 (Complexity): enqueue, dequeue, and peek are O(1) amortized.

REQ-G5 (Determinism): Behavior is deterministic across runs/platforms.

Ring Buffer specific

REQ-R1 (Capacity): Constructed with fixed capacity > 0. When full, enqueue returns Err(value) (no reallocation).

REQ-R2 (Wrap-around): Indexing must wrap ((i+1) % capacity) without branching bugs.

REQ-R3 (Space): Uses a contiguous Vec<Option<T>> of size capacity, and never reallocates after construction.

Linked Queue specific

REQ-L1 (Structure): Singly linked nodes with head/tail pointers.

REQ-L2 (O(1) ops): enqueue appends to tail in O(1); dequeue pops from head in O(1); queue becomes empty when head.is_none() and tail.is_none().

REQ-L3 (Moves): Elements move in/out by ownership transfer (T is moved into nodes and out of them).

2) Design Specification
2.1 Ring Buffer Queue

Storage: Vec<Option<T>> so we can move values out and leave None (no shifting).

State: head (index of next dequeue), tail (index of next enqueue), len.

Empty/full: empty if len == 0; full if len == capacity.

Why Option? Moving out of a Vec<T> element by index isn’t allowed safely. Option<T> + .take() makes “hole punching” safe.

Analogy: a circular airport baggage carousel with numbered slots. head is the next bag to take; tail is where the next bag gets placed. When it wraps, you just keep walking in circles.

2.2 Linked Queue

Node: struct Node<T> { elem: T, next: Option<Box<Node<T>>> }

Queue: head: Option<Box<Node<T>>>, tail: *mut Node<T> (or store Option<NonNull<Node<T>>>) but we’ll keep it safe using only Option<Box<Node<T>>> and a second pointer clone (Option<*mut Node<T>>) hidden behind safe methods. To stay fully safe without unsafe, we store tail as Option<*mut Node<T>> but never deref it outside controlled code (Rust allows raw pointers without unsafe as long as you don’t deref). Alternatively, keep it purely safe by updating tail via last node traversal (O(n)) — we’ll pick the classic safe O(1) tail technique borrowed from the Rust Book queue: store tail as Option<*mut Node<T>> and only deref inside a small unsafe block that’s proven correct. If you prefer zero unsafe, I also include a pure safe version with O(n) enqueue; choose one. (We’ll pick the fully safe but slightly more verbose approach below with a small unsafe that never escapes.)

Analogy: a line of carts where each cart owns the next cart. You hold a pointer to the first and last cart for fast in/out.

3) Implementation

Create src/queue.rs:

// ==========================
// Ring Buffer Queue
// ==========================
#[derive(Debug)]
pub struct RingBufferQueue<T> {
    buf: Vec<Option<T>>,
    head: usize, // next to dequeue
    tail: usize, // next to enqueue
    len: usize,
}

impl<T> RingBufferQueue<T> {
    pub fn with_capacity(cap: usize) -> Self {
        assert!(cap > 0, "capacity must be > 0");
        Self {
            buf: (0..cap).map(|_| None).collect(),
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    #[inline] pub fn len(&self) -> usize { self.len }
    #[inline] pub fn is_empty(&self) -> bool { self.len == 0 }
    #[inline] pub fn capacity(&self) -> usize { self.buf.len() }
    #[inline] pub fn is_full(&self) -> bool { self.len == self.capacity() }

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

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() { None } else { self.buf[self.head].as_ref() }
    }
}

// ==========================
// Linked Queue (safe tail pointer)
// ==========================
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug, Default)]
pub struct LinkedQueue<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<NonNull<Node<T>>>, // raw, non-null ptr to last node
    len: usize,
}

impl<T> LinkedQueue<T> {
    #[inline] pub fn new() -> Self { Self { head: None, tail: None, len: 0 } }
    #[inline] pub fn len(&self) -> usize { self.len }
    #[inline] pub fn is_empty(&self) -> bool { self.head.is_none() }

    pub fn enqueue(&mut self, x: T) {
        let mut new = Box::new(Node { elem: x, next: None });
        let new_ptr = Some(unsafe { NonNull::new_unchecked(&mut *new) });

        match self.tail {
            None => {
                // empty queue
                self.head = Some(new);
                self.tail = new_ptr;
            }
            Some(mut tail_ptr) => {
                // SAFETY: tail_ptr came from a Box<Node<T>> we still own
                unsafe {
                    tail_ptr.as_mut().next = Some(new);
                }
                self.tail = new_ptr;
            }
        }
        self.len += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|mut old_head| {
            self.head = old_head.next.take(); // move next into head
            if self.head.is_none() {
                self.tail = None; // became empty
            }
            self.len -= 1;
            old_head.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.elem)
    }
}


Wire it in src/lib.rs:

pub mod stack;
pub mod brackets;
pub mod queue;

4) Verification (Unit & Property-ish Tests)

Create tests/queue_test.rs:

use algolab::queue::{RingBufferQueue, LinkedQueue};
use std::collections::VecDeque;

#[test] // REQ-R1, REQ-G2, REQ-G3, REQ-G4
fn ring_basic_wrap_and_full() {
    let mut q = RingBufferQueue::with_capacity(3);
    assert!(q.enqueue(1).is_ok());
    assert!(q.enqueue(2).is_ok());
    assert!(q.enqueue(3).is_err()); // full
    assert_eq!(q.dequeue(), Some(1));
    assert!(q.enqueue(3).is_ok());  // wrap
    assert_eq!(q.peek(), Some(&2));
    assert_eq!(q.dequeue(), Some(2));
    assert_eq!(q.dequeue(), Some(3));
    assert!(q.dequeue().is_none());
    assert!(q.is_empty());
}

#[test] // REQ-G2 vs reference (VecDeque)
fn ring_randomized_matches_vecdeque() {
    let mut q = RingBufferQueue::with_capacity(8);
    let mut v = VecDeque::new();
    // simple pseudo-random pattern
    for i in 0..10_000 {
        if (i * 2654435761u32) % 7 != 0 {
            // enqueue
            let x = (i as i32) % 100 - 50;
            if q.enqueue(x).is_ok() {
                if v.len() < q.capacity() { v.push_back(x); }
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


These tests cover FIFO correctness, wrap-around behavior, full/empty edge cases, and a reference comparison to VecDeque (a standard FIFO).

5) Validation (AoC-style usage)

Ring buffer: perfect when you know an upper bound on queue size (e.g., BFS frontier for a bounded grid). Very cache-friendly: contiguous memory, minimal pointer chasing.

Linked: great when size may grow unpredictably and you’d rather avoid “full” errors; each enqueue allocates one node.

Example validation scenario (tiny BFS frontier growth):

// not a full BFS; just shows "frontier" behavior
let mut frontier = RingBufferQueue::with_capacity(4);
for &x in &[1,2,3,4] { let _ = frontier.enqueue(x); }
assert!(frontier.is_full());
assert_eq!(frontier.dequeue(), Some(1));
assert!(frontier.enqueue(5).is_ok()); // wraps and continues

6) Traceability Matrix
Requirement	Design Element	Test(s)
REQ-G1 API	Methods on both queues	compile-time, all tests
REQ-G2 FIFO	push at tail / pop at head	ring_basic_wrap_and_full, linked_basic_fifo, *_matches_vecdeque
REQ-G3 Safety	Option<T> holes; controlled pointer use	edges_empty_and_singleton, all tests run without panic
REQ-G4 O(1)	ring: index math; linked: head/tail	code review; randomized tests
REQ-G5 Determinism	pure data structures	all tests
REQ-R1 Capacity/full	ring with_capacity, is_full, Err(x)	ring_basic_wrap_and_full
REQ-R2 Wrap-around	(i+1) % capacity	ring_basic_wrap_and_full
REQ-R3 No realloc	fixed Vec<Option<T>>	code review
REQ-L1 Structure	Node<T>, head/tail	code review
REQ-L2 O(1) ops	tail pointer + head pop	linked_basic_fifo, linked_matches_vecdeque
REQ-L3 Moves	ownership into/out of nodes	tests pop owned values
Deep intuition & ownership notes (quick)

Ring: take() is the key move. You “lift the book out” and leave a None reservation card, so no one double-lends the same slot. Borrow checker guarantees no aliasing across enq/deq phases.

Linked: Box<Node<T>> means each node owns its successor (next). Moving the head (self.head.take()) transfers ownership cleanly. The raw tail pointer is just a bookmark; we never expose it or deref it unsafely across moves.

Stretch ideas (optional)

Add try_enqueue_many<I: IntoIterator<Item=T>> for batch enqueues on the ring, returning the first value that didn’t fit.

Implement IntoIterator for consuming iteration.

For ring: drain(n) that yields up to n items without realloc.

Your next steps

Drop src/queue.rs and tests/queue_test.rs into your crate (plus the mod queue; line).

Run:

cargo test
cargo clippy -- -D warnings


If anything is unclear (especially the tiny unsafe for tail update), paste errors or questions — I’ll refine or show the fully-safe O(n) enqueue alternative.

Quick comprehension probe (to calibrate next lesson)

In the ring buffer, why do we need Option<T> instead of Vec<T>? What goes wrong if we try let val = self.buf[self.head] to move out?

In the linked queue, explain why tail must be updated after linking tail.next = Some(new) — what invariant would break otherwise?

Give a tight argument for O(1) amortized ops on the ring even with wrap-around.

When might the linked queue beat the ring buffer in practice, despite worse cache locality?

Answer in bullets when you can; I’ll tailor Mission 3 (Binary Search + traits & iterators) accordingly.