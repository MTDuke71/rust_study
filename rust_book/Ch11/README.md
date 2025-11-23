# Chapter 11: Writing Automated Tests

## 🔗 Zettelkasten Links
- **Overview**: [[zettelkasten/rust_book/rust-book-ch11]]
- **Previous**: [[zettelkasten/rust_book/rust-book-ch10]]
- **Next**: [[zettelkasten/rust_book/rust-book-ch12]]
- **Missions**: [[mission-8]] - Testing algorithm implementations | [[mission-1]] - Unit testing data structures
- **Daily Study**: [[daily-study/Day25]] | [[daily-study/Day26]] | [[daily-study/Day27]]
- **Book MOC**: [[Rust Book MOC]]

## 📚 Overview

Chapter 11 introduces Rust's built-in testing framework and best practices for writing reliable, maintainable tests. Testing is crucial for ensuring your code works as expected and continues to work as you make changes.

---

## 🎯 Key Concepts

### 1. **Test Functions**
Tests are functions annotated with `#[test]` that verify code behavior.

```rust
#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn exploration() {
    assert_eq!(2 + 2, 4);
}
```

### 2. **Assertion Macros**
Rust provides several macros for making assertions in tests:

```rust
// Basic assertions
assert!(true);                    // Panics if false
assert_eq!(4, 2 + 2);            // Panics if not equal
assert_ne!(3, 2 + 2);            // Panics if equal

// With custom messages
assert!(result.is_ok(), "Operation failed: {:?}", result);
assert_eq!(expected, actual, "Values should be equal");
```

### 3. **Test Organization**
Tests can be organized as unit tests (in the same file) or integration tests (in separate files).

```rust
// Unit tests in the same file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_test() {
        // Test private functions
    }
}
```

---

## 📂 Subchapters

### [11.1 How to Write Tests](./testing_basics/)
- Writing test functions with `#[test]`
- Using assertion macros
- Testing for panics with `#[should_panic]`
- Using `Result<T, E>` in tests

### [11.2 Controlling How Tests Are Run](./test_execution/)
- Running tests in parallel vs sequentially
- Filtering tests by name
- Ignoring tests with `#[ignore]`
- Running specific tests

### [11.3 Test Organization](./test_organization/)
- Unit tests vs integration tests
- Testing private functions
- Submodules in integration tests
- Common patterns

---

## 🚀 Quick Start

```bash
# Navigate to Chapter 11
cd rust_book/Ch11

# Run all chapter examples
cargo test --workspace

# Run specific test examples
cargo test --package testing_basics
cargo test --package test_execution
cargo test --package test_organization

# Run with output
cargo test -- --nocapture

# Run tests sequentially
cargo test -- --test-threads=1
```

---

## 🔬 Practical Examples

### Testing Data Structures (Mission Integration)

```rust
// Testing a stack implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_push_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        
        let mut stack = Stack::new();
        stack.push(1);
        assert!(!stack.is_empty());
    }
}
```

### Testing Algorithms

```rust
// Testing BFS implementation
#[cfg(test)]
mod algorithm_tests {
    use super::*;

    #[test]
    fn test_bfs_shortest_path() {
        let graph = create_test_graph();
        let path = bfs_shortest_path(&graph, 0, 4);
        
        assert!(path.is_some());
        assert_eq!(path.unwrap().len(), 3); // Expected path length
    }

    #[test]
    #[should_panic(expected = "Node not found")]
    fn test_bfs_invalid_node() {
        let graph = create_test_graph();
        bfs_shortest_path(&graph, 0, 999); // Invalid node
    }
}
```

---

## 📊 Testing Best Practices

### 1. **Test Naming Conventions**
```rust
#[test]
fn test_function_name_expected_behavior() {
    // Clear, descriptive names
}

#[test]
fn stack_push_increases_length() {
    // Describes what is being tested
}

#[test]
fn empty_queue_returns_none_on_dequeue() {
    // Describes the scenario and expected outcome
}
```

### 2. **Arrange-Act-Assert Pattern**
```rust
#[test]
fn test_hashmap_insert_and_get() {
    // Arrange
    let mut map = HashMap::new();
    let key = "test_key";
    let value = 42;
    
    // Act
    map.insert(key, value);
    let result = map.get(key);
    
    // Assert
    assert_eq!(result, Some(&value));
}
```

### 3. **Edge Case Testing**
```rust
#[test]
fn test_empty_input() {
    assert_eq!(process_data(&[]), vec![]);
}

#[test]
fn test_single_element() {
    assert_eq!(process_data(&[1]), vec![1]);
}

#[test]
fn test_large_input() {
    let large_vec: Vec<i32> = (0..10000).collect();
    let result = process_data(&large_vec);
    assert_eq!(result.len(), 10000);
}
```

---

## 🎯 Learning Objectives

By the end of this chapter, you should be able to:

1. **Write effective unit tests** using Rust's testing framework
2. **Use assertion macros** appropriately for different scenarios
3. **Organize tests** into logical modules and files
4. **Control test execution** with various cargo test options
5. **Test edge cases and error conditions** thoroughly
6. **Apply testing best practices** to real-world projects
7. **Integrate testing** into your development workflow

---

## 🔗 Integration with Learning Tracks

### **Mission Integration**
- **Mission 1 (Stack)**: Unit testing stack operations
- **Mission 2 (Queue)**: Testing FIFO behavior and edge cases
- **Mission 5 (HashMap)**: Testing collision handling and performance
- **Mission 8 (Algorithms)**: Testing algorithm correctness and performance

### **Daily Study Connection**
- **Day 25**: Unit testing fundamentals and assertion macros
- **Day 26**: Integration testing and test organization
- **Day 27**: Advanced testing patterns and TDD

### **AoC Application**
- Test AoC solutions with provided examples
- Validate algorithm correctness with sample inputs
- Performance testing with large datasets

---

## 📈 Progress Tracking

- [ ] Complete 11.1 - How to Write Tests
- [ ] Complete 11.2 - Controlling How Tests Are Run  
- [ ] Complete 11.3 - Test Organization
- [ ] Apply testing to current Mission work
- [ ] Write comprehensive test suite for one data structure
- [ ] Practice TDD (Test-Driven Development) approach

---

## 🔧 Tools and Commands

```bash
# Basic testing commands
cargo test                       # Run all tests
cargo test --doc                # Run documentation tests
cargo test test_name            # Run specific test
cargo test --package pkg_name   # Run tests for specific package

# Advanced testing options
cargo test -- --nocapture      # Show println! output
cargo test -- --test-threads=1 # Run tests sequentially
cargo test -- --ignored        # Run ignored tests only
cargo test --release           # Run tests in release mode

# Test coverage (with tarpaulin)
cargo tarpaulin --out Html     # Generate coverage report
```

---

## 📚 Additional Resources

- **Rust Book Chapter 11**: [Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- **Rust Testing Documentation**: [https://doc.rust-lang.org/rust-by-example/testing.html](https://doc.rust-lang.org/rust-by-example/testing.html)
- **Test-Driven Development**: [[TDD Patterns]]
- **Mission Testing Examples**: [[Mission Testing Strategies]]

---

*Tags: #rust-book #chapter11 #testing #unit-tests #integration-tests #tdd #assertions*
*Links: [[Rust Book MOC]] | [[Testing Patterns]] | [[Mission Testing Strategies]] | [[Daily Study MOC]]*