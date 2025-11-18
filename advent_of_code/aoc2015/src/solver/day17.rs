use anyhow::Result;

/// AoC 2015 Day 17: No Such Thing as Too Much
///
/// Part 1: How many different combinations of containers can exactly fit all 150 liters of eggnog?
/// Part 2: How many ways can you fill 150 liters using the minimum number of containers?
///
/// ## Problem Overview
///
/// This is a **subset sum problem** - finding all combinations of container capacities
/// that sum to exactly the target volume (150 liters for the actual puzzle).
///
/// ## Example
///
/// Given containers `[20, 15, 10, 5, 5]` and target `25`:
/// - Valid combinations: `[15, 10]`, `[20, 5]`, `[20, 5]`, `[15, 5, 5]`
/// - Total: 4 combinations
///
/// ## Algorithm
///
/// Uses **recursive backtracking** with an include/exclude decision tree:
/// - At each step: try including current container OR excluding it
/// - Base cases: found exact match (remaining = 0) or exhausted options
/// - Time complexity: O(2^n) where n = number of containers
///
///   Count all combinations of containers that sum to exactly `target`.
///
///   This is the main entry point for Part 1. It delegates to the recursive helper
///   to perform the actual counting.
///
///   # Arguments
///
///   * `containers` - Slice of container capacities (in liters)
///   * `target` - Target volume to achieve (150 liters for the puzzle)
///
///   # Returns
///
///   The number of distinct combinations that sum to exactly `target`
///
///   # Example
///
///   ```
///   use aoc2015::solver::day17::count_combinations;
///   let containers = vec![20, 15, 10, 5, 5];
///   let count = count_combinations(&containers, 25);
///   assert_eq!(count, 4); // Four ways to make 25 liters
///   ```
pub fn count_combinations(containers: &[usize], target: usize) -> usize {
    count_recursive(containers, 0, target)
}

/// Recursive helper function that counts combinations starting from a specific index.
///
/// This implements the core backtracking algorithm using the "include/exclude" pattern:
/// - **Include branch**: Try using the current container (if it fits)
/// - **Exclude branch**: Skip the current container and move to the next
///
/// # Arguments
///
/// * `containers` - Slice of all available container capacities
/// * `index` - Current position in the containers array (which container to consider)
/// * `remaining` - How much volume still needs to be filled
///
/// # Returns
///
/// The number of valid combinations that can be made starting from `index`
///
/// # Execution Flow
///
/// ```text
/// Example: containers = [20, 15, 10], target = 25, starting from index 0
///
///                     count_recursive([20,15,10], 0, 25)
///                    /                                    \
///          Include 20                                  Exclude 20
///    count_recursive(..., 1, 5)                  count_recursive(..., 1, 25)
///         /              \                            /                  \
///   Include 15       Exclude 15               Include 15            Exclude 15
///   (5-15<0)       count_rec(...,2,5)      count_rec(...,2,10)   count_rec(...,2,25)
///   return 0         /        \               /         \             ...
///                Inc 10    Exc 10         Inc 10     Exc 10
///               (invalid)  (return 0)   ✓ MATCH!   (return 0)
///                                       return 1
/// ```
///
/// # Base Cases
///
/// - `remaining == 0`: Found a valid combination, return 1
/// - `index >= containers.len()`: No more containers to try, return 0
/// - `containers[index] > remaining`: Current container too large, skip include branch
fn count_recursive(containers: &[usize], index: usize, remaining: usize) -> usize {
    // Base cases
    if remaining == 0 {
        return 1; // Found valid combination
    }
    if index >= containers.len() {
        return 0; // No more containers
    }

    // Recursive cases:
    // 1. Include current container (if it doesn't exceed remaining)
    let include = if containers[index] <= remaining {
        count_recursive(containers, index + 1, remaining - containers[index])
    } else {
        0
    };

    // 2. Exclude current container
    let exclude = count_recursive(containers, index + 1, remaining);

    include + exclude
}

/// Find and collect all valid combinations of containers that sum to the target.
///
/// Unlike `count_combinations`, this function actually builds and stores all valid
/// combinations. This is necessary for Part 2, where we need to analyze the size
/// of each combination to find the minimum.
///
/// # Arguments
///
/// * `containers` - Slice of container capacities
/// * `target` - Target volume to achieve
///
/// # Returns
///
/// A vector of vectors, where each inner vector represents one valid combination
/// of container capacities.
///
/// # Example
///
/// ```
/// use aoc2015::solver::day17::find_all_combinations;
/// let containers = vec![20, 15, 10, 5, 5];
/// let combinations = find_all_combinations(&containers, 25);
/// // Returns: [[15, 10], [20, 5], [20, 5], [15, 5, 5]]
/// assert_eq!(combinations.len(), 4);
/// ```
///
/// # Memory Note
///
/// This function requires O(k * m) memory where:
/// - k = number of valid combinations
/// - m = average containers per combination
///
/// For the actual puzzle input, this is still reasonable (~hundreds of combinations).
pub fn find_all_combinations(containers: &[usize], target: usize) -> Vec<Vec<usize>> {
    let mut results = Vec::new();
    let mut current = Vec::new();
    backtrack(containers, 0, target, &mut current, &mut results);
    results
}

