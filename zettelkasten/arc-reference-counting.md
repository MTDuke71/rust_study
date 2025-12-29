# Arc<T> - Atomic Reference Counting (Thread-Safe Shared Ownership)

*Part of [[rust-concurrency-moc]] - [[shared-state-concurrency]]*

---

## 🎯 **Core Concept**

**Arc<T>** (Atomic Reference Counted) is the thread-safe version of `Rc<T>`. It uses **atomic operations** internally to manage reference counting, allowing safe shared ownership of data across multiple threads.

**Key Differences from Rc<T>**:
- ✅ **Thread-safe**: Can be sent across thread boundaries (`Send` + `Sync`)
- ⚡ **Atomic overhead**: Reference count updates use atomic CPU instructions (slower than `Rc`)
- 🔒 **Still immutable**: Like `Rc`, provides only immutable access (combine with `Mutex`/`RwLock` for mutation)

**When to Use**: Need to share read-only data across multiple threads without copying.

---

## 📋 **Quick Reference**

### **Basic Usage**

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3, 4, 5]);

let handles: Vec<_> = (0..3)
    .map(|i| {
        let data_clone = Arc::clone(&data);  // Increment atomic counter
        thread::spawn(move || {
            println!("Thread {}: {:?}", i, data_clone);
        })
    })
    .collect();

for handle in handles {
    handle.join().unwrap();
}
// data deallocated when last Arc goes out of scope
```

### **Core API**

```rust
use std::sync::Arc;

// Creation
let arc1 = Arc::new(42);
let arc2 = Arc::clone(&arc1);  // Atomic increment (NOT deep copy)

// Reference counting
let count = Arc::strong_count(&arc1);     // Current reference count
let weak_count = Arc::weak_count(&arc1);  // Weak reference count

// Weak references (break cycles, same as Rc)
let weak = Arc::downgrade(&arc1);
if let Some(strong) = weak.upgrade() {
    // Use strong reference
}

// Get owned value (requires unique ownership)
if let Ok(value) = Arc::try_unwrap(arc1) {
    // Now own value directly (no more references)
}
```

---

## 🔍 **Arc vs Rc Comparison**

| Feature | `Rc<T>` | `Arc<T>` |
|---------|---------|----------|
| **Thread Safety** | ❌ Single-threaded only | ✅ Thread-safe |
| **Performance** | Faster (non-atomic) | Slower (atomic overhead) |
| **Use Case** | Shared ownership in one thread | Shared ownership across threads |
| **Traits** | `!Send`, `!Sync` | `Send + Sync` (if `T: Send + Sync`) |
| **Reference Count** | Simple integer increment | Atomic fetch_add/fetch_sub |
| **Overhead** | Lower | Higher (memory barriers) |

**Rule of Thumb**: Use `Rc` unless you need thread safety, then use `Arc`.

---

## 🛠️ **Common Patterns**

### **Pattern 1: Arc<Mutex<T>> - Shared Mutable State**

The most common concurrent pattern: shared ownership + mutation.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let handles: Vec<_> = (0..10)
    .map(|_| {
        let counter_clone = Arc::clone(&counter);
        thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        })
    })
    .collect();

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap()); // 10
```

**Why Both?**
- **`Arc`**: Multiple threads need ownership of the data
- **`Mutex`**: Only one thread can mutate at a time

### **Pattern 2: Arc<RwLock<T>> - Read-Heavy Workloads**

When many readers, few writers:

```rust
use std::sync::{Arc, RwLock};
use std::thread;

let config = Arc::new(RwLock::new(vec![1, 2, 3]));

// Multiple readers
let readers: Vec<_> = (0..5)
    .map(|i| {
        let config_clone = Arc::clone(&config);
        thread::spawn(move || {
            let data = config_clone.read().unwrap();
            println!("Reader {}: {:?}", i, *data);
        })
    })
    .collect();

// Single writer
let config_clone = Arc::clone(&config);
let writer = thread::spawn(move || {
    let mut data = config_clone.write().unwrap();
    data.push(4);
});

for handle in readers {
    handle.join().unwrap();
}
writer.join().unwrap();
```

**Advantage**: Multiple concurrent readers, exclusive writer access.

### **Pattern 3: Arc for Read-Only Shared Data**

When data is immutable, `Arc` alone is sufficient:

```rust
use std::sync::Arc;
use std::thread;

// Large read-only configuration
let large_config = Arc::new(vec![0; 1_000_000]);

let handles: Vec<_> = (0..10)
    .map(|i| {
        let config = Arc::clone(&large_config);
        thread::spawn(move || {
            // All threads read same data without copying
            let sum: i32 = config.iter().sum();
            println!("Thread {}: sum = {}", i, sum);
        })
    })
    .collect();

for handle in handles {
    handle.join().unwrap();
}
```

**Use Case**: Sharing large immutable datasets (lookup tables, embeddings, game assets).

---

## ⚡ **Performance Considerations**

### **Atomic Overhead**

```rust
use std::rc::Rc;
use std::sync::Arc;

// Rc: Simple integer increment (fast)
let rc = Rc::new(42);
let rc2 = Rc::clone(&rc);  // ~1-2 CPU cycles

// Arc: Atomic fetch_add with memory barriers (slower)
let arc = Arc::new(42);
let arc2 = Arc::clone(&arc);  // ~10-50 CPU cycles (platform-dependent)
```

**Guideline**: ~5-10x slower for clone/drop operations compared to `Rc`.

### **When to Optimize**

