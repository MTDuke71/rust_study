//! # Problem 3: Largest Prime Factor
//!
//! Find the largest prime factor of 600851475143.
//!
//! ## Mathematical Foundation
//!
//! Trial division with progressive factor elimination:
//! - Remove factor 2 until odd.
//! - Try odd factors up to $\sqrt{n}$, dividing out repeatedly.
//! - If remaining $n>1$, it is prime and thus the largest factor.
//!
//! Complexity: Roughly $O(\sqrt{n})$ worst-case for composite numbers.
//!
/// Compute the largest prime factor of `n`.
///
/// Efficiently divides out small factors first, then iterates odd candidates.
/// Returns 1 for `n<2` by convention (no prime factors).
///
/// # Examples
/// ```
/// # use project_euler::problems::p003::largest_prime_factor;
/// assert_eq!(largest_prime_factor(13195), 29); // 5,7,13,29 → largest 29
/// ```
pub fn largest_prime_factor(mut n: u64) -> u64 {
    if n < 2 {
        return 1;
    }

    let mut last_factor = 1u64;

    // Remove factor 2
    while n.is_multiple_of(2) {
        last_factor = 2;
        n /= 2;
    }

    // Check odd factors
    let mut f = 3u64;
    while f * f <= n {
        if n.is_multiple_of(f) {
            last_factor = f;
            while n.is_multiple_of(f) {
                n /= f;
            }
        }
        f += 2;
    }

    // If remainder is >1, it's prime and larger than any tested factor.
    if n > 1 {
        n
    } else {
        last_factor
    }
}

/// Solve Problem 3 for the canonical input `600851475143`.
///
/// # Examples
/// ```
/// use project_euler::problems::p003::solve;
/// assert_eq!(solve(), 6857);
/// ```
pub fn solve() -> u64 {
    largest_prime_factor(600_851_475_143)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(largest_prime_factor(1), 1);
        assert_eq!(largest_prime_factor(2), 2);
        assert_eq!(largest_prime_factor(3), 3);
        assert_eq!(largest_prime_factor(4), 2);
        assert_eq!(largest_prime_factor(13195), 29);
    }

    #[test]
    fn test_solution() {
        assert_eq!(solve(), 6857);
    }
}