/// Backtracking helper that builds and collects all valid combinations.
///
/// This function implements the classic backtracking pattern:
/// 1. Make a choice (include current container)
/// 2. Recursively explore that choice
/// 3. Undo the choice (pop from current)
/// 4. Make alternative choice (exclude current container)
/// 5. Recursively explore the alternative
///
/// # Arguments
///
/// * `containers` - Slice of all available containers
/// * `index` - Current position in containers array
/// * `remaining` - Volume still needed to reach target
/// * `current` - Mutable vector tracking the current combination being built
/// * `results` - Mutable vector where valid combinations are stored
///
/// # Execution Pattern
///
/// ```text
/// backtrack([20, 15, 10], 0, 25, [], results)
/// │
/// ├─ Try including 20: current = [20]
/// │  └─ backtrack([...], 1, 5, [20], results)
/// │     ├─ Try including 15: current = [20, 15] (invalid, 5-15 < 0)
/// │     └─ Try excluding 15: current = [20]
/// │        └─ backtrack([...], 2, 5, [20], results)
/// │           ├─ Try including 10: current = [20, 10] (invalid)
/// │           └─ Try excluding 10: return (no more containers)
/// │  
/// └─ Try excluding 20: current = []
///    └─ backtrack([...], 1, 25, [], results)
///       ├─ Try including 15: current = [15]
///       │  └─ backtrack([...], 2, 10, [15], results)
///       │     ├─ Try including 10: current = [15, 10]
///       │     │  └─ remaining = 0 → results.push([15, 10]) ✓
///       │     └─ Try excluding 10: return
///       └─ Try excluding 15: ...continues...
/// ```
///
/// # Side Effects
///
/// - Modifies `current` during execution (but restores it via pop)
/// - Appends valid combinations to `results`
fn backtrack(
    containers: &[usize],
    index: usize,
    remaining: usize,
    current: &mut Vec<usize>,
    results: &mut Vec<Vec<usize>>,
) {
    // Found valid combination
    if remaining == 0 {
        results.push(current.clone());
        return;
    }

    // No more containers
    if index >= containers.len() {
        return;
    }

    // Try including current container
    if containers[index] <= remaining {
        current.push(containers[index]);
        backtrack(
            containers,
            index + 1,
            remaining - containers[index],
            current,
            results,
        );
        current.pop();
    }

    // Try excluding current container
    backtrack(containers, index + 1, remaining, current, results);
}

/// Count combinations using the minimum number of containers (Part 2).
///
/// This function solves Part 2 by:
/// 1. Finding ALL valid combinations (using `find_all_combinations`)
/// 2. Determining the minimum number of containers used in any combination
/// 3. Counting how many combinations use exactly that minimum number
///
/// # Arguments
///
/// * `containers` - Slice of container capacities
/// * `target` - Target volume to achieve (150 liters for the puzzle)
///
/// # Returns
///
/// The count of combinations that use the minimum number of containers
///
/// # Example
///
/// ```
/// use aoc2015::solver::day17::count_minimum_combinations;
/// // Given containers = [20, 15, 10, 5, 5] and target = 25
/// // All combinations and their sizes:
/// // [15, 10]     → 2 containers
/// // [20, 5]      → 2 containers
/// // [20, 5]      → 2 containers (different 5)
/// // [15, 5, 5]   → 3 containers
/// //
/// // Minimum = 2, count with minimum = 3
/// let containers = vec![20, 15, 10, 5, 5];
/// let count = count_minimum_combinations(&containers, 25);
/// assert_eq!(count, 3);
/// ```
///
/// # Algorithm Steps
///
/// 1. Generate all valid combinations
/// 2. Find `min_count = minimum length among all combinations`
/// 3. Filter and count combinations where `length == min_count`
pub fn count_minimum_combinations(containers: &[usize], target: usize) -> usize {
    let all_combinations = find_all_combinations(containers, target);

    if all_combinations.is_empty() {
        return 0;
    }

    // Find minimum container count
    let min_count = all_combinations
        .iter()
        .map(|combo| combo.len())
        .min()
        .unwrap();

    // Count how many combinations use minimum containers
    all_combinations
        .iter()
        .filter(|combo| combo.len() == min_count)
        .count()
}

/// Solve Part 1: Count all combinations that fit exactly 150 liters.
///
/// Parses the input to extract container capacities, then uses the
/// recursive counting algorithm to find how many distinct combinations
/// sum to exactly 150 liters.
///
/// # Arguments
///
/// * `input` - Multi-line string where each line contains one container capacity
///
/// # Returns
///
/// `Result<String>` containing the count as a string, or an error
///
/// # Input Format
///
/// ```text
/// 20
/// 15
/// 10
/// 5
/// 5
/// ```
///
/// # Example
///
/// ```
/// use aoc2015::solver::day17::solve_part1;
/// let input = "20\n15\n10\n5\n5";
/// let result = solve_part1(input).unwrap();
/// // Result depends on target (150 for actual puzzle)
/// ```
pub fn solve_part1(input: &str) -> Result<String> {
    let containers: Vec<usize> = input
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect();

    let count = count_combinations(&containers, 150);
    Ok(count.to_string())
}

