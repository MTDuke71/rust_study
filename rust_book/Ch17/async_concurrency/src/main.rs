// Section 17.2: Applying Concurrency with Async
// Running multiple async operations concurrently

use tokio::time::{sleep, Duration, Instant};

/// Simulate fetching user data from a database
async fn fetch_user(id: u32) -> String {
    println!("Fetching user {}...", id);
    sleep(Duration::from_secs(1)).await;
    format!("User {}", id)
}

/// Simulate fetching posts for a user
async fn fetch_posts(user_id: u32) -> Vec<String> {
    println!("Fetching posts for user {}...", user_id);
    sleep(Duration::from_millis(800)).await;
    vec![format!("Post 1 by user {}", user_id), format!("Post 2 by user {}", user_id)]
}

/// Simulate fetching comments
async fn fetch_comments(user_id: u32) -> Vec<String> {
    println!("Fetching comments for user {}...", user_id);
    sleep(Duration::from_millis(600)).await;
    vec![format!("Comment by user {}", user_id)]
}

#[tokio::main]
async fn main() {
    println!("=== Concurrent Async Operations ===\n");

    // Example 1: Sequential (slow)
    println!("Example 1: Sequential execution");
    let start = Instant::now();
    
    let user = fetch_user(1).await;
    let posts = fetch_posts(1).await;
    let comments = fetch_comments(1).await;
    
    println!("Got: {}, {} posts, {} comments", user, posts.len(), comments.len());
    println!("Sequential time: {:?}\n", start.elapsed());
    // Total: ~2.4 seconds (1.0 + 0.8 + 0.6)

    // Example 2: Concurrent with tokio::join!
    println!("Example 2: Concurrent execution with join!");
    let start = Instant::now();
    
    let (user, posts, comments) = tokio::join!(
        fetch_user(2),
        fetch_posts(2),
        fetch_comments(2)
    );
    
    println!("Got: {}, {} posts, {} comments", user, posts.len(), comments.len());
    println!("Concurrent time: {:?}\n", start.elapsed());
    // Total: ~1.0 seconds (longest task) - 2.4x faster!

    // Example 3: Spawning background tasks
    println!("Example 3: Spawning background tasks");
    
    let task1 = tokio::spawn(async {
        sleep(Duration::from_millis(500)).await;
        "Task 1 done"
    });
    
    let task2 = tokio::spawn(async {
        sleep(Duration::from_millis(300)).await;
        "Task 2 done"
    });
    
    // Do other work while tasks run
    println!("Tasks spawned, doing other work...");
    sleep(Duration::from_millis(100)).await;
    
    // Wait for results
    let result1 = task1.await.unwrap();
    let result2 = task2.await.unwrap();
    println!("{}, {}\n", result1, result2);

    // Example 4: try_join! for fallible operations
    println!("Example 4: try_join! for Results");
    
    async fn might_fail(should_fail: bool) -> Result<String, &'static str> {
        sleep(Duration::from_millis(100)).await;
        if should_fail {
            Err("Operation failed")
        } else {
            Ok(String::from("Success"))
        }
    }
    
    match tokio::try_join!(
        might_fail(false),
        might_fail(false),
        might_fail(false)
    ) {
        Ok((r1, r2, r3)) => println!("All succeeded: {}, {}, {}", r1, r2, r3),
        Err(e) => println!("At least one failed: {}", e),
    }
    
    println!("\nKey Insight: join! waits for ALL futures to complete");
    println!("For racing futures, see select_futures.rs example");
}
