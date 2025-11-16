# Chapter 16: Fearless Concurrency

**Handling concurrent programming safely and efficiently in Rust**

---

## 📖 **Chapter Overview**

Chapter 16 introduces Rust's powerful concurrency model, which enables safe concurrent programming through the ownership system and type checker. Learn how to write multi-threaded programs without data races.

### **Sections**
- **16.1**: Using Threads to Run Code Simultaneously
- **16.2**: Using Message Passing to Transfer Data Between Threads
- **16.3**: Shared-State Concurrency
- **16.4**: Extensible Concurrency with the Sync and Send Traits

---

## 🎯 **Learning Objectives**

By completing this chapter, you will:
- ✅ **Spawn threads** to run multiple computations concurrently
- ✅ **Use message passing** with channels for thread communication
- ✅ **Share state** safely using Arc<Mutex<T>> and RwLock<T>
- ✅ **Understand Sync and Send** traits for thread safety guarantees
- ✅ **Avoid data races** at compile time through Rust's ownership system
- ✅ **Apply concurrency patterns** to real-world problems

---

## 📂 **Section Structure**

```
Ch16/
├── README.md                        # This file
├── threads/                         # 16.1 - Thread basics
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                  # Basic thread spawning
│       └── examples/
│           ├── join_handles.rs      # Waiting for threads
│           ├── move_closures.rs     # Moving ownership to threads
│           └── thread_spawning.rs   # Multiple thread patterns
├── message_passing/                 # 16.2 - Channels
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                  # Basic channel usage
│       └── examples/
│           ├── multiple_senders.rs  # Multiple producer channels
│           ├── mpsc_basics.rs       # Message passing channel
│           └── producer_consumer.rs # Producer-consumer pattern
├── shared_state/                    # 16.3 - Shared state
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                  # Mutex basics
│       └── examples/
│           ├── arc_mutex.rs         # Arc<Mutex<T>> pattern
│           ├── deadlock.rs          # Deadlock demonstration
│           ├── mutex_basics.rs      # Basic mutex usage
│           └── rwlock.rs            # Read-write locks
└── sync_send/                       # 16.4 - Sync and Send traits
    ├── Cargo.toml
    └── src/
        ├── main.rs                  # Sync/Send examples
        └── examples/
            ├── custom_types.rs      # Custom thread-safe types
            ├── send_trait.rs        # Send trait examples
            └── sync_trait.rs        # Sync trait examples
```

---

## 🚀 **Quick Start**

### **Running Examples**
```bash
# Section 16.1 - Threads
cd rust_book/Ch16/threads
cargo run                           # Basic thread spawning
cargo run --example join_handles    # Thread joining
cargo run --example move_closures   # Moving data to threads

# Section 16.2 - Message Passing
cd rust_book/Ch16/message_passing
cargo run                           # Basic channels
cargo run --example multiple_senders # Multiple producers
cargo run --example producer_consumer # Producer-consumer

# Section 16.3 - Shared State
cd rust_book/Ch16/shared_state
cargo run                           # Mutex basics
cargo run --example arc_mutex       # Arc<Mutex<T>> pattern
cargo run --example rwlock          # Read-write locks

# Section 16.4 - Sync and Send
cd rust_book/Ch16/sync_send
cargo run                           # Sync/Send traits
cargo run --example custom_types    # Thread-safe custom types
```

---

## 📚 **Section Details**

### **16.1: Using Threads to Run Code Simultaneously**

**Key Concepts:**
- Thread spawning with `std::thread::spawn`
- Join handles and waiting for threads
- Move closures to transfer ownership
- Thread safety guarantees

**Examples:**
```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

**Common Patterns:**
- Spawning threads for parallel computation
- Using `move` to transfer ownership to threads
- Joining threads to wait for completion
- Thread panics and error handling

---

### **16.2: Using Message Passing to Transfer Data Between Threads**

**Key Concepts:**
- Multiple producer, single consumer (mpsc) channels
- Sending and receiving messages
- Channel ownership and move semantics
- Multiple senders with cloning

**Examples:**
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

**Common Patterns:**
- Producer-consumer workflows
- Pipeline processing with multiple channels
- Broadcast patterns with multiple receivers (using other crates)
- Non-blocking receives with `try_recv()`

---

### **16.3: Shared-State Concurrency**

**Key Concepts:**
- Mutex<T> for mutual exclusion
- Arc<T> for atomic reference counting
- RwLock<T> for read-write locks
- Deadlock avoidance strategies

**Examples:**
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

**Common Patterns:**
- Shared counters and state
- Thread-safe caches
- Read-heavy workloads with RwLock
- Interior mutability with Arc<Mutex<T>>

---

### **16.4: Extensible Concurrency with Sync and Send**

**Key Concepts:**
- `Send` trait for ownership transfer between threads
- `Sync` trait for safe reference sharing
- Automatic trait implementation
- Manual unsafe implementations

**Key Points:**
- Almost all Rust types implement `Send`
- `Rc<T>` does NOT implement `Send` (use `Arc<T>`)
- Types that implement `Sync` allow `&T` to be `Send`
- Primitive types are both `Send` and `Sync`

**Examples:**
```rust
// Send: Can transfer ownership between threads
fn send_example<T: Send>(value: T) {
    std::thread::spawn(move || {
        // value is moved to this thread
        drop(value);
    });
}

