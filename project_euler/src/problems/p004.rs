//! # Problem 4: Largest Palindrome Product
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.
//!
//! ## Approach
//!
//! 1. Iterate through products of 3-digit numbers (100-999)
//! 2. Check if product is palindrome
//! 3. Track maximum palindrome found
//!
//! ## Optimizations
//!
//! - Start from 999 and work down for early termination
//! - Skip symmetric cases (a < b)
//! - Break inner loop when product can't exceed current max

/// Check if a number is a palindrome
fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

/// Find the largest palindrome made from the product of two n-digit numbers
pub fn largest_palindrome_product(min_factor: u64, max_factor: u64) -> (u64, u64, u64) {
    let mut max_palindrome = 0u64;
    let mut factor_a = 0u64;
    let mut factor_b = 0u64;

    // Start from largest factors and work down
    for a in (min_factor..=max_factor).rev() {
        // Optimization: break if a * max_factor can't beat current max
        if a * max_factor <= max_palindrome {
            break;
        }

        for b in (min_factor..=a).rev() {
            let product = a * b;

            // Early termination: if b is too small, remaining products won't help
            if product <= max_palindrome {
                break;
            }

            if is_palindrome(product) && product > max_palindrome {
                max_palindrome = product;
                factor_a = a;
                factor_b = b;
            }
        }
    }

    (max_palindrome, factor_a, factor_b)
}

/// Solve Problem 4 for 3-digit numbers
pub fn solve() -> u64 {
    let (palindrome, _, _) = largest_palindrome_product(100, 999);
    palindrome
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(9009));
        assert!(is_palindrome(1));
        assert!(is_palindrome(121));
        assert!(is_palindrome(12321));
        assert!(!is_palindrome(123));
        assert!(!is_palindrome(1234));
    }

    #[test]
    fn test_two_digit_example() {
        let (palindrome, a, b) = largest_palindrome_product(10, 99);
        assert_eq!(palindrome, 9009);
        assert_eq!(a * b, 9009);
        // Verify factors are 2-digit numbers
        assert!((10..=99).contains(&a));
        assert!((10..=99).contains(&b));
    }

    #[test]
    fn test_three_digit_solution() {
        let (palindrome, a, b) = largest_palindrome_product(100, 999);
        // Verify it's a palindrome
        assert!(is_palindrome(palindrome));
        // Verify factors multiply correctly
        assert_eq!(a * b, palindrome);
        // Verify factors are 3-digit numbers
        assert!((100..=999).contains(&a));
        assert!((100..=999).contains(&b));
        // Solution should be 906609 = 913 × 993
        assert_eq!(palindrome, 906609);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 906609);
    }
}
