// Example: Basic MPSC Channel Usage
// Multiple producer, single consumer channel basics

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== MPSC Channel Basics ===\n");

    // Example 1: Single sender
    println!("Example 1: Single message");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("Hello from thread!").unwrap();
    });

    let msg = rx.recv().unwrap();
    println!("Received: {}\n", msg);

    // Example 2: Multiple messages
    println!("Example 2: Multiple messages");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec!["Hello", "from", "the", "thread"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Receive messages until channel closes
    for received in rx {
        println!("Got: {}", received);
    }
    println!();

    // Example 3: Non-blocking receive
    println!("Example 3: Non-blocking with try_recv()");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        tx.send("Delayed message").unwrap();
    });

    println!("Checking for messages (non-blocking)...");
    for _ in 0..10 {
        match rx.try_recv() {
            Ok(msg) => {
                println!("Received: {}", msg);
                break;
            }
            Err(mpsc::TryRecvError::Empty) => {
                println!("No message yet...");
                thread::sleep(Duration::from_millis(100));
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("Channel disconnected!");
                break;
            }
        }
    }
    println!();

    // Example 4: Sending complex data
    println!("Example 4: Sending structured data");
    
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Task {
        id: u32,
        description: String,
        priority: u8,
    }

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let tasks = vec![
            Task { id: 1, description: "Process data".to_string(), priority: 1 },
            Task { id: 2, description: "Send report".to_string(), priority: 2 },
            Task { id: 3, description: "Clean up".to_string(), priority: 3 },
        ];

        for task in tasks {
            tx.send(task).unwrap();
        }
    });

    for task in rx {
        println!("Received task: {:?}", task);
    }
}
