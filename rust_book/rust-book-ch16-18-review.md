# Rust Book Chapters 16-18: Concurrency, Async, and OOP

> **Knowledge Integration**: Mastering fearless concurrency, async/await patterns, and object-oriented programming idioms in Rust

---

## 📚 Overview

This review covers three chapters that provide Rust's solutions to complex programming paradigms: safe concurrent programming with threads, modern async/await for I/O-bound operations, and Rust's approach to object-oriented design patterns. Together, these concepts enable building performant, safe applications that leverage multiple execution models while maintaining Rust's core guarantees.

**Chapter Coverage:**
- **Chapter 16**: Fearless Concurrency - Threads, message passing, and shared state
- **Chapter 17**: Async and Await - Futures, async runtime, and concurrent I/O
- **Chapter 18**: Object-Oriented Programming Features - Encapsulation, polymorphism, and state patterns

**Cross-References:**
- [[rust-book-ch13-15-review]] - Foundation: Closures (for thread spawning), smart pointers (Arc, Rc)
- [[zettelkasten/rust_book/rust-book-ch16]] - Threading and concurrency deep dive
- [[zettelkasten/rust_book/rust-book-ch17]] - Async programming comprehensive guide
- [[zettelkasten/rust_book/rust-book-ch18]] - OOP features in Rust
- [[zettelkasten/state-pattern-rust]] - State pattern implementations

---

## 🧵 Chapter 16: Fearless Concurrency

### **Core Philosophy**

Rust's ownership system prevents data races at compile time. Unlike other languages where concurrency bugs appear at runtime, Rust makes many concurrency errors impossible to compile. This is "fearless concurrency" - you can write concurrent code with confidence that the compiler will catch mistakes.

**Two Primary Communication Models:**
1. **Message Passing**: "Do not communicate by sharing memory; share memory by communicating" (Go proverb)
2. **Shared State**: Traditional mutex-based synchronization, made safe by Rust's type system

---

### **16.1 - Using Threads to Run Code Simultaneously**

#### **Thread Spawning Basics**

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Main thread work
    for i in 1..5 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for spawned thread to complete
    handle.join().unwrap();
}
```

#### **JoinHandle Placement Matters**

```rust
// Pattern 1: Sequential (spawned completes BEFORE main work)
let handle = thread::spawn(|| { /* work */ });
handle.join().unwrap();  // Block here
// main thread work after

// Pattern 2: Concurrent (both run, then wait)
let handle = thread::spawn(|| { /* work */ });
// main thread work runs concurrently
handle.join().unwrap();  // Wait at end
```

#### **Move Closures for Thread Ownership**

Threads need to own their data - references might outlive the main thread:

```rust
let v = vec![1, 2, 3];

// ❌ Won't compile - v might not live long enough
// let handle = thread::spawn(|| println!("{:?}", v));

// ✅ Transfer ownership with move
let handle = thread::spawn(move || {
    println!("vector: {:?}", v);
});

// v is no longer accessible here - ownership transferred
handle.join().unwrap();
```

**Key Insight**: The `move` keyword forces the closure to take ownership of captured variables, ensuring the thread has all data it needs without dangling references.

---

### **16.2 - Message Passing with Channels**

#### **Channel Fundamentals**

Rust's channels are **multiple producer, single consumer (mpsc)**:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // Create channel - tx (transmitter), rx (receiver)
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
        // val is moved, can't use it here
    });

    // recv() blocks until message received
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

#### **Ownership Transfer Through Channels**

**Critical Insight**: `send()` moves ownership - this prevents data races:

```rust
thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
    // println!("{}", val); // ❌ Compile error! val was moved
});
```

#### **Multiple Messages as Iterator**

```rust
thread::spawn(move || {
    let vals = vec!["hi", "from", "thread"];
    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_millis(200));
    }
});

// Receiver as iterator - blocks until channel closes
for received in rx {
    println!("Got: {}", received);
}
```

#### **Multiple Producers**

```rust
let (tx, rx) = mpsc::channel();
let tx1 = tx.clone(); // Clone the transmitter

thread::spawn(move || {
    tx.send("from thread 1").unwrap();
});

thread::spawn(move || {
    tx1.send("from thread 2").unwrap();
});

