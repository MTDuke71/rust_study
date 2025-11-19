# Atomic Operations & Memory Ordering - Lock-Free Concurrency

*Part of [[rust-book-ch16]] - [[shared-state-concurrency]]*

## 🎯 **Core Concept**

**Atomics** are lock-free primitive types that provide thread-safe operations without `Mutex` overhead. They use CPU-level instructions to guarantee:
1. **Atomicity**: Operation completes fully or not at all (no partial updates)
2. **Visibility**: Changes are visible to other threads
3. **Ordering**: Control how operations are reordered by compiler/CPU

**Why They Exist**: `Arc<Mutex<i32>>` for a simple counter is massive overkill - you're paying for lock acquisition/release, potential thread blocking, poison checking, and heap allocation. Atomics solve this with hardware instructions.

## 🔍 **Detailed Content**

### **Memory Ordering - The Deep Part**

Every atomic operation requires an **ordering** parameter that controls synchronization guarantees:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let counter = AtomicUsize::new(0);

// Different ordering guarantees:
counter.fetch_add(1, Ordering::Relaxed);  // Fastest, no synchronization
counter.fetch_add(1, Ordering::Acquire);  // Synchronize reads
counter.fetch_add(1, Ordering::Release);  // Synchronize writes
counter.fetch_add(1, Ordering::AcqRel);   // Both acquire + release
counter.fetch_add(1, Ordering::SeqCst);   // Strongest, total ordering
```

### **Ordering Reference Table**

| Ordering | Use When | Cost | Guarantees |
|----------|----------|------|------------|
| **Relaxed** | Independent counters, statistics | Lowest | Just atomicity, no synchronization |
| **Acquire** | Reading shared state set by another thread | Low | Reads after this see writes before Release |
| **Release** | Writing shared state for another thread | Low | Writes before this visible after Acquire |
| **AcqRel** | Read-modify-write (fetch_add, compare_exchange) | Medium | Both Acquire + Release |
| **SeqCst** | When you need global ordering across all threads | Highest | Total order visible to all threads |

### **Why Memory Ordering Matters - Practical Example**

```rust
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

// ❌ WRONG - Relaxed can cause data races with non-atomic data
fn broken_synchronization() {
    let ready = Arc::new(AtomicBool::new(false));
    let data = Arc::new(AtomicUsize::new(0));

    let ready_clone = ready.clone();
    let data_clone = data.clone();

    thread::spawn(move || {
        data_clone.store(42, Ordering::Relaxed);
        ready_clone.store(true, Ordering::Relaxed); 
        // BUG: Reader might see ready=true but data=0 due to reordering
    });

    while !ready.load(Ordering::Relaxed) {}
    // Might read data=0 even though ready=true!
}

// ✅ CORRECT - Release/Acquire synchronization
fn correct_synchronization() {
    let ready = Arc::new(AtomicBool::new(false));
    let data = Arc::new(AtomicUsize::new(0));

    let ready_clone = ready.clone();
    let data_clone = data.clone();

    thread::spawn(move || {
        data_clone.store(42, Ordering::Relaxed);
        ready_clone.store(true, Ordering::Release); 
        // Release: all writes before this are visible to Acquire
    });

    while !ready.load(Ordering::Acquire) {} 
    // Acquire: reads after this see all writes before Release
    assert_eq!(data.load(Ordering::Relaxed), 42); // Guaranteed to see 42
}
```

**The Key Insight**: `Relaxed` only guarantees the atomic operation itself is indivisible. It doesn't prevent the CPU/compiler from reordering other operations around it. `Acquire`/`Release` create **synchronization points** where memory visibility is guaranteed.

## 🛠️ **Common Atomic Patterns**

### **Pattern 1: Simple Counter (Relaxed is Fine)**

When you just need to count things independently without coordinating other data:

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

let hits = Arc::new(AtomicUsize::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let hits = Arc::clone(&hits);
    handles.push(thread::spawn(move || {
        for _ in 0..1000 {
            hits.fetch_add(1, Ordering::Relaxed); // Just counting, order doesn't matter
        }
    }));
}

for handle in handles {
    handle.join().unwrap();
}

println!("Total hits: {}", hits.load(Ordering::Relaxed));
```

**Why Relaxed works**: Each counter increment is independent. We don't care which thread's increment happens "first" - we just want the final total.

### **Pattern 2: Flag Synchronization (Need Acquire/Release)**

When an atomic flag signals that other data is ready:

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![0; 1000]); // Shared non-atomic data
let ready = Arc::new(AtomicBool::new(false));

let data_clone = Arc::clone(&data);
let ready_clone = Arc::clone(&ready);

// Writer thread
thread::spawn(move || {
    // Modify shared data (unsafe without proper synchronization)
    // ... modify data_clone ...
    
    ready_clone.store(true, Ordering::Release); 
    // Release: ensures all data modifications are visible
});

