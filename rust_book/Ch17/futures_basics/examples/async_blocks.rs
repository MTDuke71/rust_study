// Example: Async blocks and closures
// Different ways to create async code

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("=== Async Blocks and Closures ===\n");

    // Example 1: Async blocks
    println!("Example 1: Async blocks");
    
    let value = 42;
    let future = async {
        println!("Inside async block");
        sleep(Duration::from_millis(100)).await;
        value * 2
    };
    
    let result = future.await;
    println!("Result: {}\n", result);

    // Example 2: Async block with move
    println!("Example 2: Async block with move");
    
    let data = vec![1, 2, 3, 4, 5];
    let future = async move {
        println!("Processing {} items", data.len());
        sleep(Duration::from_millis(100)).await;
        data.iter().sum::<i32>()
    };
    
    // data is moved into the async block
    // println!("{:?}", data);  // ERROR: data was moved
    
    let sum = future.await;
    println!("Sum: {}\n", sum);

    // Example 3: Multiple async blocks
    println!("Example 3: Composing async blocks");
    
    let fetch_data = async {
        sleep(Duration::from_millis(500)).await;
        vec![10, 20, 30]
    };
    
    let process_data = async {
        let data = fetch_data.await;
        data.iter().map(|x| x * 2).collect::<Vec<_>>()
    };
    
    let result = process_data.await;
    println!("Processed: {:?}\n", result);

    // Example 4: Async closures (manual implementation)
    println!("Example 4: Function returning async block");
    
    fn make_async_closure(multiplier: i32) -> impl Future<Output = i32> {
        async move {
            sleep(Duration::from_millis(100)).await;
            multiplier * 2
        }
    }
    
    let result = make_async_closure(21).await;
    println!("Result: {}\n", result);

    // Example 5: Storing async blocks
    println!("Example 5: Storing futures in variables");
    
    let futures = vec![
        async { 1 },
        async { 2 },
        async { 3 },
    ];
    
    for (i, future) in futures.into_iter().enumerate() {
        let value = future.await;
        println!("Future {} returned: {}", i, value);
    }
}

use std::future::Future;
