//! # Problem 8: Largest Product in a Series
//!
//! Find the thirteen adjacent digits in the 1000-digit number that have the greatest product.
//!
//! ## Problem Statement
//!
//! The four adjacent digits in the 1000-digit number that have the greatest product are
//! 9 × 9 × 8 × 9 = 5832.
//!
//! Find the thirteen adjacent digits in the 1000-digit number that have the greatest product.
//! What is the value of this product?
//!
//! ## Algorithm
//!
//! Uses a **sliding window** approach:
//! 1. Convert the number string to a vector of digits
//! 2. Slide a window of size 13 across the digits
//! 3. Calculate the product for each window
//! 4. Track the maximum product
//!
//! **Optimization**: Skip windows containing zeros since they produce a product of zero.
//!
//! ## Complexity
//!
//! - **Time**: O(n × k) where n is the number of digits and k is the window size (13)
//! - **Space**: O(n) for storing the digit array
//!
//! ## Mathematical Foundation
//!
//! This problem demonstrates the **sliding window pattern**, a common algorithmic technique
//! for processing contiguous subsequences efficiently.
//!
//! See `zettelkasten/math-foundations/sliding-window-pattern.md` for theory.

const NUMBER: &str = "\
73167176531330624919225119674426574742355349194934\
96983520312774506326239578318016984801869478851843\
85861560789112949495459501737958331952853208805511\
12540698747158523863050715693290963295227443043557\
66896648950445244523161731856403098711121722383113\
62229893423380308135336276614282806444486645238749\
30358907296290491560440772390713810515859307960866\
70172427121883998797908792274921901699720888093776\
65727333001053367881220235421809751254540594752243\
52584907711670556013604839586446706324415722155397\
53697817977846174064955149290862569321978468622482\
83972241375657056057490261407972968652414535100474\
82166370484403199890008895243450658541227588666881\
16427171479924442928230863465674813919123162824586\
17866458359124566529476545682848912883142607690042\
24219022671055626321111109370544217506941658960408\
07198403850962455444362981230987879927244284909188\
84580156166097919133875499200524063689912560717606\
05886116467109405077541002256983155200055935729725\
71636269561882670428252483600823257530420752963450";

/// Finds the largest product of `window_size` consecutive digits in the given number string.
///
/// # Arguments
///
/// * `number` - String representation of the number (digits only)
/// * `window_size` - Number of consecutive digits to multiply
///
/// # Returns
///
/// The largest product found, or 0 if the number is too short.
///
/// # Examples
///
/// ```
/// use project_euler::problems::p008::largest_product_in_series;
///
/// let number = "123456789";
/// let product = largest_product_in_series(number, 4);
/// assert_eq!(product, 3024); // 6 × 7 × 8 × 9
/// ```
pub fn largest_product_in_series(number: &str, window_size: usize) -> u64 {
    // Convert string to digit array
    let digits: Vec<u64> = number
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u64)
        .collect();

    // If the number is too short, return 0
    if digits.len() < window_size {
        return 0;
    }

    let mut max_product = 0u64;

    // Slide the window across all positions
    for window_start in 0..=digits.len() - window_size {
        let window = &digits[window_start..window_start + window_size];

        // Optimization: skip windows containing zeros
        if window.contains(&0) {
            continue;
        }

        // Calculate product of this window
        let product = window.iter().product::<u64>();

        if product > max_product {
            max_product = product;
        }
    }

    max_product
}

/// Solves Problem 8 using the standard input (1000-digit number, 13 adjacent digits).
///
/// # Returns
///
/// The largest product of 13 consecutive digits in the 1000-digit number.
///
/// # Examples
///
/// ```
/// use project_euler::problems::p008::solve;
///
/// let answer = solve();
/// assert_eq!(answer, 23514624000);
/// ```
pub fn solve() -> u64 {
    largest_product_in_series(NUMBER, 13)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        // The problem states: "The four adjacent digits in the 1000-digit number
        // that have the greatest product are 9 × 9 × 8 × 9 = 5832"
        let result = largest_product_in_series(NUMBER, 4);
        assert_eq!(result, 5832);
    }

    #[test]
    fn test_simple_cases() {
        assert_eq!(largest_product_in_series("123456789", 1), 9);
        assert_eq!(largest_product_in_series("123456789", 2), 8 * 9);
        assert_eq!(largest_product_in_series("123456789", 3), 7 * 8 * 9);
    }

    #[test]
    fn test_with_zeros() {
        // Zeros should not affect the result (skipped)
        assert_eq!(largest_product_in_series("1230456789", 3), 7 * 8 * 9);
        assert_eq!(largest_product_in_series("9998000123", 3), 9 * 9 * 9);
    }

    #[test]
    fn test_too_short() {
        assert_eq!(largest_product_in_series("123", 5), 0);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(largest_product_in_series("0000", 2), 0);
    }

    #[test]
    fn test_solve() {
        let answer = solve();
        assert_eq!(answer, 23514624000);
    }
}
