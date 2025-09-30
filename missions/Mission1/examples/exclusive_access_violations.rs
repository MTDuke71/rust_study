use mission1::Stack;

fn try_to_violate_exclusive_access() {
    println!("=== ATTEMPTING TO VIOLATE EXCLUSIVE ACCESS ===");
    println!("(These examples show what Rust prevents at compile time)");
    
    let mut stack = Stack::new();
    stack.push("Test data".to_string());
    
    println!("\nExample 1: Trying to get two mutable references");
    println!("Code that would fail:");
    println!("    let mut_ref1 = stack.peek_mut().unwrap();");
    println!("    let mut_ref2 = stack.peek_mut().unwrap(); // ❌ ERROR!");
    
    // Uncomment these lines to see the actual compile error:
    {
        //let mut_ref1 = stack.peek_mut().unwrap();
        //let mut_ref2 = stack.peek_mut().unwrap(); // ERROR: second mutable borrow
        //println!("mut_ref1: {}", mut_ref1);
        //println!("mut_ref2: {}", mut_ref2);
    }
    
    println!("\nExample 2: Trying to mix mutable and immutable borrows");
    println!("Code that would fail:");
    println!("    let mut_ref = stack.peek_mut().unwrap();");
    println!("    let imm_ref = stack.peek().unwrap(); // ❌ ERROR!");
    
    // Uncomment these lines to see the actual compile error:
    // {
    //     let mut_ref = stack.peek_mut().unwrap();
    //     let imm_ref = stack.peek().unwrap(); // ERROR: immutable borrow while mutable exists
    //     println!("mut_ref: {}", mut_ref);
    //     println!("imm_ref: {}", imm_ref);
    // }
    
    println!("\nExample 3: Trying to call methods while mutably borrowed");
    println!("Code that would fail:");
    println!("    let mut_ref = stack.peek_mut().unwrap();");
    println!("    stack.push(\"new\".to_string()); // ❌ ERROR!");
    println!("    let len = stack.len(); // ❌ ERROR!");
    
    // Uncomment these lines to see the actual compile error:
    // {
    //     let mut_ref = stack.peek_mut().unwrap();
    //     stack.push("new".to_string()); // ERROR: cannot borrow as mutable
    //     let len = stack.len(); // ERROR: cannot borrow as immutable
    //     println!("mut_ref: {}", mut_ref);
    // }
    
    println!("\n✅ What DOES work: Proper exclusive access");
    {
        let mut_ref = stack.peek_mut().unwrap();
        println!("Got exclusive access to: {}", mut_ref);
        *mut_ref = "Safely modified".to_string();
        println!("Modified to: {}", mut_ref);
    } // Exclusive access ends here
    
    // Now we can do other operations
    println!("After exclusive access ended:");
    println!("  Can read: {}", stack.peek().unwrap());
    println!("  Can check length: {}", stack.len());
    stack.push("Can add new items".to_string());
    println!("  Final stack: {:?}", stack);
}

fn demonstrate_the_danger_rust_prevents() {
    println!("\n=== THE DANGER RUST PREVENTS ===");
    println!("Imagine if this pseudocode were allowed:");
    println!();
    println!("// DANGEROUS PSEUDOCODE (not actual Rust!)");
    println!("let mut_ref1 = stack.peek_mut().unwrap();  // Get mutable reference");
    println!("let mut_ref2 = stack.peek_mut().unwrap();  // Get ANOTHER mutable reference");
    println!(""); 
    println!("// Now both references point to the same memory!");
    println!("*mut_ref1 = String::from(\"Thread 1 data\");");
    println!("*mut_ref2 = String::from(\"Thread 2 data\");");
    println!("");
    println!("// Which value wins? Undefined behavior!");
    println!("// Could crash, corrupt data, or work 'by accident'");
    println!();
    println!("🛡️  Rust prevents this entire class of bugs at COMPILE TIME!");
    println!("🛡️  No runtime overhead - the checks are free!");
    println!("🛡️  No crashes, no corruption, no undefined behavior!");
}

fn show_memory_layout_during_exclusive_access() {
    println!("\n=== MEMORY LAYOUT DURING EXCLUSIVE ACCESS ===");
    
    let mut stack = Stack::new();
    stack.push("Data in memory".to_string());
    
    println!("Before exclusive access:");
    println!("Stack owns the data: {:?}", stack);
    
    {
        let mut_ref = stack.peek_mut().unwrap();
        println!("\nDuring exclusive access:");
        println!("  mut_ref points to: {}", mut_ref);
        println!("  Memory address: {:p}", mut_ref);
        println!("  Stack cannot be used");
        println!("  No other references can exist");
        println!("  Complete control over this memory location");
        
        // We have guaranteed exclusive access to modify
        *mut_ref = "Modified through exclusive access".to_string();
        
        println!("  Modified through exclusive access: {}", mut_ref);
        
    } // mut_ref is dropped here, exclusive access ends
    
    println!("\nAfter exclusive access:");
    println!("Stack owns the data again: {:?}", stack);
    println!("Safe to create new references now");
}

fn main() {
    try_to_violate_exclusive_access();
    demonstrate_the_danger_rust_prevents();
    show_memory_layout_during_exclusive_access();
}