# 🔄 TDD (Test-Driven Development)

**Comprehensive guide to test-first development methodology, red-green-refactor cycle, and writing tests before implementation in Rust**

## 🎯 Overview

Test-Driven Development (TDD) is a software development methodology where tests are written before the implementation code. The TDD cycle follows three phases: Red (write a failing test), Green (write minimal code to pass the test), and Refactor (improve the code while keeping tests passing). This approach ensures high test coverage, better design, and more maintainable code.

## 🔄 The TDD Cycle

### **Red-Green-Refactor Process**
```rust
// 1. RED: Write a failing test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_push() {
        let mut stack = Stack::new();
        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
        // This test will fail because Stack doesn't exist yet
    }
}

// 2. GREEN: Write minimal code to make test pass
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}

// 3. REFACTOR: Improve the implementation
impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    // Add additional methods as needed
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    pub fn len(&self) -> usize {
        self.items.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
```

## 🎯 TDD in Practice

### **Mission1: Stack Implementation**
```rust
// Step 1: RED - Write failing tests for REQ-1
#[cfg(test)]
mod tests {
    use super::*;

    #[test] // REQ-1: Generic Support
    fn req1_generic_support() {
        // Test with different types
        let mut int_stack = Stack::<i32>::new();
        int_stack.push(42);
        assert_eq!(int_stack.pop(), Some(42));
        
        let mut string_stack = Stack::<String>::new();
        string_stack.push("Hello".to_string());
        assert_eq!(string_stack.pop(), Some("Hello".to_string()));
    }
}

// Step 2: GREEN - Minimal implementation
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}

// Step 3: RED - Add more failing tests
#[test] // REQ-2: Performance
fn req2_amortized_constant_time() {
    let mut stack = Stack::new();
    // Test that push operations are O(1) amortized
    for i in 0..1000 {
        stack.push(i);
    }
    assert_eq!(stack.len(), 1000);
}

#[test] // REQ-3: Ownership
fn req3_ownership_transfer() {
    let mut stack = Stack::new();
    let owned_string = String::from("owned");
    stack.push(owned_string);
    // owned_string is moved, cannot be used
    let popped = stack.pop().unwrap();
    assert_eq!(popped, "owned");
}

// Step 4: GREEN - Add minimal implementation
impl<T> Stack<T> {
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

// Step 5: REFACTOR - Improve implementation
impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        Self { items: Vec::with_capacity(capacity) }
    }
    
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    pub fn len(&self) -> usize {
        self.items.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
```

### **Mission2: Queue Implementation**
```rust
// Step 1: RED - Write failing tests for queue
#[cfg(test)]
mod tests {
    use super::*;

    #[test] // REQ-1: FIFO Queue
    fn req1_fifo_queue() {
        let mut queue = RingBufferQueue::with_capacity(3);
        queue.enqueue(1).unwrap();
        queue.enqueue(2).unwrap();
        
        assert_eq!(queue.dequeue(), Some(1)); // FIFO order
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), None);
    }
}

// Step 2: GREEN - Minimal implementation
pub struct RingBufferQueue<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
    count: usize,
}

impl<T> RingBufferQueue<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: vec![None; capacity],
            head: 0,
            tail: 0,
            count: 0,
        }
    }
    
    pub fn enqueue(&mut self, item: T) -> Result<(), QueueError> {
        if self.count >= self.buffer.len() {
            return Err(QueueError::Full);
        }
        
        self.buffer[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.buffer.len();
        self.count += 1;
        Ok(())
    }
    
    pub fn dequeue(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        
        let item = self.buffer[self.head].take();
        self.head = (self.head + 1) % self.buffer.len();
        self.count -= 1;
        item
    }
}

#[derive(Debug)]
pub enum QueueError {
    Full,
}

// Step 3: RED - Add more tests
#[test]
fn req2_circular_behavior() {
    let mut queue = RingBufferQueue::with_capacity(3);
    
    // Fill queue
    queue.enqueue(1).unwrap();
    queue.enqueue(2).unwrap();
    queue.enqueue(3).unwrap();
    
    // Dequeue one, making space
    assert_eq!(queue.dequeue(), Some(1));
    
    // Enqueue another - should wrap around
    queue.enqueue(4).unwrap();
    
    assert_eq!(queue.dequeue(), Some(2));
    assert_eq!(queue.dequeue(), Some(3));
    assert_eq!(queue.dequeue(), Some(4));
}

// Step 4: GREEN - Implementation already handles this
// Step 5: REFACTOR - Add helper methods and improve
impl<T> RingBufferQueue<T> {
    pub fn len(&self) -> usize {
        self.count
    }
    
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    
    pub fn is_full(&self) -> bool {
        self.count >= self.buffer.len()
    }
}
```

## 🧪 TDD Patterns

