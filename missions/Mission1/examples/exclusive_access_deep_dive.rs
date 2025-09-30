use mission1::Stack;

fn demonstrate_exclusive_access_concept() {
    println!("=== EXCLUSIVE ACCESS EXPLAINED ===");
    
    let mut stack = Stack::new();
    stack.push(String::from("Hello"));
    stack.push(String::from("World"));
    
    println!("Stack: {:?}", stack);
    
    // SCENARIO 1: What exclusive access PREVENTS
    println!("\n--- What Rust PREVENTS (would cause data races in other languages) ---");
    
    // In other languages, this could be dangerous:
    // Thread 1 gets mutable reference: let mut_ref1 = &mut top_value;
    // Thread 2 gets mutable reference: let mut_ref2 = &mut top_value;  
    // Thread 1 writes: *mut_ref1 = "Changed by thread 1";
    // Thread 2 writes: *mut_ref2 = "Changed by thread 2";
    // RESULT: Undefined behavior, data corruption, crashes!
    
    println!("In other languages, multiple threads could:");
    println!("1. Both get mutable access to the same memory");
    println!("2. Both try to modify it simultaneously");
    println!("3. Cause data corruption or crashes");
    println!("4. Create 'data races' - unpredictable behavior");
    
    // SCENARIO 2: How Rust enforces exclusive access
    println!("\n--- How Rust ENFORCES exclusive access ---");
    
    {
        println!("Getting ONE mutable reference...");
        let mut_ref = stack.peek_mut().unwrap();
        println!("Got mutable reference to: {}", mut_ref);
        
        // While mut_ref exists, these would ALL cause compile errors:
        // let another_mut = stack.peek_mut();  // ❌ Second mutable borrow
        // let immut_ref = stack.peek();        // ❌ Immutable while mutable exists  
        // stack.push("new".to_string());       // ❌ Method that needs mutable access
        // let len = stack.len();               // ❌ Even innocent methods are blocked!
        
        println!("While mutable reference exists:");
        println!("- No other mutable references allowed");
        println!("- No immutable references allowed");
        println!("- No method calls on the stack allowed");
        println!("- GUARANTEED exclusive access!");
        
        *mut_ref = String::from("Modified safely");
        println!("Modified through exclusive reference: {}", mut_ref);
        
    } // ← mut_ref goes out of scope here, exclusive access ends
    
    println!("\nAfter exclusive access ends:");
    println!("Stack: {:?}", stack);
    println!("Now we can use the stack normally again!");
}

fn demonstrate_why_exclusive_access_matters() {
    println!("\n=== WHY EXCLUSIVE ACCESS MATTERS ===");
    
    let mut stack = Stack::new();
    stack.push(vec![1, 2, 3, 4, 5]);
    
    println!("Stack with vector: {:?}", stack);
    
    // SAFE: Exclusive mutable access
    {
        let mut_ref = stack.peek_mut().unwrap();
        println!("Original vector: {:?}", mut_ref);
        
        // We can safely modify because we have exclusive access
        mut_ref.push(6);
        mut_ref.push(7);
        
        println!("Modified vector: {:?}", mut_ref);
        
        // No one else can access this data while we're modifying it
        // This prevents:
        // 1. Someone reading while we're writing (seeing inconsistent state)
        // 2. Someone else writing while we're writing (data corruption)
        // 3. Memory being freed while we're using it (use-after-free)
        
    } // Exclusive access ends here
    
    println!("Final stack: {:?}", stack);
    
    // Now others can access it safely
    let final_vec = stack.peek().unwrap();
    println!("Safe to read now: {:?}", final_vec);
}

fn show_contrast_with_multiple_immutable() {
    println!("\n=== CONTRAST: Multiple Immutable Borrows (SAFE) ===");
    
    let mut stack = Stack::new();
    stack.push("Shared data".to_string());
    
    // Multiple readers are safe because no one is modifying
    {
        let reader1 = stack.peek().unwrap();
        let reader2 = stack.peek().unwrap();
        let reader3 = stack.peek().unwrap();
        
        println!("Reader 1 sees: {}", reader1);
        println!("Reader 2 sees: {}", reader2);
        println!("Reader 3 sees: {}", reader3);
        
        // All see the exact same data
        // No one can modify while readers exist
        // This is safe because data is read-only
        
        println!("All readers see identical data - this is safe!");
        
    } // All immutable borrows end here
    
    // Now we can modify again
    stack.push("New data".to_string());
    println!("Added new data after readers finished: {:?}", stack);
}

fn main() {
    demonstrate_exclusive_access_concept();
    demonstrate_why_exclusive_access_matters();
    show_contrast_with_multiple_immutable();
}