use mission1::Stack;

fn demonstrate_ownership_transfer() {
    println!("=== Example 1: Copying types (i32) ===");
    let mut int_stack = Stack::new();
    let number = 42;
    int_stack.push(number);
    // i32 implements Copy, so we can still use 'number'
    println!("Original number still available: {}", number);
    println!("Int stack: {:?}", int_stack);
    
    println!("\n=== Example 2: Moving types (String) ===");
    let mut string_stack = Stack::new();
    let text = String::from("Hello World");
    println!("Before push - text: {}", text);
    
    string_stack.push(text);
    // String does NOT implement Copy, so 'text' is moved
    // println!("After push - text: {}", text); // This would cause a compile error!
    
    println!("Stack after pushing string: {:?}", string_stack);
    
    println!("\n=== Example 3: What happens internally ===");
    let value1 = String::from("First");
    let value2 = String::from("Second");
    
    println!("About to push value1: {}", value1);
    string_stack.push(value1); // value1 is MOVED into the stack
    // value1 is now invalid
    
    println!("About to push value2: {}", value2);
    string_stack.push(value2); // value2 is MOVED into the stack
    // value2 is now invalid
    
    println!("Final stack: {:?}", string_stack);
    println!("Stack length: {}", string_stack.len());
}

fn main() {
    demonstrate_ownership_transfer();
}