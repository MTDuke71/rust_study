# Future Trait Deep Dive

*Navigation: [[zettel-index]] | [[rust-concurrency-moc]] | [[async-await-basics]]*

---

## Overview

The **Future trait** is the foundation of Rust's async/await system. Understanding its mechanics—along with **Pin**, **Unpin**, and the **Stream** trait—reveals how Rust achieves zero-cost async abstractions while maintaining memory safety.

**Source**: Rust Book Chapter 17.5 - "A Closer Look at the Traits for Async"  
**Implementation**: `rust_book/Ch17/async_traits/src/ch17_05_traits.rs`

---

## The Future Trait

Every `async fn` and `async {}` block produces a type that implements the `Future` trait:

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

### Key Components

| Component | Purpose |
|-----------|---------|
| `Output` | Associated type defining what the future resolves to |
| `poll()` | Checks if the future is complete, advances state if possible |
| `Pin<&mut Self>` | Guarantees the future won't move in memory (critical for self-referential types) |
| `Context` | Provides the `Waker` for the runtime to know when to re-poll |

### The Poll Enum

```rust
enum Poll<T> {
    Ready(T),   // Future complete, here's the value
    Pending,    // Not ready yet, poll again when woken
}
```

**Execution Model**:
- Runtime calls `poll()` on the future
- `Pending` → Runtime parks the task, wakes it later via `Waker`
- `Ready(value)` → Future is complete, return value to caller

---

## How await Compiles to poll()

When you write:

```rust
let title = page_title(url).await;
```

Rust compiles it conceptually to:

```rust
// Pseudocode - DO NOT write this yourself!
let mut page_title_fut = page_title(url);
loop {
    match page_title_fut.poll(cx) {
        Poll::Ready(value) => {
            // Use value
            break;
        }
        Poll::Pending => {
            // Runtime yields, will wake when ready
            // Task is parked, not busy-waiting
            return Poll::Pending;
        }
    }
}
```

**Key Insight**: The runtime handles the loop and waking—you never write polling loops manually.

---

## Pin and Unpin

### The Problem: Self-Referential Futures

Async blocks can create **self-referential data structures**:

```rust
async {
    let data = vec![1, 2, 3];
    let reference = &data[0];  // Points to data above
    some_async_operation().await;
    println!("{}", reference); // Still needs valid reference
}
```

This compiles to a struct like:

```
┌─────────────────────┐
│ fut                 │
├─────────────────────┤
│ data: Vec<i32>      │
│ reference ───────┐  │ ← Points to data above
│                  ↓  │
│ (target location)   │
└─────────────────────┘
```

**If the future moves in memory**, the internal reference becomes **dangling**!

### The Solution: Pin<T>

`Pin<P>` is a wrapper that prevents moving the pointed-to value:

```rust
// Future::poll requires Pin<&mut Self>
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
```

**Guarantees**: Once pinned, the value cannot move to a different memory location.

### Pin<Box<T>> Mechanics

```
Pin<Box<T>>:
┌─────┐     ┌────┐     ┌────────┐
│ Pin │ --> │ Box│ --> │ pinned │
└─────┘     └────┘     │  data  │
                       └────────┘
```

The `Box` pointer itself can move (be reassigned), but **the data it points to cannot**:

```
After moving the Box:
┌─────┐     ┌────┐     ┌────────┐
│ Pin │ --> │Box'│ --> │ pinned │
└─────┘     └────┘     │  data  │
  (original Box gone)  │ (SAME) │
                       └────────┘
```

### The Unpin Trait

`Unpin` is a **marker trait** (no methods):

```rust
// Most types implement Unpin automatically
impl Unpin for i32 {}
impl Unpin for String {}
impl<T> Unpin for Vec<T> {}

// Async blocks are !Unpin (do NOT implement Unpin)
// They may contain self-references
```

**Key Rules**:

| Type | Unpin? | Can be moved when pinned? |
|------|--------|---------------------------|
| `i32`, `String`, `Vec<T>` | Yes | ✅ Safe to move |
| `async {}` blocks | No (`!Unpin`) | ❌ Must stay pinned |
| Custom self-referential | No | ❌ Must stay pinned |

### Practical Implications

