//! Chapter 17.5: A Closer Look at the Traits for Async
//!
//! This module demonstrates the key traits from the Rust Book chapter 17-05:
//! - Future trait and Poll enum
//! - Pin and Unpin traits
//! - Stream trait
//!
//! Reference: https://doc.rust-lang.org/stable/book/ch17-05-traits-for-async.html

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// ============================================================================
// THE FUTURE TRAIT
// ============================================================================

// The Future trait as defined in std::future
//
// This is how Rust defines it (we can't redefine it, but here's the signature):
// pub trait Future {
//     type Output;
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
// }
//
// Key points:
// - `Output` associated type: what the future resolves to (like Iterator's Item)
// - `poll` method: checks if future is ready
// - `Pin<&mut Self>`: prevents self-referential futures from moving
// - `Context`: allows runtime to wake the future when ready

// The Poll enum - represents whether a future is ready
// enum Poll<T> {
//     Ready(T),   // Future completed, here's the value
//     Pending,    // Not ready yet, check again later
// }

// ============================================================================
// HOW AWAIT COMPILES TO POLL
// ============================================================================

// Conceptual desugaring of await
//
// When you write:
//     let title = page_title(url).await;
//
// Rust compiles it to something like:
//     match page_title(url).poll() {
//         Poll::Ready(page_title) => { /* Use page_title */ }
//         Poll::Pending => { /* Runtime will call poll again later */ }
//     }
//
// The runtime handles the loop and waking logic.

/// Example: What a polling loop conceptually looks like
///
/// Note: This is illustrative - you wouldn't write this yourself.
/// The runtime handles this with proper waking instead of busy-looping.
pub fn conceptual_poll_loop_example() {
    println!("Conceptual poll loop (DO NOT use in real code):");
    println!();
    println!("let mut page_title_fut = page_title(url);");
    println!("loop {{");
    println!("    match page_title_fut.poll() {{");
    println!("        Ready(value) => {{");
    println!("            // Use value");
    println!("            break;");
    println!("        }}");
    println!("        Pending => {{");
    println!("            // Runtime yields and tries other futures");
    println!("            // Will be woken when this future might be ready");
    println!("            continue;");
    println!("        }}");
    println!("    }}");
    println!("}}");
}

// ============================================================================
// THE PIN AND UNPIN TRAITS
// ============================================================================

/// Explanation of Pin and Unpin
///
/// # Why Pin exists
///
/// Async blocks can create self-referential data structures:
/// - Variables in async blocks may reference each other
/// - If the future moves in memory, internal references become invalid
///
/// # How Pin helps
///
/// - `Pin<P>` wraps a pointer-like type (e.g., `&mut T`, `Box<T>`)
/// - Guarantees the pointed-to value won't move in memory
/// - Required for Future::poll: `self: Pin<&mut Self>`
///
/// # The Unpin trait
///
/// - Marker trait (no methods)
/// - Types that implement Unpin are safe to move even when pinned
/// - Most types implement Unpin automatically
/// - `!Unpin` types (like async blocks) must stay pinned
///
/// # Key relationships
///
/// - `Pin<Box<T>>` pins the `T`, not the `Box`
/// - The `Box` pointer can move; the `T` data cannot
/// - `Pin<&mut String>` allows replacement because String is Unpin
pub fn pin_and_unpin_explanation() {
    println!("=== Pin and Unpin ===");
    println!();
    println!("Self-referential future example:");
    println!("┌─────────────────────┐");
    println!("│ fut                 │");
    println!("├─────────────────────┤");
    println!("│ data: 0             │");
    println!("│ data: 1             │");
    println!("│ ref ─────┐          │ <- points to data above");
    println!("│          ↓          │");
    println!("│ data: 1 (target)    │");
    println!("└─────────────────────┘");
    println!();
    println!("If we move the future, the internal reference becomes invalid!");
    println!();
    println!("Pin prevents this by guaranteeing the future won't move.");
}

/// Demonstrates the relationship between Pin, Box, and the pinned value
pub fn pin_box_relationship() {
    println!("=== Pin<Box<T>> Relationship ===");
    println!();
    println!("Pin<Box<T>>:");
    println!("┌─────┐     ┌────┐     ┌────────┐");
    println!("│ Pin │ --> │ b1 │ --> │ pinned │");
    println!("└─────┘     └────┘     │  fut   │");
    println!("                       │ (data) │");
    println!("                       └────────┘");
    println!();
    println!("The Box pointer (b1) can move to (b2):");
    println!("┌─────┐     ┌────┐     ┌────────┐");
    println!("│ Pin │ --> │ b2 │ --> │ pinned │");
    println!("└─────┘     └────┘     │  fut   │");
    println!("  (b1 is gone)         │ (same) │");
    println!("                       └────────┘");
    println!();
    println!("Key: The pinned data stays in the same memory location!");
}

// ============================================================================
// THE STREAM TRAIT
// ============================================================================

// The Stream trait - async version of Iterator
//
// Not in std yet, but common definition from futures crate:
// trait Stream {
//     type Item;
//
//     fn poll_next(
//         self: Pin<&mut Self>,
//         cx: &mut Context<'_>
//     ) -> Poll<Option<Self::Item>>;
// }
//
// Combines:
// - Iterator's `Option<Self::Item>` (sequence of items)
// - Future's `Poll` (readiness over time)
//
// Return type breakdown:
// - `Poll::Pending` - not ready, check later
// - `Poll::Ready(Some(item))` - here's the next item
// - `Poll::Ready(None)` - stream is exhausted
//
// StreamExt trait provides convenient methods:
// trait StreamExt: Stream {
//     async fn next(&mut self) -> Option<Self::Item>
//     where Self: Unpin;
// }
// - Automatically implemented for all Stream types
// - Provides `next()` method for use with await
// - Handles the poll_next complexity for you

