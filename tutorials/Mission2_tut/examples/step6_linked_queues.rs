// Mission2_tut Step 6: Linked Queue Implementations
// Explore unbounded queues using linked structures and compare trade-offs

use std::collections::VecDeque;

fn main() {
    println!("=== Mission2_tut Step 6: Linked Queue Implementations ===\n");

    // 1. Linked vs Array-Based Trade-offs
    println!("1. Linked vs Array-Based Queue Trade-offs:");
    analyze_tradeoffs();

    // 2. Simple Linked Queue Implementation
    println!("\n2. Simple Linked Queue Implementation:");
    demonstrate_simple_linked_queue();

    // 3. Optimized Linked Queue with Tail Pointer
    println!("\n3. Optimized Linked Queue with Tail Pointer:");
    demonstrate_optimized_linked_queue();

    // 4. Memory Management and Ownership
    println!("\n4. Memory Management and Ownership:");
    demonstrate_memory_management();

    // 5. Performance Analysis
    println!("\n5. Performance Analysis and Benchmarking:");
    benchmark_implementations();

    // 6. When to Use Each Implementation
    println!("\n6. Implementation Selection Guide:");
    implementation_selection_guide();

    self_assessment();
}

fn analyze_tradeoffs() {
    println!("   ⚖️  **Array-Based vs Linked Queue Comparison:**");

    println!("\n   🏠 **Array-Based Queues (Ring Buffer):**");
    println!("      ✅ Advantages:");
    println!("      • Cache-friendly sequential memory access");
    println!("      • No per-element allocation overhead");
    println!("      • Predictable memory usage (bounded)");
    println!("      • Simple memory layout, fewer pointers to chase");
    println!("      • Better for real-time systems (no GC pressure)");
    println!("      • Vectorization opportunities for bulk operations");

    println!("\n      ❌ Disadvantages:");
    println!("      • Fixed capacity (bounded by design)");
    println!("      • May waste memory if capacity >> actual usage");
    println!("      • Requires knowing maximum size in advance");
    println!("      • Cannot shrink to reclaim unused memory");

    println!("\n   🔗 **Linked Queues:**");
    println!("      ✅ Advantages:");
    println!("      • Unbounded growth (limited only by available memory)");
    println!("      • Dynamic memory allocation matches actual usage");
    println!("      • Can grow and shrink as needed");
    println!("      • No need to predict maximum capacity");
    println!("      • Memory usage scales with content");

    println!("\n      ❌ Disadvantages:");
    println!("      • Cache-unfriendly (nodes scattered in memory)");
    println!("      • Per-element allocation overhead");
    println!("      • Additional memory for storing pointers");
    println!("      • Potential memory fragmentation");
    println!("      • Garbage collection pressure in managed languages");
    println!("      • More complex memory management");

    println!("\n   📊 **Performance Characteristics:**");
    println!("      Operation        | Array-Based  | Linked");
    println!("      -----------------|--------------|----------");
    println!("      Enqueue          | O(1)         | O(1)");
    println!("      Dequeue          | O(1)         | O(1)");
    println!("      Memory Access    | Sequential   | Random");
    println!("      Cache Performance| Excellent    | Poor");
    println!("      Memory Overhead  | None         | ~16-24 bytes/node");
    println!("      Allocation       | Once         | Per element");

    println!("\n   🧠 **Memory Layout Visualization:**");
    println!("      Array-Based Queue:");
    println!("      [A][B][C][_][_][_] (contiguous block)");
    println!("      ↑head     ↑tail");

    println!("\n      Linked Queue:");
    println!("      [A|*]→[B|*]→[C|∅]");
    println!("      ↑head        ↑tail");
    println!("      (nodes scattered throughout heap)");

    println!("\n   🎯 **Key Decision Factors:**");
    println!("      Choose Array-Based when:");
    println!("      • Known maximum capacity");
    println!("      • Performance is critical");
    println!("      • Memory usage predictability needed");
    println!("      • Real-time or embedded systems");

    println!("\n      Choose Linked when:");
    println!("      • Unknown or highly variable capacity");
    println!("      • Memory efficiency more important than speed");
    println!("      • Occasional very large queues acceptable");
    println!("      • Simplicity of not managing capacity");
}

