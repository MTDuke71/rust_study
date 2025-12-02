# Async Streams - Sequential Futures

*Navigation: [[zettel-index]] | [[rust-concurrency-moc]] | [[async-await-basics]]*

---

## Overview

**Async streams** in Rust represent sequences of values that arrive asynchronously over time. Unlike regular iterators that provide values immediately, streams yield futures that must be awaited. This pattern is essential for processing data pipelines, handling network events, and managing continuous data flows.

**Core Concept**: Streams are to async what iterators are to sync - they provide sequential access to values, but each value might require waiting.

## The Stream Trait

```rust
use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {
    type Item;
    
    fn poll_next(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}
```

**Key Differences from Iterator**:

- **Iterator**: `fn next(&mut self) -> Option<Self::Item>` (immediate)
- **Stream**: `fn poll_next(...) -> Poll<Option<Self::Item>>` (potentially async)
- **Item arrival**: Iterator items exist immediately; stream items may need waiting

## Stream Processing Patterns

### Pattern 1: Async Iteration with `for await`

```rust
use futures::stream::{self, StreamExt};

async fn process_stream() {
    let stream = stream::iter(vec![1, 2, 3, 4, 5])
        .then(|x| async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            x * 2
        });
    
    // Process each item as it arrives
    let mut stream = std::pin::pin!(stream);
    while let Some(value) = stream.next().await {
        println!("Processed: {}", value);
        // Output: 2, 4, 6, 8, 10 (with 100ms delay between each)
    }
}
```

### Pattern 2: Stream Transformation Pipeline

```rust
use futures::stream::{self, StreamExt};

async fn data_pipeline() {
    let source_data = vec!["hello", "world", "rust", "streams"];
    
    let processed_stream = stream::iter(source_data)
        .map(|s| s.to_uppercase())           // Transform: hello → HELLO
        .filter(|s| future::ready(s.len() > 4)) // Async filter
        .then(|s| async move {               // Async transformation
            tokio::time::sleep(Duration::from_millis(50)).await;
            format!("[{}]", s)               // HELLO → [HELLO]
        })
        .take(3);                           // Limit to first 3 items
    
    let results: Vec<String> = processed_stream.collect().await;
    println!("Pipeline results: {:?}", results);
    // Output: ["[HELLO]", "[WORLD]", "[STREAMS]"]
}
```

### Pattern 3: Custom Stream Implementation

```rust
use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::{Duration, Instant, Sleep};

/// A stream that produces numbers at regular intervals
struct IntervalStream {
    current: u32,
    max: u32,
    interval: Duration,
    sleep: Pin<Box<Sleep>>,
}

impl IntervalStream {
    fn new(max: u32, interval: Duration) -> Self {
        Self {
            current: 0,
            max,
            interval,
            sleep: Box::pin(tokio::time::sleep(interval)),
        }
    }
}

impl Stream for IntervalStream {
    type Item = u32;
    
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<u32>> {
        // Check if we've reached the limit
        if self.current >= self.max {
            return Poll::Ready(None);
        }
        
        // Poll the sleep future
        match self.sleep.as_mut().poll(cx) {
            Poll::Ready(()) => {
                // Timer expired - produce next value
                let value = self.current;
                self.current += 1;
                
                // Reset timer for next interval
                self.sleep = Box::pin(tokio::time::sleep(self.interval));
                
                Poll::Ready(Some(value))
            }
            Poll::Pending => Poll::Pending,  // Still waiting for timer
        }
    }
}

// Usage
async fn custom_stream_example() {
    let stream = IntervalStream::new(5, Duration::from_millis(200));
    let mut pinned = std::pin::pin!(stream);
    
    while let Some(value) = pinned.next().await {
        println!("Interval value: {}", value);
        // Prints 0, 1, 2, 3, 4 with 200ms between each
    }
}
```

### Pattern 4: Buffering and Batching

```rust
use futures::stream::{self, StreamExt};
use futures::future;

async fn buffering_example() {
    // Create a stream of network requests (simulated)
    let request_stream = stream::iter(0..10)
        .then(|id| async move {
            // Simulate network delay
            tokio::time::sleep(Duration::from_millis(100)).await;
            format!("Request-{}", id)
        });
    
    // Buffer up to 3 concurrent requests
    let buffered = request_stream.buffered(3);
    
    // Collect results as they complete
    let results: Vec<String> = buffered.collect().await;
    println!("Buffered results: {:?}", results);
}

async fn batching_example() {
    let data_stream = stream::iter(1..=20);
    
    // Group items into chunks of 5
    let batched = data_stream.chunks(5);
    
    let mut pinned = std::pin::pin!(batched);
    while let Some(batch) = pinned.next().await {
        println!("Processing batch: {:?}", batch);
        // Prints: [1,2,3,4,5], [6,7,8,9,10], [11,12,13,14,15], [16,17,18,19,20]
    }
}
```

