//! # Problem 35: Circular Primes
//!
//! The number, 197, is called a circular prime because all rotations of the
//! digits: 197, 971, and 719, are themselves prime.
//!
//! There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37,
//! 71, 73, 79, and 97.
//!
//! How many circular primes are there below one million?
//!
//! ## Mathematical Foundation
//!
//! A **circular prime** is a prime p such that every cyclic rotation of its
//! decimal digits is also prime. Rotation here means moving the leading digit
//! to the end (or equivalently, the trailing digit to the front).
//!
//! **Digit constraint**: For a circular prime p > 5, every digit must lie in
//! {1, 3, 7, 9}. Reason: if any digit is even (0, 2, 4, 6, 8), some rotation
//! ends in that digit and is divisible by 2; if any digit is 5, some rotation
//! ends in 5 and is divisible by 5. The single-digit primes 2 and 5 are the
//! only exceptions and must be enumerated explicitly.
//!
//! See [[math-foundations/sieve-of-eratosthenes]] for prime generation,
//! [[math-foundations/primality-fundamentals]] for primality testing.
//!
//! ## Approach
//!
//! 1. Sieve all primes below 1,000,000 into a boolean lookup table.
//! 2. For each prime p, generate every cyclic rotation of its decimal digits.
//! 3. p is circular iff every rotation is also prime — verified via O(1)
//!    sieve lookups.
//! 4. Count the circular primes.
//!
//! Performance shortcut: skipping primes containing any digit in {0, 2, 4, 5,
//! 6, 8} (except 2 and 5 themselves) avoids ~78,000 rotation checks. We omit
//! this filter for clarity since the sieve makes verification cheap enough.
//!
//! ## Complexity
//!
//! - Time: O(N · log N) sieve + O(P · d) where P ≈ 78,498 primes < 10⁶ and
//!   d ≤ 6 digits. Total: ~5 ms.
//! - Space: O(N) for the sieve bitmap.

use crate::utils::primes::sieve_bitmap;

/// Number of decimal digits in `n`. Returns 1 for n = 0.
fn digit_count(n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut count = 0u32;
    let mut x = n;
    while x > 0 {
        count += 1;
        x /= 10;
    }
    count
}

/// Rotate the decimal digits of `n` left by one position (leading digit moves
/// to the trailing position), preserving the digit count.
///
/// # Examples
/// ```
/// use project_euler::problems::p035::rotate_left;
/// assert_eq!(rotate_left(197, 3), 971);
/// assert_eq!(rotate_left(971, 3), 719);
/// assert_eq!(rotate_left(719, 3), 197); // back to start
/// ```
pub fn rotate_left(n: u64, num_digits: u32) -> u64 {
    let pow = 10u64.pow(num_digits - 1);
    let leading = n / pow;
    (n - leading * pow) * 10 + leading
}

/// Test whether `n` is a circular prime, given an `is_prime` sieve large
/// enough to look up every rotation of `n`.
///
/// # Examples
/// ```
/// use project_euler::problems::p035::is_circular_prime;
/// use project_euler::utils::primes::sieve_bitmap;
/// let sieve = sieve_bitmap(1_000);
/// assert!(is_circular_prime(197, &sieve));
/// assert!(!is_circular_prime(19, &sieve)); // 19 is prime but 91 = 7 × 13 is not
/// ```
pub fn is_circular_prime(n: u64, is_prime: &[bool]) -> bool {
    if !is_prime[n as usize] {
        return false;
    }
    let d = digit_count(n);
    let mut current = n;
    for _ in 0..d {
        if !is_prime[current as usize] {
            return false;
        }
        current = rotate_left(current, d);
    }
    true
}

/// Count circular primes strictly below `limit`.
///
/// # Examples
/// ```
/// use project_euler::problems::p035::count_circular_primes_below;
/// // From problem statement: 13 circular primes below 100
/// assert_eq!(count_circular_primes_below(100), 13);
/// ```
pub fn count_circular_primes_below(limit: usize) -> u64 {
    let sieve = sieve_bitmap(limit);
    (2..limit as u64)
        .filter(|&n| is_circular_prime(n, &sieve))
        .count() as u64
}

/// Solve Problem 35: count circular primes below one million.
pub fn solve() -> u64 {
    count_circular_primes_below(1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_count() {
        assert_eq!(digit_count(0), 1);
        assert_eq!(digit_count(7), 1);
        assert_eq!(digit_count(42), 2);
        assert_eq!(digit_count(197), 3);
        assert_eq!(digit_count(999_999), 6);
    }

    #[test]
    fn test_rotate_left() {
        // 3-digit rotations of 197: 197 → 971 → 719 → 197
        assert_eq!(rotate_left(197, 3), 971);
        assert_eq!(rotate_left(971, 3), 719);
        assert_eq!(rotate_left(719, 3), 197);

        // 2-digit rotation: 13 → 31
        assert_eq!(rotate_left(13, 2), 31);
    }

    #[test]
    fn test_problem_statement_example_197() {
        let sieve = sieve_bitmap(1_000);
        // From problem: 197, 971, 719 are all prime
        assert!(is_circular_prime(197, &sieve));
    }

    #[test]
    fn test_problem_statement_below_100() {
        // From problem: thirteen circular primes below 100
        assert_eq!(count_circular_primes_below(100), 13);
    }

    #[test]
    fn test_thirteen_below_100_set() {
        // Verify the exact set listed in the problem
        let sieve = sieve_bitmap(100);
        let circulars: Vec<u64> = (2..100u64)
            .filter(|&n| is_circular_prime(n, &sieve))
            .collect();
        assert_eq!(
            circulars,
            vec![2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, 97]
        );
    }

    #[test]
    fn test_non_circular_prime() {
        let sieve = sieve_bitmap(1_000);
        // 19 is prime, but rotation 91 = 7 × 13 is not
        assert!(!is_circular_prime(19, &sieve));
        // 23 is prime, but rotation 32 is even
        assert!(!is_circular_prime(23, &sieve));
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 55);
    }
}
