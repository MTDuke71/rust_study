# Chapter 13: Functional Language Features - Iterators and Closures

This chapter covers Rust's functional programming features, focusing on closures and iterators.

## Topics Covered

### 13.1 - Closures: Anonymous Functions that Capture Their Environment
- Closure syntax and type inference
- Capturing the environment (borrowing, mutable borrowing, taking ownership)
- Moving closures and the `move` keyword
- Fn traits: `Fn`, `FnMut`, `FnOnce`

### 13.2 - Processing a Series of Items with Iterators
- The Iterator trait
- Consuming adaptors and iterator adaptors
- Methods that consume iterators
- Creating custom iterators

### 13.3 - Improving Our I/O Project
- Refactoring with iterators
- Performance comparisons
- Zero-cost abstractions

### 13.4 - Comparing Performance: Loops vs Iterators
- Zero-cost abstractions in practice
- Benchmarking loops vs iterators
- Compiler optimization examples
- When to use loops vs iterators
- Performance pitfalls and solutions
- Assembly analysis insights

## Examples

Run examples with:
```bash
cargo run --example ch13_1_closures      # Closure basics and environment capture
cargo run --example ch13_2_iterators     # Iterator trait and adaptors
cargo run --example ch13_3_io_project    # Refactoring with iterators
cargo run --example ch13_4_performance   # Performance comparison and benchmarks
```

## Learning Schedule

- **Nov 2**: Ch 13.1 - Closures and environment capture
- **Nov 3**: Ch 13.2 - Iterators and the Iterator trait
- **Nov 4**: Ch 13.3 - Refactoring I/O project
- **Nov 5**: Ch 13.4 - Performance analysis

---

*Navigation: [[../README.md|Rust Book Overview]]*
