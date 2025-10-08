# Rust Learning Roadmap - The Master Plan

*The original phase-by-phase strategy for mastering Rust through algorithms & data structures.*

---

## 🗺️ **The Vision**

Learn Rust not through isolated syntax lessons, but through **building real data structures and algorithms** with:
- Tight feedback loops
- Deep technical understanding
- Practical AoC preparation
- Professional engineering discipline

---

## 📅 **Phase 0: Setup + Habits** (≈1 day)

### **Install Toolchain**
```bash
# Core Rust toolchain
rustup install stable
rustup default stable

# Essential tools
cargo install cargo-criterion  # Benchmarking
cargo install cargo-watch      # Auto-recompile
cargo install cargo-audit       # Security checks
```

### **IDE Setup**
- **VS Code** with rust-analyzer extension
- **CodeLLDB** for debugging
- **Error Lens** for inline diagnostics

### **Establish Habits**

✅ **Every function has tests**
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() { /* ... */ }
}
```

✅ **Run clippy & fmt before every commit**
```bash
cargo clippy -- -D warnings
cargo fmt
```

✅ **Profile when performance matters**
```bash
cargo bench
```

**Status:** ✅ **COMPLETED** - Workspace established with professional tooling

---

## 📦 **Phase 1: Ownership Mechanics via Classic DS** (≈1-2 weeks)

**Goal:** Master Rust's ownership system through fundamental data structures.

### **Mission 1: Stacks** ✅ **COMPLETED**
**Focus:** Move vs borrow semantics, Vec<T>, Option<T>, pattern matching

**Key Concepts:**
- Ownership transfer in `push()` and `pop()`
- Immutable borrows with `peek()`
- Mutable borrows with `peek_mut()`
- Generic type parameter `<T>`

**Implementation:** `missions/Mission1/`
- ✅ 40+ tests with requirements traceability
- ✅ V-cycle methodology established
- ✅ Performance benchmarking

### **Mission 2: Queues** ✅ **COMPLETED**
**Focus:** Ring buffers, FIFO semantics, Option<T> for slot management

**Key Concepts:**
- Ring buffer with `Vec<Option<T>>`
- Wrap-around logic with modulo arithmetic
- Amortized O(1) operations
- `Option::take()` pattern

**Implementation:** `missions/Mission2/`
- ✅ Comparison with VecDeque
- ✅ Property-based testing with randomized operations

### **String Handling** (Integrated across missions)
**Focus:** String vs &str, slicing, UTF-8 invariants

**Key Concepts:**
- Owned `String` vs borrowed `&str`
- UTF-8 validation and indexing issues
- Slice lifetimes

**Status:** ✅ **COMPLETED** - Foundational ownership mastered

---

## 🎓 **Phase 2: Lifetimes, Traits, Generics** (≈1-2 weeks)

**Goal:** Advanced type system features for generic, reusable code.

### **Mission 3: Binary Search + Iterators** ✅ **COMPLETED**
**Focus:** Iterator trait, IntoIterator, zero-cost abstractions

**Key Concepts:**
- Custom iterator implementation (`RangeIter`)
- `Searchable` trait for multiple container types
- Lifetime parameters in trait methods
- Extension trait pattern

**Implementation:** `missions/Mission3/`
- ✅ Trait-based generic search
- ✅ Iterator composition patterns
- ✅ 40 tests with full V-cycle

**Zettelkasten:**
- [[Binary Search Iterator Patterns]]
- [[Trait Design Patterns - Mission3 Lessons]]
- [[AoC Binary Search Applications]]

### **Mission 5: Hash Maps** ✅ **COMPLETED**
**Focus:** BinaryHeap<T>, custom comparators with Ord

**Key Concepts:**
- Hash function design
- Collision resolution strategies
- Trait-based key definitions
- Generic container with `Ord` bounds

**Implementation:** `missions/Mission5/`
- ✅ Custom hash table implementation
- ✅ Comparison with std::collections::HashMap

### **Mission 7: Graphs** ✅ **COMPLETED**
**Focus:** Adjacency lists, node storage, algorithm foundations

**Key Concepts:**
- `Vec<Vec<usize>>` for adjacency representation
- DFS (recursive + iterative)
- BFS with shortest paths
- Connected components

**Implementation:** `missions/Mission7/`
- ✅ 36 tests (9 unit + 26 integration + 1 doc)
- ✅ Complete V-cycle with REQ-1 through REQ-6

### **Tries & Arenas** (PLANNED)
**Focus:** String interning, borrowing across arenas, lifetime parameters

**Status:** 🔄 **PARTIALLY COMPLETED** - Core concepts mastered

---

## ⚙️ **Phase 3: Algorithms with Ownership Constraints** (≈2-3 weeks)

**Goal:** Classic algorithms adapted to Rust's ownership model.

### **Union-Find (DSU)** (PLANNED)
**Focus:** Disjoint set union, path compression

**Concepts:**
- Index-based representation
- Avoiding borrowing conflicts with indices
- Rank optimization

### **Graph Algorithms** (IN PROGRESS)
**Focus:** Kruskal's MST, Dijkstra, A*

**Concepts:**
- Priority queue with `BinaryHeap`
- Edge list representations
- Visited set management

**Current:** Basic graph traversal (DFS/BFS) completed in Mission7

### **Dynamic Programming** (PLANNED)
**Focus:** Top-down memoization vs bottom-up

**Concepts:**
- `HashMap<K, V>` for memoization
- Lifetime issues with cached borrows
- Trade-offs between approaches

### **Range Query Structures** (PLANNED)
**Focus:** Segment trees, Fenwick trees

**Concepts:**
- Array-based tree representation
- Lazy propagation
- Iterator-based queries

**Status:** 🎯 **IN PLANNING** - Foundation laid with Mission7

---

## ⚡ **Phase 4: Performance + Unsafe** (opt-in) (≈1-2 weeks)

**Goal:** Optimization techniques and when to use `unsafe`.

### **Performance Optimization**
**Focus:** Cache-aware layouts, `#[inline]`, micro-optimizations

