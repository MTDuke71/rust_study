// Integration Test: LIFO Order Validation
// Run with: cargo run --example lifo_integration

use mission1::Stack;

fn main() {
    println!("🧪 Integration Test: LIFO Order Validation\n");
    
    // ==============================================
    // Test: Push a sequence, pop and verify order
    // ==============================================
    
    let mut stack = Stack::new();
    let input_sequence = vec!["A", "B", "C", "D", "E"];
    
    println!("📥 Pushing sequence: {:?}", input_sequence);
    
    // Push all items
    for item in &input_sequence {
        stack.push(*item);
        println!("  Pushed: {}", item);
    }
    
    println!("\nStack state: {:?}", stack);
    
    println!("\n📤 Popping and checking LIFO order:");
    
    // Expected: should pop in reverse order (LIFO)
    let expected_output = vec!["E", "D", "C", "B", "A"];
    let mut actual_output = Vec::new();
    
    // Pop all items
    while let Some(item) = stack.pop() {
        println!("  Popped: {}", item);
        actual_output.push(item);
    }
    
    println!("\n🔍 Order Verification:");
    println!("Input order:    {:?}", input_sequence);
    println!("Expected output: {:?}", expected_output);
    println!("Actual output:   {:?}", actual_output);
    
    // Verify LIFO property
    assert_eq!(actual_output, expected_output, "❌ LIFO order violated!");
    
    println!("\n✅ LIFO Test PASSED!");
    println!("✅ Last item pushed (E) was first item popped");
    println!("✅ First item pushed (A) was last item popped");
    
    // ==============================================
    // Additional test: Interleaved push/pop
    // ==============================================
    println!("\n🔄 Bonus Test: Interleaved Operations");
    
    let mut stack = Stack::new();
    
    stack.push(1);
    stack.push(2);
    println!("Pushed 1, 2. Popped: {:?}", stack.pop()); // Should be 2
    
    stack.push(3);
    stack.push(4);
    println!("Pushed 3, 4. Popped: {:?}", stack.pop()); // Should be 4
    println!("Popped again: {:?}", stack.pop()); // Should be 3
    println!("Popped again: {:?}", stack.pop()); // Should be 1
    println!("Popped empty: {:?}", stack.pop()); // Should be None
    
    println!("\n🎉 All integration tests passed!");
}