// Your second day with Rust ownership
// Run this with: cargo run --example second_day

use mission1::Stack;

fn main() {
    println!("🎯 Welcome to DAY 2 of Rust ownership!");
    println!("Yesterday: Numbers are copied");
    println!("Today: Strings are moved (given away)\n");
    
    // ==============================================
    // DAY 2: Strings are like original documents
    // ==============================================
    println!("📚 LESSON 2: Strings are like original documents");
    println!("When you give someone the original, YOU don't have it anymore!\n");
    
    let my_text = String::from("Hello World");
    println!("I wrote this document: {}", my_text);
    
    let mut stack = Stack::new();
    
    println!("Now I'm giving it to the stack...");
    stack.push(my_text);  // This MOVES the string (gives it away)
    println!("Stack received it: {:?}", stack);
    
    // This next line would be an ERROR if uncommented:
    // println!("My document says: {}", my_text);  // ❌ Error! I don't have it anymore
    
    println!("✅ I can't use my_text anymore - the stack owns it now!");
    
    println!("\n{}", "=".repeat(50));
    
    // ==============================================
    // Getting it back
    // ==============================================
    println!("\n📚 BONUS: Getting your document back");
    println!("I can ask the stack to give it back to me:");
    
    let got_it_back = stack.pop().unwrap();
    println!("Got my document back: {}", got_it_back);
    println!("Now I own it again! Stack is empty: {:?}", stack);
    
    println!("\n🎉 Congratulations!");
    println!("You learned: Strings are MOVED (given away)");
    println!("But you can get them back with pop()!");
    println!("\nSee you tomorrow for borrowing! 🌟");
}