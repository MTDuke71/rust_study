# Atomic Operations

*See [[atomic-operations-memory-ordering]] for complete coverage*

---

**Atomic operations** are lock-free primitive types (`AtomicBool`, `AtomicUsize`, `AtomicPtr`, etc.) that provide thread-safe operations without `Mutex` overhead using CPU-level instructions.

## Quick Reference

**Core Atomics**:
- `AtomicBool` - Boolean flags
- `AtomicUsize`, `AtomicIsize` - Integer counters
- `AtomicPtr<T>` - Raw pointer atomics
- `AtomicU8`, `AtomicI8` through `AtomicU64`, `AtomicI64`

**Common Operations**:
```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let counter = AtomicUsize::new(0);
counter.fetch_add(1, Ordering::Relaxed);     // Increment
counter.load(Ordering::Acquire);             // Read
counter.store(42, Ordering::Release);        // Write
counter.swap(5, Ordering::AcqRel);           // Exchange
counter.compare_exchange(old, new, ...);     // CAS
```

**When to Use**:
- ✅ Simple counters (statistics, metrics)
- ✅ Flags (shutdown signals, ready states)
- ✅ Lock-free algorithms (advanced)
- ❌ Complex shared state → use `Mutex<T>` instead

## Essential Concepts

1. **Atomicity**: Operation completes fully or not at all
2. **Memory Ordering**: Controls visibility and reordering (`Relaxed`, `Acquire`, `Release`, `SeqCst`)
3. **No Deadlocks**: Lock-free by nature
4. **Performance**: Much faster than `Mutex` for simple operations

## Full Documentation

→ **[[atomic-operations-memory-ordering]]** - Complete guide with:
- Memory ordering detailed explanation
- Acquire/Release synchronization patterns
- Compare-exchange operations
- Atomic vs Mutex performance comparison
- Common pitfalls and solutions
- Practical examples from Ch16

---

## Related Concepts

- [[shared-state-concurrency]] - When to use atomics vs `Mutex<T>`
- [[rust-concurrency-moc]] - Overall concurrency navigation
- [[check-then-execute-pattern]] - Validation patterns that work with atomics
- [[arc-reference-counting]] - Thread-safe reference counting using atomics internally
- [[rust-book-ch16]] - Fearless concurrency chapter

*Links:*
- Referenced in: [[check-then-execute-pattern]], [[rust-concurrency-moc]]
- Full coverage: [[atomic-operations-memory-ordering]]
- Related: [[shared-state-concurrency]], [[arc-reference-counting]]
