# 🧪 Unit Testing

**Comprehensive guide to function-level testing, component validation, and individual requirement testing in Rust**

## 🎯 Overview

Unit testing in Rust focuses on testing individual functions, methods, and components in isolation. This approach ensures that each piece of functionality works correctly before integrating with other components. Unit tests are fast, reliable, and provide immediate feedback during development.

## 🔧 Basic Unit Testing

### **Function-Level Testing**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(subtract(10, 3), 7);
        assert_eq!(multiply(4, 5), 20);
        assert_eq!(divide(15, 3), 5);
    }

    #[test]
    fn test_edge_cases() {
        // Test with zero
        assert_eq!(add(0, 5), 5);
        assert_eq!(multiply(0, 100), 0);
        
        // Test with negative numbers
        assert_eq!(add(-5, 3), -2);
        assert_eq!(subtract(3, 5), -2);
    }
}
```

### **Method Testing**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_push_pop() {
        let mut stack = Stack::new();
        
        // Test empty stack
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.pop(), None);
        
        // Test push operation
        stack.push(42);
        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.peek(), Some(&42));
        
        // Test pop operation
        assert_eq!(stack.pop(), Some(42));
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }
}
```

## 🎯 Test Organization

### **Modular Test Structure**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    mod basic_operations {
        use super::*;
        
        #[test]
        fn test_push() {
            let mut stack = Stack::new();
            stack.push(42);
            assert_eq!(stack.peek(), Some(&42));
        }
        
        #[test]
        fn test_pop() {
            let mut stack = Stack::new();
            stack.push(42);
            assert_eq!(stack.pop(), Some(42));
        }
    }
    
    mod edge_cases {
        use super::*;
        
        #[test]
        fn test_empty_stack_operations() {
            let mut stack = Stack::new();
            assert_eq!(stack.pop(), None);
            assert_eq!(stack.peek(), None);
        }
        
        #[test]
        fn test_single_element() {
            let mut stack = Stack::new();
            stack.push(42);
            assert_eq!(stack.len(), 1);
            assert_eq!(stack.pop(), Some(42));
            assert_eq!(stack.len(), 0);
        }
    }
    
    mod error_conditions {
        use super::*;
        
        #[test]
        fn test_overflow_handling() {
            let mut stack = Stack::with_capacity(2);
            stack.push(1);
            stack.push(2);
            
            // Should handle overflow gracefully
            let result = stack.push(3);
            assert!(result.is_err());
        }
    }
}
```

### **Test Data Helpers**
```rust
#[cfg(test)]
mod test_helpers {
    use super::*;
    
    fn create_test_stack() -> Stack<i32> {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack
    }
    
    fn create_large_stack(size: usize) -> Stack<i32> {
        let mut stack = Stack::new();
        for i in 0..size {
            stack.push(i as i32);
        }
        stack
    }
    
    fn create_stack_with_data<T: Clone>(items: &[T]) -> Stack<T> {
        let mut stack = Stack::new();
        for item in items {
            stack.push(item.clone());
        }
        stack
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_helpers::*;
    
    #[test]
    fn test_helper_functions() {
        let stack = create_test_stack();
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.peek(), Some(&3));
    }
    
    #[test]
    fn test_large_stack() {
        let stack = create_large_stack(1000);
        assert_eq!(stack.len(), 1000);
    }
}
```

## 🔍 Assertion Patterns

### **Basic Assertions**
```rust
#[test]
fn test_basic_assertions() {
    let result = calculate_value(42);
    
    // Equality assertions
    assert_eq!(result, 84);
    assert_ne!(result, 0);
    
    // Boolean assertions
    assert!(result > 0);
    assert!(!result.is_negative());
    
    // Option assertions
    let option_result = find_value("key");
    assert!(option_result.is_some());
    assert_eq!(option_result.unwrap(), "value");
    
    // Result assertions
    let result_value = parse_number("42");
    assert!(result_value.is_ok());
    assert_eq!(result_value.unwrap(), 42);
}
```

### **Custom Assertions**
```rust
fn assert_stack_valid<T>(stack: &Stack<T>) {
    if !stack.is_empty() {
        assert!(stack.len() > 0);
        assert!(stack.peek().is_some());
    } else {
        assert_eq!(stack.len(), 0);
        assert!(stack.peek().is_none());
    }
}

