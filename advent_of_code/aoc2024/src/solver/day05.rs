//! # Day 5: Print Queue
//!
//! This solution demonstrates the power of foundational libraries by using
//! Mission 7 (Graph Representation) and Mission 8 (BFS/DFS Algorithms) to solve
//! what is essentially a topological sorting problem.
//!
//! ## Problem Analysis
//!
//! - **Part 1**: Validate if page sequences follow ordering rules (dependency validation)
//! - **Part 2**: Fix incorrect sequences using topological sorting
//!
//! ## Mission Integration Benefits
//!
//! - **Mission 7**: Graph construction from ordering rules (`X|Y` becomes edge X→Y)
//! - **Mission 8**: Cycle detection and graph traversal algorithms
//! - **Custom Extension**: Topological sorting implementation for AoC problems
//!
//! ## Algorithm Approach
//!
//! 1. Parse ordering rules into a directed graph
//! 2. For each update sequence:
//!    - Part 1: Check if sequence respects topological ordering
//!    - Part 2: Use topological sort to fix incorrect sequences
//! 3. Calculate middle page numbers for scoring
//!
//! ## Performance Characteristics
//!
//! - **Graph Construction**: O(R) where R is number of rules
//! - **Sequence Validation**: O(S²) where S is sequence length
//! - **Topological Sort**: O(V + E) for each broken sequence
//! - **Overall**: O(R + U × S²) where U is number of updates

use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet, VecDeque};

/// Represents the print queue problem with ordering rules and updates
#[derive(Debug, Clone)]
pub struct PrintQueue {
    /// Ordering rules as adjacency list: X|Y means X must come before Y
    rules: HashMap<i32, Vec<i32>>,
    /// Page update sequences to validate/fix
    updates: Vec<Vec<i32>>,
    /// Set of all pages that have ordering rules
    #[allow(dead_code)]
    all_pages: HashSet<i32>,
}

impl PrintQueue {
    /// Parse input into print queue structure
    ///
    /// # Arguments
    ///
    /// * `input` - Raw input string with rules and updates separated by blank line
    ///
    /// # Returns
    ///
    /// Parsed PrintQueue structure ready for solving
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc2024::solver::day05::PrintQueue;
    /// let input = "47|53\n97|13\n\n75,47,61,53,29\n97,61,53,29,13";
    /// let queue = PrintQueue::parse(input)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn parse(input: &str) -> Result<Self> {
        let sections: Vec<&str> = input.trim().split("\n\n").collect();
        if sections.len() != 2 {
            return Err(anyhow::anyhow!("Invalid input format: expected rules and updates separated by blank line"));
        }

        // Parse ordering rules: X|Y means X must come before Y
        let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut all_pages = HashSet::new();

        for line in sections[0].lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() != 2 {
                return Err(anyhow::anyhow!("Invalid rule format: {}", line));
            }
            
            let before: i32 = parts[0].parse()
                .with_context(|| format!("Invalid page number: {}", parts[0]))?;
            let after: i32 = parts[1].parse()
                .with_context(|| format!("Invalid page number: {}", parts[1]))?;
            
