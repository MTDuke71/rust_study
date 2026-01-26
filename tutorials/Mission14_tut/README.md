# Mission 14 Tutorial: Concurrent Data Structures (10 Steps)

**Tutorial Focus**: Thread-safe data structures, synchronization primitives, lock-free algorithms

**Zettelkasten**: [[mission-14]] | [[rust-concurrency]] | [[lock-free-algorithms]]

---

## 📚 Overview

This tutorial explores **concurrent data structures** for multi-threaded Rust programs. Learn how to build thread-safe collections using Mutex, RwLock, atomic operations, and lock-free techniques. Apply knowledge from Rust Book Ch16 (Fearless Concurrency) and Ch17 (Async) to practical data structure design.

**Key Concepts**:
- Thread safety with Sync and Send traits
- Mutex vs RwLock for shared state
- Atomic operations and memory ordering
- Lock-free data structures
- Channel-based concurrency
- Arc for shared ownership

**Why Concurrent Data Structures Matter**:
- **Performance**: Multi-core utilization for parallel workloads
- **Real-world**: Servers, databases, background processing
- **Integrator Perspective**: Understanding sync primitives for safe composition
- **Rust Strength**: Compile-time thread safety guarantees

---

## 🎯 Learning Path (7 Days)

### **Day 1: Thread Safety Fundamentals** (Step 1)
**File**: `examples/step1_thread_safety_basics.rs`

**Topics**:
- Sync and Send traits
- Arc<T> for shared ownership across threads
- Mutex<T> for interior mutability
- The problem concurrent data structures solve

**Core Traits**:
```rust
// Send: Type can be transferred across threads
// Sync: Type can be referenced from multiple threads

// Examples:
// - i32: Send + Sync (copyable, immutable)
// - Rc<T>: !Send + !Sync (not thread-safe)
// - Arc<T>: Send + Sync (atomic reference counting)
// - Mutex<T>: Send + Sync (provides interior mutability)
```

**Basic Pattern**:
```rust
use std::sync::{Arc, Mutex};

let shared_data = Arc::new(Mutex::new(vec![1, 2, 3]));

// Clone Arc for each thread
let thread_data = Arc::clone(&shared_data);
thread::spawn(move || {
    let mut data = thread_data.lock().unwrap();
    data.push(4);
});
```

**Learning Outcomes**:
- ✅ Understand Send and Sync trait requirements
- ✅ Use Arc for shared ownership
- ✅ Protect mutable state with Mutex

---

### **Day 2: Concurrent Stack & Queue** (Step 2)
**File**: `examples/step2_concurrent_stack_queue.rs`

**Topics**:
- Thread-safe stack with Mutex
- Thread-safe queue with Mutex
- Performance: contention and lock granularity

**Implementation**:
```rust
pub struct ConcurrentStack<T> {
    data: Arc<Mutex<Vec<T>>>,
}

impl<T> ConcurrentStack<T> {
    pub fn push(&self, value: T) {
        let mut stack = self.data.lock().unwrap();
        stack.push(value);
    }
    
    pub fn pop(&self) -> Option<T> {
        let mut stack = self.data.lock().unwrap();
        stack.pop()
    }
}
```

**Testing Concurrency**:
```rust
// Spawn multiple threads pushing/popping
let stack = Arc::new(ConcurrentStack::new());
let handles: Vec<_> = (0..10)
    .map(|i| {
        let s = Arc::clone(&stack);
        thread::spawn(move || {
            s.push(i);
        })
    })
    .collect();

for handle in handles {
    handle.join().unwrap();
}
```

**Learning Outcomes**:
- ✅ Wrap Mission 1/2 structures with Mutex
- ✅ Test concurrent access patterns
- ✅ Identify contention bottlenecks

---

### **Day 3: RwLock for Read-Heavy Workloads** (Step 3)
**File**: `examples/step3_rwlock_patterns.rs`

**Topics**:
- RwLock<T> for multiple readers, single writer
- Read vs write lock acquisition
- When to use RwLock vs Mutex

**Pattern**:
```rust
use std::sync::RwLock;

pub struct ConcurrentCache<K, V> {
    data: Arc<RwLock<HashMap<K, V>>>,
}

impl<K: Hash + Eq, V: Clone> ConcurrentCache<K, V> {
    pub fn get(&self, key: &K) -> Option<V> {
        let cache = self.data.read().unwrap();  // Multiple readers OK
        cache.get(key).cloned()
    }
    
    pub fn insert(&self, key: K, value: V) {
        let mut cache = self.data.write().unwrap();  // Exclusive write
        cache.insert(key, value);
    }
}
```

