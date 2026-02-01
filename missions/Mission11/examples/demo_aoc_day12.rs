//! AoC 2023 Day 12 pattern demonstration
//!
//! Shows how the Hot Springs problem uses generic memoization
//! with 3D state tuples (pos, group_idx, current_run).

use std::collections::HashMap;

type Memo = HashMap<(usize, usize, usize), usize>;

/// Simplified version of AoC 2023 Day 12 memoization pattern
///
/// # Requirements Satisfied
/// - REQ-1: Generic HashMap with tuple keys
/// - REQ-2: Top-down DP pattern
/// - REQ-5: Composite state design (3D tuple)
/// - REQ-6: Cache statistics demonstrate overlapping subproblems
fn count_arrangements(
    springs: &[u8],
    groups: &[usize],
    pos: usize,
    group_idx: usize,
    current_run: usize,
    memo: &mut Memo,
) -> usize {
    // Base case: reached end of springs
    if pos == springs.len() {
        if group_idx == groups.len() && current_run == 0 {
            return 1; // Valid arrangement
        }
        if group_idx == groups.len() - 1 && current_run == groups[group_idx] {
            return 1; // Last group completed
        }
        return 0; // Invalid
    }

    // Memoization check (REQ-2 pattern)
    let key = (pos, group_idx, current_run);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let mut count = 0;
    let ch = springs[pos];

    // Try placing '.' (operational spring)
    if ch == b'.' || ch == b'?' {
        if current_run == 0 {
            count += count_arrangements(springs, groups, pos + 1, group_idx, 0, memo);
        } else if group_idx < groups.len() && current_run == groups[group_idx] {
            count += count_arrangements(springs, groups, pos + 1, group_idx + 1, 0, memo);
        }
    }

    // Try placing '#' (damaged spring)
    if (ch == b'#' || ch == b'?') && group_idx < groups.len() && current_run < groups[group_idx] {
        count += count_arrangements(springs, groups, pos + 1, group_idx, current_run + 1, memo);
    }

    // Cache and return (REQ-2 pattern)
    memo.insert(key, count);
    count
}

fn main() {
    println!("=== AoC 2023 Day 12 Memoization Pattern ===\n");

    // Example input
    let springs = "???.###".as_bytes();
    let groups = vec![1, 1, 3];

    println!(
        "Input: {} {:?}",
        std::str::from_utf8(springs).unwrap(),
        groups
    );

    let mut memo = HashMap::new();
    let result = count_arrangements(springs, &groups, 0, 0, 0, &mut memo);

    println!("\nResults:");
    println!("  Arrangements: {}", result);
    println!("  Cache size:   {} states", memo.len());
    println!("  State space:  (pos × groups × max_run)");

    // Show cache effectiveness with more complex example
    println!("\n--- Complex Example ---");
    let springs2 = ".??..??...?##.".as_bytes();
    let groups2 = vec![1, 1, 3];

    println!(
        "Input: {} {:?}",
        std::str::from_utf8(springs2).unwrap(),
        groups2
    );

    let mut memo2 = HashMap::new();
    let result2 = count_arrangements(springs2, &groups2, 0, 0, 0, &mut memo2);

    println!("\nResults:");
    println!("  Arrangements: {}", result2);
    println!("  Cache size:   {} states", memo2.len());

    println!("\n=== Key Pattern Elements ===");
    println!("✓ 3D State Tuple: (position, group_index, current_run)");
    println!("✓ Base Case Check: Early return for completion/failure");
    println!("✓ Cache Lookup: Before expensive recursion");
    println!("✓ Branching Logic: Try multiple transitions, sum results");
    println!("✓ Result Caching: Store before returning");
    println!("\nThis pattern appears in many AoC DP problems!");
}
