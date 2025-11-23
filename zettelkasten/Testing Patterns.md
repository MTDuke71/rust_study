# 🧪 Testing Patterns in Rust

**Comprehensive guide to testing strategies, patterns, and best practices for Rust applications**

---

## 🎯 **Testing Philosophy in Rust**

### **Rust's Testing Approach**
Rust's testing philosophy emphasizes:
- **Fast feedback cycles** - Tests run quickly and provide immediate results
- **Confidence in correctness** - Tests catch bugs before they reach production
- **Documentation through examples** - Tests serve as living documentation
- **Zero-cost testing** - Testing infrastructure doesn't impact production performance
- **Fearless refactoring** - Comprehensive tests enable safe code changes

### **Types of Testing in Rust**
```rust
// Unit tests - Test individual functions/modules in isolation
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn unit_test_example() {
        assert_eq!(add(2, 2), 4);
    }
}

// Integration tests - Test public API and component interaction
// tests/integration_test.rs
use my_crate::public_function;

#[test]
fn integration_test_example() {
    assert!(public_function().is_ok());
}

// Documentation tests - Ensure examples in docs work
/// Adds two numbers together
/// 
/// # Examples
/// 
/// ```
/// use my_crate::add;
/// assert_eq!(add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

---

## 📋 **Unit Testing Patterns**

### **Basic Test Structure**
```rust
#[cfg(test)]
mod tests {
    use super::*; // Import parent module items
    
    #[test]
    fn descriptive_test_name_should_describe_behavior() {
        // Arrange - Set up test data
        let input = "test data";
        let expected = "expected result";
        
        // Act - Execute the function under test
        let result = function_under_test(input);
        
        // Assert - Verify the result
        assert_eq!(result, expected);
    }
}
```

### **Test Organization Patterns**

**Pattern 1: Module-Level Tests**
```rust
// src/calculator.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    mod add_tests {
        use super::*;
        
        #[test]
        fn positive_numbers() {
            assert_eq!(add(2, 3), 5);
        }
        
        #[test]
        fn negative_numbers() {
            assert_eq!(add(-2, -3), -5);
        }
        
        #[test]
        fn mixed_signs() {
            assert_eq!(add(-2, 3), 1);
        }
    }
    
    mod multiply_tests {
        use super::*;
        
        #[test]
        fn basic_multiplication() {
            assert_eq!(multiply(3, 4), 12);
        }
        
        #[test]
        fn zero_multiplication() {
            assert_eq!(multiply(5, 0), 0);
        }
    }
}
```

**Pattern 2: Test-Driven Development (TDD)**
```rust
// Step 1: Write failing test
#[test]
fn fibonacci_should_return_correct_sequence() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(5), 5);
}

// Step 2: Write minimal implementation
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// Step 3: Refactor with tests as safety net
pub fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    
    let (mut a, mut b) = (0, 1);
    for _ in 2..=n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}
```

### **Assertion Patterns**

**Standard Assertions**
```rust
#[test]
fn assertion_examples() {
    // Equality testing
    assert_eq!(actual, expected);
    assert_ne!(actual, unexpected);
    
    // Boolean testing
    assert!(condition_is_true);
    assert!(!condition_is_false);
    
    // Custom messages
    assert_eq!(actual, expected, "Custom failure message: got {}, expected {}", actual, expected);
    
    // Floating point comparison (use approx crate)
    use approx::assert_relative_eq;
    assert_relative_eq!(3.14, std::f64::consts::PI, epsilon = 0.01);
}
```

**Error Testing Patterns**
```rust
use std::panic;

#[test]
fn should_panic_on_invalid_input() {
    // Test that function panics
    let result = panic::catch_unwind(|| {
        dangerous_function_that_should_panic();
    });
    assert!(result.is_err());
}

#[test]
#[should_panic(expected = "Invalid input")]
fn should_panic_with_specific_message() {
    dangerous_function("invalid");
}

#[test]
fn should_return_error_for_invalid_input() {
    // Prefer Result over panic for recoverable errors
    let result = fallible_function("invalid");
    assert!(result.is_err());
    
    match result {
        Err(e) => assert_eq!(e.to_string(), "Expected error message"),
        Ok(_) => panic!("Expected error but got Ok"),
    }
}
```

---

## 🏗️ **Integration Testing Patterns**

### **File Organization**
```
project/
├── src/
│   ├── lib.rs
│   ├── calculator.rs
│   └── parser.rs
├── tests/
│   ├── integration_test.rs
│   ├── api_tests.rs
│   └── common/
│       └── mod.rs  # Shared test utilities
└── Cargo.toml
```

