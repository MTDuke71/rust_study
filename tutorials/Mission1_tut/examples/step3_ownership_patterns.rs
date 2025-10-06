// Mission1_tut Step 3: Ownership and Move Semantics  
// Master Rust ownership patterns in stack operations

fn main() {
    println!("=== Mission1_tut Step 3: Ownership and Move Semantics ===\n");
    
    // 1. Understanding Ownership in Stack Operations
    println!("1. Ownership Fundamentals:");
    explain_ownership_basics();
    
    // 2. Move Semantics in Push Operations
    println!("\n2. Move Semantics in Push:");
    demonstrate_move_semantics();
    
    // 3. Borrowing with Peek Operations  
    println!("\n3. Borrowing with Peek:");
    demonstrate_borrowing_patterns();
    
    // 4. Mutable vs Immutable References
    println!("\n4. Mutable vs Immutable References:");
    demonstrate_reference_types();
    
    // 5. Common Ownership Patterns
    println!("\n5. Common Ownership Patterns:");
    demonstrate_ownership_patterns();
    
    self_assessment();
}

/// Our generic stack from Step 2, now with focus on ownership
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
    
    /// push takes ownership of the item (move semantics)
    fn push(&mut self, item: T) {
        self.items.push(item);
        // item is moved into the Vec, caller loses ownership
    }
    
    /// pop returns ownership of the item back to caller
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
        // Returns ownership of the popped item (or None if empty)
    }
    
    /// peek borrows the top item immutably
    fn peek(&self) -> Option<&T> {
        self.items.last()
        // Returns a reference (&T), not ownership
        // Multiple immutable borrows are allowed simultaneously
    }
    
    /// peek_mut borrows the top item mutably
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.items.last_mut()
        // Returns a mutable reference (&mut T)
        // Only one mutable borrow allowed at a time
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
}

fn explain_ownership_basics() {
    println!("   Rust Ownership Rules:");
    println!("   1. Each value has exactly one owner at a time");
    println!("   2. When owner goes out of scope, value is dropped");
    println!("   3. Values can be moved (ownership transferred) or borrowed (temporary access)");
    
    println!("\n   How This Applies to Stacks:");
    println!("   • push(item) - Stack takes ownership of item");
    println!("   • pop() - Stack gives ownership back to caller");
    println!("   • peek() - Stack lends temporary access (borrow)");
    
    println!("\n   Memory Safety Benefits:");
    println!("   ✅ No dangling pointers - references always valid");
    println!("   ✅ No double-free errors - each value dropped exactly once");
    println!("   ✅ No data races - borrowing rules prevent concurrent modification");
}

fn demonstrate_move_semantics() {
    println!("   Move Semantics Example:");
    
    let mut stack: Stack<String> = Stack::new();
    
    // Create some strings
    let greeting = String::from("Hello");
    let target = String::from("World");
    
    println!("   Before push:");
    println!("   greeting = {:?}", greeting);
    println!("   target = {:?}", target);
    
    // Push moves ownership to the stack
    stack.push(greeting);  // greeting is moved, no longer accessible
    stack.push(target);    // target is moved, no longer accessible
    
    println!("   After push:");
    println!("   stack contains: {:?}", stack);
    
    // These lines would cause compile errors:
    // println!("greeting: {}", greeting);  // ❌ greeting was moved
    // println!("target: {}", target);      // ❌ target was moved
    
    println!("   ✅ Strings successfully moved into stack");
    
    // Pop returns ownership back to caller
    if let Some(popped) = stack.pop() {
        println!("   Popped value (now owned by caller): {:?}", popped);
        // popped variable now owns the String
    }
    
    demonstrate_copy_vs_move();
}

fn demonstrate_copy_vs_move() {
    println!("\n   Copy vs Move Types:");
    
    // Copy types (like i32) are copied, not moved
    let mut int_stack: Stack<i32> = Stack::new();
    let number = 42;
    
    int_stack.push(number);  // number is copied, not moved
    println!("   After push, number is still accessible: {}", number);
    
    // Move types (like String) are moved, not copied
    let mut string_stack: Stack<String> = Stack::new();
    let text = String::from("Rust");
    
    string_stack.push(text);  // text is moved, not copied
    // println!("text: {}", text);  // ❌ Would be compile error
    
    println!("   Copy types: i32, f64, bool, char, etc.");
    println!("   Move types: String, Vec<T>, custom structs (usually), etc.");
}

fn demonstrate_borrowing_patterns() {
    println!("   Immutable Borrowing with peek():");
    
    let mut stack: Stack<String> = Stack::new();
    stack.push("First".to_string());
    stack.push("Second".to_string());
    stack.push("Third".to_string());
    
    // Multiple immutable borrows are allowed
    let top_ref1 = stack.peek();
    let top_ref2 = stack.peek();
    let top_ref3 = stack.peek();
    
    println!("   Reference 1: {:?}", top_ref1);
    println!("   Reference 2: {:?}", top_ref2);
    println!("   Reference 3: {:?}", top_ref3);
    
    // All references point to the same value
    if let Some(top) = stack.peek() {
        println!("   Top item (borrowed): {}", top);
        println!("   String length: {}", top.len());
        
        // Can call methods on borrowed value
        println!("   Uppercase: {}", top.to_uppercase());
        
        // Original string in stack is unchanged
    }
    
    println!("   Stack still owns all items: {:?}", stack);
}

