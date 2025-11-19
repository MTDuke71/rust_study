# Sync & Send Traits - Thread Safety Contracts

*Auto traits that certify whether data can move between threads (`Send`) or be shared by reference across threads (`Sync`), forming the foundation of fearless concurrency audits.*

---

## 🎯 **Core Concept**

Every type in Rust implicitly advertises its thread-safety guarantees through the **`Send`** and **`Sync`** auto traits:
- `Send`: Ownership of values can be transferred across thread boundaries.
- `Sync`: Shared references (`&T`) can be accessed from multiple threads simultaneously.

If a type is `Send + Sync`, it can be placed inside `Arc<Mutex<T>>`, shared across worker pools, and safely mutated. If the compiler refuses to auto-implement either trait, it’s telling you a field depends on thread-local data (e.g., raw pointers, `Rc<T>`, `Cell<T>`). Understanding these signals prevents subtle undefined behavior and enables confident AoC optimizations.

## 🧠 **Mental Models**

- **Ownership Passport (Send)**: Moving a value to another thread stamps its passport; only `Send` travelers are allowed through customs.
- **Shared Viewing Window (Sync)**: Multiple observers can look through the same window only if the scene behind it is immutable or internally synchronized.
- **Hazmat Labels**: If a struct embeds `Rc<RefCell<T>>`, the compiler slaps a “do not ship” sticker on `Send`/`Sync`. Wrapping it in `Arc<Mutex<T>>` is like repackaging with proper shielding.

## 🔍 **Detailed Content**

### **Auto Trait Rules and Verification**

- Most primitives (`i32`, `String`, `Vec<T>`) are both `Send` and `Sync` as long as their contained types are.
- Types like `Rc<T>` and `RefCell<T>` are neither `Send` nor `Sync`, because they implement non-thread-safe reference counting or interior mutability.
- Marker types (e.g., `PhantomData<*const T>`) can forcefully opt-out; use sparingly.
- Verify expectations with compile-time assertions:

```rust
use static_assertions::{assert_impl_all, assert_not_impl_any};
use std::cell::RefCell;
use std::rc::Rc;

assert_impl_all!(Vec<u8>: Send, Sync);
assert_impl_all!(std::sync::Mutex<Vec<u8>>: Send, Sync);
assert_not_impl_any!(Rc<u8>: Send, Sync);
assert_not_impl_any!(RefCell<u8>: Sync);
```

### **AoC Thread-Safety Audit Workflow**

1. **Inventory the State**: List structs shared between threads (global memoization caches, dynamic programming tables, input registries).
2. **Check Auto Traits**: Use `cargo expand` or the `static_assertions` crate to confirm `Send + Sync` status.
3. **Wrap or Redesign**: Replace `Rc<T>` with `Arc<T>`, move `RefCell<T>` logic behind `Mutex<T>`/`RwLock<T>`, or split mutable pieces into channel messages.
4. **Document Guarantees**: In mission code and zettelkasten, note which fields break `Send/Sync` and how they were mitigated—future you (or future AI agents) will thank you.

### **Custom Types and Manual Implementations**

Manual `unsafe impl Send/Sync for MyType {}` should be rare and justified. Before considering it:
- Verify every field already satisfies the trait or is protected by synchronization primitives.
- Prove that invariants hold even if a thread panics while mutating the type.
- Prefer composition (e.g., `Arc<Mutex<_>>`) over unsafe impls unless you are building synchronization primitives yourself.

### **Comparing Strategies**

| Scenario | Preferred Pattern | Notes |
|----------|-------------------|-------|
| Shared read-only AoC lookup tables | `Arc<T>` (`T: Sync`) | No locking needed once data is built |
| Shared mutable scoreboard | `Arc<Mutex<T>>` (requires `T: Send`) | See [[shared-state-concurrency]] |
| Broadcast results | `crossbeam-channel` or `std::sync::mpsc` | Avoid shared mutable data entirely |
| Non-thread-safe crate type | Wrap in `Mutex`/`RwLock`, or confine to single-threaded executor | Never `unsafe impl` just to suppress compiler errors |

## 💡 **Key Takeaways**

- `Send` moves ownership, `Sync` shares references—learn to read compiler errors as design feedback.
- Auto trait derivation propagates through fields; a single `Rc<RefCell<T>>` makes the entire struct neither `Send` nor `Sync`.
- Use `static_assertions` or `std::marker::PhantomData` intentionally; never guess about thread safety.
- Wrapping data in `Arc<Mutex<T>>` usually restores `Send + Sync`, but measure the contention cost ([[shared-state-concurrency]]).
- AoC optimizations involving multi-threaded caches must document their thread-safety story to stay reliable.

## 🔗 **Integration Points**

### **Builds On**
- [[ownership]] - Foundation for understanding moves vs borrows
- [[rust-threading-basics]] - Thread spawning behavior rooted in Send/Sync
- [[shared-state-concurrency]] - Practical Arc<Mutex<T>> usage that depends on correct trait bounds

### **Enables**
- [[message-passing-channels]] - Choosing channels vs shared state based on type properties
- [[Send and Sync Deep Dive]] - Further exploration of marker trait internals
- [[rust_book/rust-book-ch16]] - Chapter 16.4 exercises on extensible concurrency

### **Related Concepts**
- [[interior-mutability]] - When `Cell/RefCell` purposely reject Sync
- [[deterministic-debugging]] - Logging Send/Sync violations during mission work
- [[Mission10_tut Overview]] - Union-Find REST API guarantees rely on `Send + Sync` states

---

*Tags: #concurrency #thread-safety #pattern #rust-book #intermediate*

*Links: [[zettel-index]] | [[rust-threading-basics]] | [[shared-state-concurrency]] | [[message-passing-channels]] | [[Send and Sync Deep Dive]] | [[deterministic-debugging]]*