fn demonstrate_simple_linked_queue() {
    println!("   🔗 **Simple Linked Queue Implementation:**");

    // Node structure
    #[derive(Debug)]
    struct Node<T> {
        data: T,
        next: Option<Box<Node<T>>>,
    }

    impl<T> Node<T> {
        fn new(data: T) -> Self {
            Node { data, next: None }
        }
    }

    // Simple linked queue (head pointer only)
    #[derive(Debug)]
    struct SimpleLinkedQueue<T> {
        head: Option<Box<Node<T>>>,
    }

    impl<T> SimpleLinkedQueue<T> {
        fn new() -> Self {
            SimpleLinkedQueue { head: None }
        }

        fn is_empty(&self) -> bool {
            self.head.is_none()
        }

        // Enqueue at the end (requires traversal - O(n)!)
        fn enqueue(&mut self, item: T) {
            let new_node = Box::new(Node::new(item));

            if self.head.is_none() {
                self.head = Some(new_node);
            } else {
                // Find the last node - this is O(n)!
                let mut current = self.head.as_mut().unwrap();
                while current.next.is_some() {
                    current = current.next.as_mut().unwrap();
                }
                current.next = Some(new_node);
            }
        }

        // Dequeue from the front - O(1)
        fn dequeue(&mut self) -> Option<T> {
            if let Some(old_head) = self.head.take() {
                self.head = old_head.next;
                Some(old_head.data)
            } else {
                None
            }
        }

        // Peek at front - O(1)
        fn peek(&self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.data)
        }

        fn len(&self) -> usize {
            let mut count = 0;
            let mut current = self.head.as_ref();
            while let Some(node) = current {
                count += 1;
                current = node.next.as_ref();
            }
            count
        }
    }

    println!("\n   🧪 **Testing Simple Linked Queue:**");

    let mut queue = SimpleLinkedQueue::new();
    println!("      Created empty queue: {:?}", queue);

    // Test basic operations
    println!("\n      Testing enqueue operations:");
    for item in ["First", "Second", "Third"] {
        queue.enqueue(item);
        println!("      Enqueued '{}', queue length: {}", item, queue.len());
    }

    println!("      Queue structure: {:?}", queue);

    // Test peek
    if let Some(front) = queue.peek() {
        println!("      Front item (peek): '{}'", front);
    }

    // Test dequeue
    println!("\n      Testing dequeue operations:");
    while let Some(item) = queue.dequeue() {
        println!(
            "      Dequeued: '{}', remaining length: {}",
            item,
            queue.len()
        );
    }

    println!("      Final queue state: {:?}", queue);

    println!("\n   ⚠️  **Critical Performance Issue:**");
    println!("      The enqueue operation is O(n) because we must traverse");
    println!("      the entire list to find the tail!");
    println!("      With 1000 items: 1000 pointer dereferences per enqueue");
    println!("      This makes the queue unusable for large datasets");

    // Demonstrate the performance problem
    println!("\n   📉 **Performance Demonstration:**");
    let mut slow_queue = SimpleLinkedQueue::new();

    let start = std::time::Instant::now();
    for i in 0..100 {
        slow_queue.enqueue(i);
    }
    let time_100 = start.elapsed();

    let start = std::time::Instant::now();
    for i in 0..10 {
        slow_queue.enqueue(i + 1000);
    }
    let time_10_more = start.elapsed();

    println!("      Time to add first 100 items: {:?}", time_100);
    println!("      Time to add 10 more items: {:?}", time_10_more);
    println!("      Notice: Later additions take longer due to O(n) traversal!");
}

