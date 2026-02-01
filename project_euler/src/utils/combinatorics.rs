//! Combinatorics utilities
//!
//! Factorials, permutations, combinations, binomial coefficients.

/// Factorial of n
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

/// Binomial coefficient C(n, k) = n! / (k! * (n-k)!)
pub fn binomial(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }

    let k = k.min(n - k); // Optimization: C(n,k) = C(n, n-k)

    let mut result = 1u64;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

/// Number of permutations P(n, k) = n! / (n-k)!
pub fn permutations(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }

    let mut result = 1u64;
    for i in 0..k {
        result *= n - i;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_binomial() {
        assert_eq!(binomial(5, 2), 10); // C(5,2) = 10
        assert_eq!(binomial(10, 3), 120); // C(10,3) = 120
        assert_eq!(binomial(5, 0), 1);
        assert_eq!(binomial(5, 5), 1);
    }

    #[test]
    fn test_permutations() {
        assert_eq!(permutations(5, 2), 20); // P(5,2) = 20
        assert_eq!(permutations(10, 3), 720); // P(10,3) = 720
    }
}
