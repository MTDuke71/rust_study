# Async Concurrency Patterns

*Navigation: [[zettel-index]] | [[rust-concurrency-moc]] | [[async-await-basics]]*

---

## Overview

Async concurrency in Rust enables **multiple async operations to progress simultaneously on a single thread** through cooperative task scheduling. Unlike sequential async code, concurrent patterns allow overlapping I/O waits, dramatically improving throughput for I/O-bound workloads.

## Core Concurrency Primitives

### select! - Racing Futures

Execute multiple futures concurrently, return the first to complete and **cancel** the others:

```rust
use trpl::{Either, select};

let future1 = fetch_data(url1);
let future2 = fetch_data(url2);

match select(future1, future2).await {
    Either::Left(result1) => {
        println!("URL1 won the race!");
        // future2 is CANCELLED (dropped)
    }
    Either::Right(result2) => {
        println!("URL2 won the race!");
        // future1 is CANCELLED (dropped)
    }
}
```

**Key Characteristics:**
- **First-to-finish wins**: Fastest operation determines result
- **Losers cancelled**: Non-winning futures dropped immediately
- **No waiting**: Returns as soon as ANY future completes
- **Use cases**: Timeouts, redundant requests, fastest-source patterns

**Common Pattern - Timeout:**
```rust
let work = async { expensive_operation().await };
let timeout = async { sleep(Duration::from_secs(5)).await };

match select(work, timeout).await {
    Either::Left(result) => println!("Completed: {:?}", result),
    Either::Right(_) => println!("Timed out!"),
}
```

### join! - Wait for All

Execute multiple futures concurrently, wait for **all** to complete:

```rust
use tokio::join;

let future1 = fetch_data(url1);
let future2 = fetch_data(url2);
let future3 = fetch_data(url3);

// All three run concurrently, wait for ALL
let (result1, result2, result3) = join!(future1, future2, future3);

println!("All complete: {}, {}, {}", result1, result2, result3);
```

**Key Characteristics:**
- **Concurrent execution**: All futures progress simultaneously
- **Wait for completion**: Returns when ALL are done
- **Preserves order**: Results in same order as futures
- **No cancellation**: All futures run to completion
- **Use cases**: Batch operations, parallel data fetching

**Performance Impact:**
```rust
// Sequential: 3 seconds total (1s + 1s + 1s)
let r1 = fetch(url1).await;  // 1 second
let r2 = fetch(url2).await;  // 1 second
let r3 = fetch(url3).await;  // 1 second

// Concurrent with join!: ~1 second total (all overlap)
let (r1, r2, r3) = join!(
    fetch(url1),  // All three run
    fetch(url2),  // at the same
    fetch(url3),  // time!
);
```

### spawn_task - Background Execution

Create an independent task that runs in the background:

```rust
use trpl::spawn_task;

let handle = spawn_task(async {
    for i in 1..=5 {
        println!("Background work item {}", i);
        sleep(Duration::from_millis(100)).await;
    }
    println!("Background work complete!");
});

// Main task continues independently
println!("Main task doing other work...");
let result = fetch_data(url).await;
println!("Main task got: {}", result);

// Wait for background task to finish
handle.await.unwrap();
```

**Key Characteristics:**
- **Independent lifecycle**: Continues even after spawner moves on
- **No automatic cancellation**: Must explicitly await or drop handle
- **Background execution**: Runs concurrently with spawner
- **Join point**: `.await` on handle waits for completion
- **Use cases**: Fire-and-forget work, long-running background tasks

## Advanced Concurrency Patterns

### Interrupt and Resume

Race work against I/O, but allow work to continue after interruption:

```rust
// Spawn work as independent task
let work_handle = spawn_task(async {
    for i in 1..=5 {
        println!("Work item {}", i);
        sleep(Duration::from_millis(100)).await;
    }
});

// Wait for HTTP, doesn't cancel work
let result = fetch_data(url).await;
println!("HTTP completed: {}", result);

// Work continues in background...
work_handle.await.unwrap();  // Wait for it to finish
println!("All tasks complete!");
```