**Concepts:**
- `SmallVec` for stack allocation
- Branch prediction hints
- SIMD when appropriate

### **Unsafe Rust** (OPTIONAL)
**Focus:** `NonNull<T>`, `MaybeUninit<T>`, when and how

**Concepts:**
- Raw pointers
- Manual memory management
- FFI boundaries

**Status:** 📚 **FUTURE TOPIC** - Not critical for AoC

---

## 🎓 **Capstone Project** (1-2 weeks)

**Choose One:**

### **Option A: Incremental Search Engine**
- Trie for text indexing
- Posting lists
- BM25 scoring algorithm

### **Option B: Game AI Pathfinding Toolkit**
- A* implementation
- Pathfinding visualizations
- Grid-based movement

### **Option C: Mini Time-Series DB**
- LSM-tree-like structure
- Write-optimized storage
- Range queries

### **Option D: Static Site Generator**
- Parallel markdown processing
- Template engine
- Asset pipeline

### **Option E: Chess Engine**
- Bitboard representation
- Move generation
- Basic evaluation function

**Status:** 🎯 **DECISION PENDING** - Will choose after Phase 3

---

## 📊 **Current Progress**

### **✅ Completed Missions:**
1. ✅ Mission1 - Stack (Ownership fundamentals)
2. ✅ Mission2 - Queue (Ring buffers, Option<T>)
3. ✅ Mission3 - Binary Search (Traits, iterators, lifetimes)
4. ✅ Mission4 - Linked List (Box<T>, pointer manipulation)
5. ✅ Mission5 - Hash Maps (Hash functions, collision resolution)
6. ✅ Mission7 - Graphs (Adjacency lists, DFS/BFS)

### **📁 Supporting Projects:**
- ✅ Brackets_Basic - Stack application (bracket validation)
- ✅ Brackets_Ext - Extended validation with error reporting
- ✅ competitive_linked_tree - Tree diameter algorithms
- ✅ competitive_ring_bfs - BFS patterns for AoC
- ✅ aoc_pattern_recognition - Pattern library

### **📚 Zettelkasten Knowledge Pages:**
- Binary Search Iterator Patterns
- Trait Design Patterns
- AoC Binary Search Applications
- Ownership Mental Model - Library Analogy
- V-Cycle in Rust Development
- Graph Network Density

**Completion:** ~60% of original roadmap ✅

---

## 🎯 **Next Steps**

### **Immediate (Next 1-2 weeks):**
1. Complete Phase 2 remaining missions
2. Build more AoC-style validation problems
3. Expand zettelkasten with new patterns

### **Short-term (Next 1-2 months):**
1. Begin Phase 3 algorithm implementations
2. Dynamic programming patterns
3. Advanced graph algorithms

### **Long-term (3+ months):**
1. Complete capstone project
2. Contribute to open-source Rust projects
3. AoC 2025 participation

---

## 💡 **Key Insights from the Journey**

### **What Worked:**
- ✅ **V-cycle methodology** - Requirements-driven development
- ✅ **Incremental missions** - Small, focused learning units
- ✅ **Zettelkasten system** - Knowledge preservation and cross-referencing
- ✅ **AoC focus** - Practical, competitive programming orientation
- ✅ **Comprehensive testing** - 40+ tests per mission

### **Adaptations Made:**
- Missions reorganized based on dependency relationships
- Added zettelkasten for knowledge management
- Expanded testing beyond original plan
- Integrated daily study notes

### **Unique Approach:**
Instead of "learn Rust then build projects," this roadmap **learns Rust BY building projects** with professional engineering discipline.

---

## 🔗 **Related Resources**

- [[Project Origin Story]] - How this roadmap was created
- [[V-Cycle in Rust Development]] - The methodology
- [[Ownership Mental Model - The Library Analogy]] - Core concepts
- [[zettel-index]] - All extracted knowledge

---

## 📖 **For Future Learners**

If you're following this roadmap:

1. **Start with Mission1** - Don't skip ownership fundamentals
2. **Complete each V-cycle** - Requirements → Tests → Implementation
3. **Build zettelkasten** - Extract insights as you learn
4. **Apply to AoC problems** - Validate with real challenges
5. **Iterate based on confusion** - Deepen understanding before advancing

The roadmap is a **guide, not a prison**. Adapt based on what confuses you.

---

*Tags: #roadmap #learning-plan #curriculum #rust #algorithms #data-structures #aoc*

*Links: [[zettel-index]] | [[Project Origin Story]] | [[V-Cycle in Rust Development]]*
