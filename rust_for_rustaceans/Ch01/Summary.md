# Chapter 1: Foundations

## Overview

Chapter 1, titled **Foundations**, focuses on establishing a precise understanding of Rust's fundamental concepts regarding memory, ownership, and borrowing to support reasoning about more complex topics later in the book.

---

## 1. Talking about Memory

### a. Memory Terminology

#### Values
The combination of a type and an element of that type's domain (e.g., the number `6` as a `u8`).

#### Variables
Named slots on the stack that hold values.

#### Pointers
Values that hold the address of a region of memory.

---

### b. Variables in Depth

#### High-Level Model
Views variables as names given to values within dependency flows; variables exist only as long as they hold legal values.

#### Low-Level Model
Views variables as memory locations (value slots) that may or may not hold legal values at any given time.

---

### c. Memory Regions

#### The Stack
A scratch space for function calls using stack frames. Variables here live only as long as the frame exists.

#### The Heap
A pool of memory for values that need to live beyond a specific function call. It requires explicit allocation and deallocation (freeing).

#### Static Memory
Contains the program binary and static variables. Values here live for the entire execution of the program (`'static`).

---

## 2. Ownership

### Move Semantics
Values have a single owner. When moved, the old location is invalidated.

### Copy Semantics
Types implementing the `Copy` trait duplicate their bits rather than moving ownership.

### Dropping
Owners are responsible for cleaning up values (dropping) when execution leaves the scope.

---

## 3. Borrowing and Lifetimes

### a. Shared References (`&T`)
Pointers that allow shared access but disallow mutation. The compiler assumes values behind these do not change.

### b. Mutable References (`&mut T`)
Pointers that guarantee exclusive access. The compiler assumes no other threads or pointers are accessing the target value.

### c. Interior Mutability
Types that allow mutation through shared references (e.g., `Mutex`, `RefCell`) using `UnsafeCell` under the hood.

### d. Lifetimes

#### The Borrow Checker
Checks that references point to valid values by tracing data flows. Lifetimes are regions of code where a reference is valid and do not necessarily need to be contiguous.

#### Generic Lifetimes
Creating types generic over lifetimes to store references.

#### Variance
How subtypes interact with generic parameters (Covariance, Invariance, and Contravariance) and how this impacts the borrow checker.
