use mission1::Stack;

fn demonstrate_complete_stack_usage() {
    println!("=== COMPLETE STACK WALKTHROUGH ===");
    
    // 1. Create stack (constructor)
    let mut stack: Stack<String> = Stack::new();
    println!("1. Created empty stack");
    println!("   Empty? {}", stack.is_empty());
    println!("   Length: {}", stack.len());
    
    // 2. Push items (ownership transfer)
    println!("\n2. Pushing items (ownership transfer):");
    let item1 = String::from("First");
    let item2 = String::from("Second");
    let item3 = String::from("Third");
    
    println!("   Pushing: {}", item1);
    stack.push(item1); // item1 is moved
    
    println!("   Pushing: {}", item2);
    stack.push(item2); // item2 is moved
    
    println!("   Pushing: {}", item3);
    stack.push(item3); // item3 is moved
    
    println!("   Stack: {:?}", stack);
    println!("   Length: {}", stack.len());
    
    // 3. Peek (immutable borrowing)
    println!("\n3. Peek (immutable borrowing):");
    if let Some(top) = stack.peek() {
        println!("   Top item: {}", top);
        println!("   Stack still has {} items", stack.len());
    }
    
    // 4. Peek mut (mutable borrowing)
    println!("\n4. Peek mut (mutable borrowing):");
    if let Some(top_mut) = stack.peek_mut() {
        println!("   Original top: {}", top_mut);
        *top_mut = String::from("Modified Third");
        println!("   Modified top: {}", top_mut);
    }
    println!("   Stack after modification: {:?}", stack);
    
    // 5. Pop items (ownership transfer back)
    println!("\n5. Pop items (ownership transfer back):");
    while !stack.is_empty() {
        let popped = stack.pop().unwrap();
        println!("   Popped: {} (Length now: {})", popped, stack.len());
    }
    
    // 6. Try to pop from empty stack
    println!("\n6. Pop from empty stack:");
    match stack.pop() {
        Some(item) => println!("   Got: {}", item),
        None => println!("   Stack is empty - got None"),
    }
    
    println!("\nFinal state:");
    println!("   Empty? {}", stack.is_empty());
    println!("   Length: {}", stack.len());
}

fn demonstrate_borrowing_rules_violations() {
    println!("\n=== BORROWING RULES (COMPILE ERRORS) ===");
    
    let mut stack = Stack::new();
    stack.push(42);
    
    println!("These would cause compile errors:");
    println!("1. Using value after moving to push()");
    println!("2. Multiple mutable borrows with peek_mut()");
    println!("3. Mixing immutable and mutable borrows");
    println!("4. Using popped value from stack's perspective");
    
    // Example of what works:
    {
        let peek_ref = stack.peek().unwrap();
        println!("Current top: {}", peek_ref);
        // peek_ref goes out of scope here
    }
    
    // Now we can mutate
    stack.push(99);
    println!("Successfully pushed 99: {:?}", stack);
}

fn main() {
    demonstrate_complete_stack_usage();
    demonstrate_borrowing_rules_violations();
}