// Example: Understanding the Future trait
// Shows what happens under the hood

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::Sleep;

/// A custom Future that completes after a delay
struct DelayedValue {
    value: i32,
    sleep: Pin<Box<Sleep>>,
}

impl DelayedValue {
    fn new(value: i32, duration: Duration) -> Self {
        Self {
            value,
            sleep: Box::pin(tokio::time::sleep(duration)),
        }
    }
}

impl Future for DelayedValue {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Poll the inner sleep future
        match self.sleep.as_mut().poll(cx) {
            Poll::Ready(()) => {
                println!("DelayedValue is ready!");
                Poll::Ready(self.value)
            }
            Poll::Pending => {
                println!("DelayedValue is pending...");
                Poll::Pending
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("=== Understanding the Future Trait ===\n");

    // Example 1: Using our custom Future
    println!("Example 1: Custom Future implementation");
    let future = DelayedValue::new(42, Duration::from_secs(2));

    println!("Awaiting custom future...");
    let result = future.await;
    println!("Got: {}\n", result);

    // Example 2: What async fn creates
    println!("Example 2: async fn creates a Future");

    async fn example_async() -> String {
        tokio::time::sleep(Duration::from_secs(1)).await;
        String::from("Done!")
    }

    // This is approximately what the compiler generates:
    // fn example_async() -> impl Future<Output = String> { ... }

    let future = example_async(); // Returns Future, doesn't execute
    println!("Created future from async fn");

    let result = future.await; // Poll the future to completion
    println!("Result: {}\n", result);

    // Example 3: Demonstrating lazy evaluation
    println!("Example 3: Futures are lazy");

    let future = async {
        println!("This only prints when awaited!");
        100
    };

    println!("Future created but not awaited yet");
    std::thread::sleep(Duration::from_secs(1));
    println!("Still hasn't printed...");

    let value = future.await;
    println!("Now it printed! Value: {}", value);
}