// Sync: Safe to share references between threads
fn sync_example<T: Sync>(value: &T) {
    let value_ref = value;
    std::thread::spawn(move || {
        // Can use value_ref safely
    });
}
```

---

## 🔗 **Integration with Learning Tracks**

### **Mission Connections**
- **Mission 10**: Union-Find with thread-safe operations
- **Future Missions**: Parallel graph algorithms, concurrent data structures

### **Daily Study Integration**
- **Week 8, Day 50**: Thread basics and thread safety
- **Week 8, Day 51**: Message passing with channels
- **Week 8, Day 52**: Shared state with Arc<Mutex<T>>
- **Week 8, Day 53**: RwLock and atomic operations

### **AoC Applications**
- Parallel processing of independent AoC test cases
- Concurrent parsing and processing pipelines
- Thread-safe result collection and aggregation

### **Zettelkasten Links**
- [[rust-threading-basics]] - Thread spawning and joining
- [[message-passing-channels]] - Channel patterns
- [[shared-state-concurrency]] - Arc<Mutex<T>> patterns
- [[sync-send-traits]] - Thread safety guarantees

---

## 🎓 **Key Takeaways**

### **Fearless Concurrency Principles**
1. **Ownership prevents data races** - Compiler enforces thread safety
2. **Choose the right pattern** - Message passing vs shared state
3. **Understand Send and Sync** - Know what can cross thread boundaries
4. **Avoid deadlocks** - Lock ordering and timeout strategies

### **When to Use Each Pattern**

**Use Threads When:**
- Independent parallel computations
- CPU-bound work that benefits from parallelism
- Background tasks that don't block main thread

**Use Message Passing When:**
- Producer-consumer workflows
- Pipeline processing
- Clear ownership transfer between stages

**Use Shared State When:**
- Multiple threads need read/write access
- Shared counters or caches
- Complex state that can't be easily messaged

---

## 🧪 **Exercises**

### **Exercise 1: Parallel Computation**
Spawn multiple threads to compute different parts of a large computation, then collect results.

### **Exercise 2: Producer-Consumer Pipeline**
Create a multi-stage pipeline with channels connecting each stage.

### **Exercise 3: Thread-Safe Cache**
Implement a thread-safe cache using Arc<Mutex<HashMap<K, V>>>.

### **Exercise 4: Deadlock Demonstration**
Create and then fix a deadlock scenario with proper lock ordering.

---

## 📖 **Additional Resources**

### **Rust Documentation**
- [std::thread](https://doc.rust-lang.org/std/thread/) - Thread primitives
- [std::sync::mpsc](https://doc.rust-lang.org/std/sync/mpsc/) - Channel types
- [std::sync::Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html) - Mutual exclusion
- [std::sync::Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html) - Atomic reference counting

### **External Crates**
- **rayon** - Data parallelism library
- **crossbeam** - Advanced concurrency utilities
- **parking_lot** - Faster synchronization primitives

---

## 🚦 **Common Pitfalls**

### **Using Rc<T> in Threads** ❌
```rust
// DON'T: Rc is not Send
let counter = Rc::new(Mutex::new(0));
thread::spawn(move || {  // ERROR: Rc cannot be sent between threads
    let mut num = counter.lock().unwrap();
    *num += 1;
});
```

**Fix: Use Arc<T>** ✅
```rust
let counter = Arc::new(Mutex::new(0));
thread::spawn(move || {  // OK: Arc is Send
    let mut num = counter.lock().unwrap();
    *num += 1;
});
```

### **Forgetting to Join Threads** ⚠️
```rust
// Thread might not complete before main exits
thread::spawn(|| {
    println!("This might not print!");
});
// main exits immediately
```

### **Holding Locks Too Long** ⚠️
```rust
// BAD: Lock held during expensive operation
let data = mutex.lock().unwrap();
expensive_operation(&data);  // Lock held entire time
drop(data);

// GOOD: Minimize lock duration
let value = {
    let data = mutex.lock().unwrap();
    data.clone()  // Copy what we need
};  // Lock dropped here
expensive_operation(&value);
```

---

*Tags: #rust-book #ch16 #concurrency #threads #message-passing #shared-state #sync-send #fearless-concurrency*

*Links: [[../README]] | [[../../zettelkasten/rust-threading-basics]] | [[../../zettelkasten/message-passing-channels]] | [[../../MONTHLY_CALENDAR]]*
