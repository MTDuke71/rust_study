/// AoC 2015 Day 24: It Hangs in the Balance
///
/// Part 1: Find the quantum entanglement of the smallest package group that allows
///         the remaining packages to be split into two groups of equal weight
///
/// ## Problem Overview
///
/// Santa needs to split packages into 3 groups of equal weight for balanced sleigh.
/// First group (passenger compartment) should be as small as possible, with minimal
/// quantum entanglement (product of weights) as tie-breaker.
///
/// ## Solution Approach
///
/// 1. Parse package weights from input
/// 2. Calculate target weight = total_weight / 3
/// 3. Find all subsets summing to target, tracking minimal size
/// 4. Among minimal size subsets, find minimal quantum entanglement
/// 5. Validate remaining packages can form two more groups
///
/// How It Works
/// - Compute target per group = total / num_groups (3 or 4)
/// - Sort weights descending to improve pruning
/// - Enumerate exact-size subsets that sum to target (`find_subsets`, by indices)
/// - For each candidate, remove chosen indices and verify remaining can form k-1 groups
///   via `can_partition_remaining` (base: `remaining_groups == 1` → true)
/// - Stop at the first size that yields any valid partition; among those, pick minimal QE
///
/// See Also
/// - Examples: `examples/day24_execution_trace.rs`, `examples/day24_code_walkthrough.rs`
/// - Problem: `Problem_Statements/day24.md` (Implementation Notes + commands)
use anyhow::Result;

/// Parse package weights from input string
pub fn parse_weights(input: &str) -> Result<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .parse::<u64>()
                .map_err(|_| anyhow::anyhow!("Invalid weight: {}", line))
        })
        .collect()
}

/// Calculate quantum entanglement (product) of a group
pub fn quantum_entanglement(group: &[u64]) -> u64 {
    group.iter().product()
}

/// Find all subsets of given size that sum to target weight
/// Returns vectors of indices into `weights` for each valid subset
pub fn find_subsets(weights: &[u64], target: u64, exact_size: usize) -> Vec<Vec<usize>> {
    let mut results = Vec::new();
    let mut current = Vec::with_capacity(exact_size);

    fn backtrack(
        weights: &[u64],
        target: u64,
        start: usize,
        current: &mut Vec<usize>,
        current_sum: u64,
        exact_size: usize,
        results: &mut Vec<Vec<usize>>,
    ) {
        if current_sum == target && current.len() == exact_size {
            results.push(current.clone());
            return;
        }

        if current_sum >= target || current.len() >= exact_size {
            return;
        }

        for i in start..weights.len() {
            let w = weights[i];
            if current_sum + w > target {
                continue;
            }
            current.push(i);
            backtrack(
                weights,
                target,
                i + 1,
                current,
                current_sum + w,
                exact_size,
                results,
            );
            current.pop();
        }
    }

    backtrack(
        weights,
        target,
        0,
        &mut current,
        0,
        exact_size,
        &mut results,
    );
    results
}

/// Check if packages can be partitioned into remaining_groups of equal weight
/// For Part 1: remaining_groups = 2 (after first group)
/// For Part 2: remaining_groups = 3 (after first group)
pub fn can_partition_remaining(packages: &[u64], target: u64, remaining_groups: u64) -> bool {
    if remaining_groups == 1 {
        // If we can form all but the last group, the remainder must sum to target
        // given the initial total check, so this is a success.
        return true;
    }
    if packages.is_empty() {
        return false;
    }

    let mut nums = packages.to_vec();
    nums.sort_by(|a, b| b.cmp(a));

    fn dfs(
        nums: &[u64],
        target: u64,
        start: usize,
        sum: u64,
        selected: &mut Vec<bool>,
        remaining_groups: u64,
    ) -> bool {
        if sum == target {
            // Build remaining set and try to partition further
            let remaining: Vec<u64> = nums
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| if !selected[i] { Some(v) } else { None })
                .collect();
            return can_partition_remaining(&remaining, target, remaining_groups - 1);
        }
        if sum > target {
            return false;
        }
        for i in start..nums.len() {
            if selected[i] {
                continue;
            }
            let w = nums[i];
            if sum + w > target {
                continue;
            }
            selected[i] = true;
            if dfs(nums, target, i + 1, sum + w, selected, remaining_groups) {
                return true;
            }
            selected[i] = false;
        }
        false
    }

    let mut selected = vec![false; nums.len()];
    dfs(&nums, target, 0, 0, &mut selected, remaining_groups)
}

