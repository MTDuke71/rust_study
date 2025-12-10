# 🔄 Sync vs Async - Execution Models in Rust

*Understanding when to use synchronous blocking code versus asynchronous non-blocking code, with practical decision frameworks*

---

## 🎯 **Core Concept**

**Synchronous (sync)** code executes sequentially - each operation completes before the next begins. The thread **blocks** (waits) during I/O operations.

**Asynchronous (async)** code uses cooperative multitasking - operations **yield** control at `.await` points, allowing other work to progress during waits.

```rust
// Synchronous: Thread blocks during each read
fn sync_example() {
    let data1 = std::fs::read_to_string("file1.txt").unwrap();  // Block ~10ms
    let data2 = std::fs::read_to_string("file2.txt").unwrap();  // Block ~10ms
    // Total: ~20ms (sequential)
}

// Asynchronous: Yields during I/O, interleaves operations
async fn async_example() {
    let (data1, data2) = tokio::join!(
        tokio::fs::read_to_string("file1.txt"),
        tokio::fs::read_to_string("file2.txt"),
    );
    // Total: ~10ms (concurrent)
}
```

**Key Distinction**: Sync wastes CPU time waiting; async uses wait time productively.

---

## 🧠 **Mental Models**

### **Restaurant Analogy**

| Model | Waiter Behavior |
|-------|-----------------|
| **Sync** | Waiter stands at table waiting for order, ignores other tables |
| **Async** | Waiter takes order, serves other tables while kitchen prepares |

### **AUTOSAR Integration Analogy**

| Model | Component Behavior |
|-------|-------------------|
| **Sync** | Runnable blocks on CAN message, halts entire task cycle |
| **Async** | Runnable registers callback, RTE schedules other runnables while waiting |

### **Timeline Comparison**

```
Synchronous (blocking):
Thread: ████████░░░░░░░░████████░░░░░░░░████████
        [Work1 ][Wait  ][Work2 ][Wait  ][Work3 ]
        
Asynchronous (non-blocking):  
Task A: ████░░░░████░░░░████
Task B:     ████░░░░████░░░░████
Task C:         ████░░░░████░░░░████
        ↑       ↑       ↑
     .await  .await  .await (yield points)

Timeline: ████████████████████████
          [Productive CPU usage throughout]
```

---

## ⚖️ **Comparison Matrix**

| Aspect | Synchronous | Asynchronous |
|--------|-------------|--------------|
| **Execution** | Sequential, blocking | Concurrent, non-blocking |
| **Mental Model** | Simple, linear flow | State machine, yield points |
| **Best For** | CPU-bound work | I/O-bound work |
| **Scalability** | 1 task per thread | 1000s tasks per thread |
| **Error Handling** | Standard `?` propagation | Same, but across `.await` |
| **Debugging** | Straightforward stack traces | More complex (async stack) |
| **Dependencies** | Standard library only | Requires async runtime |
| **Learning Curve** | Lower | Higher (futures, pinning) |

---

## 🔍 **When to Use Each**

### **Use Synchronous Code When:**

```rust
// ✅ CPU-bound computation - no waiting involved
fn calculate_prime(n: u64) -> bool {
    (2..=(n as f64).sqrt() as u64).all(|i| n % i != 0)
}

// ✅ Simple scripts and CLI tools
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let content = std::fs::read_to_string(&args[1]).unwrap();
    println!("Lines: {}", content.lines().count());
}

// ✅ Single-threaded processing where simplicity matters
fn process_file(path: &str) -> Result<Data, Error> {
    let content = std::fs::read_to_string(path)?;
    let parsed = parse(&content)?;
    let result = transform(parsed)?;
    Ok(result)
}
```

**Sync is ideal for:**
- CPU-intensive computation (parsing, number crunching)
- Simple scripts and utilities
- Code where simplicity > concurrency
- When you control the entire execution environment

### **Use Asynchronous Code When:**

```rust
// ✅ Many concurrent I/O operations
async fn fetch_all(urls: Vec<String>) -> Vec<Response> {
    let futures: Vec<_> = urls.iter()
        .map(|url| reqwest::get(url))
        .collect();
    futures::future::join_all(futures).await
}

// ✅ Network servers handling many connections
async fn handle_connections(listener: TcpListener) {
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(handle_client(socket));  // Thousands concurrently
    }
}

// ✅ Operations that spend most time waiting
async fn database_workflow(db: &Database) {
    let user = db.get_user(id).await?;           // Wait for DB
    let orders = db.get_orders(user.id).await?;  // Wait for DB
    let summary = compute_summary(&orders);       // CPU work (no await)
    db.save_summary(summary).await?;             // Wait for DB
}
```

**Async is ideal for:**
- Network services (HTTP, WebSocket, gRPC)
- Database-heavy applications
- Concurrent file I/O
- Any workload dominated by waiting

---

## 🚫 **Common Mistakes**

### **1. Blocking in Async Context**

