//! # Problem 2: Even Fibonacci Numbers
//!
//! Sum all even-valued Fibonacci terms not exceeding 4,000,000.
//!
//! ## Mathematical Foundation
//!
//! - Fibonacci sequence: $F_0=0, F_1=1, F_{n}=F_{n-1}+F_{n-2}$
//! - Every third Fibonacci number is even.
//! - Efficient recurrence for even terms: $E_n = 4\,E_{n-1} + E_{n-2}$.
//!
//! Complexity: O(k) where k is the number of even terms under the limit (very small).
//!
//! Full write-up: see zettelkasten note once created.
//!
/// Return the sum of even Fibonacci numbers not exceeding `limit`.
///
/// Uses the recurrence on even Fibonacci terms for efficiency:
/// starting from 2 and 8, next even term is `4*prev + prev_prev`.
///
/// # Examples
/// ```
/// # use project_euler::problems::p002::sum_even_fib;
/// assert_eq!(sum_even_fib(100), 44); // 2 + 8 + 34 = 44
/// ```
pub fn sum_even_fib(limit: u64) -> u64 {
    if limit < 2 {
        return 0;
    }
    // Even Fibonacci seed terms: E0=2 (F3), E1=8 (F6)
    let mut sum = 2u64; // include 2
    let mut e_prev = 2u64;
    let mut e_curr = 8u64;

    while e_curr <= limit {
        sum += e_curr;
        let next = 4 * e_curr + e_prev; // E(n) = 4*E(n-1) + E(n-2)
        e_prev = e_curr;
        e_curr = next;
    }

    sum
}

/// Solve Problem 2: sum of even Fibonacci numbers <= 4,000,000
///
/// # Examples
/// ```
/// use project_euler::problems::p002::solve;
/// assert_eq!(solve(), 4613732);
/// ```
pub fn solve() -> u64 {
    sum_even_fib(4_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_limit() {
        assert_eq!(sum_even_fib(8), 10); // 2 + 8
        assert_eq!(sum_even_fib(34), 44); // 2 + 8 + 34
    }

    #[test]
    fn test_solution() {
        assert_eq!(solve(), 4613732);
    }
}
