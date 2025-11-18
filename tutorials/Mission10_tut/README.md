# Mission 10 Tutorial: Union-Find Disjoint Sets

A step-by-step tutorial for understanding and implementing the Union-Find data structure.

---

## 🎯 Tutorial Overview

This tutorial teaches you how to build an efficient Union-Find (Disjoint Set Union) data structure from scratch. You'll learn:

1. Basic Union-Find without optimizations
2. Path compression optimization
3. Union by rank optimization
4. Combined optimizations and complexity analysis
5. Real-world applications (cycle detection, MST, connected components)
6. Advanced variants and extensions
7. Problem-solving patterns
8. REST API with OpenAPI/Swagger

---

## 📚 Tutorial Steps

### Step 1: Basic Union-Find
**File**: `examples/step1_basic_union_find.rs`

Learn the fundamentals:
- What is Union-Find and why it's useful
- Basic find() operation (naive implementation)
- Basic union() operation
- Understanding the tree representation
- Time complexity without optimizations: O(n) worst case

**Run**: `cargo run --example step1_basic_union_find`

---

### Step 2: Path Compression
**File**: `examples/step2_path_compression.rs`

Optimize the find operation:
- Understanding path compression
- Implementing iterative path compression
- Flattening tree structure during traversal
- Analyzing the improvement
- Time complexity: O(log n) average case

**Run**: `cargo run --example step2_path_compression`

---

### Step 3: Union by Rank
**File**: `examples/step3_union_by_rank.rs`

Optimize the union operation:
- Understanding tree height/rank
- Implementing union by rank
- Keeping trees balanced
- Combining with path compression
- Time complexity: O(α(n)) amortized

**Run**: `cargo run --example step3_union_by_rank`

---

### Step 4: Combined Optimizations
**File**: `examples/step4_combined_optimizations.rs`

Put it all together:
- Both optimizations working together
- Inverse Ackermann function α(n)
- Performance analysis and benchmarking
- Best practices

**Run**: `cargo run --example step4_combined_optimizations`

---

### Step 5: Real-World Applications ✅
**File**: `examples/step5_applications.rs`  
**Exercises**: `exercises/step5_exercises.rs`  
**Status**: Complete

Learn practical applications:
- **Kruskal's Minimum Spanning Tree** - Graph algorithm with cycle detection
- **Connected Components** - Finding all components in undirected graphs
- **Cycle Detection** - Detecting cycles during edge addition
- **Social Network Friend Circles** - Transitive relationship modeling
- **Image Segmentation** - Grouping pixels by color similarity
- **Performance Comparison** - Union-Find vs DFS/BFS vs adjacency matrix
- **7 Practice Exercises** - LeetCode problems and advanced challenges

**Run**: `cargo run --example step5_applications`  
**Practice**: `cargo run --bin step5_exercises`  
**Tests**: `cargo test --example step5_applications`

---

### Step 6: Advanced Variants
**File**: `examples/step6_advanced_variants.rs`

Extensions and variations:
- Weighted Union-Find
- Union-Find with deletion
- Persistent Union-Find
- Partial persistence
- Size vs Rank strategies

**Run**: `cargo run --example step6_advanced_variants`

---

### Step 7: Problem Solving Patterns
**File**: `examples/step7_problem_solving.rs`

Competitive programming patterns:
- When to use Union-Find
- Common problem patterns
- Edge cases and gotchas
- Optimization tricks
- Interview questions

**Run**: `cargo run --example step7_problem_solving`

---

### Step 8: REST API with OpenAPI/Swagger ✅
**File**: `examples/step8_rest_api/`  
**Status**: ✅ COMPLETE

Build a production-ready REST API:
- **RESTful Design** - Clean, resource-based API endpoints
- **Axum & Tokio** - Modern async web server implementation
- **OpenAPI/Swagger** - Automatic interactive documentation
- **6 Complete Endpoints** - Full CRUD operations for Union-Find
- **Production Features** - Input validation, error handling, state management
- **Comprehensive Tutorial** - See `examples/step8_rest_api/TUTORIAL.md`

**Run**: `cd examples/step8_rest_api && cargo run`  
**Docs**: Open http://localhost:8080/swagger-ui  
**Tutorial**: See `examples/step8_rest_api/TUTORIAL.md` for detailed learning guide
 
 ---

## 🚀 Getting Started

```bash
# Clone and navigate to tutorial
cd tutorials/Mission10_tut

# Run step 1
cargo run --example step1_basic_union_find

# Run tests (when implemented)
cargo test

# Run all examples in sequence
for i in {1..7}; do cargo run --example step${i}_*; done
```

---

## 📖 Learning Path

```
Step 1: Basic → Step 2: Path Compression → Step 3: Union by Rank
                              ↓
                   Step 4: Combined (Full Implementation)
                              ↓
         Step 5: Applications ← → Step 6: Variants
                              ↓
                               ↓
                    Step 7: Problem Solving
                               ↓
                    Step 8: REST API
```

---

## 🎓 Prerequisites

Before starting this tutorial, you should understand:
- Basic Rust syntax and ownership
- Vectors and indexing
- Tree data structures (conceptually)
- Basic graph terminology

---

## 🎯 Learning Objectives

By completing this tutorial, you will:
- [ ] Understand how Union-Find works internally
- [ ] Implement path compression optimization
- [ ] Implement union by rank optimization
- [ ] Analyze time complexity with inverse Ackermann function
- [ ] Apply Union-Find to solve real problems
- [ ] Recognize when to use Union-Find
- [ ] Handle edge cases and optimize implementations

---

## 📊 Complexity Summary

| Implementation | Find | Union | Space |
|---------------|------|-------|-------|
| Naive | O(n) | O(n) | O(n) |
| With Path Compression | O(log n) avg | O(log n) avg | O(n) |
| With Union by Rank | O(log n) | O(log n) | O(n) |
| Both Optimizations | O(α(n))* | O(α(n))* | O(n) |

*α(n) is the inverse Ackermann function, effectively constant for all practical n

---

## 🔗 Additional Resources

- **Mission 10**: `../../missions/Mission10/` - Full implementation
- **Week 6 Daily Study**: Module organization and Cargo features
- **Rust Book Ch 13**: Closures (for understanding Fn traits)
- **Competitive Programming**: Union-Find problem sets

---

## 📝 Tutorial Status

- [x] Step 1: Basic Union-Find (To create)
- [ ] Step 2: Path Compression (To create)
- [ ] Step 3: Union by Rank (To create)
- [ ] Step 4: Combined Optimizations (To create)
- [ ] Step 5: Applications (To create)
- [ ] Step 6: Advanced Variants (To create)
- [ ] Step 7: Problem Solving (To create)
- [ ] Step 8: REST API (To create)

---

*Tutorial Start: November 2, 2025*
*Aligned with: Mission 10 (Nov 2-8)*

*Navigation: [[../README.md|Tutorials Overview]] | [[../../missions/Mission10/README.md|Mission 10]]*
