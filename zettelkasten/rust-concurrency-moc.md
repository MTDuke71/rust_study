# Rust Concurrency - Map of Content

*Navigation: [[zettel-index]]*

---

## Overview

This Map of Content (MOC) organizes all concurrency-related concepts in Rust, spanning both **async/await cooperative multitasking** and **thread-based parallelism**. Rust provides multiple concurrency models optimized for different workloads: async for I/O-bound work, threads for CPU-bound work, and hybrid approaches for complex applications.

---

## 🔄 Async/Await (Cooperative Concurrency)

### Foundations

- [[async-await-basics]] - Futures, `.await`, runtime execution model, non-blocking I/O
- [[future-trait-deep-dive]] - Future trait internals, Pin/Unpin, Stream trait (Ch17.5)
- [[futures-and-polling]] - Future trait, `poll()` mechanism, `Poll::Ready`/`Pending`
- [[runtime-executors]] - tokio, async-std, smol comparison and selection guide
- [[pin-and-unpin]] - Memory safety for self-referential futures, `Pin<T>`

### Concurrency Patterns

- [[async-concurrency]] - `select()`, `join!()`, `spawn_task()`, racing and composition
- [[async-streams]] - Async iteration, `Stream` trait, backpressure handling
- [[async-channels]] - Message passing between tasks (mpsc, broadcast, watch)
- [[async-timeouts]] - Timeout patterns, cancellation, deadline handling
- [[async-retry-patterns]] - Retry with exponential backoff, circuit breakers

### Architecture

- [[task-spawning-strategies]] - When to spawn vs inline, task granularity
- [[structured-concurrency]] - Parent-child task relationships, scoped tasks
- [[graceful-shutdown]] - Coordinated task termination, cleanup patterns
- [[blocking-in-async]] - `spawn_blocking()` for CPU-bound work in async contexts

### Error Handling

- [[async-error-propagation]] - `?` operator in async, `Result<T, E>` patterns
- [[timeout-error-handling]] - Distinguishing timeout vs operation errors
- [[concurrent-error-aggregation]] - Collecting errors from `join!()`, `try_join!()`

---

## 🧵 Thread-Based Concurrency (Preemptive Parallelism)

### Foundations

- [[rust-threading-basics]] - `std::thread`, `JoinHandle`, thread creation and joining
- [[thread-safety]] - `Send` and `Sync` traits, thread-safe types
- [[data-races]] - Race conditions, undefined behavior, prevention strategies
- [[thread-local-storage]] - Thread-local data, `thread_local!` macro

### Synchronization Primitives

- [[mutex-and-rwlock]] - `Mutex<T>`, `RwLock<T>`, lock acquisition patterns
- [[arc-reference-counting]] - `Arc<T>` for shared ownership across threads
- [[atomic-operations]] - `AtomicUsize`, `AtomicBool`, memory ordering
- [[condition-variables]] - `Condvar`, waiting and notification patterns
- [[barriers-and-latches]] - Synchronization points for multiple threads

### Message Passing

- [[mpsc-channels]] - Multi-producer, single-consumer channels
- [[crossbeam-channels]] - Advanced channel patterns, select operations
- [[flume-channels]] - MPMC channels, performance characteristics

### Parallelism

- [[rayon-parallel-iterators]] - Data parallelism, `par_iter()`, work stealing
- [[thread-pools]] - Thread pool patterns, `threadpool` crate
- [[scoped-threads]] - Borrowing across threads with `thread::scope()`

### Common Pitfalls

- [[deadlock-prevention]] - Lock ordering, timeout strategies, detection
- [[thread-starvation]] - Priority inversion, fairness concerns
- [[false-sharing]] - Cache line contention, padding strategies

---

## ⚖️ Choosing Concurrency Models

### Decision Guides

- [[sync-vs-async]] - When to use threads vs async, workload characteristics
- [[cpu-bound-vs-io-bound]] - Identifying workload types, measurement techniques
- [[hybrid-concurrency]] - Combining async I/O with threaded CPU work

### Performance Considerations