### Pattern 5: Error Handling in Streams

```rust
use futures::stream::{self, StreamExt, TryStreamExt};
use futures::future;

async fn error_handling_example() {
    // Stream that occasionally fails
    let risky_stream = stream::iter(0..10)
        .then(|i| async move {
            if i == 3 || i == 7 {
                Err(format!("Error at {}", i))
            } else {
                Ok(i * 10)
            }
        });
    
    // Handle errors with try_fold
    let result = risky_stream
        .try_fold(Vec::new(), |mut acc, value| async move {
            acc.push(value);
            Ok(acc)
        })
        .await;
    
    match result {
        Ok(values) => println!("Collected before error: {:?}", values),
        Err(e) => println!("Stream failed: {}", e),
    }
    
    // Alternative: Filter out errors and continue
    let filtered_stream = stream::iter(0..10)
        .then(|i| async move {
            if i == 3 || i == 7 {
                Err(format!("Error at {}", i))
            } else {
                Ok(i * 10)
            }
        })
        .filter_map(|result| future::ready(result.ok()));
    
    let success_values: Vec<i32> = filtered_stream.collect().await;
    println!("Success values only: {:?}", success_values);
    // Output: [0, 10, 20, 40, 50, 60, 80, 90]
}
```

## Advanced Stream Patterns

### Pattern 6: Stream Merging and Racing

```rust
use futures::stream::{self, StreamExt, select_all};

async fn stream_merging() {
    // Create multiple streams with different intervals
    let stream1 = stream::iter(vec!["A1", "A2", "A3"])
        .then(|s| async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            format!("Stream1: {}", s)
        });
    
    let stream2 = stream::iter(vec!["B1", "B2"])
        .then(|s| async move {
            tokio::time::sleep(Duration::from_millis(150)).await;
            format!("Stream2: {}", s)
        });
    
    let stream3 = stream::iter(vec!["C1", "C2", "C3", "C4"])
        .then(|s| async move {
            tokio::time::sleep(Duration::from_millis(75)).await;
            format!("Stream3: {}", s)
        });
    
    // Merge all streams - items arrive in completion order
    let merged = select_all(vec![
        stream1.boxed(),
        stream2.boxed(),
        stream3.boxed(),
    ]);
    
    let results: Vec<String> = merged.collect().await;
    println!("Merged order: {:?}", results);
    // Items arrive based on timing, not source stream order
}
```

### Pattern 7: Stream Splitting and Fan-out

```rust
use futures::stream::{self, StreamExt};

async fn stream_splitting() {
    let source = stream::iter(1..=10);
    
    // Split stream into even and odd
    let (even_tx, even_rx) = tokio::sync::mpsc::unbounded_channel();
    let (odd_tx, odd_rx) = tokio::sync::mpsc::unbounded_channel();
    
    // Producer task
    let producer = tokio::spawn(async move {
        let mut pinned = std::pin::pin!(source);
        while let Some(value) = pinned.next().await {
            if value % 2 == 0 {
                even_tx.send(value).unwrap();
            } else {
                odd_tx.send(value).unwrap();
            }
        }
    });
    
    // Consumer tasks
    let even_consumer = tokio::spawn(async move {
        let stream = tokio_stream::wrappers::UnboundedReceiverStream::new(even_rx);
        let evens: Vec<i32> = stream.collect().await;
        println!("Even numbers: {:?}", evens);
    });
    
    let odd_consumer = tokio::spawn(async move {
        let stream = tokio_stream::wrappers::UnboundedReceiverStream::new(odd_rx);
        let odds: Vec<i32> = stream.collect().await;
        println!("Odd numbers: {:?}", odds);
    });
    
    // Wait for all tasks
    let _ = tokio::join!(producer, even_consumer, odd_consumer);
}
```

## Real-World Applications

### Application 1: Network Event Processing

```rust
use futures::stream::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::wrappers::TcpListenerStream;

async fn network_event_stream() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let listener_stream = TcpListenerStream::new(listener);
    
    listener_stream
        .map(|conn_result| {
            match conn_result {
                Ok(stream) => {
                    println!("New connection: {:?}", stream.peer_addr());
                    // Handle connection in separate task
                    tokio::spawn(handle_connection(stream));
                }
                Err(e) => println!("Connection error: {}", e),
            }
        })
        .collect::<()>()  // Run forever
        .await;
}

async fn handle_connection(stream: TcpStream) {
    // Connection handling logic
    println!("Handling connection...");
}
```

