# Chapter 17: Asynchronous Programming - Async, Await, Futures, and Streams

## Writing efficient concurrent I/O-bound programs with async/await

---

## 📖 **Chapter Overview**

Chapter 17 introduces Rust's asynchronous programming model, enabling you to write efficient I/O-bound concurrent programs using `async`/`await` syntax and the `Future` trait. Unlike threads which consume OS resources, async tasks are lightweight and can scale to millions of concurrent operations.

### **Sections**

- **17.1**: Futures and the Async Syntax
- **17.2**: Applying Concurrency with Async
- **17.3**: Working With Any Number of Futures
- **17.4**: Streams: Futures in Sequence
- **17.5**: A Closer Look at the Traits for Async
- **17.6**: Futures, Tasks, and Threads

---

## 🎯 **Learning Objectives**

By completing this chapter, you will:

- ✅ **Understand futures** and lazy evaluation with async/await
- ✅ **Write async functions** that return Future types
- ✅ **Execute async code** with runtime executors (tokio/async-std)
- ✅ **Handle multiple futures** with join!, select!, and combinators
- ✅ **Work with streams** for asynchronous iteration
- ✅ **Choose between async and threads** based on workload characteristics

---

## 📂 **Section Structure**

```text
Ch17/
├── README.md                          # This file
├── futures_basics/                    # 17.1 - Futures and async syntax
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                    # Basic async/await
│       └── examples/
│           ├── future_basics.rs       # Future trait fundamentals
│           ├── async_blocks.rs        # Async blocks and closures
│           └── executor_comparison.rs # tokio vs async-std
├── async_concurrency/                 # 17.2 - Concurrent async
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                    # Multiple async tasks
│       └── examples/
│           ├── join_futures.rs        # Waiting for multiple futures
│           ├── select_futures.rs      # Racing futures with select!
│           └── timeout_patterns.rs    # Timeouts and cancellation
├── async_streams/                     # 17.3-17.4 - Futures collections
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                    # Stream basics
│       └── examples/
│           ├── stream_processing.rs   # Stream transformations
│           ├── async_iteration.rs     # for await loops
│           └── buffering.rs           # Buffered streams
└── async_traits/                      # 17.5-17.6 - Advanced async
    ├── Cargo.toml
    └── src/
        ├── main.rs                    # Trait implementations
        └── examples/
            ├── future_trait.rs        # Custom Future implementations
            ├── pin_unpin.rs           # Pin and self-referential types
            └── tasks_vs_threads.rs    # Async tasks vs OS threads
```

---

## 🚀 **Quick Start**

### **Prerequisites**

Async Rust requires a runtime executor. This chapter uses **tokio** (most popular) and **async-std** (alternative).

```bash
# All examples already have tokio configured in Cargo.toml
# Just run them directly
```

### **Running Examples**

```bash
# Section 17.1 - Futures Basics
cd rust_book/Ch17/futures_basics
cargo run                              # Basic async/await
cargo run --example future_basics      # Future trait
cargo run --example async_blocks       # Async blocks
cargo run --example executor_comparison # Runtime comparison

# Section 17.2 - Async Concurrency
cd rust_book/Ch17/async_concurrency
cargo run                              # Concurrent tasks
cargo run --example join_futures       # tokio::join!
cargo run --example select_futures     # tokio::select!
cargo run --example timeout_patterns   # Timeouts

# Section 17.3-17.4 - Streams
cd rust_book/Ch17/async_streams
cargo run                              # Stream basics
cargo run --example stream_processing  # Stream transformations
cargo run --example async_iteration    # Async for loops
cargo run --example buffering          # Buffered processing

# Section 17.5-17.6 - Advanced Traits
cd rust_book/Ch17/async_traits
cargo run                              # Trait implementations
cargo run --example future_trait       # Custom Future
cargo run --example pin_unpin          # Pin fundamentals
cargo run --example tasks_vs_threads   # Design decisions
```

---

## 📚 **Section Details**

### **17.1: Futures and the Async Syntax**

**Key Concepts:**

- `async fn` returns `impl Future<Output = T>`
- Futures are **lazy** - nothing happens until `.await`
- Async blocks create inline futures
- Runtime executors poll futures to completion

**The Mental Model:**

```text
Synchronous:          Asynchronous:
  Call function    →    Create future (lazy)
  Block until done →    .await to execute
  Return value     →    Future completes, yields value
```

**Basic Example:**

