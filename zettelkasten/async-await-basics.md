# Async/Await Basics

*Navigation: [[zettel-index]] | [[rust-concurrency-moc]]*

---

## Overview

Async/await in Rust provides **cooperative multitasking** on a single thread through the Future trait. Unlike traditional threading, async code uses lightweight tasks that yield control at `.await` points, allowing thousands of concurrent operations without the overhead of OS threads.

## Core Concepts

### Futures - Lazy Work Tickets

A Future represents work that will complete eventually but hasn't started yet:

```rust
// Creating a future doesn't execute it
let future = async {
    println!("This only prints when awaited!");
    42
};

// Execution happens at .await
let value = future.await;  // NOW it runs and prints
```

**Key Properties:**

- **Lazy evaluation**: Created but not executed until `.await`
- **Zero-cost until polled**: No overhead for unused futures
- **Single execution**: Each future runs once when awaited

### The Future Trait

Every async operation implements the `Future` trait with a `poll()` method:

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),      // Work complete, here's the result
    Pending,       // Not done yet, check back later
}
```

**Runtime Behavior:**

1. Runtime calls `poll()` on future
2. Future returns `Poll::Pending` if not ready (yields to runtime)
3. Runtime switches to other tasks
4. When ready (timer expires, I/O completes), runtime polls again
5. Future returns `Poll::Ready(value)` - work complete

### .await - The Yield Point

The `.await` keyword is where tasks pause and resume:

```rust
async fn fetch_data() -> String {
    // Point 1: Start HTTP request
    let response = http_get(url).await;  // ← YIELD: Task pauses here
    // Point 2: Resume when HTTP completes
    
    // Point 3: Start text decode
    let text = response.text().await;    // ← YIELD: Task pauses here
    // Point 4: Resume when decode completes
    
    text
}
```

**What happens at `.await`:**

1. Task starts async operation (HTTP request, sleep timer)
2. Operation not immediately ready → returns `Poll::Pending`
3. Task yields control back to runtime
4. Runtime switches to other tasks
5. Operation completes → runtime wakes task
6. Task resumes from `.await` point with result

### Async Functions

`async fn` is syntactic sugar for functions returning `impl Future`:

```rust
// This async function...
async fn example() -> String {
    sleep(Duration::from_secs(1)).await;
    String::from("Done!")
}

// ...is equivalent to this manual form:
fn example() -> impl Future<Output = String> {
    async {
        sleep(Duration::from_secs(1)).await;
        String::from("Done!")
    }
}
```

### Async Blocks

Anonymous async functions created inline:

```rust
let future = async {
    println!("Inside async block");
    sleep(Duration::from_millis(100)).await;
    42  // Last expression is return value (no semicolon)
};

let result = future.await;  // Executes the block, returns 42
```

**With `move` keyword:**

```rust
let data = vec![1, 2, 3];
let future = async move {
    data.iter().sum::<i32>()  // Ownership moved into async block
};
```

## Runtime Execution Model

### The Runtime as Orchestrator

Async code requires a **runtime** (executor) to actually run:

```rust
#[tokio::main]  // Creates and runs runtime
async fn main() {
    // Your async code here
}

// Equivalent manual form:
fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        // Your async code here
    });
}
```

**Runtime responsibilities:**

- Poll futures to drive them to completion
- Manage task scheduling (which future to poll next)
- Wake tasks when operations complete (I/O ready, timer expired)
- Handle task spawning and cancellation

### Cooperative vs Preemptive Multitasking

**Async (Cooperative):**

- Tasks explicitly yield at `.await` points
- Runtime switches tasks only at yield points
- Single-threaded by default (though can use thread pool)
- Lightweight: ~KB per task, millions possible

**Threads (Preemptive):**

- OS forcibly switches threads (time slicing)
- No explicit yield needed
- Always multi-threaded
- Heavy: ~1MB stack per thread, ~1000s maximum

## Non-Blocking I/O

### Blocking vs Non-Blocking

**Blocking (bad in async):**

```rust
std::thread::sleep(Duration::from_secs(1));  // ❌ Freezes entire thread!
// Runtime can't run other tasks during this
```

**Non-Blocking (correct in async):**

```rust
tokio::time::sleep(Duration::from_secs(1)).await;  // ✅ Yields to runtime
// Runtime runs other tasks during 1-second wait
```

**Key Difference:**

- **`std::thread::sleep`**: Blocks OS thread, nothing else can run
- **`tokio::time::sleep`**: Task yields, runtime switches to other tasks

### When to Use Async

**✅ Async excels at I/O-bound work:**

- Network requests (HTTP, database queries)
- File I/O (reading/writing files)
- Waiting on external events
- Many concurrent connections (web servers)

**❌ Avoid async for CPU-bound work:**

- Heavy computation (image processing, cryptography)
- Pure algorithm work (sorting, searching large datasets)
- Use threads or `spawn_blocking()` instead

## Common Patterns

### Sequential Execution

```rust
async fn sequential() {
    let data1 = fetch(url1).await;  // Wait for first
    let data2 = fetch(url2).await;  // Then wait for second
    let data3 = fetch(url3).await;  // Then wait for third
    // Total time: sum of all three
}
```

### Lazy Future Creation

```rust
// Create futures without starting them
let future1 = fetch(url1);  // NOT executing yet
let future2 = fetch(url2);  // NOT executing yet

