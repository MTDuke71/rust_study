// Mission1_tut Step 5: Dynamic Growth and Performance
// Understand Vec backing storage, amortized complexity, and performance optimization

use std::time::Instant;

fn main() {
    println!("=== Mission1_tut Step 5: Dynamic Growth and Performance ===\n");

    // 1. Understanding Vec Backing Storage
    println!("1. Vec Backing Storage and Capacity:");
    explain_vec_internals();

    // 2. Amortized Complexity Analysis
    println!("\n2. Amortized Complexity:");
    demonstrate_amortized_complexity();

    // 3. Performance Measurements
    println!("\n3. Performance Measurements:");
    measure_stack_performance();

    // 4. Memory Layout and Cache Efficiency
    println!("\n4. Memory Layout and Cache Efficiency:");
    explain_memory_layout();

    // 5. Performance Optimization Strategies
    println!("\n5. Optimization Strategies:");
    demonstrate_optimizations();

    self_assessment();
}

/// Stack with capacity tracking for performance analysis
#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    /// Create stack with pre-allocated capacity
    fn with_capacity(capacity: usize) -> Self {
        Stack {
            items: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    #[allow(dead_code)]
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get current capacity (allocated memory)
    fn capacity(&self) -> usize {
        self.items.capacity()
    }

    /// Reserve additional capacity
    fn reserve(&mut self, additional: usize) {
        self.items.reserve(additional);
    }

    /// Shrink capacity to fit current length
    fn shrink_to_fit(&mut self) {
        self.items.shrink_to_fit();
    }

    /// Efficient way to extend stack
    #[allow(dead_code)]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.items.extend(iter);
    }
}

fn explain_vec_internals() {
    println!("   Vec<T> Internal Structure:");
    println!("   • Pointer to heap-allocated array");
    println!("   • Length: number of elements currently stored");
    println!("   • Capacity: total allocated space for elements");

    println!("\n   Memory Layout:");
    println!("   Stack Frame:     Heap Memory:");
    println!("   ┌─────────┐      ┌─┬─┬─┬─┬─┬─┬─┬─┐");
    println!("   │ ptr     │────▶ │1│2│3│ │ │ │ │ │");
    println!("   │ len: 3  │      └─┴─┴─┴─┴─┴─┴─┴─┘");
    println!("   │ cap: 8  │      ↑─len─↑ ↑─cap─↑");
    println!("   └─────────┘");

    let mut stack: Stack<i32> = Stack::new();
    println!("\n   Capacity Growth Demonstration:");
    println!(
        "   Initial - len: {}, cap: {}",
        stack.len(),
        stack.capacity()
    );

    // Watch capacity grow as we add items
    for i in 1..=10 {
        let old_cap = stack.capacity();
        stack.push(i);
        let new_cap = stack.capacity();

        if new_cap != old_cap {
            println!(
                "   After push({}): len: {}, cap: {} (REALLOCATION!)",
                i,
                stack.len(),
                stack.capacity()
            );
        }
    }

    println!("   Final - len: {}, cap: {}", stack.len(), stack.capacity());
}

fn demonstrate_amortized_complexity() {
    println!("   Amortized O(1) Analysis:");
    println!("   • Most push() operations are O(1) - just write to existing space");
    println!("   • Occasionally push() is O(n) - must reallocate and copy all elements");
    println!("   • Amortized analysis spreads the cost of expensive operations");

    println!("\n   Growth Strategy (typical Vec implementation):");
    println!("   • Start with capacity 0");
    println!("   • First allocation: capacity 4");
    println!("   • When full, double capacity: 4 → 8 → 16 → 32 → ...");
    println!("   • This ensures amortized O(1) push operations");

    analyze_growth_pattern();
}

fn analyze_growth_pattern() {
    println!("\n   Detailed Growth Pattern Analysis:");

    let mut stack: Stack<i32> = Stack::new();
    let mut total_copies = 0;
    let mut reallocations = 0;

    for i in 1..=20 {
        let old_cap = stack.capacity();
        stack.push(i);
        let new_cap = stack.capacity();

        if new_cap > old_cap {
            reallocations += 1;
            // When reallocating, all existing elements are copied
            total_copies += stack.len() - 1; // -1 because new element wasn't copied
            println!(
                "   Reallocation #{}: {} → {} (copied {} elements)",
                reallocations,
                old_cap,
                new_cap,
                stack.len() - 1
            );
        }
    }

    println!("\n   Summary for {} push operations:", 20);
    println!("   • Reallocations: {}", reallocations);
    println!("   • Total element copies: {}", total_copies);
    println!(
        "   • Average copies per push: {:.2}",
        total_copies as f64 / 20.0
    );
    println!("   • This demonstrates why it's 'amortized' O(1)");
}

