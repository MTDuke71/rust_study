// Example: Async iteration patterns

use tokio_stream::StreamExt;
use tokio::time::{sleep, Duration, interval};

#[tokio::main]
async fn main() {
    println!("=== Async Iteration Patterns ===\n");

    // Example 1: Manual iteration with while let
    println!("Example 1: Manual iteration");
    
    let mut stream = tokio_stream::iter(1..=5);
    
    while let Some(value) = stream.next().await {
        println!("Got: {}", value);
    }
    println!();

    // Example 2: Interval stream (timer-based)
    println!("Example 2: Interval stream (ticks every 500ms)");
    
    let mut ticker = interval(Duration::from_millis(500));
    let mut count = 0;
    
    loop {
        ticker.tick().await;
        count += 1;
        println!("Tick {}", count);
        
        if count >= 3 {
            break;
        }
    }
    println!();

    // Example 3: Channel as stream
    println!("Example 3: Channel receiver as stream");
    
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    
    // Spawn producer
    tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(i * 10).await.unwrap();
            sleep(Duration::from_millis(100)).await;
        }
    });
    
    // Consume as stream
    while let Some(value) = rx.recv().await {
        println!("Received: {}", value);
    }
    println!();

    // Example 4: Stream with early termination
    println!("Example 4: Take while condition");
    
    let stream = tokio_stream::iter(1..=10)
        .take_while(|n| *n < 6);
    
    tokio::pin!(stream);
    
    while let Some(value) = stream.next().await {
        println!("Value: {}", value);
    }
    println!();

    // Example 5: Enumerate stream (manual enumeration)
    println!("Example 5: Enumerated stream");
    
    // Streams don't have enumerate(), so we manually track the index
    let mut stream = tokio_stream::iter(vec!["apple", "banana", "cherry"]);
    let mut index = 0;
    
    while let Some(fruit) = stream.next().await {
        println!("{}: {}", index, fruit);
        index += 1;
    }
    
    println!("\nNote: Streams must be pinned for manual iteration");
    println!("Use tokio::pin! or Box::pin() before .next().await");
}
