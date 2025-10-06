// Mission1_tut Step 4: Error Handling Strategies
// Learn to handle empty stack cases gracefully with Option<T>

fn main() {
    println!("=== Mission1_tut Step 4: Error Handling Strategies ===\n");
    
    // 1. Why Stack Operations Return Option<T>
    println!("1. Why Option<T> for Stack Operations:");
    explain_option_necessity();
    
    // 2. Pattern Matching with Option
    println!("\n2. Pattern Matching with Option:");
    demonstrate_pattern_matching();
    
    // 3. Safe Stack Operations
    println!("\n3. Safe Stack Operations:");
    demonstrate_safe_operations();
    
    // 4. Error Handling Patterns
    println!("\n4. Error Handling Patterns:");
    demonstrate_error_patterns();
    
    // 5. Defensive Programming
    println!("\n5. Defensive Programming:");
    demonstrate_defensive_programming();
    
    self_assessment();
}

/// Enhanced stack with better error handling examples
#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            items: Vec::new(),
        }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    /// Returns Option<T> because stack might be empty
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    /// Returns Option<&T> because stack might be empty
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    /// Returns Option<&mut T> because stack might be empty
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.items.last_mut()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
    
    /// Safe pop that doesn't panic on empty stack
    fn try_pop(&mut self) -> Result<T, &'static str> {
        match self.items.pop() {
            Some(item) => Ok(item),
            None => Err("Cannot pop from empty stack"),
        }
    }
}

fn explain_option_necessity() {
    println!("   Why not just return T directly?");
    println!("   Problem: What should pop() return when stack is empty?");
    println!("   • Return default value? - What if there's no meaningful default?");
    println!("   • Panic? - Crashes the program, not graceful");
    println!("   • Return null pointer? - Rust doesn't have null pointers!");
    
    println!("\n   Option<T> Solution:");
    println!("   • Some(value) - Operation succeeded, here's the value");
    println!("   • None - Operation failed gracefully, no value available");
    println!("   • Forces caller to handle both cases explicitly");
    println!("   • Compiler prevents forgetting to check for errors");
    
    println!("\n   Benefits of Option<T>:");
    println!("   ✅ No null pointer exceptions - impossible in Rust");
    println!("   ✅ Explicit error handling - can't ignore failure cases");
    println!("   ✅ Composable - can chain operations with map(), and_then(), etc.");
    println!("   ✅ Zero-cost abstraction - compiles to efficient code");
}

fn demonstrate_pattern_matching() {
    println!("   Pattern Matching with match:");
    
    let mut stack: Stack<i32> = Stack::new();
    
    // Try popping from empty stack
    match stack.pop() {
        Some(value) => println!("   Popped: {}", value),
        None => println!("   Stack was empty - no value to pop"),
    }
    
    // Add some items and try again
    stack.push(10);
    stack.push(20);
    
    match stack.pop() {
        Some(value) => println!("   Successfully popped: {}", value),
        None => println!("   This won't happen since we just added items"),
    }
    
    demonstrate_if_let_pattern();
}

fn demonstrate_if_let_pattern() {
    println!("\n   if let Pattern (concise for single case):");
    
    let mut stack: Stack<String> = Stack::new();
    stack.push("Hello".to_string());
    stack.push("World".to_string());
    
    // if let for when you only care about success case
    if let Some(top) = stack.peek() {
        println!("   Top item exists: {}", top);
    }
    
    // while let for processing until empty
    println!("   Processing all items:");
    while let Some(item) = stack.pop() {
        println!("   Processing: {}", item);
    }
    
    // Try one more pop (should be empty now)
    if let Some(item) = stack.pop() {
        println!("   Got item: {}", item);
    } else {
        println!("   Stack is now empty");
    }
}

fn demonstrate_safe_operations() {
    println!("   Safe Stack Operations Examples:");
    
    let mut stack: Stack<i32> = Stack::new();
    
    // Safe peek before popping
    fn safe_pop_twice(stack: &mut Stack<i32>) -> (Option<i32>, Option<i32>) {
        let first = stack.pop();
        let second = stack.pop();
        (first, second)
    }
    
    // Test with empty stack
    println!("   Testing safe operations on empty stack:");
    let (first, second) = safe_pop_twice(&mut stack);
    println!("   First pop: {:?}, Second pop: {:?}", first, second);
    
    // Test with one item
    stack.push(42);
    println!("\n   Testing with one item:");
    let (first, second) = safe_pop_twice(&mut stack);
    println!("   First pop: {:?}, Second pop: {:?}", first, second);
    
    // Test with multiple items
    stack.push(100);
    stack.push(200);
    println!("\n   Testing with multiple items:");
    let (first, second) = safe_pop_twice(&mut stack);
    println!("   First pop: {:?}, Second pop: {:?}", first, second);
}

fn demonstrate_error_patterns() {
    println!("   Common Error Handling Patterns:");
    
    let mut stack: Stack<String> = Stack::new();
    stack.push("Important Data".to_string());
    
    // Pattern 1: unwrap() - use only when you're certain
    println!("\n   Pattern 1: unwrap() (use carefully!):");
    let item = stack.pop().unwrap();  // Panics if None
    println!("   Unwrapped item: {}", item);
    
    // Pattern 2: unwrap_or() - provide default value
    println!("\n   Pattern 2: unwrap_or() (safe default):");
    let item = stack.pop().unwrap_or("Default".to_string());
    println!("   Item or default: {}", item);
    
    // Pattern 3: unwrap_or_else() - compute default lazily
    println!("\n   Pattern 3: unwrap_or_else() (lazy default):");
    let item = stack.pop().unwrap_or_else(|| {
        println!("   Computing default value...");
        "Computed Default".to_string()
    });
    println!("   Item or computed default: {}", item);
    
    // Pattern 4: expect() - panic with custom message
    stack.push("Test".to_string());
    println!("\n   Pattern 4: expect() (descriptive panic):");
    let item = stack.pop().expect("Stack should not be empty here");
    println!("   Expected item: {}", item);
    
    // Pattern 5: map() - transform success value
    stack.push("hello".to_string());
    println!("\n   Pattern 5: map() (transform value):");
    let uppercase = stack.peek().map(|s| s.to_uppercase());
    println!("   Uppercase option: {:?}", uppercase);
    
    demonstrate_chaining_operations();
}

