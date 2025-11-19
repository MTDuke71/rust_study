// Example: Understanding Pin and Unpin

use std::pin::Pin;
use std::marker::PhantomPinned;

/// A normal struct - can be moved freely
#[derive(Debug)]
#[allow(dead_code)]
struct Movable {
    data: String,
}

/// A self-referential struct (conceptually)
/// In real async code, the compiler generates these
#[derive(Debug)]
#[allow(dead_code)]
struct SelfReferential {
    data: String,
    // In reality, this would be a pointer into `data`
    // pointer: *const String,  // Points to self.data
    _pinned: PhantomPinned,  // Opts out of Unpin
}

#[tokio::main]
async fn main() {
    println!("=== Pin and Unpin ===\n");

    // Example 1: Movable types (most types)
    println!("Example 1: Normal types can move freely");
    
    let movable1 = Movable {
        data: String::from("Hello"),
    };
    
    let movable2 = movable1;  // Moved!
    println!("Moved value: {:?}\n", movable2);

    // Example 2: Pinned types
    println!("Example 2: Pinning prevents movement");
    
    let self_ref = SelfReferential {
        data: String::from("Pinned data"),
        _pinned: PhantomPinned,
    };
    
    let pinned = Box::pin(self_ref);
    
    // Can't move out of a Pin
    // let moved = *pinned;  // ERROR: can't move out of Pin
    
    // Can only get a pinned reference
    let _pinned_ref: Pin<&SelfReferential> = pinned.as_ref();
    println!("Pinned value is safe from movement\n");

    // Example 3: Why async needs Pin
    println!("Example 3: Async state machines are self-referential");
    
    async fn example() {
        let data = String::from("local");
        let reference = &data;  // Reference to local data
        
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        
        // After .await, `data` and `reference` must stay in same location!
        println!("{}", reference);
    }
    
    example().await;
    println!();

    // Example 4: Pin in practice
    println!("Example 4: Manual Future pinning");
    
    let future = async {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        42
    };
    
    // Futures must be pinned to be polled
    tokio::pin!(future);  // Macro creates Pin<&mut F>
    
    // Now we can .await it
    let result = future.await;
    println!("Result: {}\n", result);

    // Example 5: Unpin trait
    println!("Example 5: Most types implement Unpin");
    
    fn is_unpin<T: Unpin>(_: T) {
        println!("Type implements Unpin - safe to move even when pinned");
    }
    
    is_unpin(42);
    is_unpin(String::from("test"));
    is_unpin(vec![1, 2, 3]);
    
    println!("\nTypes that DON'T implement Unpin:");
    println!("- Async futures (self-referential)");
    println!("- Types with PhantomPinned");
    println!("- Types explicitly opting out with !Unpin");
    
    println!("\nKey Takeaways:");
    println!("- Pin prevents moving self-referential types");
    println!("- Most types are Unpin (safe to move)");
    println!("- Async futures need Pin because of .await");
    println!("- Use tokio::pin! or Box::pin() for pinning");
}
