// Example: Buffering and batching patterns

use futures::stream::{self, StreamExt};
use tokio::time::{sleep, Duration, Instant};

async fn process_item(item: i32) -> String {
    sleep(Duration::from_millis(100)).await;
    format!("Processed: {}", item)
}

#[tokio::main]
async fn main() {
    println!("=== Buffering and Batching ===\n");

    // Example 1: Sequential processing (slow)
    println!("Example 1: Sequential processing");
    let start = Instant::now();
    
    let results: Vec<_> = stream::iter(1..=5)
        .then(process_item)
        .collect()
        .await;
    
    println!("Results: {:?}", results);
    println!("Time: {:?}\n", start.elapsed());
    // Total: ~500ms (5 * 100ms sequential)

    // Example 2: Buffered processing (faster!)
    println!("Example 2: Buffered concurrent processing");
    let start = Instant::now();
    
    let results: Vec<_> = stream::iter(1..=5)
        .map(process_item)
        .buffered(3)  // Process up to 3 concurrently
        .collect()
        .await;
    
    println!("Results: {:?}", results);
    println!("Time: {:?}\n", start.elapsed());
    // Total: ~200ms (ceil(5/3) * 100ms with concurrency)

    // Example 3: Unordered buffering (fastest, order not preserved)
    println!("Example 3: Buffer unordered (results can be out of order)");
    let start = Instant::now();
    
    let results: Vec<_> = stream::iter(1..=5)
        .map(|n| async move {
            sleep(Duration::from_millis(100 * (6 - n) as u64)).await;
            n
        })
        .buffer_unordered(5)
        .collect()
        .await;
    
    println!("Results (potentially out of order): {:?}", results);
    println!("Time: {:?}\n", start.elapsed());

    // Example 4: Batching items
    println!("Example 4: Processing in batches");
    
    let batches = stream::iter(1..=10)
        .ready_chunks(3);  // Collect chunks of 3
    
    tokio::pin!(batches);
    
    let mut batch_num = 1;
    while let Some(batch) = batches.next().await {
        println!("Processing batch {}: {:?}", batch_num, batch);
        // Simulate batch processing
        sleep(Duration::from_millis(100)).await;
        batch_num += 1;
    }
    println!();

    // Example 5: Rate limiting
    println!("Example 5: Throttling stream");
    
    let throttled = stream::iter(1..=5)
        .then(|n| async move {
            sleep(Duration::from_millis(200)).await;  // Rate limit
            n
        });
    
    let start = Instant::now();
    let results: Vec<_> = throttled.collect().await;
    println!("Throttled results: {:?}", results);
    println!("Time: {:?} (enforced 200ms between items)\n", start.elapsed());

    println!("Key Insights:");
    println!("- buffered(N): Preserve order, process N concurrently");
    println!("- buffer_unordered(N): Fastest, order not preserved");
    println!("- ready_chunks(N): Batch items for bulk processing");
}