// Reader thread
while !ready.load(Ordering::Acquire) {
    // Acquire: ensures we see all modifications before Release
    std::hint::spin_loop(); // Better than busy waiting
}
// Now safe to read data
```

**Why Acquire/Release needed**: The flag coordinates access to non-atomic data. `Release` ensures all writes are visible, `Acquire` ensures we see those writes.

### **Pattern 3: Compare-and-Swap (Lock-Free Algorithms)**

Atomically update only if current value matches expected:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let value = AtomicUsize::new(10);

// Try to update from 10 to 20
match value.compare_exchange(
    10,                      // Expected current value
    20,                      // New value if current == expected
    Ordering::SeqCst,        // Success ordering (when swap happens)
    Ordering::SeqCst         // Failure ordering (when current != expected)
) {
    Ok(old) => println!("Successfully updated from {}", old),
    Err(actual) => println!("Failed - actual value was {}", actual),
}

// Common pattern: retry until success
let mut current = value.load(Ordering::SeqCst);
loop {
    let new = current + 1;
    match value.compare_exchange(current, new, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(_) => break,
        Err(actual) => current = actual, // Retry with updated value
    }
}
```

**Use case**: Building lock-free data structures (stacks, queues). This is the primitive operation for concurrent algorithms.

### **Pattern 4: Spin Lock (Educational, Don't Use in Production)**

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    fn new() -> Self {
        Self { locked: AtomicBool::new(false) }
    }
    
    fn lock(&self) {
        // Spin until we successfully swap false -> true
        while self.locked.compare_exchange(
            false, 
            true, 
            Ordering::Acquire,  // Acquire when we get the lock
            Ordering::Relaxed   // Relaxed on retry
        ).is_err() {
            std::hint::spin_loop(); // CPU hint: we're spinning
        }
    }
    
    fn unlock(&self) {
        self.locked.store(false, Ordering::Release); // Release the lock
    }
}
```

**Why this pattern matters**: Demonstrates how `Mutex` works internally (though real `Mutex` is smarter - it sleeps instead of spinning).

## 📊 **Atomics vs Mutex - Decision Guide**

| Scenario | Use This | Why |
|----------|----------|-----|
| Simple counter/flag | `AtomicUsize`/`AtomicBool` | 10-100x faster than Mutex |
| Shared struct with multiple fields | `Mutex<T>` | Can't atomically update multiple fields |
| Read-heavy data | `RwLock<T>` | Multiple concurrent readers |
| Complex invariants | `Mutex<T>` | Need to update multiple things together atomically |
| Lock-free queue/stack | Custom with atomics | Advanced, avoid unless necessary |
| Statistics/metrics collection | Atomics with `Relaxed` | Minimal overhead |

**Rule of Thumb**: If you need to protect more than one piece of data with consistent invariants, use `Mutex`. If it's a single primitive value, consider atomics.

## 🧬 **Available Atomic Types**

```rust
use std::sync::atomic::*;

// Boolean
AtomicBool

// Signed integers
AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize

// Unsigned integers  
AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize

// Raw pointers (advanced)
AtomicPtr<T>
```

**Notable Absence**: No `AtomicF32` or `AtomicF64` - floating point doesn't have portable atomic operations in hardware. Use `AtomicU32`/`AtomicU64` with `f32::from_bits()`/`f64::from_bits()` if needed.

## ⚡ **Performance Characteristics**

```rust
// Mission 5 HashMap: Atomic counters for statistics
use std::sync::atomic::{AtomicUsize, Ordering};

struct HashMapStats {
    insertions: AtomicUsize,
    collisions: AtomicUsize,
    lookups: AtomicUsize,
}

impl HashMapStats {
    fn record_insertion(&self) {
        self.insertions.fetch_add(1, Ordering::Relaxed);
    }
    
    fn record_collision(&self) {
        self.collisions.fetch_add(1, Ordering::Relaxed);
    }
    
    fn get_stats(&self) -> (usize, usize, usize) {
        (
            self.insertions.load(Ordering::Relaxed),
            self.collisions.load(Ordering::Relaxed),
            self.lookups.load(Ordering::Relaxed),
        )
    }
}

// This is MUCH faster than Arc<Mutex<(usize, usize, usize)>>
// - No lock contention
// - No blocking
// - No allocation
// - Cache-line friendly (with proper padding)
```

**Benchmark Reality** (typical x86-64):
- `Relaxed` atomic operation: ~1-2 CPU cycles
- `SeqCst` atomic operation: ~10-20 CPU cycles  
- Uncontended `Mutex` lock: ~25-50 cycles
- Contended `Mutex` lock: 1000+ cycles (due to OS scheduler)

## 🔐 **Thread Safety: Send and Sync**

All atomic types implement both `Send` and `Sync`:

```rust
// ✅ Valid - AtomicUsize is Sync
let counter = Arc::new(AtomicUsize::new(0));

// ❌ Invalid - Cell<T> is NOT Sync
// let counter = Arc::new(Cell::new(0)); // Won't compile

