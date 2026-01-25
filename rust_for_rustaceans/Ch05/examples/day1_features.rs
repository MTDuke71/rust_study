 //! Day 1: Features - Conditional Compilation with Feature Flags
//! 
//! This example demonstrates:
//! - Defining and using features
//! - Optional dependencies
//! - Feature-gated code
//! - The additive principle

/// Core functionality available without any features
pub fn core_operation(x: i32) -> i32 {
    println!("Core operation: {} * 2 = {}", x, x * 2);
    x * 2
}

/// Standard library feature - enabled by default
#[cfg(feature = "std")]
pub fn std_operation(items: Vec<i32>) -> Vec<i32> {
    println!("Standard operation using Vec");
    items.iter().map(|x| x * 2).collect()
}

/// Advanced feature - must be explicitly enabled
#[cfg(feature = "advanced")]
pub fn advanced_operation(x: i32) -> i32 {
    println!("Advanced operation: {} ^ 2 = {}", x, x * x);
    x * x
}

/// Networking feature example
#[cfg(feature = "networking")]
pub fn network_operation() {
    println!("Networking feature enabled!");
    println!("Simulating network request...");
}

/// Compression feature example
#[cfg(feature = "compression")]
pub fn compression_operation() {
    println!("Compression feature enabled!");
    println!("Simulating data compression...");
}

/// Experimental feature - unstable API
#[cfg(feature = "experimental")]
pub fn experimental_operation() {
    println!("⚠️  Experimental feature - API may change!");
}

/// Example using cfg! macro for runtime checks
pub fn check_features() {
    println!("\n=== Feature Detection ===");
    
    if cfg!(feature = "std") {
        println!("✓ std feature enabled");
    } else {
        println!("✗ std feature disabled");
    }
    
    if cfg!(feature = "advanced") {
        println!("✓ advanced feature enabled");
    } else {
        println!("✗ advanced feature disabled");
    }
    
    if cfg!(feature = "networking") {
        println!("✓ networking feature enabled");
    } else {
        println!("✗ networking feature disabled");
    }
    
    if cfg!(feature = "compression") {
        println!("✓ compression feature enabled");
    } else {
        println!("✗ compression feature disabled");
    }
    
    if cfg!(feature = "experimental") {
        println!("✓ experimental feature enabled");
    } else {
        println!("✗ experimental feature disabled");
    }
    
    if cfg!(feature = "serde") {
        println!("✓ serde feature enabled");
    } else {
        println!("✗ serde feature disabled");
    }
}

/// Optional dependency example
#[cfg(feature = "serde")]
pub fn serde_operation() {
    println!("Serde feature enabled - serialization available");
}

fn main() {
    println!("🎯 Day 1: Features Example\n");
    
    // Core operations always available
    core_operation(5);
    
    // Standard library features
    #[cfg(feature = "std")]
    {
        let numbers = vec![1, 2, 3, 4, 5];
        let doubled = std_operation(numbers);
        println!("Doubled: {:?}", doubled);
    }
    
    // Advanced features
    #[cfg(feature = "advanced")]
    {
        advanced_operation(7);
        
        #[cfg(feature = "networking")]
        network_operation();
        
        #[cfg(feature = "compression")]
        compression_operation();
    }
    
    // Experimental features
    #[cfg(feature = "experimental")]
    experimental_operation();
    
    // Optional dependency
    #[cfg(feature = "serde")]
    serde_operation();
    
    // Feature detection
    check_features();
    
    println!("\n📝 Try running with different features:");
    println!("  cargo run --example day1_features");
    println!("  cargo run --example day1_features --no-default-features");
    println!("  cargo run --example day1_features --features advanced");
    println!("  cargo run --example day1_features --features advanced,networking");
    println!("  cargo run --example day1_features --all-features");
}
