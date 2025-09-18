// Your first day with Rust ownership
// Run this with: cargo run --example first_day

use mission1::Stack;

fn main() {
    println!("🎯 Welcome to your first day with Rust ownership!");
    println!("Today we'll learn ONE simple rule at a time.\n");
    
    // ==============================================
    // DAY 1: Integers are easy (they copy)
    // ==============================================
    println!("📚 LESSON 1: Numbers are like photocopies");
    println!("When you give someone a number, you still have yours too!\n");
    
    let my_number = 42;
    println!("I have the number: {}", my_number);
    
    let mut stack = Stack::new();
    stack.push(my_number);  // This COPIES the number
    println!("I gave a copy to the stack");
    println!("I STILL have my number: {}", my_number);
    println!("Stack also has a copy: {:?}\n", stack);
    
    println!("✅ See? I can use my number AND the stack has one too!");
    println!("Numbers are copied, not moved.\n");
    
    println!("{}", "=".repeat(50));
    
    // ==============================================
    // That's it for today! One concept at a time.
    // ==============================================
    println!("🎉 Congratulations!");
    println!("You learned: Numbers (i32, f64, etc.) are COPIED");
    println!("Tomorrow we'll learn about Strings (which are MOVED)");
    println!("\nThat's enough for today! 🌟");
}