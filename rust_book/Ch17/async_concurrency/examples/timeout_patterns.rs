// Example: Timeout patterns and cancellation

use tokio::time::{sleep, timeout, Duration};

async fn unreliable_operation() -> Result<String, &'static str> {
    // Simulates operation that might take too long
    sleep(Duration::from_secs(3)).await;
    Ok(String::from("Operation complete"))
}

async fn quick_operation() -> String {
    sleep(Duration::from_millis(500)).await;
    String::from("Quick result")
}

#[tokio::main]
async fn main() {
    println!("=== Timeout Patterns ===\n");

    // Example 1: Basic timeout
    println!("Example 1: Basic timeout wrapper");

    match timeout(Duration::from_secs(1), unreliable_operation()).await {
        Ok(Ok(result)) => println!("Success: {}", result),
        Ok(Err(e)) => println!("Operation error: {}", e),
        Err(_) => println!("Timeout! Operation took too long"),
    }
    println!();

    // Example 2: Timeout with fallback
    println!("Example 2: Timeout with fallback");

    let result = match timeout(Duration::from_secs(1), quick_operation()).await {
        Ok(data) => data,
        Err(_) => {
            println!("Primary operation timed out, using fallback");
            String::from("Fallback value")
        }
    };

    println!("Result: {}\n", result);

    // Example 3: Retry with timeout
    println!("Example 3: Retry with timeout");

    let mut attempts = 0;
    let max_attempts = 3;

    loop {
        attempts += 1;
        println!("Attempt {}/{}", attempts, max_attempts);

        match timeout(Duration::from_secs(1), flaky_operation(attempts)).await {
            Ok(Ok(result)) => {
                println!("Success: {}", result);
                break;
            }
            Ok(Err(e)) => {
                println!("Error: {}", e);
                if attempts >= max_attempts {
                    println!("Max attempts reached, giving up");
                    break;
                }
            }
            Err(_) => {
                println!("Timeout");
                if attempts >= max_attempts {
                    println!("Max attempts reached, giving up");
                    break;
                }
            }
        }

        sleep(Duration::from_millis(100)).await;
    }
    println!();

    // Example 4: Cancellation by dropping
    println!("Example 4: Cancellation by dropping future");

    let handle = tokio::spawn(async {
        println!("Long task starting...");
        sleep(Duration::from_secs(10)).await;
        println!("Long task done (you won't see this)");
    });

    // Let it run a bit
    sleep(Duration::from_millis(100)).await;

    // Cancel by dropping the handle
    drop(handle);
    println!("Task cancelled by dropping JoinHandle");
    println!();

    // Example 5: Graceful cancellation with select!
    println!("Example 5: Graceful cancellation");

    let (cancel_tx, mut cancel_rx) = tokio::sync::mpsc::channel::<()>(1);

    let task = tokio::spawn(async move {
        let mut count = 0;
        loop {
            tokio::select! {
                _ = sleep(Duration::from_millis(200)) => {
                    count += 1;
                    println!("Working... (iteration {})", count);
                }
                _ = cancel_rx.recv() => {
                    println!("Cancellation received, cleaning up...");
                    break;
                }
            }
        }
        println!("Task shut down gracefully");
    });

    // Let it run for a bit
    sleep(Duration::from_secs(2)).await;

    // Send cancellation signal
    cancel_tx.send(()).await.unwrap();

    // Wait for graceful shutdown
    task.await.unwrap();

    println!("\nKey Takeaways:");
    println!("- Use timeout() to prevent hanging");
    println!("- Dropping futures cancels them");
    println!("- Use channels for graceful cancellation");
}

async fn flaky_operation(attempt: u32) -> Result<String, &'static str> {
    sleep(Duration::from_millis(100)).await;

    if attempt >= 2 {
        Ok(String::from("Success on retry"))
    } else {
        Err("Temporary failure")
    }
}
