use mission1::Stack;

fn demonstrate_peek_borrowing() {
    println!("=== PEEK: IMMUTABLE BORROWING ===");
    
    let mut stack = Stack::new();
    stack.push(String::from("Hello"));
    stack.push(String::from("World"));
    stack.push(String::from("Rust"));
    
    println!("Stack: {:?}", stack);
    
    // peek() gives us a REFERENCE to the top value
    let top_ref = stack.peek().unwrap();
    println!("Top value (via reference): {}", top_ref);
    
    // The stack still owns the data
    println!("Stack still contains: {:?}", stack);
    println!("Length is still: {}", stack.len());
    
    // We can get multiple immutable references
    let ref1 = stack.peek().unwrap();
    let ref2 = stack.peek().unwrap();
    let ref3 = stack.peek().unwrap();
    
    println!("All references point to same data:");
    println!("  ref1: {}", ref1);
    println!("  ref2: {}", ref2);
    println!("  ref3: {}", ref3);
    
    // All references are pointing to the same memory location
    println!("Memory addresses (should be same):");
    println!("  ref1 address: {:p}", ref1);
    println!("  ref2 address: {:p}", ref2);
    println!("  ref3 address: {:p}", ref3);
    
    // IMPORTANT: While we have immutable references, we CANNOT modify
    // Uncommenting the next line would cause a compile error:
    // stack.push(String::from("New")); // ERROR: cannot borrow as mutable
    
    println!("\n=== After references go out of scope ===");
    // The references end here, so now we can modify again
} // All references (ref1, ref2, ref3, top_ref) are dropped here

fn demonstrate_reference_lifetime() {
    println!("\n=== REFERENCE LIFETIMES ===");
    
    let mut stack = Stack::new();
    stack.push(42);
    stack.push(99);
    
    {
        println!("Creating reference in inner scope...");
        let peek_ref = stack.peek().unwrap();
        println!("Peeked value: {}", peek_ref);
        
        // While peek_ref exists, we cannot mutate
        // stack.push(123); // Would cause compile error
        
        println!("Reference exists, cannot mutate stack");
    } // peek_ref is dropped here
    
    // Now we can mutate again!
    println!("Reference dropped, can mutate again");
    stack.push(123);
    println!("Successfully pushed 123");
    println!("Final stack: {:?}", stack);
}

fn main() {
    demonstrate_peek_borrowing();
    demonstrate_reference_lifetime();
}