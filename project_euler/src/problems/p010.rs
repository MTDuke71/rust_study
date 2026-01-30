//! # Problem 10: Summation of Primes
//!
//! Find the sum of all primes below 2,000,000.
//!
//! ## Mathematical Foundation
//!
//! Uses **Sieve of Eratosthenes** with optimizations:
//! - Only check odd numbers (2 is handled separately)
//! - Sieve up to √n for marking composites
//! - Sum during iteration (no need to store all primes)
//!
//! See `zettelkasten/math-foundations/sieve-of-eratosthenes.md` for theory.
//!
//! ## Approach
//!
//! 1. Create boolean array for odd numbers only
//! 2. Mark composites using sieve
//! 3. Sum all primes during final pass
//!
//! ## Complexity
//!
//! - Time: O(n log log n) - Sieve of Eratosthenes
//! - Space: O(n) - boolean array

/// Sum all primes below limit using optimized sieve
///
/// Uses a compact sieve that only tracks odd numbers, saving ~50% memory.
/// The even number 2 is handled as a special case.
///
/// # Examples
/// ```
/// use project_euler::problems::p010::sum_primes_below;
/// assert_eq!(sum_primes_below(10), 17); // 2 + 3 + 5 + 7 = 17
/// ```
pub fn sum_primes_below(limit: usize) -> u64 {
    if limit <= 2 {
        return 0;
    }
    
    // Start with 2 (the only even prime)
    let mut sum = 2u64;
    
    if limit <= 3 {
        return sum; // Only prime below 3 is 2
    }
    
    // Create sieve for odd numbers only: represents 3, 5, 7, 9, 11, ...
    // Index i represents number (2*i + 3)
    // We want numbers from 3 up to but not including limit
    let sieve_size = (limit - 1 - 3) / 2 + 1;
    let mut is_prime = vec![true; sieve_size];
    
    // Sieve of Eratosthenes for odd numbers
    let sqrt_limit = (limit as f64).sqrt() as usize;
    
    for i in 0..sieve_size {
        if is_prime[i] {
            let prime = 2 * i + 3; // Convert index to actual number
            
            if prime > sqrt_limit {
                break; // No need to mark composites beyond √limit
            }
            
            // Mark multiples of this prime as composite
            // Start from prime² (earlier multiples already marked)
            let start = (prime * prime - 3) / 2;
            
            for j in (start..sieve_size).step_by(prime) {
                is_prime[j] = false;
            }
        }
    }
    
    // Sum all primes
    for i in 0..sieve_size {
        if is_prime[i] {
            sum += (2 * i + 3) as u64;
        }
    }
    
    sum
}

/// Solve Problem 10: Sum of all primes below 2,000,000
pub fn solve() -> u64 {
    sum_primes_below(2_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_primes_small() {
        assert_eq!(sum_primes_below(10), 17); // 2 + 3 + 5 + 7
    }

    #[test]
    fn test_sum_primes_example() {
        // Primes below 10: 2, 3, 5, 7
        // Sum: 2 + 3 + 5 + 7 = 17
        assert_eq!(sum_primes_below(10), 17);
    }

    #[test]
    fn test_sum_primes_below_20() {
        // Primes below 20: 2, 3, 5, 7, 11, 13, 17, 19
        // Sum: 2 + 3 + 5 + 7 + 11 + 13 + 17 + 19 = 77
        assert_eq!(sum_primes_below(20), 77);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(sum_primes_below(2), 0); // No primes < 2
        assert_eq!(sum_primes_below(3), 2); // Only 2
        assert_eq!(sum_primes_below(5), 5); // 2 + 3 = 5
    }

    #[test]
    fn test_solution() {
        let result = solve();
        println!("Problem 10 solution: {}", result);
        assert!(result > 0);
    }
}