**Integration Test Example**
```rust
// tests/integration_test.rs
use my_crate::{Calculator, ParseError};

#[test]
fn full_workflow_integration() {
    let calc = Calculator::new();
    
    // Test complete user workflow
    let expression = "2 + 3 * 4";
    let tokens = calc.tokenize(expression).expect("Tokenization failed");
    let ast = calc.parse(tokens).expect("Parsing failed");
    let result = calc.evaluate(ast).expect("Evaluation failed");
    
    assert_eq!(result, 14);
}

#[test]
fn error_handling_integration() {
    let calc = Calculator::new();
    
    let result = calc.evaluate_expression("2 + + 3");
    match result {
        Err(ParseError::InvalidSyntax { position, .. }) => {
            assert_eq!(position, 4); // Error at second '+'
        },
        _ => panic!("Expected ParseError::InvalidSyntax"),
    }
}
```

### **Common Utilities Pattern**
```rust
// tests/common/mod.rs
use my_crate::*;

pub struct TestFixture {
    pub calculator: Calculator,
    pub sample_data: Vec<TestCase>,
}

pub struct TestCase {
    pub input: &'static str,
    pub expected: i32,
}

impl TestFixture {
    pub fn new() -> Self {
        Self {
            calculator: Calculator::new(),
            sample_data: vec![
                TestCase { input: "1 + 1", expected: 2 },
                TestCase { input: "2 * 3", expected: 6 },
                TestCase { input: "10 / 2", expected: 5 },
            ],
        }
    }
    
    pub fn run_test_cases(&self) {
        for case in &self.sample_data {
            let result = self.calculator.evaluate_expression(case.input)
                .expect(&format!("Failed to evaluate: {}", case.input));
            assert_eq!(result, case.expected, 
                "Test case failed: {} should equal {}", case.input, case.expected);
        }
    }
}

// tests/calculator_tests.rs
mod common;
use common::TestFixture;

#[test]
fn basic_operations() {
    let fixture = TestFixture::new();
    fixture.run_test_cases();
}
```

---

## 📚 **Documentation Testing Patterns**

### **Effective Doc Tests**
```rust
/// Calculates the factorial of a number.
/// 
/// # Arguments
/// 
/// * `n` - A non-negative integer
/// 
/// # Returns
/// 
/// The factorial of `n`
/// 
/// # Panics
/// 
/// Panics if `n` is greater than 20 (to prevent overflow)
/// 
/// # Examples
/// 
/// Basic usage:
/// 
/// ```
/// use my_crate::factorial;
/// 
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(1), 1);
/// assert_eq!(factorial(5), 120);
/// ```
/// 
/// Error cases:
/// 
/// ```should_panic
/// use my_crate::factorial;
/// 
/// factorial(25); // Panics due to overflow protection
/// ```
/// 
/// You can also use it in more complex scenarios:
/// 
/// ```
/// use my_crate::factorial;
/// 
/// let numbers = vec![1, 2, 3, 4, 5];
/// let factorials: Vec<u64> = numbers.iter()
///     .map(|&n| factorial(n))
///     .collect();
/// 
/// assert_eq!(factorials, vec![1, 2, 6, 24, 120]);
/// ```
pub fn factorial(n: u8) -> u64 {
    if n > 20 {
        panic!("Input too large, would overflow");
    }
    
    match n {
        0 | 1 => 1,
        _ => (2..=n as u64).product(),
    }
}
```

### **Doc Test Attributes**
```rust
/// ```ignore
/// // This test is ignored (won't run during `cargo test`)
/// expensive_operation_that_takes_too_long();
/// ```
/// 
/// ```no_run
/// // This compiles but doesn't run (useful for examples that need setup)
/// use std::net::TcpListener;
/// let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
/// ```
/// 
/// ```compile_fail
/// // This test should fail to compile
/// let x: i32 = "not a number"; // Type error
/// ```
/// 
/// ```rust,edition2018
/// // Specify Rust edition
/// async fn example() { }
/// ```
pub fn documented_function() {}
```

---

## 🎛️ **Advanced Testing Patterns**

### **Property-Based Testing**
```rust
use proptest::prelude::*;

// Property: reversing a vector twice should give original
proptest! {
    #[test]
    fn reverse_reverse_is_identity(mut vec: Vec<i32>) {
        let original = vec.clone();
        vec.reverse();
        vec.reverse();
        prop_assert_eq!(vec, original);
    }
    
    #[test]
    fn addition_is_commutative(a: i32, b: i32) {
        // Skip cases that would overflow
        prop_assume!(a.checked_add(b).is_some());
        prop_assert_eq!(a + b, b + a);
    }
}

