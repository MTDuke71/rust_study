// Mission2_tut Step 3: Ring Buffer Basics
// Master circular indexing and wrap-around logic

fn main() {
    println!("=== Mission2_tut Step 3: Ring Buffer Fundamentals ===\n");

    // 1. Ring Buffer Concept
    println!("1. Ring Buffer Concept:");
    explain_ring_buffer_concept();

    // 2. Circular Indexing with Modulo
    println!("\n2. Circular Indexing Mathematics:");
    demonstrate_modulo_arithmetic();

    // 3. Head and Tail Pointers
    println!("\n3. Head and Tail Pointer Management:");
    demonstrate_head_tail_pointers();

    // 4. Wrap-Around Behavior
    println!("\n4. Wrap-Around in Action:");
    demonstrate_wraparound();

    // 5. Full vs Empty Detection
    println!("\n5. Detecting Full vs Empty States:");
    demonstrate_full_empty_detection();

    self_assessment();
}

fn explain_ring_buffer_concept() {
    println!("   🎠 **Ring Buffer Mental Model:**");
    println!("      Think of a circular carousel with numbered seats:");
    println!("      - Seats are numbered 0, 1, 2, 3, 4, 5, ..., capacity-1");
    println!("      - After the last seat, you wrap back to seat 0");
    println!("      - 'head' points to where next person gets off");
    println!("      - 'tail' points to where next person sits down");

    println!("\n   🎯 **Key Advantages over Simple Array:**");
    println!("      ✅ No element shifting needed");
    println!("      ✅ O(1) enqueue and dequeue operations");
    println!("      ✅ Fixed memory usage - no growth");
    println!("      ✅ Bounded capacity by design");
    println!("      ✅ Cache-friendly access patterns");

    println!("\n   🔄 **The Circular Property:**");
    println!("      - Array indices: [0] [1] [2] [3] [4]");
    println!("      - Conceptually:     ↑               ↓");
    println!("                       [0] ← [4] ← [3] ← [2]");
    println!("                        ↓               ↑");
    println!("                       [1] → [2] → [3] → [4] → wraps to [0]");

    // Visual demonstration with small array
    println!("\n   📍 **Visual Example (capacity=5):**");
    let capacity = 5;
    let positions = (0..capacity).collect::<Vec<_>>();

    println!("      Array: {:?}", positions);
    println!("      Index:  0   1   2   3   4");
    println!("              ↑                 ↓");
    println!("              └─────wrap────────┘");

    println!("\n   🔢 **Index Wrapping Formula:**");
    println!("      next_index = (current_index + 1) % capacity");
    println!("      Examples with capacity=5:");
    for i in 0..8 {
        let next = (i + 1) % capacity;
        let wraps = if i == capacity - 1 { " (wraps!)" } else { "" };
        println!("      {} → {}{}", i, next, wraps);
    }
}

fn demonstrate_modulo_arithmetic() {
    println!("   🧮 **Modulo Arithmetic for Circular Indexing:**");

    let capacity = 5;
    println!("      Ring buffer capacity: {}", capacity);

    println!("\n   ➡️  **Forward Movement (increment):**");
    println!("      Formula: (index + 1) % capacity");

    let mut current = 0;
    for step in 0..10 {
        println!(
            "      Step {}: index {} → next index {}",
            step,
            current,
            (current + 1) % capacity
        );
        current = (current + 1) % capacity;
    }

    println!("\n   ⬅️  **Backward Movement (decrement - trickier!):**");
    println!("      Formula: (index + capacity - 1) % capacity");
    println!("      Why add capacity? Prevents negative numbers!");

    current = 0;
    for step in 0..8 {
        let prev = (current + capacity - 1) % capacity;
        println!(
            "      Step {}: index {} → prev index {}",
            step, current, prev
        );
        current = prev;
    }

    println!("\n   🎯 **Why (index - 1) % capacity doesn't work:**");
    println!("      In Rust, negative numbers % positive = negative!");
    println!("      (-1) % 5 = -1, not 4");
    println!("      So we use: (index + capacity - 1) % capacity");
    println!("      (0 + 5 - 1) % 5 = 4 % 5 = 4 ✅");

    println!("\n   🧪 **Testing Different Capacities:**");
    for cap in [3, 4, 8, 16] {
        println!("\n      Capacity {}: Sequence of forward moves:", cap);
        let sequence: Vec<usize> = (0..cap * 2).map(|i| i % cap).collect();
        println!("      {:?}", sequence);
    }
}