**Comparison**:
| Lock Type | Multiple Readers | Multiple Writers | Best For |
|-----------|------------------|------------------|----------|
| Mutex | ❌ No | ❌ No | Write-heavy |
| RwLock | ✅ Yes | ❌ No | Read-heavy |

**Learning Outcomes**:
- ✅ Use RwLock for read-heavy scenarios
- ✅ Balance read/write lock overhead
- ✅ Benchmark Mutex vs RwLock performance

---

### **Day 4: Atomic Operations** (Step 4)
**File**: `examples/step4_atomic_operations.rs`

**Topics**:
- Atomic types (AtomicBool, AtomicUsize, AtomicPtr)
- Memory ordering (Relaxed, Acquire, Release, SeqCst)
- Compare-and-swap (CAS) operations
- Building lock-free counters

**Atomic Counter**:
```rust
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct AtomicCounter {
    count: AtomicUsize,
}

impl AtomicCounter {
    pub fn increment(&self) -> usize {
        self.count.fetch_add(1, Ordering::SeqCst)
    }
    
    pub fn get(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
}
```

**Memory Ordering Guide**:
- **Relaxed**: No ordering guarantees (fastest)
- **Acquire**: Loads before this can't move after
- **Release**: Stores after this can't move before
- **SeqCst**: Total ordering (safest, default)

**Learning Outcomes**:
- ✅ Use atomic types for simple shared state
- ✅ Understand memory ordering basics
- ✅ Recognize when atomics suffice vs needing locks

---

### **Day 5: Lock-Free Stack** (Step 5)
**File**: `examples/step5_lock_free_stack.rs`

**Topics**:
- Treiber stack (lock-free stack algorithm)
- AtomicPtr for lock-free linked list
- ABA problem and solutions
- Compare performance vs Mutex stack

**Treiber Stack**:
```rust
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

pub struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> LockFreeStack<T> {
    pub fn push(&self, value: T) {
        let new_node = Box::into_raw(Box::new(Node {
            value,
            next: ptr::null_mut(),
        }));
        
        loop {
            let head = self.head.load(Ordering::Relaxed);
            unsafe { (*new_node).next = head; }
            
            // CAS: if head unchanged, update to new_node
            if self.head
                .compare_exchange(head, new_node, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }
        }
    }
}
```

**ABA Problem**:
- Thread 1 sees head=A
- Thread 2: pops A, pops B, pushes A (head=A again!)
- Thread 1: CAS succeeds but state changed!
- Solution: Tagged pointers or epoch-based reclamation

**Learning Outcomes**:
- ✅ Implement lock-free stack with CAS
- ✅ Understand ABA problem
- ✅ Compare lock-free vs lock-based performance

---

### **Day 6: Channel-Based Concurrency** (Step 6)
**File**: `examples/step6_channels.rs`

**Topics**:
- mpsc::channel for message passing
- Bounded vs unbounded channels
- crossbeam-channel for advanced patterns
- Work-stealing queue pattern

**Message-Passing Pattern**:
```rust
use std::sync::mpsc;

pub struct WorkQueue<T> {
    sender: mpsc::Sender<T>,
    receiver: Arc<Mutex<mpsc::Receiver<T>>>,
}

impl<T> WorkQueue<T> {
    pub fn push(&self, item: T) {
        self.sender.send(item).unwrap();
    }
    
    pub fn pop(&self) -> Option<T> {
        let rx = self.receiver.lock().unwrap();
        rx.try_recv().ok()
    }
}

// Worker threads:
let queue = Arc::new(WorkQueue::new());
for _ in 0..num_workers {
    let q = Arc::clone(&queue);
    thread::spawn(move || {
        while let Some(work) = q.pop() {
            process(work);
        }
    });
}
```

**crossbeam Enhancements**:
- Multiple producers, multiple consumers
- Select over multiple channels
- Bounded channels with backpressure

**Learning Outcomes**:
- ✅ Use channels for thread communication
- ✅ Build worker pool patterns
- ✅ Compare channels vs shared memory

---

### **Day 7: Concurrent HashMap & Advanced Patterns** (Step 7)
**File**: `examples/step7_concurrent_hashmap.rs`

**Topics**:
- Sharded locking for concurrent HashMap
- DashMap (production concurrent map)
- Parking lot optimizations
- Integration with Mission 5 HashMap