// Custom generators
fn small_positive_numbers() -> impl Strategy<Value = i32> {
    1..=100i32
}

proptest! {
    #[test]
    fn factorial_is_increasing(n in small_positive_numbers()) {
        prop_assume!(n < 20); // Prevent overflow
        if n > 1 {
            prop_assert!(factorial(n) > factorial(n - 1));
        }
    }
}
```

### **Mock and Stub Patterns**
```rust
use std::collections::HashMap;

// Trait for dependency injection
trait DataStore {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: String, value: String);
}

// Production implementation
struct DatabaseStore {
    connection: DatabaseConnection,
}

impl DataStore for DatabaseStore {
    fn get(&self, key: &str) -> Option<String> {
        self.connection.query(key)
    }
    
    fn set(&mut self, key: String, value: String) {
        self.connection.insert(key, value);
    }
}

// Test double (mock/stub)
struct MockDataStore {
    data: HashMap<String, String>,
    get_calls: Vec<String>,
}

impl MockDataStore {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            get_calls: Vec::new(),
        }
    }
    
    fn verify_get_called_with(&self, key: &str) -> bool {
        self.get_calls.contains(&key.to_string())
    }
}

impl DataStore for MockDataStore {
    fn get(&self, key: &str) -> Option<String> {
        self.get_calls.push(key.to_string());
        self.data.get(key).cloned()
    }
    
    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
}

// System under test
struct UserService<T: DataStore> {
    store: T,
}

impl<T: DataStore> UserService<T> {
    fn new(store: T) -> Self {
        Self { store }
    }
    
    fn get_user_name(&self, id: &str) -> Option<String> {
        self.store.get(&format!("user:{}", id))
    }
}

// Test using mock
#[test]
fn user_service_retrieves_correct_user() {
    let mut mock_store = MockDataStore::new();
    mock_store.data.insert("user:123".to_string(), "Alice".to_string());
    
    let service = UserService::new(mock_store);
    let result = service.get_user_name("123");
    
    assert_eq!(result, Some("Alice".to_string()));
    assert!(service.store.verify_get_called_with("user:123"));
}
```

### **Performance Testing Patterns**
```rust
use std::time::{Duration, Instant};

#[test]
fn performance_should_be_under_threshold() {
    let start = Instant::now();
    
    // Operation under test
    let result = expensive_computation(1000);
    
    let duration = start.elapsed();
    
    // Verify result correctness
    assert_eq!(result, expected_value);
    
    // Verify performance constraint
    assert!(duration < Duration::from_millis(100), 
        "Operation took too long: {:?}", duration);
}

// Benchmark-style test
#[test]
fn algorithm_comparison() {
    let data = generate_test_data(10000);
    
    let start = Instant::now();
    let result1 = algorithm_a(&data);
    let time_a = start.elapsed();
    
    let start = Instant::now();
    let result2 = algorithm_b(&data);
    let time_b = start.elapsed();
    
    assert_eq!(result1, result2); // Same correctness
    
    // Log performance for manual analysis
    println!("Algorithm A: {:?}, Algorithm B: {:?}", time_a, time_b);
}
```

---

## 🏆 **Mission-Specific Testing Patterns**

### **Data Structure Testing**
```rust
// Testing Stack implementation (Mission 1)
#[cfg(test)]
mod stack_tests {
    use super::*;
    
    #[test]
    fn new_stack_should_be_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }
    
    #[test]
    fn push_should_add_elements() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        
        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 2);
    }
    
    #[test]
    fn pop_should_return_lifo_order() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
    
    #[test]
    fn peek_should_not_modify_stack() {
        let mut stack = Stack::new();
        stack.push(42);
        
        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.len(), 1); // Still has element
        
        assert_eq!(stack.pop(), Some(42)); // Can still pop
    }
}
```

### **Algorithm Testing**
```rust
// Testing Binary Search (Mission 3)
#[cfg(test)]
mod binary_search_tests {
    use super::*;
    
    #[test]
    fn empty_array_should_return_none() {
        let arr: Vec<i32> = vec![];
        assert_eq!(binary_search(&arr, 5), None);
    }
    
    #[test]
    fn single_element_found() {
        let arr = vec![42];
        assert_eq!(binary_search(&arr, 42), Some(0));
    }
    
