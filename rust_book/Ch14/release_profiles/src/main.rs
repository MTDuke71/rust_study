//! 14.1 - Customizing Builds with Release Profiles
//!
//! This example demonstrates how different release profiles affect
//! compilation and runtime performance.

fn main() {
    println!("Release Profiles Example");
    println!("========================");
    
    // This function will be optimized differently based on the profile
    let result = compute_expensive_operation(1_000_000);
    println!("Result: {}", result);
    
    // Check if debug assertions are enabled
    #[cfg(debug_assertions)]
    println!("Debug assertions: ENABLED (dev profile)");
    
    #[cfg(not(debug_assertions))]
    println!("Debug assertions: DISABLED (release profile)");
    
    // Demonstrate optimization differences
    demonstrate_optimization();
}

/// An expensive computation that benefits from optimization
fn compute_expensive_operation(n: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..n {
        sum += i * i;
    }
    sum
}

/// Demonstrate how different profiles affect code generation
fn demonstrate_optimization() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // This loop may be optimized away or unrolled in release mode
    let sum: i32 = numbers.iter().sum();
    println!("Sum of numbers: {}", sum);
    
    // Debug assertions only active in dev profile
    debug_assert!(sum > 0, "Sum should always be positive");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expensive_operation() {
        // In release mode, this might be optimized differently
        let result = compute_expensive_operation(100);
        assert!(result > 0);
    }
}

