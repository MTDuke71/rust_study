// Example: Deadlock Demonstration
// Shows how deadlock can occur and how to avoid it

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Deadlock Example ===\n");

    println!("WARNING: This example will deadlock!");
    println!("Press Ctrl+C to stop if it hangs.\n");

    // Two mutexes that threads will try to lock in different orders
    let resource1 = Arc::new(Mutex::new(1));
    let resource2 = Arc::new(Mutex::new(2));

    // Thread 1: locks resource1 then resource2
    let r1_clone = Arc::clone(&resource1);
    let r2_clone = Arc::clone(&resource2);
    let thread1 = thread::spawn(move || {
        println!("Thread 1: Trying to lock resource1...");
        let _lock1 = r1_clone.lock().unwrap();
        println!("Thread 1: Locked resource1");
        
        thread::sleep(Duration::from_millis(100)); // Simulate some work
        
        println!("Thread 1: Trying to lock resource2...");
        let _lock2 = r2_clone.lock().unwrap();
        println!("Thread 1: Locked resource2");
        
        println!("Thread 1: Done!");
    });

    // Thread 2: locks resource2 then resource1 (OPPOSITE ORDER - causes deadlock)
    let r1_clone = Arc::clone(&resource1);
    let r2_clone = Arc::clone(&resource2);
    let thread2 = thread::spawn(move || {
        println!("Thread 2: Trying to lock resource2...");
        let _lock2 = r2_clone.lock().unwrap();
        println!("Thread 2: Locked resource2");
        
        thread::sleep(Duration::from_millis(100)); // Simulate some work
        
        println!("Thread 2: Trying to lock resource1...");
        let _lock1 = r1_clone.lock().unwrap();
        println!("Thread 2: Locked resource1");
        
        println!("Thread 2: Done!");
    });

    // This will hang forever in a deadlock
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("\nIf you see this, deadlock was avoided somehow!");
}

// SOLUTION: Always acquire locks in the same order
#[allow(dead_code)]
fn correct_approach() {
    println!("\n=== Correct Approach (No Deadlock) ===\n");

    let resource1 = Arc::new(Mutex::new(1));
    let resource2 = Arc::new(Mutex::new(2));

    // Thread 1: locks resource1 then resource2
    let r1_clone = Arc::clone(&resource1);
    let r2_clone = Arc::clone(&resource2);
    let thread1 = thread::spawn(move || {
        let _lock1 = r1_clone.lock().unwrap();
        println!("Thread 1: Locked resource1");
        let _lock2 = r2_clone.lock().unwrap();
        println!("Thread 1: Locked resource2");
        println!("Thread 1: Done!");
    });

    // Thread 2: locks in SAME ORDER (resource1 then resource2)
    let r1_clone = Arc::clone(&resource1);
    let r2_clone = Arc::clone(&resource2);
    let thread2 = thread::spawn(move || {
        let _lock1 = r1_clone.lock().unwrap(); // Same order as thread1
        println!("Thread 2: Locked resource1");
        let _lock2 = r2_clone.lock().unwrap();
        println!("Thread 2: Locked resource2");
        println!("Thread 2: Done!");
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("\nNo deadlock! Both threads completed successfully.");
}
