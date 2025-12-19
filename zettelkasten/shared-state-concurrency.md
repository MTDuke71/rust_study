# Shared-State Concurrency - Coordinating Access with Arc<Mutex<T>>

*Using `Arc<Mutex<T>>` (and friends) to let multiple Advent of Code worker threads mutate shared state without data races or deadlocks.*

---

## 🎯 **Core Concept**

Shared-state concurrency appears whenever multiple threads must mutate the **same** data structure. Rust's answer is to combine `Arc<T>` for shared ownership with `Mutex<T>` (or `RwLock<T>`) for synchronized interior mutability. This pattern keeps state in one place while guaranteeing compile-time thread safety plus runtime mutual exclusion. For AoC pipelines it powers global scoreboards, memoization caches, and rate-limited resource pools that every worker needs.

## 🧠 **Mental Models**

- **Key to the Records Room**: `Arc` hands out many identical keys (clones) to the same archive. `Mutex` is the guard who only lets one person inside at a time.
- **Command Queue Ledger**: Everyone writes to the same ledger, but only while holding the pen. Dropping the lock immediately hands the pen to the next worker.
- **One-at-a-Time Checkout**: `Arc<Mutex<T>>` is like a library checkout desk—many patrons (threads) share the collection, but each checkout is exclusive and leaves a paper trail (lock poisoning) if anything goes wrong.

## 🔍 **Detailed Content**

### **Arc<Mutex<T>> Essentials**

1. Wrap shared data: `let data = Arc::new(Mutex::new(initial_state));`
2. Clone the `Arc` into each thread so everyone references the same heap allocation.
3. Lock as late and as briefly as possible: `let mut state = data.lock().unwrap();` (guard unlocks when dropped).
4. Be mindful of poisoning: if a thread panics while holding the lock, future calls receive `PoisonError`. Either recover with `into_inner()` or treat as fatal.
5. Promote read-heavy workloads to `RwLock` to allow concurrent readers.

### **Complete Runnable Example: AoC Frequency Counter**

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Reading {
    sensor_id: usize,
    value: i64,
}

fn aggregate_readings(readings: Vec<Reading>, worker_count: usize) -> HashMap<usize, i64> {
    let shared = Arc::new(Mutex::new(HashMap::<usize, i64>::new()));

    let chunks: Vec<_> = readings.chunks((readings.len() / worker_count).max(1)).map(|c| c.to_vec()).collect();

    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| {
            let state = Arc::clone(&shared);
            thread::spawn(move || {
                for reading in chunk {
                    let mut totals = state.lock().unwrap();
                    *totals.entry(reading.sensor_id).or_default() += reading.value;
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::into_inner(shared)
        .expect("no outstanding references")
        .into_inner()
        .expect("mutex not poisoned")
}

fn main() {
    let data = vec![
        Reading { sensor_id: 0, value: 5 },
        Reading { sensor_id: 1, value: 3 },
        Reading { sensor_id: 0, value: 7 },
        Reading { sensor_id: 1, value: -1 },
        Reading { sensor_id: 2, value: 9 },
    ];

    let totals = aggregate_readings(data, 3);
    println!("totals: {totals:?}");
}
```

**Why this matches AoC workloads**: Many puzzles (e.g., 2015 Day 7 circuits, 2023 Day 20 pulses) have central registries updated by parallel evaluators. Workers independently compute contributions then merge them into a shared map without copying entire structures per thread.

### **Operational Risks & Mitigations**

| Risk | Example | Mitigation |
|------|---------|------------|
| **Deadlock** | Thread A locks registry, Thread B locks cache, then they wait on each other | Establish a lock ordering discipline, or consolidate data into one mutexed struct |
| **Long Critical Sections** | Parsing + computation happen while holding the lock | Do heavy work outside the lock; capture inputs, release, then compute |
| **Excessive Contention** | Hundreds of workers compete for one mutex | Use sharded maps (`Vec<Mutex<_>>`), `DashMap`, or switch to message-passing-channels |
| **Lock Poisoning** | Worker panics mid-update | Wrap `lock()` with `unwrap_or_else(\|e\| e.into_inner())` and log the failure |

### **When to Prefer Shared State vs Message Passing**

- Choose **shared-state** when every thread must see and mutate a single data structure (global memoization cache, union-find parents table, metrics registry).
- Choose **message passing** for embarrassingly parallel pipelines or when you want *ownership transfer* instead of *shared mutation*.
- Hybrid approaches often shine: use channels to distribute work and Arc<Mutex<T>> only for the final aggregation step.

## 💡 **Key Takeaways**

- `Arc<Mutex<T>>` is the idiomatic unlock for shared mutable state; it composes with any `T` that is `Send`.
- Keep lock scopes minimal and deterministic to avoid contention and deadlocks.
- Poison handling is a feature, not a nuisance—log it to supercharge [[deterministic-debugging]].
- Sharding plus `Arc<Vec<Mutex<_>>>` scales AoC workloads to all cores without rewriting algorithms.
- Always balance against `[[message-passing-channels]]` to ensure you're not defaulting to shared state unnecessarily.

## 🔗 **Integration Points**

### **Builds On**

- [[rust-threading-basics]] - Thread spawning, JoinHandle patterns, and ownership moves
- [[ownership]] - Transfer semantics matter when moving data into the shared structure
- [[Send and Sync Deep Dive]] - Guarantees for sharing `Arc<Mutex<T>>` across threads

### **Enables**

- [[thread-pool-pattern]] - Arc<Mutex<Receiver>> for sharing work queue across workers
- [[sync-send-traits]] - Analyzing whether custom AoC data structures stay thread-safe after wrapping in locks
- [[AoC Pattern Library]] - Adds a reusable shared-state aggregation recipe
- [[deterministic-debugging]] - Centralized logging/state capture for multi-threaded scenarios

### **Related Concepts**

- [[atomic-operations-memory-ordering]] - Lock-free alternative to Mutex for simple counters and flags
- [[message-passing-channels]] - Ownership-transfer alternative without shared mutation
- [[Performance Patterns]] - Measuring contention, batching updates, and using lock-free data structures
- [[hashmap-ownership-patterns]] - Strategies for storing owned vs borrowed data inside the shared registry

---

*Tags: #concurrency #pattern #aoc #rust-book #intermediate*

*Links: [[zettel-index]] | [[rust-threading-basics]] | [[message-passing-channels]] | [[sync-send-traits]] | [[deterministic-debugging]] | [[AoC Pattern Library]] | [[../rust_book/Ch21/README]]*
