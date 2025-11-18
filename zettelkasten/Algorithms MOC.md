# Algorithms MOC (Map of Content)

**Related:** [[AoC Patterns MOC]], [[rust-concepts-MOC]], [[Collections MOC]], [[zettel-index]]

## Overview

This Map of Content organizes all algorithm-related knowledge in the Rust Study workspace. It serves as a central hub for algorithmic patterns, analysis techniques, and problem-solving strategies essential for competitive programming and software engineering.

## 🎯 Core Algorithm Categories

### **Search & Traversal Algorithms**

**Graph Traversal:**
- [[BFS Patterns]] - Breadth-first search for shortest paths and level-order traversal
- [[DFS Patterns]] - Depth-first search for exhaustive exploration and backtracking
- [[Graph Algorithms]] - General graph algorithm patterns and applications
- [[A-Star-Algorithm-Deep-Dive]] - Heuristic-based optimal pathfinding
- [[Dijkstra Algorithm]] - Single-source shortest paths with priority queue

**Tree Algorithms:**
- [[Binary Search Tree Patterns]] - BST operations and balancing
- [[Tree Traversal]] - In-order, pre-order, post-order patterns
- [[Lowest Common Ancestor]] - LCA algorithms and applications

**Array & String Search:**
- [[Binary Search]] - Efficient searching in sorted arrays
- [[Binary Search Iterator Patterns]] - Iterator-based binary search in Rust
- [[Pattern Matching]] - String search algorithms (KMP, Boyer-Moore)

### **Dynamic Programming**

- [[Dynamic Programming]] - Optimization through memoization and tabulation
- [[Memoization Patterns]] - Top-down caching strategies with HashMap and Vec
- [[Recursion]] - Recursive problem solving and base cases
- [[State Machine Patterns]] - DP with state transitions

### **Greedy Algorithms**

- [[Greedy Algorithms]] - Locally optimal choices for global optimization
- [[Activity Selection]] - Interval scheduling problems
- [[Huffman Coding]] - Optimal prefix codes

### **Divide and Conquer**

- [[Divide and Conquer]] - Breaking problems into subproblems
- [[Merge Sort]] - O(n log n) divide-and-conquer sorting
- [[Quick Sort]] - Average O(n log n) partition-based sorting
- [[Binary Search]] - Dividing search space in half

### **Sorting & Ordering**

**Comparison Sorts:**
- [[Merge Sort]] - Stable O(n log n) divide-and-conquer
- [[Quick Sort]] - In-place O(n log n) average case
- [[Heap Sort]] - O(n log n) using binary heap

**Linear Time Sorts:**
- [[Counting Sort]] - O(n + k) for small range integers
- [[Radix Sort]] - O(d * n) for fixed-length keys
- [[Bucket Sort]] - O(n) average for uniform distribution

### **Graph Algorithms**

**Shortest Paths:**
- [[Dijkstra Algorithm]] - Single-source shortest paths
- [[Bellman-Ford]] - Single-source with negative weights
- [[Floyd-Warshall]] - All-pairs shortest paths
- [[A-Star-Algorithm-Deep-Dive]] - Heuristic search

**Graph Structures:**
- [[Union-Find]] - Disjoint set with path compression
- [[Minimum Spanning Tree]] - Kruskal's and Prim's algorithms
- [[Topological Sort]] - DAG ordering
- [[Strongly Connected Components]] - Tarjan's and Kosaraju's algorithms

**Graph Properties:**
- [[Graph Network Density]] - Network connectivity metrics
- [[Cycle Detection]] - Finding cycles in graphs
- [[Bipartite Graph Detection]] - Two-coloring algorithms

### **Computational Geometry**

**Distance Metrics:**
- [[Manhattan Distance]] - L1 norm, 4-connected grid distance
- [[Euclidean Distance]] - L2 norm, continuous space distance
- [[Chebyshev Distance]] - L∞ norm, 8-connected chessboard distance

**Spatial Algorithms:**
- [[Convex Hull]] - Graham scan, Jarvis march
- [[Line Intersection]] - Computational geometry basics
- [[Closest Pair Problem]] - Divide and conquer approach

### **String Algorithms**

- [[Pattern Matching]] - String search algorithms
- [[Longest Common Subsequence]] - DP string comparison
- [[Edit Distance]] - Levenshtein distance with DP
- [[Trie Data Structure]] - Prefix tree for string operations
- [[Suffix Array]] - String sorting and pattern matching

### **Mathematical Algorithms**

- [[Number Theory]] - Prime numbers, GCD, modular arithmetic
- [[Fast Exponentiation]] - Exponentiation by squaring
- [[Sieve of Eratosthenes]] - Prime number generation
- [[Extended Euclidean Algorithm]] - GCD with Bézout coefficients

## 🔬 Algorithm Analysis

### **Complexity Analysis**

- [[Algorithm Analysis]] - Comprehensive complexity and performance analysis
- [[Big-O Notation]] - Asymptotic complexity notation
- [[Space-Time Tradeoffs]] - Memory vs computation optimization
- [[Amortized Analysis]] - Average cost over sequences (covered in [[Algorithm Analysis]])

### **Performance Optimization**

- [[Performance Optimization Guide]] - Systematic optimization strategies
- [[Cache Efficiency]] - Cache-aware algorithm design
- [[SIMD Optimization]] - Parallel data processing
- [[zero-cost-abstractions]] - Rust's performance guarantees

### **Benchmarking & Profiling**

