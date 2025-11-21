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
    
    for use_cache in [false, true] {
        println!("\n  Testing with use_cache = {}", use_cache);
        let start = std::time::Instant::now();
        
        let data = tokio::select! {
            cached = fetch_from_cache(), if use_cache => {
                println!("  ✓ Using cached data");
                cached
            }
            fresh = fetch_from_network() => {
                println!("  ✓ Fetched fresh data");
                fresh
            }
        };
        
        let elapsed = start.elapsed();
        println!("  Data: {}", data);
        println!("  Time: {:.0}ms", elapsed.as_secs_f64() * 1000.0);
    }
    
    println!();

    // Example 4: Multiple channels
    println!("Example 4: Waiting on multiple channels");
    
    let (tx1, mut rx1) = tokio::sync::mpsc::channel::<i32>(10);
    let (tx2, mut rx2) = tokio::sync::mpsc::channel::<String>(10);
    
    // Send messages - number arrives at 300ms, string at 500ms
    tokio::spawn(async move {
        sleep(Duration::from_millis(300)).await;
        tx1.send(42).await.unwrap();
        
        sleep(Duration::from_millis(200)).await;
        tx2.send(String::from("Hello")).await.unwrap();
    });
    
    // Receive from whichever channel is ready first
    println!("\n  Scenario 1: Number first (300ms), then String (500ms)");
    let start = std::time::Instant::now();
    for _ in 0..2 {
        tokio::select! {
            Some(num) = rx1.recv() => {
                let elapsed = start.elapsed();
                println!("    [{}ms] Received number: {}", elapsed.as_millis(), num);
            }
            Some(msg) = rx2.recv() => {
                let elapsed = start.elapsed();
                println!("    [{}ms] Received message: {}", elapsed.as_millis(), msg);
            }
        }
    }

    // Example 4b: Reversed timing - string arrives first
    println!("\n  Scenario 2: String first (200ms), then Number (300ms)");
    
    let (tx1, mut rx1) = tokio::sync::mpsc::channel::<i32>(10);
    let (tx2, mut rx2) = tokio::sync::mpsc::channel::<String>(10);
    
    // Send messages - string arrives at 200ms, number at 300ms
    tokio::spawn(async move {
        sleep(Duration::from_millis(200)).await;
        tx2.send(String::from("Hello")).await.unwrap();
        
        sleep(Duration::from_millis(100)).await;
        tx1.send(42).await.unwrap();
    });
    
    // Receive from whichever channel is ready first
    let start = std::time::Instant::now();
    for _ in 0..2 {
        tokio::select! {
            Some(num) = rx1.recv() => {
                let elapsed = start.elapsed();
                println!("    [{}ms] Received number: {}", elapsed.as_millis(), num);
            }
            Some(msg) = rx2.recv() => {
                let elapsed = start.elapsed();
                println!("    [{}ms] Received message: {}", elapsed.as_millis(), msg);
            }
        }
    }

    // Example 4c: Both senders running concurrently from time 0
    println!("\n  Scenario 3: Both senders start at time 0, run in parallel");
    
    let (tx1, mut rx1) = tokio::sync::mpsc::channel::<i32>(10);
    let (tx2, mut rx2) = tokio::sync::mpsc::channel::<String>(10);
    
    let start = std::time::Instant::now();
    
    // Spawn TWO separate tasks - they run concurrently!
    let sender1 = tokio::spawn(async move {
        let task_start = std::time::Instant::now();
        println!("    [{}ms] Sender 1 starting...", task_start.elapsed().as_millis());
        sleep(Duration::from_millis(300)).await;
        tx1.send(42).await.unwrap();
        println!("    [{}ms] Sender 1 sent number", task_start.elapsed().as_millis());
    });
    
    let sender2 = tokio::spawn(async move {
        let task_start = std::time::Instant::now();
        println!("    [{}ms] Sender 2 starting...", task_start.elapsed().as_millis());
        sleep(Duration::from_millis(200)).await;
        tx2.send(String::from("Hello")).await.unwrap();
        println!("    [{}ms] Sender 2 sent string", task_start.elapsed().as_millis());
    });
    
    // Receive from whichever channel is ready first
    for _ in 0..2 {
        tokio::select! {
            Some(num) = rx1.recv() => {
                let elapsed = start.elapsed();
                println!("    [{}ms] Received number: {}", elapsed.as_millis(), num);
            }
            Some(msg) = rx2.recv() => {
                let elapsed = start.elapsed();
                println!("    [{}ms] Received message: {}", elapsed.as_millis(), msg);
            }
        }
    }
    
    // Wait for both senders to complete
    sender1.await.unwrap();
    sender2.await.unwrap();
    
    println!("    Both senders completed concurrently!");
    
    println!();
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