fn demonstrate_optimized_linked_queue() {
    println!("   🚀 **Optimized Linked Queue with Tail Pointer:**");

    #[derive(Debug)]
    struct Node<T> {
        data: T,
        next: Option<Box<Node<T>>>,
    }

    impl<T> Node<T> {
        fn new(data: T) -> Self {
            Node { data, next: None }
        }
    }

    // Optimized linked queue with both head and tail pointers
    struct OptimizedLinkedQueue<T> {
        head: Option<Box<Node<T>>>,
        tail: *mut Node<T>, // Raw pointer for efficiency
        length: usize,
    }

    impl<T> OptimizedLinkedQueue<T> {
        fn new() -> Self {
            OptimizedLinkedQueue {
                head: None,
                tail: std::ptr::null_mut(),
                length: 0,
            }
        }

        fn is_empty(&self) -> bool {
            self.length == 0
        }

        fn len(&self) -> usize {
            self.length
        }

        // Enqueue at tail - O(1)
        fn enqueue(&mut self, item: T) {
            let new_node = Box::new(Node::new(item));
            let new_node_ptr = new_node.as_ref() as *const Node<T> as *mut Node<T>;

            if self.head.is_none() {
                // First node
                self.head = Some(new_node);
                self.tail = new_node_ptr;
            } else {
                // Add to tail
                unsafe {
                    (*self.tail).next = Some(new_node);
                    self.tail = new_node_ptr;
                }
            }
            self.length += 1;
        }

        // Dequeue from head - O(1)
        fn dequeue(&mut self) -> Option<T> {
            if let Some(old_head) = self.head.take() {
                self.head = old_head.next;
                self.length -= 1;

                if self.length == 0 {
                    self.tail = std::ptr::null_mut();
                }

                Some(old_head.data)
            } else {
                None
            }
        }

        fn peek(&self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.data)
        }
    }

    // Safe wrapper to avoid unsafe in public API
    impl<T> Drop for OptimizedLinkedQueue<T> {
        fn drop(&mut self) {
            while self.dequeue().is_some() {}
        }
    }

    // Debug implementation
    impl<T: std::fmt::Debug> std::fmt::Debug for OptimizedLinkedQueue<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::new();
            let mut current = self.head.as_ref();
            while let Some(node) = current {
                items.push(&node.data);
                current = node.next.as_ref();
            }
            f.debug_struct("OptimizedLinkedQueue")
                .field("length", &self.length)
                .field("items", &items)
                .finish()
        }
    }

    println!("\n   🧪 **Testing Optimized Linked Queue:**");

    let mut queue = OptimizedLinkedQueue::new();
    println!("      Created empty queue");

    // Test performance improvement
    println!("\n      Testing O(1) enqueue performance:");
    let start = std::time::Instant::now();

    for i in 0..1000 {
        queue.enqueue(i);
        if i % 100 == 99 {
            println!("      Added {} items, length: {}", i + 1, queue.len());
        }
    }

    let enqueue_time = start.elapsed();
    println!("      Total time for 1000 enqueues: {:?}", enqueue_time);

    // Test dequeue performance
    let start = std::time::Instant::now();
    let mut dequeued_count = 0;

    while queue.dequeue().is_some() {
        dequeued_count += 1;
    }

    let dequeue_time = start.elapsed();
    println!(
        "      Total time for {} dequeues: {:?}",
        dequeued_count, dequeue_time
    );

    println!("\n   ✅ **Performance Benefits:**");
    println!("      • Both enqueue and dequeue are now O(1)");
    println!("      • No traversal needed for enqueue operations");
    println!("      • Constant time regardless of queue size");
    println!("      • Tail pointer eliminates the performance bottleneck");

    println!("\n   ⚠️  **Unsafe Code Trade-offs:**");
    println!("      • Raw pointer usage requires careful memory management");
    println!("      • Must ensure tail pointer always valid or null");
    println!("      • Drop implementation needed for proper cleanup");
    println!("      • Less idiomatic Rust, but sometimes necessary for performance");

    // Alternative safe implementation using Rc<RefCell<>>
    println!("\n   🛡️  **Safe Alternative with Rc<RefCell<>>:**");

    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    struct SafeNode<T> {
        data: T,
        next: Option<Rc<RefCell<SafeNode<T>>>>,
    }

    #[derive(Debug)]
    struct SafeLinkedQueue<T> {
        head: Option<Rc<RefCell<SafeNode<T>>>>,
        tail: Option<Rc<RefCell<SafeNode<T>>>>,
        length: usize,
    }

    impl<T: std::fmt::Debug> SafeLinkedQueue<T> {
        fn new() -> Self {
            SafeLinkedQueue {
                head: None,
                tail: None,
                length: 0,
            }
        }

        fn enqueue(&mut self, item: T) {
            let new_node = Rc::new(RefCell::new(SafeNode {
                data: item,
                next: None,
            }));

            match self.tail.take() {
                Some(old_tail) => {
                    old_tail.borrow_mut().next = Some(new_node.clone());
                    self.tail = Some(new_node);
                }
                None => {
                    self.head = Some(new_node.clone());
                    self.tail = Some(new_node);
                }
            }
            self.length += 1;
        }

        fn dequeue(&mut self) -> Option<T> {
            self.head.take().map(|old_head| {
                match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        self.head = Some(new_head);
                    }
                    None => {
                        self.tail = None;
                    }
                }
                self.length -= 1;

                // Extract data from Rc<RefCell<>>
                Rc::try_unwrap(old_head).unwrap().into_inner().data
            })
        }

        fn len(&self) -> usize {
            self.length
        }
    }

    let mut safe_queue = SafeLinkedQueue::new();
    for i in 0..5 {
        safe_queue.enqueue(format!("item_{}", i));
    }
    println!("      Safe queue with {} items", safe_queue.len());

    while let Some(item) = safe_queue.dequeue() {
        println!("      Dequeued: {}", item);
    }

    println!("\n   🎯 **Safe vs Unsafe Trade-offs:**");
    println!("      Unsafe version:");
    println!("      ✅ Maximum performance");
    println!("      ✅ Minimal memory overhead");
    println!("      ❌ Risk of memory safety bugs");
    println!("      ❌ More complex implementation");

    println!("\n      Safe version (Rc<RefCell<>>):");
    println!("      ✅ Memory safe by construction");
    println!("      ✅ Easier to reason about");
    println!("      ❌ Reference counting overhead");
    println!("      ❌ Runtime borrow checking");
    println!("      ❌ Potential for panic if borrowing rules violated");
}