fn demonstrate_head_tail_pointers() {
    println!("   👥 **Head and Tail Pointer Roles:**");

    println!("\n   📤 **HEAD Pointer (dequeue position):**");
    println!("      - Points to the NEXT item to be removed");
    println!("      - When dequeue: take item at head, then advance head");
    println!("      - head = (head + 1) % capacity");

    println!("\n   📥 **TAIL Pointer (enqueue position):**");
    println!("      - Points to the NEXT empty slot for insertion");
    println!("      - When enqueue: put item at tail, then advance tail");
    println!("      - tail = (tail + 1) % capacity");

    println!("\n   🎬 **Animation of Head/Tail Movement:**");

    let capacity = 5;
    let mut head = 0;
    let mut tail = 0;
    let mut buffer = vec!["_"; capacity];
    let mut length = 0;

    fn show_state(buffer: &[&str], head: usize, tail: usize, length: usize, capacity: usize) {
        println!("      Buffer: {:?}", buffer);
        let mut pointers = vec![" "; capacity];
        pointers[head] = "H";
        if head != tail {
            pointers[tail] = "T";
        } else if length > 0 {
            pointers[head] = "HT"; // When head == tail
        }
        println!("      Pointers: {:?} (H=head, T=tail)", pointers);
        println!("      Length: {}, Head: {}, Tail: {}", length, head, tail);
    }

    println!("\n   Initial state:");
    show_state(&buffer, head, tail, length, capacity);

    // Enqueue some items
    let items = ["A", "B", "C"];
    for &item in &items {
        println!("\n   📥 Enqueue '{}':", item);
        buffer[tail] = item;
        tail = (tail + 1) % capacity;
        length += 1;
        show_state(&buffer, head, tail, length, capacity);
    }

    // Dequeue one item
    println!("\n   📤 Dequeue:");
    let removed = buffer[head];
    buffer[head] = "_";
    head = (head + 1) % capacity;
    length -= 1;
    println!("      Removed: '{}'", removed);
    show_state(&buffer, head, tail, length, capacity);

    // Enqueue more to show wrap-around
    let more_items = ["D", "E", "F"];
    for &item in &more_items {
        if length < capacity {
            println!("\n   📥 Enqueue '{}' (tail wrapping!):", item);
            buffer[tail] = item;
            tail = (tail + 1) % capacity;
            length += 1;
            show_state(&buffer, head, tail, length, capacity);
        }
    }
}

fn demonstrate_wraparound() {
    println!("   🔄 **Wrap-Around Behavior in Detail:**");

    // Create a visual ring buffer simulation
    #[derive(Debug)]
    struct RingBufferVisualizer<T> {
        buffer: Vec<Option<T>>,
        head: usize,
        tail: usize,
        length: usize,
    }

    impl<T: Clone + std::fmt::Display> RingBufferVisualizer<T> {
        fn new(capacity: usize) -> Self {
            Self {
                buffer: vec![None; capacity],
                head: 0,
                tail: 0,
                length: 0,
            }
        }

        fn capacity(&self) -> usize {
            self.buffer.len()
        }

        fn enqueue(&mut self, item: T) -> Result<(), T> {
            if self.length == self.capacity() {
                return Err(item);
            }

            self.buffer[self.tail] = Some(item);
            self.tail = (self.tail + 1) % self.capacity();
            self.length += 1;
            Ok(())
        }

        fn dequeue(&mut self) -> Option<T> {
            if self.length == 0 {
                return None;
            }

            let item = self.buffer[self.head].take();
            self.head = (self.head + 1) % self.capacity();
            self.length -= 1;
            item
        }

        fn show_state(&self, operation: &str) {
            println!("\n      After {}:", operation);

            // Show buffer contents
            let display: Vec<String> = self
                .buffer
                .iter()
                .map(|opt| match opt {
                    Some(val) => format!("{}", val),
                    None => "_".to_string(),
                })
                .collect();
            println!("      Buffer: {:?}", display);

            // Show pointers
            let mut pointers = vec!["  "; self.capacity()];
            if self.length == 0 {
                pointers[self.head] = "HT";
            } else if self.head == self.tail {
                pointers[self.head] = "HT"; // Full buffer case
            } else {
                pointers[self.head] = "H ";
                pointers[self.tail] = "T ";
            }
            println!("      Points: {:?}", pointers);
            println!(
                "      H={}, T={}, Len={}/{}",
                self.head,
                self.tail,
                self.length,
                self.capacity()
            );
        }
    }

    let mut ring = RingBufferVisualizer::new(4);
    ring.show_state("initialization");

    // Fill the buffer
    println!("\n   📥 **Filling the buffer:**");
    for (i, &item) in ["A", "B", "C", "D"].iter().enumerate() {
        let _ = ring.enqueue(item);
        ring.show_state(&format!("enqueue '{}'", item));
    }

    // Try to add when full
    println!("\n   🚫 **Attempting to add when full:**");
    match ring.enqueue("E") {
        Err(item) => println!("      Cannot add '{}' - buffer is full!", item),
        Ok(_) => println!("      This shouldn't happen!"),
    }

    // Remove some items
    println!("\n   📤 **Removing items:**");
    for _ in 0..2 {
        if let Some(item) = ring.dequeue() {
            ring.show_state(&format!("dequeue '{}'", item));
        }
    }

    // Add more items to demonstrate wrap-around
    println!("\n   🔄 **Adding more items (tail wraps around):**");
    for &item in ["E", "F"] {
        let _ = ring.enqueue(item);
        ring.show_state(&format!("enqueue '{}' (wrapped!)", item));
    }

    println!("\n   🎯 **Key Observations:**");
    println!("      - Tail pointer wrapped from index 3 back to index 0");
    println!("      - No elements were shifted or moved");
    println!("      - Buffer space is reused efficiently");
    println!("      - All operations remain O(1)");
}