**Sharded HashMap**:
```rust
pub struct ConcurrentHashMap<K, V> {
    shards: Vec<RwLock<HashMap<K, V>>>,
    num_shards: usize,
}

impl<K: Hash + Eq, V> ConcurrentHashMap<K, V> {
    fn shard_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.num_shards
    }
    
    pub fn insert(&self, key: K, value: V) {
        let index = self.shard_index(&key);
        let mut shard = self.shards[index].write().unwrap();
        shard.insert(key, value);
    }
}
```

**Why Sharding Works**:
- Reduces contention (different keys → different locks)
- Trade-off: memory overhead vs parallelism
- Typical: 16-256 shards depending on workload

**Advanced Patterns**:
- **Epoch-based reclamation**: Safe lock-free memory reclamation
- **Hazard pointers**: Alternative to epoch-based
- **Seqlock**: Optimistic locking for small data

**Learning Outcomes**:
- ✅ Implement sharded concurrent map
- ✅ Understand contention reduction strategies
- ✅ Use production libraries (DashMap, parking_lot)

---

## 🔗 Integration Points

### **Rust Book Ch16/17**
- Chapter 16: Threads, message passing, shared state
- Chapter 17: Async runtime integration
- Apply book concepts to data structure design

### **Mission 1-10 (Data Structures)**
- Make existing structures thread-safe
- Mission 1/2: Concurrent stack/queue
- Mission 5: Concurrent HashMap (sharded)
- Mission 13: Concurrent priority queue

### **Real-World Applications**
- Web servers (shared request state)
- Database connection pools
- Background job processing
- Cache implementations

---

## 📊 Performance Characteristics

| Technique | Throughput | Latency | Complexity |
|-----------|-----------|---------|------------|
| **Mutex** | Low (serial) | Low | Simple |
| **RwLock** | High (reads) | Medium | Simple |
| **Atomics** | Very High | Very Low | Medium |
| **Lock-Free** | Highest | Lowest | Complex |
| **Channels** | Medium | Low | Simple |
| **Sharding** | High | Low | Medium |

---

## 🎓 Learning Objectives

By completing this tutorial, you will:

### Thread Safety
- ✅ Understand Send and Sync traits
- ✅ Use Arc for shared ownership
- ✅ Protect state with Mutex and RwLock

### Atomic Operations
- ✅ Use atomic types for simple counters
- ✅ Understand memory ordering basics
- ✅ Implement compare-and-swap patterns

### Lock-Free Algorithms
- ✅ Build lock-free stack (Treiber)
- ✅ Recognize ABA problem
- ✅ Compare lock-free vs lock-based

### Message Passing
- ✅ Use channels for thread communication
- ✅ Build worker pool patterns
- ✅ Choose channels vs shared memory

### Advanced Techniques
- ✅ Implement sharded concurrent HashMap
- ✅ Reduce contention with striping
- ✅ Use production concurrent libraries

---

## 🚀 Getting Started

```bash
cd tutorials/Mission14_tut

# Run examples in sequence
cargo run --example step1_thread_safety_basics
cargo run --example step2_concurrent_stack_queue
cargo run --example step3_rwlock_patterns
cargo run --example step4_atomic_operations
cargo run --example step5_lock_free_stack
cargo run --example step6_channels
cargo run --example step7_concurrent_hashmap

# Concurrent tests (may need --test-threads=1 to see output)
cargo test
cargo test -- --test-threads=8  # Test with parallelism

# Benchmarks comparing lock strategies
cargo bench
```

---

## 📚 References

**Standard Library**:
- `std::sync::{Arc, Mutex, RwLock}`
- `std::sync::atomic::{AtomicBool, AtomicUsize, Ordering}`
- `std::sync::mpsc::channel`

**External Crates** (for comparison/reference):
- `crossbeam` - Advanced concurrent utilities
- `parking_lot` - Faster Mutex/RwLock
- `dashmap` - Production concurrent HashMap
- `lockfree` - Lock-free data structures

**AoC Applications**:
- Parallel puzzle solving
- Multi-threaded input processing
- Concurrent search algorithms

---

## ⚠️ Common Pitfalls

**1. Deadlocks**:
```rust
// ❌ BAD - can deadlock!
let _lock1 = mutex1.lock();
let _lock2 = mutex2.lock();  // If another thread locks in reverse order

// ✅ GOOD - consistent lock ordering
// Always acquire locks in same order
```

**2. Lock Poisoning**:
```rust
// Mutex poisoned if thread panics while holding lock
let data = mutex.lock().unwrap();  // unwrap() panics if poisoned
// Better: handle error explicitly
```

