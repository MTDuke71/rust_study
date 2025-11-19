// Example: tokio::join! for waiting on multiple futures

use tokio::time::{sleep, Duration, Instant};

async fn task_a() -> &'static str {
    println!("Task A starting");
    sleep(Duration::from_secs(2)).await;
    println!("Task A done");
    "A"
}

async fn task_b() -> u32 {
    println!("Task B starting");
    sleep(Duration::from_secs(1)).await;
    println!("Task B done");
    42
}

async fn task_c() -> Vec<i32> {
    println!("Task C starting");
    sleep(Duration::from_millis(500)).await;
    println!("Task C done");
    vec![1, 2, 3]
}

#[tokio::main]
async fn main() {
    println!("=== tokio::join! Examples ===\n");

    // Example 1: Basic join
    println!("Example 1: Joining different return types");
    let start = Instant::now();
    
    let (a, b, c) = tokio::join!(task_a(), task_b(), task_c());
    
    println!("\nResults: {}, {}, {:?}", a, b, c);
    println!("Total time: {:?} (longest task)", start.elapsed());
    println!();

    // Example 2: Join with inline futures
    println!("Example 2: Inline futures");
    
    let (r1, r2, r3) = tokio::join!(
        async {
            sleep(Duration::from_millis(300)).await;
            1
        },
        async {
            sleep(Duration::from_millis(200)).await;
            2
        },
        async {
            sleep(Duration::from_millis(100)).await;
            3
        }
    );
    
    println!("Results: {}, {}, {}\n", r1, r2, r3);

    // Example 3: Many futures
    println!("Example 3: Join many futures");
    
    let start = Instant::now();
    let results = tokio::join!(
        fetch_data(1),
        fetch_data(2),
        fetch_data(3),
        fetch_data(4),
        fetch_data(5)
    );
    
    println!("Got {} results in {:?}", 5, start.elapsed());
    println!("Results: {:?}\n", results);

    println!("Note: join! executes all futures concurrently");
    println!("All futures must complete before join! returns");
}

async fn fetch_data(id: u32) -> String {
    sleep(Duration::from_millis(100 * id as u64)).await;
    format!("Data {}", id)
}