### **Test-First Function Design**
```rust
// 1. RED - Write test for function that doesn't exist
#[test]
fn test_calculate_fibonacci() {
    assert_eq!(calculate_fibonacci(0), 0);
    assert_eq!(calculate_fibonacci(1), 1);
    assert_eq!(calculate_fibonacci(2), 1);
    assert_eq!(calculate_fibonacci(3), 2);
    assert_eq!(calculate_fibonacci(10), 55);
}

// 2. GREEN - Write minimal implementation
fn calculate_fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2),
    }
}

// 3. RED - Add performance test
#[test]
fn test_fibonacci_performance() {
    let start = std::time::Instant::now();
    let result = calculate_fibonacci(40);
    let duration = start.elapsed();
    
    assert_eq!(result, 102334155);
    assert!(duration.as_millis() < 1000); // Should be fast
}

// 4. REFACTOR - Optimize implementation
fn calculate_fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    
    let mut prev = 0;
    let mut curr = 1;
    
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    
    curr
}
```

### **Error Handling TDD**
```rust
// 1. RED - Test error conditions
#[test]
fn test_parse_number_success() {
    assert_eq!(parse_number("42"), Ok(42));
    assert_eq!(parse_number("0"), Ok(0));
    assert_eq!(parse_number("-10"), Ok(-10));
}

#[test]
fn test_parse_number_errors() {
    assert!(parse_number("invalid").is_err());
    assert!(parse_number("").is_err());
    assert!(parse_number("3.14").is_err()); // Not an integer
}

#[test]
fn test_parse_number_overflow() {
    let large_number = format!("{}", i32::MAX as i64 + 1);
    assert!(parse_number(&large_number).is_err());
}

// 2. GREEN - Implement error handling
#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidFormat,
    Overflow,
}

fn parse_number(s: &str) -> Result<i32, ParseError> {
    if s.is_empty() {
        return Err(ParseError::InvalidFormat);
    }
    
    s.parse::<i32>()
        .map_err(|_| {
            if s.parse::<i64>().is_ok() {
                ParseError::Overflow
            } else {
                ParseError::InvalidFormat
            }
        })
}

// 3. REFACTOR - Improve error messages and handling
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidFormat => write!(f, "Invalid number format"),
            ParseError::Overflow => write!(f, "Number too large"),
        }
    }
}

impl std::error::Error for ParseError {}
```

## 🔄 TDD Workflow

### **Development Cycle**
```rust
// 1. Start with a failing test
#[test]
fn test_user_creation() {
    let user = User::new("Alice", "alice@example.com");
    assert_eq!(user.name(), "Alice");
    assert_eq!(user.email(), "alice@example.com");
    assert!(!user.is_verified());
}

// 2. Write minimal code to pass
pub struct User {
    name: String,
    email: String,
    verified: bool,
}

impl User {
    pub fn new(name: &str, email: &str) -> Self {
        Self {
            name: name.to_string(),
            email: email.to_string(),
            verified: false,
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn email(&self) -> &str {
        &self.email
    }
    
    pub fn is_verified(&self) -> bool {
        self.verified
    }
}

// 3. Add more tests (RED)
#[test]
fn test_user_verification() {
    let mut user = User::new("Alice", "alice@example.com");
    assert!(!user.is_verified());
    
    user.verify();
    assert!(user.is_verified());
}

#[test]
fn test_email_validation() {
    assert!(User::new("Alice", "valid@example.com").is_valid());
    assert!(!User::new("Alice", "invalid-email").is_valid());
}

// 4. Implement new features (GREEN)
impl User {
    pub fn verify(&mut self) {
        self.verified = true;
    }
    
    pub fn is_valid(&self) -> bool {
        self.email.contains('@')
    }
}

// 5. REFACTOR - Improve design
impl User {
    pub fn new(name: &str, email: &str) -> Result<Self, UserError> {
        if !email.contains('@') {
            return Err(UserError::InvalidEmail);
        }
        
        Ok(Self {
            name: name.to_string(),
            email: email.to_string(),
            verified: false,
        })
    }
    
    pub fn verify(&mut self) {
        self.verified = true;
    }
    
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty() && self.email.contains('@')
    }
}

#[derive(Debug)]
pub enum UserError {
    InvalidEmail,
}
```

## 🎯 TDD Best Practices

### **Writing Good Tests First**
```rust
// GOOD: Clear, focused test
#[test]
fn test_stack_push_increases_length() {
    let mut stack = Stack::new();
    assert_eq!(stack.len(), 0);
    
    stack.push(42);
    assert_eq!(stack.len(), 1);
    
    stack.push(24);
    assert_eq!(stack.len(), 2);
}

// BAD: Testing too many things at once
#[test]
fn test_stack_everything() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.len(), 2);
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.peek(), Some(&1));
    // Too many assertions in one test
}

// GOOD: Test one behavior per test
#[test]
fn test_stack_push_increases_length() {
    // Test only length behavior
}

#[test]
fn test_stack_pop_returns_last_element() {
    // Test only pop behavior
}

#[test]
fn test_stack_peek_does_not_remove_element() {
    // Test only peek behavior
}
```