**Timeline:**
```
0ms:   Work task starts, HTTP request starts
100ms: Work item 1 completes
200ms: HTTP completes → prints result → work continues
300ms: Work item 3 completes
500ms: Work completes → "All tasks complete!"
```

### True Interleaving with Nested select()

Allow HTTP completion to interrupt work loop immediately:

```rust
let work_future = async {
    for i in 1..=5 {
        println!("Work item {}", i);
        sleep(Duration::from_millis(100)).await;
    }
    None  // Work finished, no HTTP result
};

let http_race = async {
    match select(http1, http2).await {
        Either::Left(result) => Some(result),
        Either::Right(result) => Some(result),
    }
};

// Outer select: work loop vs HTTP race
match select(work_future, http_race).await {
    Either::Left(_) => println!("Work finished first"),
    Either::Right(Some(http_result)) => {
        println!("HTTP interrupted work loop!");
        println!("Result: {:?}", http_result);
    }
}
```

**Execution Flow:**
- If HTTP completes at 200ms: Work items 1-2 execute, then HTTP interrupts
- If work completes at 500ms: All 5 items execute, HTTP still pending

### Concurrent Work During I/O

Demonstrate that async allows useful work while waiting:

```rust
// Start HTTP requests (not awaited yet - just created)
let http1 = fetch(url1);
let http2 = fetch(url2);

println!("HTTP requests started!");

// Do other work while HTTP is in flight
for i in 1..=5 {
    println!("Local work item {}", i);
    sleep(Duration::from_millis(100)).await;
}

println!("Finished local work, checking HTTP...");

// Now check which HTTP finished first
match select(http1, http2).await {
    Either::Left(result) => println!("URL1 finished: {:?}", result),
    Either::Right(result) => println!("URL2 finished: {:?}", result),
}
```

**Key Insight:** Both HTTP requests are in flight during the work loop. All three operations (HTTP1, HTTP2, work loop) progress concurrently on the same thread!

## Execution Model Deep Dive

### Polling and Yield Points

The runtime drives concurrency by polling futures at `.await` points:

```rust
async fn example() {
    // Point 1: Start operation
    operation1().await;  // ← YIELD: Task pauses if not ready
    
    // Point 2: Resume when operation1 completes
    operation2().await;  // ← YIELD: Task pauses if not ready
    
    // Point 3: Resume when operation2 completes
}
```

**Runtime Scheduling:**
1. Poll `example()` future
2. Reaches first `.await` → operation not ready → `Poll::Pending`
3. Runtime switches to other tasks
4. Operation1 completes → runtime wakes task
5. Poll `example()` again → resumes at first `.await`
6. Continues to second `.await` → operation not ready → `Poll::Pending`
7. Runtime switches to other tasks
8. Operation2 completes → runtime wakes task
9. Poll `example()` again → completes → `Poll::Ready(())`

### Sleep Interruptibility

**CRITICAL:** `sleep()` is NOT interruptible during its duration:

```rust
for i in 1..=5 {
    println!("Work item {}", i);
    sleep(Duration::from_millis(100)).await;  // ← 100ms uninterruptible block
}
```

**Timeline:**
- 0ms: Iteration 1 starts
- 0-100ms: Sleep active, task idle (no polling possible)
- 100ms: Sleep completes, `.await` returns, print happens
- 100ms: Iteration 2 starts, sleep begins
- 100-200ms: Sleep active, task idle
- 200ms: If HTTP completes here, it must wait for sleep to finish
- 200ms: Sleep completes, `.await` returns, then runtime can switch

**Polling happens AFTER each sleep, not during:**
- Runtime can only switch tasks at `.await` points
- During `sleep()`, the task is suspended waiting for timer
- If HTTP completes during sleep, it can't interrupt
- HTTP completion is handled at next `.await` point