fn measure_stack_performance() {
    println!("   Performance Measurement:");

    // Measure push performance without pre-allocation
    let start = Instant::now();
    let mut stack1: Stack<i32> = Stack::new();
    for i in 0..100_000 {
        stack1.push(i);
    }
    let duration1 = start.elapsed();

    // Measure push performance with pre-allocation
    let start = Instant::now();
    let mut stack2: Stack<i32> = Stack::with_capacity(100_000);
    for i in 0..100_000 {
        stack2.push(i);
    }
    let duration2 = start.elapsed();

    println!("   100,000 pushes without pre-allocation: {:?}", duration1);
    println!("   100,000 pushes with pre-allocation:    {:?}", duration2);
    println!(
        "   Speedup factor: {:.2}x",
        duration1.as_nanos() as f64 / duration2.as_nanos() as f64
    );

    // Measure pop performance
    let start = Instant::now();
    while stack1.pop().is_some() {}
    let pop_duration = start.elapsed();
    println!("   100,000 pops: {:?}", pop_duration);

    demonstrate_cache_effects();
}

fn demonstrate_cache_effects() {
    println!("\n   Cache Efficiency Comparison:");

    // Stack operations (good cache locality)
    let start = Instant::now();
    let mut stack: Stack<i32> = Stack::with_capacity(10_000);
    for i in 0..10_000 {
        stack.push(i);
    }
    // Access elements in LIFO order (good cache locality)
    let mut sum = 0;
    while let Some(value) = stack.pop() {
        sum += value;
    }
    let stack_duration = start.elapsed();

    // Simulate linked list operations (poor cache locality)
    let start = Instant::now();
    let mut vec: Vec<i32> = Vec::with_capacity(10_000);
    for i in 0..10_000 {
        vec.push(i);
    }
    // Access elements in random order (simulates pointer chasing)
    let mut sum2 = 0;
    for i in (0..vec.len()).rev() {
        sum2 += vec[i]; // Similar computation, different access pattern
    }
    let vec_duration = start.elapsed();

    println!("   Stack LIFO access: {:?} (sum: {})", stack_duration, sum);
    println!("   Vec reverse access: {:?} (sum: {})", vec_duration, sum2);

    if sum == sum2 {
        println!("   ✅ Same computational work, similar performance due to Vec backing");
    }
}

fn explain_memory_layout() {
    println!("   Why Vec<T> is Perfect for Stack:");

    println!("\n   ✅ Contiguous Memory Layout:");
    println!("   [item1][item2][item3][item4]...");
    println!("   • CPU cache loads adjacent elements automatically");
    println!("   • Predictable memory access patterns");
    println!("   • Branch predictor works well with simple indexing");

    println!("\n   ✅ Efficient Stack Operations:");
    println!("   • push(): Write to known location (items[len]), increment len");
    println!("   • pop(): Decrement len, return items[len] (no memory copy needed)");
    println!("   • peek(): Read items[len-1] (no bounds checking in release mode)");

    println!("\n   🔄 Compared to Linked List:");
    println!("   Linked List: [data|ptr] → [data|ptr] → [data|ptr]");
    println!("   • Each access requires pointer dereference");
    println!("   • Poor cache locality (nodes scattered in memory)");
    println!("   • Extra memory overhead for storing pointers");
    println!("   • More complex memory management");

    demonstrate_memory_overhead();
}

