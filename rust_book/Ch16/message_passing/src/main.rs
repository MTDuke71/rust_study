// Chapter 16.2: Using Message Passing to Transfer Data Between Threads
// Multiple producer, single consumer (mpsc) channels

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Basic Channel Usage ===\n");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("Sent: hi");
    });

    let received = rx.recv().unwrap();
    println!("Received: {}\n", received);

    println!("=== Sending Multiple Messages ===\n");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    println!("\nMessage passing examples complete!");
    println!("Run examples with: cargo run --example <name>");
    println!("  - mpsc_basics");
    println!("  - multiple_senders");
    println!("  - producer_consumer");
}
