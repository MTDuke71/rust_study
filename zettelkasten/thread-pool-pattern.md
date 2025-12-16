# Thread Pool Pattern - Fixed Worker Concurrency Design

*A concurrency design pattern that maintains a fixed pool of reusable worker threads, distributing incoming jobs across them to control resource usage while enabling parallel execution.*

---

## 🎯 **Core Concept**

The **Thread Pool Pattern** solves the resource control problem in concurrent systems: spawning a new thread for every task creates unbounded overhead (context switching, memory per thread, OS limits). Instead, a **fixed-size pool** of worker threads is created upfront, and all workers share a job queue. Each worker loops indefinitely, pulling tasks from the queue and executing them. This pattern provides:

1. **Bounded resource usage** - Fixed number of threads regardless of workload
2. **Thread reuse** - No creation/destruction overhead per task
3. **Controlled concurrency** - Predictable parallelism based on pool size
4. **Backpressure handling** - Queue depth indicates system load

**Real-world application**: Web servers (handle N concurrent connections), job processors (process M tasks in parallel), parallel iterators (data parallelism across cores).

---

## 🧠 **Mental Models**

### **Restaurant Kitchen Model**
- **Cooks = Worker threads** (fixed staff, e.g., 4 cooks)
- **Order tickets = Jobs** (arrive continuously)
- **Ticket rail = Job queue** (FIFO, shared between cooks)
- **Each cook**: Grabs next ticket → prepares dish → returns to rail
- **Busy cooks don't spawn helpers** - orders queue up instead
- **Closing time = Graceful shutdown** - finish current orders, then stop

### **Airport Security Checkpoint**
- **TSA agents = Workers** (staffed according to expected throughput)
- **Passenger queue = Job queue** (everyone waits in same line)
- **Fast screening = Short jobs** (processed quickly, agent becomes available)
- **Bag checks = Long jobs** (agent busy longer, but others keep moving)
- **Opening more lanes = Increasing pool size** (trade space/cost for throughput)

### **Contrast with Thread-Per-Task**
| **Thread-Per-Task** | **Thread Pool** |
|---------------------|-----------------|
| `thread::spawn` per request | Fixed `N` workers upfront |
| Unbounded thread count | Bounded at pool size |
| High creation overhead | Amortized cost (reuse) |
| OS limits hit quickly | Controlled resource usage |
| Good for few, long tasks | Good for many, short tasks |

---

## 🔍 **Detailed Content**

### **Architecture Components**

```rust
// Simplified mental model (actual implementation in rust_book/Ch21/src/lib.rs)

pub struct ThreadPool {
    workers: Vec<Worker>,              // Fixed-size worker array
    sender: mpsc::Sender<Job>,         // Producer side: send jobs to queue
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,            // OS thread handle
}

type Job = Box<dyn FnOnce() + Send>;  // Closure trait object for flexibility
```

**Key invariants**:
1. **Workers share one receiver**: `Arc<Mutex<Receiver<Job>>>` allows multiple threads to `recv()` from same channel
2. **Jobs are owned closures**: `FnOnce` means task executes once and owns all captured data (`Send + 'static`)
3. **Pool size is fixed**: Workers are created at construction, never added/removed dynamically
4. **FIFO semantics**: First job sent is first job executed (assuming worker availability)

### **Implementation Deep Dive**

#### **1. Pool Creation**
```rust
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));  // Share across workers
        
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool { workers, sender: Some(sender) }
    }
}
```

