// Mission2_tut Step 4: Ring Buffer with Option<T>
// Master Rust ownership patterns for safe element storage and removal

fn main() {
    println!("=== Mission2_tut Step 4: Ring Buffer with Option<T> ===\n");

    // 1. Why Option<T> is Essential
    println!("1. Understanding Option<T> Necessity:");
    explain_option_necessity();

    // 2. Option<T> Memory Layout
    println!("\n2. Option<T> Memory Layout and Performance:");
    demonstrate_option_memory_layout();

    // 3. Safe Element Removal with take()
    println!("\n3. Safe Element Removal Patterns:");
    demonstrate_safe_removal();

    // 4. Complete Ring Buffer Implementation
    println!("\n4. Complete Ring Buffer Implementation:");
    demonstrate_complete_ring_buffer();

    // 5. Error Handling Patterns
    println!("\n5. Proper Error Handling:");
    demonstrate_error_handling();

    // 6. Real-World Usage Examples
    println!("\n6. Real-World Applications:");
    demonstrate_real_world_usage();

    self_assessment();
}

fn explain_option_necessity() {
    println!("   🤔 **The Rust Ownership Challenge:**");
    println!("      In languages like C++ or Java:");
    println!("      - Arrays can hold 'null' pointers or references");
    println!("      - Easy to represent 'empty' slots");
    println!("      - But also easy to have null pointer bugs!");

    println!("\n   🦀 **Rust's Solution: Option<T>**");
    println!("      - No null pointers in Rust!");
    println!("      - Option<T> explicitly represents 'maybe has a value'");
    println!("      - Some(value) = slot has a value");
    println!("      - None = slot is empty");
    println!("      - Compiler forces you to handle both cases");

    println!("\n   ❌ **Why Vec<T> doesn't work for ring buffers:**");

    // Demonstrate the problem
    println!("      Try 1: Vec<String> - but how to represent empty slots?");
    let attempt1 = vec![String::new(); 5];
    println!("      {:?}", attempt1);
    println!("      Problem: Empty strings look like empty slots, but they're not!");

    println!("\n      Try 2: Pre-fill with dummy values?");
    let mut attempt2 = vec!["EMPTY".to_string(); 5];
    attempt2[0] = "Real Value".to_string();
    println!("      {:?}", attempt2);
    println!("      Problem: Magic strings are error-prone and wasteful!");

    println!("\n   ✅ **Option<T> Solution:**");
    let mut ring_buffer = vec![None; 5];
    ring_buffer[0] = Some("Real Value".to_string());
    ring_buffer[1] = Some("Another Value".to_string());
    println!("      {:?}", ring_buffer);
    println!("      Perfect! Clear distinction between Some(value) and None");

    println!("\n   🎯 **Key Benefits of Option<T>:**");
    println!("      ✅ Explicit empty state representation");
    println!("      ✅ Compiler prevents accessing empty spots");
    println!("      ✅ Safe removal with take() method");
    println!("      ✅ No magic values or sentinel objects needed");
    println!("      ✅ Works with any type T (String, i32, custom structs)");

    println!("\n   🧠 **Mental Model:**");
    println!("      Think of Option<T> as a box that either:");
    println!("      📦 Contains something (Some(value))");
    println!("      📭 Is empty (None)");
    println!("      The box itself always exists - it's just empty or full");
}

