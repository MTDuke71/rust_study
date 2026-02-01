//! Common mathematical utilities for AoC 2023
//!
//! **Number Theory Functions**:
//! - GCD (Greatest Common Divisor) - Euclidean algorithm
//! - LCM (Least Common Multiple) - Used for cycle synchronization problems
//!
//! **Usage Pattern**: Cycle detection and synchronization
//! - Day 8: Ghost navigation - find when multiple paths synchronize
//! - Day 20: Pulse propagation - find when multiple counters align
//!
//! **Mathematical Foundation**:
//! - LCM(a, b) = (a × b) / GCD(a, b)
//! - GCD uses Euclidean algorithm: O(log min(a, b))
//!
//! **Zettelkasten Links**: \[\[number-theory\]\], \[\[cycle-detection\]\]

/// Calculate Greatest Common Divisor using Euclidean algorithm
///
/// # Examples
/// ```
/// use aoc2023::math_utils::gcd;
/// assert_eq!(gcd(12, 18), 6);
/// assert_eq!(gcd(21, 14), 7);
/// ```
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Calculate Least Common Multiple
///
/// # Examples
/// ```
/// use aoc2023::math_utils::lcm;
/// assert_eq!(lcm(4, 6), 12);
/// assert_eq!(lcm(21, 6), 42);
/// assert_eq!(lcm(12, 18), 36);
/// ```
pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

/// GCD for usize (used in some day solutions)
pub fn gcd_usize(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// LCM for usize (used in some day solutions)
pub fn lcm_usize(a: usize, b: usize) -> usize {
    a * b / gcd_usize(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(21, 14), 7);
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 13), 1); // Coprime
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(21, 6), 42);
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(lcm(7, 5), 35); // Coprime
    }

    #[test]
    fn test_gcd_usize() {
        assert_eq!(gcd_usize(12, 18), 6);
        assert_eq!(gcd_usize(21, 14), 7);
    }

    #[test]
    fn test_lcm_usize() {
        assert_eq!(lcm_usize(4, 6), 12);
        assert_eq!(lcm_usize(21, 6), 42);
    }
}
