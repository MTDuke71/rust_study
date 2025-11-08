# 🧪 Testing Strategies

**Comprehensive guide to testing approaches, patterns, and best practices in Rust development**

## 🎯 Overview

Testing strategies in Rust development encompass multiple levels of validation - from unit tests for individual functions to integration tests for complete systems. This guide covers the full spectrum of testing approaches used throughout the Rust learning journey, with emphasis on V-Cycle methodology and requirement-based validation.

## 🔄 Testing Levels

### **1. Unit Testing**

#### **Function-Level Testing**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let result = calculate_sum(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(calculate_sum(0, 0), 0);
        assert_eq!(calculate_sum(-1, 1), 0);
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_panic_conditions() {
        divide(10, 0);
    }
}
```

#### **Error Condition Testing**
```rust
#[test]
fn test_error_handling() {
    let result = parse_number("invalid");
    assert!(result.is_err());
    
    match result {
        Err(ParseError::InvalidFormat) => {}, // Expected
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_option_handling() {
    let result = find_element(&vec![1, 2, 3], 4);
    assert!(result.is_none());
    
    let result = find_element(&vec![1, 2, 3], 2);
    assert_eq!(result, Some(1)); // Index 1
}
```

### **2. Integration Testing**

#### **Component Integration**
```rust
// tests/integration_tests.rs
use my_crate::{Stack, Queue};

#[test]
fn test_stack_queue_integration() {
    let mut stack = Stack::new();
    let mut queue = Queue::new();
    
    // Test data flow between components
    stack.push(42);
    queue.enqueue(stack.pop().unwrap());
    
    assert_eq!(queue.dequeue(), Some(42));
}
```

#### **API Integration Testing**
```rust
#[test]
fn test_api_endpoints() {
    let client = create_test_client();
    
    let response = client.post("/api/data", json!({"value": 42}));
    assert_eq!(response.status(), 200);
    
    let data = response.json::<ResponseData>();
    assert_eq!(data.value, 42);
}
```

### **3. Requirement-Based Testing (V-Cycle)**

#### **REQ-1 Foundation Testing**
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
    assert_eq!(string_stack.pop(), Some("Hello".to_string()));
    
    // Test with custom types
    let mut struct_stack = Stack::<CustomStruct>::new();
    struct_stack.push(CustomStruct::new(1, 2));
    assert!(struct_stack.pop().is_some());
}
```

#### **Requirement Validation Matrix**
```rust
#[test] // REQ-2: Performance Characteristics
fn req2_amortized_constant_time() {
    let mut stack = Stack::with_capacity(2);
    let start = std::time::Instant::now();
    
    // Push 1000 elements - should be O(1) amortized
    for i in 0..1000 {
        stack.push(i);
    }
    
    let duration = start.elapsed();
    assert!(duration.as_millis() < 10); // Should be very fast
}

#[test] // REQ-3: Ownership Semantics
fn req3_ownership_transfer() {
    let mut stack = Stack::new();
    let owned_string = String::from("owned");
    
    stack.push(owned_string);
    // owned_string is now moved and cannot be used
    
    let popped = stack.pop().unwrap();
    assert_eq!(popped, "owned");
}
```

## 🛠️ Testing Patterns

### **1. Property-Based Testing**

#### **Invariant Testing**
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_stack_invariants(
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
            
            // Invariant: if not empty, peek should match last pushed
            if !stack.is_empty() {
                assert!(stack.peek().is_some());
            }
        }
    }
}
```

### **2. Mock and Stub Testing**

#### **Trait Mocking**
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
    mock_db.expect_get_user()
        .with(eq(42))
        .times(1)
        .returning(|_| Ok(User::new(42, "Alice")));
    
    let service = UserService::new(mock_db);
    let user = service.get_user(42).unwrap();
    
    assert_eq!(user.id, 42);
    assert_eq!(user.name, "Alice");
}
```

### **3. Fuzz Testing**