```rust
// Unpin types: Pin has no effect on movement
let mut s = String::from("hello");
let pinned: Pin<&mut String> = Pin::new(&mut s);
// String is Unpin, so this is safe - data can still move

// !Unpin types: Must use Box::pin or pin! macro
let fut = async { /* ... */ };
let pinned: Pin<Box<dyn Future<Output = ()>>> = Box::pin(fut);
// Future is !Unpin, data cannot move after pinning
```

---

## The Stream Trait

`Stream` is the **async version of Iterator**:

```rust
trait Stream {
    type Item;
    
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}
```

### Comparison

| Trait | Method | Return Type |
|-------|--------|-------------|
| Iterator | `next()` | `Option<Item>` |
| Future | `poll()` | `Poll<Output>` |
| **Stream** | `poll_next()` | `Poll<Option<Item>>` |

### Return Value Meanings

```rust
Poll::Pending           // Not ready, try later
Poll::Ready(Some(item)) // Here's the next item
Poll::Ready(None)       // Stream exhausted
```

### StreamExt Convenience

The `StreamExt` trait provides `async fn next()`:

```rust
use tokio_stream::StreamExt;

while let Some(item) = stream.next().await {
    process(item);
}
```

---

## Custom Future Implementation

From `ch17_05_traits.rs`:

```rust
pub struct CountdownFuture {
    count: u32,
}

impl Future for CountdownFuture {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count == 0 {
            println!("  CountdownFuture: Ready! Liftoff!");
            Poll::Ready("Launched!")
        } else {
            println!("  CountdownFuture: {} remaining...", self.count);
            self.count -= 1;
            Poll::Pending  // Would normally register waker
        }
    }
}
```

**Note**: Real futures should use `cx.waker()` to schedule wake-ups. This example returns `Pending` repeatedly for demonstration.

---

## Key Takeaways

1. **Future::poll** returns `Poll::Ready(value)` or `Poll::Pending`
2. **await** compiles to poll loops managed by the async runtime
3. **Pin** prevents self-referential futures from moving in memory
4. Most types are **Unpin** (safe to move even when "pinned")
5. **Stream** = async Iterator (`poll_next()` → `Poll<Option<Item>>`)
6. **StreamExt** provides `async fn next()` for convenient async iteration

---

## Related Concepts

### Async Foundations
- [[async-await-basics]] - Futures, `.await`, execution model
- [[async-concurrency]] - `join!()`, `select!()`, concurrent composition
- [[async-streams]] - Async iteration patterns

### Runtime & Execution
- [[rust-threading-basics]] - Thread-based concurrency comparison
- [[async-vs-threads-decision]] - When to choose async vs threads
- [[sync-send-traits]] - Thread safety markers

### Memory & Safety
- [[ownership-fundamentals]] - Ownership model that enables Pin safety
- [[Box Smart Pointer]] - Heap allocation and `Box::pin()`
- [[interior-mutability]] - RefCell patterns in async contexts

### Advanced Topics
- [[async-performance-timer-resolution]] - Windows timer resolution analysis
- [[message-passing-channels]] - Channel-based async communication

---

## Code Examples Location

- **Implementation**: `rust_book/Ch17/async_traits/src/ch17_05_traits.rs`
- **Examples**: `rust_book/Ch17/async_traits/examples/`
  - `future_trait.rs` - Custom Future implementations
  - `pin_unpin.rs` - Pin fundamentals and demonstrations
  - `tasks_vs_threads.rs` - Async vs threading comparison

```bash
# Run the demonstration
cd rust_book/Ch17/async_traits
cargo run                           # Full trait demonstration
cargo test                          # Unit tests for custom futures
```

---

## References

- **Rust Book Ch17.5**: https://doc.rust-lang.org/stable/book/ch17-05-traits-for-async.html
- **std::future::Future**: https://doc.rust-lang.org/std/future/trait.Future.html
- **std::pin::Pin**: https://doc.rust-lang.org/std/pin/struct.Pin.html
- **std::marker::Unpin**: https://doc.rust-lang.org/std/marker/trait.Unpin.html

---

*Tags: #async #futures #pin #unpin #stream #concurrency #rust-book #ch17*

*Links:*
- Builds on: [[async-await-basics]]
- Related to: [[async-concurrency]], [[async-streams]], [[sync-send-traits]]
- Applied in: [[rust_book/Ch17/async_traits]]
- Part of: [[rust-concurrency-moc]], [[rust-book]]
