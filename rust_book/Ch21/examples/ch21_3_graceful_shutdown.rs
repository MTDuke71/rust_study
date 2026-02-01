/// Chapter 21.3: Graceful Shutdown and Cleanup
///
/// This example demonstrates:
/// - Drop trait for RAII (Resource Acquisition Is Initialization)
/// - Graceful shutdown by sending Terminate messages
/// - Joining worker threads to ensure completion
/// - Proper resource cleanup
use rust_book_ch21::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    println!("🦀 Graceful shutdown server listening on http://127.0.0.1:7878");
    println!("🧵 Thread pool created with 4 workers");
    println!("\n📝 Try these URLs:");
    println!("   http://127.0.0.1:7878/");
    println!("   http://127.0.0.1:7878/sleep");
    println!("\n✨ NEW: Graceful shutdown when server stops!");
    println!("   Press Ctrl+C to see workers shut down cleanly.\n");

    // Take only 4 requests (2 source and 2 icons) for demonstration, then exit
    // In production, this would run indefinitely until interrupted
    for stream in listener.incoming().take(4) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("\n🛑 Server shutting down...");
    println!("   (ThreadPool Drop will be called)\n");

    // When pool goes out of scope here, Drop::drop is called
    // This triggers graceful shutdown
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

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

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    println!("✅ Worker {:?} sent response: {}\n", thread_id, status_line);
}

