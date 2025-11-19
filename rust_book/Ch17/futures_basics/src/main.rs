// Section 17.1: Futures and the Async Syntax
// Basic async/await demonstration

use tokio::time::{sleep, Duration};

/// An async function that simulates fetching data from a network
async fn fetch_user_data(user_id: u32) -> String {
    println!("Fetching user {}...", user_id);
    
    // Simulate network delay - this is non-blocking!
    sleep(Duration::from_secs(1)).await;
    
    format!("User {} data", user_id)
}

/// An async function that processes data
async fn process_data(data: String) -> String {
    println!("Processing: {}", data);
    
    sleep(Duration::from_millis(500)).await;
    
    format!("Processed: {}", data)
}

#[tokio::main]
async fn main() {
    println!("=== Futures Basics ===\n");
    
    // Example 1: Basic async/await
    println!("Example 1: Sequential async operations");
    let data = fetch_user_data(1).await;
    let processed = process_data(data).await;
    println!("Result: {}\n", processed);
    
    // Example 2: Futures are lazy
    println!("Example 2: Futures don't run until .await");
    let future1 = fetch_user_data(2);  // Creates future but doesn't execute
    println!("Future created, but nothing happened yet!");
    
    // Now it executes
    let data = future1.await;
    println!("Result: {}\n", data);
    
    // Example 3: Async blocks
    println!("Example 3: Async blocks create inline futures");
    let future = async {
        println!("Inside async block");
        sleep(Duration::from_millis(100)).await;
        42
    };
    
    let result = future.await;
    println!("Async block returned: {}\n", result);
    
    // Example 4: Multiple sequential awaits
    println!("Example 4: Sequential execution (slow)");
    let start = std::time::Instant::now();
    
    let data1 = fetch_user_data(3).await;
    let data2 = fetch_user_data(4).await;
    let data3 = fetch_user_data(5).await;
    
    println!("Got: {}, {}, {}", data1, data2, data3);
    println!("Sequential time: {:?}", start.elapsed());
    println!("(See async_concurrency examples for concurrent execution)");
}