```rust
// This is an async function
async fn fetch_data() -> String {
    // Simulate network request
    tokio::time::sleep(Duration::from_secs(1)).await;
    String::from("Data from server")
}

#[tokio::main]
async fn main() {
    // Calling async fn returns a Future
    let future = fetch_data();
    
    // Nothing happens yet! Future is lazy.
    
    // .await executes the future
    let data = future.await;
    println!("Got: {}", data);
}
```

**Key Insight:** `async fn fetch_data()` desugars to:

```rust
fn fetch_data() -> impl Future<Output = String> {
    async move {
        tokio::time::sleep(Duration::from_secs(1)).await;
        String::from("Data from server")
    }
}
```

---

### **17.2: Applying Concurrency with Async**

**Key Concepts:**

- Run multiple futures concurrently with `tokio::join!`
- Race futures with `tokio::select!`
- Spawn background tasks with `tokio::spawn`
- Timeouts and cancellation

**Why Async Beats Threads for I/O:**

| Threads | Async Tasks |
|---------|-------------|
| OS resource (~1MB stack) | Runtime task (~few KB) |
| Limited to ~1000s | Scale to millions |
| Good for CPU-bound | Best for I/O-bound |
| Preemptive scheduling | Cooperative yielding |

**Concurrent Execution:**

```rust
use tokio::time::{sleep, Duration};

async fn task1() -> u32 {
    sleep(Duration::from_secs(2)).await;
    42
}

async fn task2() -> String {
    sleep(Duration::from_secs(1)).await;
    String::from("done")
}

#[tokio::main]
async fn main() {
    // Execute both concurrently, wait for both to complete
    let (result1, result2) = tokio::join!(task1(), task2());
    
    // Total time: 2 seconds (not 3!)
    println!("Results: {}, {}", result1, result2);
}
```

**Racing Futures:**

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let result = tokio::select! {
        data = fetch_from_cache() => {
            println!("Cache hit!");
            data
        }
        data = fetch_from_network() => {
            println!("Cache miss, fetched from network");
            data
        }
    };
}
```

---

### **17.3-17.4: Working With Any Number of Futures / Streams**

**Key Concepts:**

- `Stream` trait for asynchronous iteration
- `StreamExt` for stream combinators (map, filter, fold)
- `for await` loops (requires `#![feature(async_iterator)]` or StreamExt)
- Buffering and batching streams

**Streams vs Iterators:**

| Iterator | Stream |
|----------|--------|
| Synchronous | Asynchronous |
| `next() -> Option<T>` | `next() -> Future<Option<T>>` |
| Blocking | Non-blocking |

**Stream Example:**

```rust
use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() {
    let stream = stream::iter(1..=10)
        .then(|n| async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            n * 2
        });
    
    let results: Vec<_> = stream.collect().await;
    println!("Results: {:?}", results);
}
```

**Real-World Use Case:**

```rust
// Processing paginated API results
async fn fetch_all_users() -> Vec<User> {
    let mut all_users = Vec::new();
    let mut page = 1;
    
    loop {
        let users = fetch_page(page).await;
        if users.is_empty() { break; }
        all_users.extend(users);
        page += 1;
    }
    
    all_users
}

// Better: Use a stream
use futures::stream::{self, StreamExt};

fn user_stream() -> impl Stream<Item = User> {
    stream::unfold(1, |page| async move {
        let users = fetch_page(page).await;
        if users.is_empty() {
            None
        } else {
            Some((stream::iter(users), page + 1))
        }
    }).flatten()
}
```

---

### **17.5: A Closer Look at the Traits for Async**

**Key Concepts:**

- `Future` trait with `poll()` method
- `Pin<T>` for self-referential types
- `Unpin` marker trait
- State machines generated by `async fn`

**The Future Trait:**

```rust
pub trait Future {
    type Output;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),    // Future completed
    Pending,     // Future not ready, will be woken later
}
```

**How async/await Works:**

1. `async fn` generates a state machine
2. Each `.await` point is a state transition
3. Runtime calls `poll()` to advance state
4. When `Pending`, runtime saves the task
5. When I/O completes, task is woken and polled again

**Example State Machine:**

```rust
async fn example() {
    println!("State 1");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("State 2");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("State 3");
}

// Compiler generates (conceptually):
enum ExampleFuture {
    State1,
    State2 { sleep1: Sleep },
    State3 { sleep2: Sleep },
    Done,
}
```