### Task Switching Mechanics

```rust
// Task 1
async fn task1() {
    println!("Task 1 start");
    sleep(100ms).await;    // ← YIELD POINT
    println!("Task 1 end");
}

// Task 2
async fn task2() {
    println!("Task 2 start");
    sleep(50ms).await;     // ← YIELD POINT
    println!("Task 2 end");
}

// Execution timeline (single thread):
// 0ms:   Task 1 polls → starts sleep → returns Pending
// 0ms:   Task 2 polls → starts sleep → returns Pending
// 50ms:  Task 2 timer expires → runtime wakes Task 2
// 50ms:  Task 2 polls → completes → prints "Task 2 end"
// 100ms: Task 1 timer expires → runtime wakes Task 1
// 100ms: Task 1 polls → completes → prints "Task 1 end"
```

## Pattern Comparison

| **Pattern** | **Futures** | **Wait For** | **Cancellation** | **Use Case** |
|-------------|-------------|--------------|------------------|--------------|
| **Sequential** | One at a time | Current only | N/A | Dependencies between operations |
| **select()** | All concurrent | First to finish | Yes, losers cancelled | Racing, timeouts, fastest-wins |
| **join!()** | All concurrent | All to finish | No | Batch operations, need all results |
| **spawn_task()** | Independent | Explicit await on handle | Only if handle dropped | Background work, fire-and-forget |

## Integrator Perspective (AUTOSAR Parallels)

### Async Patterns → AUTOSAR Equivalents

| **Rust Async Pattern** | **AUTOSAR Pattern** | **Explanation** |
|------------------------|---------------------|-----------------|
| `select()` | First-available event | Process whichever event arrives first |
| `join!()` | Synchronization point | Wait for multiple runnables to complete cycle |
| `spawn_task()` | Independent runnable | Separate periodic task (e.g., 10ms vs 100ms) |
| Concurrent futures | Concurrent runnables | Multiple runnables scheduled by RTE |
| `.await` yield point | Runnable completion | Hand control back to scheduler |

### Component Orchestration Analogy

```rust
// Rust: Racing HTTP requests
match select(sensor_a_read, sensor_b_read).await {
    Either::Left(data) => process(data),
    Either::Right(data) => process(data),
}

// AUTOSAR: First sensor data wins
// Runnable triggered by first CAN message arrival
// Both sensors send, process whichever arrives first
```

**Key Insight:** Async Rust is **component orchestration** - coordinating multiple independent work units (futures) through a scheduler (runtime), just like AUTOSAR RTE coordinates runnables.

## Common Pitfalls

### ❌ Blocking in Async Context

```rust
async fn bad_example() {
    std::thread::sleep(Duration::from_secs(1));  // ❌ BLOCKS ENTIRE RUNTIME!
    // No other tasks can run during this second
}

async fn good_example() {
    tokio::time::sleep(Duration::from_secs(1)).await;  // ✅ Yields to runtime
    // Other tasks run during this second
}
```

### ❌ Forgetting to .await

```rust
async fn bad_example() {
    let _future = fetch_data(url);  // ❌ Created but never executed!
    // Future dropped without running
}

async fn good_example() {
    let result = fetch_data(url).await;  // ✅ Actually executes
}
```

### ❌ Sequential When Concurrent Intended

```rust
async fn slow() {
    // Sequential: 3 seconds total
    let r1 = fetch(url1).await;  // 1 second
    let r2 = fetch(url2).await;  // 1 second  
    let r3 = fetch(url3).await;  // 1 second
}

async fn fast() {
    // Concurrent: ~1 second total
    let (r1, r2, r3) = join!(
        fetch(url1),
        fetch(url2),
        fetch(url3),
    );
}
```

## Performance Characteristics