// Receive from both producers
for received in rx {
    println!("Got: {}", received);
}
```

---

### **16.3 - Shared-State Concurrency with Mutex**

#### **Mutex Basics**

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // Acquire lock
        *num = 6;
    } // Lock released when MutexGuard drops

    println!("m = {:?}", m);
}
```

**Key Properties:**
- `lock()` blocks until lock is available
- Returns `MutexGuard` smart pointer (Deref to inner data)
- Lock automatically released when guard goes out of scope
- `unwrap()` handles poisoned mutex (panic in another thread holding lock)

#### **Arc: Atomic Reference Counting for Threads**

`Rc<T>` is not thread-safe. Use `Arc<T>` for shared ownership across threads:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

**Pattern**: `Arc<Mutex<T>>` - Shared ownership (Arc) of mutable data (Mutex)

#### **Deadlock Example**

```rust
// Thread 1: locks A, then tries to lock B
// Thread 2: locks B, then tries to lock A
// Result: Deadlock - each waiting for the other

let a = Arc::new(Mutex::new(1));
let b = Arc::new(Mutex::new(2));

// Thread 1
let a1 = Arc::clone(&a);
let b1 = Arc::clone(&b);
thread::spawn(move || {
    let _a = a1.lock().unwrap();
    thread::sleep(Duration::from_millis(10));
    let _b = b1.lock().unwrap(); // Waits forever
});

// Thread 2
let a2 = Arc::clone(&a);
let b2 = Arc::clone(&b);
thread::spawn(move || {
    let _b = b2.lock().unwrap();
    thread::sleep(Duration::from_millis(10));
    let _a = a2.lock().unwrap(); // Waits forever
});
```

**Prevention**: Always acquire locks in consistent order across all threads.

---

### **16.4 - Send and Sync Traits**

#### **The Send Trait**

Types implementing `Send` can be transferred between threads:

```rust
// Most types implement Send automatically
// Notable exception: Rc<T> - not thread-safe reference counting

// ❌ Won't compile
// let rc = Rc::new(5);
// thread::spawn(move || println!("{}", rc));

// ✅ Use Arc instead
let arc = Arc::new(5);
thread::spawn(move || println!("{}", arc));
```

#### **The Sync Trait**

Types implementing `Sync` can be referenced from multiple threads:

```rust
// T is Sync if &T is Send
// Meaning: immutable references can be shared across threads

// RefCell<T> is NOT Sync - runtime borrow checking isn't thread-safe
// Mutex<T> IS Sync - provides thread-safe interior mutability
```

#### **Relationship Summary**

| Type | Send | Sync | Thread Usage |
|------|------|------|--------------|
| `i32`, `String` | ✅ | ✅ | Full thread support |
| `Rc<T>` | ❌ | ❌ | Single-threaded only |
| `Arc<T>` | ✅ | ✅ | Multi-threaded shared ownership |
| `RefCell<T>` | ✅ | ❌ | Can move to thread, can't share refs |
| `Mutex<T>` | ✅ | ✅ | Thread-safe interior mutability |

---

## ⚡ Chapter 17: Async and Await

### **Core Philosophy**

Async/await provides **cooperative multitasking** for I/O-bound operations. Unlike threads (OS-managed, preemptive), async tasks yield control voluntarily at `.await` points. This enables handling thousands of concurrent I/O operations with minimal overhead.

**Key Distinction from Threads:**
- **Threads**: OS-scheduled, preemptive, ~1MB stack each, good for CPU-bound parallelism
- **Async**: Runtime-scheduled, cooperative, minimal overhead, ideal for I/O-bound concurrency

---

### **17.1 - Futures and the Async Syntax**

#### **Basic Async Function**

```rust
async fn fetch_data() -> String {
    // This becomes a Future
    String::from("data")
}

// Calling returns a Future, doesn't execute
let future = fetch_data(); // Nothing happens yet

// Must be awaited or spawned to execute
let data = future.await; // Now it runs
```

#### **Async Blocks**

```rust
let future = async {
    let a = fetch_a().await;
    let b = fetch_b().await;
    a + b
};
```

#### **The Future Trait (Conceptual)**

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),   // Future completed with value
    Pending,    // Not ready, check again later
}
```

**How await works (conceptually):**
```rust
// When you write:
let result = some_future.await;

