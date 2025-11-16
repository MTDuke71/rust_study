// Example: Multiple Senders
// Demonstrates cloning the sender to have multiple producers

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Multiple Senders Example ===\n");

    let (tx, rx) = mpsc::channel();

    // Clone the sender for multiple producers
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    let tx3 = tx;

    // Spawn first producer
    thread::spawn(move || {
        let messages = vec!["Producer 1: msg 1", "Producer 1: msg 2", "Producer 1: msg 3"];
        for msg in messages {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Spawn second producer
    thread::spawn(move || {
        let messages = vec!["Producer 2: msg A", "Producer 2: msg B"];
        for msg in messages {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    });

    // Spawn third producer
    thread::spawn(move || {
        let messages = vec!["Producer 3: alpha", "Producer 3: beta", "Producer 3: gamma"];
        for msg in messages {
            tx3.send(msg).unwrap();
            thread::sleep(Duration::from_millis(120));
        }
    });

    // Single consumer receives from all producers
    println!("Consumer receiving from multiple producers:\n");
    
    // Collect all messages (channel closes when all senders drop)
    let mut count = 0;
    for received in rx {
        count += 1;
        println!("[{}] {}", count, received);
    }

    println!("\nTotal messages received: {}", count);
}
