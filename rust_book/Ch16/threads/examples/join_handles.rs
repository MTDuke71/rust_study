// Example: Working with Join Handles
// Demonstrates waiting for threads to complete and collecting results

use std::thread;

fn main() {
    println!("=== Join Handles Example ===\n");

    // Spawn multiple threads and collect their handles
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Thread {} starting...", i);
            let result = i * i;
            println!("Thread {} computed: {}", i, result);
            result
        });
        handles.push(handle);
    }

    println!("\nMain thread waiting for all threads to complete...\n");

    // Join all threads and collect results
    let results: Vec<i32> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect();

    println!("\nAll threads completed!");
    println!("Results: {:?}", results);
    println!("Sum of results: {}", results.iter().sum::<i32>());
}
