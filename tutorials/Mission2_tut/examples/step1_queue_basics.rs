// Mission2_tut Step 1: Queue Basics
// Understand FIFO principles and queue operations

fn main() {
    println!("=== Mission2_tut Step 1: Queue Fundamentals ===\n");
    
    // 1. FIFO vs LIFO Comparison
    println!("1. FIFO (First In, First Out) vs LIFO (Last In, First Out):");
    demonstrate_fifo_vs_lifo();
    
    // 2. Real-World Queue Analogies
    println!("\n2. Real-World Queue Examples:");
    demonstrate_real_world_analogies();
    
    // 3. Basic Queue Operations
    println!("\n3. Core Queue Operations:");
    demonstrate_queue_operations();
    
    // 4. Queue vs Stack Behavior
    println!("\n4. Queue vs Stack: Side-by-Side Comparison:");
    compare_queue_vs_stack();
    
    // 5. Common Queue Use Cases
    println!("\n5. When to Use Queues:");
    demonstrate_use_cases();
    
    self_assessment();
}

fn demonstrate_fifo_vs_lifo() {
    println!("   FIFO (Queue): Like a line at a coffee shop");
    println!("   - First person in line gets served first");
    println!("   - New customers join at the back");
    println!("   - Order: Alice → Bob → Charlie");
    println!("   - Served: Alice (first), then Bob, then Charlie");
    
    println!("\n   LIFO (Stack): Like a stack of plates");
    println!("   - Last plate placed on top gets taken first");
    println!("   - New plates go on top");
    println!("   - Order: Alice's plate → Bob's plate → Charlie's plate");
    println!("   - Taken: Charlie's (last in), then Bob's, then Alice's");
    
    // Simple demonstration with vectors
    let mut queue_simulation = vec!["Alice", "Bob", "Charlie"];
    let mut stack_simulation = vec!["Alice", "Bob", "Charlie"];
    
    println!("\n   Queue Processing (FIFO):");
    while let Some(person) = queue_simulation.get(0).copied() {
        queue_simulation.remove(0); // Remove from front (not efficient, just for demo)
        println!("   Serving: {}", person);
    }
    
    println!("\n   Stack Processing (LIFO):");
    while let Some(person) = stack_simulation.pop() {
        println!("   Taking: {}", person);
    }
}

fn demonstrate_real_world_analogies() {
    println!("   🏪 **Grocery Store Checkout Line**");
    println!("      - Customers join at the back");
    println!("      - Cashier serves from the front");
    println!("      - Fair: first come, first served");
    
    println!("\n   🖨️ **Printer Queue**");
    println!("      - Print jobs submitted in order");
    println!("      - Printer processes oldest job first");
    println!("      - Prevents newer jobs from cutting in line");
    
    println!("\n   🚗 **Drive-Through**");
    println!("      - Cars line up behind each other");
    println!("      - First car gets served at window");
    println!("      - New cars join at the back");
    
    println!("\n   🏭 **Assembly Line Buffer**");
    println!("      - Parts arrive from supplier");
    println!("      - Workers take parts in arrival order");
    println!("      - Maintains production sequence");
    
    println!("\n   🎮 **Game Event Processing**");
    println!("      - User actions (click, keypress) queued");
    println!("      - Game processes events in chronological order");
    println!("      - Preserves user interaction sequence");
}

fn demonstrate_queue_operations() {
    println!("   Core queue operations and their purposes:");
    
    println!("\n   📥 **enqueue(item)** - Add to Back");
    println!("      - Adds new element to the rear of queue");
    println!("      - Like joining the end of a line");
    println!("      - Rust: queue.enqueue(value) -> Result<(), T>");
    
    println!("\n   📤 **dequeue()** - Remove from Front");
    println!("      - Removes and returns element from front");
    println!("      - Like being served at a counter");
    println!("      - Rust: queue.dequeue() -> Option<T>");
    
    println!("\n   👀 **peek()** - Look at Front (No Removal)");
    println!("      - Shows what's next without removing it");
    println!("      - Like seeing who's first in line");
    println!("      - Rust: queue.peek() -> Option<&T>");
    
    println!("\n   📊 **len()** - Count Elements");
    println!("      - Returns number of items in queue");
    println!("      - Like counting people in line");
    println!("      - Rust: queue.len() -> usize");
    
    println!("\n   ❓ **is_empty()** - Check if Empty");
    println!("      - Returns true if no elements");
    println!("      - Like checking if line is empty");
    println!("      - Rust: queue.is_empty() -> bool");
    
    // Demonstrate with a simple vector simulation
    let mut simple_queue = Vec::new();
    
    println!("\n   🎬 **Operation Sequence Demo:**");
    
    // Enqueue operations
    simple_queue.push("Task A");
    println!("   enqueue('Task A') → Queue: ['Task A']");
    
    simple_queue.push("Task B");
    println!("   enqueue('Task B') → Queue: ['Task A', 'Task B']");
    
    simple_queue.push("Task C");
    println!("   enqueue('Task C') → Queue: ['Task A', 'Task B', 'Task C']");
    
    // Peek operation
    if let Some(front) = simple_queue.first() {
        println!("   peek() → '{}'", front);
    }
    
    // Dequeue operations
    if !simple_queue.is_empty() {
        let item = simple_queue.remove(0);
        println!("   dequeue() → '{}', Queue: {:?}", item, simple_queue);
    }
    
    if !simple_queue.is_empty() {
        let item = simple_queue.remove(0);
        println!("   dequeue() → '{}', Queue: {:?}", item, simple_queue);
    }
    
    println!("   len() → {}", simple_queue.len());
    println!("   is_empty() → {}", simple_queue.is_empty());
}

