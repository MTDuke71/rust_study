// Mission2_tut Step 2: Simple Array Queue
// Understanding why naive array approaches don't scale

fn main() {
    println!("=== Mission2_tut Step 2: Simple Array Queue ===\n");
    
    // 1. Naive Array Queue Implementation
    println!("1. Naive Array-Based Queue:");
    demonstrate_naive_queue();
    
    // 2. Performance Problems with Shifting
    println!("\n2. Performance Problems:");
    analyze_performance_issues();
    
    // 3. Memory Usage Analysis
    println!("\n3. Memory Usage Issues:");
    demonstrate_memory_waste();
    
    // 4. Alternative Approaches Preview
    println!("\n4. Why We Need Better Solutions:");
    preview_better_approaches();
    
    self_assessment();
}

/// Simple but inefficient queue using Vec with shifting
#[derive(Debug)]
struct SimpleArrayQueue<T> {
    items: Vec<T>,
}

impl<T> SimpleArrayQueue<T> {
    /// Create a new empty queue
    fn new() -> Self {
        SimpleArrayQueue {
            items: Vec::new(),
        }
    }
    
    /// Add an item to the back of the queue
    /// Time Complexity: O(1) - just push to end of Vec
    fn enqueue(&mut self, item: T) {
        self.items.push(item);
        println!("   Enqueued item, queue length: {}", self.items.len());
    }
    
    /// Remove and return item from front of queue
    /// Time Complexity: O(n) - must shift all remaining elements!
    fn dequeue(&mut self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            let item = self.items.remove(0); // Remove from index 0
            println!("   Dequeued item, {} items shifted left", self.items.len());
            Some(item)
        }
    }
    
    /// Look at the front item without removing it
    /// Time Complexity: O(1) - just index access
    fn peek(&self) -> Option<&T> {
        self.items.first()
    }
    
    /// Get the number of items in the queue
    fn len(&self) -> usize {
        self.items.len()
    }
    
    /// Check if the queue is empty
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    /// Show the internal state for educational purposes
    fn debug_state(&self) {
        println!("   Internal Vec: {:?}", self.items);
        println!("   Capacity: {}, Length: {}", self.items.capacity(), self.items.len());
    }
}

fn demonstrate_naive_queue() {
    println!("   Creating a simple array-based queue...");
    
    let mut queue: SimpleArrayQueue<&str> = SimpleArrayQueue::new();
    
    // Add some items
    println!("\n   📥 Adding items to queue:");
    queue.enqueue("First Task");
    queue.enqueue("Second Task");
    queue.enqueue("Third Task");
    queue.enqueue("Fourth Task");
    
    queue.debug_state();
    
    // Look at front item
    if let Some(front) = queue.peek() {
        println!("\n   👀 Front item: '{}'", front);
    }
    
    // Remove items and observe shifting
    println!("\n   📤 Removing items (watch the shifting!):");
    
    while let Some(item) = queue.dequeue() {
        println!("   Processed: '{}'", item);
        queue.debug_state();
        println!(); // Empty line for readability
    }
    
    println!("   Queue is now empty: {}", queue.is_empty());
}

fn analyze_performance_issues() {
    println!("   🐌 **The Shifting Problem:**");
    println!("      - dequeue() removes item at index 0");
    println!("      - All remaining items must shift left by 1 position");
    println!("      - Vec::remove(0) is O(n) operation!");
    
    println!("\n   📊 **Performance Analysis:**");
    
    // Demonstrate with different queue sizes
    let sizes = vec![10, 100, 1000, 10000];
    
    for size in sizes {
        println!("\n   Queue with {} items:", size);
        
        let mut queue: SimpleArrayQueue<i32> = SimpleArrayQueue::new();
        
        // Fill the queue
        for i in 0..size {
            queue.enqueue(i);
        }
        
        // Time how long it takes to dequeue everything
        let start = std::time::Instant::now();
        let mut operations = 0;
        
        while !queue.is_empty() {
            queue.dequeue();
            operations += 1;
        }
        
        let duration = start.elapsed();
        
        println!("      {} dequeue operations took: {:?}", operations, duration);
        println!("      Average per operation: {:?}", duration / operations);
        
        // Calculate total shift operations
        let total_shifts = (size * (size - 1)) / 2; // Sum of 0 + 1 + 2 + ... + (n-1)
        println!("      Total element shifts: {}", total_shifts);
    }
    
    println!("\n   🎯 **Key Insight:**");
    println!("      - Each dequeue requires shifting (n-1) elements");
    println!("      - Total work: O(n²) to empty an n-element queue");
    println!("      - Unacceptable for large queues!");
}

