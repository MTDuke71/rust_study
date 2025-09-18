// Demonstration: Why the original req1 test was bad
// This file shows the compile errors you'd get

use mission1::Stack;

fn main() {
    println!("🔍 Demonstrating why the original req1 test failed\n");
    
    // ==============================================
    // What the original test tried to do
    // ==============================================
    
    println!("❌ This is what the ORIGINAL test tried to do:");
    println!("let mut s = Stack::new();");
    println!("s.push(\"str\");    // Makes s a Stack<&str>");
    println!("s.push(42);        // ERROR! Can't push i32 to Stack<&str>");
    println!("s.push(3.14);      // ERROR! Can't push f64 to Stack<&str>");
    
    // Let's try it and see the error:
    let mut s = Stack::new();
    s.push("str");    // This line determines that s is Stack<&str>
    
    // Uncomment these lines to see the compile errors:
    //s.push(42);       // ❌ Compile error!
    //s.push(3.14);     // ❌ Compile error!
    
    println!("\n🎯 Why this fails:");
    println!("1. When you call Stack::new(), Rust doesn't know the type yet");
    println!("2. s.push(\"str\") tells Rust: 'This is a Stack<&str>'");
    println!("3. Now s can ONLY hold &str values");
    println!("4. Trying to push 42 (i32) fails because 42 ≠ &str");
    
    // ==============================================
    // The correct approach
    // ==============================================
    
    println!("\n✅ The CORRECT way to test generics:");
    println!("Make separate stacks for each type!");
    
    let mut string_stack = Stack::new();
    string_stack.push("str");
    println!("string_stack can hold: &str");
    
    let mut int_stack = Stack::new();
    int_stack.push(42);
    println!("int_stack can hold: i32");
    
    let mut float_stack = Stack::new();
    float_stack.push(3.14);
    println!("float_stack can hold: f64");
    
    println!("\n🎉 This proves Stack<T> works with ANY type T!");
    println!("Each stack is strongly typed, but you can make stacks of different types.");
}

// ==============================================
// The type annotations explained
// ==============================================

#[allow(dead_code)]
fn type_annotations_demo() {
    // These are all different types:
    let _string_stack: Stack<&str> = Stack::new();    // For string slices
    let _int_stack: Stack<i32> = Stack::new();        // For 32-bit integers  
    let _float_stack: Stack<f64> = Stack::new();      // For 64-bit floats
    let _vec_stack: Stack<Vec<i32>> = Stack::new();   // For vectors of integers
    
    // You can even have stacks of stacks!
    let _nested: Stack<Stack<i32>> = Stack::new();    // Stack of stacks
}