/// Solve Part 2: Count combinations using the minimum number of containers.
///
/// Parses the input, finds all valid combinations, determines the minimum
/// container count among them, and returns how many combinations achieve
/// that minimum.
///
/// # Arguments
///
/// * `input` - Multi-line string where each line contains one container capacity
///
/// # Returns
///
/// `Result<String>` containing the count of minimum-container combinations
///
/// # Input Format
///
/// Same as Part 1: one container capacity per line
///
/// # Example
///
/// ```
/// use aoc2015::solver::day17::solve_part2;
/// // If minimum is 2 containers and 3 combinations use 2 containers:
/// let input = "20\n15\n10\n5\n5";
/// let result = solve_part2(input).unwrap();
/// // Returns "3" (for target 25, but actual puzzle uses 150)
/// ```
///
/// # Algorithm
///
/// 1. Parse container capacities from input
/// 2. Find all valid combinations that sum to 150
/// 3. Determine minimum container count across all combinations
/// 4. Count how many combinations use exactly that minimum
pub fn solve_part2(input: &str) -> Result<String> {
    let containers: Vec<usize> = input
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect();

    let count = count_minimum_combinations(&containers, 150);
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
20
15
10
5
5";

    #[test]
    fn test_example_part1() {
        let containers: Vec<usize> = EXAMPLE_INPUT
            .lines()
            .filter_map(|line| line.parse().ok())
            .collect();

        let count = count_combinations(&containers, 25);
        assert_eq!(count, 4, "Should find 4 combinations for 25 liters");
    }

    #[test]
    fn test_example_combinations() {
        let containers = vec![20, 15, 10, 5, 5];
        let combinations = find_all_combinations(&containers, 25);

        // Verify we get 4 combinations
        assert_eq!(combinations.len(), 4);

        // Verify each combination sums to 25
        for combo in &combinations {
            let sum: usize = combo.iter().sum();
            assert_eq!(sum, 25, "Each combination should sum to 25");
        }

        // Expected combinations (in any order):
        // [15, 10]
        // [20, 5]
        // [20, 5]
        // [15, 5, 5]
        let mut sizes: Vec<usize> = combinations.iter().map(|c| c.len()).collect();
        sizes.sort_unstable();

        // Should have combinations of size: 2, 2, 2, 3
        assert_eq!(sizes, vec![2, 2, 2, 3]);
    }

    #[test]
    fn test_example_part2() {
        let containers: Vec<usize> = EXAMPLE_INPUT
            .lines()
            .filter_map(|line| line.parse().ok())
            .collect();

        let count = count_minimum_combinations(&containers, 25);
        // Minimum is 2 containers, and there are 3 such combinations
        assert_eq!(
            count, 3,
            "Should find 3 combinations using minimum containers"
        );
    }

    #[test]
    fn test_empty_containers() {
        let containers: Vec<usize> = vec![];
        let count = count_combinations(&containers, 25);
        assert_eq!(count, 0, "No containers = no combinations");
    }

    #[test]
    fn test_exact_single_container() {
        let containers = vec![25];
        let count = count_combinations(&containers, 25);
        assert_eq!(count, 1, "Single exact-fit container");
    }

    #[test]
    fn test_no_valid_combinations() {
        let containers = vec![10, 20, 30];
        let count = count_combinations(&containers, 15);
        assert_eq!(count, 0, "Cannot make 15 from [10, 20, 30]");
    }

    #[test]
    fn test_recursive_count_matches_backtrack() {
        let containers = vec![20, 15, 10, 5, 5];
        let count1 = count_combinations(&containers, 25);
        let combinations = find_all_combinations(&containers, 25);
        assert_eq!(count1, combinations.len(), "Both methods should agree");
    }

    #[test]
    fn test_solve_part1() {
        let result = solve_part1(EXAMPLE_INPUT).unwrap();
        // Note: This tests with target 150, not 25, so result will be "0"
        // since the example containers can't make 150
        assert_eq!(result, "0");
    }

    #[test]
    fn test_solve_part2() {
        let result = solve_part2(EXAMPLE_INPUT).unwrap();
        // Same as above, testing with target 150
        assert_eq!(result, "0");
    }

    #[test]
    fn test_parse_input() {
        let input = "20\n15\n10\n5\n5\n";
        let containers: Vec<usize> = input
            .lines()
            .filter_map(|line| line.trim().parse().ok())
            .collect();

        assert_eq!(containers.len(), 5);
        assert_eq!(containers, vec![20, 15, 10, 5, 5]);
    }

    #[test]
    fn test_parse_input_with_whitespace() {
        let input = "  20  \n\n  15  \n  10  \n";
        let containers: Vec<usize> = input
            .lines()
            .filter_map(|line| line.trim().parse().ok())
            .collect();

        assert_eq!(containers.len(), 3);
        assert_eq!(containers, vec![20, 15, 10]);
    }
}
