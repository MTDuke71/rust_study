//! # Problem 31: Coin Sums
//!
//! In the United Kingdom the currency is made up of pound (£) and pence (p).
//! There are eight coins in general circulation:
//!
//!   1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), £2 (200p)
//!
//! How many different ways can £2 (200p) be made using any number of these coins?
//!
//! ## Mathematical Foundation
//!
//! This is the **restricted integer partition** problem (also called the
//! **coin change counting** problem). We count the number of multisets of coin
//! denominations that sum to the target — order does not matter.
//!
//! Generating function view: the answer is the coefficient of x^200 in
//!
//!   ∏ 1 / (1 − x^c)   for c ∈ {1, 2, 5, 10, 20, 50, 100, 200}
//!
//! See [[math-foundations/integer-partitions]] for the unrestricted partition
//! function p(n) and how restricted partitions specialize it.
//!
//! ## Approach
//!
//! Classic 1D dynamic programming. Let `ways[s]` = number of ways to make sum `s`.
//!
//! ```text
//! ways[0] = 1                     (empty multiset makes 0)
//! for each coin c:
//!     for s = c..=target:
//!         ways[s] += ways[s - c]
//! ```
//!
//! Iterating coins in the **outer** loop and sums in the **inner** loop is the
//! key trick: it counts unordered combinations. Swapping the loop order would
//! count ordered sequences (compositions), which would double-count e.g.
//! 1p+2p and 2p+1p as distinct.
//!
//! ## Complexity
//!
//! - Time: O(target × |coins|)
//! - Space: O(target)

const COINS: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

/// Count the number of unordered ways to make `target` pence using the given coins.
///
/// Each coin may be used any number of times (including zero).
///
/// # Examples
/// ```
/// use project_euler::problems::p031::count_coin_combinations;
/// // Only one way to make 0: use no coins
/// assert_eq!(count_coin_combinations(0, &[1, 2, 5]), 1);
/// // Ways to make 5p with {1, 2, 5}: 5; 2+2+1; 2+1+1+1; 1+1+1+1+1; 5 = 4 ways
/// assert_eq!(count_coin_combinations(5, &[1, 2, 5]), 4);
/// ```
pub fn count_coin_combinations(target: usize, coins: &[usize]) -> u64 {
    let mut ways = vec![0u64; target + 1];
    ways[0] = 1;

    for &coin in coins {
        for s in coin..=target {
            ways[s] += ways[s - coin];
        }
    }

    ways[target]
}

/// Solve Problem 31: number of ways to make £2 (200p) using UK coins.
pub fn solve() -> u64 {
    count_coin_combinations(200, &COINS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_target() {
        // Empty multiset is the unique way to make 0
        assert_eq!(count_coin_combinations(0, &COINS), 1);
    }

    #[test]
    fn test_single_coin() {
        // With only 1p coins, every target has exactly one combination
        assert_eq!(count_coin_combinations(7, &[1]), 1);
    }

    #[test]
    fn test_small_example() {
        // 5p with {1, 2, 5}: {5}, {2,2,1}, {2,1,1,1}, {1,1,1,1,1} = 4 ways
        assert_eq!(count_coin_combinations(5, &[1, 2, 5]), 4);
    }

    #[test]
    fn test_target_below_smallest_coin() {
        // Cannot make 3p with only 5p coins
        assert_eq!(count_coin_combinations(3, &[5]), 0);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 73682);
    }
}