fn demonstrate_option_memory_layout() {
    println!("   💾 **Option<T> Memory Layout Insights:**");

    println!("\n   🔍 **Size Comparison:**");
    use std::mem;

    println!("      Size of i32: {} bytes", mem::size_of::<i32>());
    println!(
        "      Size of Option<i32>: {} bytes",
        mem::size_of::<Option<i32>>()
    );
    println!("      Size of String: {} bytes", mem::size_of::<String>());
    println!(
        "      Size of Option<String>: {} bytes",
        mem::size_of::<Option<String>>()
    );

    println!("\n   🎭 **Null Pointer Optimization:**");
    println!("      For types like Box<T>, Vec<T>, String:");
    println!("      - These can never be 'null' naturally");
    println!("      - Rust uses null representation for None");
    println!("      - Option<Box<T>> same size as Box<T>!");
    println!("      - Zero-cost abstraction in many cases");

    println!("\n   📊 **Memory Layout Visualization:**");
    println!("      Vec<i32> with capacity 4:");
    println!("      [Some(10), None, Some(20), None]");
    println!("      Memory: [tag+10] [tag+null] [tag+20] [tag+null]");
    println!("      Where tag indicates Some/None state");

    println!("\n   ⚡ **Performance Characteristics:**");
    println!("      ✅ Pattern matching compiles to simple checks");
    println!("      ✅ No heap allocation for Option itself");
    println!("      ✅ Cache-friendly sequential access");
    println!("      ✅ take() is just a memory move, not copy");

    // Demonstrate performance with simple benchmark
    println!("\n   🏁 **Quick Performance Demo:**");
    let mut buffer = vec![None; 10000];

    // Fill with values
    let start = std::time::Instant::now();
    for i in 0..10000 {
        buffer[i] = Some(i);
    }
    let fill_time = start.elapsed();

    // Take all values
    let start = std::time::Instant::now();
    for slot in &mut buffer {
        let _ = slot.take();
    }
    let take_time = start.elapsed();

    println!("      Fill 10,000 slots: {:?}", fill_time);
    println!("      Take 10,000 values: {:?}", take_time);
    println!("      Both operations are extremely fast!");
}

fn demonstrate_safe_removal() {
    println!("   🔒 **Safe Element Removal with take():**");

    println!("\n   📝 **The take() Method:**");
    println!("      - Replaces Option<T> with None");
    println!("      - Returns the previous value");
    println!("      - Leaves slot in clean 'empty' state");
    println!("      - No risk of use-after-free bugs");

    println!("\n   🎭 **take() vs Clone vs Move:**");

    let mut slot1 = Some("Hello".to_string());
    let slot2 = Some("World".to_string());
    let mut slot3 = Some("Rust".to_string());

    println!("      Initial state:");
    println!("      slot1: {:?}", slot1);
    println!("      slot2: {:?}", slot2);
    println!("      slot3: {:?}", slot3);

    // Method 1: take() - ownership transfer
    println!("\n   🎯 Method 1: take() (preferred)");
    let value1 = slot1.take();
    println!("      After slot1.take():");
    println!("      slot1: {:?}", slot1);
    println!("      value1: {:?}", value1);
    println!("      ✅ Slot is now None, value is owned by value1");

    // Method 2: clone() - expensive
    println!("\n   💰 Method 2: clone() (expensive)");
    let value2 = slot2.clone();
    println!("      After slot2.clone():");
    println!("      slot2: {:?}", slot2);
    println!("      value2: {:?}", value2);
    println!("      ⚠️  Slot still has value, but we made an expensive copy");

    // Method 3: unwrap() - dangerous
    println!("\n   💥 Method 3: unwrap() (dangerous)");
    let value3 = slot3.take().unwrap();
    println!("      After slot3.take().unwrap():");
    println!("      value3: {:?}", value3);
    println!("      ⚠️  Would panic if slot was None!");

    println!("\n   🎯 **Ring Buffer Context:**");
    println!("      In dequeue operation:");
    println!("      1. Check if slot has value (pattern matching)");
    println!("      2. Use take() to safely remove and own value");
    println!("      3. Slot becomes None automatically");
    println!("      4. Return owned value to caller");

    // Show the pattern
    fn safe_dequeue_pattern<T>(buffer: &mut Vec<Option<T>>, head: usize) -> Option<T> {
        buffer[head].take()
    }

    let mut demo_buffer = vec![Some(42), None, Some(99)];
    println!("\n   🔄 **Safe Dequeue Pattern Demo:**");
    println!("      Buffer before: {:?}", demo_buffer);

    let result1 = safe_dequeue_pattern(&mut demo_buffer, 0);
    println!("      Dequeue from index 0: {:?}", result1);
    println!("      Buffer after: {:?}", demo_buffer);

    let result2 = safe_dequeue_pattern(&mut demo_buffer, 1);
    println!("      Dequeue from index 1: {:?}", result2);
    println!("      Buffer after: {:?}", demo_buffer);

    println!("\n   ✨ **Magic of take():**");
    println!("      - No panics even on empty slots");
    println!("      - No cloning or expensive copies");
    println!("      - Clean state management");
    println!("      - Impossible to leave dangling references");
}

