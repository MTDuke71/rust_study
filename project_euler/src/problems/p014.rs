//! # Problem 14: Longest Collatz Sequence
//!
//! The following iterative sequence is defined for positive integers:
//!   n → n/2    (n is even)
//!   n → 3n + 1 (n is odd)
//!
//! Which starting number, under one million, produces the longest chain?
//!
//! ## Mathematical Foundation
//!
//! The **Collatz conjecture** (unsolved!) states every positive integer
//! eventually reaches 1. We exploit memoization: many chains share suffixes.
//!
//! ## Approach
//!
//! Memoized chain lengths - cache results for starting values < limit
//! so overlapping subproblems are computed only once.
//!
//! ## Complexity
//!
//! - Time: O(n) amortized with memoization
//! - Space: O(n) for the cache

/// Compute the Collatz chain length for a starting number (no memoization).
///
/// # Examples
/// ```
/// use project_euler::problems::p014::collatz_chain_length;
/// assert_eq!(collatz_chain_length(13), 10);
/// ```
pub fn collatz_chain_length(mut n: u64) -> u64 {
    let mut count = 1;
    while n != 1 {
        if n.is_multiple_of(2) {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        count += 1;
    }
    count
}

/// Solve Problem 14
///
/// Find the starting number under 1,000,000 that produces the longest Collatz chain.
pub fn solve() -> u64 {
    longest_collatz_under(1_000_000)
}

/// Find the starting number under `limit` with the longest Collatz chain.
pub fn longest_collatz_under(limit: u64) -> u64 {
    let limit = limit as usize;
    // cache[i] = chain length starting from i (0 means not yet computed)
    let mut cache = vec![0u32; limit];
    cache[1] = 1;

    let half = limit / 2;
    let mut best_start = 1;
    let mut best_len = 1u32;

    for i in 2..limit {
        // Skip even numbers < half: their double (2i < limit) always has a longer chain
        if i < half && i.is_multiple_of(2) {
            continue;
        }
        if cache[i] != 0 {
            continue;
        }
        // Walk the chain, collecting uncached values with their step costs
        // Each entry is (value, steps_to_next) — odd shortcut counts as 2 steps
        let mut chain: Vec<(u64, u32)> = Vec::new();
        let mut n = i as u64;
        while n as usize >= limit || cache[n as usize] == 0 {
            if n.is_multiple_of(2) {
                chain.push((n, 1));
                n /= 2;
            } else {
                // 3n+1 is always even, so combine: n → 3n+1 → (3n+1)/2
                chain.push((n, 2));
                #[allow(clippy::manual_div_ceil)] // Collatz formula: 3n+1 then /2, not ceil division
                    { n = (3 * n + 1) / 2; }
            }
        }
        // Now cache[n] is known - backfill using each entry's step cost
        let mut length = cache[n as usize];
        for &(val, steps) in chain.iter().rev() {
            length += steps;
            if (val as usize) < limit {
                cache[val as usize] = length;
                if length > best_len {
                    best_len = length;
                    best_start = val as usize;
                }
            }
        }
    }

    best_start as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_example() {
        // 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1 = 10 terms
        assert_eq!(collatz_chain_length(13), 10);
    }

    #[test]
    fn test_collatz_base_cases() {
        assert_eq!(collatz_chain_length(1), 1);
        assert_eq!(collatz_chain_length(2), 2); // 2 → 1
    }

    #[test]
    fn test_small_range() {
        // Under 10, starting number 9 has the longest chain
        // 9 → 28 → 14 → 7 → 22 → 11 → 34 → 17 → 52 → 26 → 13 → ... → 1 = 20 terms
        assert_eq!(collatz_chain_length(9), 20);
        assert_eq!(longest_collatz_under(10), 9);
    }

    #[test]
    fn test_solve() {
        let answer = solve();
        println!("Problem 14 answer: {}", answer);
        println!("Chain length: {}", collatz_chain_length(answer));
        assert!(answer > 0);
        assert!(answer < 1_000_000);
    }

    #[test]
    fn analyze_cache_effectiveness() {
        let limit: usize = 500;
        let mut cache = vec![0u32; limit];
        cache[1] = 1;

        let half = limit / 2;
        let mut chains_computed = 0u64; // starting numbers we actually walk
        let mut steps_walked = 0u64; // steps we compute during walks
        let mut steps_saved = 0u64; // steps skipped via cache hit
        let mut evens_skipped = 0u64; // even numbers < half we skip entirely
        let mut already_cached = 0u64; // starting numbers already filled by backfill

        for i in 2..limit {
            if i < half && i.is_multiple_of(2) {
                evens_skipped += 1;
                continue;
            }
            if cache[i] != 0 {
                already_cached += 1;
                continue;
            }

            chains_computed += 1;
            let mut chain: Vec<(u64, u32)> = Vec::new();
            let mut n = i as u64;
            while n as usize >= limit || cache[n as usize] == 0 {
                if n.is_multiple_of(2) {
                    chain.push((n, 1));
                    n /= 2;
                } else {
                    chain.push((n, 2));
                    #[allow(clippy::manual_div_ceil)] // Collatz formula: 3n+1 then /2, not ceil division
                    { n = (3 * n + 1) / 2; }
                }
            }
            steps_walked += chain.iter().map(|&(_, s)| s as u64).sum::<u64>();
            steps_saved += cache[n as usize] as u64; // the cached tail we didn't recompute

            let mut length = cache[n as usize];
            for &(val, steps) in chain.iter().rev() {
                length += steps;
                if (val as usize) < limit {
                    cache[val as usize] = length;
                }
            }
        }

        // Compare to naive: total steps if every number walked to 1 from scratch
        let naive_total_steps: u64 = (2..limit as u64)
            .map(|n| collatz_chain_length(n) - 1) // -1 because length includes start
            .sum();

        println!("\n=== Cache Effectiveness Analysis (limit = {}) ===\n", limit);
        println!("Starting numbers (2..{}):     {}", limit, limit - 2);
        println!("  Evens skipped (< half):     {}", evens_skipped);
        println!("  Already cached (backfill):  {}", already_cached);
        println!("  Chains computed:            {}", chains_computed);
        println!();
        println!("Steps walked (memoized):      {}", steps_walked);
        println!("Steps saved (cache hits):     {}", steps_saved);
        println!("Naive total steps:            {}", naive_total_steps);
        println!();
        let total_memoized = steps_walked + steps_saved;
        println!(
            "Memoized total (walk+cache):  {} (sanity check)",
            total_memoized
        );
        println!(
            "Reduction vs naive:           {:.1}% fewer steps ({} → {})",
            (1.0 - steps_walked as f64 / naive_total_steps as f64) * 100.0,
            naive_total_steps,
            steps_walked
        );
        println!(
            "Cache hit ratio:              {:.1}% of total steps came from cache",
            steps_saved as f64 / total_memoized as f64 * 100.0
        );
    }
}
