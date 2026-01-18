# Auto-Traits: Send and Sync - Compiler-Enforced Thread Safety

Auto-traits are special marker traits that Rust automatically implements for types based on their composition. The two most important are `Send` and `Sync`, which form the foundation of Rust's fearless concurrency.

---

## Core Concept

**Auto-traits** are traits that:
- Have no methods (pure marker traits)
- Are automatically implemented by the compiler based on structural rules
- Can be explicitly opted-out using negative trait bounds
- Are `unsafe` to manually implement (requires upholding invariants the compiler can't verify)

The two critical auto-traits for concurrency:
- **`Send`**: Type can be transferred across thread boundaries (ownership transfer)
- **`Sync`**: Type can be shared between threads via `&T` (immutable reference sharing)

## Mental Models

**Send = "Ownership Passport"**
- Can this value move to another thread?
- Think: "If I own it, can I hand it to another thread?"
- Most types are `Send` (integers, strings, owned collections)

**Sync = "Shared Viewing Window"**
- Can multiple threads safely hold `&T` references simultaneously?
- Think: "If I have a reference, can I share that reference across threads?"
- Immutable data is usually `Sync`

**The Relationship**: If `T: Sync`, then `&T: Send`
- Because sharing immutable references across threads is safe when the type is `Sync`

**PhantomData = "Type-Level Marker"**
- `PhantomData<T>` has zero runtime cost
- Inherits auto-trait properties of `T`
- Used to intentionally opt-out of `Send`/`Sync`

## Detailed Content

### Auto-Implementation Rules

The compiler automatically implements `Send` and `Sync` based on composition:

```rust
// Automatically Send + Sync
struct Point {
    x: i32,
    y: i32,
}

// Automatically Send + Sync if T: Send + Sync
struct Wrapper<T> {
    value: T,
}

// NOT Send/Sync - contains Rc (single-threaded reference counting)
use std::rc::Rc;
struct NotThreadSafe {
    shared: Rc<i32>,
}
```

### Types That Are NOT Send

- `Rc<T>` - Single-threaded reference counting (use `Arc<T>` instead)
- `*const T` / `*mut T` - Raw pointers (no safety guarantees)
- Types with thread-local state
- Types containing `PhantomData<*const ()>`

```rust
use std::marker::PhantomData;

// Intentionally NOT Send
struct NotSend {
    _marker: PhantomData<*const ()>,
}
```

### Types That Are NOT Sync

- `Cell<T>` - Interior mutability without synchronization
- `RefCell<T>` - Runtime-checked interior mutability
- `Rc<T>` - Single-threaded reference counting
- Unsynchronized mutable state

```rust
use std::cell::Cell;

// Cell is Send but NOT Sync
// (can move to another thread, but can't share references)
let cell = Cell::new(42);
```

### Making Types Thread-Safe

Replace single-threaded primitives with thread-safe equivalents:

| **Single-Threaded** | **Thread-Safe Alternative** |
|---------------------|------------------------------|
| `Rc<T>` | `Arc<T>` (atomic reference counting) |
| `Cell<T>` | `Mutex<T>` or `RwLock<T>` |
| `RefCell<T>` | `Mutex<T>` or `RwLock<T>` |

```rust
use std::sync::{Arc, Mutex};

// NOT thread-safe
use std::rc::Rc;
use std::cell::RefCell;
let bad = Rc::new(RefCell::new(vec![1, 2, 3]));

// Thread-safe version
let good = Arc::new(Mutex::new(vec![1, 2, 3]));
```

### Intentionally Opting Out

Use `PhantomData` to prevent auto-implementation:

```rust
use std::marker::PhantomData;

// Prevent Send (raw pointer semantics)
struct NotSend {
    _marker: PhantomData<*const ()>,
}

// Prevent Sync but allow Send
struct NotSync {
    _marker: PhantomData<std::cell::Cell<()>>,
}
```

**Why `*const ()`?**
- `*const ()` is a raw pointer (NOT Send, NOT Sync)
- `()` is the unit type (zero-sized)
- `PhantomData<*const ()>` has zero runtime cost but inherits the "not Send/Sync" properties

### Compile-Time Verification

Always test that your types maintain their thread-safety guarantees:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn ensure_thread_safe() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        
        assert_send::<MyType>();
        assert_sync::<MyType>();
    }
    
    // Verify intentional opt-out
    #[test]
    fn ensure_not_send() {
        fn assert_not_send<T: Send>() {}
        // This should NOT compile:
        // assert_not_send::<NotSendType>();
    }
}
```

### Common Pitfalls

**Accidentally Breaking Send/Sync**
```rust
// Before: Send + Sync
struct Cache {
    data: Vec<String>,
}

