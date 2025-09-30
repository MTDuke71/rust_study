use mission1::Stack;

fn demonstrate_generics() {
    // T = String
    let mut string_stack: Stack<String> = Stack::new();
    
    // T = i32  
    let mut int_stack: Stack<i32> = Stack::new();
    
    // T = bool
    let mut bool_stack: Stack<bool> = Stack::new();
    
    // Each stack is a DIFFERENT TYPE even though they use the same code!
    string_stack.push("Hello".to_string());
    int_stack.push(42);
    bool_stack.push(true);
    
    // You can't mix types - this would be a compile error:
    // int_stack.push("Hello"); // ERROR: expected i32, found &str
    
    println!("String stack: {:?}", string_stack);
    println!("Int stack: {:?}", int_stack);  
    println!("Bool stack: {:?}", bool_stack);
}

fn main() {
    demonstrate_generics();
}