fn compare_queue_vs_stack() {
    println!("   Processing same data with Queue vs Stack:");
    
    let data = vec![1, 2, 3, 4, 5];
    println!("   Input data: {:?}", data);
    
    // Queue simulation (FIFO)
    let mut queue_sim = data.clone();
    println!("\n   📥 Queue (FIFO) Processing:");
    let mut queue_results = Vec::new();
    while !queue_sim.is_empty() {
        let item = queue_sim.remove(0); // Remove from front
        queue_results.push(item);
        println!("   Process: {} → Results so far: {:?}", item, queue_results);
    }
    
    // Stack simulation (LIFO)
    let mut stack_sim = data.clone();
    println!("\n   📚 Stack (LIFO) Processing:");
    let mut stack_results = Vec::new();
    while let Some(item) = stack_sim.pop() { // Remove from back
        stack_results.push(item);
        println!("   Process: {} → Results so far: {:?}", item, stack_results);
    }
    
    println!("\n   📊 **Final Results:**");
    println!("   Queue output: {:?} (same order as input)", queue_results);
    println!("   Stack output: {:?} (reversed order)", stack_results);
    
    println!("\n   🎯 **Key Insight:**");
    println!("   - Queue preserves original order (FIFO)");
    println!("   - Stack reverses order (LIFO)");
    println!("   - Choose based on whether order matters!");
}

fn demonstrate_use_cases() {
    println!("   🎯 **When to Choose Queues:**");
    
    println!("\n   ✅ **Breadth-First Search (BFS)**");
    println!("      - Explore all neighbors before going deeper");
    println!("      - Queue ensures level-by-level processing");
    println!("      - Example: Finding shortest path in maze");
    
    println!("\n   ✅ **Task Scheduling**");
    println!("      - Process jobs in submission order");
    println!("      - Fair scheduling: first submitted, first executed");
    println!("      - Example: Print queue, CPU process scheduler");
    
    println!("\n   ✅ **Buffer Management**");
    println!("      - Handle data streams with bounded capacity");
    println!("      - Smooth out burst traffic");
    println!("      - Example: Network packet buffers, audio streaming");
    
    println!("\n   ✅ **Event Processing**");
    println!("      - Handle events in chronological order");
    println!("      - Preserve user action sequences");
    println!("      - Example: GUI event loops, game input handling");
    
    println!("\n   ✅ **Producer-Consumer Patterns**");
    println!("      - Multiple producers add work");
    println!("      - Multiple consumers process work");
    println!("      - Example: Web server request handling");
    
    println!("\n   ❌ **When NOT to Use Queues:**");
    println!("      - Need to process most recent items first → Use Stack");
    println!("      - Need random access by index → Use Vec or Array");
    println!("      - Need to search/sort frequently → Use HashMap or BTreeMap");
    println!("      - Memory usage is critical → Consider ring buffer variant");
}

fn self_assessment() {
    println!("\n=== Self-Assessment Questions ===");
    println!("After completing Step 1, you should understand:");
    println!("1. What does FIFO mean and how does it differ from LIFO?");
    println!("2. What are the 5 core queue operations and their purposes?");
    println!("3. Can you name 3 real-world examples of queue behavior?");
    println!("4. When would you choose a queue over a stack?");
    println!("5. What happens to element order when processing with queues vs stacks?");
    
    println!("\n=== Real-World Applications ===");
    println!("✅ BFS algorithms for shortest path finding");
    println!("✅ Task scheduling in operating systems");  
    println!("✅ Handling network packet buffers");
    println!("✅ Managing print job queues");
    println!("✅ Processing user interface events");
    
    println!("\n=== Implementation Preview ===");
    println!("Next steps will cover:");
    println!("• Step 2: Simple array-based queue (and why it's inefficient)");
    println!("• Step 3: Ring buffer concepts and circular indexing");
    println!("• Step 4: Handling Rust ownership in circular buffers");
    println!("• Step 5-6: Linked queue implementation with pointers");
    println!("• Step 7: Production-ready implementations");
    
    println!("\n=== Key Concepts Mastered ===");
    println!("🎯 FIFO (First In, First Out) principle");
    println!("🎯 Queue operations: enqueue, dequeue, peek, len, is_empty");
    println!("🎯 Real-world queue analogies and applications");
    println!("🎯 Queue vs Stack behavioral differences");
    println!("🎯 When to choose queues for problem solving");
    
    println!("\nRun: cargo run --example step2_simple_array_queue");
}