# Async vs Threads Decision Guide

*Navigation: [[zettel-index]] | [[rust-concurrency-moc]] | [[async-await-basics]] | [[rust-threading-basics]]*

---

## Overview

Choosing between **async/await** and **threads** is a fundamental architectural decision in Rust. Each approach has distinct characteristics that make it better suited for different workloads. This guide provides a decision framework based on Rust Book Chapter 17.6 concepts.

**Source**: Rust Book Chapter 17.6 - "Futures, Tasks, and Threads"  
**Related Code**: `rust_book/Ch17/async_traits/`

---

## The Core Trade-off

| Aspect | Async/Await | Threads |
|--------|-------------|---------|
| **Concurrency Model** | Cooperative (yields at `.await`) | Preemptive (OS schedules) |
| **Resource Cost** | Lightweight tasks (~KB stack) | Heavy OS threads (~MB stack) |
| **Scalability** | Thousands of concurrent tasks | Hundreds of threads |
| **Best For** | I/O-bound work (network, files) | CPU-bound work (computation) |
| **Blocking** | Must never block | Blocking is fine |
| **Parallelism** | Single-threaded by default | True parallelism |

---

## Decision Tree

```
                    ┌─────────────────────────┐
                    │   What type of work?    │
                    └───────────┬─────────────┘
                                │
              ┌─────────────────┼─────────────────┐
              ▼                 ▼                 ▼
        ┌──────────┐     ┌──────────────┐   ┌──────────────┐
        │ I/O-bound│     │  CPU-bound   │   │    Mixed     │
        │ (waiting)│     │ (computing)  │   │   workload   │
        └────┬─────┘     └──────┬───────┘   └──────┬───────┘
             │                  │                  │
             ▼                  ▼                  ▼
      ┌────────────┐    ┌────────────┐     ┌────────────────┐
      │   ASYNC    │    │  THREADS   │     │ ASYNC + SPAWN  │
      │  Best for  │    │  Best for  │     │  spawn_blocking│
      │  network,  │    │  number    │     │  for CPU work  │
      │  file I/O  │    │  crunching │     │  in async ctx  │
      └────────────┘    └────────────┘     └────────────────┘
```

---

## When to Use Async

### ✅ **Use Async For:**

1. **Network Operations**
   - HTTP requests, API calls
   - Database queries
   - WebSocket connections

2. **High Concurrency I/O**
   - Handling thousands of connections
   - Chat servers, web servers
   - Event-driven systems

3. **Waiting on External Resources**
   - File I/O (with async runtime support)
   - Timers and delays
   - Inter-process communication

```rust
// Perfect for async: many concurrent network requests
async fn fetch_all_urls(urls: Vec<&str>) -> Vec<String> {
    let futures: Vec<_> = urls.iter()
        .map(|url| fetch_url(url))
        .collect();
    
    // All requests run concurrently, not sequentially
    futures::future::join_all(futures).await
}
```

### ⚠️ **Async Anti-patterns:**

```rust
// DON'T: CPU-intensive work blocks the async runtime
async fn bad_example() {
    let result = expensive_computation();  // Blocks entire runtime!
    println!("{}", result);
}

// DON'T: Blocking I/O in async context
async fn also_bad() {
    let data = std::fs::read_to_string("file.txt")?;  // Blocks!
    // Use tokio::fs::read_to_string instead
}
```

---

## When to Use Threads

### ✅ **Use Threads For:**

1. **CPU-Bound Computation**
   - Number crunching, cryptography
   - Image/video processing
   - Scientific calculations

2. **Parallelism on Multiple Cores**
   - Dividing work across cores
   - Independent subtasks
   - Data parallelism (processing chunks)

3. **Blocking Operations**
   - Synchronous library calls
   - Legacy code integration
   - System calls that must block

```rust
use std::thread;

// Perfect for threads: CPU-intensive parallel work
fn parallel_computation(data: Vec<i64>) -> i64 {
    let chunk_size = data.len() / num_cpus::get();
    let handles: Vec<_> = data
        .chunks(chunk_size)
        .map(|chunk| {
            let chunk = chunk.to_vec();
            thread::spawn(move || {
                chunk.iter().map(|n| expensive_calc(*n)).sum::<i64>()
            })
        })
        .collect();
    
    handles.into_iter()
        .map(|h| h.join().unwrap())
        .sum()
}
```

### ⚠️ **Thread Anti-patterns:**

```rust
// DON'T: Spawn a thread per request (doesn't scale)
fn handle_connections(listener: TcpListener) {
    for stream in listener.incoming() {
        thread::spawn(|| handle_client(stream));  // 10,000 threads?!
    }
}

// BETTER: Use async for connection handling
async fn handle_connections_async(listener: TcpListener) {
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(handle_client(stream));  // Lightweight tasks
    }
}
```

---

## Hybrid Approach: spawn_blocking

When you need to do CPU work inside an async context, use `spawn_blocking`:

