# Chapter 1: Foundations

## Overview

Chapter 1, titled **Foundations**, focuses on establishing a precise understanding of Rust's fundamental concepts regarding memory, ownership, and borrowing to support reasoning about more complex topics later in the book.

**Related Concepts:** [[Memory Management]], [[Ownership and Borrowing]], [[Borrow Checker Fundamentals]], [[Lifetime Parameters]], [[rust-concepts-MOC]]

---

## 1. Talking about Memory

### a. Memory Terminology

**Related:** [[Memory Management]], [[ownership-fundamentals]]

#### Values
The combination of a type and an element of that type's domain (e.g., the number `6` as a `u8`).

#### Variables
Named slots on the stack that hold values.

#### Pointers
Values that hold the address of a region of memory (references). See also [[Smart Pointers MOC]] for advanced pointer types.

---

### b. Variables in Depth

**Related:** [[ownership-fundamentals]], [[Ownership Mental Model - The Library Analogy]]

#### High-Level Model (useful at level of lifetimes and borrows)
Views variables as names given to values within dependency flows; variables exist only as long as they hold legal values.

#### Low-Level Model (useful at level of unsafe code and raw pointers)
Views variables as memory locations (value slots) that may or may not hold legal values at any given time. See [[memory-address-analysis]] for low-level details.

---

### c. Memory Regions

**Related:** [[Memory Management]], [[arena-allocation]], [[box-learning-guide]]

#### The Stack
A scratch space for function calls using stack frames. Variables here live only as long as the frame exists.

#### The Heap
A pool of memory for values that need to live beyond a specific function call. It requires explicit allocation and deallocation (freeing). See [[box-learning-guide]] for heap allocation patterns.

#### Static Memory
Contains the program binary and static variables. Values here live for the entire execution of the program (`'static`). Related: [[static-lifetime]].

---

## 2. Ownership

**Related:** [[Ownership and Borrowing]], [[ownership-fundamentals]], [[Ownership Mental Model - The Library Analogy]]

### Move Semantics
Values have a single owner. When moved, the old location is invalidated. See [[move-semantics]] for detailed patterns.

### Copy Semantics
Types implementing the `Copy` trait duplicate their bits rather than moving ownership. Related: [[copy-trait]], [[Clone vs Copy]].

### Dropping
Owners are responsible for cleaning up values (dropping) when execution leaves the scope. See [[drop-trait]] for RAII patterns and automatic cleanup.

---

## 3. Borrowing and Lifetimes

**Related:** [[Ownership and Borrowing]], [[Borrow Checker Fundamentals]], [[Lifetime Parameters]]

### a. Shared References (`&T`)
Pointers that allow shared access but disallow mutation. The compiler assumes values behind these do not change. See [[Borrow Checker Fundamentals]] for detailed rules.

### b. Mutable References (`&mut T`)
Pointers that guarantee exclusive access. The compiler assumes no other threads or pointers are accessing the target value. See [[Borrow Checker Patterns and Troubleshooting]] for common patterns.

### c. Interior Mutability
Types that allow mutation through shared references (e.g., `Mutex`, `RefCell`) using `UnsafeCell` under the hood. See [[interior-mutability]], [[refcell-interior-mutability]], and [[shared-state-concurrency]].

### d. Lifetimes

**Related:** [[Lifetime Parameters]], [[Multiple Lifetimes Deep Dive]]

#### The Borrow Checker
Checks that references point to valid values by tracing data flows. Lifetimes are regions of code where a reference is valid and do not necessarily need to be contiguous. See [[Borrow Checker Fundamentals]] for mental models and examples.

#### Generic Lifetimes
Creating types generic over lifetimes to store references. See [[Lifetime Parameters]] and [[Generic Programming]].

#### Variance
How subtypes interact with generic parameters (Covariance, Invariance, and Contravariance) and how this impacts the borrow checker. See [[variance]] and [[phantom-data-type-safety]].

---

## Links and References

### Existing Zettelkasten Notes
- [[Memory Management]] - Comprehensive guide to Rust's memory safety
- [[Ownership and Borrowing]] - Core Rust concepts
- [[Borrow Checker Fundamentals]] - Understanding the borrow checker with mental models
- [[Lifetime Parameters]] - Lifetime annotations and generic lifetimes
- [[interior-mutability]] - Interior mutability patterns with `RefCell`, `Cell`, `Mutex`
- [[drop-trait]] - Automatic cleanup and RAII patterns
- [[Smart Pointers MOC]] - Box, Rc, RefCell, and more

### Concepts to Create
- [[move-semantics]] - Detailed move semantics patterns and examples
- [[copy-trait]] - Copy trait implementation and semantics
- [[Clone vs Copy]] - When to use Clone vs Copy
- [[static-lifetime]] - Understanding the 'static lifetime
- [[memory-address-analysis]] - Low-level memory layout and addressing
- [[variance]] - Covariance, Invariance, and Contravariance in Rust
- [[phantom-data-type-safety]] - Using PhantomData for variance and type safety

### Related to Missions
- [[mission-4]] - LinkedList (Ownership and interior mutability with Rc/RefCell)
- [[mission-5]] - HashMap (Memory layout and performance)
- [[mission-1]] - Stack (Ownership and drop semantics)

---

*Tags: #rust-for-rustaceans #chapter-1 #foundations #memory #ownership #borrowing #lifetimes #borrow-checker*
