// Example: Advanced stream processing patterns

use futures::stream::{self, StreamExt};
use tokio::time::{sleep, Duration};

async fn fetch_page(page: u32) -> Vec<String> {
    println!("Fetching page {}...", page);
    sleep(Duration::from_millis(200)).await;
    
    vec![
        format!("Item {}.1", page),
        format!("Item {}.2", page),
        format!("Item {}.3", page),
    ]
}

#[tokio::main]
async fn main() {
    println!("=== Advanced Stream Processing ===\n");

    // Example 1: Flattening nested streams
    println!("Example 1: Flattening paginated results");
    
    let pages = stream::iter(1..=3)
        .then(|page| async move {
            fetch_page(page).await
        })
        .flat_map(|items| stream::iter(items));
    
    let all_items: Vec<_> = pages.collect().await;
    println!("All items: {:?}\n", all_items);

    // Example 2: Chunking streams
    println!("Example 2: Processing in chunks");
    
    let chunks = stream::iter(1..=10)
        .chunks(3);
    
    let mut chunk_num = 1;
    tokio::pin!(chunks);
    
    while let Some(chunk) = chunks.next().await {
        println!("Chunk {}: {:?}", chunk_num, chunk);
        chunk_num += 1;
    }
    println!();

    // Example 3: Zipping streams
    println!("Example 3: Zipping two streams");
    
    let numbers = stream::iter(1..=5);
    let letters = stream::iter(vec!["a", "b", "c", "d", "e"]);
    
    let zipped = numbers.zip(letters);
    let pairs: Vec<_> = zipped.collect().await;
    println!("Zipped: {:?}\n", pairs);

    // Example 4: Concurrent processing with buffer_unordered
    println!("Example 4: Concurrent processing (buffer_unordered)");
    
    let start = std::time::Instant::now();
    
    let results = stream::iter(1..=5)
        .map(|n| async move {
            let delay = (6 - n) * 100;  // Reverse delays
            sleep(Duration::from_millis(delay as u64)).await;
            println!("Task {} complete (delayed {}ms)", n, delay);
            n * 2
        })
        .buffer_unordered(5)  // Run up to 5 concurrently
        .collect::<Vec<_>>()
        .await;
    
    println!("Results: {:?}", results);
    println!("Time: {:?} (concurrent, not sequential!)\n", start.elapsed());

    // Example 5: Custom stream creation
    println!("Example 5: Custom stream with unfold");
    
    let fibonacci = stream::unfold((0, 1), |(a, b)| async move {
        if a > 100 {
            None
        } else {
            Some((a, (b, a + b)))
        }
    });
    
    let fib_nums: Vec<_> = fibonacci.collect().await;
    println!("Fibonacci: {:?}\n", fib_nums);

    println!("Key Patterns:");
    println!("- flat_map: Flatten nested collections");
    println!("- buffer_unordered: Process concurrently");
    println!("- unfold: Create custom streams");
}