**3. Excessive Locking**:
```rust
// ❌ BAD - holds lock too long
let mut data = mutex.lock().unwrap();
expensive_computation(&data);  // Lock held during computation

// ✅ GOOD - minimize critical section
let value = {
    let data = mutex.lock().unwrap();
    data.clone()  // Copy only what you need
};
expensive_computation(&value);  // Lock released
```

---

### **Day 8: Lock-Free Queue** (Step 8)
**File**: `examples/step8_lockfree_queue.rs`

**Topics**:
- Michael-Scott lock-free queue algorithm
- Compare-and-swap (CAS) operations
- ABA problem and solutions
- Memory ordering: Acquire/Release semantics

**Lock-Free Queue Structure**:
```rust
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

struct Node<T> {
    data: Option<T>,
    next: AtomicPtr<Node<T>>,
}

pub struct LockFreeQueue<T> {
    head: AtomicPtr<Node<T>>,  // Dequeue from head
    tail: AtomicPtr<Node<T>>,  // Enqueue to tail
}

impl<T> LockFreeQueue<T> {
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(Node {
            data: None,
            next: AtomicPtr::new(ptr::null_mut()),
        }));
        
        Self {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
        }
    }
    
    pub fn enqueue(&self, value: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data: Some(value),
            next: AtomicPtr::new(ptr::null_mut()),
        }));
        
        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };
            
            if next.is_null() {
                // Try to link new node at tail
                if unsafe { (*tail).next.compare_exchange(
                    ptr::null_mut(),
                    new_node,
                    Ordering::Release,
                    Ordering::Acquire,
                ).is_ok() } {
                    // Success! Try to swing tail (okay if this fails)
                    let _ = self.tail.compare_exchange(
                        tail,
                        new_node,
                        Ordering::Release,
                        Ordering::Acquire,
                    );
                    return;
                }
            } else {
                // Help other thread: swing tail forward
                let _ = self.tail.compare_exchange(
                    tail,
                    next,
                    Ordering::Release,
                    Ordering::Acquire,
                );
            }
        }
    }
}
```

**ABA Problem**:
```
Thread 1 reads: tail = A
Thread 2: dequeue A, enqueue B, dequeue B, enqueue A (now tail = A again!)
Thread 1: CAS succeeds (tail still A), but it's a DIFFERENT A!
```

**Solutions**:
- Use epoch-based reclamation (see Day 10)
- Tag pointers with version numbers
- Hazard pointers

**Learning Outcomes**:
- ✅ Implement lock-free algorithm with CAS
- ✅ Understand ABA problem
- ✅ Use proper memory ordering (Acquire/Release)

---

### **Day 9: Concurrent Skip List** (Step 9)
**File**: `examples/step9_concurrent_skiplist.rs`

**Topics**:
- Lock-free skip list (probabilistic balanced tree)
- Lazy deletion with marking
- Fine-grained locking alternative
- Sorted concurrent set

**Why Skip List?**
- Easier to implement lock-free than balanced tree (AVL/RB-Tree)
- O(log n) search/insert/delete (probabilistic)
- Better concurrency than tree (less restructuring)

**Structure**:
```rust
const MAX_LEVEL: usize = 32;

struct Node<K, V> {
    key: K,
    value: AtomicPtr<V>,
    next: [AtomicPtr<Node<K, V>>; MAX_LEVEL],  // Tower of forward pointers
    marked: AtomicBool,  // Logically deleted
    fully_linked: AtomicBool,
}

pub struct ConcurrentSkipList<K, V> {
    head: *mut Node<K, V>,
    max_level: AtomicUsize,
}
```

**Lock-Free Search**:
```rust
impl<K: Ord, V> ConcurrentSkipList<K, V> {
    fn find(&self, key: &K) -> Option<&V> {
        let mut level = self.max_level.load(Ordering::Acquire);
        let mut current = self.head;
        
        while level > 0 {
            level -= 1;
            let next = unsafe { (*current).next[level].load(Ordering::Acquire) };
            
            if !next.is_null() && unsafe { (*next).key < *key } {
                current = next;
                level += 1;  // Continue at this level
            }
        }
        
        let next = unsafe { (*current).next[0].load(Ordering::Acquire) };
        if !next.is_null() && unsafe { (*next).key == *key } 
            && !unsafe { (*next).marked.load(Ordering::Acquire) } {
            Some(unsafe { &*(*next).value.load(Ordering::Acquire) })
        } else {
            None
        }
    }
}
```