### **Minimal Implementation**
```rust
// RED: Failing test
#[test]
fn test_calculator_add() {
    let calc = Calculator::new();
    assert_eq!(calc.add(2, 3), 5);
}

// GREEN: Minimal implementation
pub struct Calculator;

impl Calculator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b  // Minimal code to pass test
    }
}

// RED: Add more tests
#[test]
fn test_calculator_subtract() {
    let calc = Calculator::new();
    assert_eq!(calc.subtract(5, 3), 2);
}

// GREEN: Add minimal implementation
impl Calculator {
    pub fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b  // Minimal code to pass test
    }
}

// REFACTOR: Improve implementation
impl Calculator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    
    pub fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }
    
    pub fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }
    
    pub fn divide(&self, a: i32, b: i32) -> Result<i32, CalcError> {
        if b == 0 {
            Err(CalcError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }
}

#[derive(Debug)]
pub enum CalcError {
    DivisionByZero,
}
```

## 🔧 TDD with Mocking

### **Test-First with Dependencies**
```rust
// 1. RED - Write test with mock dependency
#[test]
fn test_user_service_get_user() {
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

// 2. GREEN - Implement with dependency
pub struct UserService {
    database: Box<dyn Database>,
}

impl UserService {
    pub fn new(database: impl Database + 'static) -> Self {
        Self {
            database: Box::new(database),
        }
    }
    
    pub fn get_user(&self, id: u32) -> Result<User, ServiceError> {
        self.database.get_user(id)
            .map_err(|e| ServiceError::DatabaseError(e))
    }
}

// 3. REFACTOR - Improve error handling and design
pub trait Database {
    fn get_user(&self, id: u32) -> Result<User, DatabaseError>;
}

#[derive(Debug)]
pub enum ServiceError {
    DatabaseError(DatabaseError),
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ServiceError::DatabaseError(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl std::error::Error for ServiceError {}
```

## 🔗 Related Concepts

### **Testing Integration**
- **[[Unit Testing]]** - Individual component testing
- **[[Integration Testing]]** - Component interaction testing
- **[[Testing Strategies]]** - Comprehensive testing approaches

### **Development Methodology**
- **[[V-Cycle Methodology]]** - Systematic development approach
- **[[REQ-1 Test Strategy]]** - Requirement-based testing
- **[[Debugging Lessons]]** - Using TDD for debugging

### **Mission Applications**
- **[[Mission1 Overview]]** - TDD for stack implementation
- **[[Mission2 Overview]]** - TDD for queue implementation
- **[[Mission4 Overview]]** - TDD for interior mutability patterns
- **[[Mission5 Overview]]** - TDD for HashMap implementation
- **[[Mission6 Overview]]** - TDD for grid algorithms

## 🎯 TDD Benefits

### **Code Quality**
- **High Test Coverage**: Tests written first ensure comprehensive coverage
- **Better Design**: TDD leads to more modular, loosely coupled code
- **Documentation**: Tests serve as executable documentation
- **Regression Prevention**: Existing functionality protected by tests

### **Development Process**
- **Faster Feedback**: Immediate feedback on code changes
- **Confidence**: Safe refactoring with test safety net
- **Requirements Clarity**: Tests clarify requirements before implementation
- **Reduced Debugging**: Fewer bugs through test-first approach

## 📋 TDD Checklist

### **Before Writing Code:**
- [ ] Write a failing test for the feature
- [ ] Ensure test fails for the right reason
- [ ] Run the test to confirm it's red

### **While Implementing:**
- [ ] Write minimal code to make test pass
- [ ] Run test to confirm it's green
- [ ] Add more tests for edge cases
- [ ] Refactor code while keeping tests green

### **After Implementation:**
- [ ] All tests pass
- [ ] Code is clean and well-designed
- [ ] No duplicate code
- [ ] Good test coverage

---

*Tags: #tdd #test-driven-development #red-green-refactor #test-first #development-methodology #v-cycle*
*Links: [[zettel-index]] | [[rust-book-ch9-12-review]] | [[Unit Testing]] | [[Integration Testing]] | [[Testing Strategies]] | [[V-Cycle Methodology]] | [[REQ-1 Test Strategy]] | [[Debugging Lessons]] | [[Mission1 Overview]] | [[Mission2 Overview]] | [[Mission4 Overview]] | [[Mission5 Overview]] | [[Mission6 Overview]]*
