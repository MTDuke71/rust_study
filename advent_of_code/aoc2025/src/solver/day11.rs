//! Day 11: Reactor - Path Counting in Directed Acyclic Graph
//!
//! ## Problem Summary
//! - Given a directed graph of device connections
//! - Count all distinct paths from "you" to "out"
//! - Data flows forward only (DAG - no cycles)
//!
//! ## Solution Approach
//! - Parse input into `HashMap<String, Vec<String>>` (adjacency list)
//! - Use memoized DFS to count paths efficiently
//! - Time: O(V + E) with memoization, Space: O(V)
//!
//! ## Note on Mission 8
//! - Mission 8's Graph trait requires `Copy` nodes (e.g., u32, &str)
//! - String is not Copy, so we can't use Graph trait directly
//! - Instead: implement custom traversal using HashMap methods

use anyhow::Result;
use std::collections::HashMap;

/// Parse input into adjacency list representation.
///
/// Format: "source: target1 target2 target3"
/// Example: "you: bbb ccc" → you has edges to bbb and ccc
fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            let (source, targets) = line.split_once(": ")?;
            let neighbors: Vec<String> = targets.split_whitespace().map(String::from).collect();

            Some((source.to_string(), neighbors))
        })
        .collect()
}

/// Count all paths from start to end using memoized DFS.
///
/// # Algorithm
/// - If current == target: found one path, return 1
/// - If already computed: return cached result
/// - Otherwise: sum paths from all neighbors, cache result
///
/// # Complexity
/// - Time: O(V + E) - each node visited once due to memoization
/// - Space: O(V) - memo table stores result for each node
fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    memo: &mut HashMap<String, usize>,
) -> usize {
    // Base case: reached target
    if current == target {
        return 1;
    }

    // Check memo cache
    if let Some(&cached) = memo.get(current) {
        return cached;
    }

    // Recursive case: sum paths through all neighbors
    let count: usize = graph
        .get(current)
        .map(|neighbors| {
            neighbors
                .iter()
                .map(|next| count_paths(graph, next, target, memo))
                .sum()
        })
        .unwrap_or(0);

    // Cache result
    memo.insert(current.to_string(), count);
    count
}

pub fn solve_part1(input: &str) -> Result<String> {
    let graph = parse_input(input);

    // Verify graph contains start node
    let start = "you";
    let end = "out";

    if !graph.contains_key(start) {
        anyhow::bail!("Graph doesn't contain start node 'you'");
    }

    let mut memo = HashMap::new();
    let count = count_paths(&graph, start, end, &mut memo);

    Ok(count.to_string())
}

/// Count paths from start to end that pass through all required nodes.
///
/// Uses memoization with state = (current_node, set_of_visited_required_nodes)
/// This dramatically reduces computation by caching results for each unique state.
///
/// # Algorithm
/// - State: (current node, bitmask of which required nodes visited)
/// - For 2 required nodes: 4 possible states per node (00, 01, 10, 11)
/// - Cache results for each state to avoid recomputation
///
/// # Complexity
/// - Time: O(V * 2^R) where R = number of required nodes
/// - Space: O(V * 2^R) for memo table
fn count_paths_with_requirements_memo(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    required: &[&str],
    visited_mask: usize, // Bitmask: which required nodes visited
    memo: &mut HashMap<(String, usize), usize>,
) -> usize {
    // Base case: reached target
    if current == target {
        // Check if all required nodes visited (all bits set)
        let all_visited = (1 << required.len()) - 1;
        return if visited_mask == all_visited { 1 } else { 0 };
    }

    // Check memo cache
    let state = (current.to_string(), visited_mask);
    if let Some(&cached) = memo.get(&state) {
        return cached;
    }

    // Update visited mask if current node is required
    let mut new_mask = visited_mask;
    for (i, &req) in required.iter().enumerate() {
        if current == req {
            new_mask |= 1 << i;
        }
    }

    // Recursive case: explore all neighbors
    let count: usize = graph
        .get(current)
        .map(|neighbors| {
            neighbors
                .iter()
                .map(|next| {
                    count_paths_with_requirements_memo(
                        graph, next, target, required, new_mask, memo,
                    )
                })
                .sum()
        })
        .unwrap_or(0);

    // Cache result
    memo.insert(state, count);
    count
}

pub fn solve_part2(input: &str) -> Result<String> {
    let graph = parse_input(input);

    // Part 2: count paths from "svr" to "out" that visit both "dac" and "fft"
    let start = "svr";
    let end = "out";
    let required = vec!["dac", "fft"];

    if !graph.contains_key(start) {
        anyhow::bail!("Graph doesn't contain start node 'svr'");
    }

    let mut memo = HashMap::new();
    let count = count_paths_with_requirements_memo(&graph, start, end, &required, 0, &mut memo);

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

    #[test]
    fn test_parse_input() {
        let graph = parse_input(EXAMPLE);

        // Check "you" node
        assert!(graph.contains_key("you"));
        let you_neighbors = graph.get("you").unwrap();
        assert_eq!(you_neighbors.len(), 2);
        assert!(you_neighbors.contains(&"bbb".to_string()));
        assert!(you_neighbors.contains(&"ccc".to_string()));

        // Check "ccc" node has 3 neighbors
        let ccc_neighbors = graph.get("ccc").unwrap();
        assert_eq!(ccc_neighbors.len(), 3);
    }

    #[test]
    fn test_example_paths() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, "5", "Example should have 5 paths from you to out");
    }

    #[test]
    fn test_simple_path_count() {
        // Simple linear path: A -> B -> C
        let input = "A: B\nB: C\nC: out";
        let graph = parse_input(input);
        let mut memo = HashMap::new();
        let count = count_paths(&graph, "A", "out", &mut memo);
        assert_eq!(count, 1, "Linear path should have 1 path");
    }

    #[test]
    fn test_branching_paths() {
        // A has 2 branches both leading to out
        //     B
        //    / \
        //   A   out
        //    \ /
        //     C
        let input = "A: B C\nB: out\nC: out";
        let graph = parse_input(input);
        let mut memo = HashMap::new();
        let count = count_paths(&graph, "A", "out", &mut memo);
        assert_eq!(count, 2, "Should have 2 paths: A->B->out and A->C->out");
    }

    #[test]
    fn test_part2_example() {
        let example = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;

        let result = solve_part2(example).unwrap();
        assert_eq!(
            result, "2",
            "Part 2 example should have 2 paths visiting both dac and fft"
        );
    }

    #[test]
    fn test_path_with_requirements() {
        // Simple test: A -> B -> C -> D -> out
        // Requirement: must visit B and D
        let input = "A: B\nB: C\nC: D\nD: out";
        let graph = parse_input(input);
        let mut memo = HashMap::new();
        let count =
            count_paths_with_requirements_memo(&graph, "A", "out", &["B", "D"], 0, &mut memo);
        assert_eq!(count, 1, "Should have 1 path visiting both B and D");
    }
}