fn demonstrate_memory_overhead() {
    println!("\n   Memory Overhead Analysis:");

    // Stack with Vec<T> backing
    let stack: Stack<i32> = Stack::with_capacity(1000);
    let vec_overhead = std::mem::size_of::<Vec<i32>>(); // Just ptr + len + cap
    let element_space = stack.capacity() * std::mem::size_of::<i32>();

    println!("   Vec-based Stack:");
    println!("   • Control structure: {} bytes", vec_overhead);
    println!("   • Element storage (1000 i32s): {} bytes", element_space);
    println!("   • Total: {} bytes", vec_overhead + element_space);
    println!(
        "   • Overhead per element: {:.2} bytes",
        vec_overhead as f64 / 1000.0
    );

    // Simulate linked list overhead
    struct ListNode {
        _data: i32,
        _next: Option<Box<ListNode>>,
    }
    let node_size = std::mem::size_of::<ListNode>();
    let list_total = 1000 * node_size;

    println!("\n   Linked List Stack (simulated):");
    println!(
        "   • Per-node overhead: {} bytes (includes pointer)",
        node_size
    );
    println!("   • Total for 1000 nodes: {} bytes", list_total);
    println!(
        "   • Overhead per element: {:.2} bytes",
        (node_size - 4) as f64
    );

    let efficiency = (vec_overhead + element_space) as f64 / list_total as f64;
    println!("   • Vec is {:.1}x more memory efficient", 1.0 / efficiency);
}

fn demonstrate_optimizations() {
    println!("   Performance Optimization Strategies:");

    // 1. Pre-allocation
    println!("\n   1. Pre-allocation Strategy:");
    fn create_optimized_stack(expected_size: usize) -> Stack<i32> {
        let stack = Stack::with_capacity(expected_size);
        println!("   Pre-allocated capacity: {}", stack.capacity());
        stack
    }

    let _optimized = create_optimized_stack(1000);

    // 2. Reserve additional capacity
    println!("\n   2. Reserve Strategy:");
    let mut stack: Stack<i32> = Stack::new();
    println!("   Initial capacity: {}", stack.capacity());
    stack.reserve(500); // Reserve space for 500 more elements
    println!("   After reserve(500): {}", stack.capacity());

    // 3. Shrink when appropriate
    println!("\n   3. Memory Management:");
    let mut stack: Stack<i32> = Stack::with_capacity(1000);
    for i in 0..100 {
        stack.push(i);
    }
    println!(
        "   After 100 pushes - len: {}, cap: {}",
        stack.len(),
        stack.capacity()
    );

    stack.shrink_to_fit();
    println!(
        "   After shrink_to_fit - len: {}, cap: {}",
        stack.len(),
        stack.capacity()
    );

    demonstrate_bulk_operations();
}

fn demonstrate_bulk_operations() {
    println!("\n   Bulk Operations Optimization:");

    // Efficient way to create stack from iterator
    fn from_iter<I>(iter: I) -> Stack<i32>
    where
        I: IntoIterator<Item = i32>,
    {
        let items: Vec<i32> = iter.into_iter().collect();
        Stack { items }
    }

    let stack = from_iter(1..=5);
    println!("   Created stack from iterator: {:?}", stack);

    // Efficient way to extend stack

    let mut stack: Stack<i32> = Stack::new();
    stack.extend(vec![1, 2, 3, 4, 5]);
    println!("   Extended stack efficiently: len = {}", stack.len());
}

fn self_assessment() {
    println!("\n=== Self-Assessment Questions ===");
    println!("After completing Step 5, you should understand:");
    println!("1. What's the difference between Vec length and capacity?");
    println!("2. Why is push() O(1) amortized instead of O(1) worst-case?");
    println!("3. When should you use with_capacity() vs new()?");
    println!("4. Why is Vec<T> more cache-efficient than linked lists?");
    println!("5. What memory overhead does Vec<T> have compared to alternatives?");

    println!("\n=== Performance Best Practices ===");
    println!("✅ Pre-allocate capacity when size is known");
    println!("✅ Use reserve() when planning to add many elements");
    println!("✅ Consider shrink_to_fit() after removing many elements");
    println!("✅ Leverage bulk operations like extend() when possible");
    println!("✅ Measure actual performance in your specific use case");

    println!("\n=== Key Insights ===");
    println!("• Vec doubling strategy ensures amortized O(1) push");
    println!("• Contiguous memory layout provides excellent cache performance");
    println!("• Stack operations map naturally to Vec's end-access patterns");
    println!("• Memory overhead is minimal compared to pointer-based alternatives");

    println!("\n=== Next Step Preview ===");
    println!("Step 6 will explore real-world applications of stacks:");
    println!("• Bracket and parentheses validation");
    println!("• Expression parsing and evaluation");
    println!("• Undo/redo systems implementation");
    println!("• Function call simulation");
    println!("Run: cargo run --example step6_real_world_applications");
}
