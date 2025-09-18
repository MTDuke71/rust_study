use mission1::Stack;

fn demonstrate_pop_ownership() {
    let mut stack = Stack::new();
    
    println!("=== Setting up the stack ===");
    stack.push(String::from("First"));
    stack.push(String::from("Second"));  
    stack.push(String::from("Third"));
    println!("Stack after pushes: {:?}", stack);
    println!("Length: {}", stack.len());
    
    println!("\n=== Pop transfers ownership ===");
    
    // Pop the top item - we get ownership
    let popped1 = stack.pop().unwrap(); // Gets "Third"
    println!("Popped value: {}", popped1);
    println!("Stack after first pop: {:?}", stack);
    println!("Length: {}", stack.len());
    
    // We own 'popped1' now - we can do anything with it
    let modified = format!("{} (modified)", popped1);
    println!("We can modify our owned value: {}", modified);
    
    // Pop another
    let popped2 = stack.pop().unwrap(); // Gets "Second"
    println!("Popped value: {}", popped2);
    println!("Stack after second pop: {:?}", stack);
    
    // Pop the last one
    let popped3 = stack.pop().unwrap(); // Gets "First"
    println!("Popped value: {}", popped3);
    println!("Stack after third pop: {:?}", stack);
    
    // Try to pop from empty stack
    let empty_pop = stack.pop();
    println!("Pop from empty stack: {:?}", empty_pop);
    
    println!("\n=== All popped values are still owned by us ===");
    println!("popped1: {}", popped1);
    println!("popped2: {}", popped2);
    println!("popped3: {}", popped3);
    println!("The stack has NO access to these values anymore!");
}

fn main() {
    demonstrate_pop_ownership();
}