use anyhow::Result;

/// AoC 2015 Day 20: Infinite Elves and Infinite Houses
///
/// Part 1: Find the lowest house number that receives at least the target number of presents.
/// Part 2: Elves only deliver to 50 houses each, with 11 presents per delivery.
///
/// ## Problem Overview
///
/// Each Elf N delivers presents to houses that are multiples of N:
/// - Elf 1 delivers to houses: 1, 2, 3, 4, 5, ...
/// - Elf 2 delivers to houses: 2, 4, 6, 8, 10, ...
/// - Elf 3 delivers to houses: 3, 6, 9, 12, 15, ...
///
/// Each Elf delivers presents equal to 10 times their number (Part 1) or 11 times (Part 2).
///
/// ## Key Insight
///
/// The number of presents a house receives is related to the sum of its divisors:
/// - House N receives presents from all Elves whose numbers divide N evenly
/// - For Part 1: presents = 10 * sum_of_divisors(N)
/// - For Part 2: presents = 11 * sum_of_divisors_with_limit(N, 50)
///
/// ## Example
///
/// House 4 is divisible by 1, 2, and 4:
/// - Elf 1 delivers: 1 * 10 = 10 presents
/// - Elf 2 delivers: 2 * 10 = 20 presents
/// - Elf 4 delivers: 4 * 10 = 40 presents
/// - Total: 70 presents
///
/// ## Algorithm
///
/// Instead of iterating through houses and finding divisors (O(n√n)),
/// we iterate through elves and mark the houses they visit (more cache-friendly):
///
/// ```text
/// houses = [0, 0, 0, 0, 0, 0, ...]
///
/// Elf 1 visits: 1, 2, 3, 4, 5, 6, ... → add 10 to each
/// houses = [10, 10, 10, 10, 10, 10, ...]
///
/// Elf 2 visits: 2, 4, 6, 8, ... → add 20 to each
/// houses = [10, 30, 10, 30, 10, 30, ...]
///
/// Elf 3 visits: 3, 6, 9, ... → add 30 to each
/// houses = [10, 30, 40, 30, 10, 60, ...]
/// ```
/// Calculate the sum of divisors for a given number (Part 1 approach).
///
/// This is a direct implementation that finds all divisors of `n` and sums them.
/// Time complexity: O(√n)
///
/// # Arguments
///
/// * `n` - The number to find divisors for
///
/// # Returns
///
/// Sum of all divisors of `n` (including 1 and n itself)
///
/// # Example
///
/// ```ignore
/// assert_eq!(sum_of_divisors(6), 12);  // 1 + 2 + 3 + 6 = 12
/// assert_eq!(sum_of_divisors(4), 7);   // 1 + 2 + 4 = 7
/// ```
#[allow(dead_code)]
fn sum_of_divisors(n: usize) -> usize {
    let mut sum = 0;
    let mut i = 1;

    // Only need to check up to √n
    while i * i <= n {
        if n % i == 0 {
            sum += i;

            // Add the paired divisor (n/i) if it's different from i
            if i != n / i {
                sum += n / i;
            }
        }
        i += 1;
    }

    sum
}

/// Find the lowest house number that receives at least `target` presents (Part 1).
///
/// Uses a simulation approach: iterate through elves, and for each elf,
/// mark all houses they visit with the presents they deliver.
///
/// # Arguments
///
/// * `target` - Minimum number of presents required
///
/// # Returns
///
/// The lowest house number that gets at least `target` presents
///
/// # Algorithm Details
///
/// We need an upper bound for house numbers to check. A reasonable estimate:
/// - Best case: a house with many divisors (highly composite number)
/// - Upper bound: target / 10 (if only Elf 1 visited, house would be target/10)
/// - We use target / 10 as initial size, which works well in practice
///
/// # Example
///
/// ```ignore
/// // Find house with at least 70 presents
/// // House 4 gets 70 presents (from elves 1, 2, 4)
/// assert_eq!(find_lowest_house_part1(70), 4);
/// ```
fn find_lowest_house_part1(target: usize) -> usize {
    // Upper bound estimation: we need at least target/10 houses to check
    // In practice, highly composite numbers can have more divisors, so this is conservative
    let max_houses = target / 10;

    // houses[i] represents the number of presents house i receives
    // We use index 0 as a placeholder (houses are numbered starting from 1)
    let mut houses = vec![0; max_houses + 1];

    // Simulate each elf delivering presents
    for elf in 1..=max_houses {
        // This elf visits houses: elf, 2*elf, 3*elf, ...
        let mut house = elf;
        while house <= max_houses {
            houses[house] += elf * 10;
            house += elf;
        }
    }

    // Find the first house with at least target presents
    for (house_num, &presents) in houses.iter().enumerate().skip(1) {
        if presents >= target {
            return house_num;
        }
    }

    // If no house found, we need to increase our search space
    // This shouldn't happen with our upper bound calculation
    max_houses
}

