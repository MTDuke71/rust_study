use mission1::Stack;

fn demonstrate_mutable_borrowing() {
    println!("=== PEEK_MUT: MUTABLE BORROWING ===");
    
    let mut stack = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.push(42);
    
    println!("Original stack: {:?}", stack);
    
    // Get a mutable reference to the top value
    {
        let mut_ref = stack.peek_mut().unwrap();
        println!("Original top value: {}", mut_ref);
        
        // We can modify the value through the mutable reference
        *mut_ref = 99;
        println!("Modified top value: {}", mut_ref);
        
        // While this mutable reference exists, we CANNOT:
        // 1. Get another mutable reference
        // let another_mut = stack.peek_mut(); // ERROR!
        
        // 2. Get an immutable reference  
        // let immut_ref = stack.peek(); // ERROR!
        
        // 3. Call any other methods that borrow the stack
        // stack.push(123); // ERROR!
        // let len = stack.len(); // ERROR!
        
        println!("Exclusive access - no other borrows allowed!");
        
    } // Mutable borrow ends here
    
    // Now we can borrow again
    println!("After mutable borrow ends:");
    println!("Modified stack: {:?}", stack);
    println!("Top value is now: {}", stack.peek().unwrap());
    
    // Now we can do other operations
    stack.push(200);
    println!("After pushing 200: {:?}", stack);
}

fn demonstrate_borrowing_rules() {
    println!("\n=== BORROWING RULES COMPARISON ===");
    
    let mut stack = Stack::new();
    stack.push("First".to_string());
    stack.push("Second".to_string());
    
    println!("Case 1: Multiple immutable borrows (OK)");
    {
        let ref1 = stack.peek().unwrap();
        let ref2 = stack.peek().unwrap();
        let ref3 = stack.peek().unwrap();
        
        println!("  ref1: {}", ref1);
        println!("  ref2: {}", ref2);
        println!("  ref3: {}", ref3);
        println!("  All immutable borrows coexist peacefully!");
    }
    
    println!("\nCase 2: One mutable borrow (OK)");
    {
        let mut_ref = stack.peek_mut().unwrap();
        println!("  Original: {}", mut_ref);
        *mut_ref = "Modified".to_string();
        println!("  Modified: {}", mut_ref);
        println!("  One mutable borrow, exclusive access!");
    }
    
    println!("\nCase 3: What's NOT allowed (would cause compile errors):");
    println!("  - Mutable + Immutable borrows together");
    println!("  - Multiple mutable borrows together"); 
    println!("  - Any borrows while calling mutating methods");
    
    // These would all cause compile errors:
    // let mut_ref = stack.peek_mut().unwrap();
    // let immut_ref = stack.peek().unwrap(); // ERROR!
    
    // let mut1 = stack.peek_mut().unwrap();
    // let mut2 = stack.peek_mut().unwrap(); // ERROR!
    
    println!("\nFinal stack: {:?}", stack);
}

fn main() {
    demonstrate_mutable_borrowing();
    demonstrate_borrowing_rules();
}