fn demonstrate_complete_ring_buffer() {
    println!("   🎪 **Complete Ring Buffer Implementation:**");

    // Full implementation with proper error handling
    #[derive(Debug)]
    struct RingBuffer<T> {
        buffer: Vec<Option<T>>,
        head: usize,   // Next position to dequeue from
        tail: usize,   // Next position to enqueue to
        length: usize, // Current number of items
    }

    #[derive(Debug, PartialEq)]
    enum QueueError {
        Full,
        Empty,
    }

    impl<T: Clone> RingBuffer<T> {
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

        fn len(&self) -> usize {
            self.length
        }

        fn is_empty(&self) -> bool {
            self.length == 0
        }

        fn is_full(&self) -> bool {
            self.length == self.capacity()
        }

        fn enqueue(&mut self, item: T) -> Result<(), QueueError> {
            if self.is_full() {
                return Err(QueueError::Full);
            }

            // Safe to unwrap because we know slot is None when not full
            debug_assert!(self.buffer[self.tail].is_none());

            self.buffer[self.tail] = Some(item);
            self.tail = (self.tail + 1) % self.capacity();
            self.length += 1;

            Ok(())
        }

        fn dequeue(&mut self) -> Result<T, QueueError> {
            if self.is_empty() {
                return Err(QueueError::Empty);
            }

            // take() safely removes and returns the value
            let item = self.buffer[self.head]
                .take()
                .expect("Buffer slot should have value when length > 0");

            self.head = (self.head + 1) % self.capacity();
            self.length -= 1;

            Ok(item)
        }

        fn peek(&self) -> Option<&T> {
            if self.is_empty() {
                None
            } else {
                // Safe because we know head has a value when not empty
                self.buffer[self.head].as_ref()
            }
        }

        fn peek_mut(&mut self) -> Option<&mut T> {
            if self.is_empty() {
                None
            } else {
                self.buffer[self.head].as_mut()
            }
        }
    }

    // Demonstrate the complete implementation
    println!("\n   🧪 **Testing Complete Implementation:**");

    let mut queue = RingBuffer::new(3);
    println!("      Created queue with capacity 3");
    println!(
        "      Initial state: empty={}, full={}, len={}",
        queue.is_empty(),
        queue.is_full(),
        queue.len()
    );

    // Fill the queue
    println!("\n   📥 **Filling the queue:**");
    for item in ["A", "B", "C"] {
        match queue.enqueue(item) {
            Ok(()) => println!("      ✅ Enqueued: {}", item),
            Err(e) => println!("      ❌ Failed to enqueue {}: {:?}", item, e),
        }
        println!(
            "         State: len={}, empty={}, full={}",
            queue.len(),
            queue.is_empty(),
            queue.is_full()
        );
    }

    // Try to add when full
    println!("\n   🚫 **Testing full condition:**");
    match queue.enqueue("D") {
        Ok(()) => println!("      This shouldn't happen!"),
        Err(e) => println!("      ✅ Correctly rejected: {:?}", e),
    }

    // Peek at head
    println!("\n   👀 **Peeking at head:**");
    match queue.peek() {
        Some(value) => println!("      Head value: {}", value),
        None => println!("      Queue is empty"),
    }

    // Dequeue some items
    println!("\n   📤 **Dequeuing items:**");
    for _ in 0..2 {
        match queue.dequeue() {
            Ok(item) => {
                println!("      ✅ Dequeued: {}", item);
                println!(
                    "         State: len={}, empty={}, full={}",
                    queue.len(),
                    queue.is_empty(),
                    queue.is_full()
                );
            }
            Err(e) => println!("      ❌ Failed to dequeue: {:?}", e),
        }
    }

    // Add more items (demonstrating wrap-around)
    println!("\n   🔄 **Adding items after wrap-around:**");
    for item in ["D", "E"] {
        match queue.enqueue(item) {
            Ok(()) => println!("      ✅ Enqueued: {} (wrapped around!)", item),
            Err(e) => println!("      ❌ Failed: {:?}", e),
        }
    }

    // Final state
    println!("\n   🏁 **Final queue state:**");
    println!("      Length: {}/{}", queue.len(), queue.capacity());
    while !queue.is_empty() {
        if let Ok(item) = queue.dequeue() {
            println!("      Remaining item: {}", item);
        }
    }
}

