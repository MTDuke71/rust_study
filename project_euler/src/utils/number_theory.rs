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
        if n % i == 0 {
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

/// Prime factorization
pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    
    // Check for 2s
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    
    // Check odd factors
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
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
    fn test_prime_factors() {
        assert_eq!(prime_factors(12), vec![2, 2, 3]);
        assert_eq!(prime_factors(28), vec![2, 2, 7]);
        assert_eq!(prime_factors(17), vec![17]);
    }
}
