// Sections 17.5-17.6: Deep dive into async traits and choosing concurrency models

mod ch17_05_traits;
mod ch17_06_futures_tasks_threads;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::{sleep, Duration};

/// A simple custom Future that returns after a delay
struct Delay {
    sleep: Pin<Box<tokio::time::Sleep>>,
    value: i32,
}

impl Delay {
    fn new(value: i32, duration: Duration) -> Self {
        Self {
            sleep: Box::pin(sleep(duration)),
            value,
        }
    }
}

impl Future for Delay {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.sleep.as_mut().poll(cx) {
            Poll::Ready(()) => {
                println!("Delay future ready with value: {}", self.value);
                Poll::Ready(self.value)
            }
            Poll::Pending => {
                println!("Delay future still pending...");
                Poll::Pending
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("=== Understanding Async Traits ===\n");

    // Run the comprehensive trait explanations from ch17_05
    ch17_05_traits::demonstrate_traits();
    println!();

    // Example 1: Using our custom Future
    println!("Example 1: Custom Future implementation");
    let future = Delay::new(42, Duration::from_secs(1));
    let result = future.await;
    println!("Got: {}\n", result);

    // Example 2: What async fn generates
    println!("Example 2: async fn is syntactic sugar");

    // This async fn:
    async fn example() -> String {
        sleep(Duration::from_millis(100)).await;
        String::from("Done")
    }

    // Is approximately equivalent to:
    #[allow(clippy::manual_async_fn)]
    fn example_desugared() -> impl Future<Output = String> {
        async {
            sleep(Duration::from_millis(100)).await;
            String::from("Done")
        }
    }

    let result1 = example().await;
    let result2 = example_desugared().await;
    println!("async fn: {}", result1);
    println!("desugared: {}\n", result2);

    // Example 3: State machine concept
    println!("Example 3: Async generates state machines");
    println!("Each .await point is a state transition");
    println!("Runtime polls the future to advance state\n");

    // Example 4: Pinning requirement
    println!("Example 4: Why Pin is needed");
    println!("Self-referential types can't move in memory");
    println!("Pin<&mut Self> guarantees no movement");
    println!("See pin_unpin.rs for detailed examples\n");

    println!("Key Concepts:");
    println!("- Future trait: poll() returns Poll<Output>");
    println!("- Poll::Pending: Not ready yet, wake me later");
    println!("- Poll::Ready(value): Future completed");
    println!("- Pin: Prevents moving self-referential data");

    // Run Chapter 17.6 demonstrations
    println!("\n");
    ch17_06_futures_tasks_threads::run_all_demos().await;
}
