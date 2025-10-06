// Mission1_tut Step 1: Basic Stack Concepts
// Learn LIFO (Last In, First Out) behavior with a simple concrete implementation

fn main() {
    println!("=== Mission1_tut Step 1: Basic Stack Concepts ===\n");
    
    // 1. Understanding LIFO Behavior
    println!("1. LIFO (Last In, First Out) Behavior:");
    demonstrate_lifo_concept();
    
    // 2. Basic Stack Implementation  
    println!("\n2. Basic Stack Implementation:");
    demonstrate_basic_stack();
    
    // 3. Real World Stack Examples
    println!("\n3. Real World Stack Examples:");
    demonstrate_real_world_stacks();
    
    // 4. Stack Operations Analysis
    println!("\n4. Stack Operations Analysis:");
    analyze_stack_operations();
}

/// Demonstrate LIFO concept with simple analogy
fn demonstrate_lifo_concept() {
    println!("   Think of a stack of plates:");
    println!("   📒 ← You add plates on top (push)");
    println!("   📒 ← You remove plates from top (pop)"); 
    println!("   📒");
    println!("   📒 ← Bottom plate added first, removed last");
    
    println!("\n   LIFO Examples in Programming:");
    println!("   • Function call stack - return to most recent caller");
    println!("   • Undo operations - undo most recent action first");
    println!("   • Expression parsing - match most recent opening bracket");
    println!("   • Browser history - back button returns to previous page");
}

/// Basic stack implementation with i32 (concrete type)
struct BasicStack {
    items: Vec<i32>,
}

impl BasicStack {
    /// Create a new empty stack
    fn new() -> Self {
        println!("   Creating new empty stack...");
        BasicStack {
            items: Vec::new(),
        }
    }
    
    /// Add an item to the top of the stack
    fn push(&mut self, item: i32) {
        println!("   Pushing {} onto stack", item);
        self.items.push(item);
        self.print_state();
    }
    
    /// Remove and return the top item from the stack
    fn pop(&mut self) -> Option<i32> {
        let result = self.items.pop();
        match result {
            Some(item) => {
                println!("   Popped {} from stack", item);
                self.print_state();
                Some(item)
            }
            None => {
                println!("   Cannot pop from empty stack!");
                None
            }
        }
    }
    
    /// Look at the top item without removing it
    fn peek(&self) -> Option<&i32> {
        let result = self.items.last();
        match result {
            Some(item) => {
                println!("   Top item is: {}", item);
                Some(item)
            }
            None => {
                println!("   Stack is empty - nothing to peek");
                None
            }
        }
    }
    
    /// Check if the stack is empty
    fn is_empty(&self) -> bool {
        let empty = self.items.is_empty();
        println!("   Stack is empty: {}", empty);
        empty
    }
    
    /// Get the number of items in the stack
    fn len(&self) -> usize {
        let length = self.items.len();
        println!("   Stack length: {}", length);
        length
    }
    
    /// Helper function to print current stack state
    fn print_state(&self) {
        if self.items.is_empty() {
            println!("   Stack: []");
        } else {
            print!("   Stack: [");
            for (i, item) in self.items.iter().enumerate() {
                if i > 0 { print!(", "); }
                print!("{}", item);
            }
            println!("] ← top is rightmost");
        }
    }
}

fn demonstrate_basic_stack() {
    let mut stack = BasicStack::new();
    
    // Test basic operations
    println!("   Testing basic stack operations:");
    
    // Start with empty stack
    stack.is_empty();
    stack.len();
    
    // Push some items
    stack.push(10);
    stack.push(20);
    stack.push(30);
    
    // Peek at top
    stack.peek();
    
    // Pop items (LIFO order)
    stack.pop();  // Should return 30
    stack.pop();  // Should return 20
    
    // Check state
    stack.len();
    stack.peek(); // Should show 10
    
    // Pop remaining item
    stack.pop();  // Should return 10
    
    // Try to pop from empty stack
    stack.pop();  // Should return None
    stack.is_empty();
}

fn demonstrate_real_world_stacks() {
    println!("   Browser History Example:");
    let mut history = BasicStack::new();
    
    // Simulate browsing (each number represents a page ID)
    history.push(1);  // Visit homepage
    history.push(2);  // Visit products page
    history.push(3);  // Visit specific product
    
    println!("   User clicks 'Back' button:");
    history.pop();    // Go back to products page
    history.pop();    // Go back to homepage
    
    println!("\n   Calculator Undo Example:");
    let mut operations = BasicStack::new();
    
    operations.push(5);   // User enters 5
    operations.push(10);  // User enters + 10 (result: 15)
    operations.push(3);   // User enters * 3 (result: 45)
    
    println!("   User clicks 'Undo':");
    operations.pop();     // Undo multiplication (back to 15)
    operations.pop();     // Undo addition (back to 5)
}

fn analyze_stack_operations() {
    println!("   Time Complexity Analysis:");
    println!("   • push():     O(1) amortized - occasionally O(n) for reallocation");
    println!("   • pop():      O(1) always - just remove last element");
    println!("   • peek():     O(1) always - just look at last element");
    println!("   • is_empty(): O(1) always - check if length == 0");
    println!("   • len():      O(1) always - Vec tracks length");
    
    println!("\n   Space Complexity:");
    println!("   • Storage: O(n) where n is number of items");
    println!("   • Each item stored exactly once");
    println!("   • Vec may allocate extra capacity for future growth");
    
    println!("\n   Why Vec<T> for Stack Implementation?");
    println!("   ✅ Dynamic resizing - stack can grow as needed");
    println!("   ✅ Cache efficient - items stored contiguously");  
    println!("   ✅ O(1) access to end - perfect for LIFO operations");
    println!("   ✅ Rust manages memory - no manual allocation needed");
}

// Self-Assessment Questions
fn self_assessment() {
    println!("\n=== Self-Assessment Questions ===");
    println!("After running this step, you should understand:");
    println!("1. What does LIFO mean and why is it useful?");
    println!("2. What are the core stack operations (push, pop, peek)?");
    println!("3. Why do we use Vec<T> as the backing storage?");
    println!("4. What happens when you pop from an empty stack?");
    println!("5. Can you name 3 real-world applications of stacks?");
    
    println!("\n=== Next Step Preview ===");
    println!("Step 2 will transform this concrete i32 stack into a generic Stack<T>");
    println!("that works with any type - integers, strings, custom structs, etc.");
    println!("Run: cargo run --example step2_generic_stack");
}

// Call self-assessment at the end
fn call_self_assessment() {
    self_assessment();
}

// Note: We use this pattern to ensure self_assessment runs
static _: fn() = call_self_assessment;