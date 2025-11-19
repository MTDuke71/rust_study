// Example: tokio::select! for racing futures

use tokio::time::{sleep, Duration};

async fn fast_operation() -> &'static str {
    sleep(Duration::from_millis(500)).await;
    "Fast result"
}

async fn slow_operation() -> &'static str {
    sleep(Duration::from_secs(2)).await;
    "Slow result"
}

#[tokio::main]
async fn main() {
    println!("=== tokio::select! Examples ===\n");

    // Example 1: Race between two futures
    println!("Example 1: Racing two operations");
    
    let result = tokio::select! {
        r = fast_operation() => {
            println!("Fast operation won!");
            r
        }
        r = slow_operation() => {
            println!("Slow operation won!");
            r
        }
    };
    
    println!("Got: {}\n", result);

    // Example 2: Timeout pattern
    println!("Example 2: Implementing timeout");
    
    let result = tokio::select! {
        data = slow_operation() => {
            println!("Operation completed");
            Some(data)
        }
        _ = sleep(Duration::from_secs(1)) => {
            println!("Operation timed out!");
            None
        }
    };
    
    println!("Result: {:?}\n", result);

    // Example 3: Conditional branches
    println!("Example 3: Conditional select branches");
    
    let use_cache = true;
    
    let data = tokio::select! {
        cached = fetch_from_cache(), if use_cache => {
            println!("Using cached data");
            cached
        }
        fresh = fetch_from_network() => {
            println!("Fetched fresh data");
            fresh
        }
    };
    
    println!("Data: {}\n", data);

    // Example 4: Multiple channels
    println!("Example 4: Waiting on multiple channels");
    
    let (tx1, mut rx1) = tokio::sync::mpsc::channel::<i32>(10);
    let (tx2, mut rx2) = tokio::sync::mpsc::channel::<String>(10);
    
    // Send messages
    tokio::spawn(async move {
        sleep(Duration::from_millis(300)).await;
        tx1.send(42).await.unwrap();
        
        sleep(Duration::from_millis(200)).await;
        tx2.send(String::from("Hello")).await.unwrap();
    });
    
    // Receive from whichever channel is ready first
    for _ in 0..2 {
        tokio::select! {
            Some(num) = rx1.recv() => {
                println!("Received number: {}", num);
            }
            Some(msg) = rx2.recv() => {
                println!("Received message: {}", msg);
            }
        }
    }
    
    println!("\nKey Insight: select! returns when FIRST future completes");
    println!("Other futures are dropped/cancelled");
}

async fn fetch_from_cache() -> String {
    sleep(Duration::from_millis(100)).await;
    String::from("Cached data")
}

async fn fetch_from_network() -> String {
    sleep(Duration::from_secs(1)).await;
    String::from("Fresh data")
}
