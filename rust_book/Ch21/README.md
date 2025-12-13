# Chapter 21: Final Project - Building a Multithreaded Web Server

**Reference**: [The Rust Book - Chapter 21](https://doc.rust-lang.org/stable/book/ch21-00-final-project-a-web-server.html)

This chapter brings together concepts from throughout the book to build a real web server that handles HTTP requests concurrently.

---

## 🎯 Chapter Goals

1. Build a single-threaded TCP web server
2. Understand HTTP request/response format
3. Implement thread pool for concurrent request handling
4. Graceful shutdown with proper cleanup

---

## 📂 Project Structure

```
Ch21/
├── Cargo.toml
├── README.md (this file)
├── src/
│   └── lib.rs               # ThreadPool implementation
├── examples/
│   ├── ch21_1_single_threaded.rs    # Single-threaded server
│   ├── ch21_2_multithreaded.rs      # Multithreaded server
│   └── ch21_3_graceful_shutdown.rs  # Graceful shutdown
├── web_server/              # Complete standalone server
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   └── lib.rs
│   └── static/
│       ├── hello.html
│       └── 404.html
├── hello.html               # Sample HTML for examples
└── 404.html                 # Error page for examples
```

---

## 🚀 Running the Examples

### Ch21.1: Single-Threaded Web Server

```bash
cargo run --example ch21_1_single_threaded
```

Open browser to `http://127.0.0.1:7878` or `http://127.0.0.1:7878/sleep`

**Demonstrates:**
- TCP listener on port 7878
- HTTP request parsing
- Responding with HTML content
- Simulated slow request (`/sleep` endpoint)
- **Issue**: Blocks on slow requests

### Ch21.2: Multithreaded Server with Thread Pool

```bash
cargo run --example ch21_2_multithreaded
```

**Demonstrates:**
- ThreadPool with 4 worker threads
- Concurrent request handling
- Message passing with channels
- Worker thread pattern
- **Improvement**: Multiple requests handled simultaneously

### Ch21.3: Graceful Shutdown

```bash
cargo run --example ch21_3_graceful_shutdown
```

Send requests, then Ctrl+C to trigger shutdown.

**Demonstrates:**
- Drop trait for cleanup
- Joining worker threads
- Signal handling (basic)
- Proper resource cleanup

---

## 🏗️ Complete Web Server

The `web_server/` directory contains a production-ready implementation:

```bash
cd web_server
cargo run
```

Features:
- Static file serving
- 404 error handling
- Thread pool with configurable size
- Graceful shutdown
- Request logging
- Multiple routes

---

## 🔑 Key Concepts

### 1. TCP Listener

```rust
use std::net::TcpListener;

let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

for stream in listener.incoming() {
    let stream = stream.unwrap();
    // Handle connection
}
```

**What it does:**
- Binds to address and port
- Listens for incoming TCP connections
- `incoming()` returns iterator of connection attempts

### 2. HTTP Request Format

```
GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: Mozilla/5.0
Accept: text/html
```

- **Request line**: `METHOD PATH HTTP_VERSION`
- **Headers**: Key-value pairs
- **Blank line**: Separator
- **Body**: (optional, for POST/PUT)

### 3. HTTP Response Format

```
HTTP/1.1 200 OK
Content-Length: 140
Content-Type: text/html

<!DOCTYPE html>
<html>
...
</html>
```

- **Status line**: `HTTP_VERSION STATUS_CODE REASON`
- **Headers**: Content-Length, Content-Type, etc.
- **Blank line**: Separator
- **Body**: HTML content

### 4. Thread Pool Pattern

```rust
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // Create workers with shared receiver
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}
```

**Architecture:**
- Fixed number of worker threads (thread pool)
- Work queue (MPSC channel)
- Workers wait for jobs from channel
- Better than spawning thread per request

### 5. Worker Pattern

```rust
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        });
        
        Worker { id, thread: Some(thread) }
    }
}
```

**Key points:**
- Shared receiver wrapped in `Arc<Mutex<>>`
- Each worker locks mutex to receive job
- Only one worker gets each job
- Loop keeps thread alive

### 6. Graceful Shutdown with Drop

```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down all workers.");
        
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

**Pattern:**
- Drop trait automatically called when ThreadPool goes out of scope
- Signal workers to stop (send termination message)
- Join all threads to wait for completion
- Ensures clean shutdown

### 7. Message Types for Shutdown

```rust
enum Message {
    NewJob(Job),
    Terminate,
}

// Worker loop
loop {
    let message = receiver.lock().unwrap().recv().unwrap();
    
    match message {
        Message::NewJob(job) => {
            println!("Worker {id} got a job; executing.");
            job();
        }
        Message::Terminate => {
            println!("Worker {id} was told to terminate.");
            break;
        }
    }
}
```

---

## 🎓 Learning Progression

### Single-Threaded Server (Ch21.1)

**Demonstrates:**
- TCP basics
- HTTP protocol understanding
- File I/O for serving HTML
- Request parsing

**Problem:** Blocks on slow requests (try `/sleep` endpoint)

### Multithreaded Server (Ch21.2)

**Demonstrates:**
- Thread pool creation
- MPSC channel for work queue
- Arc<Mutex<>> for shared ownership
- Closure as job (FnOnce trait)

**Improvement:** Concurrent request handling

### Graceful Shutdown (Ch21.3)

**Demonstrates:**
- Drop trait for RAII
- Termination signaling
- Thread joining
- Resource cleanup

**Completion:** Production-ready patterns

---

## 🧪 Testing the Server

### Test Single-Threaded Blocking

1. Start: `cargo run --example ch21_1_single_threaded`
2. Open two browser tabs
3. Tab 1: `http://127.0.0.1:7878/sleep` (5 second delay)
4. Tab 2: `http://127.0.0.1:7878` (immediate)
5. **Observe:** Tab 2 waits for Tab 1 to complete

### Test Multithreaded Concurrency

1. Start: `cargo run --example ch21_2_multithreaded`
2. Repeat tabs test above
3. **Observe:** Tab 2 loads immediately (concurrent handling)

### Test Graceful Shutdown

1. Start: `cargo run --example ch21_3_graceful_shutdown`
2. Make several requests
3. Press Ctrl+C
4. **Observe:** Workers shut down gracefully, threads joined

---

## 🔗 Connections to Previous Chapters

| **Concept** | **Chapter** | **Application** |
|-------------|-------------|-----------------|
| Ownership & Borrowing | Ch4 | TcpStream handling, buffer management |
| Error Handling | Ch9 | `Result` for network I/O, `unwrap()` usage |
| Traits | Ch10 | `FnOnce` for jobs, `Send` for thread safety |
| Closures | Ch13 | Jobs passed to thread pool |
| Smart Pointers | Ch15 | `Box<dyn Fn>`, `Arc<Mutex<>>` for shared state |
| Concurrency | Ch16 | Threads, channels, `Arc`, `Mutex` |
| Lifetimes | Ch10 | Static lifetime for thread-safe closures |

---

## 💡 Key Takeaways

1. **TCP is foundational**: HTTP built on top of TCP connections
2. **Thread pools > spawning threads**: Fixed overhead, controlled resources
3. **Message passing for work distribution**: MPSC channels coordinate workers
4. **Arc<Mutex<>> for shared state**: Safe concurrent access to receiver
5. **Drop for RAII**: Automatic cleanup when going out of scope
6. **Graceful shutdown is complex**: Requires termination signaling and joining

---

## 🚀 Extensions (Beyond Book)

**Possible improvements:**
- Request routing (match on path)
- Query parameter parsing
- POST request handling
- Static file serving from directory
- Logging middleware
- Configuration file
- HTTPS with TLS
- WebSocket support

**See `web_server/` for extended implementation**

---

## 📚 Additional Resources

- [RFC 7230 - HTTP/1.1 Message Syntax](https://tools.ietf.org/html/rfc7230)
- [Rust std::net documentation](https://doc.rust-lang.org/std/net/)
- [Thread Pool Pattern](https://en.wikipedia.org/wiki/Thread_pool)
- [RAII in Rust](https://doc.rust-lang.org/rust-by-example/scope/raii.html)

---

## ✅ Chapter Completion Checklist

- [ ] Understand TCP listener and HTTP protocol
- [ ] Build single-threaded web server
- [ ] Experience blocking problem firsthand
- [ ] Implement ThreadPool with workers
- [ ] Understand Arc<Mutex<Receiver>> pattern
- [ ] Add graceful shutdown with Drop
- [ ] Test concurrency improvements
- [ ] Explore complete web_server implementation

---

**Next:** Appendix sections (keywords, operators, traits, tools, editions)

*Chapter 21 demonstrates real-world application of Rust's concurrency and ownership features*