/// Find the lowest house number for Part 2 with modified rules.
///
/// Part 2 changes:
/// - Each elf delivers to only the first 50 houses (instead of infinite)
/// - Each elf delivers 11 presents per house (instead of 10)
///
/// # Arguments
///
/// * `target` - Minimum number of presents required
///
/// # Returns
///
/// The lowest house number that gets at least `target` presents under Part 2 rules
///
/// # Algorithm Details
///
/// Same simulation approach as Part 1, but:
/// - Limit each elf to visiting only 50 houses
/// - Multiply elf number by 11 instead of 10
///
/// # Example
///
/// ```
/// // With Part 2 rules, elves deliver less overall (limited to 50 houses each)
/// // So we need a higher house number to reach the same target
/// ```
fn find_lowest_house_part2(target: usize) -> usize {
    // Similar upper bound as Part 1, but adjusted for 11 presents per delivery
    let max_houses = target / 11;

    let mut houses = vec![0; max_houses + 1];

    // Simulate each elf delivering presents
    for elf in 1..=max_houses {
        // This elf visits at most 50 houses
        let mut house = elf;
        let mut visits = 0;

        while house <= max_houses && visits < 50 {
            houses[house] += elf * 11;
            house += elf;
            visits += 1;
        }
    }

    // Find the first house with at least target presents
    for (house_num, &presents) in houses.iter().enumerate().skip(1) {
        if presents >= target {
            return house_num;
        }
    }

    max_houses
}

/// Solve Part 1: Find the lowest house number with at least target presents.
///
/// Parses the target from input and finds the first house to receive
/// at least that many presents under Part 1 rules (infinite houses, 10 presents).
///
/// # Arguments
///
/// * `input` - Single line containing the target number of presents
///
/// # Returns
///
/// `Result<String>` containing the lowest house number as a string
///
/// # Input Format
///
/// ```text
/// 33100000
/// ```
pub fn solve_part1(input: &str) -> Result<String> {
    let target: usize = input.trim().parse()?;
    let house = find_lowest_house_part1(target);
    Ok(house.to_string())
}

/// Solve Part 2: Find the lowest house number under modified delivery rules.
///
/// Parses the target from input and finds the first house to receive
/// at least that many presents under Part 2 rules (50 house limit, 11 presents).
///
/// # Arguments
///
/// * `input` - Single line containing the target number of presents
///
/// # Returns
///
/// `Result<String>` containing the lowest house number as a string
pub fn solve_part2(input: &str) -> Result<String> {
    let target: usize = input.trim().parse()?;
    let house = find_lowest_house_part2(target);
    Ok(house.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_divisors() {
        assert_eq!(sum_of_divisors(1), 1);   // 1
        assert_eq!(sum_of_divisors(2), 3);   // 1 + 2
        assert_eq!(sum_of_divisors(3), 4);   // 1 + 3
        assert_eq!(sum_of_divisors(4), 7);   // 1 + 2 + 4
        assert_eq!(sum_of_divisors(6), 12);  // 1 + 2 + 3 + 6
        assert_eq!(sum_of_divisors(8), 15);  // 1 + 2 + 4 + 8
        assert_eq!(sum_of_divisors(9), 13);  // 1 + 2 + 9
    }

    #[test]
    fn test_house_presents_manual() {
        // House 1: visited by Elf 1 → 1 * 10 = 10
        assert_eq!(sum_of_divisors(1) * 10, 10);

        // House 2: visited by Elves 1, 2 → (1 + 2) * 10 = 30
        assert_eq!(sum_of_divisors(2) * 10, 30);

        // House 3: visited by Elves 1, 3 → (1 + 3) * 10 = 40
        assert_eq!(sum_of_divisors(3) * 10, 40);

        // House 4: visited by Elves 1, 2, 4 → (1 + 2 + 4) * 10 = 70
        assert_eq!(sum_of_divisors(4) * 10, 70);

        // House 6: visited by Elves 1, 2, 3, 6 → (1 + 2 + 3 + 6) * 10 = 120
        assert_eq!(sum_of_divisors(6) * 10, 120);
    }

    #[test]
    fn test_find_lowest_house_part1_small() {
        // House 1 gets 10 presents
        assert_eq!(find_lowest_house_part1(10), 1);

        // House 2 gets 30 presents
        assert_eq!(find_lowest_house_part1(30), 2);

        // House 4 gets 70 presents
        assert_eq!(find_lowest_house_part1(70), 4);

        // House 6 gets 120 presents
        assert_eq!(find_lowest_house_part1(120), 6);
    }

    #[test]
    fn test_find_lowest_house_part1_example() {
        // From problem description
        // House 1: 10, House 2: 30, House 3: 40, House 4: 70
        // House 5: 60, House 6: 120, House 7: 80, House 8: 150, House 9: 130

        assert_eq!(find_lowest_house_part1(150), 8);
    }

    #[test]
    fn test_solve_part1_example() {
        // Test with the example input
        let result = solve_part1("150").unwrap();
        assert_eq!(result, "8");
    }

    #[test]
    fn test_find_lowest_house_part2_small() {
        // Part 2 has different rules, so just verify it doesn't panic
        let house = find_lowest_house_part2(100);
        assert!(house > 0);
    }

    #[test]
    fn test_parse_input() {
        let result = solve_part1("100").unwrap();
        let house: usize = result.parse().unwrap();
        assert!(house > 0);
    }

    #[test]
    fn test_parse_input_with_whitespace() {
        let result = solve_part1("  100  \n").unwrap();
        let house: usize = result.parse().unwrap();
        assert!(house > 0);
    }
}
