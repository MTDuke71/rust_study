//! Prime number utilities
//!
//! Sieve of Eratosthenes, primality testing, prime generation.

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
    
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    for i in 2..=((n as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    
    is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i) } else { None })
        .collect()
}

/// Check if n is prime
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }
    
    let limit = (n as f64).sqrt() as u64;
    for i in (3..=limit).step_by(2) {
        if n.is_multiple_of(i) {
            return false;
        }
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