#### **Input Fuzzing**
```rust
#[test]
fn test_parser_fuzzing() {
    // Generate random input strings
    for _ in 0..1000 {
        let random_input = generate_random_string();
        
        // Should never panic, always return Result
        let result = parse_input(&random_input);
        
        // Result should be either Ok(parsed) or Err(ParseError)
        match result {
            Ok(parsed) => {
                // If parsing succeeded, validate the result
                assert!(parsed.is_valid());
            }
            Err(_) => {
                // If parsing failed, that's also acceptable
                // Just ensure we don't panic
            }
        }
    }
}
```

## 📚 Documentation Testing

### **Doctest Examples**
```rust
/// Calculates the sum of two numbers
/// 
/// # Examples
/// 
/// ```
/// use my_crate::calculate_sum;
/// 
/// let result = calculate_sum(2, 3);
/// assert_eq!(result, 5);
/// ```
/// 
/// # Panics
/// 
/// This function will panic if the result overflows:
/// 
/// ```should_panic
/// use my_crate::calculate_sum;
/// 
/// // This will panic on overflow
/// calculate_sum(i32::MAX, 1);
/// ```
pub fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b
}
```

### **Code Example Validation**
```rust
/// # Examples
/// 
/// ```
/// use my_crate::Stack;
/// 
/// let mut stack = Stack::new();
/// stack.push(42);
/// assert_eq!(stack.pop(), Some(42));
/// assert_eq!(stack.pop(), None);
/// ```
impl<T> Stack<T> {
    pub fn new() -> Self { /* ... */ }
    pub fn push(&mut self, item: T) { /* ... */ }
    pub fn pop(&mut self) -> Option<T> { /* ... */ }
}
```

## 🎯 Testing Strategies by Context

### **1. Error Handling Testing**

#### **Result Type Testing**
```rust
#[test]
fn test_result_patterns() {
    // Test successful case
    let result = parse_number("42");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
    
    // Test error case
    let result = parse_number("invalid");
    assert!(result.is_err());
    
    // Test error recovery
    let recovered = parse_number("invalid").unwrap_or_default();
    assert_eq!(recovered, 0);
}
```

#### **Panic Testing**
```rust
#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_bounds_checking() {
    let vec = vec![1, 2, 3];
    let _ = vec[10]; // Should panic
}

#[test]
fn test_panic_recovery() {
    let result = std::panic::catch_unwind(|| {
        panic!("Test panic");
    });
    
    assert!(result.is_err());
}
```

### **2. Interior Mutability Testing**

#### **RefCell Testing**
```rust
#[test]
fn test_refcell_borrowing() {
    let cell = RefCell::new(42);
    
    // Test immutable borrow
    {
        let borrow = cell.borrow();
        assert_eq!(*borrow, 42);
    } // borrow goes out of scope
    
    // Test mutable borrow
    {
        let mut borrow = cell.borrow_mut();
        *borrow = 100;
    } // borrow goes out of scope
    
    assert_eq!(*cell.borrow(), 100);
}

#[test]
#[should_panic(expected = "already borrowed")]
fn test_refcell_panic_on_double_borrow() {
    let cell = RefCell::new(42);
    let _borrow1 = cell.borrow();
    let _borrow2 = cell.borrow_mut(); // Should panic
}
```

### **3. Performance Testing**

#### **Benchmark Testing**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_stack_push(c: &mut Criterion) {
    c.bench_function("stack_push", |b| {
        b.iter(|| {
            let mut stack = Stack::new();
            for i in 0..1000 {
                stack.push(black_box(i));
            }
        });
    });
}

fn benchmark_stack_pop(c: &mut Criterion) {
    c.bench_function("stack_pop", |b| {
        let mut stack = Stack::new();
        for i in 0..1000 {
            stack.push(i);
        }
        
        b.iter(|| {
            let mut stack = stack.clone();
            while stack.pop().is_some() {}
        });
    });
}

criterion_group!(benches, benchmark_stack_push, benchmark_stack_pop);
criterion_main!(benches);
```

## 🔍 Testing Best Practices

