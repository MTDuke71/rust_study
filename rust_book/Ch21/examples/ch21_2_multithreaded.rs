/// Chapter 21.2: Multithreaded Server with Thread Pool
/// 
/// This example demonstrates:
/// - ThreadPool abstraction for managing worker threads
/// - Concurrent request handling
/// - Message passing with MPSC channels
/// - Arc<Mutex<>> for shared receiver

use rust_book_ch21::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("❌ Port 7878 already in use! Kill existing server or use different port.");
    
    // Create thread pool with 4 worker threads
    let pool = ThreadPool::new(4);

    println!("🦀 Multithreaded server listening on http://127.0.0.1:7878");
    println!("🧵 Thread pool created with 4 workers");
    println!("\n📝 Try these URLs:");
    println!("   http://127.0.0.1:7878/        - Normal response");
    println!("   http://127.0.0.1:7878/sleep   - Slow response (5 seconds)");
    println!("\n✅ IMPROVEMENT: Multiple requests handled concurrently!");
    println!("   Open two tabs: one to /sleep, one to /");
    println!("   You'll see the / request loads immediately!\n");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // Execute connection handler on thread pool
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    
    // Handle empty connections (browser pre-connections, early disconnects)
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => {
            eprintln!("❌ Error reading request: {}", e);
            return;
        }
        None => {
            // Connection closed before sending data - this is normal
            return;
        }
    };

    // Get thread ID for logging
    let thread_id = thread::current().id();
    println!("📨 Request: {} (handled by {:?})", request_line, thread_id);

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "rust_book/Ch21/hello.html"),
        "GET /favicon.svg HTTP/1.1" => ("HTTP/1.1 200 OK", "rust_book/Ch21/favicon.svg"),
        "GET /sleep HTTP/1.1" => {
            println!("💤 Worker {:?} sleeping for 5 seconds...", thread_id);
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "rust_book/Ch21/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "rust_book/Ch21/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
    println!("✅ Worker {:?} sent response: {}\n", thread_id, status_line);
}

/* 
=== Key Learning Points ===

1. Thread Pool Pattern:
   - Fixed number of worker threads (4 in this example)
   - Workers wait for jobs from a shared queue
   - Reuses threads instead of creating new ones
   - Controlled resource usage

2. ThreadPool::new(4):
   - Creates 4 worker threads upfront
   - Each worker shares access to job receiver
   - Uses Arc<Mutex<Receiver>> for safe sharing
   - Workers loop waiting for jobs

3. pool.execute(closure):
   - Closure must be FnOnce() + Send + 'static
   - FnOnce: Called once (move semantics)
   - Send: Safe to send across thread boundaries
   - 'static: No borrowed references (owns all data)

4. Concurrency Improvement:
   Single-threaded:
   - Request 1 (/sleep): 5 seconds
   - Request 2 (/): Waits 5 seconds, then responds
   - Total: 10+ seconds for 2 requests

   Multithreaded:
   - Request 1 (/sleep): 5 seconds (Worker 1)
   - Request 2 (/): Immediate (Worker 2)
   - Total: ~5 seconds for 2 requests (concurrent!)

5. Architecture:
   
   ┌──────────────┐
   │ TcpListener  │
   └──────┬───────┘
          │ for stream in listener.incoming()
          ▼
   ┌──────────────┐
   │ ThreadPool   │
   │              │
   │ ┌──────────┐ │
   │ │ Worker 1 │ │  ← Waiting for jobs
   │ ├──────────┤ │
   │ │ Worker 2 │ │  ← Executing job
   │ ├──────────┤ │
   │ │ Worker 3 │ │  ← Waiting for jobs
   │ ├──────────┤ │
   │ │ Worker 4 │ │  ← Waiting for jobs
   │ └──────────┘ │
   └──────────────┘
          ▲
          │ pool.execute(|| handle_connection(stream))
          │
   ┌──────────────┐
   │ MPSC Channel │  Job queue
   └──────────────┘

6. Message Passing (MPSC):
   - Multiple Producer, Single Consumer channel
   - ThreadPool sends jobs to channel
   - Workers receive jobs from shared receiver
   - Only one worker gets each job

7. Arc<Mutex<Receiver>>:
   - Arc: Atomic Reference Counted (shared ownership)
   - Mutex: Mutual exclusion (one thread at a time)
   - Receiver: Receives jobs from channel
   - Pattern: Multiple threads safely share one receiver

=== Testing Concurrency ===

Terminal:
$ cargo run --example ch21_2_multithreaded

Browser:
1. Open Tab 1: http://127.0.0.1:7878/sleep
2. Immediately open Tab 2: http://127.0.0.1:7878
3. Observe: Tab 2 loads immediately! ✅

Console output:
📨 Request: GET /sleep HTTP/1.1 (handled by ThreadId(2))
💤 Worker ThreadId(2) sleeping for 5 seconds...
📨 Request: GET / HTTP/1.1 (handled by ThreadId(3))
✅ Worker ThreadId(3) sent response: HTTP/1.1 200 OK

Worker 3 handles / while Worker 2 is sleeping!

=== ThreadPool Implementation Highlights ===

From src/lib.rs:

```rust
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Message>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool { workers, sender: Some(sender) }
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap()
            .send(Message::NewJob(job))
            .unwrap();
    }
}
```

Worker loop:
```rust
thread::spawn(move || loop {
    let message = receiver.lock().unwrap().recv();
    match message {
        Ok(Message::NewJob(job)) => job(),
        _ => break,
    }
})
```

=== Performance Considerations ===

Thread Pool Size:
- Too few threads: Requests still queue up
- Too many threads: Context switching overhead
- Rule of thumb: Number of CPU cores or slightly higher

For our example:
- 4 threads handles up to 4 concurrent requests
- 5th request waits for available worker
- Good balance for typical workloads

=== Connection to Chapter 16 ===

This chapter combines concepts from Ch16:
- thread::spawn (16.1)
- mpsc::channel (16.2)
- Arc and Mutex (16.3)
- Send and Sync traits (16.4)

Real-world application of concurrency primitives!

=== Next Steps ===

Run ch21_3_graceful_shutdown.rs to see proper cleanup with Drop trait!
*/
