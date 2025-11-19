// Sections 17.3-17.4: Streams - Asynchronous Iteration
// Working with sequences of asynchronous values

use futures::stream::{self, StreamExt};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("=== Stream Basics ===\n");

    // Example 1: Creating a stream from an iterator
    println!("Example 1: Stream from iterator");
    
    let stream = stream::iter(1..=5);
    let values: Vec<_> = stream.collect().await;
    println!("Collected: {:?}\n", values);

    // Example 2: Async transformations
    println!("Example 2: Mapping over stream");
    
    let stream = stream::iter(1..=5)
        .then(|n| async move {
            sleep(Duration::from_millis(100)).await;
            n * 2
        });
    
    let doubled: Vec<_> = stream.collect().await;
    println!("Doubled: {:?}\n", doubled);

    // Example 3: Filtering streams
    println!("Example 3: Filtering stream");
    
    let stream = stream::iter(1..=10)
        .filter(|n| futures::future::ready(n % 2 == 0));
    
    let evens: Vec<_> = stream.collect().await;
    println!("Even numbers: {:?}\n", evens);

    // Example 4: Taking first N items
    println!("Example 4: Taking first 3 items");
    
    let stream = stream::iter(1..=100)
        .take(3);
    
    let first_three: Vec<_> = stream.collect().await;
    println!("First three: {:?}\n", first_three);

    // Example 5: Folding (reducing) a stream
    println!("Example 5: Summing stream values");
    
    let sum = stream::iter(1..=5)
        .fold(0, |acc, x| async move { acc + x })
        .await;
    
    println!("Sum: {}\n", sum);

    // Example 6: Combining streams
    println!("Example 6: Chaining operations");
    
    let result = stream::iter(1..=10)
        .filter(|n| futures::future::ready(n % 2 == 0))
        .map(|n| n * 3)
        .take(3)
        .collect::<Vec<_>>()
        .await;
    
    println!("Result: {:?}\n", result);

    println!("Key Insight: Streams are like iterators, but async!");
    println!("Each element can be produced asynchronously");
}