### **Test Organization**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Group related tests
    mod basic_operations {
        use super::*;
        
        #[test]
        fn test_push() { /* ... */ }
        
        #[test]
        fn test_pop() { /* ... */ }
    }
    
    mod edge_cases {
        use super::*;
        
        #[test]
        fn test_empty_stack() { /* ... */ }
        
        #[test]
        fn test_single_element() { /* ... */ }
    }
    
    mod error_conditions {
        use super::*;
        
        #[test]
        fn test_overflow() { /* ... */ }
        
        #[test]
        fn test_underflow() { /* ... */ }
    }
}
```

### **Test Data Management**
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_helpers::*;
    
    #[test]
    fn test_with_helper_data() {
        let stack = create_test_stack();
        assert_eq!(stack.len(), 3);
    }
}
```

## 🚀 Continuous Integration Testing

### **CI Pipeline Testing**
```yaml
# .github/workflows/test.yml
name: Test Suite

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        rust: [stable, beta, nightly]
    
    steps:
    - uses: actions/checkout@v2
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
    
    - name: Run tests
      run: cargo test --verbose
    
    - name: Run doctests
      run: cargo test --doc
    
    - name: Run integration tests
      run: cargo test --test integration_tests
```

## 📊 Test Coverage and Quality

### **Coverage Analysis**
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run tests with coverage
cargo tarpaulin --out html

# Generate coverage report
cargo tarpaulin --out xml
```

### **Quality Metrics**
- **Line Coverage**: Target 90%+ for critical paths
- **Branch Coverage**: Ensure all error paths are tested
- **Function Coverage**: All public functions should have tests
- **Integration Coverage**: All major user workflows tested

## 🔗 Related Concepts

### **Daily Study Integration**
- **[[daily-study/Day05]]** - Testing with Option and Result types
- **[[daily-study/Day30]]** - Testing error propagation patterns
- **[[daily-study/Day33]]** - Testing panic conditions and recovery
- **[[daily-study/Day34]]** - Testing error handling patterns
- **[[daily-study/Day35]]** - Testing robust parsing scenarios

### **Mission Applications**
- **[[mission-1]]** - Stack testing with ownership validation
- **[[mission-2]]** - Queue testing with FIFO semantics
- **[[Mission4 Overview]]** - Interior mutability testing patterns
- **[[Mission5 Overview]]** - HashMap testing with collision handling
- **[[Mission6 Overview]]** - Grid testing with bounds checking

### **V-Cycle Integration**
- **[[REQ-1 Test Strategy]]** - Requirement-based testing approach
- **[[V-Cycle Methodology]]** - Systematic testing in development lifecycle
- **[[Debugging Lessons]]** - Testing as debugging tool

### **Advanced Testing**
- **[[TDD (Test-Driven Development)]]** - Writing tests before implementation
- **[[Unit Testing]]** - Function-level validation
- **[[Integration Testing]]** - Component interaction testing

## 🎯 Testing Checklist

### **Before Implementation**
- [ ] Define test requirements for each REQ
- [ ] Identify edge cases and error conditions
- [ ] Plan test data and helper functions
- [ ] Set up test environment and dependencies

### **During Implementation**
- [ ] Write tests for each public function
- [ ] Test both success and error paths
- [ ] Validate ownership and borrowing rules
- [ ] Test performance characteristics

### **Before Release**
- [ ] Run full test suite
- [ ] Validate test coverage metrics
- [ ] Run integration tests with real data
- [ ] Performance regression testing

---

*Tags: #testing #unit-testing #integration-testing #requirement-testing #v-cycle #doctests #error-testing #performance-testing*
*Links: [[zettel-index]] | [[rust-book-ch9-12-review]] | [[REQ-1 Test Strategy]] | [[V-Cycle Methodology]] | [[Debugging Lessons]] | [[../rust_book/Ch11/CHAPTER_COMPLETE]] | [[daily-study/Day05]] | [[daily-study/Day33]] | [[daily-study/Day34]] | [[daily-study/Day35]] | [[Error Handling Patterns]] | [[TDD (Test-Driven Development)]] | [[Unit Testing]] | [[Integration Testing]]*
