//! Quick demonstration of the enqueue_overwrite functionality

use mission2::RingBufferQueue;

fn main() {
    println!("🔄 Quick Overwrite Demo");
    
    let mut queue = RingBufferQueue::with_capacity(3);
    
    // Fill normally
    for i in 1..=3 {
        queue.enqueue_overwrite(i);
    }
    
    // Overwrite
    let overwritten = queue.enqueue_overwrite(4);
    println!("Overwrote: {:?}", overwritten);
    
    // Verify contents
    while let Some(val) = queue.dequeue() {
        println!("Dequeued: {}", val);
    }
}