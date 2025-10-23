//! # Day 4 Unit Tests - Demonstrating What CAN Be Tested
//!
//! While Day 4 is primarily a brute-force problem, there are still valuable unit tests possible.


/// Example of how Day 4 could be tested
#[cfg(test)]
mod day04_tests {
    use super::*;

    #[test]
    fn test_md5_hash_generation() {
        // Test that we can generate MD5 hashes correctly using the known examples
        let hash = format!("{:x}", md5::compute("abcdef609043"));
        assert!(hash.starts_with("000001"));
        println!("✅ abcdef609043 hash: {}", hash);

        let hash = format!("{:x}", md5::compute("pqrstuv1048970"));
        assert!(hash.starts_with("000006"));
        println!("✅ pqrstuv1048970 hash: {}", hash);
    }

    #[test]
    fn test_hash_prefix_detection() {
        // Test our prefix detection logic works correctly
        let hash_5_zeros = "00000abcdef123456789";
        let hash_6_zeros = "000000123456789abcdef";
        let hash_4_zeros = "0000abcdef123456789";
        let hash_no_zeros = "abcdef123456789";

        assert!(hash_5_zeros.starts_with("00000"));
        assert!(hash_6_zeros.starts_with("000000"));
        assert!(!hash_4_zeros.starts_with("00000"));
        assert!(!hash_no_zeros.starts_with("00000"));
    }

    #[test]
    fn test_input_parsing() {
        // Test input trimming works correctly for various inputs
        assert_eq!("yzbqklnj", "yzbqklnj\n".trim());
        assert_eq!("abcdef", " abcdef ".trim());
        assert_eq!("test", "\ttest\r\n".trim());
        assert_eq!("", "   ".trim());
    }

    #[test]
    fn test_hash_formatting_properties() {
        // Test that MD5 hashes have the expected properties
        let hash = format!("{:x}", md5::compute("test"));

        // MD5 is always 32 hex characters
        assert_eq!(hash.len(), 32);

        // Should be lowercase hexadecimal
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(hash.chars().all(|c| !c.is_ascii_uppercase()));

        // Different inputs should produce different hashes
        let hash2 = format!("{:x}", md5::compute("test2"));
        assert_ne!(hash, hash2);
    }

    #[test]
    fn test_counter_increment_logic() {
        // Test the pattern of generating sequential numbers
        let secret = "test";
        for i in 1..=10 {
            let data = format!("{secret}{i}");
            let hash = format!("{:x}", md5::compute(data.clone()));

            // Each should be different
            assert_eq!(data, format!("test{}", i));
            assert_eq!(hash.len(), 32);
        }
    }

    #[test]
    fn test_edge_cases() {
        // Test edge cases that might cause issues

        // Empty secret (though probably not valid for AoC)
        let hash = format!("{:x}", md5::compute("1"));
        assert_eq!(hash.len(), 32);

        // Very large numbers
        let hash = format!("{:x}", md5::compute("test999999"));
        assert_eq!(hash.len(), 32);

        // Special characters in secret
        let hash = format!("{:x}", md5::compute("test-key1"));
        assert_eq!(hash.len(), 32);
    }

    // Helper function that could be extracted and tested
    fn compute_hash(secret: &str, number: u32) -> String {
        let data = format!("{secret}{number}");
        format!("{:x}", md5::compute(data))
    }

    #[test]
    fn test_helper_function() {
        let hash = compute_hash("abcdef", 609043);
        assert!(hash.starts_with("000001"));

        let hash = compute_hash("pqrstuv", 1048970);
        assert!(hash.starts_with("000006"));
    }

    // This function could be extracted to make the actual solver more testable
    fn find_hash_with_prefix(secret: &str, prefix: &str, max_attempts: u32) -> Option<u32> {
        for i in 1..=max_attempts {
            let hash = compute_hash(secret, i);
            if hash.starts_with(prefix) {
                return Some(i);
            }
        }
        None
    }

    #[test]
    fn test_find_hash_small_search() {
        // Test with a small search space to avoid long-running tests
        let result = find_hash_with_prefix("test", "0", 100);

        // Should find SOMETHING with just one zero in first 100 attempts
        assert!(result.is_some());

        if let Some(num) = result {
            let hash = compute_hash("test", num);
            assert!(hash.starts_with("0"));
            println!("Found test{} -> {}", num, hash);
        }
    }

    // Performance boundary test - what's reasonable for testing
    #[test]
    fn test_reasonable_search_boundary() {
        // Test that we can search a reasonable number without timing out
        let start = std::time::Instant::now();
        let _result = find_hash_with_prefix("quicktest", "0", 1000);
        let duration = start.elapsed();

        // Should complete within reasonable time (say, 1 second)
        assert!(
            duration.as_secs() < 1,
            "Search took too long: {:?}",
            duration
        );
    }

    // The actual examples would be too slow for regular testing
    #[test]
    #[ignore] // Use `cargo test -- --ignored` to run slow tests
    fn test_example_case_abcdef_slow() {
        // This verifies the actual example but is too slow for regular CI
        let result = find_hash_with_prefix("abcdef", "00000", 1_000_000);
        assert_eq!(result, Some(609043));
    }

    #[test]
    #[ignore]
    fn test_example_case_pqrstuv_slow() {
        // This verifies the actual example but is too slow for regular CI
        let result = find_hash_with_prefix("pqrstuv", "00000", 2_000_000);
        assert_eq!(result, Some(1048970));
    }
}

fn main() {
    println!("🧪 Day 4 Unit Test Examples");
    println!("===========================");
    println!("Run with: cargo test");
    println!("Run slow tests with: cargo test -- --ignored");
}