- [[performance-benchmarking-grid-optimization]] - Comprehensive benchmarking guide
- [[Criterion Benchmarking]] - Statistical performance measurement
- [[Profiling Tools]] - Finding performance bottlenecks

## 🎮 Problem-Solving Patterns

### **Advent of Code Patterns**

- [[AoC Patterns MOC]] - Competitive programming patterns
- [[AoC Binary Search Applications]] - Binary search in competition
- [[AoC Collection Problems]] - Collection manipulation patterns
- [[Grid Pattern Recognition]] - 2D grid problem strategies

### **Common Techniques**

**Two Pointers:**
- Fast-slow pointer (cycle detection)
- Left-right pointer (sorted array problems)
- Sliding window (substring problems)

**Prefix Sum:**
- Range query optimization
- Subarray sum problems
- 2D prefix sum for grids

**Backtracking:**
- Constraint satisfaction
- Combinatorial generation
- Puzzle solving

## 🏗️ Data Structure Algorithms

### **Mission Implementations**

- [[mission-1|Mission1]] - Stack with LIFO operations
- [[mission-2]] - Queue with ring buffer optimization
- [[mission-3]] - Binary search with traits and generics
- [[mission-4]] - Linked list with interior mutability
- [[mission-5]] - HashMap with collision resolution
- [[mission-6]] - Grid algorithms and pathfinding
- [[mission-7]] - Graph representation and traversal
- [[mission-8]] - Advanced graph algorithms and composition
- [[mission-9]] - Dijkstra and A* pathfinding

### **Advanced Data Structures**

- [[Binary Heap Data Structure]] - Priority queue implementation
- [[Trie Data Structure]] - Prefix tree for strings
- [[Segment Tree]] - Range query data structure
- [[Fenwick Tree]] - Binary indexed tree for prefix sums
- [[Disjoint Set Union]] - Union-Find with optimizations

## 🦀 Rust-Specific Algorithms

### **Iterator Patterns**

- [[Iterator Patterns]] - Rust iterator combinators
- [[Binary Search Iterator Patterns]] - Iterator-based search
- [[Iterator Performance]] - Zero-cost iterator abstractions

### **Ownership Patterns**

- [[Ownership and Borrowing]] - Memory safety in algorithms
- [[Lifetime Patterns]] - Lifetime management in data structures
- [[interior-mutability]] - RefCell and Cell for shared mutation

### **Type System Algorithms**

- [[Generic Programming]] - Type-parameterized algorithms
- [[Trait Objects]] - Dynamic dispatch patterns
- [[Associated Types]] - Type-level algorithm design

## 📊 Learning Progression

### **Beginner Level**
1. Linear search and basic iteration
2. Simple sorting (bubble, insertion)
3. Stack and queue operations
4. Basic recursion

### **Intermediate Level**
1. Binary search variants
2. Merge sort and quick sort
3. BFS and DFS traversal
4. Hash table algorithms
5. Basic dynamic programming

### **Advanced Level**
1. Advanced graph algorithms
2. Segment trees and range queries
3. String algorithms (KMP, Z-algorithm)
4. Computational geometry
5. Advanced DP and optimization

### **Expert Level**
1. Network flow algorithms
2. Suffix arrays and trees
3. Advanced data structures (treaps, splay trees)
4. Approximation algorithms
5. Online algorithms

## 🔗 Cross-References

### **Related MOCs**
- [[AoC Patterns MOC]] - Competitive programming applications
- [[Collections MOC]] - Data structure fundamentals
- [[rust-concepts-MOC]] - Language features for algorithms
- [[Daily Study MOC]] - Progressive learning path

### **Key Concepts**
- [[Complexity Analysis]] - Performance understanding
- [[Testing Strategies]] - Algorithm verification
- [[Debugging Strategies]] - Problem-solving techniques
- [[Code Optimization]] - Performance improvement

### **Applications**
- [[Mission Testing Strategies]] - Testing algorithm implementations
- [[Performance Benchmarking]] - Measuring algorithm efficiency
- [[AoC Problem Solving]] - Competitive programming practice

## 📚 Resources & References

### **Books**
- Introduction to Algorithms (CLRS)
- Algorithm Design Manual (Skiena)
- Competitive Programmer's Handbook

### **Online Resources**
- [Algorithmica](https://en.algorithmica.org/hpc/) - High-performance algorithms
- [Rust Algorithm Club](https://github.com/EbTech/rust-algorithms)
- [LeetCode](https://leetcode.com/) - Practice problems

### **Workspace Resources**
- [[rust_book/rust-book-ch10]] - Generics and traits for algorithms
- [[rust_book/rust-book-ch13]] - Iterators and closures
- [[tutorials/Mission8_tut/README]] - Advanced algorithm composition

---

## Navigation

**Quick Access:**
- [[zettel-index]] - Main knowledge base
- [[Daily Study MOC]] - Learning progression
- [[Missions Overview]] - Implementation projects
- [[Rust Book MOC]] - Language fundamentals

**Problem Solving:**
- [[AoC Patterns MOC]] - Competition patterns
- [[Grid Pattern Recognition]] - 2D problem strategies
- [[Graph Algorithms]] - Network analysis

---

*Tags: #algorithms #moc #data-structures #problem-solving #competitive-programming #graph-algorithms #dynamic-programming #sorting #searching #optimization*

*Links: [[zettel-index]] | [[AoC Patterns MOC]] | [[rust-concepts-MOC]] | [[Collections MOC]] | [[Daily Study MOC]]*

*Created: 2025-10-27 | Status: 🎯 Active Reference | Purpose: Algorithm knowledge organization*