fn demonstrate_memory_management() {
    println!("   🧠 **Memory Management and Ownership Patterns:**");

    println!("\n   1️⃣  **Box<T> Ownership Model:**");
    println!("      • Each node owns the next node via Box<Node<T>>");
    println!("      • Ownership chain: head → node1 → node2 → ... → nodeN");
    println!("      • When queue is dropped, ownership chain ensures cleanup");
    println!("      • No reference counting overhead");

    #[derive(Debug)]
    struct Node<T> {
        data: T,
        next: Option<Box<Node<T>>>,
    }

    // Demonstrate ownership chain
    let node3 = Box::new(Node {
        data: "Third",
        next: None,
    });
    let node2 = Box::new(Node {
        data: "Second",
        next: Some(node3),
    });
    let node1 = Box::new(Node {
        data: "First",
        next: Some(node2),
    });

    println!("      Created ownership chain: {:?}", node1);
    println!("      When node1 is dropped, entire chain is recursively dropped");

    println!("\n   2️⃣  **Memory Layout Analysis:**");
    use std::mem;

    println!("      Size of i32: {} bytes", mem::size_of::<i32>());
    println!(
        "      Size of Box<Node<i32>>: {} bytes",
        mem::size_of::<Box<Node<i32>>>()
    );
    println!(
        "      Size of Node<i32>: {} bytes",
        mem::size_of::<Node<i32>>()
    );
    println!(
        "      Size of Option<Box<Node<i32>>>: {} bytes",
        mem::size_of::<Option<Box<Node<i32>>>>()
    );

    println!("\n      Memory overhead per node:");
    let data_size = mem::size_of::<i32>();
    let node_size = mem::size_of::<Node<i32>>();
    let overhead = node_size - data_size;
    println!(
        "      Data: {} bytes, Node: {} bytes, Overhead: {} bytes",
        data_size, node_size, overhead
    );
    println!(
        "      Overhead percentage: {:.1}%",
        (overhead as f64 / data_size as f64) * 100.0
    );

    println!("\n   3️⃣  **Allocation Patterns:**");
    println!("      Array-based queue:");
    println!("      • Single allocation for entire capacity");
    println!("      • Contiguous memory block");
    println!("      • No per-element allocation cost");

    println!("\n      Linked queue:");
    println!("      • One allocation per enqueue operation");
    println!("      • Scattered memory locations");
    println!("      • Potential for memory fragmentation");

    // Demonstrate allocation behavior
    println!("\n   📊 **Allocation Demonstration:**");

    #[derive(Debug)]
    struct SimpleQueue<T> {
        head: Option<Box<Node<T>>>,
        length: usize,
    }

    impl<T> SimpleQueue<T> {
        fn new() -> Self {
            SimpleQueue {
                head: None,
                length: 0,
            }
        }

        fn push(&mut self, item: T) {
            let new_node = Box::new(Node {
                data: item,
                next: self.head.take(),
            });
            self.head = Some(new_node);
            self.length += 1;
        }

        fn pop(&mut self) -> Option<T> {
            self.head.take().map(|old_head| {
                self.head = old_head.next;
                self.length -= 1;
                old_head.data
            })
        }

        fn len(&self) -> usize {
            self.length
        }
    }

    let mut queue = SimpleQueue::new();
    println!("      Initial queue length: {}", queue.len());

    // Each push allocates a new node
    for i in 1..=3 {
        queue.push(i);
        println!(
            "      After push({}): length = {} (allocation #{} occurred)",
            i,
            queue.len(),
            i
        );
    }

    // Each pop deallocates a node
    while let Some(value) = queue.pop() {
        println!(
            "      After pop(): value = {}, length = {} (deallocation occurred)",
            value,
            queue.len()
        );
    }

    println!("\n   4️⃣  **Memory Safety Guarantees:**");
    println!("      ✅ No use-after-free: Ownership prevents access to deallocated nodes");
    println!("      ✅ No double-free: Each Box<T> has exactly one owner");
    println!("      ✅ No memory leaks: Drop trait ensures recursive cleanup");
    println!("      ✅ No null pointer dereference: Option<T> forces explicit handling");
    println!("      ✅ Thread safety: Single ownership prevents data races");

    println!("\n   ⚠️  **Performance Implications:**");
    println!("      • Each allocation involves system call overhead");
    println!("      • Deallocation requires traversing free lists");
    println!("      • Cache misses from scattered memory layout");
    println!("      • Potential for heap fragmentation over time");
    println!("      • GC pressure in garbage-collected environments");
}