```rust
async fn process_with_cpu_work(data: Vec<u8>) -> Result<ProcessedData> {
    // I/O: async
    let raw = fetch_from_network().await?;
    
    // CPU: offload to thread pool
    let processed = tokio::task::spawn_blocking(move || {
        expensive_cpu_computation(raw)
    }).await?;
    
    // I/O: back to async
    save_to_database(processed).await?;
    
    Ok(processed)
}
```

**How spawn_blocking works:**
1. Moves closure to a dedicated thread pool
2. Returns a future that completes when the thread finishes
3. Async runtime continues handling other tasks
4. Result is available when `.await` completes

---

## Resource Comparison

### Memory Overhead

```
Thread Stack Size (default):
├── Linux: 2-8 MB per thread
├── macOS: 512 KB - 8 MB per thread
└── Windows: 1 MB per thread

Async Task Size:
├── Minimal: ~few KB per task
├── Depends on captured state
└── No fixed stack allocation
```

### Scalability Example

```
Scenario: 10,000 concurrent connections

Threads:
├── Memory: 10,000 × 2MB = ~20 GB
├── Context switches: High overhead
└── Practical limit: ~1,000 threads

Async:
├── Memory: 10,000 × ~10KB = ~100 MB
├── Cooperative scheduling: Low overhead
└── Practical limit: Millions of tasks
```

---

## AoC Application Guide

### AoC Problems → Async

- **Network-based puzzles** (rare, but HTTP fetching input)
- **Simulations with timers** (if applicable)
- **Event-driven state machines** (when waiting matters)

### AoC Problems → Threads

Most AoC problems benefit from threads:

```rust
// AoC pattern: Parallel search space exploration
fn find_solution(range: Range<u64>) -> Option<u64> {
    let num_threads = num_cpus::get();
    let chunk_size = (range.end - range.start) / num_threads as u64;
    
    let handles: Vec<_> = (0..num_threads)
        .map(|i| {
            let start = range.start + i as u64 * chunk_size;
            let end = if i == num_threads - 1 { range.end } else { start + chunk_size };
            
            thread::spawn(move || {
                (start..end).find(|&n| is_solution(n))
            })
        })
        .collect();
    
    handles.into_iter()
        .filter_map(|h| h.join().ok().flatten())
        .next()
}
```

### Common AoC Thread Patterns

| Pattern | Use Case | Example |
|---------|----------|---------|
| **Divide and Conquer** | Split input across cores | Grid processing |
| **Parallel Search** | Find first solution | Password cracking |
| **Independent Tasks** | Multiple independent parts | Multi-part simulations |
| **Pipeline** | Producer-consumer | Parse → Process → Aggregate |

---

## Quick Reference Card

```
┌─────────────────────────────────────────────────────────┐
│                  CHOOSE YOUR APPROACH                   │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  "Am I mostly WAITING?"                                 │
│      YES → ASYNC (network, file I/O, timers)            │
│                                                         │
│  "Am I mostly COMPUTING?"                               │
│      YES → THREADS (CPU-bound, parallelism)             │
│                                                         │
│  "Both waiting AND computing?"                          │
│      → ASYNC with spawn_blocking for CPU parts          │
│                                                         │
│  "How many concurrent operations?"                      │
│      < 100   → Either works fine                        │
│      100-1K  → Threads OK, async better                 │
│      > 1K    → Async strongly preferred                 │
│                                                         │
│  "Can I tolerate blocking?"                             │
│      YES → Threads are simpler                          │
│      NO  → Must use async                               │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Integrator Perspective

Think of this like **AUTOSAR component scheduling**:

| Rust Concept | AUTOSAR Analogy |
|--------------|-----------------|
| **Async runtime** | RTE (Runtime Environment) |
| **Async task** | Runnable (lightweight, cooperative) |
| **Thread** | OS Task (preemptive, heavyweight) |
| **`.await` point** | WaitPoint in runnable |
| **`spawn_blocking`** | Offload to background task |

**Key insight**: Just as AUTOSAR schedules runnables cooperatively within a task but uses OS tasks for isolation, Rust's async schedules tasks cooperatively within a thread but uses OS threads for true parallelism.

---

## Related Concepts

### Async Ecosystem
- [[async-await-basics]] - Foundation of async/await syntax and semantics
- [[future-trait-deep-dive]] - How futures work internally (poll, Pin, Waker)
- [[async-streams]] - Async iteration with Stream trait

### Threading Ecosystem
- [[rust-threading-basics]] - Thread spawning, JoinHandle, ownership transfer
- [[shared-state-concurrency]] - Arc<Mutex<T>> for shared mutable state
- [[message-passing-channels]] - mpsc channels for thread communication
- [[sync-send-traits]] - Thread safety marker traits

### Integration Points
- [[rust-concurrency-moc]] - Map of Content for all concurrency topics

---

*Tags: #async #threads #concurrency #decision-guide #rust-book-ch17 #performance*

*Links: [[zettel-index]] | [[rust-concurrency-moc]] | [[async-await-basics]] | [[rust-threading-basics]] | [[future-trait-deep-dive]] | [[shared-state-concurrency]] | [[sync-send-traits]]*
