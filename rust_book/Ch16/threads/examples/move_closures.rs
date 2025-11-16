// Example: Move Closures with Threads
// Demonstrates transferring ownership to thread closures

use std::thread;

fn main() {
    println!("=== Move Closures Example ===\n");

    // Example 1: Moving a vector to a thread
    println!("Example 1: Moving a vector");
    let v = vec![1, 2, 3];
    println!("Created vector in main: {:?}", v);

    let handle = thread::spawn(move || {
        println!("Thread received vector: {:?}", v);
        v.iter().sum::<i32>()
    });

    let sum = handle.join().unwrap();
    println!("Sum computed in thread: {}\n", sum);

    // v is no longer accessible here - it was moved

    // Example 2: Moving multiple values
    println!("Example 2: Moving multiple values");
    let name = String::from("Alice");
    let age = 30;
    let hobbies = vec!["reading", "coding", "hiking"];

    let handle = thread::spawn(move || {
        println!("Thread processing:");
        println!("  Name: {}", name);
        println!("  Age: {}", age);
        println!("  Hobbies: {:?}", hobbies);
    });

    handle.join().unwrap();

    // name and hobbies are moved, age was copied (i32 implements Copy)
    println!("\nAge in main (copied): {}", age);
    // println!("{}", name); // Would error - name was moved

    // Example 3: Cloning before move when we need to keep a copy
    println!("\nExample 3: Cloning before move");
    let original_data = vec![10, 20, 30];
    let thread_data = original_data.clone();

    let handle = thread::spawn(move || {
        println!("Thread has: {:?}", thread_data);
    });

    println!("Main still has: {:?}", original_data);
    handle.join().unwrap();
}
