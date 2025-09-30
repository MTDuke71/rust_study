use mission1::Stack;

fn demonstrate_constructors() {
    // Method 1: new() - starts with zero capacity
    let mut stack1: Stack<i32> = Stack::new();
    println!("Empty stack1: {:?}", stack1);
    
    // Method 2: with_capacity() - pre-allocates space
    let mut stack2: Stack<String> = Stack::with_capacity(5);
    println!("Empty stack2 with capacity 5: {:?}", stack2);
    
    // Both work the same way for operations
    stack1.push(10);
    stack1.push(20);
    
    stack2.push("Hello".to_string());
    stack2.push("World".to_string());
    
    println!("stack1 after pushes: {:?}", stack1);
    println!("stack2 after pushes: {:?}", stack2);
    
    // Demonstrate the difference with capacity
    println!("stack1 length: {}", stack1.len());
    println!("stack2 length: {}", stack2.len());
}

fn main() {
    demonstrate_constructors();
}