/*
=== Key Learning Points ===

1. The Drop Trait:
   ```rust
   impl Drop for ThreadPool {
       fn drop(&mut self) {
           println!("Shutting down all workers.");

           for worker in &mut self.workers {
               if let Some(thread) = worker.thread.take() {
                   thread.join().unwrap();
               }
           }
       }
   }
   ```

   - Automatically called when ThreadPool goes out of scope
   - RAII: Resource cleanup tied to object lifetime
   - No manual cleanup needed (Rust's safety guarantee)

2. Graceful Shutdown Steps:

   Step 1: Drop sender
   - Closes the channel
   - Workers' recv() returns Err
   - Workers know to exit their loops

   Step 2: Join threads
   - worker.thread.take() moves thread out of Option
   - thread.join() waits for thread to finish
   - Ensures all work completes before main exits

3. Why Option<JoinHandle>?
   ```rust
   struct Worker {
       id: usize,
       thread: Option<thread::JoinHandle<()>>,
   }
   ```

   - Option allows taking ownership with .take()
   - Can only join a thread once (consumes the handle)
   - take() replaces Some(handle) with None
   - Prevents double-join errors

4. Message Passing for Shutdown:

   Before shutdown:
   ThreadPool      Workers
   ─────────      ───────
   sender ───┬──> receiver (Worker 1) ← Waiting for jobs
             ├──> receiver (Worker 2) ← Processing job
             ├──> receiver (Worker 3) ← Waiting for jobs
             └──> receiver (Worker 4) ← Waiting for jobs

   During shutdown:
   1. drop(sender)          ← Channel closes
   2. Workers see recv() Err
   3. Workers break from loop
   4. Threads finish
   5. main calls thread.join()

5. RAII Pattern:

   Without RAII (manual cleanup):
   ```rust
   let pool = ThreadPool::new(4);
   // Use pool...
   pool.shutdown(); // ❌ Easy to forget!
   ```

   With RAII (automatic):
   ```rust
   let pool = ThreadPool::new(4);
   // Use pool...
   // Drop called automatically ✅
   ```

=== Testing Graceful Shutdown ===

Terminal:
$ cargo run --example ch21_3_graceful_shutdown

Expected output:
🦀 Graceful shutdown server listening on http://127.0.0.1:7878
🧵 Thread pool created with 4 workers
...
📨 Request: GET / HTTP/1.1 (handled by ThreadId(2))
✅ Worker ThreadId(2) sent response: HTTP/1.1 200 OK

📨 Request: GET / HTTP/1.1 (handled by ThreadId(3))
✅ Worker ThreadId(3) sent response: HTTP/1.1 200 OK

🛑 Server shutting down...
   (ThreadPool Drop will be called)

Shutting down all workers.
Shutting down worker 0
Worker 0 disconnected; shutting down.
Shutting down worker 1
Worker 1 disconnected; shutting down.
Shutting down worker 2
Worker 2 disconnected; shutting down.
Shutting down worker 3
Worker 3 disconnected; shutting down.

Clean exit! All workers finished gracefully.

=== Shutdown Sequence Diagram ===

1. Main Thread:
   ```
   {
       let pool = ThreadPool::new(4);  // Create pool

       // Handle 4 requests...

   } // pool goes out of scope → Drop::drop() called
   ```

2. Drop Implementation:
   ```
   fn drop(&mut self) {
       drop(self.sender.take());  // Close channel

       for worker in &mut self.workers {
           // Wait for each worker to finish
           worker.thread.take().unwrap().join().unwrap();
       }
   }
   ```

3. Worker Thread:
   ```
   loop {
       match receiver.lock().unwrap().recv() {
           Ok(Message::NewJob(job)) => job(),
           Err(_) => break,  // Channel closed, exit loop
       }
   }
   // Thread exits, join() returns
   ```

=== Production Considerations ===

1. Signal Handling:
   In production, you'd want:
   ```rust
   use signal_hook::{consts::SIGINT, iterator::Signals};

   let mut signals = Signals::new(&[SIGINT]).unwrap();

   thread::spawn(move || {
       for sig in signals.forever() {
           println!("Received signal {:?}", sig);
           // Trigger shutdown
       }
   });
   ```

2. Timeout for Shutdown:
   ```rust
   use std::time::Duration;

   for worker in &mut self.workers {
       if let Some(thread) = worker.thread.take() {
           // Give worker 5 seconds to finish
           match thread.join_timeout(Duration::from_secs(5)) {
               Ok(_) => println!("Worker {} finished", worker.id),
               Err(_) => println!("Worker {} timed out!", worker.id),
           }
       }
   }
   ```

3. Pending Request Handling:
   ```rust
   // Drain remaining jobs from channel
   while let Ok(job) = receiver.try_recv() {
       println!("Warning: Dropping pending job");
   }
   ```

=== Comparison: With vs Without Graceful Shutdown ===

Without Graceful Shutdown (abrupt termination):
- Workers killed mid-request ❌
- Partial responses sent ❌
- Resources leaked ❌
- Client sees broken connection ❌

With Graceful Shutdown (Drop trait):
- In-progress requests complete ✅
- Full responses sent ✅
- Resources cleaned up ✅
- Clean client disconnect ✅

=== Connection to Rust's Ownership ===

This example perfectly demonstrates Rust's ownership guarantees:

1. No manual memory management
2. No resource leaks
3. Thread safety enforced at compile time
4. Cleanup happens automatically

Compare to C/C++:
```c++
// C++ - Easy to forget cleanup
ThreadPool* pool = new ThreadPool(4);
// Use pool...
delete pool; // ❌ Might forget this!
```

Rust:
```rust
// Rust - Automatic cleanup
let pool = ThreadPool::new(4);
// Use pool...
// Drop called automatically ✅
```

=== Summary: Evolution Across Examples ===

Ch21.1 - Single-threaded:
- Problem: Blocking
- Learning: TCP basics, HTTP protocol

Ch21.2 - Multithreaded:
- Solution: Thread pool
- Learning: Concurrency, message passing

Ch21.3 - Graceful shutdown:
- Enhancement: Clean cleanup
- Learning: RAII, Drop trait, joining threads

=== Real-World Applications ===

This pattern is used in:
- Web servers (Actix, Rocket, Axum)
- Database connection pools
- Task queues (Tokio runtime)
- Any multi-threaded resource management

The principles learned here apply broadly to concurrent systems!

=== Further Exploration ===

1. Async/await alternative (Ch17):
   - tokio runtime for async I/O
   - More efficient for I/O-bound work
   - Non-blocking concurrency

2. Production web frameworks:
   - Actix-web: Actor-based
   - Rocket: Macro-based routing
   - Axum: Built on tokio, type-safe

3. Advanced threading:
   - Work stealing (rayon)
   - Lock-free data structures
   - Thread-local storage

This completes the Ch21 learning journey! 🎉
*/