**Memory:**
- `select()`: Same as single future (losers dropped immediately)
- `join!()`: Sum of all futures' memory
- `spawn_task()`: Additional task overhead (~1-2 KB per task)

**CPU:**
- Polling overhead: Minimal with proper yielding
- Context switching: Faster than OS threads (no kernel)
- Best throughput: I/O-bound work with many concurrent operations

**Scalability:**
- Can handle thousands to millions of concurrent tasks
- Limited by memory (not thread count)
- Ideal for high-concurrency scenarios (web servers, proxies)

## Related Concepts

**Foundations:**
- [[async-await-basics]] - Future trait, `.await`, runtime basics
- [[futures-and-polling]] - How polling drives execution
- [[runtime-executors]] - tokio, async-std, smol comparison

**Advanced Patterns:**
- [[async-streams]] - Async iteration and backpressure
- [[async-channels]] - Message passing between tasks
- [[async-timeouts]] - Timeout patterns and cancellation
- [[async-retry-patterns]] - Retry with backoff

**Architecture:**
- [[task-spawning-strategies]] - When to spawn vs inline
- [[structured-concurrency]] - Parent-child task relationships
- [[graceful-shutdown]] - Coordinated task termination

**Error Handling:**
- [[async-error-propagation]] - ? operator in async contexts
- [[timeout-error-handling]] - Timeout vs operation errors
- [[concurrent-error-aggregation]] - Collecting errors from join!()

---

## Complete Example: Practical Async Concurrency

```rust
use trpl::{Either, Html, spawn_task};
use std::time::Duration;

#[trpl::main]
async fn main() {
    let url1 = "https://example.com";
    let url2 = "https://httpbin.org/html";
    
    // Pattern 1: Racing with select (first wins, losers cancelled)
    println!("=== Pattern 1: Racing HTTP Requests ===");
    let race1 = page_title(url1);
    let race2 = page_title(url2);
    
    match trpl::select(race1, race2).await {
        Either::Left((url, title)) => {
            println!("{} won! Title: {:?}", url, title);
        }
        Either::Right((url, title)) => {
            println!("{} won! Title: {:?}", url, title);
        }
    }
    
    // Pattern 2: Background task with spawn (independent lifecycle)
    println!("\n=== Pattern 2: Background Work ===");
    let work_handle = spawn_task(async {
        for i in 1..=5 {
            println!("  🔧 Work item {}", i);
            trpl::sleep(Duration::from_millis(100)).await;
        }
        println!("  ✅ Work complete!");
    });
    
    // Main task continues (HTTP + work run concurrently)
    let http1 = page_title(url1);
    let http2 = page_title(url2);
    
    let (result1, result2) = trpl::join(http1, http2).await;
    println!("HTTP results: {:?}, {:?}", result1, result2);
    
    // Wait for background work
    work_handle.await.unwrap();
    
    // Pattern 3: Nested select for true interruption
    println!("\n=== Pattern 3: Interruptible Work ===");
    let work_future = async {
        for i in 1..=5 {
            println!("  Work {}", i);
            trpl::sleep(Duration::from_millis(100)).await;
        }
        None
    };
    
    let http_future = async {
        Some(page_title(url1).await)
    };
    
    match trpl::select(work_future, http_future).await {
        Either::Left(_) => println!("Work finished first"),
        Either::Right(Some(result)) => {
            println!("HTTP interrupted work! {:?}", result);
        }
        Either::Right(None) => unreachable!(),
    }
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await.text().await;
    let title = Html::parse(&response)
        .select_first("title")
        .map(|t| t.inner_html());
    (url, title)
}
```

---

*Tags: #rust #async #concurrency #select #join #spawn #cooperative-multitasking #io-bound*

*Links: [[zettel-index]] | [[rust-concurrency-moc]] | [[async-await-basics]] | [[futures-and-polling]] | [[async-streams]]*

*Last Updated: November 21, 2025*