println!("Futures created, but HTTP hasn't started!");

// Now they execute
let result1 = future1.await;
let result2 = future2.await;
```

## Integrator Perspective (AUTOSAR Parallels)

| **Rust Async** | **AUTOSAR** | **Explanation** |
|----------------|-------------|-----------------|
| `async fn` | Runnable callback | Work unit to execute |
| `.await` | Runnable yield point | Where task can pause |
| Runtime (tokio) | RTE (Runtime Environment) | Schedules and executes work |
| Future | Event/work ticket | Promise of future work |
| `Poll::Pending` | Processing status | "Not done yet" signal |
| `Poll::Ready` | Complete status | "Work finished" result |
| Cooperative yield | Runnable completion | Explicit control handoff |

**Key Insight:**

- AUTOSAR runnables are **callbacks** - RTE invokes them, they run to completion
- Rust async functions are **pauseable** - can yield mid-execution at `.await` points
- Both: Runtime/RTE orchestrates multiple concurrent work items

## Performance Characteristics

**Memory:**

- Future size: Small (struct holding state)
- Task overhead: ~1-2 KB (vs ~1 MB for thread)
- Can have millions of tasks on modest hardware

**CPU:**

- Polling overhead: Minimal when yielding properly
- Context switch: Faster than OS thread switch (no kernel involvement)
- Best case: I/O-bound work where tasks mostly wait

**Scaling:**

- Thread model: ~1,000s of threads max
- Async model: ~100,000s to millions of tasks possible

## Related Concepts

**Foundations:**

- [[futures-and-polling]] - Deep dive into Future trait and poll mechanism
- [[runtime-executors]] - How tokio and async-std execute futures
- [[pin-and-unpin]] - Memory safety for self-referential futures

**Patterns:**

- [[async-concurrency]] - Racing, joining, spawning tasks
- [[async-error-handling]] - Result and ? operator in async contexts
- [[async-streams]] - Async iteration over sequences

**Architecture:**

- [[sync-vs-async]] - When to use threads vs async
- [[blocking-in-async]] - Handling CPU-bound work (spawn_blocking)
- [[async-trait-objects]] - Dynamic dispatch with async
- [[async-performance-timer-resolution]] - Timer granularity, runtime overhead, and yield_now performance

**AUTOSAR Comparisons:**

- [[rust-concurrency-vs-autosar]] - Mapping async patterns to automotive
- [[event-driven-patterns]] - Callback vs future-based architectures

---

## Example: Complete Demonstration

```rust
use tokio::time::{sleep, Duration};

/// Custom Future demonstrating the trait
struct DelayedValue {
    value: i32,
    sleep: Pin<Box<tokio::time::Sleep>>,
}

impl Future for DelayedValue {
    type Output = i32;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<i32> {
        match self.sleep.as_mut().poll(cx) {
            Poll::Ready(()) => Poll::Ready(self.value),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[tokio::main]
async fn main() {
    // Example 1: Async function
    async fn fetch_data() -> String {
        sleep(Duration::from_secs(1)).await;
        String::from("Data loaded")
    }
    
    // Example 2: Async block
    let compute = async {
        sleep(Duration::from_millis(500)).await;
        42
    };
    
    // Example 3: Lazy evaluation
    let future1 = fetch_data();  // Created, not started
    println!("Future created");
    
    let data = future1.await;    // NOW it executes
    println!("Got: {}", data);
    
    // Example 4: Custom future
    let delayed = DelayedValue {
        value: 100,
        sleep: Box::pin(sleep(Duration::from_millis(200))),
    };
    let value = delayed.await;
    println!("Delayed value: {}", value);
}
```

---

*Tags: #rust #async #futures #concurrency #await #cooperative-multitasking #non-blocking*

*Links: [[zettel-index]] | [[rust-concurrency-moc]] | [[async-concurrency]] | [[futures-and-polling]] | [[runtime-executors]] | [[future-trait-deep-dive]] | [[async-vs-threads-decision]] | [[async-trait-objects]]*

*Last Updated: December 1, 2025*