// It compiles to something like:
loop {
    match some_future.poll(cx) {
        Poll::Ready(value) => break value,
        Poll::Pending => yield_to_runtime(), // Let other tasks run
    }
}
```

---

### **17.2 - Concurrency with Async**

#### **Sequential vs Concurrent**

```rust
// Sequential - total time: 1.0 + 0.8 + 0.6 = 2.4s
let user = fetch_user(1).await;
let posts = fetch_posts(1).await;
let comments = fetch_comments(1).await;

// Concurrent with join! - total time: max(1.0, 0.8, 0.6) = 1.0s
let (user, posts, comments) = tokio::join!(
    fetch_user(1),
    fetch_posts(1),
    fetch_comments(1)
);
```

#### **join! vs select!**

| Macro | Behavior | Use Case |
|-------|----------|----------|
| `join!` | Wait for ALL to complete | Need all results |
| `select!` | Wait for FIRST to complete | Race, timeout, cancellation |

```rust
// select! - first future to complete wins
tokio::select! {
    result = fetch_from_server_a() => println!("A won: {:?}", result),
    result = fetch_from_server_b() => println!("B won: {:?}", result),
}

// Timeout pattern
tokio::select! {
    result = long_operation() => handle_result(result),
    _ = tokio::time::sleep(Duration::from_secs(5)) => println!("Timeout!"),
}
```

#### **Spawning Tasks**

```rust
// spawn_task - runs concurrently, returns JoinHandle
let handle = tokio::spawn(async {
    expensive_computation().await
});

// Do other work while task runs
other_work().await;

// Get result when needed
let result = handle.await.unwrap();
```

---

### **17.3 - Working with Any Number of Futures**

#### **Streams: Async Iterators**

```rust
use tokio_stream::StreamExt;

let mut stream = tokio_stream::iter(vec![1, 2, 3]);

while let Some(value) = stream.next().await {
    println!("Got: {}", value);
}
```

#### **Processing Multiple Futures Dynamically**

```rust
use futures::stream::{self, StreamExt};

let urls = vec!["url1", "url2", "url3"];

let results: Vec<_> = stream::iter(urls)
    .map(|url| async move { fetch(url).await })
    .buffer_unordered(3) // Process 3 concurrently
    .collect()
    .await;
```

---

### **17.4 - Streams: Futures in Sequence**

#### **Stream Trait (Conceptual)**

```rust
pub trait Stream {
    type Item;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) 
        -> Poll<Option<Self::Item>>;
}
```

**Relationship to Iterator:**
- `Iterator::next()` → `Option<Item>` (sync)
- `Stream::poll_next()` → `Poll<Option<Item>>` (async)

#### **Common Stream Operations**

```rust
stream
    .filter(|x| async move { x % 2 == 0 })
    .map(|x| async move { x * 2 })
    .take(5)
    .collect::<Vec<_>>()
    .await
```

---

### **17.5 - Traits for Async: Future, Pin, Unpin**

#### **Why Pin Exists**

Async blocks can create self-referential structures:

```rust
async {
    let data = vec![1, 2, 3];
    let reference = &data[0]; // Reference to data
    some_await_point().await;  // Might move the future!
    println!("{}", reference); // Would be invalid if moved
}
```

**Pin prevents moving the future after it's polled**, ensuring internal references remain valid.

#### **Unpin Trait**

- Most types implement `Unpin` automatically (safe to move when pinned)
- Self-referential futures are `!Unpin` (must stay pinned)
- You rarely interact with Pin directly - `async`/`await` handles it

---

### **17.6 - Futures, Tasks, and Threads**

#### **When to Use Each**

| Model | Best For | Overhead | Parallelism |
|-------|----------|----------|-------------|
| **Async Tasks** | I/O-bound (network, files) | Very low | Concurrent, not parallel |
| **OS Threads** | CPU-bound computation | ~1MB stack | True parallelism |
| **Hybrid** | Mixed workloads | Varies | Both |

#### **Mixing Threads and Async**

```rust
// CPU-bound work in thread, async for coordination
let handle = tokio::task::spawn_blocking(|| {
    // Heavy CPU work that would block the async runtime
    expensive_sync_computation()
});

