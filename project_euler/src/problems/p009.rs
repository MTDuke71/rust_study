//! # Problem 9: Special Pythagorean Triplet
//!
//! Find the product abc where a < b < c form a Pythagorean triplet and a + b + c = 1000.
//!
//! ## Problem Statement
//!
//! A Pythagorean triplet is a set of three natural numbers, a < b < c, for which:
//! a² + b² = c²
//!
//! For example, 3² + 4² = 9 + 16 = 25 = 5².
//!
//! There exists exactly one Pythagorean triplet for which a + b + c = 1000.
//! Find the product abc.
//!
//! ## Algorithm
//!
//! Uses **constrained search** with mathematical bounds:
//!
//! 1. Since a < b < c and a + b + c = target:
//!    - a < target/3 (if a were target/3, b and c would be equal or larger)
//!    - For given a: b ranges from a+1 to (target-a)/2
//!    - c is determined: c = target - a - b
//!
//! 2. For each valid (a, b):
//!    - Calculate c = target - a - b
//!    - Check if a² + b² = c² (Pythagorean condition)
//!    - If yes, return product a × b × c
//!
//! ## Complexity
//!
//! - **Time**: O(n²) where n is the target sum
//!   - Outer loop: a from 1 to target/3 ≈ O(n)
//!   - Inner loop: b from a+1 to (target-a)/2 ≈ O(n)
//!   - Total: O(n²)
//!
//! - **Space**: O(1) - only storing loop variables
//!
//! ## Mathematical Foundation
//!
//! **Pythagorean Theorem**: In a right triangle with legs a and b and hypotenuse c:
//! $$a^2 + b^2 = c^2$$
//!
//! **Pythagorean Triplets**: Integer solutions to the Pythagorean theorem
//! - Primitive triplets: gcd(a, b, c) = 1
//! - Non-primitive: multiples of primitive triplets
//!
//! See `zettelkasten/math-foundations/pythagorean-triplets.md` for theory.