### Application 2: Data Processing Pipeline

```rust
use futures::stream::{self, StreamExt};

#[derive(Debug, Clone)]
struct LogEntry {
    timestamp: u64,
    level: String,
    message: String,
}

async fn log_processing_pipeline() {
    // Simulate log entries coming from multiple sources
    let log_stream = stream::iter(vec![
        LogEntry { timestamp: 1, level: "INFO".to_string(), message: "Server started".to_string() },
        LogEntry { timestamp: 2, level: "ERROR".to_string(), message: "Database connection failed".to_string() },
        LogEntry { timestamp: 3, level: "INFO".to_string(), message: "User logged in".to_string() },
        LogEntry { timestamp: 4, level: "WARN".to_string(), message: "High memory usage".to_string() },
        LogEntry { timestamp: 5, level: "ERROR".to_string(), message: "API timeout".to_string() },
    ]);
    
    // Processing pipeline
    let processed = log_stream
        .filter(|entry| {
            // Only process ERROR and WARN levels
            let is_important = matches!(entry.level.as_str(), "ERROR" | "WARN");
            futures::future::ready(is_important)
        })
        .then(|entry| async move {
            // Simulate async enrichment (e.g., lookup user info)
            tokio::time::sleep(Duration::from_millis(10)).await;
            format!("[{}] {}: {}", entry.timestamp, entry.level, entry.message)
        })
        .buffered(2);  // Process up to 2 entries concurrently
    
    let alerts: Vec<String> = processed.collect().await;
    println!("Generated alerts: {:#?}", alerts);
}
```

## Performance Characteristics

### Memory Efficiency

**Stream advantages:**

- **Lazy evaluation**: Items created only when needed
- **Constant memory**: No need to store entire sequence
- **Streaming processing**: Handle datasets larger than memory

**Example - Processing large datasets:**

```rust
async fn memory_efficient_processing() {
    // Process millions of items without loading all into memory
    let large_dataset_stream = stream::iter(0..1_000_000)
        .map(|i| i * i)                     // Transform each item
        .filter(|&n| futures::future::ready(n % 2 == 0))  // Keep only even squares
        .take(100);                         // Take only first 100 results
    
    // Only 100 items max in memory at any time
    let results: Vec<i32> = large_dataset_stream.collect().await;
    println!("First 100 even squares: {:?}", &results[..10]);
}
```

### Concurrency Control

```rust
use futures::stream::{self, StreamExt};

async fn concurrency_demonstration() {
    let tasks = stream::iter(0..10)
        .map(|i| async move {
            println!("Starting task {}", i);
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("Completed task {}", i);
            i
        });
    
    // Sequential (slow): ~1 second total
    let sequential_start = std::time::Instant::now();
    let _: Vec<i32> = tasks.collect().await;
    println!("Sequential took: {:?}", sequential_start.elapsed());
    
    // Concurrent (fast): ~100ms total  
    let concurrent_start = std::time::Instant::now();
    let concurrent_tasks = stream::iter(0..10)
        .map(|i| async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            i
        });
    let _: Vec<i32> = concurrent_tasks.buffered(10).collect().await;
    println!("Concurrent took: {:?}", concurrent_start.elapsed());
}
```

## Integrator Perspective (AUTOSAR Parallels)

| **Rust Async Streams** | **AUTOSAR** | **Integration Pattern** |
|------------------------|-------------|------------------------|
| Stream items | Event notifications | Sequential data processing |
| `.next().await` | Event handling callback | Await next event |
| `buffered(n)` | Event queue depth | Concurrent event processing |
| Stream pipeline | Event filter chain | Multi-stage event processing |
| Error handling | Diagnostic event manager | Fault-tolerant event streams |
| `select_all` | Multiple event sources | Event multiplexing |
| Channel streams | Inter-ECU communication | Async message passing |

**Key Integration Insights:**

1. **Event Streams**: AUTOSAR events arriving over time ↔ Stream items
2. **Pipeline Processing**: CAN message filtering/routing ↔ Stream transformations  
3. **Buffering**: Event queue management ↔ Stream buffering
4. **Error Resilience**: Diagnostic handling ↔ Stream error propagation

## Common Pitfalls and Solutions

### Pitfall 1: Forgetting to Pin Streams