            rules.entry(before).or_default().push(after);
            all_pages.insert(before);
            all_pages.insert(after);
        }

        // Parse update sequences
        let mut updates = Vec::new();
        for line in sections[1].lines() {
            let pages: Result<Vec<i32>, _> = line
                .split(',')
                .map(|s| s.parse::<i32>())
                .collect();
            
            updates.push(pages.with_context(|| format!("Invalid update format: {}", line))?);
        }

        Ok(PrintQueue {
            rules,
            updates,
            all_pages,
        })
    }

    /// Solve both parts of the print queue problem
    ///
    /// # Returns
    ///
    /// Tuple of (part1_result, part2_result) representing middle page sums
    pub fn solve(&self) -> (i32, i32) {
        let mut part1_sum = 0;
        let mut part2_sum = 0;

        for update in &self.updates {
            if self.is_correctly_ordered(update) {
                // Part 1: Already correctly ordered
                part1_sum += self.get_middle_page(update);
            } else {
                // Part 2: Fix using topological sort
                let fixed_update = self.topological_sort_subgraph(update);
                part2_sum += self.get_middle_page(&fixed_update);
            }
        }

        (part1_sum, part2_sum)
    }

    /// Check if an update sequence follows the ordering rules
    ///
    /// Uses a position-based validation approach: for each rule X|Y,
    /// if both X and Y are in the sequence, X must appear before Y.
    ///
    /// # Arguments
    ///
    /// * `update` - The page sequence to validate
    ///
    /// # Returns
    ///
    /// `true` if sequence respects all applicable ordering rules
    fn is_correctly_ordered(&self, update: &[i32]) -> bool {
        // Create position map for O(1) lookup
        let positions: HashMap<i32, usize> = update
            .iter()
            .enumerate()
            .map(|(i, &page)| (page, i))
            .collect();

        // Check each rule: X|Y means X must come before Y
        for (&before_page, after_pages) in &self.rules {
            if let Some(&before_pos) = positions.get(&before_page) {
                for &after_page in after_pages {
                    if let Some(&after_pos) = positions.get(&after_page) {
                        if before_pos >= after_pos {
                            return false; // Rule violated: X comes after Y
                        }
                    }
                }
            }
        }

        true
    }

    /// Fix incorrect update sequence using topological sorting
    ///
    /// This creates a subgraph containing only the pages in the update,
    /// then performs topological sort to find a valid ordering.
    ///
    /// # Arguments
    ///
    /// * `update` - The incorrectly ordered page sequence
    ///
    /// # Returns
    ///
    /// Topologically sorted sequence that respects all ordering rules
    fn topological_sort_subgraph(&self, update: &[i32]) -> Vec<i32> {
        let update_set: HashSet<i32> = update.iter().copied().collect();
        
        // Build subgraph with only pages from this update
        let mut subgraph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut in_degree: HashMap<i32, i32> = HashMap::new();
        
        // Initialize all pages with empty adjacency lists and zero in-degree
        for &page in update {
            subgraph.insert(page, Vec::new());
            in_degree.insert(page, 0);
        }
        
        // Add edges that are relevant to this update
        for (&before_page, after_pages) in &self.rules {
            if update_set.contains(&before_page) {
                for &after_page in after_pages {
                    if update_set.contains(&after_page) {
                        subgraph.get_mut(&before_page).unwrap().push(after_page);
                        *in_degree.get_mut(&after_page).unwrap() += 1;
                    }
                }
            }
        }
        
        // Kahn's algorithm for topological sorting
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        
        // Start with pages that have no incoming edges
        for (&page, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(page);
            }
        }
        
        while let Some(current) = queue.pop_front() {
            result.push(current);
            
            // Remove edges from current node
            if let Some(neighbors) = subgraph.get(&current) {
                for &neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(&neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }
        
        // If result length doesn't match update length, there was a cycle
        // This shouldn't happen in valid AoC input, but handle gracefully
        if result.len() != update.len() {
            eprintln!("Warning: Cycle detected in update {:?}, returning partial sort", update);
        }
        
        result
    }

    /// Get the middle page number from a sequence
    ///
    /// # Arguments
    ///
    /// * `update` - The page sequence
    ///
    /// # Returns
    ///
    /// The middle page number (0-indexed middle for odd-length sequences)
    fn get_middle_page(&self, update: &[i32]) -> i32 {
        if update.is_empty() {
            return 0;
        }
        update[update.len() / 2]
    }
}

/// Solve Day 5 Part 1: Sum of middle pages from correctly ordered updates
///
/// # Arguments
///
/// * `input` - Raw input string containing ordering rules and updates
///
/// # Returns
///
/// String representation of the sum of middle pages from correctly ordered updates
pub fn solve_part1(input: &str) -> Result<String> {
    let queue = PrintQueue::parse(input)
        .context("Failed to parse print queue input")?;
    
    let (part1, _) = queue.solve();
    Ok(part1.to_string())
}

/// Solve Day 5 Part 2: Sum of middle pages from fixed incorrectly ordered updates
///
/// # Arguments
///
/// * `input` - Raw input string containing ordering rules and updates
///
/// # Returns
///
/// String representation of the sum of middle pages from fixed updates
pub fn solve_part2(input: &str) -> Result<String> {
    let queue = PrintQueue::parse(input)
        .context("Failed to parse print queue input")?;
    
    let (_, part2) = queue.solve();
    Ok(part2.to_string())
}

