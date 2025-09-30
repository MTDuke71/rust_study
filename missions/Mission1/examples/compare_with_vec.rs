// Validation: Compare our Stack behavior with Vec
// Run with: cargo run --example compare_with_vec

use mission1::Stack;

fn main() {
    println!("🔍 Validation: Comparing Stack vs Vec behavior\n");
    
    // ==============================================
    // Test 1: Push and Pop behavior
    // ==============================================
    println!("📋 Test 1: Push and Pop");
    
    let mut our_stack = Stack::new();
    let mut vec_as_stack = Vec::new();
    
    // Push the same items to both
    let items = vec!["first", "second", "third"];
    for item in &items {
        our_stack.push(*item);
        vec_as_stack.push(*item);  // Vec also has push
    }
    
    println!("After pushing {:?}:", items);
    println!("Our stack: {:?}", our_stack);
    println!("Vec: {:?}", vec_as_stack);
    
    // Pop from both and compare
    println!("\nPopping items:");
    while let (Some(from_stack), Some(from_vec)) = (our_stack.pop(), vec_as_stack.pop()) {
        println!("Stack popped: {}, Vec popped: {}", from_stack, from_vec);
        assert_eq!(from_stack, from_vec, "❌ Mismatch! Stack and Vec should behave the same");
    }
    
    println!("✅ Both Stack and Vec follow LIFO (Last In, First Out)!");
    
    // ==============================================
    // Test 2: Empty behavior
    // ==============================================
    println!("\n📋 Test 2: Empty behavior");
    
    assert_eq!(our_stack.pop(), vec_as_stack.pop()); // Both should return None
    println!("✅ Both return None when empty");
    
    // ==============================================
    // Test 3: Peek vs last()
    // ==============================================
    println!("\n📋 Test 3: Peek behavior");
    
    our_stack.push("peek_me");
    vec_as_stack.push("peek_me");
    
    let stack_peek = our_stack.peek();
    let vec_last = vec_as_stack.last();
    
    println!("Stack peek: {:?}", stack_peek);
    println!("Vec last: {:?}", vec_last);
    assert_eq!(stack_peek, vec_last);
    println!("✅ peek() behaves like Vec's last()!");
    
    println!("\n🎉 All validation tests passed! Our Stack behaves like Vec for shared operations.");
}