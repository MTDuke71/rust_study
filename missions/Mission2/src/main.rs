//! Mission 2 demonstration: FIFO Queue implementations
//!
//! This program demonstrates the Ring Buffer and Linked Queue implementations
//! with real-world usage scenarios including BFS frontier simulation.

use mission2::{RingBufferQueue, LinkedQueue};
use std::collections::VecDeque;

fn main() {
    println!("🚀 Mission 2: FIFO Queue Implementations Demo");
    println!("==============================================\n");

    demo_ring_buffer_queue();
    demo_linked_queue();
    demo_overwriting_buffer(); // New demo for enqueue_overwrite
    demo_performance_comparison();
    demo_bfs_simulation();
}

fn demo_ring_buffer_queue() {
    println!("🔄 Ring Buffer Queue Demo");
    println!("-------------------------");
    
    let mut queue = RingBufferQueue::with_capacity(4);
    
    println!("Created ring buffer with capacity: {}", queue.capacity());
    
    // Demonstrate basic operations
    println!("\n1. Basic Operations:");
    for i in 1..=3 {
        queue.enqueue(i).unwrap();
        println!("  Enqueued: {} (len: {})", i, queue.len());
    }
    
    println!("  Peek: {:?}", queue.peek());
    
    while let Some(value) = queue.dequeue() {
        println!("  Dequeued: {} (len: {})", value, queue.len());
    }
    
    // Demonstrate wrap-around behavior
    println!("\n2. Wrap-around Behavior:");
    // Fill queue
    for i in 10..14 {
        queue.enqueue(i).unwrap();
    }
    println!("  Filled queue: [10, 11, 12, 13]");
    
    // Partial drain
    println!("  Dequeued: {}", queue.dequeue().unwrap());
    println!("  Dequeued: {}", queue.dequeue().unwrap());
    
    // Refill (wraps around)
    queue.enqueue(14).unwrap();
    queue.enqueue(15).unwrap();
    println!("  Added 14, 15 (wrapped around)");
    
    println!("  Final order:");
    while let Some(value) = queue.dequeue() {
        println!("    {}", value);
    }
    
    // Demonstrate backpressure
    println!("\n3. Backpressure (Full Queue):");
    for i in 20..24 {
        queue.enqueue(i).unwrap();
    }
    
    match queue.enqueue(99) {
        Ok(()) => println!("  Unexpected success!"),
        Err(value) => println!("  Queue full, returned value: {}", value),
    }
    
    queue.dequeue(); // Make space
    println!("  Made space, now enqueue works: {:?}", queue.enqueue(99));
    
    println!();
}

fn demo_linked_queue() {
    println!("🔗 Linked Queue Demo");
    println!("--------------------");
    
    let mut queue = LinkedQueue::new();
    
    println!("Created linked queue (unlimited capacity)");
    
    // Demonstrate growth
    println!("\n1. Dynamic Growth:");
    for i in 1..=5 {
        queue.enqueue(format!("Item-{}", i));
        println!("  Enqueued: Item-{} (len: {})", i, queue.len());
    }
    
    println!("  Peek: {:?}", queue.peek());
    
    // Partial processing
    println!("\n2. Partial Processing:");
    for _ in 0..2 {
        if let Some(item) = queue.dequeue() {
            println!("  Processing: {} (len: {})", item, queue.len());
        }
    }
    
    // Continue adding while processing
    println!("\n3. Concurrent Add/Process:");
    for i in 6..=8 {
        queue.enqueue(format!("Item-{}", i));
        println!("  Added: Item-{}", i);
        
        if let Some(item) = queue.dequeue() {
            println!("  Processed: {}", item);
        }
    }
    
    // Final drain
    println!("\n4. Final Processing:");
    while let Some(item) = queue.dequeue() {
        println!("  Final: {}", item);
    }
    
    println!();
}

