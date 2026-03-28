//! Number theory utilities
//!
//! GCD, LCM, divisors, factorization, modular arithmetic.

/// Greatest Common Divisor using Euclidean algorithm
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Least Common Multiple
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a * b) / gcd(a, b)
}

/// Get all divisors of n
pub fn divisors(n: u64) -> Vec<u64> {
    let mut divs = Vec::new();
    let limit = (n as f64).sqrt() as u64;

    for i in 1..=limit {
        if n.is_multiple_of(i) {
            divs.push(i);
            if i != n / i {
                divs.push(n / i);
            }
        }
    }

    divs.sort_unstable();
    divs
}

/// Sum of divisors of n (excluding n itself)
pub fn sum_of_proper_divisors(n: u64) -> u64 {
    divisors(n).iter().filter(|&&d| d != n).sum()
}

/// Sieve to compute the sum of proper divisors for all numbers 0..limit.
///
/// For each divisor d (1..limit/2), adds d to every multiple of d.
/// This is analogous to the Sieve of Eratosthenes but accumulates sums
/// instead of marking composites.
///
/// Returns a Vec where `result[n]` = sum of proper divisors of n.
///
/// # Complexity
/// - Time: O(n log n) — harmonic series: n/1 + n/2 + n/3 + ... ≈ n·ln(n)
/// - Space: O(n)
///
/// # Examples
/// ```
/// use project_euler::utils::number_theory::proper_divisor_sum_sieve;
/// let sieve = proper_divisor_sum_sieve(300);
/// assert_eq!(sieve[220], 284);  // d(220) = 284
/// assert_eq!(sieve[284], 220);  // d(284) = 220 — amicable pair!
/// assert_eq!(sieve[28], 28);    // perfect number
/// ```
pub fn proper_divisor_sum_sieve(limit: usize) -> Vec<u64> {
    let mut d = vec![0u64; limit];

    for divisor in 1..limit {
        // Add `divisor` to every multiple (starting at 2*divisor to exclude n itself)
        let mut multiple = 2 * divisor;
        while multiple < limit {
            d[multiple] += divisor as u64;
            multiple += divisor;
        }
    }

    d
}

/// Prime factorization
pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();

    // Check for 2s
    while n.is_multiple_of(2) {
        factors.push(2);
        n /= 2;
    }

    // Check odd factors
    let mut i = 3;
    while i * i <= n {
        while n.is_multiple_of(i) {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }

    if n > 2 {
        factors.push(n);
    }

    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(17, 19), 1);
        assert_eq!(gcd(100, 50), 50);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(12, 8), 24);
        assert_eq!(lcm(3, 5), 15);
    }

    #[test]
    fn test_divisors() {
        assert_eq!(divisors(12), vec![1, 2, 3, 4, 6, 12]);
        assert_eq!(divisors(28), vec![1, 2, 4, 7, 14, 28]);
    }

    #[test]
    fn test_proper_divisor_sum_sieve() {
        let sieve = proper_divisor_sum_sieve(300);
        // Amicable pair: d(220) = 284, d(284) = 220
        assert_eq!(sieve[220], 284);
        assert_eq!(sieve[284], 220);
        // Perfect numbers: d(n) = n
        assert_eq!(sieve[6], 6);    // 1+2+3
        assert_eq!(sieve[28], 28);  // 1+2+4+7+14
        // Cross-check against single-value function
        assert_eq!(sieve[12], sum_of_proper_divisors(12));
        assert_eq!(sieve[100], sum_of_proper_divisors(100));
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(12), vec![2, 2, 3]);
        assert_eq!(prime_factors(28), vec![2, 2, 7]);
        assert_eq!(prime_factors(17), vec![17]);
    }
}