// ❌ Invalid - RefCell<T> is NOT Sync
// let counter = Arc::new(RefCell::new(0)); // Won't compile
```

**Why atomics are `Sync`**: The hardware guarantees make them safe to share via `&AtomicT` across threads. `Cell`/`RefCell` lack these hardware-level guarantees.

This is the key insight from the Rust Book: "atomics work like primitive types but are safe to share across threads."

## 📚 **Learning Resources**

- **Official Documentation**: [std::sync::atomic](https://doc.rust-lang.org/std/sync/atomic/index.html)
  - Module-level docs explain memory ordering models in detail
  - `Ordering` enum documentation covers formal semantics
  - Each atomic type includes practical examples

- **Related Zettelkasten Pages**:
  - [[shared-state-concurrency]] - When to use Mutex vs atomics
  - [[Send and Sync Deep Dive]] - Why atomics implement Sync
  - [[interior-mutability]] - Comparison with Cell/RefCell
  - [[Performance Patterns]] - Lock-free optimization techniques

## 🎯 **Practical Guidelines**

### **90% Use Case: Relaxed for Counters**

```rust
// Statistics, metrics, counters - independent values
stats.requests.fetch_add(1, Ordering::Relaxed);
stats.errors.fetch_add(1, Ordering::Relaxed);
stats.bytes_sent.fetch_add(n, Ordering::Relaxed);
```

### **9% Use Case: Acquire/Release for Flags**

```rust
// Coordinating access to other data
completed.store(true, Ordering::Release);  // Writer
while !completed.load(Ordering::Acquire) {} // Reader
```

### **1% Use Case: SeqCst When Unsure**

```rust
// Complex multi-threaded algorithms where you need global ordering
// Or when you can't reason about the ordering requirements
value.store(42, Ordering::SeqCst);
```

### **When to Avoid Atomics**

- **Multiple related fields**: Use `Mutex<Struct>` instead
- **Complex invariants**: `Mutex` ensures atomicity across operations
- **You're not sure**: `Mutex` is simpler and safer
- **Premature optimization**: Profile first, optimize later

**The Book's Advice is Correct**: Memory ordering is genuinely complex (PhD-level topic in concurrent algorithms). Most Rust developers use `Mutex` by default and only reach for atomics when profiling shows lock contention.

## 🔬 **Real-World Examples**

### **AoC Performance Tracking**

```rust
use std::sync::atomic::{AtomicU64, Ordering};

struct SolutionMetrics {
    iterations: AtomicU64,
    cache_hits: AtomicU64,
    cache_misses: AtomicU64,
}

impl SolutionMetrics {
    fn record_iteration(&self) {
        self.iterations.fetch_add(1, Ordering::Relaxed);
    }
    
    fn record_cache_access(&self, hit: bool) {
        if hit {
            self.cache_hits.fetch_add(1, Ordering::Relaxed);
        } else {
            self.cache_misses.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    fn hit_rate(&self) -> f64 {
        let hits = self.cache_hits.load(Ordering::Relaxed) as f64;
        let misses = self.cache_misses.load(Ordering::Relaxed) as f64;
        hits / (hits + misses)
    }
}
```

### **Mission 5 Hash Map Collision Tracking**

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    collision_count: AtomicUsize, // Track collisions without locking
}

impl<K, V> HashMap<K, V> {
    fn insert(&mut self, key: K, value: V) {
        let bucket = &mut self.buckets[hash(&key) % self.buckets.len()];
        if !bucket.is_empty() {
            self.collision_count.fetch_add(1, Ordering::Relaxed);
        }
        bucket.push((key, value));
    }
    
    pub fn collision_rate(&self) -> f64 {
        let collisions = self.collision_count.load(Ordering::Relaxed) as f64;
        let total_items = self.len() as f64;
        collisions / total_items
    }
}
```

## 🎓 **Key Takeaways**

1. **Atomics are hardware-supported lock-free primitives** - use CPU instructions instead of OS locks
2. **Memory ordering controls synchronization**, not just atomicity
3. **`Relaxed` for independent operations**, `Acquire`/`Release` for coordination
4. **Start with `Mutex`, profile, THEN consider atomics** - premature optimization introduces subtle bugs
5. **Atomics are `Sync`** - safe to share `&AtomicT` across threads, unlike `Cell`/`RefCell`
6. **Compare-exchange is the primitive for lock-free algorithms** - but hard to get right
7. **The Rust Book simplifies for good reason** - full memory models are genuinely complex

**Bottom Line**: For most concurrent Rust code, `Arc<Mutex<T>>` is the right choice. Reach for atomics when:
- Profiling shows lock contention on a single primitive value
- You're implementing a lock-free data structure (rare)
- You need wait-free statistics/metrics collection

## 📎 **Related Documentation**

*Links:*
- [[shared-state-concurrency]] - Arc<Mutex<T>> patterns and when to use locks
- [[Send and Sync Deep Dive]] - Thread safety marker traits and atomic implementations
- [[interior-mutability]] - Cell/RefCell vs Mutex vs Atomics comparison
- [[Performance Patterns]] - Lock-free optimization and contention reduction
- [[handles-resource-abstraction]] - Thread handles and concurrent patterns
- [[rust-threading-basics]] - Thread spawning and ownership fundamentals
- [[rust-book-ch16]] - Fearless Concurrency chapter overview

*Tags: #concurrency #atomics #memory-ordering #lock-free #performance #thread-safety #sync #hardware #optimization #advanced*