fn demonstrate_reference_types() {
    println!("   Mutable vs Immutable References:");
    
    let mut stack: Stack<String> = Stack::new();
    stack.push("Hello".to_string());
    stack.push("Rust".to_string());
    
    // Immutable reference - can read but not modify
    if let Some(top_ref) = stack.peek() {
        println!("   Immutable peek: {}", top_ref);
        println!("   Length: {}", top_ref.len());
        // top_ref.push_str(" World");  // ❌ Can't modify through &T
    }
    
    // Mutable reference - can read and modify
    if let Some(top_mut) = stack.peek_mut() {
        println!("   Before mutation: {}", top_mut);
        top_mut.push_str(" Programming");  // ✅ Can modify through &mut T
        println!("   After mutation: {}", top_mut);
    }
    
    println!("   Stack after modification: {:?}", stack);
    
    demonstrate_borrowing_rules();
}

fn demonstrate_borrowing_rules() {
    println!("\n   Borrowing Rules Demonstration:");
    
    let mut stack: Stack<i32> = Stack::new();
    stack.push(100);
    stack.push(200);
    
    // Rule 1: Multiple immutable references OK
    let ref1 = stack.peek();
    let ref2 = stack.peek();
    println!("   Multiple immutable refs: {:?}, {:?}", ref1, ref2);
    
    // Rule 2: Mutable reference excludes all others
    let mut_ref = stack.peek_mut();
    // let another_ref = stack.peek();  // ❌ Would be compile error
    println!("   Exclusive mutable ref: {:?}", mut_ref);
    
    // After mutable reference goes out of scope, immutable refs OK again
    drop(mut_ref);  // Explicitly drop to show scope
    let new_ref = stack.peek();
    println!("   Immutable ref after mut_ref dropped: {:?}", new_ref);
}

fn demonstrate_ownership_patterns() {
    println!("   Common Ownership Patterns:");
    
    // Pattern 1: Take ownership temporarily
    demonstrate_temporary_ownership();
    
    // Pattern 2: Clone when you need to keep original
    demonstrate_cloning_pattern();
    
    // Pattern 3: Borrow for read-only operations
    demonstrate_read_only_pattern();
}

fn demonstrate_temporary_ownership() {
    println!("\n   Pattern 1: Temporary Ownership");
    
    let mut stack: Stack<String> = Stack::new();
    
    // Build strings and move them into stack
    for i in 1..=3 {
        let item = format!("Item {}", i);  // Create owned String
        stack.push(item);                  // Move into stack
        // item is no longer accessible here
    }
    
    // Process items by taking ownership back
    while let Some(item) = stack.pop() {
        println!("   Processing owned item: {}", item);
        // item is dropped at end of loop iteration
    }
}

fn demonstrate_cloning_pattern() {
    println!("\n   Pattern 2: Clone When You Need Both");
    
    let original = String::from("Important Data");
    let mut stack: Stack<String> = Stack::new();
    
    // Clone if you need to keep the original
    stack.push(original.clone());  // Clone goes into stack
    
    println!("   Original still available: {}", original);
    println!("   Stack contains clone: {:?}", stack);
    
    // Both original and stack copy exist independently
}

fn demonstrate_read_only_pattern() {
    println!("\n   Pattern 3: Borrow for Read-Only Access");
    
    let stack: Stack<String> = {
        let mut temp_stack = Stack::new();
        temp_stack.push("Data 1".to_string());
        temp_stack.push("Data 2".to_string());
        temp_stack
    };
    
    // Function that only needs to read
    fn analyze_stack(s: &Stack<String>) {
        if let Some(top) = s.peek() {
            println!("   Analyzing top item: {} (length: {})", top, top.len());
        }
        println!("   Stack has {} items", s.len());
    }
    
    analyze_stack(&stack);  // Borrow, don't move
    println!("   Stack still available after analysis: {:?}", stack);
}

fn self_assessment() {
    println!("\n=== Self-Assessment Questions ===");
    println!("After completing Step 3, you should understand:");
    println!("1. What happens to ownership when you push() an item?");
    println!("2. Why does pop() return Option<T> instead of T?");
    println!("3. What's the difference between peek() and peek_mut()?");
    println!("4. When should you use borrowing vs moving?");
    println!("5. Why can't you have both mutable and immutable references?");
    
    println!("\n=== Common Mistakes to Avoid ===");
    println!("❌ Trying to use moved values after push()");
    println!("❌ Mixing mutable and immutable borrows");
    println!("❌ Forgetting that pop() might return None");
    println!("❌ Cloning unnecessarily (performance impact)");
    
    println!("\n=== Hands-On Challenge ===");
    println!("Try these exercises:");
    println!("1. Create a stack and push String values without cloning");
    println!("2. Use peek() to check the top without removing it");
    println!("3. Use peek_mut() to modify the top item in place");
    println!("4. Write a function that borrows a stack and analyzes it");
    
    println!("\n=== Next Step Preview ===");
    println!("Step 4 will cover error handling with Option<T>:");
    println!("• Why stack operations return Option<T>?");
    println!("• How to handle empty stack cases gracefully?");
    println!("• Pattern matching and error handling best practices?");
    println!("Run: cargo run --example step4_error_handling");
}