    #[test]
    fn single_element_not_found() {
        let arr = vec![42];
        assert_eq!(binary_search(&arr, 24), None);
    }
    
    #[test]
    fn multiple_elements_all_positions() {
        let arr = vec![1, 3, 5, 7, 9, 11];
        
        // Test finding each element
        assert_eq!(binary_search(&arr, 1), Some(0));  // First
        assert_eq!(binary_search(&arr, 5), Some(2));  // Middle
        assert_eq!(binary_search(&arr, 11), Some(5)); // Last
        
        // Test not found cases
        assert_eq!(binary_search(&arr, 0), None);   // Before first
        assert_eq!(binary_search(&arr, 6), None);   // Between elements
        assert_eq!(binary_search(&arr, 15), None);  // After last
    }
    
    #[test]
    fn large_array_performance() {
        let arr: Vec<i32> = (0..1_000_000).map(|i| i * 2).collect(); // Even numbers
        
        let start = Instant::now();
        let result = binary_search(&arr, 42);
        let duration = start.elapsed();
        
        assert_eq!(result, Some(21)); // 42 / 2 = 21
        assert!(duration < Duration::from_millis(1), "Search too slow: {:?}", duration);
    }
}
```

### **Hash Collection Testing (Mission 5)**
```rust
#[cfg(test)]
mod hashmap_tests {
    use super::*;
    
    #[test]
    fn insert_and_get() {
        let mut map = HashMap::new();
        
        assert_eq!(map.insert("key1", "value1"), None);
        assert_eq!(map.get("key1"), Some(&"value1"));
        
        // Update existing key
        assert_eq!(map.insert("key1", "new_value"), Some("value1"));
        assert_eq!(map.get("key1"), Some(&"new_value"));
    }
    
    #[test]
    fn collision_handling() {
        let mut map = HashMap::with_capacity(4); // Force collisions
        
        // Insert multiple items that might collide
        for i in 0..10 {
            map.insert(format!("key{}", i), format!("value{}", i));
        }
        
        // Verify all items are retrievable
        for i in 0..10 {
            assert_eq!(map.get(&format!("key{}", i)), Some(&format!("value{}", i)));
        }
    }
    
    #[test]
    fn resize_behavior() {
        let mut map = HashMap::with_capacity(2);
        let initial_capacity = map.capacity();
        
        // Fill beyond initial capacity
        for i in 0..10 {
            map.insert(i, i * i);
        }
        
        // Should have resized
        assert!(map.capacity() > initial_capacity);
        
        // All items still accessible
        for i in 0..10 {
            assert_eq!(map.get(&i), Some(&(i * i)));
        }
    }
}
```

---

## 🔧 **Testing Tools and Utilities**

### **Custom Assert Macros**
```rust
// Create domain-specific assertion macros
macro_rules! assert_stack_state {
    ($stack:expr, empty) => {
        assert!($stack.is_empty(), "Stack should be empty");
        assert_eq!($stack.len(), 0, "Stack length should be 0");
    };
    
    ($stack:expr, len = $expected:expr) => {
        assert_eq!($stack.len(), $expected, "Stack length mismatch");
        assert!(!$stack.is_empty(), "Stack should not be empty");
    };
    
    ($stack:expr, top = $expected:expr) => {
        assert_eq!($stack.peek(), Some(&$expected), "Stack top mismatch");
    };
}

#[test]
fn macro_usage_example() {
    let mut stack = Stack::new();
    assert_stack_state!(stack, empty);
    
    stack.push(42);
    assert_stack_state!(stack, len = 1);
    assert_stack_state!(stack, top = 42);
}
```

### **Test Helper Functions**
```rust
// Common test setup functions
fn create_sample_data() -> Vec<TestItem> {
    vec![
        TestItem::new("item1", 10),
        TestItem::new("item2", 20),
        TestItem::new("item3", 30),
    ]
}

fn assert_sorted<T: Ord>(slice: &[T]) {
    for window in slice.windows(2) {
        assert!(window[0] <= window[1], "Array not sorted: {:?}", slice);
    }
}

fn create_large_dataset(size: usize) -> Vec<i32> {
    (0..size).map(|i| rand::random::<i32>() % 1000).collect()
}
```

### **Test Configuration**
```rust
// Cargo.toml test configuration
[profile.test]
opt-level = 0
debug = true
debug-assertions = true
overflow-checks = true

// Custom test features
[features]
integration-tests = []
performance-tests = []