fn demonstrate_full_empty_detection() {
    println!("   🎯 **Detecting Full vs Empty States:**");

    println!("\n   ❓ **The Challenge:**");
    println!("      When head == tail, is the buffer empty or full?");
    println!("      - Empty: head==tail, no items");
    println!("      - Full: head==tail after wrapping around");
    println!("      - We need additional information to distinguish!");

    println!("\n   ✅ **Solution: Keep Track of Length**");
    println!("      - Maintain a separate `length` field");
    println!("      - Empty: length == 0");
    println!("      - Full: length == capacity");
    println!("      - Simple and reliable!");

    println!("\n   🔢 **Alternative Solutions (for reference):**");
    println!("      1. **Sentinel Slot**: Keep one slot always empty");
    println!("         - Full when (tail + 1) % capacity == head");
    println!("         - Wastes one slot but no length field needed");

    println!("      2. **Flag-based**: Use separate full/empty boolean");
    println!("         - Set flag when transitioning to full/empty");
    println!("         - More complex state management");

    println!("      3. **Generation Counter**: Track wrap-around count");
    println!("         - Compare generation of head vs tail");
    println!("         - Most complex but works for advanced use cases");

    println!("\n   🎪 **Demonstration of States:**");

    // Show all possible states
    let capacity = 3;
    let states = [
        (0, 0, 0, "Empty"),
        (0, 1, 1, "Partially filled"),
        (0, 2, 2, "Partially filled"),
        (0, 0, 3, "Full (head==tail but length==capacity)"),
        (1, 1, 0, "Empty after some operations"),
        (2, 1, 2, "Wrapped around"),
    ];

    println!("      Head | Tail | Length | State");
    println!("      -----|------|--------|----------------");

    for (head, tail, length, description) in states {
        println!(
            "       {}   |  {}   |   {}    | {}",
            head, tail, length, description
        );
    }

    println!("\n   💡 **Implementation Tips:**");
    println!("      - Always update length when enqueue/dequeue");
    println!("      - Check length for full/empty, not head==tail");
    println!("      - Use debug_assert! to verify length consistency");
    println!("      - Length should always be <= capacity");
}

fn self_assessment() {
    println!("\n=== Self-Assessment Questions ===");
    println!("After completing Step 3, you should understand:");
    println!("1. What is the formula for circular indexing with modulo?");
    println!("2. Why do we use (index + capacity - 1) % capacity for going backward?");
    println!("3. What do head and tail pointers represent in a ring buffer?");
    println!("4. How do you detect if a ring buffer is full vs empty?");
    println!("5. What happens when tail wraps around to index 0?");

    println!("\n=== Ring Buffer Core Concepts ===");
    println!("🎯 Circular indexing eliminates need for element shifting");
    println!("🎯 Modulo arithmetic (%) enables wrap-around behavior");
    println!("🎯 Head pointer tracks next dequeue position");
    println!("🎯 Tail pointer tracks next enqueue position");
    println!("🎯 Length field distinguishes full from empty when head==tail");

    println!("\n=== Mathematical Foundations ===");
    println!("📐 Forward: next = (current + 1) % capacity");
    println!("📐 Backward: prev = (current + capacity - 1) % capacity");
    println!("📐 Distance: (tail - head + capacity) % capacity == length");
    println!("📐 Full condition: length == capacity");
    println!("📐 Empty condition: length == 0");

    println!("\n=== Performance Benefits ===");
    println!("⚡ O(1) enqueue - just set buffer[tail] and increment");
    println!("⚡ O(1) dequeue - just get buffer[head] and increment");
    println!("⚡ No memory allocation after initialization");
    println!("⚡ No element shifting or moving required");
    println!("⚡ Cache-friendly sequential access patterns");

    println!("\n=== Implementation Readiness ===");
    println!("Next step: Handle Rust ownership with Option<T>");
    println!("• Step 4: Ring buffer with proper Rust ownership patterns");
    println!("• Learn why we need Option<T> for safe element removal");
    println!("• Implement full ring buffer with error handling");

    println!("\nRun: cargo run --example step4_ring_buffer_with_option");
}
