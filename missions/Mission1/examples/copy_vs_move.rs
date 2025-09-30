use mission1::Stack;

fn demonstrate_copy_vs_move() {
    println!("=== COPY TYPES (i32) ===");
    
    // i32 is stored entirely on the stack
    let number = 42;
    println!("Original number: {}", number);
    
    // When we "move" it, Rust actually COPIES it
    let mut stack = Stack::new();
    stack.push(number);  // This COPIES the value
    
    // We can still use the original!
    println!("Original number after 'move': {}", number);
    println!("Stack contains: {:?}", stack);
    
    // Both the original AND the stack have their own copy
    let popped = stack.pop().unwrap();
    println!("Popped value: {}", popped);
    println!("Original still exists: {}", number);
    println!("All three are independent copies!");
    
    println!("\n=== MOVE TYPES (String) ===");
    
    // String has two parts:
    // 1. Stack data: pointer, length, capacity  
    // 2. Heap data: the actual text characters
    let text = String::from("Hello World");
    println!("Original text: {}", text);
    
    // When we move it, ownership transfers
    let mut string_stack = Stack::new();
    string_stack.push(text);  // This MOVES ownership
    
    // We CAN'T use the original anymore!
    // println!("Original text after move: {}", text); // COMPILE ERROR!
    println!("Stack contains: {:?}", string_stack);
    
    // Only the stack owns the String now
    let popped_text = string_stack.pop().unwrap();
    println!("Popped text: {}", popped_text);
    // Now WE own it, and the stack doesn't
    
    println!("\n=== WHY THE DIFFERENCE? ===");
    println!("i32: Just 4 bytes on stack - cheap to copy");
    println!("String: Pointer + heap allocation - expensive to copy");
    println!("Rust prevents accidental expensive operations!");
}

// Let's also demonstrate what String actually contains
fn show_string_internals() {
    println!("\n=== STRING INTERNALS ===");
    
    let text = String::from("Hello");
    println!("String value: {}", text);
    println!("String length: {} bytes", text.len());
    println!("String capacity: {} bytes", text.capacity());
    println!("String pointer: {:p}", text.as_ptr());
    
    // The String struct on the stack contains:
    // - A pointer to heap memory (8 bytes on 64-bit)
    // - Length (8 bytes)  
    // - Capacity (8 bytes)
    // Total: 24 bytes on stack + however many bytes on heap
    
    println!("Stack part: ~24 bytes (pointer + length + capacity)");
    println!("Heap part: {} bytes (the actual characters)", text.len());
    println!("If we copied this, we'd need to allocate new heap memory!");
}

fn main() {
    demonstrate_copy_vs_move();
    show_string_internals();
}