fn demonstrate_error_handling() {
    println!("   🚨 **Proper Error Handling Patterns:**");

    println!("\n   1️⃣  **Result<T, E> for Operations:**");
    println!("      - enqueue() returns Result<(), QueueError>");
    println!("      - dequeue() returns Result<T, QueueError>");
    println!("      - Caller must handle both success and failure");

    println!("\n   2️⃣  **Option<T> for Safe Queries:**");
    println!("      - peek() returns Option<&T>");
    println!("      - peek_mut() returns Option<&mut T>");
    println!("      - No panics, just None for empty queue");

    println!("\n   3️⃣  **Debug Assertions for Invariants:**");
    println!("      - debug_assert!(buffer[tail].is_none()) before enqueue");
    println!("      - debug_assert!(length <= capacity) always");
    println!("      - Catch bugs in development, zero cost in release");

    // Show different error handling styles
    #[derive(Debug)]
    struct SimpleQueue<T> {
        buffer: Vec<Option<T>>,
        head: usize,
        tail: usize,
        length: usize,
    }

    impl<T: Clone> SimpleQueue<T> {
        fn new(capacity: usize) -> Self {
            Self {
                buffer: vec![None; capacity],
                head: 0,
                tail: 0,
                length: 0,
            }
        }

        fn try_enqueue(&mut self, item: T) -> Result<(), T> {
            if self.length == self.buffer.len() {
                Err(item) // Return the item back to caller
            } else {
                self.buffer[self.tail] = Some(item);
                self.tail = (self.tail + 1) % self.buffer.len();
                self.length += 1;
                Ok(())
            }
        }

        fn try_dequeue(&mut self) -> Option<T> {
            if self.length == 0 {
                None
            } else {
                let item = self.buffer[self.head].take().unwrap();
                self.head = (self.head + 1) % self.buffer.len();
                self.length -= 1;
                Some(item)
            }
        }
    }

    println!("\n   🧪 **Error Handling Style Comparison:**");

    let mut queue = SimpleQueue::new(2);

    println!("      Style 1: Return item on failure");
    match queue.try_enqueue("First") {
        Ok(()) => println!("      ✅ Enqueued successfully"),
        Err(item) => println!("      ❌ Queue full, returned: {}", item),
    }

    let _ = queue.try_enqueue("Second");

    match queue.try_enqueue("Third") {
        Ok(()) => println!("      This shouldn't happen!"),
        Err(item) => println!("      ✅ Queue full, returned: {}", item),
    }

    println!("\n      Style 2: Option for optional results");
    match queue.try_dequeue() {
        Some(item) => println!("      ✅ Dequeued: {}", item),
        None => println!("      ❌ Queue was empty"),
    }

    println!("\n   🎯 **Choosing Error Handling Style:**");
    println!("      - Result<T, E>: When operation can fail for specific reasons");
    println!("      - Option<T>: When absence is a normal, expected state");
    println!("      - Panic: Only for programmer errors or invariant violations");
    println!("      - Return original value: When caller might want to retry");
}