```rust
// ❌ BAD: Blocks the entire async runtime!
async fn bad_example() {
    let data = std::fs::read_to_string("big_file.txt").unwrap();
    std::thread::sleep(Duration::from_secs(5));  // Blocks runtime!
}

// ✅ GOOD: Use async equivalents
async fn good_example() {
    let data = tokio::fs::read_to_string("big_file.txt").await.unwrap();
    tokio::time::sleep(Duration::from_secs(5)).await;  // Yields to runtime
}

// ✅ GOOD: Use spawn_blocking for unavoidable sync code
async fn with_blocking() {
    let result = tokio::task::spawn_blocking(|| {
        expensive_sync_computation()
    }).await.unwrap();
}
```

### **2. CPU Work in Async Without spawn_blocking**

```rust
// ❌ BAD: CPU-intensive work starves other tasks
async fn cpu_in_async() {
    let result = (0..1_000_000).map(|n| n * n).sum::<i64>();  // No yield!
}

// ✅ GOOD: Move CPU work to blocking thread pool
async fn cpu_properly() {
    let result = tokio::task::spawn_blocking(|| {
        (0..1_000_000).map(|n| n * n).sum::<i64>()
    }).await.unwrap();
}
```

### **3. Over-engineering Simple Tasks**

```rust
// ❌ OVERKILL: Async for simple sequential file read
#[tokio::main]
async fn main() {
    let content = tokio::fs::read_to_string("config.txt").await.unwrap();
    println!("{}", content);
}

// ✅ SIMPLER: Just use sync for single operations
fn main() {
    let content = std::fs::read_to_string("config.txt").unwrap();
    println!("{}", content);
}
```

---

## 🎯 **Decision Framework**

```
Start Here
    │
    ▼
┌───────────────────────────────────┐
│  Is your workload I/O-bound?      │
│  (network, files, databases)      │
└─────────────┬─────────────────────┘
              │
     ┌────────┴────────┐
     │ YES             │ NO
     ▼                 ▼
┌─────────────┐   ┌─────────────────────┐
│ Many        │   │ CPU-bound work      │
│ concurrent  │   │ (computation)       │
│ operations? │   └──────────┬──────────┘
└──────┬──────┘              │
       │                     ▼
  ┌────┴────┐         ┌─────────────────┐
  │YES   NO │         │ Use SYNC +      │
  ▼         ▼         │ std::thread for │
┌──────┐  ┌──────┐    │ parallelism     │
│ASYNC │  │SYNC  │    └─────────────────┘
│tokio │  │simple│
└──────┘  └──────┘
```

### **Quick Decision Rules**

| Scenario | Recommendation |
|----------|----------------|
| CLI tool processing files | **Sync** |
| Web server | **Async** (tokio/axum) |
| AoC puzzle solver | **Sync** (or threads for parallelism) |
| Database CRUD app | **Async** |
| Number crunching | **Sync** + rayon/threads |
| Chat server | **Async** |
| Config file reader | **Sync** |
| API client making many requests | **Async** |

---

## 🔗 **Mixing Sync and Async**

### **Async Calling Sync (spawn_blocking)**

```rust
async fn read_and_process() {
    // Move blocking I/O to thread pool
    let data = tokio::task::spawn_blocking(|| {
        std::fs::read_to_string("file.txt")
    }).await.unwrap().unwrap();
    
    // Continue async processing
    send_to_server(&data).await;
}
```

### **Sync Calling Async (block_on)**

```rust
fn sync_main() {
    // Create runtime and block on async code
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        fetch_data().await
    });
    
    println!("Got: {:?}", result);
}
```

---

## 💡 **Key Takeaways**

1. **Sync = Simple, blocking** - Great for CPU work and simple programs
2. **Async = Concurrent, non-blocking** - Essential for I/O-heavy workloads
3. **Never block in async** - Use `spawn_blocking` for sync code
4. **Don't over-engineer** - Use sync unless you need concurrency
5. **Match the workload** - I/O-bound → async, CPU-bound → threads
6. **Async adds complexity** - Only use when benefits outweigh costs

---

## 🔗 **Integration Points**

### **Builds On**
- [[ownership-fundamentals]] - Ownership across execution boundaries
- [[rust-threading-basics]] - OS thread fundamentals
- [[async-await-basics]] - Async syntax and semantics

### **Enables**
- [[async-vs-threads-decision]] - Detailed threading comparison
- [[async-concurrency]] - Advanced async patterns (join, select)
- [[tokio-runtime]] - Production async runtime

### **Related Concepts**
- [[future-trait-deep-dive]] - How futures work internally
- [[message-passing-channels]] - Communication between tasks/threads
- [[shared-state-concurrency]] - Shared data patterns

### **Chapter References**
- [[rust_book/Ch16/README]] - Fearless Concurrency (threads)
- [[rust_book/Ch17/README]] - Async Programming

---

*Tags: #concept #rust-book #concurrency #async #threads #intermediate*

*Links: [[zettel-index]] | [[rust-concurrency-moc]] | [[async-await-basics]] | [[rust-threading-basics]] | [[async-vs-threads-decision]]*