fn benchmark_implementations() {
    println!("   🏁 **Performance Analysis and Benchmarking:**");

    // Simple implementations for benchmarking
    struct RingQueue<T> {
        buffer: Vec<Option<T>>,
        head: usize,
        tail: usize,
        length: usize,
    }

    impl<T: Clone> RingQueue<T> {
        fn new(capacity: usize) -> Self {
            Self {
                buffer: vec![None; capacity],
                head: 0,
                tail: 0,
                length: 0,
            }
        }

        fn enqueue(&mut self, item: T) -> Result<(), T> {
            if self.length == self.buffer.len() {
                return Err(item);
            }
            self.buffer[self.tail] = Some(item);
            self.tail = (self.tail + 1) % self.buffer.len();
            self.length += 1;
            Ok(())
        }

        fn dequeue(&mut self) -> Option<T> {
            if self.length == 0 {
                return None;
            }
            let item = self.buffer[self.head].take().unwrap();
            self.head = (self.head + 1) % self.buffer.len();
            self.length -= 1;
            Some(item)
        }
    }

    #[derive(Debug)]
    struct Node<T> {
        data: T,
        next: Option<Box<Node<T>>>,
    }

    struct LinkedQueue<T> {
        head: Option<Box<Node<T>>>,
        tail: *mut Node<T>,
        length: usize,
    }

    impl<T> LinkedQueue<T> {
        fn new() -> Self {
            Self {
                head: None,
                tail: std::ptr::null_mut(),
                length: 0,
            }
        }

        fn enqueue(&mut self, item: T) {
            let new_node = Box::new(Node {
                data: item,
                next: None,
            });
            let new_node_ptr = new_node.as_ref() as *const Node<T> as *mut Node<T>;

            if self.head.is_none() {
                self.head = Some(new_node);
                self.tail = new_node_ptr;
            } else {
                unsafe {
                    (*self.tail).next = Some(new_node);
                    self.tail = new_node_ptr;
                }
            }
            self.length += 1;
        }

        fn dequeue(&mut self) -> Option<T> {
            if let Some(old_head) = self.head.take() {
                self.head = old_head.next;
                self.length -= 1;
                if self.length == 0 {
                    self.tail = std::ptr::null_mut();
                }
                Some(old_head.data)
            } else {
                None
            }
        }
    }

    impl<T> Drop for LinkedQueue<T> {
        fn drop(&mut self) {
            while self.dequeue().is_some() {}
        }
    }

    println!("\n   📊 **Benchmark Results:**");

    let iterations = 10_000;

    // Benchmark RingQueue
    println!("      Testing RingQueue (bounded):");
    let start = std::time::Instant::now();
    let mut ring = RingQueue::new(1000);

    for i in 0..iterations {
        if i % 1000 == 999 {
            // Drain some items to prevent filling
            for _ in 0..100 {
                let _ = ring.dequeue();
            }
        }
        let _ = ring.enqueue(i);
    }
    let ring_time = start.elapsed();
    println!("      RingQueue time: {:?}", ring_time);

    // Benchmark LinkedQueue
    println!("\n      Testing LinkedQueue (unbounded):");
    let start = std::time::Instant::now();
    let mut linked = LinkedQueue::new();

    for i in 0..iterations {
        linked.enqueue(i);
    }

    // Dequeue all items
    while linked.dequeue().is_some() {}

    let linked_time = start.elapsed();
    println!("      LinkedQueue time: {:?}", linked_time);

    // Benchmark VecDeque
    println!("\n      Testing VecDeque (standard library):");
    let start = std::time::Instant::now();
    let mut vec_deque = VecDeque::new();

    for i in 0..iterations {
        vec_deque.push_back(i);
    }

    while vec_deque.pop_front().is_some() {}

    let vecdeque_time = start.elapsed();
    println!("      VecDeque time: {:?}", vecdeque_time);

    // Compare results
    println!("\n   📈 **Performance Comparison:**");
    let ring_ns = ring_time.as_nanos() as f64;
    let linked_ns = linked_time.as_nanos() as f64;
    let vecdeque_ns = vecdeque_time.as_nanos() as f64;

    println!("      Relative performance (lower is better):");
    println!("      RingQueue: {:.2}x baseline", 1.0);
    println!("      LinkedQueue: {:.2}x baseline", linked_ns / ring_ns);
    println!("      VecDeque: {:.2}x baseline", vecdeque_ns / ring_ns);

    println!("\n   🎯 **Analysis:**");
    if linked_ns > ring_ns * 2.0 {
        println!("      LinkedQueue significantly slower due to:");
        println!("      • Per-element allocation overhead");
        println!("      • Cache misses from scattered memory");
        println!("      • Pointer dereferencing costs");
    } else {
        println!("      Performance difference smaller than expected");
        println!("      • Small dataset may not show cache effects");
        println!("      • Modern allocators are quite efficient");
    }

    println!("\n   💾 **Memory Usage Analysis:**");
    use std::mem;

    let ring_node_size = mem::size_of::<Option<i32>>();
    let linked_node_size = mem::size_of::<Node<i32>>();

    println!("      Per-element memory usage:");
    println!("      RingQueue: {} bytes per slot", ring_node_size);
    println!("      LinkedQueue: {} bytes per node", linked_node_size);

    let overhead = linked_node_size as f64 / ring_node_size as f64;
    println!(
        "      LinkedQueue overhead: {:.1}x more memory per element",
        overhead
    );

    println!("\n      For 1000 elements:");
    println!("      RingQueue: {} KB", (ring_node_size * 1000) / 1024);
    println!("      LinkedQueue: {} KB", (linked_node_size * 1000) / 1024);
}

