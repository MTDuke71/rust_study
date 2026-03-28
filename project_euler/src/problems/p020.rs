//! # Problem 20: Factorial Digit Sum
//!
//! n! means n × (n − 1) × ... × 3 × 2 × 1
//!
//! 10! = 3,628,800, and the sum of the digits is 3+6+2+8+8+0+0 = 27.
//!
//! Find the sum of the digits in 100!
//!
//! ## Mathematical Foundation
//!
//! 100! has 158 digits (Stirling: log10(n!) ≈ n·log10(n/e) + ½·log10(2πn)).
//! This exceeds u64 (~18 digits) and u128 (~38 digits), so we use the same
//! **digit-array big multiplication** technique from Problem 16.
//!
//! Key difference from P016: instead of multiplying by 2 each step, we
//! multiply by successive factors 2, 3, ..., n. The carry can exceed a
//! single digit since factors go up to 100.
//!
//! See [[math-foundations/project-euler-p020]] for full analysis.
//!
//! ## Complexity
//!
//! - Time: O(n × d) where d = digits in n! (~158 for n=100)
//! - Space: O(d)

/// Compute the digit sum of n! using digit-array big multiplication.
///
/// Stores digits least-significant first, multiplying by each factor
/// with carry propagation — same technique as Problem 16's `power_digit_sum`.
///
/// # Examples
/// ```
/// use project_euler::problems::p020::factorial_digit_sum;
/// assert_eq!(factorial_digit_sum(10), 27);
/// ```
pub fn factorial_digit_sum(n: u32) -> u64 {
    // digits[0] = ones place (least-significant first)
    let mut digits: Vec<u32> = vec![1];

    for factor in 2..=n {
        let mut carry = 0u32;
        for d in digits.iter_mut() {
            let product = *d * factor + carry;
            *d = product % 10;
            carry = product / 10;
        }
        while carry > 0 {
            digits.push(carry % 10);
            carry /= 10;
        }
    }

    digits.iter().map(|&d| d as u64).sum()
}

/// Solve Problem 20: sum of digits in 100!
pub fn solve() -> u64 {
    factorial_digit_sum(100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_10_factorial() {
        // 10! = 3628800, digit sum = 3+6+2+8+8+0+0 = 27
        assert_eq!(factorial_digit_sum(10), 27);
    }

    #[test]
    fn test_small_factorials() {
        // 0! = 1, digit sum = 1
        assert_eq!(factorial_digit_sum(0), 1);
        // 1! = 1, digit sum = 1
        assert_eq!(factorial_digit_sum(1), 1);
        // 5! = 120, digit sum = 3
        assert_eq!(factorial_digit_sum(5), 3);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 648);
    }
}
