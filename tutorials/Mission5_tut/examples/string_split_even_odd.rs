//! # String Character Splitting by Index
//!
//! **Demonstrates**: Various ways to split strings into even/odd indexed characters

fn main() {
    println!("🦀 String Character Splitting by Index");
    println!("======================================\n");

    let test_string = "Hello, World!";
    println!("Original string: '{}'", test_string);
    println!("Indices:          0123456789...\n");

    method1_enumerate_filter(test_string);
    method2_step_by_iterator(test_string);
    method3_chunks_and_collect(test_string);
    method4_functional_approach(test_string);
    method5_manual_loop(test_string);

    // Test with different strings
    println!("\n🧪 Testing with Different Strings:");
    test_various_strings();
}

/// Method 1: Using enumerate() and filter()
fn method1_enumerate_filter(s: &str) {
    println!("📋 Method 1: enumerate() + filter()");
    println!("===================================");

    let chars: Vec<char> = s.chars().collect();

    let even_chars: String = chars
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0) // Even indices: 0, 2, 4, 6...
        .map(|(_, &c)| c)
        .collect();

    let odd_chars: String = chars
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1) // Odd indices: 1, 3, 5, 7...
        .map(|(_, &c)| c)
        .collect();

    println!("  Even indices (0,2,4...): '{}'", even_chars);
    println!("  Odd indices (1,3,5...):  '{}'", odd_chars);
    println!();
}

/// Method 2: Using step_by() iterator
fn method2_step_by_iterator(s: &str) {
    println!("🔄 Method 2: step_by() iterator");
    println!("==============================");

    let chars: Vec<char> = s.chars().collect();

    let even_chars: String = chars
        .iter()
        .step_by(2) // Take every 2nd element starting from 0
        .collect();

    let odd_chars: String = chars
        .iter()
        .skip(1) // Skip first element
        .step_by(2) // Then take every 2nd element
        .collect();

    println!("  Even indices: '{}'", even_chars);
    println!("  Odd indices:  '{}'", odd_chars);
    println!();
}

/// Method 3: Using chunks and manual collection
fn method3_chunks_and_collect(s: &str) {
    println!("📦 Method 3: Manual collection with chunks concept");
    println!("================================================");

    let chars: Vec<char> = s.chars().collect();
    let mut even_chars = String::new();
    let mut odd_chars = String::new();

    for (i, &ch) in chars.iter().enumerate() {
        if i % 2 == 0 {
            even_chars.push(ch);
        } else {
            odd_chars.push(ch);
        }
    }

    println!("  Even indices: '{}'", even_chars);
    println!("  Odd indices:  '{}'", odd_chars);
    println!();
}

/// Method 4: Functional approach with partition
fn method4_functional_approach(s: &str) {
    println!("🔧 Method 4: Functional partition approach");
    println!("=========================================");

    let indexed_chars: Vec<(usize, char)> = s.chars().enumerate().collect();

    let (even_pairs, odd_pairs): (Vec<_>, Vec<_>) =
        indexed_chars.into_iter().partition(|(i, _)| i % 2 == 0);

    let even_chars: String = even_pairs.into_iter().map(|(_, c)| c).collect();
    let odd_chars: String = odd_pairs.into_iter().map(|(_, c)| c).collect();

    println!("  Even indices: '{}'", even_chars);
    println!("  Odd indices:  '{}'", odd_chars);
    println!();
}

/// Method 5: Manual loop (most straightforward)
fn method5_manual_loop(s: &str) {
    println!("🔁 Method 5: Simple manual loop");
    println!("==============================");

    let mut even_chars = String::new();
    let mut odd_chars = String::new();

    for (i, c) in s.chars().enumerate() {
        if i % 2 == 0 {
            even_chars.push(c);
        } else {
            odd_chars.push(c);
        }
    }

    println!("  Even indices: '{}'", even_chars);
    println!("  Odd indices:  '{}'", odd_chars);
    println!();
}

/// Test with various strings to show the pattern
fn test_various_strings() {
    let test_cases = [
        "abcdef",
        "12345",
        "A",
        "",
        "Hello, World!",
        "Rust 🦀 is awesome!",
    ];

    for test_str in &test_cases {
        if test_str.is_empty() {
            println!("Empty string: even='', odd=''");
            continue;
        }

        let (even, odd) = split_even_odd_simple(test_str);
        println!("'{}' → even='{}', odd='{}'", test_str, even, odd);
    }
}

/// Utility function for clean even/odd splitting
fn split_even_odd_simple(s: &str) -> (String, String) {
    let mut even = String::new();
    let mut odd = String::new();

    for (i, c) in s.chars().enumerate() {
        if i % 2 == 0 {
            even.push(c);
        } else {
            odd.push(c);
        }
    }

    (even, odd)
}

/// Advanced: Split into even/odd with original indices preserved
#[allow(dead_code)]
fn split_with_indices(s: &str) -> (Vec<(usize, char)>, Vec<(usize, char)>) {
    s.chars().enumerate().partition(|(i, _)| i % 2 == 0)
}

/// Performance comparison function
#[allow(dead_code)]
fn performance_comparison() {
    let test_string = "A".repeat(1000);

    // These would be used for actual benchmarking
    println!("Performance notes:");
    println!("  - Method 1 (enumerate + filter): Clean but creates intermediate vectors");
    println!("  - Method 2 (step_by): Efficient for iterator chains");
    println!("  - Method 5 (manual loop): Most efficient, single pass");
    println!("  - For large strings: prefer manual loop or step_by");
}