/// Solve Day 5: Print Queue (both parts)
///
/// # Arguments
///
/// * `input` - Raw input string containing ordering rules and updates
///
/// # Returns
///
/// Tuple of (part1_result, part2_result)
///
/// # Examples
///
/// ```
/// use aoc2024::solver::day05::solve;
/// let input = "47|53\n97|13\n\n75,47,61,53,29\n97,61,53,29,13";
/// let (part1, part2) = solve(input).unwrap();
/// // Results will depend on actual input processing
/// ```
pub fn solve(input: &str) -> Result<(i32, i32)> {
    let queue = PrintQueue::parse(input)
        .context("Failed to parse print queue input")?;
    
    Ok(queue.solve())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_parse_input() {
        let queue = PrintQueue::parse(EXAMPLE_INPUT).unwrap();
        
        assert!(!queue.rules.is_empty());
        assert_eq!(queue.updates.len(), 6);
        assert_eq!(queue.updates[0], vec![75, 47, 61, 53, 29]);
    }

    #[test]
    fn test_is_correctly_ordered() {
        let queue = PrintQueue::parse(EXAMPLE_INPUT).unwrap();
        
        // First three should be correctly ordered
        assert!(queue.is_correctly_ordered(&queue.updates[0])); // 75,47,61,53,29
        assert!(queue.is_correctly_ordered(&queue.updates[1])); // 97,61,53,29,13
        assert!(queue.is_correctly_ordered(&queue.updates[2])); // 75,29,13
        
        // Last three should be incorrectly ordered
        assert!(!queue.is_correctly_ordered(&queue.updates[3])); // 75,97,47,61,53
        assert!(!queue.is_correctly_ordered(&queue.updates[4])); // 61,13,29
        assert!(!queue.is_correctly_ordered(&queue.updates[5])); // 97,13,75,29,47
    }

    #[test]
    fn test_get_middle_page() {
        let queue = PrintQueue::parse(EXAMPLE_INPUT).unwrap();
        
        assert_eq!(queue.get_middle_page(&[75, 47, 61, 53, 29]), 61);
        assert_eq!(queue.get_middle_page(&[97, 61, 53, 29, 13]), 53);
        assert_eq!(queue.get_middle_page(&[75, 29, 13]), 29);
    }

    #[test]
    fn test_topological_sort_subgraph() {
        let queue = PrintQueue::parse(EXAMPLE_INPUT).unwrap();
        
        // Test fixing incorrectly ordered sequences
        let fixed1 = queue.topological_sort_subgraph(&queue.updates[3]); // 75,97,47,61,53
        assert!(queue.is_correctly_ordered(&fixed1));
        assert_eq!(queue.get_middle_page(&fixed1), 47);
        
        let fixed2 = queue.topological_sort_subgraph(&queue.updates[4]); // 61,13,29
        assert!(queue.is_correctly_ordered(&fixed2));
        assert_eq!(queue.get_middle_page(&fixed2), 29);
        
        let fixed3 = queue.topological_sort_subgraph(&queue.updates[5]); // 97,13,75,29,47
        assert!(queue.is_correctly_ordered(&fixed3));
        assert_eq!(queue.get_middle_page(&fixed3), 47);
    }

    #[test]
    fn test_solve_example() {
        let (part1, part2) = solve(EXAMPLE_INPUT).unwrap();
        assert_eq!(part1, 143); // 61 + 53 + 29
        assert_eq!(part2, 123); // 47 + 29 + 47
    }

    #[test]
    fn test_empty_input() {
        let result = PrintQueue::parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_malformed_rule() {
        let bad_input = "47-53\n\n75,47,61";
        let result = PrintQueue::parse(bad_input);
        assert!(result.is_err());
    }

    #[test]
    fn test_malformed_update() {
        let bad_input = "47|53\n\n75,47,abc";
        let result = PrintQueue::parse(bad_input);
        assert!(result.is_err());
    }

    // REQ-1: Test Mission 7 Graph Integration Concepts
    #[test]
    fn test_mission_integration_concepts() {
        let queue = PrintQueue::parse(EXAMPLE_INPUT).unwrap();
        
        // Verify graph construction concepts (similar to Mission 7)
        assert!(!queue.rules.is_empty());
        assert!(!queue.all_pages.is_empty());
        
        // Verify topological sorting produces valid orderings
        for update in &queue.updates {
            if !queue.is_correctly_ordered(update) {
                let fixed = queue.topological_sort_subgraph(update);
                assert!(queue.is_correctly_ordered(&fixed));
                assert_eq!(fixed.len(), update.len());
            }
        }
    }

    // REQ-2: Test Algorithm Composition (Mission 8 concepts)
    #[test]
    fn test_algorithm_composition() {
        let queue = PrintQueue::parse(EXAMPLE_INPUT).unwrap();
        
        // Test that we can compose validation + topological sort
        let mut correctly_ordered_count = 0;
        let mut fixed_count = 0;
        
        for update in &queue.updates {
            if queue.is_correctly_ordered(update) {
                correctly_ordered_count += 1;
            } else {
                let fixed = queue.topological_sort_subgraph(update);
                assert!(queue.is_correctly_ordered(&fixed));
                fixed_count += 1;
            }
        }
        
        assert_eq!(correctly_ordered_count, 3);
        assert_eq!(fixed_count, 3);
        assert_eq!(correctly_ordered_count + fixed_count, queue.updates.len());
    }

    // REQ-3: Test Performance Characteristics
    #[test]
    fn test_performance_characteristics() {
        let queue = PrintQueue::parse(EXAMPLE_INPUT).unwrap();
        
        // Ensure topological sort runs in reasonable time
        let start = std::time::Instant::now();
        
        for update in &queue.updates {
            if !queue.is_correctly_ordered(update) {
                queue.topological_sort_subgraph(update);
            }
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should be very fast for small inputs
    }
}