/// Displays an explanation of the Stream trait.
pub fn stream_trait_explanation() {
    println!("=== Stream Trait ===");
    println!();
    println!("Stream combines Iterator + Future:");
    println!();
    println!("Iterator: next() -> Option<Item>");
    println!("Future:   poll() -> Poll<Output>");
    println!("Stream:   poll_next() -> Poll<Option<Item>>");
    println!();
    println!("Return values:");
    println!("  Poll::Pending           -> Not ready, try later");
    println!("  Poll::Ready(Some(item)) -> Here's the next item");
    println!("  Poll::Ready(None)       -> Stream exhausted");
    println!();
    println!("StreamExt provides the async fn next() for convenience:");
    println!("  while let Some(item) = stream.next().await {{ ... }}");
}

// ============================================================================
// PRACTICAL EXAMPLES
// ============================================================================

/// A custom Future implementation
///
/// This demonstrates implementing Future manually, which is rare
/// but helps understand how async works under the hood.
pub struct CountdownFuture {
    count: u32,
}

impl CountdownFuture {
    pub fn new(start: u32) -> Self {
        Self { count: start }
    }
}

impl std::future::Future for CountdownFuture {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count == 0 {
            println!("  CountdownFuture: Ready! Liftoff!");
            Poll::Ready("Launched!")
        } else {
            println!("  CountdownFuture: {} remaining...", self.count);
            self.count -= 1;
            // In real code, you'd register a waker and return Pending
            // Here we just immediately schedule another poll
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// Demonstrates a simple custom Stream implementation
///
/// Similar to implementing Iterator, but async-aware
pub struct CountStream {
    current: u32,
    max: u32,
}

impl CountStream {
    pub fn new(max: u32) -> Self {
        Self { current: 0, max }
    }
}

// Note: In real code, you'd use futures::Stream trait
// This is a simplified demonstration
impl CountStream {
    /// Simulates poll_next behavior
    pub fn poll_next_simulated(&mut self) -> Poll<Option<u32>> {
        if self.current < self.max {
            let value = self.current;
            self.current += 1;
            Poll::Ready(Some(value))
        } else {
            Poll::Ready(None)
        }
    }
}

// ============================================================================
// DEMONSTRATION
// ============================================================================

pub fn demonstrate_traits() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  Chapter 17.5: A Closer Look at the Traits for Async         ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    println!("1. THE FUTURE TRAIT");
    println!("{}", "─".repeat(60));
    println!("pub trait Future {{");
    println!("    type Output;");
    println!("    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;");
    println!("}}");
    println!();

    println!("2. THE POLL ENUM");
    println!("{}", "─".repeat(60));
    println!("enum Poll<T> {{");
    println!("    Ready(T),   // Future is complete");
    println!("    Pending,    // Not ready, poll again later");
    println!("}}");
    println!();

    conceptual_poll_loop_example();
    println!();

    println!("3. PIN AND UNPIN");
    println!("{}", "─".repeat(60));
    pin_and_unpin_explanation();
    println!();
    pin_box_relationship();
    println!();

    println!("4. THE STREAM TRAIT");
    println!("{}", "─".repeat(60));
    stream_trait_explanation();
    println!();

    println!("5. CUSTOM IMPLEMENTATIONS");
    println!("{}", "─".repeat(60));

    // Demonstrate CountdownFuture
    println!("CountdownFuture (polls multiple times before Ready):");
    let mut countdown = CountdownFuture::new(3);
    let waker = futures::task::noop_waker();
    let mut cx = std::task::Context::from_waker(&waker);
    loop {
        match std::pin::Pin::new(&mut countdown).poll(&mut cx) {
            Poll::Pending => println!("    Pending"),
            Poll::Ready(result) => {
                println!("    Ready({})", result);
                break;
            }
        }
    }
    println!();

    // Demonstrate CountStream
    println!("CountStream (simulated async iteration):");
    let mut stream = CountStream::new(3);
    loop {
        match stream.poll_next_simulated() {
            Poll::Ready(Some(value)) => println!("    Ready(Some({}))", value),
            Poll::Ready(None) => {
                println!("    Ready(None) - stream exhausted");
                break;
            }
            Poll::Pending => println!("    Pending"),
        }
    }
    println!();

    println!("{}", "═".repeat(60));
    println!("Key Takeaways:");
    println!("{}", "─".repeat(60));
    println!("• Future::poll returns Poll::Ready or Poll::Pending");
    println!("• await compiles to poll loops managed by the runtime");
    println!("• Pin prevents self-referential futures from moving");
    println!("• Most types are Unpin (safe to move even when pinned)");
    println!("• Stream = async Iterator (poll_next -> Poll<Option<Item>>)");
    println!("• StreamExt provides async fn next() for convenience");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_countdown_future_starts_at_count() {
        let future = CountdownFuture::new(5);
        assert_eq!(future.count, 5);
    }

    #[test]
    fn test_count_stream_creation() {
        let stream = CountStream::new(3);
        assert_eq!(stream.current, 0);
        assert_eq!(stream.max, 3);
    }

    #[test]
    fn test_count_stream_iteration() {
        let mut stream = CountStream::new(3);

        assert!(matches!(stream.poll_next_simulated(), Poll::Ready(Some(0))));
        assert!(matches!(stream.poll_next_simulated(), Poll::Ready(Some(1))));
        assert!(matches!(stream.poll_next_simulated(), Poll::Ready(Some(2))));
        assert!(matches!(stream.poll_next_simulated(), Poll::Ready(None)));
    }
}