- [[concurrency-overhead]] - Context switching costs, memory overhead
- [[scalability-patterns]] - Horizontal scaling, sharding strategies
- [[concurrency-benchmarking]] - Measuring throughput, latency, contention

### Real-World Patterns

- [[web-server-concurrency]] - HTTP server patterns, connection handling
- [[database-connection-pools]] - Pool sizing, connection lifecycle
- [[background-job-processing]] - Queue workers, rate limiting

---

## 🔧 Advanced Topics

### Memory Models

- [[rust-memory-model]] - Happens-before, synchronizes-with relationships
- [[memory-ordering]] - Acquire, Release, SeqCst, Relaxed semantics
- [[lock-free-programming]] - Compare-and-swap, ABA problem

### Custom Primitives

- [[custom-futures]] - Implementing `Future` trait, state machines
- [[custom-executors]] - Building async runtimes, waker contracts
- [[custom-synchronization]] - Spinlocks, semaphores, custom primitives

### Integration Patterns

- [[async-trait-objects]] - `Box<dyn Future>`, `Pin<Box<...>>`, dynamic dispatch
- [[ffi-thread-safety]] - C interop, thread safety across FFI boundaries
- [[rust-concurrency-vs-autosar]] - Mapping Rust patterns to automotive AUTOSAR

---

## 📚 Concept Hierarchies

### Async Progression

```
Futures (lazy work tickets)
    ↓
.await (yield points)
    ↓
Runtime (executor/scheduler)
    ↓
Concurrency Patterns (select, join, spawn)
    ↓
Error Handling & Cancellation
    ↓
Production Patterns (timeouts, retries, shutdown)
```

### Thread Progression

```
Thread Creation (std::thread)
    ↓
Thread Safety (Send/Sync)
    ↓
Synchronization (Mutex, RwLock, Arc)
    ↓
Message Passing (channels)
    ↓
Parallelism (rayon, thread pools)
    ↓
Advanced (atomics, lock-free)
```

---

## 🎯 Learning Path Recommendations

### Beginner (Foundation)

1. Start with [[rust-threading-basics]] - Understand preemptive multithreading
2. Learn [[mutex-and-rwlock]] - Basic synchronization primitives
3. Study [[async-await-basics]] - Async fundamentals and Future trait
4. Practice [[async-concurrency]] - select, join, spawn patterns
5. Understand [[sync-vs-async]] - When to use which model

### Intermediate (Patterns)

1. Master [[rayon-parallel-iterators]] - Data parallelism for CPU work
2. Learn [[async-streams]] - Async iteration and backpressure
3. Study [[mpsc-channels]] - Message passing between threads/tasks
4. Practice [[async-error-propagation]] - Error handling in concurrent code
5. Explore [[hybrid-concurrency]] - Combining async and threads

### Advanced (Deep Dive)

1. Study [[atomic-operations]] - Lock-free programming foundations
2. Master [[pin-and-unpin]] - Memory safety in self-referential futures
3. Learn [[custom-futures]] - Implementing Future trait from scratch
4. Understand [[memory-ordering]] - Advanced synchronization semantics
5. Explore [[lock-free-programming]] - High-performance concurrent structures

---

## 🔗 Cross-Cutting Concerns

### Performance

- [[concurrency-overhead]] - Understanding costs
- [[concurrency-benchmarking]] - Measurement techniques
- [[scalability-patterns]] - Horizontal scaling strategies

### Safety

- [[thread-safety]] - Send/Sync traits
- [[data-races]] - Prevention and detection
- [[deadlock-prevention]] - Lock ordering and timeouts

### Architecture

- [[task-spawning-strategies]] - Granularity decisions
- [[structured-concurrency]] - Lifecycle management
- [[graceful-shutdown]] - Clean termination patterns

---

## 🚀 Practical Applications

### Web Development

- [[web-server-concurrency]] - HTTP connection handling with tokio/actix
- [[database-connection-pools]] - Managing DB connections in async context
- [[api-rate-limiting]] - Token bucket, leaky bucket patterns

### Systems Programming

- [[file-io-concurrency]] - Async file I/O, buffering strategies
- [[network-protocols]] - Implementing async network protocols
- [[os-integration]] - Signals, epoll/kqueue, platform-specific concerns