fn implementation_selection_guide() {
    println!("   🎯 **Implementation Selection Guide:**");

    println!("\n   📋 **Decision Matrix:**");
    println!("      | Requirement | RingQueue | LinkedQueue | VecDeque |");
    println!("      |-------------|-----------|-------------|----------|");
    println!("      | Bounded size| ✅ Perfect | ❌ No       | ⚠️ Manual |");
    println!("      | Unbounded   | ❌ No      | ✅ Perfect  | ✅ Perfect|");
    println!("      | Performance | 🚀 Fast    | 🐌 Slower   | ⚡ Fast   |");
    println!("      | Memory usage| 💚 Low     | 💛 Higher   | 💚 Low    |");
    println!("      | Predictable | ✅ Yes     | ❌ No       | ⚠️ Mostly |");
    println!("      | Real-time   | ✅ Great   | ❌ Poor     | ⚠️ Good   |");

    println!("\n   🎮 **Use Case Recommendations:**");

    println!("\n   🎯 **Choose RingQueue (Array-Based) when:**");
    println!("      ✅ Real-time systems (audio, games, embedded)");
    println!("      ✅ Known maximum capacity requirements");
    println!("      ✅ Performance is critical");
    println!("      ✅ Memory usage must be predictable");
    println!("      ✅ Producer-consumer with bounded buffering");

    println!("      Example scenarios:");
    println!("      • Audio sample buffers (fixed sample rate)");
    println!("      • Network packet queues (MTU limits)");
    println!("      • Game engine message queues");
    println!("      • Real-time data acquisition systems");

    println!("\n   🔗 **Choose LinkedQueue when:**");
    println!("      ✅ Capacity requirements unknown or highly variable");
    println!("      ✅ Memory efficiency more important than speed");
    println!("      ✅ Occasional very large queues are acceptable");
    println!("      ✅ Simplicity of unbounded growth desired");

    println!("      Example scenarios:");
    println!("      • Task queues with unpredictable load");
    println!("      • Event processing systems");
    println!("      • Message passing between threads");
    println!("      • Breadth-first search algorithms");

    println!("\n   📚 **Choose VecDeque (Standard Library) when:**");
    println!("      ✅ General-purpose queue needs");
    println!("      ✅ Want battle-tested implementation");
    println!("      ✅ Need rich API (insert, remove at arbitrary positions)");
    println!("      ✅ Don't want to maintain custom implementation");

    println!("      Example scenarios:");
    println!("      • Most application-level queueing needs");
    println!("      • Prototyping and development");
    println!("      • When unsure about requirements");
    println!("      • Need deque operations (both ends)");

    println!("\n   🔬 **Hybrid Approaches:**");
    println!("      Sometimes the best solution combines approaches:");

    println!("\n      🎯 **Chunked Queue:**");
    println!("      • LinkedQueue of fixed-size arrays");
    println!("      • Better cache locality than pure linked");
    println!("      • Less allocation overhead than per-element");

    println!("\n      🎯 **Bounded VecDeque:**");
    println!("      • VecDeque with capacity checking wrapper");
    println!("      • Get VecDeque performance with bounded guarantees");
    println!("      • Custom policy for handling overflow");

    println!("\n      🎯 **Segmented Queue:**");
    println!("      • Multiple RingQueues in sequence");
    println!("      • Fixed memory per segment with overflow handling");
    println!("      • Balance between bounded and unbounded");

    println!("\n   ⚖️  **Summary Decision Tree:**");
    println!("      1. Do you know maximum capacity? → Yes: Consider RingQueue");
    println!("      2. Is performance critical? → Yes: RingQueue or VecDeque");
    println!("      3. Is memory predictability required? → Yes: RingQueue");
    println!("      4. Need unbounded growth? → Yes: LinkedQueue or VecDeque");
    println!("      5. Want simplicity? → Yes: VecDeque");
    println!("      6. Default choice: VecDeque (battle-tested, feature-complete)");
}