**Lazy Deletion**:
1. Mark node as deleted (atomically)
2. Unlink from all levels (can happen later)
3. Reclaim memory (epoch-based)

**Learning Outcomes**:
- ✅ Implement probabilistic concurrent data structure
- ✅ Use lazy deletion pattern
- ✅ Understand skip list advantages over trees

---

### **Day 10: Epoch-Based Memory Reclamation** (Step 10)
**File**: `examples/step10_epoch_reclamation.rs`

**Topics**:
- Safe memory reclamation in lock-free structures
- Epoch-based reclamation (EBR)
- crossbeam::epoch crate
- Hazard pointers (alternative)

**The Problem**: Lock-free data structures can't use `Box`/`Arc` for nodes:
```rust
// UNSAFE! Another thread might be reading this node!
let old_node = self.head.swap(new_node, Ordering::AcqRel);
drop(Box::from_raw(old_node));  // ❌ DANGER!
```

**Epoch-Based Reclamation**:
```rust
use crossbeam::epoch::{self, Atomic, Owned, Shared};

pub struct EpochQueue<T> {
    head: Atomic<Node<T>>,
    tail: Atomic<Node<T>>,
}

impl<T> EpochQueue<T> {
    pub fn enqueue(&self, value: T) {
        let guard = epoch::pin();  // Enter current epoch
        
        let new_node = Owned::new(Node {
            data: Some(value),
            next: Atomic::null(),
        });
        
        loop {
            let tail = self.tail.load(Ordering::Acquire, &guard);
            let next = unsafe { tail.deref() }.next.load(Ordering::Acquire, &guard);
            
            if next.is_null() {
                match unsafe { tail.deref() }.next.compare_exchange(
                    Shared::null(),
                    new_node,
                    Ordering::Release,
                    Ordering::Acquire,
                    &guard,
                ) {
                    Ok(_) => {
                        let _ = self.tail.compare_exchange(
                            tail,
                            new_node,
                            Ordering::Release,
                            Ordering::Acquire,
                            &guard,
                        );
                        return;
                    }
                    Err(e) => new_node = e.new,  // Retry with same node
                }
            }
        }
        // guard dropped: safe to reclaim nodes from 2 epochs ago
    }
    
    pub fn dequeue(&self) -> Option<T> {
        let guard = epoch::pin();
        
        loop {
            let head = self.head.load(Ordering::Acquire, &guard);
            let next = unsafe { head.deref() }.next.load(Ordering::Acquire, &guard);
            
            if let Some(next_ref) = unsafe { next.as_ref() } {
                if self.head.compare_exchange(
                    head,
                    next,
                    Ordering::Release,
                    Ordering::Acquire,
                    &guard,
                ).is_ok() {
                    unsafe {
                        guard.defer_destroy(head);  // Schedule for reclamation
                    }
                    return next_ref.data.take();
                }
            } else {
                return None;  // Queue empty
            }
        }
    }
}
```

**How It Works**:
1. **Epochs**: Global counter (0, 1, 2, ...)
2. **Pin**: Thread announces "I'm using epoch N"
3. **Defer**: Mark memory for deletion (not freed yet)
4. **Reclaim**: Free memory from epoch N-2 (safe - no threads there)

**Comparison**:

| Technique | Overhead | Complexity | Use Case |
|-----------|----------|------------|----------|
| Epoch (EBR) | Low | Medium | General lock-free (crossbeam) |
| Hazard Pointers | Medium | High | Fine-grained protection |
| Reference Counting | High | Low | Rare updates (Arc) |
| Garbage Collection | Low* | Low | Managed languages |

**Learning Outcomes**:
- ✅ Understand memory reclamation challenges in lock-free code
- ✅ Use crossbeam::epoch for safe reclamation
- ✅ Implement production-ready lock-free structures

---

## 🎯 Next Steps

After completing this tutorial:
- [ ] **Mission 14**: Formal V-Cycle concurrent data structures
- [ ] Apply to parallel AoC solutions
- [ ] Build thread-safe versions of Mission 1-13 structures
- [ ] Explore async/await integration (Tokio channels, etc.)

---

*Zettelkasten Integration*:
- [[rust-concurrency]] - Thread safety fundamentals
- [[lock-free-algorithms]] - CAS patterns
- [[mission-1]] through [[mission-13]] - Add concurrent versions
- [[rust-book-ch16]] - Fearless Concurrency
- [[rust-book-ch17]] - Async programming

*Created: 2026-01-25*  
*Part of: Mission Track - Advanced Data Structures*
