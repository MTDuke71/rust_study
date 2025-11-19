# 🚀 Rust Performance Tools Guide

**Source**: "The ultimate Rust performance guide" (Let's Get Rusty, June 2025)  
**Website**: https://www.youtube.com/watch?v=q3VOsGzkM-M
**Context**: Essential tooling for Mission 5+ performance requirements and V-Cycle optimization.

---

## 🛠️ Tool Inventory

### 1. Benchmarking & Baselines
**Tool**: `hyperfine`  
**Purpose**: Command-line benchmarking tool to establish performance baselines.  
**Use Case**: Comparing the runtime of different implementations or before/after optimization.  
**Command**: `hyperfine 'cargo run --release'`

### 2. CPU Profiling
**Tool**: `cargo-flamegraph`  
**Purpose**: Generates flame graphs to visualize where your application spends CPU time.  
**Use Case**: Identifying "hot paths" in Mission algorithms (e.g., finding O(n²) bottlenecks).  
**Command**: `cargo flamegraph`

### 3. Memory Profiling
**Tool**: `dhat` (Heap Allocation Analyzer)  
**Purpose**: Heap profiling to understand memory allocation patterns and detect wasteful allocations.  
**Use Case**: Optimizing memory usage in data-intensive missions (e.g., reducing clones in Day 09).  
**Integration**: Requires `dhat` crate integration in code.

### 4. Async Diagnostics
**Tool**: `tracing` (Crate)  
**Purpose**: Structured, event-based diagnostics framework.  
**Use Case**: Instrumenting async code (e.g., web servers, concurrent missions) to track request lifecycles.

### 5. Async Visualization
**Tools**: `tracing-chrome`, `tokio-console`  
**Purpose**: Visualizing async runtime behavior.  
**Use Case**: Debugging stuck tasks, analyzing concurrency, and real-time monitoring of Tokio tasks.

### 6. Load Testing
**Tool**: `ohio`  
**Purpose**: Fast Rust-based CLI for HTTP load testing.  
**Use Case**: Stress testing web services or API endpoints (relevant for advanced web missions).

---

## ⚡ Performance Best Practices (from Video)

### Build Configuration
- **Release Mode**: Always benchmark with `--release`.
- **LTO**: Enable Link Time Optimization (`lto = "fat"`) in `Cargo.toml`.
- **Codegen Units**: Set `codegen-units = 1` for better optimization (slower compile).
- **Panic Strategy**: `panic = "abort"` to remove unwinding overhead.
- **CPU Targeting**: `RUSTFLAGS="-C target-cpu=native"` for SIMD.

### Code Patterns
- **Allocations**: The #1 performance killer. Prefer Stack > Heap.
- **Hashing**: Use `FxHash` or `AHash` instead of default SipHash for non-crypto speed.
- **Iterators**: Often faster than C loops due to bounds check elimination.
- **Parallelism**: Use `rayon` for easy data parallelism (`.par_iter()`).

---

## 📦 Installation

```bash
# Benchmarking
cargo install hyperfine

# Profiling
cargo install flamegraph

# Async Console
cargo install tokio-console
```

---

## 🔗 Related Documentation

### **Performance Analysis & Benchmarking**
- [[performance-benchmarking-grid-optimization]] - Comprehensive benchmarking guide with Criterion.rs
- [[black-box-benchmarking]] - Preventing compiler optimizations in benchmarks
- [[dead-code-elimination]] - Understanding DCE in performance testing
- [[Algorithm Analysis]] - Complexity analysis and amortized analysis patterns

### **Mission Integration**
- [[missions/mission-5]] - HashMap performance requirements (V-Cycle)
- [[missions/mission-6]] - Grid optimization case studies
- [[missions/mission-8]] - Graph algorithm performance reports
- [[missions/mission-9]] - Pathfinding optimization and tuning

### **Workspace Resources**
- [[../missions/Mission8/PERFORMANCE_REPORT]] - Real-world benchmarking example
- [[../missions/Mission9/docs/PERFORMANCE_TUNING]] - Algorithm selection strategies

### **Navigation**
- [[zettel-index]] - Master index
- [[rust-concepts-MOC]] - Core language concepts

---

*Tags: #performance #rust #tools #profiling #benchmarking #optimization #criterion #flamegraph*