### Data Processing

- [[rayon-parallel-iterators]] - Parallel data transformations
- [[background-job-processing]] - Queue-based worker patterns
- [[stream-processing]] - Real-time data pipeline patterns

---

## 📖 External Resources

### Official Documentation

- [Rust Book Chapter 16](https://doc.rust-lang.org/book/ch16-00-concurrency.html) - Fearless Concurrency
- [Rust Book Chapter 17](https://doc.rust-lang.org/book/ch17-00-async-await.html) - Async and Await
- [Async Book](https://rust-lang.github.io/async-book/) - Comprehensive async programming guide
- [Nomicon - Concurrency](https://doc.rust-lang.org/nomicon/concurrency.html) - Advanced memory model

### Key Crates

- **tokio** - Most popular async runtime (production-ready)
- **async-std** - Async standard library (familiar API)
- **rayon** - Data parallelism library (CPU-bound work)
- **crossbeam** - Advanced concurrent data structures
- **parking_lot** - Faster synchronization primitives

---

## 🏷️ Related MOCs

- [[Collections MOC]] - Thread-safe collections (`DashMap`, concurrent queues)
- [[Error Handling Patterns]] - Concurrent error handling strategies
- [[Ownership and Borrowing]] - Foundation for thread safety (`Send`/`Sync`)
- [[Traits]] - `Future`, `Send`, `Sync`, `Stream` trait implementations

---

## 📝 Study Notes

### AUTOSAR Integration Perspective

For developers with AUTOSAR (automotive) background:

| **Rust Concept** | **AUTOSAR Equivalent** | **Key Difference** |
|------------------|------------------------|-------------------|
| async fn | Runnable callback | Rust: pauseable; AUTOSAR: run-to-completion |
| Runtime (tokio) | RTE (Runtime Environment) | Both schedule/execute work units |
| .await yield | Runnable completion | Rust: mid-function yield; AUTOSAR: end-of-runnable |
| select() | First-available event | Similar racing semantics |
| spawn_task() | Independent runnable | Different periodic rates (10ms vs 100ms) |
| Future | Event/work ticket | Promise of future completion |
| Thread | Separate ECU core | True parallelism, heavy resource cost |

**Key Insight:** Async Rust is **component orchestration** on single thread (like AUTOSAR RTE scheduling runnables), while Rust threads are true parallelism (like multi-core ECU).

---

## ✅ Checklist: Concurrency Mastery

### Async Fundamentals

- [ ] Understand Future trait and lazy evaluation
- [ ] Master `.await` as yield points
- [ ] Know when async blocks execute vs futures
- [ ] Distinguish blocking vs non-blocking operations
- [ ] Understand runtime role (tokio/async-std)

### Concurrency Patterns

- [ ] Use `select()` for racing futures
- [ ] Use `join!()` for concurrent batch operations
- [ ] Use `spawn_task()` for background work
- [ ] Handle timeouts with nested select
- [ ] Implement interrupt-and-resume patterns

### Thread Fundamentals

- [ ] Create and join threads with `std::thread`
- [ ] Understand `Send` and `Sync` traits
- [ ] Use `Arc<Mutex<T>>` for shared state
- [ ] Avoid deadlocks with lock ordering
- [ ] Use `mpsc` channels for message passing

### Performance

- [ ] Choose async for I/O-bound work
- [ ] Choose threads/rayon for CPU-bound work
- [ ] Measure concurrency overhead
- [ ] Optimize task granularity
- [ ] Profile contention and bottlenecks

### Production Readiness

- [ ] Implement graceful shutdown
- [ ] Add timeout handling
- [ ] Implement retry with backoff
- [ ] Handle concurrent errors properly
- [ ] Monitor task/thread health

---

*Tags: #moc #rust #concurrency #async #threads #parallelism #tokio #rayon*

*Links: [[zettel-index]] | [[async-await-basics]] | [[async-concurrency]] | [[rust-threading-basics]] | [[rayon-parallel-iterators]]*

*Last Updated: November 21, 2025*