```rust
// ❌ Wrong: Stream not pinned
async fn wrong_way() {
    let stream = stream::iter(vec![1, 2, 3]);
    while let Some(item) = stream.next().await {  // Compile error!
        println!("{}", item);
    }
}

// ✅ Correct: Pin the stream
async fn correct_way() {
    let stream = stream::iter(vec![1, 2, 3]);
    let mut pinned = std::pin::pin!(stream);
    while let Some(item) = pinned.next().await {
        println!("{}", item);
    }
}
```

### Pitfall 2: Blocking Operations in Async Context

```rust
// ❌ Wrong: Blocking in async stream
let blocking_stream = stream::iter(0..5)
    .map(|i| {
        std::thread::sleep(Duration::from_millis(100));  // Blocks entire runtime!
        i * 2
    });

// ✅ Correct: Use async operations
let async_stream = stream::iter(0..5)
    .then(|i| async move {
        tokio::time::sleep(Duration::from_millis(100)).await;  // Non-blocking
        i * 2
    });
```

### Pitfall 3: Unbounded Memory Growth

```rust
// ❌ Dangerous: All items collected into memory
let all_items: Vec<i32> = huge_stream.collect().await;  // May exhaust memory

// ✅ Better: Process items one by one
let mut pinned = std::pin::pin!(huge_stream);
while let Some(item) = pinned.next().await {
    process_item(item).await;  // Constant memory usage
}

// ✅ Alternative: Use try_fold for accumulation
let result = huge_stream
    .try_fold(0, |acc, item| async move {
        Ok(acc + process_item(item).await?)
    })
    .await;
```

## Related Concepts

**Foundations:**
- [[async-await-basics]] - Understanding futures and the async execution model
- [[futures-and-polling]] - How the Future trait drives async computation
- [[pin-and-unpin]] - Memory safety for self-referential stream types

**Concurrency Patterns:**
- [[async-concurrency]] - Concurrent execution patterns with join!/select!
- [[message-passing-channels]] - Channel-based communication with streams
- [[tokio-runtime]] - Runtime configuration for stream processing

**Advanced Techniques:**
- [[async-trait-objects]] - Dynamic dispatch with streams (`Box<dyn Stream>`)
- [[custom-async-primitives]] - Building custom stream types
- [[backpressure-control]] - Managing flow control in stream pipelines

**Performance:**
- [[async-performance-patterns]] - Optimizing stream processing pipelines
- [[memory-efficient-streaming]] - Handling large datasets with constant memory
- [[stream-parallelism]] - Balancing concurrency vs sequential processing

---

## Complete Working Example

```rust
use futures::stream::{self, StreamExt};
use std::time::Duration;
use tokio;

/// Comprehensive stream processing demonstration
#[tokio::main]
async fn main() {
    // 1. Basic stream creation and iteration
    println!("=== Basic Stream Processing ===");
    let numbers = stream::iter(1..=5);
    let doubled: Vec<i32> = numbers.map(|x| x * 2).collect().await;
    println!("Doubled: {:?}", doubled);
    
    // 2. Async transformations
    println!("\n=== Async Transformations ===");
    let async_stream = stream::iter(1..=3)
        .then(|x| async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            x * x
        });
    
    let squared: Vec<i32> = async_stream.collect().await;
    println!("Async squared: {:?}", squared);
    
    // 3. Error handling
    println!("\n=== Error Handling ===");
    let risky_stream = stream::iter(0..5)
        .then(|i| async move {
            if i == 2 {
                Err(format!("Error at {}", i))
            } else {
                Ok(i * 10)
            }
        });
    
    let safe_values: Vec<i32> = risky_stream
        .filter_map(|result| futures::future::ready(result.ok()))
        .collect()
        .await;
    println!("Safe values: {:?}", safe_values);
    
    // 4. Buffered concurrency
    println!("\n=== Buffered Concurrency ===");
    let start = std::time::Instant::now();
    let concurrent_results: Vec<String> = stream::iter(0..5)
        .then(|i| async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            format!("Task-{}", i)
        })
        .buffered(3)  // Up to 3 concurrent tasks
        .collect()
        .await;
    
    println!("Concurrent results: {:?}", concurrent_results);
    println!("Buffered time: {:?}", start.elapsed());
    
    println!("\n=== Stream Complete ===");
}

async fn process_item(item: i32) -> i32 {
    tokio::time::sleep(Duration::from_millis(10)).await;
    item + 100
}
```

---

*Tags: #rust #async #streams #futures #iterator #pipeline #concurrent-processing #buffering*

*Links: [[zettel-index]] | [[async-await-basics]] | [[async-concurrency]] | [[futures-and-polling]] | [[rust-concurrency-moc]] | [[future-trait-deep-dive]]*

*Last Updated: December 1, 2025*