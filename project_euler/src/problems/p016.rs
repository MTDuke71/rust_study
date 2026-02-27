//! # Problem 16: Power Digit Sum
//!
//! 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
//!
//! What is the sum of the digits of the number 2^1000?
//!
//! ## Mathematical Foundation
//!
//! 2^1000 has ~302 decimal digits (log10(2^1000) = 1000 * log10(2) ≈ 301.03),
//! far exceeding u64 (~18 digits) or u128 (~38 digits).
//!
//! We use **big number arithmetic via digit arrays**: store each decimal digit
//! in a Vec, multiply by 2 with carry propagation, 1000 times.
//!
//! See [[math-foundations/combinatorics-fundamentals]] for counting theory.
//!
//! ## Complexity
//!
//! - Time: O(exponent × digits) ≈ O(1000 × 302) = O(302,000)
//! - Space: O(digits) ≈ O(302)

/// Compute the sum of digits of `base^exponent` using digit-array arithmetic.
///
/// # Examples
/// ```
/// use project_euler::problems::p016::power_digit_sum;
/// assert_eq!(power_digit_sum(2, 15), 26);
/// ```
pub fn power_digit_sum(base: u32, exponent: u32) -> u64 {
    // digits stored least-significant first: digits[0] = ones place
    let mut digits: Vec<u32> = vec![1];

    for _ in 0..exponent {
        let mut carry = 0;
        for d in digits.iter_mut() {
            let product = *d * base + carry;
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

/// Solve Problem 16: sum of digits of 2^1000.
pub fn solve() -> u64 {
    power_digit_sum(2, 1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_2_pow_15() {
        // From problem statement: 2^15 = 32768, digit sum = 26
        assert_eq!(power_digit_sum(2, 15), 26);
    }

    #[test]
    fn test_small_powers() {
        // 2^0 = 1, digit sum = 1
        assert_eq!(power_digit_sum(2, 0), 1);
        // 2^1 = 2, digit sum = 2
        assert_eq!(power_digit_sum(2, 1), 2);
        // 2^10 = 1024, digit sum = 7
        assert_eq!(power_digit_sum(2, 10), 7);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 1366);
    }
}