fn self_assessment() {
    println!("\n=== Self-Assessment Questions ===");
    println!("After completing Step 6, you should understand:");
    println!("1. What are the key trade-offs between array-based and linked queues?");
    println!("2. Why does a naive linked queue have O(n) enqueue operations?");
    println!("3. How does a tail pointer optimize linked queue performance?");
    println!("4. What are the memory overhead implications of linked structures?");
    println!("5. When should you choose each type of queue implementation?");

    println!("\n=== Implementation Trade-offs ===");
    println!("🎯 Array-based: Fixed capacity, excellent performance, predictable memory");
    println!("🎯 Linked: Unbounded growth, allocation overhead, scattered memory");
    println!("🎯 Hybrid approaches can combine benefits of both");
    println!("🎯 Standard library (VecDeque) often best default choice");
    println!("🎯 Custom implementations when specific requirements not met");

    println!("\n=== Performance Characteristics ===");
    println!("⚡ Array-based: O(1) operations, cache-friendly, minimal overhead");
    println!("⚡ Linked: O(1) operations, cache-unfriendly, allocation overhead");
    println!("⚡ Memory usage: Array has fixed cost, linked scales with content");
    println!("⚡ Allocation patterns: Array is bulk, linked is per-element");
    println!("⚡ Real-time suitability: Array excellent, linked problematic");

    println!("\n=== Memory Management ===");
    println!("💾 Box<T> provides ownership-based memory management");
    println!("💾 Automatic cleanup through Drop trait implementation");
    println!("💾 Memory overhead from pointers and allocation metadata");
    println!("💾 Potential fragmentation from scattered allocations");
    println!("💾 Safe alternatives like Rc<RefCell<>> have runtime costs");

    println!("\n=== Implementation Selection ===");
    println!("🎯 Choose based on specific requirements and constraints");
    println!("🎯 Consider performance, memory, and growth characteristics");
    println!("🎯 Standard library often provides good general solution");
    println!("🎯 Custom implementations for specialized needs");
    println!("🎯 Benchmarking important for performance-critical applications");

    println!("\n=== Implementation Readiness ===");
    println!("Final step: Integration with Mission2 and production use");
    println!("• Step 7: Mission2 integration and real-world applications");
    println!("• Connect tutorial learning to main Mission2 implementation");
    println!("• Apply knowledge to solve actual problems");

    println!("\nRun: cargo run --example step7_mission_integration");
}