#[test]
fn test_custom_assertions() {
    let mut stack = Stack::new();
    assert_stack_valid(&stack);
    
    stack.push(42);
    assert_stack_valid(&stack);
    
    stack.pop();
    assert_stack_valid(&stack);
}
```

### **Error Testing**
```rust
#[test]
fn test_error_conditions() {
    // Test Result error cases
    let result = parse_number("invalid");
    assert!(result.is_err());
    
    match result {
        Err(ParseError::InvalidFormat) => {}, // Expected
        _ => panic!("Unexpected error type"),
    }
    
    // Test Option None cases
    let option = find_element(&vec![1, 2, 3], 4);
    assert!(option.is_none());
}

#[test]
#[should_panic(expected = "Division by zero")]
fn test_panic_conditions() {
    divide(10, 0);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_bounds_checking() {
    let vec = vec![1, 2, 3];
    let _ = vec[10];
}
```

## 🎯 Requirement-Based Unit Testing

### **REQ-1: Generic Support Testing**
```rust
#[test] // REQ-1: Generic Support
fn req1_generic_support() {
    // Test with primitive types
    let mut int_stack = Stack::<i32>::new();
    int_stack.push(42);
    assert_eq!(int_stack.pop(), Some(42));
    
    // Test with heap-allocated types
    let mut string_stack = Stack::<String>::new();
    string_stack.push("Hello".to_string());
    assert_eq!(int_stack.pop(), Some("Hello".to_string()));
    
    // Test with custom types
    #[derive(Debug, Clone, PartialEq)]
    struct TestStruct {
        value: i32,
        name: String,
    }
    
    let mut struct_stack = Stack::<TestStruct>::new();
    struct_stack.push(TestStruct {
        value: 42,
        name: "test".to_string(),
    });
    assert!(struct_stack.pop().is_some());
}
```

### **REQ-2: Performance Characteristics**
```rust
#[test] // REQ-2: Amortized Constant Time
fn req2_amortized_constant_time() {
    let mut stack = Stack::with_capacity(2);
    
    // Test that push operations are O(1) amortized
    let start = std::time::Instant::now();
    
    for i in 0..1000 {
        stack.push(i);
    }
    
    let duration = start.elapsed();
    assert!(duration.as_millis() < 10); // Should be very fast
    
    // Verify all elements were pushed correctly
    assert_eq!(stack.len(), 1000);
}
```

### **REQ-3: Ownership Semantics**
```rust
#[test] // REQ-3: Ownership Transfer
fn req3_ownership_transfer() {
    let mut stack = Stack::new();
    let owned_string = String::from("owned");
    
    // Push transfers ownership
    stack.push(owned_string);
    // owned_string is now moved and cannot be used
    
    // Pop returns ownership
    let popped = stack.pop().unwrap();
    assert_eq!(popped, "owned");
    
    // popped is now owned by this scope
    println!("Popped value: {}", popped);
}
```

## 🔧 Mock Testing

### **Trait Mocking**
```rust
use mockall::*;

#[automock]
trait Database {
    fn get_user(&self, id: u32) -> Result<User, DatabaseError>;
    fn save_user(&self, user: &User) -> Result<(), DatabaseError>;
}

#[test]
fn test_user_service_with_mock() {
    let mut mock_db = MockDatabase::new();
    
    // Set up mock expectations
    mock_db.expect_get_user()
        .with(eq(42))
        .times(1)
        .returning(|_| Ok(User::new(42, "Alice")));
    
    mock_db.expect_save_user()
        .withf(|user| user.id == 42)
        .times(1)
        .returning(|_| Ok(()));
    
    // Test the service with mocked dependencies
    let service = UserService::new(mock_db);
    
    let user = service.get_user(42).unwrap();
    assert_eq!(user.id, 42);
    assert_eq!(user.name, "Alice");
    
    let save_result = service.save_user(&user);
    assert!(save_result.is_ok());
}
```

### **State Mocking**
```rust
#[test]
fn test_with_mock_state() {
    let mut mock_state = MockState::new();
    
    // Set up initial state
    mock_state.expect_get_value()
        .return_const(42);
    
    mock_state.expect_set_value()
        .with(eq(100))
        .times(1)
        .returning(|_| Ok(()));
    
    // Test component with mocked state
    let component = Component::new(mock_state);
    
    let current_value = component.get_current_value();
    assert_eq!(current_value, 42);
    
    let result = component.update_value(100);
    assert!(result.is_ok());
}
```

## 📊 Test Coverage and Quality

### **Coverage Analysis**
```rust
#[test]
fn test_all_code_paths() {
    // Test success path
    let result = process_data("valid");
    assert!(result.is_ok());
    
    // Test error path
    let result = process_data("invalid");
    assert!(result.is_err());
    
    // Test edge case
    let result = process_data("");
    assert!(result.is_err());
}

#[test]
fn test_boundary_conditions() {
    // Test minimum values
    let result = calculate(0);
    assert_eq!(result, 0);
    
    // Test maximum values
    let result = calculate(i32::MAX);
    assert!(result > 0);
    
    // Test edge of valid range
    let result = calculate(100);
    assert_eq!(result, 10000);
}
```

### **Property-Based Testing**
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_stack_properties(
        operations in prop::collection::vec(any::<StackOperation>(), 0..100)
    ) {
        let mut stack = Stack::new();
        
        for op in operations {
            match op {
                StackOperation::Push(val) => stack.push(val),
                StackOperation::Pop => { stack.pop(); }
            }
            
            // Invariant: length should never be negative
            assert!(stack.len() >= 0);
            
            // Invariant: if not empty, peek should be valid
            if !stack.is_empty() {
                assert!(stack.peek().is_some());
            }
        }
    }
}
```

## 🚀 Performance Testing

### **Benchmark Unit Tests**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_push_operation(c: &mut Criterion) {
    c.bench_function("stack_push", |b| {
        b.iter(|| {
            let mut stack = Stack::new();
            for i in 0..1000 {
                stack.push(black_box(i));
            }
        });
    });
}