fn demonstrate_chaining_operations() {
    println!("\n   Chaining Option Operations:");
    
    let mut stack: Stack<String> = Stack::new();
    stack.push("  hello world  ".to_string());
    
    // Chain multiple operations
    let processed = stack.peek()
        .map(|s| s.trim())                    // Remove whitespace
        .map(|s| s.to_uppercase())            // Convert to uppercase
        .map(|s| format!(">>> {} <<<", s));   // Add decorations
    
    println!("   Chained result: {:?}", processed);
    
    // and_then() for operations that return Option
    let length = stack.peek()
        .and_then(|s| {
            if s.trim().is_empty() {
                None  // Reject empty strings
            } else {
                Some(s.trim().len())  // Return length of trimmed string
            }
        });
    
    println!("   Length of non-empty string: {:?}", length);
}

fn demonstrate_defensive_programming() {
    println!("   Defensive Programming Techniques:");
    
    // Always check before operating
    fn safe_stack_processor<T>(stack: &mut Stack<T>) -> usize
    where
        T: std::fmt::Debug,
    {
        let mut processed_count = 0;
        
        while !stack.is_empty() {
            if let Some(item) = stack.pop() {
                println!("   Processing item: {:?}", item);
                processed_count += 1;
            } else {
                // This should never happen given is_empty() check,
                // but defensive programming handles the impossible case
                println!("   Warning: Unexpected empty stack!");
                break;
            }
        }
        
        processed_count
    }
    
    let mut stack: Stack<i32> = Stack::new();
    for i in 1..=3 {
        stack.push(i);
    }
    
    let count = safe_stack_processor(&mut stack);
    println!("   Processed {} items safely", count);
    
    demonstrate_error_propagation();
}

fn demonstrate_error_propagation() {
    println!("\n   Error Propagation Patterns:");
    
    // Function that might fail
    fn process_stack_top(stack: &mut Stack<i32>) -> Option<i32> {
        let top = stack.pop()?;  // Early return if None
        let doubled = top * 2;
        Some(doubled)
    }
    
    let mut stack: Stack<i32> = Stack::new();
    
    // Test with empty stack
    match process_stack_top(&mut stack) {
        Some(result) => println!("   Processed result: {}", result),
        None => println!("   Could not process - stack was empty"),
    }
    
    // Test with value
    stack.push(21);
    match process_stack_top(&mut stack) {
        Some(result) => println!("   Processed result: {}", result),
        None => println!("   Could not process - stack was empty"),
    }
    
    demonstrate_result_type();
}

fn demonstrate_result_type() {
    println!("\n   Using Result<T, E> for More Detailed Errors:");
    
    #[derive(Debug)]
    enum StackError {
        Empty,
        InvalidOperation,
    }
    
    impl<T> Stack<T> {
        fn pop_result(&mut self) -> Result<T, StackError> {
            self.items.pop().ok_or(StackError::Empty)
        }
        
        fn peek_result(&self) -> Result<&T, StackError> {
            self.items.last().ok_or(StackError::Empty)
        }
    }
    
    let mut stack: Stack<i32> = Stack::new();
    
    match stack.pop_result() {
        Ok(value) => println!("   Popped: {}", value),
        Err(e) => println!("   Error: {:?}", e),
    }
    
    stack.push(42);
    match stack.pop_result() {
        Ok(value) => println!("   Successfully popped: {}", value),
        Err(e) => println!("   Error: {:?}", e),
    }
}

fn self_assessment() {
    println!("\n=== Self-Assessment Questions ===");
    println!("After completing Step 4, you should understand:");
    println!("1. Why do stack operations return Option<T> instead of T?");
    println!("2. How do you handle None values safely?");
    println!("3. What's the difference between unwrap(), unwrap_or(), and expect()?");
    println!("4. When should you use match vs if let vs while let?");
    println!("5. How do you chain Option operations with map() and and_then()?");
    
    println!("\n=== Error Handling Best Practices ===");
    println!("✅ Always handle Option/Result explicitly");
    println!("✅ Use unwrap() only when you can prove it's safe");
    println!("✅ Provide meaningful error messages with expect()");
    println!("✅ Consider using unwrap_or() for reasonable defaults");
    println!("✅ Chain operations to avoid nested match statements");
    
    println!("\n=== Common Patterns ===");
    println!("• if let Some(x) = optional {{ ... }}  - Handle success case only");
    println!("• while let Some(x) = iter.next() {{ ... }}  - Process until None");
    println!("• optional.map(|x| transform(x))  - Transform success values");
    println!("• optional.unwrap_or(default)  - Provide fallback value");
    
    println!("\n=== Next Step Preview ===");
    println!("Step 5 will explore dynamic growth and performance characteristics:");
    println!("• How does Vec handle memory allocation?");
    println!("• What is amortized O(1) complexity?");
    println!("• How to optimize stack performance for different use cases?");
    println!("Run: cargo run --example step5_dynamic_growth");
}