fn demonstrate_memory_waste() {
    println!("   💾 **Memory Usage Problems:**");
    
    let mut queue: SimpleArrayQueue<String> = SimpleArrayQueue::new();
    
    // Add many items to grow the Vec
    println!("\n   Adding 1000 items...");
    for i in 0..1000 {
        queue.enqueue(format!("Item {}", i));
    }
    
    println!("   Capacity after adding 1000 items: {}", queue.items.capacity());
    println!("   Length: {}", queue.len());
    
    // Remove most items
    println!("\n   Removing 950 items...");
    for _ in 0..950 {
        queue.dequeue();
    }
    
    println!("   Capacity after removing 950 items: {}", queue.items.capacity());
    println!("   Length: {}", queue.len());
    
    println!("\n   🚨 **The Memory Waste Problem:**");
    println!("      - Vec grew to hold 1000 items (capacity ~1000+)");
    println!("      - Only 50 items remain, but capacity stays large");
    println!("      - Wasted memory = {} slots", queue.items.capacity() - queue.len());
    println!("      - Vec::remove() doesn't shrink capacity");
    
    println!("\n   💡 **Additional Issues:**");
    println!("      - No way to set maximum capacity");
    println!("      - Can't implement bounded queues easily");
    println!("      - Memory usage grows indefinitely");
    println!("      - Cache performance suffers from shifting");
}

fn preview_better_approaches() {
    println!("   🎯 **Why We Need Better Solutions:**");
    
    println!("\n   ❌ **Problems with Simple Array Queue:**");
    println!("      1. O(n) dequeue due to element shifting");
    println!("      2. O(n²) total time to process n items");
    println!("      3. Memory waste - capacity never shrinks");
    println!("      4. No bounded capacity control");
    println!("      5. Poor cache performance from shifting");
    
    println!("\n   ✅ **Solution 1: Ring Buffer (Circular Array)**");
    println!("      - Use head/tail pointers instead of shifting");
    println!("      - Wrap around when reaching end of array");
    println!("      - Fixed capacity, no memory waste");
    println!("      - O(1) enqueue and dequeue operations");
    println!("      - Great for bounded queues");
    
    println!("\n   ✅ **Solution 2: Linked Queue**");
    println!("      - Use linked nodes instead of array");
    println!("      - Head pointer for dequeue, tail pointer for enqueue");
    println!("      - No shifting needed, just pointer updates");
    println!("      - Dynamic size, grows/shrinks as needed");
    println!("      - O(1) operations with no capacity limit");
    
    println!("\n   📊 **Performance Comparison Preview:**");
    println!("      Operation      | Simple Array | Ring Buffer | Linked Queue");
    println!("      enqueue()      |     O(1)     |     O(1)    |     O(1)");
    println!("      dequeue()      |     O(n)     |     O(1)    |     O(1)");
    println!("      peek()         |     O(1)     |     O(1)    |     O(1)");
    println!("      Memory         |   Wasteful   |   Fixed     |   Dynamic");
    println!("      Cache locality |     Poor     |    Good     |     Fair");
    
    println!("\n   🎮 **Real-World Impact:**");
    println!("      - Game with 60 FPS needs fast queue operations");
    println!("      - O(n) dequeue → frame drops and lag");
    println!("      - Ring buffer → smooth 60 FPS performance");
    println!("      - Web server with 1000s of requests → needs O(1) operations");
}

fn self_assessment() {
    println!("\n=== Self-Assessment Questions ===");
    println!("After completing Step 2, you should understand:");
    println!("1. Why is Vec::remove(0) an O(n) operation?");
    println!("2. What happens to other elements when you remove the first element?");
    println!("3. Why does processing n items take O(n²) time with simple array queue?");
    println!("4. What memory problems occur with this approach?");
    println!("5. What are the two main alternatives to solve these problems?");
    
    println!("\n=== Performance Impact ===");
    println!("Understanding the problems with naive approaches:");
    println!("🐌 O(n) dequeue operations due to element shifting");
    println!("🐌 O(n²) total time to empty a queue of n items");
    println!("💾 Memory waste - capacity grows but never shrinks");
    println!("🚫 No way to implement bounded/fixed-capacity queues");
    println!("📉 Poor cache performance from constant memory movement");
    
    println!("\n=== Solution Preview ===");
    println!("Next steps will implement efficient alternatives:");
    println!("• Step 3: Ring Buffer - circular indexing with wrap-around");
    println!("• Step 4: Option<T> handling for safe element removal");
    println!("• Step 5: Linked Queue - pointer-based implementation");
    println!("• Both achieve O(1) operations without shifting!");
    
    println!("\n=== Key Learning Outcomes ===");
    println!("🎯 Understanding why naive approaches fail at scale");
    println!("🎯 Recognizing O(n) vs O(1) operation importance");
    println!("🎯 Memory efficiency considerations in data structures");
    println!("🎯 The motivation for more sophisticated queue implementations");
    
    println!("\n=== Algorithm Complexity Insights ===");
    println!("📈 Simple Array Queue Complexity:");
    println!("   • enqueue(): O(1) - just Vec::push()");
    println!("   • dequeue(): O(n) - Vec::remove(0) shifts all elements");
    println!("   • peek(): O(1) - just Vec::first()");
    println!("   • Total to process n items: O(n²)");
    
    println!("\nNext: Learn how ring buffers solve these problems!");
    println!("Run: cargo run --example step3_ring_buffer_basics");
}