**Pin and Self-Referential Types:**

Problem: Async state machines can have self-referential pointers

```rust
struct SelfReferential {
    data: String,
    pointer: *const String,  // Points to `data` field
}
```

If this moves in memory, the pointer becomes invalid!

Solution: `Pin<T>` prevents moving

```rust
// Pin guarantees the value won't move in memory
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
```

---

### **17.6: Futures, Tasks, and Threads**

**Key Concepts:**

- **Futures**: Lazy computation units
- **Tasks**: Spawned async work (like lightweight threads)
- **Threads**: OS-level parallelism
- Choosing the right concurrency primitive

**Decision Tree:**

```text
What's the workload?
├─ I/O-bound (network, disk, timers)
│  └─ Use async tasks
│     ├─ Single async fn → .await
│     ├─ Few concurrent ops → tokio::join!
│     └─ Many concurrent ops → tokio::spawn
│
└─ CPU-bound (computation, crypto, parsing)
   └─ Use threads
      ├─ Few cores → std::thread::spawn
      └─ Data parallelism → rayon
```

**Hybrid Approach:**

```rust
#[tokio::main]
async fn main() {
    // Async for I/O
    let data = fetch_from_network().await;
    
    // Spawn thread for CPU-intensive work
    let processed = tokio::task::spawn_blocking(move || {
        expensive_cpu_work(data)
    }).await.unwrap();
    
    // Back to async for I/O
    save_to_database(processed).await;
}
```

**Task Spawning:**

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    // Spawn background task (doesn't block)
    let handle = task::spawn(async {
        background_processing().await
    });
    
    // Do other work concurrently
    let result = foreground_work().await;
    
    // Wait for background task if needed
    let bg_result = handle.await.unwrap();
}
```

---

## 🔗 **Integration with Learning Tracks**

### **Comparison with Ch16 Threading**

| Aspect | Ch16 Threads | Ch17 Async |
|--------|-------------|------------|
| **Best For** | CPU-bound work | I/O-bound work |
| **Scalability** | ~1000s of threads | Millions of tasks |
| **Memory** | ~1MB per thread | ~few KB per task |
| **Scheduling** | Preemptive (OS) | Cooperative (runtime) |
| **Blocking** | Can block safely | Must not block! |
| **Primitives** | Mutex, channels, Arc | async/await, join!, select! |

### **Daily Study Integration**

- **Week 8, Day 54**: Async basics and Future trait
- **Week 8, Day 55**: Concurrent async with tokio
- **Week 8, Day 56**: Streams and async iteration
- **Week 8, Day 57**: Advanced async patterns
- **[[../../zettelkasten/Daily Notes/2025-11-21]]**: Chapter 17.2 completion - Concurrency deep dive

### **AoC Applications**

- Fetching multiple puzzle inputs concurrently
- Parallel test case execution (I/O-bound)
- Streaming large input files without loading into memory

### **Zettelkasten Links**

- [[async-await-fundamentals]] - Core async concepts
- [[future-trait-deep-dive]] - Understanding the Future trait
- [[tokio-runtime-internals]] - How async runtimes work
- [[async-vs-threads-decision]] - Choosing concurrency model

---

## 🎓 **Key Takeaways**

### **Fundamental Principles**

1. **Futures are lazy** - Nothing happens until `.await`
2. **Async is for I/O** - Use threads for CPU-bound work
3. **Cooperative yielding** - Tasks must explicitly yield with `.await`
4. **Non-blocking is critical** - Never block the executor thread
5. **Pin prevents moves** - Required for self-referential state machines

### **Common Patterns**

#### Pattern 1: Concurrent I/O

```rust
// Fetch multiple resources concurrently
let (user, posts, comments) = tokio::join!(
    fetch_user(id),
    fetch_posts(id),
    fetch_comments(id)
);
```

#### Pattern 2: Timeout Protection

```rust
use tokio::time::{timeout, Duration};

match timeout(Duration::from_secs(5), fetch_data()).await {
    Ok(data) => println!("Success: {:?}", data),
    Err(_) => println!("Request timed out"),
}
```

#### Pattern 3: Background Tasks

```rust
// Fire-and-forget background work
tokio::spawn(async {
    log_metrics().await;
});
```

#### Pattern 4: Stream Processing

```rust
use futures::stream::StreamExt;

stream::iter(items)
    .map(|item| process(item))
    .buffer_unordered(10)  // Process 10 concurrently
    .collect::<Vec<_>>()
    .await
