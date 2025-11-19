// Example: Comparing tokio and async-std runtimes

use std::time::Duration;

/// Example using tokio runtime
async fn tokio_example() {
    println!("=== Tokio Runtime ===");
    
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Tokio sleep completed\n");
}

/// Example using async-std runtime
async fn async_std_example() {
    println!("=== Async-std Runtime ===");
    
    async_std::task::sleep(Duration::from_secs(1)).await;
    println!("Async-std sleep completed\n");
}

fn main() {
    println!("Comparing Async Runtimes\n");
    
    // Run with tokio
    println!("Running tokio example:");
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(tokio_example());
    
    // Run with async-std
    println!("Running async-std example:");
    async_std::task::block_on(async_std_example());
    
    println!("Key Differences:");
    println!("- tokio: Most popular, feature-rich, used in production");
    println!("- async-std: std-like API, simpler learning curve");
    println!("- Both are excellent choices!");
    println!("\nThis workspace uses tokio for consistency.");
}
