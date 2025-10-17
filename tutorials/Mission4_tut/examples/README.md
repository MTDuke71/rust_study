# Mission4 Tutorial Examples

This directory contains step-by-step examples that demonstrate linked list concepts in Rust progressively.

## How to Run the Examples

Each example is a standalone Rust program. Run them with:

```powershell
# From the Mission4_tut directory
cargo run --example <example_name>

# For performance examples, use release mode:
cargo run --example step7_performance --release
```

## Example Progression

### 1. Basic Structure (`step1_basic_structure.rs`)
- Learn the fundamental `Node` and `SimpleLinkedList` structures
- Understand `Option<Box<Node<T>>>` pattern
- See how Rust handles recursive types

**Key Concepts:**
- `Box<T>` for heap allocation
- `Option<T>` for nullable pointers
- Generic type parameters

### 2. Adding Elements (`step2_push_front.rs`)
- Implement `push_front()` operation
- Understand ownership transfer with `take()`
- See LIFO (Last In, First Out) behavior

**Key Concepts:**
- `take()` method for moving out of `Option`
- Ownership transfer in linked structures
- Stack-like behavior

### 3. Safe Peeking (`step3_peeking.rs`)
- Implement safe reference access with `peek_front()`
- Learn `as_ref()` and `as_mut()` patterns
- Understand borrowing vs. ownership

**Key Concepts:**
- `as_ref()` for converting owned to borrowed
- Immutable vs. mutable references
- Non-consuming access patterns

### 4. Removing Elements (`step4_popping.rs`)
- Implement `pop_front()` with ownership transfer
- Handle empty list edge cases
- Demonstrate value extraction

**Key Concepts:**
- Destructuring and pattern matching
- Edge case handling
- Owned value return

### 5. Rc Basics (`step5_rc_basics.rs`)
- Introduction to `Rc<RefCell<T>>` pattern
- Understand reference counting
- Learn about shared ownership

**Key Concepts:**
- `Rc::new()` and `clone()` for sharing
- Reference counting mechanics
- Shared vs. unique ownership

### 6. Borrow Checking (`step6_borrow_checking.rs`)
- Understand runtime borrow checking
- Handle `BorrowError` and `BorrowMutError`
- Learn scope management techniques

**Key Concepts:**
- Runtime vs. compile-time checking
- `try_borrow()` for safe access
- Borrow conflict resolution

### 7. Performance Comparison (`step7_performance.rs`)
- Compare `Box<T>` vs. `Rc<RefCell<T>>` performance
- Analyze memory usage patterns
- Understand cache locality effects

**Key Concepts:**
- Performance measurement
- Memory overhead analysis
- Cache-friendly data structures

## Expected Output Examples

### Step 1 Output:
```
=== Step 1: Basic Structure ===
Created empty list: SimpleLinkedList { head: None, length: 0 }
Length: 0
Is empty: true
Created empty string list: SimpleLinkedList { head: None, length: 0 }
✅ Basic structure works!
```

### Step 6 Output:
```
=== Step 6: Runtime Borrow Checking ===
=== Demonstrating Successful Borrowing ===
First borrow: 24
Second borrow: 24
Both borrows active simultaneously: OK!

=== Demonstrating Borrow Conflicts ===
Acquired mutable borrow
✅ Correctly detected borrow conflict (mut + immut)
✅ Correctly detected borrow conflict (mut + mut)
✅ Borrowing works after conflict resolved: 100
```

### Step 7 Output (Release Mode):
```
=== Benchmarking 10000 elements ===
Box<T> implementation: 2.1ms
Rc<RefCell<T>> implementation: 4.8ms
Overhead ratio: 2.29x
⚠️  Moderate overhead
```

## Learning Path Recommendations

### For Beginners:
1. Start with steps 1-4 to understand basic linked list operations
2. Focus on understanding `Box<T>` and ownership transfer
3. Practice the exercises after each step

### For Intermediate Learners:
1. Work through all steps in order
2. Pay special attention to steps 5-6 for advanced patterns
3. Try modifying the examples to add new features

### For Advanced Users:
1. Focus on steps 6-7 for performance and error handling
2. Use these as templates for your own data structures
3. Experiment with the troubleshooting scenarios

## Common Exercises

After running each example, try these modifications:

1. **Add Display**: Implement a method to print all elements
2. **Add push_back**: Implement insertion at the end
3. **Add length tracking**: Ensure length is always correct
4. **Add iterators**: Implement `Iterator` trait
5. **Add error handling**: Use `Result` types for operations
6. **Add tests**: Write unit tests for each operation

## Debugging Tips

- Use `println!` liberally to understand data flow
- Pay attention to ownership transfer in debugger
- Use `Rc::strong_count()` to track reference counts
- Test edge cases (empty lists, single elements)

---

## 🔗 Related Resources

### **Main Tutorial**
- [[../README]] - Mission4_tut main tutorial guide
- [[../../Mission4/README]] - Mission4 main implementation

### **Zettelkasten Knowledge Base**
- [[../../../zettelkasten/Missions Overview]] - All mission overviews
- [[../../../zettelkasten/Collections MOC]] - Data structure patterns
- [[../../../zettelkasten/Box Smart Pointer Patterns]] - Box<T> deep dive
- [[../../../zettelkasten/zettel-index]] - Main knowledge index

### **Compilation & Debugging**
- [[../compilation_stages/README]] - Understanding Rust compilation stages
- [[../TROUBLESHOOTING]] - Common errors and solutions
- [[../COMPILE_ERROR_ANALYSIS]] - Error message analysis

### **Daily Study Integration**
- [[../../../daily_study/rust_learning_week1_notes/Day02]] - Ownership basics
- [[../../../daily_study/rust_learning_week1_notes/Day03]] - Borrowing and references
- [[../../../daily_study/rust_learning_week2_notes/Day08]] - Vec fundamentals (similar patterns)

---

*Tags: #mission4 #linked-list #tutorial #examples #box #rc #refcell #ownership #borrowing*

*Links: [[../README]] | [[../../Mission4/README]] | [[../../../zettelkasten/Missions Overview]] | [[../../../zettelkasten/Collections MOC]] | [[../../../zettelkasten/zettel-index]]*

---

**Next Steps**: After completing these examples, move on to the exercises in the `exercises/` directory for hands-on practice!