```

---

## 🧪 **Exercises**

### **Exercise 1: Concurrent API Calls**

Write an async function that fetches data from 3 different APIs concurrently and combines results.

### **Exercise 2: Timeout Handler**

Implement retry logic with exponential backoff for a failing async operation.

### **Exercise 3: Stream Pipeline**

Create a stream that reads a file line-by-line, processes each line asynchronously, and writes results.

### **Exercise 4: Mixed Workload**

Combine async I/O with blocking CPU work using `spawn_blocking`.

---

## 📖 **Additional Resources**

### **Rust Documentation**

- [std::future::Future](https://doc.rust-lang.org/std/future/trait.Future.html) - Future trait
- [Async Book](https://rust-lang.github.io/async-book/) - Official async programming guide
- [tokio docs](https://tokio.rs/) - Tokio runtime documentation

### **Essential Crates**

- **tokio** - Most popular async runtime (used in this chapter)
- **async-std** - Alternative runtime with std-like API
- **futures** - Future combinators and utilities
- **async-trait** - Async methods in traits

### **Advanced Topics**

- **Pin and Unpin** - Memory safety for self-referential types
- **Waker API** - How futures notify the executor
- **Custom runtimes** - Building your own executor
- **Async cancellation** - Dropping futures safely

---

## 🚦 **Common Pitfalls**

### **Blocking in Async Context** ❌

```rust
#[tokio::main]
async fn main() {
    // DON'T: This blocks the executor thread!
    std::thread::sleep(Duration::from_secs(1));
    
    // DO: Use async sleep
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

### **Forgetting .await** ❌

```rust
async fn wrong() {
    let future = fetch_data();  // Future created but never executed!
    // Nothing happens - future is lazy
}

async fn correct() {
    let data = fetch_data().await;  // Future executed
}
```

### **Holding Mutex Across .await** ❌

```rust
use std::sync::Mutex;

// DON'T: Mutex locked across .await point
async fn bad(mutex: &Mutex<i32>) {
    let guard = mutex.lock().unwrap();  // Lock acquired
    async_operation().await;            // Executor may switch tasks
                                        // Other tasks blocked!
    drop(guard);
}

// DO: Use tokio::sync::Mutex for async
use tokio::sync::Mutex;

async fn good(mutex: &Mutex<i32>) {
    let guard = mutex.lock().await;  // Async-aware lock
    async_operation().await;
    drop(guard);
}
```

### **Not Handling Cancellation** ⚠️

```rust
// Dropping a future cancels it
let handle = tokio::spawn(async {
    important_cleanup().await;  // Might not run if dropped!
});

// Make sure to await important tasks
handle.await.unwrap();
```

---

## 🔍 **Async vs Threads Cheat Sheet**

```rust
// Threads (Ch16) - CPU-bound work
use std::thread;

let handles: Vec<_> = (0..10)
    .map(|i| thread::spawn(move || compute(i)))
    .collect();

let results: Vec<_> = handles
    .into_iter()
    .map(|h| h.join().unwrap())
    .collect();

// Async (Ch17) - I/O-bound work  
use tokio::task;

let futures: Vec<_> = (0..1000)
    .map(|i| task::spawn(fetch(i)))
    .collect();

let results: Vec<_> = futures
    .into_iter()
    .map(|f| f.await.unwrap())
    .collect();
```

**Key Difference:**

- Threads: 10 OS threads = ~10MB memory, limited scalability
- Async: 1000 tasks = ~few MB memory, scales to millions

---

## 🎯 **Chapter Completion Checklist**

- [ ] Understand Future trait and lazy evaluation
- [ ] Write async functions with async/await
- [ ] Use tokio::join! for concurrent futures
- [ ] Use tokio::select! for racing futures
- [ ] Process streams with StreamExt
- [ ] Understand Pin and why it's needed
- [ ] Know when to use async vs threads
- [ ] Handle timeouts and cancellation
- [ ] Avoid blocking in async context
- [ ] Use tokio::sync primitives correctly

---

**Tags:** #rust-book #ch17 #async #await #futures #streams #tokio #concurrency #io-bound #async-programming

**Links:** [[../README]] | [[../Ch16/README]] | [[../../zettelkasten/async-await-basics]] | [[../../zettelkasten/async-concurrency]] | [[../../zettelkasten/async-performance-timer-resolution]] | [[learning-plan]]
