//! # Chapter 11.2: Controlling How Tests Are Run
//!
//! Demonstrates various ways to control test execution, including
//! parallel vs sequential execution, filtering tests, and handling test output.
//!
//! Run with: `cargo test` (see comments for various options)

use std::thread;
use std::time::Duration;

fn main() {
    println!("🎯 Chapter 11.2: Controlling How Tests Are Run");
    println!("===============================================");
    
    example1_test_output();
    example2_parallel_execution();
    example3_filtering_tests();
    example4_ignoring_tests();
    
    println!("✅ All examples completed!");
    println!("📖 Try these commands:");
    println!("  cargo test                    # Run all tests");
    println!("  cargo test -- --nocapture     # Show output");
    println!("  cargo test -- --test-threads=1 # Sequential");
    println!("  cargo test fast_test          # Filter by name");
    println!("  cargo test -- --ignored       # Run ignored tests");
}

fn example1_test_output() {
    println!("📺 Example 1: Test Output Control");
    println!("=================================");
    
    println!("By default, cargo test captures output from successful tests.");
    println!("Use --nocapture to see println! statements from passing tests.");
    
    println!();
}

fn example2_parallel_execution() {
    println!("⚡ Example 2: Parallel vs Sequential Execution");
    println!("==============================================");
    
    println!("Tests run in parallel by default for speed.");
    println!("Use --test-threads=1 for sequential execution.");
    println!("This is useful when tests share resources.");
    
    println!();
}

fn example3_filtering_tests() {
    println!("🔍 Example 3: Filtering Tests");
    println!("=============================");
    
    println!("You can run specific tests by name:");
    println!("  cargo test test_name");
    println!("  cargo test partial_name  # Runs all matching tests");
    
    println!();
}

fn example4_ignoring_tests() {
    println!("⏭️ Example 4: Ignoring Expensive Tests");
    println!("======================================");
    
    println!("Use #[ignore] for slow tests that run only when requested.");
    println!("Run with: cargo test -- --ignored");
    
    // Demonstrate the functions that are tested
    println!("\nDemonstrating functions used in tests:");
    println!("add_numbers(5, 3) = {}", add_numbers(5, 3));
    println!("slow_operation() = {}", slow_operation());
    println!("expensive_computation(4) = {}", expensive_computation(4));
    println!("shared_resource_operation(42) = {}", shared_resource_operation(42));
    
    println!();
}

// Functions for testing
fn add_numbers(a: i32, b: i32) -> i32 {
    println!("Adding {} and {} = {}", a, b, a + b);
    a + b
}

fn slow_operation() -> String {
    println!("Starting slow operation...");
    thread::sleep(Duration::from_millis(100)); // Simulate slow work
    println!("Slow operation completed!");
    "completed".to_string()
}

fn expensive_computation(n: u64) -> u64 {
    println!("Computing expensive operation for n = {}", n);
    // Simulate expensive computation
    thread::sleep(Duration::from_millis(50));
    n * n * n
}

fn shared_resource_operation(id: u32) -> String {
    println!("Accessing shared resource with ID: {}", id);
    // Simulate file or database access
    thread::sleep(Duration::from_millis(10));
    format!("Resource {} accessed", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fast tests that show output
    #[test]
    fn fast_test_with_output() {
        println!("This is output from a fast test");
        let result = add_numbers(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn another_fast_test() {
        println!("This is another fast test");
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn fast_string_test() {
        println!("Testing string operations");
        let s = String::from("hello");
        assert_eq!(s.len(), 5);
    }

    // Tests for parallel execution demonstration
    #[test]
    fn parallel_test_1() {
        println!("Parallel test 1 starting");
        thread::sleep(Duration::from_millis(20));
        println!("Parallel test 1 completed");
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn parallel_test_2() {
        println!("Parallel test 2 starting");
        thread::sleep(Duration::from_millis(20));
        println!("Parallel test 2 completed");
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn parallel_test_3() {
        println!("Parallel test 3 starting");
        thread::sleep(Duration::from_millis(20));
        println!("Parallel test 3 completed");
        assert_eq!(3 + 3, 6);
    }

    // Tests that might interfere if run in parallel
    #[test]
    fn shared_resource_test_a() {
        let result = shared_resource_operation(1);
        assert!(result.contains("Resource 1"));
    }

    #[test]
    fn shared_resource_test_b() {
        let result = shared_resource_operation(2);
        assert!(result.contains("Resource 2"));
    }

    // Slow/expensive tests marked with #[ignore]
    #[test]
    #[ignore]
    fn expensive_test() {
        println!("Running expensive test - this takes time!");
        let result = expensive_computation(1000);
        assert_eq!(result, 1_000_000_000);
    }

    #[test]
    #[ignore]
    fn very_slow_test() {
        println!("This test is very slow...");
        thread::sleep(Duration::from_millis(500));
        println!("Finally finished!");
        assert!(true);
    }

    #[test]
    #[ignore]
    fn integration_test() {
        println!("Running integration test with external dependencies");
        // This might require database, network, etc.
        thread::sleep(Duration::from_millis(200));
        assert!(true);
    }

    // Tests for filtering demonstrations
    #[test]
    fn test_addition_basic() {
        assert_eq!(add_numbers(1, 2), 3);
    }

    #[test]
    fn test_addition_negative() {
        assert_eq!(add_numbers(-1, -2), -3);
    }

    #[test]
    fn test_addition_zero() {
        assert_eq!(add_numbers(0, 5), 5);
    }

    #[test]
    fn multiplication_test() {
        assert_eq!(3 * 4, 12);
    }

    #[test]
    fn division_test() {
        assert_eq!(10 / 2, 5);
    }

    // Tests that demonstrate filtering by partial name
    #[test]
    fn math_operation_add() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn math_operation_subtract() {
        assert_eq!(5 - 3, 2);
    }

    #[test]
    fn math_operation_multiply() {
        assert_eq!(3 * 3, 9);
    }

    // String tests for filtering
    #[test]
    fn string_test_concat() {
        let result = format!("{}{}", "hello", "world");
        assert_eq!(result, "helloworld");
    }

    #[test]
    fn string_test_length() {
        assert_eq!("hello".len(), 5);
    }

    #[test]
    fn string_test_contains() {
        assert!("hello world".contains("world"));
    }

    // Performance sensitive tests
    #[test]
    fn performance_test_vector() {
        println!("Testing vector performance");
        let mut vec = Vec::new();
        for i in 0..1000 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 1000);
    }

    #[test]
    fn performance_test_hashmap() {
        use std::collections::HashMap;
        println!("Testing HashMap performance");
        
        let mut map = HashMap::new();
        for i in 0..100 {
            map.insert(i, i * 2);
        }
        assert_eq!(map.len(), 100);
    }

    // Test that might fail occasionally (flaky test example)
    #[test]
    #[ignore]
    fn flaky_test() {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        println!("Running potentially flaky test");
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        // This test might be flaky based on timing
        // We ignore it to prevent CI failures
        assert!(timestamp > 0);
    }

    // Documentation tests (these run with `cargo test --doc`)
    /// Example function that demonstrates doc tests
    /// 
    /// # Examples
    /// 
    /// ```
    /// let result = test_execution::doc_test_example(5);
    /// assert_eq!(result, 10);
    /// ```
    pub fn doc_test_example(x: i32) -> i32 {
        x * 2
    }

    #[test]
    fn test_doc_test_example() {
        assert_eq!(doc_test_example(5), 10);
    }
}