/// Represents a Pythagorean triplet (a, b, c) where a² + b² = c².
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PythagoreanTriplet {
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

impl PythagoreanTriplet {
    /// Creates a new Pythagorean triplet if the values satisfy a² + b² = c².
    ///
    /// # Arguments
    ///
    /// * `a` - First value (should be smallest)
    /// * `b` - Second value (middle)
    /// * `c` - Third value (should be largest)
    ///
    /// # Returns
    ///
    /// `Some(PythagoreanTriplet)` if valid, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use project_euler::problems::p009::PythagoreanTriplet;
    ///
    /// let triplet = PythagoreanTriplet::new(3, 4, 5);
    /// assert!(triplet.is_some());
    ///
    /// let invalid = PythagoreanTriplet::new(1, 2, 3);
    /// assert!(invalid.is_none());
    /// ```
    pub fn new(a: u32, b: u32, c: u32) -> Option<Self> {
        if a < b && b < c && a * a + b * b == c * c {
            Some(Self { a, b, c })
        } else {
            None
        }
    }

    /// Returns the sum a + b + c.
    pub fn sum(&self) -> u32 {
        self.a + self.b + self.c
    }

    /// Returns the product a × b × c.
    pub fn product(&self) -> u32 {
        self.a * self.b * self.c
    }
}

/// Finds the unique Pythagorean triplet where a + b + c equals the target sum.
///
/// # Arguments
///
/// * `target` - The desired sum of the triplet
///
/// # Returns
///
/// `Some(PythagoreanTriplet)` if found, `None` otherwise.
///
/// # Algorithm
///
/// Searches through all possible combinations of (a, b) with optimized bounds:
/// - a ranges from 1 to target/3
/// - b ranges from a+1 to (target-a)/2
/// - c is calculated as target - a - b
/// - Checks if a² + b² = c²
///
/// # Examples
///
/// ```
/// use project_euler::problems::p009::find_pythagorean_triplet;
///
/// // Example: 3 + 4 + 5 = 12
/// let triplet = find_pythagorean_triplet(12).unwrap();
/// assert_eq!(triplet.a, 3);
/// assert_eq!(triplet.b, 4);
/// assert_eq!(triplet.c, 5);
/// ```
pub fn find_pythagorean_triplet(target: u32) -> Option<PythagoreanTriplet> {
    // a < b < c and a + b + c = target
    // Since a is smallest: a < target/3
    for a in 1..target / 3 {
        // For given a: b < c and b + c = target - a
        // Since b < c: b < (target - a) / 2
        // But also b > a, so b starts at a + 1
        // Upper bound: Since c = target - a - b and we need b < c:
        //   b < target - a - b
        //   2b < target - a
        //   b < (target - a) / 2
        for b in (a + 1)..=(target - a) / 2 {
            let c = target - a - b;

            // Check Pythagorean condition
            if a * a + b * b == c * c {
                return PythagoreanTriplet::new(a, b, c);
            }
        }
    }

    None
}

/// Solves Problem 9 by finding the product of the Pythagorean triplet summing to 1000.
///
/// # Returns
///
/// The product a × b × c of the unique Pythagorean triplet where a + b + c = 1000.
///
/// # Examples
///
/// ```
/// use project_euler::problems::p009::solve;
///
/// let answer = solve();
/// assert_eq!(answer, 31875000);
/// ```
pub fn solve() -> u32 {
    find_pythagorean_triplet(1000)
        .map(|t| t.product())
        .expect("Problem guarantees a solution exists")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classic_triplet() {
        let triplet = PythagoreanTriplet::new(3, 4, 5).unwrap();
        assert_eq!(triplet.a, 3);
        assert_eq!(triplet.b, 4);
        assert_eq!(triplet.c, 5);
        assert_eq!(triplet.sum(), 12);
        assert_eq!(triplet.product(), 60);
    }

    #[test]
    fn test_invalid_triplet() {
        assert!(PythagoreanTriplet::new(1, 2, 3).is_none());
        assert!(PythagoreanTriplet::new(2, 3, 4).is_none());
    }

    #[test]
    fn test_triplet_ordering() {
        // Must be strictly increasing
        assert!(PythagoreanTriplet::new(4, 3, 5).is_none());
        assert!(PythagoreanTriplet::new(3, 5, 4).is_none());
    }

    #[test]
    fn test_find_triplet_sum_12() {
        let triplet = find_pythagorean_triplet(12).unwrap();
        assert_eq!(triplet.a, 3);
        assert_eq!(triplet.b, 4);
        assert_eq!(triplet.c, 5);
    }

    #[test]
    fn test_find_triplet_sum_30() {
        let triplet = find_pythagorean_triplet(30).unwrap();
        assert_eq!(triplet.a, 5);
        assert_eq!(triplet.b, 12);
        assert_eq!(triplet.c, 13);
    }

    #[test]
    fn test_find_triplet_sum_24() {
        // 6² + 8² = 36 + 64 = 100 = 10²
        // 6 + 8 + 10 = 24
        let triplet = find_pythagorean_triplet(24).unwrap();
        assert_eq!(triplet.a, 6);
        assert_eq!(triplet.b, 8);
        assert_eq!(triplet.c, 10);
    }

    #[test]
    fn test_no_triplet_for_sum_11() {
        assert!(find_pythagorean_triplet(11).is_none());
    }

    #[test]
    fn test_solve() {
        let answer = solve();
        assert_eq!(answer, 31875000);
    }

    #[test]
    fn test_triplet_properties() {
        let triplet = find_pythagorean_triplet(1000).unwrap();
        // Verify Pythagorean condition
        assert_eq!(
            triplet.a * triplet.a + triplet.b * triplet.b,
            triplet.c * triplet.c
        );
        // Verify sum
        assert_eq!(triplet.sum(), 1000);
        // Verify ordering
        assert!(triplet.a < triplet.b && triplet.b < triplet.c);
    }
}