fn demo_overwriting_buffer() {
    println!("🔄 Overwriting Ring Buffer Demo");
    println!("-------------------------------");
    
    let mut queue = RingBufferQueue::with_capacity(4);
    
    println!("Created overwriting ring buffer with capacity: {}", queue.capacity());
    
    // Fill the queue normally
    println!("\n1. Filling queue normally:");
    for i in 1..=4 {
        let result = queue.enqueue_overwrite(i);
        println!("  Enqueue {}: returned {:?} (len: {})", i, result, queue.len());
    }
    
    // Now it's full - demonstrate overwriting
    println!("\n2. Overwriting oldest values when full:");
    for i in 5..=7 {
        let overwritten = queue.enqueue_overwrite(i);
        println!("  Enqueue {}: overwrote {:?} (len: {})", i, overwritten, queue.len());
        println!("    Current front: {:?}", queue.peek());
    }
    
    // Show the power of overwriting with a message buffer simulation
    println!("\n3. Message Buffer Simulation:");
    let mut msg_buffer = RingBufferQueue::with_capacity(3);
    
    let messages = vec![
        "User connected",
        "User logged in", 
        "User opened file",
        "User saved file",
        "User logged out", // This will overwrite "User connected"
        "User disconnected", // This will overwrite "User logged in"
    ];
    
    for msg in messages {
        if let Some(old_msg) = msg_buffer.enqueue_overwrite(msg.to_string()) {
            println!("  📝 New: '{}' (replaced: '{}')", msg, old_msg);
        } else {
            println!("  📝 New: '{}'", msg);
        }
    }
    
    println!("\n4. Recent messages (most recent 3):");
    while let Some(msg) = msg_buffer.dequeue() {
        println!("    📜 {}", msg);
    }
    
    // Compare with regular enqueue behavior
    println!("\n5. Comparison with regular enqueue:");
    let mut regular_queue = RingBufferQueue::with_capacity(3);
    
    println!("  Regular enqueue:");
    for i in 1..=3 {
        regular_queue.enqueue(i).unwrap();
    }
    match regular_queue.enqueue(4) {
        Ok(()) => println!("    Enqueue 4: Success"),
        Err(value) => println!("    Enqueue 4: REJECTED (returned {})", value),
    }
    
    let mut overwrite_queue = RingBufferQueue::with_capacity(3);
    println!("\n  Overwrite enqueue:");
    for i in 1..=3 {
        overwrite_queue.enqueue_overwrite(i);
    }
    match overwrite_queue.enqueue_overwrite(4) {
        None => println!("    Enqueue 4: Success (no overwrite)"),
        Some(old) => println!("    Enqueue 4: Success (overwrote {})", old),
    }
    
    println!("\n  Final contents:");
    print!("    Regular queue: ");
    while let Some(val) = regular_queue.dequeue() {
        print!("{} ", val);
    }
    println!();
    
    print!("    Overwrite queue: ");
    while let Some(val) = overwrite_queue.dequeue() {
        print!("{} ", val);
    }
    println!();
    
    println!();
}

fn demo_performance_comparison() {
    println!("⚡ Performance Comparison");
    println!("------------------------");
    
    use std::time::Instant;
    const N: usize = 100_000;
    
    // Ring Buffer performance
    let start = Instant::now();
    let mut ring = RingBufferQueue::with_capacity(1000);
    let mut processed = 0;
    
    for i in 0..N {
        if ring.enqueue(i).is_err() {
            // Process some elements when full
            for _ in 0..100 {
                if ring.dequeue().is_some() {
                    processed += 1;
                }
            }
            ring.enqueue(i).unwrap();
        }
    }
    
    // Process remaining
    while ring.dequeue().is_some() {
        processed += 1;
    }
    
    let ring_time = start.elapsed();
    println!("Ring Buffer: {} operations in {:?}", N, ring_time);
    println!("  Processed: {} items", processed);
    
    // Linked Queue performance
    let start = Instant::now();
    let mut linked = LinkedQueue::new();
    
    for i in 0..N {
        linked.enqueue(i);
    }
    
    let mut processed = 0;
    while linked.dequeue().is_some() {
        processed += 1;
    }
    
    let linked_time = start.elapsed();
    println!("Linked Queue: {} operations in {:?}", N, linked_time);
    println!("  Processed: {} items", processed);
    
    // VecDeque comparison
    let start = Instant::now();
    let mut vecdeque = VecDeque::new();
    
    for i in 0..N {
        vecdeque.push_back(i);
    }
    
    let mut processed = 0;
    while vecdeque.pop_front().is_some() {
        processed += 1;
    }
    
    let vec_time = start.elapsed();
    println!("VecDeque:     {} operations in {:?}", N, vec_time);
    println!("  Processed: {} items", processed);
    
    println!();
}