**Why `Arc<Mutex<Receiver>>`?**
- **`Arc`**: Each worker needs shared ownership of the receiver (can't move/copy it)
- **`Mutex`**: Only one worker should call `recv()` at a time (not `Sync` by default)
- **Pattern**: Multiple consumers pulling from single queue (fan-out from channel)

#### **2. Worker Loop**
```rust
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // Lock receiver, blocking if another worker holds it
            let message = receiver.lock().unwrap().recv();
            
            match message {
                Ok(job) => {
                    println!("Worker {id} executing job");
                    job();  // Execute FnOnce closure
                }
                Err(_) => {
                    // Channel closed (all senders dropped)
                    println!("Worker {id} shutting down");
                    break;
                }
            }
        });
        
        Worker { id, thread }
    }
}
```

**Critical design choices**:
- **`recv()` blocks** - Worker sleeps until job arrives (no busy-waiting)
- **Lock scope minimal** - `recv()` returns owned `Job`, lock released immediately
- **Graceful shutdown** - Dropping all `Sender`s signals workers to exit via `Err(_)`

#### **3. Job Submission**
```rust
impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}
```

**Trait bounds decoded**:
- **`FnOnce()`**: Closure consumes captured variables, returns nothing
- **`Send`**: Safe to transfer across thread boundaries (no `Rc` or local refs)
- **`'static`**: No borrowed references (must own all data or use shared pointers like `Arc`)

**Job boxing**: `Box<dyn FnOnce() + Send>` creates trait object for dynamic dispatch (different closure types unified)

### **Concurrency Characteristics**

#### **Performance Model**
```
Pool size N, Job duration T, Arrival rate λ:

Throughput (ideal): N / T jobs per second
Actual throughput ≤ min(N/T, λ)  (bounded by workers or arrivals)

Queue depth grows when: λ > N/T  (arriving faster than processing)
Latency = queue_time + execution_time
  queue_time = 0 if workers available
  queue_time > 0 if all workers busy
```

**Sizing heuristics**:
- **CPU-bound tasks**: `pool_size = num_cpus()` (avoid oversubscription)
- **I/O-bound tasks**: `pool_size = 2 * num_cpus()` or higher (threads often blocked)
- **Mixed workloads**: Profile and tune based on queue depth metrics

#### **Comparison to Other Patterns**

| **Pattern** | **Resource Usage** | **Latency** | **Scalability** | **Use Case** |
|-------------|-------------------|-------------|-----------------|--------------|
| **Thread-per-task** | Unbounded (O(tasks)) | Low (immediate) | Poor (OS limits) | Few, long tasks |
| **Thread pool** | Bounded (O(workers)) | Queued if busy | Good (controlled) | Many, short tasks |
| **Async runtime** | Very low (few threads) | Event-loop delay | Excellent (100k+ tasks) | I/O-bound, high concurrency |
| **Work-stealing** | Balanced (dynamic) | Low (load balanced) | Excellent | CPU-bound, tree parallelism |

**Thread pool sweet spot**: 100s-1000s of short tasks where async overhead isn't worth it.

### **Complete Runnable Example: Web Server**

```rust
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
    sync::{mpsc, Arc, Mutex},
};

// Simplified ThreadPool (production version in rust_book/Ch21/src/lib.rs)
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Box<dyn FnOnce() + Send>>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers = (0..size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect();
        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}

struct Worker {
    _handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Box<dyn FnOnce() + Send>>>>) -> Self {
        let _handle = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();
            match job {
                Ok(job) => {
                    println!("Worker {id} executing");
                    job();
                }
                Err(_) => break,
            }
        });
        Worker { _handle }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);  // 4 concurrent connections max

    println!("Server listening with 4 workers");
    println!("Try: http://127.0.0.1:7878/");
    println!("     http://127.0.0.1:7878/sleep (5-second delay)");
    println!("\nOpen both URLs - / loads instantly even while /sleep is running!\n");

    for stream in listener.incoming().take(10) {  // Handle 10 requests then exit
        let stream = stream.unwrap();
        pool.execute(move || handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        _ => return,
    };

    let thread_id = thread::current().id();
    println!("📨 [{thread_id:?}] {request_line}");

    let (status, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            println!("💤 [{thread_id:?}] Sleeping 5 seconds...");
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = "<html><body><h1>Hello from thread pool!</h1></body></html>";
    let response = format!("{status}\r\nContent-Length: {}\r\n\r\n{contents}", contents.len());
    stream.write_all(response.as_bytes()).unwrap();
    println!("✅ [{thread_id:?}] Sent response\n");
}
```

**Testing concurrency**:
1. Run server: `cargo run`
2. Open browser tab 1: `http://127.0.0.1:7878/sleep` (blocks one worker for 5s)
3. **Immediately** open tab 2: `http://127.0.0.1:7878/` (loads instantly!)
4. Console shows different `ThreadId`s handling requests concurrently

**Why it works**: While Worker 1 is sleeping (simulating slow I/O), Worker 2 is free to handle the second request. Without the thread pool, the single-threaded server would block everyone until `/sleep` completes.

### **Common Pitfalls and Solutions**

#### **1. Pool Size Too Small**
```
Problem: All workers busy, jobs queue up → high latency
Symptom: Response time spikes under load
Solution: Increase pool size OR add more servers (horizontal scaling)
```

#### **2. Pool Size Too Large**
```
Problem: Context switching overhead, memory per thread
Symptom: Diminishing returns (16 threads not 2x faster than 8)
Solution: Profile actual CPU usage, tune to num_cpus for CPU-bound work
```

#### **3. Blocking Workers**
```rust
// ❌ BAD: Worker holds lock during I/O
let data = receiver.lock().unwrap();
let job = data.recv().unwrap();
// Lock still held! Other workers blocked!
job();

// ✅ GOOD: Release lock before executing
let job = {
    let data = receiver.lock().unwrap();
    data.recv().unwrap()
};  // Lock dropped here
job();  // Workers can proceed
```

#### **4. Panic Poisoning**
```rust
// If worker panics while holding Mutex, it poisons the lock
match receiver.lock() {
    Ok(guard) => guard.recv(),
    Err(poisoned) => {
        eprintln!("Worker panicked! Recovering...");
        poisoned.into_inner().recv()  // Recover or terminate
    }
}
```

#### **5. Graceful Shutdown**
```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());  // Close channel (workers see Err)
        
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();  // Wait for worker to finish current job
            }
        }
    }
}
```

### **Rust-Specific Advantages**

1. **Compile-time thread safety**: `Send + 'static` prevents capturing dangling references
2. **Zero-cost abstractions**: `Box<dyn FnOnce>` has same overhead as function pointers
3. **Automatic cleanup**: `Drop` trait ensures workers are joined and resources freed
4. **Panic isolation**: Worker panics don't crash other workers (mutex poisoning detected)
5. **Type-safe job queue**: Closures enforce correct capture semantics at compile time

---

## 💡 **Key Takeaways**

1. **Thread pools trade queueing latency for bounded resource usage** - Ideal when you have more tasks than cores and want controlled concurrency
2. **`Arc<Mutex<Receiver>>` is the idiomatic pattern** for multiple consumers sharing a work queue in Rust
3. **Pool size tuning is workload-dependent** - CPU-bound: ~num_cpus, I/O-bound: 2-4x num_cpus, profile to confirm
4. **Graceful shutdown requires channel closure** - Drop all senders to signal workers via `recv()` error
5. **Async runtimes supersede thread pools for I/O** - 1000s of tasks better handled by Tokio/async-std with few threads

---

## 🔗 **Integration Points**

### **Builds On**
- [[rust-threading-basics]] - `thread::spawn`, `JoinHandle`, ownership transfer across thread boundaries
- [[message-passing-channels]] - `mpsc::channel` for producer-consumer job distribution
- [[shared-state-concurrency]] - `Arc<Mutex<T>>` pattern for sharing receiver across workers
- [[Send and Sync Deep Dive]] - Trait bounds ensuring closure safety (`FnOnce + Send + 'static`)

### **Enables**
- [[async-vs-threads-decision]] - Understanding when to use thread pools vs async runtimes
- [[rayon-parallel-iterators]] - Work-stealing thread pool (advanced variant)
- [[performance-benchmarking]] - Tuning pool size based on throughput/latency metrics
- [[deterministic-debugging]] - Worker ID logging patterns for tracing concurrent execution

### **Related Concepts**
- [[async-concurrency]] - Modern alternative for I/O-bound workloads (event loop vs threads)
- [[handles-resource-abstraction]] - `JoinHandle` as resource management pattern
- [[drop-trait]] - Automatic cleanup of worker threads on pool destruction
- [[graceful-shutdown-patterns]] - Coordinated thread pool termination (Ch21.3)
- [[tcp-listener]] - Practical application in web servers (see [[rust_book/rust-book-ch21]])

### **Real-World Applications**
- **Web servers** - [[rust_book/rust-book-ch21]] implements multi-threaded HTTP server
- **AoC parallelism** - [[AoC Pattern Library]] uses thread pools for embarrassingly parallel stages
- **Job processors** - Background task systems (email sending, image processing)
- **Database connection pools** - Same pattern applied to DB connections instead of threads

---

*Tags: #concurrency #threading #design-patterns #rust-book-ch21 #arc-mutex #mpsc #performance #web-servers #resource-management*

*Links: [[zettel-index]] | [[rust-concurrency-moc]] | [[message-passing-channels]] | [[shared-state-concurrency]] | [[rust-threading-basics]] | [[async-vs-threads-decision]] | [[rust_book/rust-book-ch21]] | [[tcp-listener]] | [[graceful-shutdown-patterns]]*
