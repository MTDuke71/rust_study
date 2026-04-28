//! Prime number utilities
//!
//! Sieve of Eratosthenes, primality testing, prime generation.

/// Sieve of Eratosthenes as a primality bitmap.
///
/// Returns a `Vec<bool>` of length `limit` where `result[i]` is `true` iff
/// `i` is prime. This form supports O(1) primality lookup, which `sieve(n)`
/// (returning the prime list) does not.
///
/// Use this when you need to look up primality of arbitrary values (e.g.,
/// digit-rotation checks in [[project-euler-p035]]). Use `sieve(n)` when you
/// only need to enumerate primes.
///
/// # Examples
/// ```
/// use project_euler::utils::primes::sieve_bitmap;
/// let p = sieve_bitmap(10); // indices 0..10
/// assert!(!p[0] && !p[1]);
/// assert!(p[2] && p[3] && !p[4] && p[5] && !p[6] && p[7]);
/// assert!(!p[8] && !p[9]);
/// ```
pub fn sieve_bitmap(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit];
    if limit > 0 {
        is_prime[0] = false;
    }
    if limit > 1 {
        is_prime[1] = false;
    }
    let mut i = 2;
    while i * i < limit {
        if is_prime[i] {
            let mut j = i * i;
            while j < limit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    is_prime
}

/// Generate primes up to n using Sieve of Eratosthenes
///
/// # Examples
/// ```
/// use project_euler::utils::primes::sieve;
/// let primes = sieve(10);
/// assert_eq!(primes, vec![2, 3, 5, 7]);
/// ```
pub fn sieve(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    sieve_bitmap(n + 1)
        .iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i) } else { None })
        .collect()
}

/// Check if n is prime using trial division with 6k±1 optimization
///
/// All primes > 3 are of the form 6k±1. This reduces trial division
/// candidates by ~33% compared to testing all odd numbers.
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n.is_multiple_of(2) || n.is_multiple_of(3) {
        return false;
    }

    // All primes > 3 are of form 6k±1
    // Test divisors: 5, 7, 11, 13, 17, 19, 23, 25, 29, 31, ...
    // Pattern: 6k-1, 6k+1, 6k+5, 6k+7, ... = 6k-1, 6k+1 repeating
    let limit = (n as f64).sqrt() as u64;
    let mut i = 5;
    while i <= limit {
        if n.is_multiple_of(i) || n.is_multiple_of(i + 2) {
            return false;
        }
        i += 6; // Next candidates: 6k-1 and 6k+1
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve() {
        assert_eq!(sieve(10), vec![2, 3, 5, 7]);
        assert_eq!(sieve(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn test_sieve_bitmap_basic() {
        let p = sieve_bitmap(20);
        // 0, 1 are not prime
        assert!(!p[0]);
        assert!(!p[1]);
        // First few primes
        assert!(p[2] && p[3] && p[5] && p[7] && p[11] && p[13] && p[17] && p[19]);
        // Composites
        assert!(!p[4] && !p[6] && !p[8] && !p[9] && !p[10] && !p[15]);
    }

    #[test]
    fn test_sieve_bitmap_empty_and_tiny() {
        // Edge cases: limit < 2 should not panic
        assert_eq!(sieve_bitmap(0), Vec::<bool>::new());
        assert_eq!(sieve_bitmap(1), vec![false]);
        assert_eq!(sieve_bitmap(2), vec![false, false]);
    }

    #[test]
    fn test_sieve_bitmap_matches_sieve() {
        // Both forms must agree on which indices are prime
        let limit = 100usize;
        let bitmap = sieve_bitmap(limit);
        let primes_list = sieve(limit - 1);
        for (i, &is_p) in bitmap.iter().enumerate() {
            assert_eq!(is_p, primes_list.contains(&i), "mismatch at i={i}");
        }
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(is_prime(97));
        assert!(!is_prime(100));
    }
}