- **Hot path cloning**: If cloning `Arc` millions of times per second, consider alternatives
- **Pre-clone once**: Clone `Arc` before tight loops rather than inside
- **Weak references**: Use `Weak<T>` for non-owning references to reduce strong count overhead

---

## 🧠 **Common Use Cases in Practice**

### **1. Thread Pool Worker Sharing**

```rust
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

let (tx, rx) = mpsc::channel::<Job>();
let receiver = Arc::new(Mutex::new(rx));  // Shared across workers

for i in 0..4 {
    let receiver = Arc::clone(&receiver);
    thread::spawn(move || {
        loop {
            let job = receiver.lock().unwrap().recv();
            match job {
                Ok(job) => job(),
                Err(_) => break,  // Channel closed
            }
        }
    });
}
```

**Pattern**: `Arc<Mutex<Receiver<T>>>` allows multiple workers to share one receiver.

### **2. AoC Concurrent Scoreboard**

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::HashMap;

let scoreboard = Arc::new(Mutex::new(HashMap::new()));

let handles: Vec<_> = (0..10)
    .map(|player_id| {
        let scoreboard = Arc::clone(&scoreboard);
        thread::spawn(move || {
            // Simulate game scoring
            let score = player_id * 100;
            scoreboard.lock().unwrap().insert(player_id, score);
        })
    })
    .collect();

for handle in handles {
    handle.join().unwrap();
}

println!("Final scores: {:?}", scoreboard.lock().unwrap());
```

### **3. Shared Configuration Across Services**

```rust
use std::sync::Arc;

struct AppConfig {
    api_key: String,
    timeout_ms: u64,
    max_connections: usize,
}

let config = Arc::new(AppConfig {
    api_key: "secret".to_string(),
    timeout_ms: 5000,
    max_connections: 100,
});

// Pass to multiple components without cloning data
let http_service = HttpService::new(Arc::clone(&config));
let cache_service = CacheService::new(Arc::clone(&config));
let db_service = DbService::new(Arc::clone(&config));
```

**Benefit**: Single source of truth, no data duplication.

---

## 🔗 **Relationship to Other Concepts**

### **Arc Internally Uses Atomics**

```rust
// Simplified Arc implementation concept
struct Arc<T> {
    ptr: *const ArcInner<T>,
}

struct ArcInner<T> {
    strong_count: AtomicUsize,  // ← Atomic operations!
    weak_count: AtomicUsize,
    data: T,
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        // Atomic increment
        self.inner().strong_count.fetch_add(1, Ordering::Relaxed);
        Arc { ptr: self.ptr }
    }
}
```

**Takeaway**: `Arc` is built on top of [[atomic-operations]] - specifically `AtomicUsize` for reference counting.

### **Arc with Interior Mutability**

| Pattern | Single-Threaded | Multi-Threaded |
|---------|-----------------|----------------|
| **Shared immutable** | `Rc<T>` | `Arc<T>` |
| **Shared mutable** | `Rc<RefCell<T>>` | `Arc<Mutex<T>>` or `Arc<RwLock<T>>` |

---

## ⚠️ **Common Pitfalls**

### **1. Deadlocks with Arc<Mutex<T>>**

```rust
// ❌ DEADLOCK!
let data = Arc::new(Mutex::new(42));

let lock1 = data.lock().unwrap();
// Still holding lock1...
let lock2 = data.lock().unwrap();  // Blocks forever!
```

**Solution**: Drop locks before reacquiring, use lock ordering.

### **2. Reference Cycles (Memory Leaks)**

```rust
use std::sync::{Arc, Mutex};

struct Node {
    next: Option<Arc<Mutex<Node>>>,
}

// Cycle: node1 → node2 → node1
// Both Arc strong counts stay at 2, never deallocated!
```

**Solution**: Use `Weak<T>` for one direction:

```rust
struct Node {
    next: Option<Arc<Mutex<Node>>>,
    prev: Option<Weak<Mutex<Node>>>,  // ← Weak reference
}
```

### **3. Unnecessary Arc Cloning in Hot Loops**

```rust
// ❌ BAD: Clone Arc inside loop
for i in 0..1_000_000 {
    let data = Arc::clone(&shared_data);  // Atomic operation every iteration!
    process(data);
}

// ✅ BETTER: Clone once outside loop (if possible)
let data = Arc::clone(&shared_data);
for i in 0..1_000_000 {
    process(&data);  // Just borrow
}
```

---

## 📚 **Learning Path**

1. Start with [[rc-shared-ownership]] - Understand reference counting concept
2. Learn [[atomic-operations]] - Foundation of Arc's thread safety
3. Study [[shared-state-concurrency]] - Arc + Mutex patterns
4. Practice with [[thread-pool-pattern]] - Real-world Arc usage

---

## Related Concepts

- [[rc-shared-ownership]] - Single-threaded version of Arc
- [[atomic-operations]] - What Arc uses internally for thread-safe counting
- [[shared-state-concurrency]] - Arc<Mutex<T>> patterns
- [[rust-concurrency-moc]] - Overall concurrency navigation
- [[Smart Pointers MOC]] - Arc in the smart pointer ecosystem
- [[reference-cycles]] - Memory leak prevention with Weak<T>
- [[rust-book-ch16]] - Fearless concurrency chapter covering Arc

*Links:*
- Part of: [[rust-concurrency-moc]], [[Smart Pointers MOC]]
- Referenced in: [[atomic-operations]], [[thread-pool-pattern]], [[shared-state-concurrency]]
- Related: [[rc-shared-ownership]], [[atomic-operations-memory-ordering]]
