// Example: Implementing the Future trait manually

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::{sleep_until, Instant, Sleep};

/// A counter that increments every interval
struct PeriodicCounter {
    count: u32,
    max: u32,
    interval: Duration,
    next_tick: Pin<Box<Sleep>>,
}

impl PeriodicCounter {
    fn new(max: u32, interval: Duration) -> Self {
        let now = Instant::now();
        Self {
            count: 0,
            max,
            interval,
            next_tick: Box::pin(sleep_until(now + interval)),
        }
    }
}

impl Future for PeriodicCounter {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Poll the sleep future
        match self.next_tick.as_mut().poll(cx) {
            Poll::Ready(()) => {
                self.count += 1;
                println!("Count: {}/{}", self.count, self.max);

                if self.count >= self.max {
                    Poll::Ready(self.count)
                } else {
                    // Schedule next tick
                    let next = Instant::now() + self.interval;
                    self.next_tick = Box::pin(sleep_until(next));
                    
                    // Tell runtime to poll us again
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

/// A future that completes based on a condition
struct ConditionalFuture<F, C>
where
    F: Future,
    C: Fn(&F::Output) -> bool,
{
    inner: Pin<Box<F>>,
    condition: C,
}

impl<F, C> Future for ConditionalFuture<F, C>
where
    F: Future,
    C: Fn(&F::Output) -> bool,
{
    type Output = F::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.inner.as_mut().poll(cx) {
            Poll::Ready(value) => {
                if (self.condition)(&value) {
                    Poll::Ready(value)
                } else {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[tokio::main]
async fn main() {
    println!("=== Custom Future Implementations ===\n");

    // Example 1: Periodic counter
    println!("Example 1: Counting periodically");
    let counter = PeriodicCounter::new(3, Duration::from_millis(500));
    let final_count = counter.await;
    println!("Final count: {}\n", final_count);

    // Example 2: Conditional future
    println!("Example 2: Conditional completion");
    
    async fn get_number() -> i32 {
        tokio::time::sleep(Duration::from_millis(100)).await;
        42
    }
    
    let conditional = ConditionalFuture {
        inner: Box::pin(get_number()),
        condition: |n| *n > 40,
    };
    
    let result = conditional.await;
    println!("Got: {} (passed condition)\n", result);

    println!("Key Insights:");
    println!("- Implementing Future gives fine-grained control");
    println!("- poll() can return Pending multiple times");
    println!("- use cx.waker() to schedule re-polling");
    println!("- Most code should use async/await instead!");
}