fn demonstrate_real_world_usage() {
    println!("   🌍 **Real-World Ring Buffer Applications:**");

    println!("\n   1️⃣  **Audio Buffer Example:**");
    #[derive(Debug, Clone)]
    struct AudioSample {
        left: f32,
        right: f32,
    }

    struct AudioBuffer {
        samples: Vec<Option<AudioSample>>,
        head: usize,
        tail: usize,
        length: usize,
    }

    impl AudioBuffer {
        fn new(capacity: usize) -> Self {
            Self {
                samples: vec![None; capacity],
                head: 0,
                tail: 0,
                length: 0,
            }
        }

        fn write_sample(&mut self, sample: AudioSample) -> bool {
            if self.length == self.samples.len() {
                return false; // Buffer overrun
            }

            self.samples[self.tail] = Some(sample);
            self.tail = (self.tail + 1) % self.samples.len();
            self.length += 1;
            true
        }

        fn read_sample(&mut self) -> Option<AudioSample> {
            if self.length == 0 {
                return None; // Buffer underrun
            }

            let sample = self.samples[self.head].take().unwrap();
            self.head = (self.head + 1) % self.samples.len();
            self.length -= 1;
            Some(sample)
        }
    }

    let mut audio_buffer = AudioBuffer::new(1024);
    println!("      Created audio buffer with 1024 sample capacity");
    println!("      Use case: Producer writes audio data, consumer plays it");
    println!("      Ring buffer smooths out timing differences");

    // Simulate some audio processing
    for i in 0..5 {
        let sample = AudioSample {
            left: (i as f32) * 0.1,
            right: (i as f32) * 0.1,
        };
        audio_buffer.write_sample(sample);
    }

    if let Some(sample) = audio_buffer.read_sample() {
        println!("      First audio sample: {:?}", sample);
    }

    println!("\n   2️⃣  **Network Packet Buffer:**");
    #[derive(Debug, Clone)]
    struct Packet {
        id: u32,
        data: Vec<u8>,
    }

    struct PacketBuffer {
        packets: Vec<Option<Packet>>,
        head: usize,
        tail: usize,
        length: usize,
    }

    impl PacketBuffer {
        fn new(capacity: usize) -> Self {
            Self {
                packets: vec![None; capacity],
                head: 0,
                tail: 0,
                length: 0,
            }
        }

        fn receive_packet(&mut self, packet: Packet) -> Result<(), Packet> {
            if self.length == self.packets.len() {
                return Err(packet); // Drop packet due to full buffer
            }

            self.packets[self.tail] = Some(packet);
            self.tail = (self.tail + 1) % self.packets.len();
            self.length += 1;
            Ok(())
        }

        fn process_next_packet(&mut self) -> Option<Packet> {
            if self.length == 0 {
                return None;
            }

            let packet = self.packets[self.head].take().unwrap();
            self.head = (self.head + 1) % self.packets.len();
            self.length -= 1;
            Some(packet)
        }

        fn has_packets(&self) -> bool {
            self.length > 0
        }
    }

    let mut packet_buffer = PacketBuffer::new(100);
    println!("      Created network packet buffer with 100 packet capacity");
    println!("      Use case: Buffer incoming packets for processing");

    // Simulate receiving packets
    for i in 0..3 {
        let packet = Packet {
            id: i,
            data: vec![0u8; 64], // 64-byte packet
        };
        match packet_buffer.receive_packet(packet) {
            Ok(()) => println!("      ✅ Received packet {}", i),
            Err(dropped) => println!("      ❌ Dropped packet {} (buffer full)", dropped.id),
        }
    }

    // Process packets
    while packet_buffer.has_packets() {
        if let Some(packet) = packet_buffer.process_next_packet() {
            println!(
                "      📤 Processing packet {} ({} bytes)",
                packet.id,
                packet.data.len()
            );
        }
    }

    println!("\n   3️⃣  **Message Queue Example:**");
    #[derive(Debug, Clone)]
    enum Message {
        Text(String),
        Image {
            width: u32,
            height: u32,
            data: Vec<u8>,
        },
        Video {
            duration: f32,
            fps: u32,
        },
    }

    struct MessageQueue {
        messages: Vec<Option<Message>>,
        head: usize,
        tail: usize,
        length: usize,
    }

    impl MessageQueue {
        fn new(capacity: usize) -> Self {
            Self {
                messages: vec![None; capacity],
                head: 0,
                tail: 0,
                length: 0,
            }
        }

        fn send(&mut self, message: Message) -> Result<(), &'static str> {
            if self.length == self.messages.len() {
                return Err("Message queue full");
            }

            self.messages[self.tail] = Some(message);
            self.tail = (self.tail + 1) % self.messages.len();
            self.length += 1;
            Ok(())
        }

        fn receive(&mut self) -> Option<Message> {
            if self.length == 0 {
                return None;
            }

            let message = self.messages[self.head].take().unwrap();
            self.head = (self.head + 1) % self.messages.len();
            self.length -= 1;
            Some(message)
        }
    }

    let mut msg_queue = MessageQueue::new(10);
    println!("      Created message queue with capacity 10");

    // Send different types of messages
    let _ = msg_queue.send(Message::Text("Hello, World!".to_string()));
    let _ = msg_queue.send(Message::Image {
        width: 1920,
        height: 1080,
        data: vec![0; 1000],
    });
    let _ = msg_queue.send(Message::Video {
        duration: 30.0,
        fps: 60,
    });

    // Receive and process messages
    while let Some(message) = msg_queue.receive() {
        match message {
            Message::Text(text) => println!("      📝 Text message: {}", text),
            Message::Image { width, height, .. } => {
                println!("      🖼️  Image: {}x{}", width, height)
            }
            Message::Video { duration, fps } => {
                println!("      🎥 Video: {:.1}s @ {}fps", duration, fps)
            }
        }
    }

    println!("\n   🎯 **Common Ring Buffer Patterns:**");
    println!("      ✅ Producer-Consumer scenarios");
    println!("      ✅ Buffering between different processing speeds");
    println!("      ✅ Fixed-size caches with automatic eviction");
    println!("      ✅ Audio/Video streaming applications");
    println!("      ✅ Network packet buffering");
    println!("      ✅ Event queues in game engines");
    println!("      ✅ Log message buffering");
    println!("      ✅ Real-time data processing pipelines");
}