fn benchmark_pop_operation(c: &mut Criterion) {
    c.bench_function("stack_pop", |b| {
        b.iter(|| {
            let mut stack = Stack::new();
            for i in 0..1000 {
                stack.push(i);
            }
            while stack.pop().is_some() {}
        });
    });
}

criterion_group!(benches, benchmark_push_operation, benchmark_pop_operation);
criterion_main!(benches);
```

## 🔗 Related Concepts

### **Testing Integration**
- **[[Integration Testing]]** - Testing component interactions
- **[[Testing Strategies]]** - Comprehensive testing approaches
- **[[TDD (Test-Driven Development)]]** - Writing tests before implementation

### **Mission Applications**
- **[[mission-1]]** - Stack unit testing with ownership validation
- **[[mission-2]]** - Queue unit testing with FIFO semantics
- **[[Mission4 Overview]]** - Interior mutability unit testing
- **[[Mission5 Overview]]** - HashMap unit testing with collision handling
- **[[Mission6 Overview]]** - Grid unit testing with bounds checking

### **Development Methodology**
- **[[REQ-1 Test Strategy]]** - Requirement-based unit testing approach
- **[[V-Cycle Methodology]]** - Systematic unit testing in development lifecycle
- **[[Debugging Lessons]]** - Using unit tests for debugging

### **Error Handling Integration**
- **[[Error Handling Patterns]]** - Testing error conditions and recovery
- **[[daily-study/Day33]]** - Testing panic conditions
- **[[daily-study/Day34]]** - Testing error handling patterns

## 🎯 Unit Testing Best Practices

### **DO:**
- Write tests for all public functions
- Test both success and error paths
- Use descriptive test names
- Group related tests in modules
- Use helper functions for test data
- Test edge cases and boundary conditions
- Keep tests fast and independent

### **DON'T:**
- Test private implementation details
- Write tests that depend on other tests
- Use complex setup in tests
- Ignore failing tests
- Test external dependencies directly
- Write tests that are too broad or too narrow

## 📋 Unit Testing Checklist

### **Before Writing Tests:**
- [ ] Identify all public functions to test
- [ ] Plan test data and edge cases
- [ ] Design helper functions for common setup
- [ ] Consider error conditions and panic cases

### **While Writing Tests:**
- [ ] Test each function in isolation
- [ ] Cover both success and error paths
- [ ] Validate ownership and borrowing rules
- [ ] Test performance characteristics
- [ ] Use descriptive test names

### **After Writing Tests:**
- [ ] Run tests frequently during development
- [ ] Ensure all tests pass before committing
- [ ] Review test coverage metrics
- [ ] Refactor tests for clarity and maintainability

---

*Tags: #unit-testing #function-testing #component-testing #requirement-testing #assertions #mocking #coverage*
*Links: [[zettel-index]] | [[rust-book-ch9-12-review]] | [[Integration Testing]] | [[Testing Strategies]] | [[TDD (Test-Driven Development)]] | [[REQ-1 Test Strategy]] | [[V-Cycle Methodology]] | [[../rust_book/Ch11/CHAPTER_COMPLETE]] | [[Debugging Lessons]] | [[Error Handling Patterns]] | [[mission-1]] | [[mission-2]] | [[Mission4 Overview]] | [[Mission5 Overview]] | [[Mission6 Overview]]*
