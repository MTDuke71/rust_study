# Rust for Rustaceans - Chapter 1: Foundations

Comprehensive examples and exercises covering Chapter 1 of "Rust for Rustaceans" by Jon Gjengset.

## 📚 Chapter Overview

Chapter 1 establishes precise understanding of Rust's fundamental concepts:
- Memory terminology (values, variables, pointers)
- Memory regions (stack, heap, static)
- Ownership (move semantics, copy semantics, dropping)
- Borrowing (shared and mutable references)
- Lifetimes (borrow checker, generic lifetimes, variance)

## 🚀 Running Examples

### Build and Run All Examples

```powershell
cd rust_for_rustaceans\Ch01\foundations

# Run main binary
cargo run

# Run specific example
cargo run --example memory_terminology
cargo run --example ownership_semantics
cargo run --example borrowing_lifetimes
cargo run --example interior_mutability
cargo run --example variance
cargo run --example full_chapter
```

### Quick Test

```powershell
# Test all code compiles
cargo build --examples

# Check with clippy
cargo clippy --all-targets -- -D warnings
```

## 📖 Example Organization

### 1. Memory Terminology (`memory_terminology.rs`)
- Values, variables, and pointers
- Stack vs heap vs static memory
- High-level vs low-level variable models
- Memory safety guarantees

**Key Concepts:**
- Values = Type + Domain element
- Variables = Named slots on stack
- Pointers = Address holders
- Memory regions have different lifetimes

### 2. Ownership Semantics (`ownership_semantics.rs`)
- Move semantics for non-Copy types
- Copy semantics for stack-only types
- Drop trait and automatic cleanup
- `mem::take()`, `mem::replace()`, `mem::swap()`

**Key Concepts:**
- Each value has ONE owner
- Ownership transfers invalidate old owner
- Copy types duplicate on assignment
- Dropping happens automatically at scope end

### 3. Borrowing and Lifetimes (`borrowing_lifetimes.rs`)
- Shared references (`&T`) - multiple immutable borrows
- Mutable references (`&mut T`) - single exclusive borrow
- Borrow checker flow analysis
- Lifetime annotations and elision
- Non-contiguous lifetimes

**Key Concepts:**
- Multiple `&T` OR one `&mut T`
- References must always be valid
- Lifetimes track reference validity
- Compiler optimizes based on aliasing rules

### 4. Interior Mutability (`interior_mutability.rs`)
- `Cell<T>` for Copy types
- `RefCell<T>` for runtime borrow checking
- `Rc<RefCell<T>>` for shared mutable ownership
- `Mutex<T>` for thread-safe mutation
- `UnsafeCell<T>` as the foundation

**Key Concepts:**
- Mutate through shared reference
- Runtime borrow checking instead of compile-time
- Built on `UnsafeCell<T>` primitive
- Trade safety for flexibility

### 5. Variance (`variance.rs`)
- Covariance (subtyping preserved)
- Invariance (no subtyping)
- Contravariance (subtyping reversed)
- `PhantomData` for controlling variance
- Variance table for common types

**Key Concepts:**
- `&'a T` is covariant
- `&'a mut T` is invariant over `'a`
- Affects borrow checker behavior
- Critical for unsafe code

### 6. Full Chapter (`full_chapter.rs`)
All listings from Chapter 1 in one comprehensive example with detailed output.

## 🔗 Integration with Zettelkasten

Related zettelkasten notes:
- [[Memory Management]] - Comprehensive memory guide
- [[Ownership and Borrowing]] - Core concepts
- [[Borrow Checker Fundamentals]] - Mental models
- [[lifetime-parameters]] - Generic lifetimes
- [[interior-mutability]] - Interior mutability patterns
- [[drop-trait]] - RAII and cleanup
- [[Smart Pointers MOC]] - Box, Rc, RefCell

Concepts to create:
- [[move-semantics]] - Move patterns
- [[copy-trait]] - Copy trait details
- [[static-lifetime]] - 'static lifetime
- [[variance]] - Variance deep dive

## 📝 Original Code

The original `Ch01.rs` file contains all the book listings and has been reorganized into focused examples for better learning.

## 🎯 Learning Path

1. **Start with memory_terminology** - Foundation concepts
2. **Move to ownership_semantics** - Understanding ownership rules
3. **Study borrowing_lifetimes** - References and borrow checker
4. **Explore interior_mutability** - Advanced mutation patterns
5. **Understand variance** - Advanced type system concepts
6. **Run full_chapter** - See everything together

## ✅ Workspace Integration

This package is part of the main `rust_study` workspace. Add to root `Cargo.toml`:

```toml
members = [
    # ... other members
    "rust_for_rustaceans/Ch01/foundations",
]
```

## 🔍 Quick Reference

### Ownership Rules
1. Each value has one owner
2. Only one owner at a time
3. Value dropped when owner goes out of scope

### Borrowing Rules
1. Multiple immutable borrows OR one mutable borrow
2. References must always be valid
3. Cannot borrow while value is moved

### Memory Regions
- **Stack**: Fast, fixed size, automatic cleanup
- **Heap**: Flexible size, manual management (automated by Rust)
- **Static**: Lives for entire program (`'static`)

### Variance Quick Reference
| Type | Variance over T | Variance over 'a |
|------|----------------|-----------------|
| `&'a T` | Covariant | Covariant |
| `&'a mut T` | Covariant | Invariant |
| `Box<T>` | Covariant | N/A |
| `Cell<T>` | Invariant | N/A |

## 📚 Further Reading

- [Rust for Rustaceans](https://rust-for-rustaceans.com/) - The book
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - Unsafe Rust
- [Rust Reference: Subtyping](https://doc.rust-lang.org/reference/subtyping.html)
- [[rust-concepts-MOC]] - Zettelkasten concepts map

---

*Part of the [rust_study](../../README.md) learning workspace*
