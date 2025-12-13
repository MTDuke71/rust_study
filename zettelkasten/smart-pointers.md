# Smart Pointers - Ownership-Aware Indirection in Rust

*Smart pointers are the “integration boundary” between ownership rules and pointer-like ergonomics (heap allocation, sharing, mutation, cleanup).*

---

## 🎯 Core Concept

In Rust, a “smart pointer” is a type that wraps some indirection and typically implements:

- `Deref` (so it behaves like a reference in many contexts)
- and/or `Drop` (so it can run deterministic cleanup)

Smart pointers let you model cases that plain references can’t:

- owning heap data (e.g., recursive structures)
- shared ownership (reference counting)
- interior mutability (runtime-checked borrowing)
- cycle breaking (weak references)
- thread-safe sharing/mutation (atomic RC + locks)

## 🧠 Mental Models

- **Pick the ownership story first, then the pointer**: single owner vs shared owners vs “owned by an arena”.
- **Indirection is a cost you buy on purpose**: to enable recursion, sharing, or mutation patterns that otherwise don’t type-check.
- **Keep algorithms independent of pointer choice**: keep data structure choices behind a small interface.

## 🔍 Detailed Content

### 1) The core axes (how to choose)

When you’re choosing between smart pointers, ask:

1) **Who owns the data?** (one owner vs many)
2) **Who mutates it?** (nobody vs one at a time)
3) **Where are the lifetimes controlled?** (scopes vs arenas)
4) **Single-threaded or multi-threaded?**

A quick rule-of-thumb:

- **Single owner, heap allocation** → `Box<T>`
- **Shared owners, single-threaded** → `Rc<T>` (and `Weak<T>` to break cycles)
- **Shared owners + mutation, single-threaded** → `Rc<RefCell<T>>`
- **Shared owners, multi-threaded** → `Arc<T>`
- **Shared owners + mutation, multi-threaded** → `Arc<Mutex<T>>` / `Arc<RwLock<T>>`
- **Many nodes with same lifetime, want performance** → arena allocation (indices)

### 2) How this shows up in this workspace

- `Box<T>` is the foundation for recursive structures and heap placement.
  - See [[box-in-aoc-problems]].
- `Deref` + deref-coercion is what makes wrapper types ergonomic.
  - See [[deref-trait]].
- `Drop` is the deterministic cleanup story that makes RAII work.
  - See [[drop-trait]].
- `Rc`/`RefCell`/`Weak` patterns matter for graphs/trees, but come with cycle risks.
  - See [[rc-shared-ownership]], [[refcell-interior-mutability]], [[reference-cycles]].
- For graph-like structures that would otherwise create reference cycles, arenas are often the “integrator move”.
  - See [[arena-allocation]].

### 3) Relationship to graph/pathfinding work

A lot of graph/pathfinding code benefits from *avoiding* pointer-heavy node graphs entirely:

- remap labels → dense indices
- store edges/weights in `Vec`-based structures
- run algorithms on indices

That tends to be simpler, faster, and easier to test.

## 💡 Key Takeaways

- Smart pointers are about **ownership + capabilities**, not just “pointers”.
- Start with the ownership model (single vs shared) and mutation model (compile-time vs runtime checks).
- Separate “algorithm core” from “storage choice” so you can swap representations later.
- For many graph problems, index-based storage (and arenas) beats pointer graphs.

## 🔗 Integration Points

### Builds On
- [[ownership-fundamentals]] - The rules smart pointers are designed around
- [[deref-trait]] - Pointer-like ergonomics
- [[drop-trait]] - Deterministic cleanup

### Enables
- [[Smart Pointers MOC]] - Full ecosystem map and deeper dives
- [[box-in-aoc-problems]] - Recursive structures and heap placement
- [[rc-shared-ownership]] - Multiple owners
- [[refcell-interior-mutability]] - Runtime borrow checking
- [[reference-cycles]] - Avoiding leaks in reference-counted graphs
- [[arena-allocation]] - Index-based ownership for performance + cycle avoidance
- [[rust_book/rust-book-ch15]] - Official grounding for these concepts

---

*Tags: #concept #smart-pointers #ownership #memory-management #intermediate*

*Links: [[zettel-index]] | [[Smart Pointers MOC]] | [[rust_book/rust-book-ch15]]*