fn self_assessment() {
    println!("\n=== Self-Assessment Questions ===");
    println!("After completing Step 4, you should understand:");
    println!("1. Why does a ring buffer need Option<T> instead of just Vec<T>?");
    println!("2. How does take() safely remove values without leaving dangling references?");
    println!("3. When should you use Result vs Option for error handling?");
    println!("4. How do debug_assert! help catch invariant violations?");
    println!("5. What are the memory characteristics of Option<T>?");

    println!("\n=== Option<T> Mastery ===");
    println!("🎯 Option<T> provides safe 'maybe empty' semantics");
    println!("🎯 take() transfers ownership while leaving None behind");
    println!("🎯 Pattern matching forces explicit handling of None case");
    println!("🎯 Zero-cost abstraction for many pointer-like types");
    println!("🎯 No null pointer bugs possible in safe Rust");

    println!("\n=== Error Handling Patterns ===");
    println!("📋 Result<T, E> for operations that can fail for known reasons");
    println!("📋 Option<T> for queries that might not have a result");
    println!("📋 Return original value on failure for retry scenarios");
    println!("📋 debug_assert! for development-time invariant checking");
    println!("📋 expect() with clear messages for 'impossible' failures");

    println!("\n=== Implementation Quality ===");
    println!("⚡ Proper state invariants (length ≤ capacity)");
    println!("⚡ Clear method contracts (when they succeed/fail)");
    println!("⚡ Safe memory management with automatic cleanup");
    println!("⚡ Zero-copy operations where possible");
    println!("⚡ Comprehensive error coverage");

    println!("\n=== Real-World Applications ===");
    println!("🌍 Audio/video streaming buffers");
    println!("🌍 Network packet queues");
    println!("🌍 Message passing systems");
    println!("🌍 Producer-consumer patterns");
    println!("🌍 Event handling in games/UIs");

    println!("\n=== Implementation Readiness ===");
    println!("Next step: Build production-ready queue with full API");
    println!("• Step 5: Complete queue with iterator support and optimizations");
    println!("• Learn advanced patterns like drain() and extend()");
    println!("• Implement Display, Debug and other useful traits");

    println!("\nRun: cargo run --example step5_production_queue");
}