fn demo_bfs_simulation() {
    println!("🌐 BFS Frontier Simulation");
    println!("--------------------------");
    
    // Simulate BFS on a small grid using ring buffer for frontier
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Point { x: i32, y: i32 }
    
    let mut frontier = RingBufferQueue::with_capacity(20);
    let mut visited = std::collections::HashSet::new();
    
    // Start BFS from origin
    let start = Point { x: 0, y: 0 };
    frontier.enqueue(start).unwrap();
    visited.insert((start.x, start.y));
    
    println!("Starting BFS from {:?}", start);
    println!("Frontier capacity: {}", frontier.capacity());
    
    let mut level = 0;
    let max_levels = 3;
    
    while !frontier.is_empty() && level < max_levels {
        let current_level_size = frontier.len();
        println!("\nLevel {}: {} nodes", level, current_level_size);
        
        for _ in 0..current_level_size {
            if let Some(current) = frontier.dequeue() {
                println!("  Visiting: {:?}", current);
                
                // Add neighbors (4-directional)
                let neighbors = [
                    Point { x: current.x + 1, y: current.y },
                    Point { x: current.x - 1, y: current.y },
                    Point { x: current.x, y: current.y + 1 },
                    Point { x: current.x, y: current.y - 1 },
                ];
                
                for neighbor in &neighbors {
                    let key = (neighbor.x, neighbor.y);
                    if !visited.contains(&key) && 
                       neighbor.x.abs() <= 3 && neighbor.y.abs() <= 3 {
                        
                        if frontier.enqueue(*neighbor).is_ok() {
                            visited.insert(key);
                            println!("    Added: {:?}", neighbor);
                        } else {
                            println!("    Frontier full! Skipped: {:?}", neighbor);
                        }
                    }
                }
            }
        }
        level += 1;
    }
    
    println!("\nBFS completed. Visited {} nodes.", visited.len());
    println!("Final frontier size: {}", frontier.len());
    
    // Compare with unlimited linked queue
    println!("\n🔗 Unlimited BFS with Linked Queue:");
    let mut linked_frontier = LinkedQueue::new();
    let mut linked_visited = std::collections::HashSet::new();
    
    linked_frontier.enqueue(start);
    linked_visited.insert((start.x, start.y));
    
    let mut level = 0;
    while !linked_frontier.is_empty() && level < max_levels {
        let current_level_size = linked_frontier.len();
        println!("Level {}: {} nodes", level, current_level_size);
        
        for _ in 0..current_level_size {
            if let Some(current) = linked_frontier.dequeue() {
                let neighbors = [
                    Point { x: current.x + 1, y: current.y },
                    Point { x: current.x - 1, y: current.y },
                    Point { x: current.x, y: current.y + 1 },
                    Point { x: current.x, y: current.y - 1 },
                ];
                
                for neighbor in &neighbors {
                    let key = (neighbor.x, neighbor.y);
                    if !linked_visited.contains(&key) && 
                       neighbor.x.abs() <= 3 && neighbor.y.abs() <= 3 {
                        linked_frontier.enqueue(*neighbor);
                        linked_visited.insert(key);
                    }
                }
            }
        }
        level += 1;
    }
    
    println!("Linked queue visited {} nodes.", linked_visited.len());
    
    println!("\n✅ Demo completed successfully!");
}