let result = handle.await.unwrap();
```

---

## 🎭 Chapter 18: Object-Oriented Programming Features

### **Core Philosophy**

Rust isn't traditionally OOP, but supports OOP patterns through its type system. Rather than inheritance hierarchies, Rust favors **composition and traits**. The chapter explores what OOP concepts Rust supports and how to implement common patterns idiomatically.

---

### **18.1 - Characteristics of Object-Oriented Languages**

#### **What Rust Supports**

| OOP Concept | Rust Support | Mechanism |
|-------------|--------------|-----------|
| **Encapsulation** | ✅ Full | `pub` visibility, modules |
| **Inheritance** | ⚠️ Limited | Trait default methods only |
| **Polymorphism** | ✅ Full | Traits + generics or trait objects |

#### **Encapsulation Example**

```rust
pub struct AveragedCollection {
    list: Vec<i32>,      // Private
    average: f64,        // Private
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average(); // Invariant maintained
    }

    pub fn average(&self) -> f64 {
        self.average // Read-only access
    }

    fn update_average(&mut self) { // Private helper
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

**Key Insight**: Fields are private by default. Even with `pub struct`, fields remain private unless explicitly marked `pub`.

---

### **18.2 - Using Trait Objects for Polymorphism**

#### **Trait Objects with dyn**

```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // Heterogeneous collection
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw(); // Dynamic dispatch
        }
    }
}
```

#### **Static vs Dynamic Dispatch**

| Approach | Syntax | Performance | Flexibility |
|----------|--------|-------------|-------------|
| **Static (Generics)** | `fn draw<T: Draw>(item: &T)` | Zero-cost, inlined | Homogeneous only |
| **Dynamic (Trait Objects)** | `fn draw(item: &dyn Draw)` | Vtable lookup | Heterogeneous |

```rust
// Static dispatch - monomorphized at compile time
fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// Dynamic dispatch - runtime vtable lookup
fn notify(item: &dyn Summary) {
    println!("{}", item.summarize());
}
```

#### **Object Safety Rules**

A trait is object-safe if:
1. Return type isn't `Self`
2. No generic type parameters in methods

```rust
// ❌ NOT object-safe
trait Clone {
    fn clone(&self) -> Self; // Returns Self
}

// ✅ Object-safe
trait Draw {
    fn draw(&self); // No Self in return, no generics
}
```

---

### **18.3 - Implementing the State Pattern**

#### **Approach 1: OOP State Pattern (Runtime)**

Traditional OOP with trait objects:

```rust
pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str { "" }
}

pub struct Draft {}
pub struct PendingReview {}
pub struct Published {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // Can't approve draft
    }
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
}
```

**Trade-offs:**
- ✅ Runtime flexibility, extensible
- ✅ Familiar to OOP developers
- ❌ Box allocation overhead
- ❌ Invalid transitions fail silently

#### **Approach 2: Type States (Compile-Time)**

Encode states as types - invalid transitions won't compile:

```rust
pub struct DraftPost { content: String }
pub struct PendingReviewPost { content: String }
pub struct Post { content: String }

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { content: self.content }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post { content: self.content }
    }
    
    pub fn reject(self) -> DraftPost {
        DraftPost { content: self.content }
    }
}

impl Post {
    pub fn content(&self) -> &str {
        &self.content
    }
}
```

**Compile-Time Safety:**
```rust
let draft = DraftPost::new();
draft.approve();     // ❌ COMPILE ERROR - no such method
draft.content();     // ❌ COMPILE ERROR - no such method

let pending = draft.request_review();
pending.add_text("x"); // ❌ COMPILE ERROR - no such method
```

**Trade-offs:**
- ✅ Invalid transitions are compile errors
- ✅ Zero runtime overhead
- ✅ Self-documenting API
- ❌ Can't store mixed states in collections
- ❌ State must be known at compile time

#### **Comparison Summary**

| Criterion | OOP State Pattern | Type States |
|-----------|-------------------|-------------|
| **Invalid transition** | Silent no-op | Compile error |
| **Performance** | Box + vtable | Zero-cost |
| **Mixed collections** | ✅ Yes | ❌ No |
| **Runtime flexibility** | ✅ Yes | ❌ No |
| **Rust idiom** | OOP-familiar | "Rusty" |

**Recommendation**: Prefer type states when state transitions are known at compile time. Use OOP pattern when runtime flexibility or heterogeneous collections are required.

---

## 🔗 Integration: Connecting the Chapters

### **Thread Safety and Async**

```rust
// Arc<Mutex<T>> works in both sync and async contexts
let shared = Arc::new(Mutex::new(0));

// In threads
let shared_clone = Arc::clone(&shared);
thread::spawn(move || {
    *shared_clone.lock().unwrap() += 1;
});

// In async (with tokio::sync::Mutex for async-aware locking)
let async_shared = Arc::new(tokio::sync::Mutex::new(0));
tokio::spawn(async move {
    *async_shared.lock().await += 1;
});
```

### **When to Use Each Concurrency Model**

| Scenario | Best Choice | Reason |
|----------|-------------|--------|
| CPU-intensive computation | Threads | True parallelism |
| Many network requests | Async | Low overhead, thousands of connections |
| File I/O | Either | Depends on access pattern |
| GUI applications | Async on main + threads for work | Responsive UI |
| Mixed workloads | Hybrid | `spawn_blocking` for CPU work in async |

### **OOP Patterns in Concurrent Code**

```rust
// Trait objects for heterogeneous task handling
trait Task: Send + Sync {
    fn execute(&self);
}

struct Worker {
    tasks: Vec<Box<dyn Task>>,
}

impl Worker {
    fn run_all(&self) {
        for task in &self.tasks {
            task.execute();
        }
    }
}
```

---

## 📊 Quick Reference

### **Chapter 16: Threading Essentials**

```rust
// Spawn thread
let handle = thread::spawn(move || { /* work */ });
handle.join().unwrap();

// Channel
let (tx, rx) = mpsc::channel();
tx.send(value).unwrap();
let received = rx.recv().unwrap();

// Shared state
let counter = Arc::new(Mutex::new(0));
let mut num = counter.lock().unwrap();
```

### **Chapter 17: Async Essentials**

```rust
// Async function
async fn fetch() -> Data { /* ... */ }

// Concurrent execution
let (a, b, c) = tokio::join!(fut_a, fut_b, fut_c);

// Racing futures
tokio::select! {
    result = fut_a => handle_a(result),
    result = fut_b => handle_b(result),
}
```

### **Chapter 18: OOP Essentials**

```rust
// Trait object
let items: Vec<Box<dyn Draw>> = vec![
    Box::new(Button { label: "OK" }),
    Box::new(SelectBox { options: vec![] }),
];

// Type state
let draft = Post::new();
let pending = draft.request_review(); // Type changes!
let published = pending.approve();    // Type changes!
```

---

## 🎯 Key Takeaways

### **Chapter 16**
1. **Move closures** transfer ownership to threads
2. **Channels** transfer ownership through messages
3. **Arc<Mutex<T>>** for shared mutable state
4. **Send/Sync** traits define thread safety

### **Chapter 17**
1. **Async is cooperative** - yields at await points
2. **join! for all**, **select! for first**
3. **Streams** are async iterators
4. **Pin** prevents self-referential futures from moving

### **Chapter 18**
1. **Encapsulation** via pub/private fields
2. **Polymorphism** via trait objects (dyn) or generics
3. **Type states** encode invariants in the type system
4. **Composition over inheritance** is the Rust way

---

## 📚 Related Resources

### **Zettelkasten Connections**
- [[state-pattern-rust]] - Deep dive on OOP vs Type State patterns
- [[trait-objects-polymorphism]] - Comprehensive trait object guide
- [[future-trait-deep-dive]] - Pin, Unpin, and Future internals
- [[async-streams]] - Stream processing patterns

### **Code Examples**
- `rust_book/Ch16/` - Threading examples with threads, channels, mutex
- `rust_book/Ch17/` - Async examples with tokio runtime
- `rust_book/Ch18/` - OOP patterns with exercises

### **Mission Connections**
- [[mission-8]] - Graph algorithms use trait-based design (Ch18)
- [[mission-6]] - Grid infrastructure used in async contexts

---

*Tags: #concurrency #async #oop #threads #futures #state-pattern #rust-book*

*Links: [[rust-book-ch13-15-review]] | [[rust-book-ch16]] | [[rust-book-ch17]] | [[rust-book-ch18]] | [[state-pattern-rust]] | [[trait-objects-polymorphism]]*