// After: NOT Send + NOT Sync (breaking change!)
use std::rc::Rc;
struct Cache {
    data: Vec<String>,
    shared: Rc<i32>,  // ❌ Breaks thread safety!
}
```

**Solution**: Always test auto-traits in your test suite.

**Unsafe Manual Implementation**
```rust
// ⚠️ DANGEROUS - only if you REALLY know what you're doing
unsafe impl Send for MyUnsafeType {}
unsafe impl Sync for MyUnsafeType {}
```

Only implement these manually if:
- You're using raw pointers or FFI
- You've ensured thread safety through external synchronization
- You understand the safety invariants

## Rust for Rustaceans Connection

From **Chapter 3: Designing Interfaces**, auto-traits are part of the "hidden contract":

- **Re-export hazard**: Exposing dependency types can break Send/Sync
- **SemVer violation**: Removing Send/Sync is a breaking change
- **Testing requirement**: Verify auto-traits in tests to catch regressions

```rust
// Good: Wrap foreign types to control auto-traits
pub struct Wrapper {
    inner: ForeignType,  // Hidden from public API
}

// Bad: Expose foreign type directly
pub use foreign_crate::Type;  // ❌ Changes to foreign type break you
```

## AoC Applications

### Thread-Safe Caching

```rust
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

type Cache = Arc<Mutex<HashMap<String, usize>>>;

fn solve_with_cache(cache: Cache) {
    let mut guard = cache.lock().unwrap();
    guard.insert("key".to_string(), 42);
}
```

### Parallel Processing

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3, 4, 5]);
let mut handles = vec![];

for i in 0..5 {
    let data_clone = Arc::clone(&data);
    handles.push(thread::spawn(move || {
        println!("Thread {} sees: {:?}", i, data_clone);
    }));
}

for handle in handles {
    handle.join().unwrap();
}
```

## When to Care About Auto-Traits

1. **Concurrent programming**: Using threads, async, or parallel iterators
2. **Library design**: Public types should maintain Send/Sync unless documented otherwise
3. **Foreign types**: Wrapping FFI or raw pointers
4. **Performance**: Shared state across worker threads

## Key Takeaways

- Auto-traits are automatically implemented based on field composition
- `Send` = ownership transfer, `Sync` = reference sharing
- Removing Send/Sync is a **breaking change** (test for it!)
- Use thread-safe primitives (`Arc`, `Mutex`) instead of single-threaded ones (`Rc`, `RefCell`)
- `PhantomData` can intentionally opt-out (zero runtime cost)
- Raw pointers (`*const T`) break Send/Sync by default

---

## Related Concepts

*Links:*
- [[Send and Sync Deep Dive]] - Comprehensive thread safety guide
- [[sync-send-traits]] - Thread safety contracts and AoC workflow
- [[rust-for-rustaceans]] - Chapter 3: Designing Interfaces
- [[semver-trick]] - Maintaining compatibility when dependencies change
- [[shared-state-concurrency]] - Mutex and RwLock patterns
- [[arc-reference-counting]] - Thread-safe reference counting
- [[interior-mutability]] - Cell, RefCell, and when to use Mutex instead
- [[phantom-data-type-safety]] - Zero-cost type-level markers

*Tags: #rust #concurrency #auto-traits #send #sync #thread-safety #marker-traits #rustaceans*