// Conditional test compilation
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn basic_test() {
        // Always runs
    }
    
    #[test]
    #[cfg(feature = "integration-tests")]
    fn integration_test() {
        // Only runs with --features integration-tests
    }
    
    #[test]
    #[cfg(feature = "performance-tests")]
    fn performance_test() {
        // Only runs with --features performance-tests
    }
}
```

---

## 📊 **Testing Best Practices**

### **Test Naming Conventions**
```rust
// ✅ Good: Describes behavior and conditions
#[test]
fn push_on_empty_stack_should_increase_size_to_one() { }

#[test] 
fn pop_on_single_element_stack_should_return_element_and_become_empty() { }

#[test]
fn binary_search_on_sorted_array_with_target_present_should_return_index() { }

// ❌ Bad: Unclear what is being tested
#[test]
fn test_push() { }

#[test]
fn test_search() { }
```

### **Test Independence**
```rust
// ✅ Good: Each test is independent
#[test]
fn test_a() {
    let mut stack = Stack::new(); // Fresh instance
    stack.push(1);
    assert_eq!(stack.len(), 1);
}

#[test]
fn test_b() {
    let mut stack = Stack::new(); // Fresh instance
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.len(), 2);
}

// ❌ Bad: Tests depend on shared state
static mut GLOBAL_STACK: Option<Stack<i32>> = None;

#[test]
fn test_setup() {
    unsafe {
        GLOBAL_STACK = Some(Stack::new());
    }
}

#[test]
fn test_depends_on_setup() { // Fragile - depends on test order
    unsafe {
        GLOBAL_STACK.as_mut().unwrap().push(1);
    }
}
```

### **Error Message Quality**
```rust
#[test]
fn comprehensive_error_messages() {
    let result = complex_calculation(input);
    
    // ✅ Good: Descriptive failure message
    assert_eq!(result.len(), expected_len, 
        "Result length mismatch for input {:?}. Expected {} items, got {}. Result: {:?}",
        input, expected_len, result.len(), result);
    
    // ✅ Good: Custom assertion with context
    assert!(result.iter().all(|&x| x > 0), 
        "All results should be positive. Found negative values: {:?}",
        result.iter().filter(|&&x| x <= 0).collect::<Vec<_>>());
}
```

---

## 🔗 **Integration with Learning System**

### **Mission Integration**
- **[[mission-1]]** - Stack testing with ownership patterns
- **[[mission-2]]** - Queue testing with enum state verification  
- **[[mission-3]]** - Algorithm testing with property-based approaches
- **[[mission-5]]** - HashMap testing with collision and resize scenarios
- **[[mission-6]]** - Graph algorithm testing with complex state verification

### **Documentation Integration**
- **[[Documentation Standards]]** - Testing documentation requirements
- **[[API Design Patterns]]** - Testable API design principles
- **[[Rust Book MOC]]** - Chapter 11 testing foundations

### **Quality Assurance**
- **[[Quality Assurance]]** - Comprehensive QA processes including testing
- **[[V-Cycle Methodology]]** - Testing in requirements-driven development

### **AoC Applications**
- **[[AoC Patterns MOC]]** - Testing competitive programming solutions
- **Input validation testing** for robust AoC solutions
- **Performance testing** for algorithm optimization
- **Edge case testing** for contest problem correctness

---

## 📚 **External Resources**

### **Rust Testing Ecosystem**
- **[Rust Book Chapter 11](https://doc.rust-lang.org/book/ch11-00-testing.html)** - Official testing guide
- **[proptest](https://github.com/proptest-rs/proptest)** - Property-based testing
- **[mockall](https://github.com/asomers/mockall)** - Mock object library
- **[criterion](https://github.com/bheisler/criterion.rs)** - Benchmarking library

### **Testing Philosophy**
- **[Test-Driven Development](https://en.wikipedia.org/wiki/Test-driven_development)** - Red-Green-Refactor cycle
- **[Behavior-Driven Development](https://en.wikipedia.org/wiki/Behavior-driven_development)** - Specification by example
- **[Property-Based Testing](https://hypothesis.works/articles/what-is-property-based-testing/)** - Testing with generated inputs

---

*Tags: #testing #patterns #rust #unit-tests #integration-tests #tdd #quality-assurance #best-practices #mission-testing #documentation-tests*
*Links: [[zettel-index]] | [[Quality Assurance]] | [[Documentation Standards]] | [[API Design Patterns]] | [[Rust Book MOC]] | [[V-Cycle Methodology]] | [[mission-1]] | [[AoC Patterns MOC]]*