/// Check if packages can be partitioned into two groups of equal weight (legacy function)
pub fn can_partition_into_two_groups(packages: &[u64], target: u64) -> bool {
    can_partition_remaining(packages, target, 2)
}

/// Solve Part 1: Find optimal first group configuration for 3 groups
pub fn solve_part1(input: &str) -> Result<String> {
    solve_balanced_partition(input, 3)
}

/// Solve Part 2: Find optimal first group configuration for 4 groups
pub fn solve_part2(input: &str) -> Result<String> {
    solve_balanced_partition(input, 4)
}

/// Core balanced partitioning algorithm
/// num_groups: 3 for Part 1, 4 for Part 2
fn solve_balanced_partition(input: &str, num_groups: u64) -> Result<String> {
    let weights = parse_weights(input)?;
    let total: u64 = weights.iter().sum();

    if !total.is_multiple_of(num_groups) {
        return Err(anyhow::anyhow!(
            "Total weight {} not divisible by {}",
            total,
            num_groups
        ));
    }

    let target = total / num_groups;

    // Sort weights descending for better subset finding
    let mut sorted_weights = weights.clone();
    sorted_weights.sort_by(|a, b| b.cmp(a));

    let mut best_qe = None as Option<u64>;

    for size in 1..=sorted_weights.len() {
        let subsets = find_subsets(&sorted_weights, target, size);
        if subsets.is_empty() {
            continue;
        }

        let mut found_for_size = false;
        for idxs in subsets {
            // Build the chosen group and remaining pool
            let mut chosen = Vec::with_capacity(size);
            let mut used = vec![false; sorted_weights.len()];
            for i in idxs {
                chosen.push(sorted_weights[i]);
                used[i] = true;
            }
            let remaining: Vec<u64> = sorted_weights
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| if !used[i] { Some(v) } else { None })
                .collect();

            if can_partition_remaining(&remaining, target, num_groups - 1) {
                let qe = quantum_entanglement(&chosen);
                match best_qe {
                    None => {
                        best_qe = Some(qe);
                        found_for_size = true;
                    }
                    Some(current) => {
                        if qe < current {
                            best_qe = Some(qe);
                        }
                        found_for_size = true;
                    }
                }
            }
        }

        if found_for_size {
            // We've found solutions with minimal group size; no need to consider larger sizes
            break;
        }
    }

    match best_qe {
        Some(qe) => Ok(qe.to_string()),
        None => Err(anyhow::anyhow!("No valid partition found")),
    }
}

// Remove the duplicate solve_part2 function (it's now defined at the top)

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1\n2\n3\n4\n5\n7\n8\n9\n10\n11";

    #[test]
    fn test_parse_weights() {
        let input = "1\n2\n3";
        let weights = parse_weights(input).unwrap();
        assert_eq!(weights, vec![1, 2, 3]);
    }

    #[test]
    fn test_quantum_entanglement() {
        assert_eq!(quantum_entanglement(&[11, 9]), 99);
        assert_eq!(quantum_entanglement(&[10, 9, 1]), 90);
    }

    #[test]
    fn test_can_partition_into_two_groups() {
        // Simple test cases that should work
        let packages = vec![10, 10];
        assert!(can_partition_into_two_groups(&packages, 10));

        // Single package can't make two groups
        let packages = vec![1];
        assert!(!can_partition_into_two_groups(&packages, 10));
    }

    #[test]
    fn test_solve_part1_example() {
        let result = solve_part1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "99");
    }

    #[test]
    fn test_solve_part2_example() {
        let result = solve_part2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "44"); // From Part 2 example: 11 × 4 = 44
    }
}
