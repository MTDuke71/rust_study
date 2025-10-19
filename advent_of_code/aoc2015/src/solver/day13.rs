use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};

/// AoC 2015 Day 13: Knights of the Dinner Table
/// Optimal seating arrangement problem
///
/// ## Problem Description
/// Find the optimal seating arrangement at a circular dinner table to maximize
/// happiness. Each person has preferences for sitting next to each other person,
/// expressed as happiness gain/loss values.
///
/// ## Part 1
/// Find the maximum happiness achievable by arranging all people around the table.
///
/// ## Part 2  
/// Add yourself to the table (with neutral happiness toward everyone) and find
/// the new maximum happiness.
pub fn solve_part1(input: &str) -> Result<String> {
    println!("🎄 Day 13 Part 1: Knights of the Dinner Table");
    
    let graph = parse_happiness_graph(input)?;
    let (max_happiness, _arrangement) = find_optimal_seating(&graph);
    
    Ok(max_happiness.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    println!("🎄 Day 13 Part 2: Knights of the Dinner Table (with yourself)");
    
    let mut graph = parse_happiness_graph(input)?;
    
    // Add yourself with neutral happiness toward everyone
    graph.add_neutral_person("You");
    
    let (max_happiness, _arrangement) = find_optimal_seating(&graph);
    
    Ok(max_happiness.to_string())
}

// ============================================================================
// Data Structures
// ============================================================================

/// Represents the happiness graph for dinner table seating
/// 
/// This is a directed weighted graph where:
/// - Nodes are people (identified by name)
/// - Edges are happiness values (can be positive or negative)
/// - Each person has a happiness value for sitting next to each other person
#[derive(Debug, Clone, PartialEq)]
pub struct HappinessGraph {
    /// Maps (person_a, person_b) -> happiness change for person_a sitting next to person_b
    pub edges: HashMap<(String, String), i32>,
    /// Set of all people in the graph
    pub people: HashSet<String>,
}

impl Default for HappinessGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl HappinessGraph {
    /// Creates a new empty happiness graph
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            people: HashSet::new(),
        }
    }

    /// Adds a happiness edge to the graph
    /// 
    /// ## Arguments
    /// * `person_a` - The person whose happiness is being described
    /// * `person_b` - The person they would sit next to
    /// * `happiness` - The happiness change (positive = gain, negative = loss)
    pub fn add_edge(&mut self, person_a: &str, person_b: &str, happiness: i32) {
        self.edges.insert((person_a.to_string(), person_b.to_string()), happiness);
        self.people.insert(person_a.to_string());
        self.people.insert(person_b.to_string());
    }

    /// Gets the happiness value for person_a sitting next to person_b
    /// Returns 0 if no edge exists (neutral happiness)
    pub fn get_happiness(&self, person_a: &str, person_b: &str) -> i32 {
        self.edges.get(&(person_a.to_string(), person_b.to_string()))
            .copied()
            .unwrap_or(0)
    }

    /// Gets all people in the graph
    pub fn get_people(&self) -> Vec<String> {
        self.people.iter().cloned().collect()
    }

    /// Adds a person with neutral happiness (0) toward everyone
    /// This is used for Part 2 where you add yourself to the table
    pub fn add_neutral_person(&mut self, person: &str) {
        for existing_person in &self.people.clone() {
            // You have 0 happiness toward everyone
            self.add_edge(person, existing_person, 0);
            // Everyone has 0 happiness toward you
            self.add_edge(existing_person, person, 0);
        }
        self.people.insert(person.to_string());
    }
}

// ============================================================================
// Parsing Functions
// ============================================================================

/// Parses the input into a happiness graph
/// 
/// ## Input Format
/// Each line follows the pattern:
/// "Alice would gain 54 happiness units by sitting next to Bob."
/// "Carol would lose 79 happiness units by sitting next to David."
/// 
/// ## Algorithm
/// 1. Parse each line to extract person_a, person_b, and happiness value
/// 2. Handle both "gain" (positive) and "lose" (negative) happiness
/// 3. Build directed graph edges
/// 
/// ## Examples
/// ```
/// # use aoc2015::solver::day13::parse_happiness_graph;
/// let input = "Alice would gain 54 happiness units by sitting next to Bob.\nBob would lose 7 happiness units by sitting next to Alice.";
/// let graph = parse_happiness_graph(input).unwrap();
/// assert_eq!(graph.get_happiness("Alice", "Bob"), 54);
/// assert_eq!(graph.get_happiness("Bob", "Alice"), -7);
/// ```
pub fn parse_happiness_graph(input: &str) -> Result<HappinessGraph> {
    let mut graph = HappinessGraph::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse line: "Alice would gain 54 happiness units by sitting next to Bob."
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        // Validate line format
        if parts.len() < 11 {
            continue; // Skip malformed lines
        }

        let person_a = parts[0];
        let gain_or_lose = parts[2]; // "gain" or "lose"
        let happiness_str = parts[3];
        let person_b = parts[10].trim_end_matches('.');

        // Parse happiness value
        let happiness_value: i32 = happiness_str.parse()
            .with_context(|| format!("Failed to parse happiness value: {}", happiness_str))?;

        // Apply sign based on gain/lose
        let happiness = match gain_or_lose {
            "gain" => happiness_value,
            "lose" => -happiness_value,
            _ => return Err(anyhow::anyhow!("Expected 'gain' or 'lose', found: {}", gain_or_lose)),
        };

        graph.add_edge(person_a, person_b, happiness);
    }

    Ok(graph)
}

// ============================================================================
// Seating Optimization Algorithms (Adapted from TSP)
// ============================================================================

/// Finds the optimal seating arrangement to maximize happiness
/// 
/// Uses brute force permutation generation to explore all possible circular
/// seating arrangements and finds the one with maximum total happiness.
/// 
/// ## Algorithm
/// 1. Generate all permutations of people using Heap's algorithm
/// 2. For each permutation, calculate total happiness in circular arrangement
/// 3. Track the arrangement with maximum happiness
/// 
/// ## Circular Seating Calculation
/// Each person sits between exactly 2 neighbors:
/// - Left neighbor: `perm[(i + perm.len() - 1) % perm.len()]`
/// - Right neighbor: `perm[(i + 1) % perm.len()]`
/// - Total happiness += person's happiness toward both neighbors
/// 
/// ## Time Complexity
/// O(n! × n) where n is number of people
/// 
/// ## Examples
/// ```
/// # use aoc2015::solver::day13::{HappinessGraph, find_optimal_seating};
/// let mut graph = HappinessGraph::new();
/// graph.add_edge("Alice", "Bob", 54);
/// graph.add_edge("Bob", "Alice", 83);
/// 
/// let (happiness, arrangement) = find_optimal_seating(&graph);
/// assert!(happiness > 0);
/// assert_eq!(arrangement.len(), 2);
/// ```
pub fn find_optimal_seating(graph: &HappinessGraph) -> (i32, Vec<String>) {
    let people: Vec<String> = graph.get_people();
    
    if people.is_empty() {
        return (0, Vec::new());
    }
    
    if people.len() == 1 {
        return (0, people);
    }
    
    let mut best_happiness = i32::MIN;
    let mut best_arrangement = people.clone();
    
    // Generate all permutations and find the one with maximum happiness
    let all_permutations = generate_permutations(&people);
    
    for arrangement in all_permutations {
        let total_happiness = calculate_circular_happiness(graph, &arrangement);
        
        if total_happiness > best_happiness {
            best_happiness = total_happiness;
            best_arrangement = arrangement;
        }
    }
    
    (best_happiness, best_arrangement)
}

/// Calculates total happiness for a circular seating arrangement
/// 
/// In a circular arrangement, each person sits between exactly two others.
/// We sum up each person's happiness toward their left and right neighbors.
/// 
/// ## Important: With 2 people in circular arrangement
/// When only 2 people sit at a circular table, each person has the other person
/// as BOTH their left AND right neighbor. This means:
/// - Alice sees Bob on her left AND right
/// - Bob sees Alice on his left AND right
/// 
/// So we count Alice's happiness toward Bob twice, and Bob's toward Alice twice.
/// 
/// ## Parameters
/// - `graph`: The happiness graph containing all happiness values
/// - `arrangement`: The seating arrangement (circular order)
/// 
/// ## Returns
/// Total happiness value for this arrangement
/// 
/// ## Examples
/// ```
/// # use aoc2015::solver::day13::{HappinessGraph, calculate_circular_happiness};
/// let mut graph = HappinessGraph::new();
/// graph.add_edge("Alice", "Bob", 10);
/// graph.add_edge("Bob", "Alice", 20);
/// 
/// let arrangement = vec!["Alice".to_string(), "Bob".to_string()];
/// let happiness = calculate_circular_happiness(&graph, &arrangement);
/// assert_eq!(happiness, 60); // (10+10) + (20+20) = 60
/// ```
pub fn calculate_circular_happiness(graph: &HappinessGraph, arrangement: &[String]) -> i32 {
    if arrangement.len() < 2 {
        return 0;
    }
    
    let mut total_happiness = 0;
    
    for i in 0..arrangement.len() {
        let person = &arrangement[i];
        
        // Find left and right neighbors in circular arrangement
        let left_neighbor = &arrangement[(i + arrangement.len() - 1) % arrangement.len()];
        let right_neighbor = &arrangement[(i + 1) % arrangement.len()];
        
        // Add this person's happiness toward both neighbors
        total_happiness += graph.get_happiness(person, left_neighbor);
        total_happiness += graph.get_happiness(person, right_neighbor);
    }
    
    total_happiness
}

/// Generates all possible permutations using Heap's algorithm
/// 
/// This is adapted from Day 9's permutation generation but works with String vectors
/// instead of string slices for better ownership semantics with the happiness graph.
/// 
/// ## Algorithm
/// Heap's algorithm generates permutations by systematically swapping elements.
/// It minimizes the number of swaps needed and generates permutations efficiently.
/// 
/// ## Time Complexity
/// O(n!) - generates all n! possible permutations
/// 
/// ## Space Complexity  
/// O(n! × n) - stores all permutations
/// 
/// ## Examples
/// ```
/// # use aoc2015::solver::day13::generate_permutations;
/// let people = vec!["Alice".to_string(), "Bob".to_string()];
/// let perms = generate_permutations(&people);
/// assert_eq!(perms.len(), 2); // 2! = 2 permutations
/// ```
pub fn generate_permutations(people: &[String]) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let mut current = people.to_vec();
    
    heap_algorithm(&mut current, people.len(), &mut result);
    result
}

/// Heap's algorithm implementation for generating permutations
/// 
/// This is a recursive implementation that generates all permutations by
/// systematically swapping elements based on whether the size is even or odd.
/// 
/// ## Algorithm Details
/// - If size is odd: swap first and last elements
/// - If size is even: swap current index and last elements  
/// - Recursively generate permutations for smaller sizes
/// 
/// ## Parameters
/// - `arr`: Mutable array to permute
/// - `size`: Current size to consider (counts down to 1)
/// - `result`: Vector to store all generated permutations
fn heap_algorithm(arr: &mut [String], size: usize, result: &mut Vec<Vec<String>>) {
    // Base case: if size is 1, we have a complete permutation
    if size == 1 {
        result.push(arr.to_vec());
        return;
    }
    
    for i in 0..size {
        // Generate permutations with the first size-1 elements
        heap_algorithm(arr, size - 1, result);
        
        // If size is odd, swap first and last element
        // If size is even, swap i-th and last element
        if size % 2 == 1 {
            arr.swap(0, size - 1);
        } else {
            arr.swap(i, size - 1);
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // Graph Structure Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_happiness_graph_creation() {
        let mut graph = HappinessGraph::new();
        
        // Initially empty
        assert_eq!(graph.people.len(), 0);
        assert_eq!(graph.edges.len(), 0);

        // Add some edges
        graph.add_edge("Alice", "Bob", 54);
        graph.add_edge("Bob", "Alice", -7);

        // Verify structure
        assert_eq!(graph.people.len(), 2);
        assert!(graph.people.contains("Alice"));
        assert!(graph.people.contains("Bob"));
        
        assert_eq!(graph.get_happiness("Alice", "Bob"), 54);
        assert_eq!(graph.get_happiness("Bob", "Alice"), -7);
        assert_eq!(graph.get_happiness("Alice", "Carol"), 0); // Non-existent edge
    }

    #[test]
    fn test_add_neutral_person() {
        let mut graph = HappinessGraph::new();
        
        // Add initial people with some preferences
        graph.add_edge("Alice", "Bob", 10);
        graph.add_edge("Bob", "Alice", -5);
        
        // Add neutral person (yourself)
        graph.add_neutral_person("You");
        
        // Verify neutral connections
        assert_eq!(graph.get_happiness("You", "Alice"), 0);
        assert_eq!(graph.get_happiness("You", "Bob"), 0);
        assert_eq!(graph.get_happiness("Alice", "You"), 0);
        assert_eq!(graph.get_happiness("Bob", "You"), 0);
        
        // Verify original edges still exist
        assert_eq!(graph.get_happiness("Alice", "Bob"), 10);
        assert_eq!(graph.get_happiness("Bob", "Alice"), -5);
        
        // Verify people count
        assert_eq!(graph.people.len(), 3);
    }

    // ------------------------------------------------------------------------
    // Parsing Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_parse_single_gain_line() {
        let input = "Alice would gain 54 happiness units by sitting next to Bob.";
        let graph = parse_happiness_graph(input).unwrap();
        
        assert_eq!(graph.get_happiness("Alice", "Bob"), 54);
        assert_eq!(graph.people.len(), 2);
        assert!(graph.people.contains("Alice"));
        assert!(graph.people.contains("Bob"));
    }

    #[test]
    fn test_parse_single_lose_line() {
        let input = "Bob would lose 7 happiness units by sitting next to Alice.";
        let graph = parse_happiness_graph(input).unwrap();
        
        assert_eq!(graph.get_happiness("Bob", "Alice"), -7);
        assert_eq!(graph.people.len(), 2);
    }

    #[test]
    fn test_parse_multiple_lines() {
        let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Bob would gain 83 happiness units by sitting next to Alice.
Carol would gain 20 happiness units by sitting next to Alice."#;
        
        let graph = parse_happiness_graph(input).unwrap();
        
        // Verify all edges
        assert_eq!(graph.get_happiness("Alice", "Bob"), 54);
        assert_eq!(graph.get_happiness("Alice", "Carol"), -79);
        assert_eq!(graph.get_happiness("Bob", "Alice"), 83);
        assert_eq!(graph.get_happiness("Carol", "Alice"), 20);
        
        // Verify people
        assert_eq!(graph.people.len(), 3);
        assert!(graph.people.contains("Alice"));
        assert!(graph.people.contains("Bob"));
        assert!(graph.people.contains("Carol"));
    }

    #[test]
    fn test_parse_example_from_problem() {
        // This is the example typically provided in AoC Day 13
        let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#;

        let graph = parse_happiness_graph(input).unwrap();
        
        // Verify it's a complete directed graph (4 people, 12 directed edges)
        assert_eq!(graph.people.len(), 4);
        assert_eq!(graph.edges.len(), 12);
        
        // Spot check a few values
        assert_eq!(graph.get_happiness("Alice", "Bob"), 54);
        assert_eq!(graph.get_happiness("Alice", "Carol"), -79);
        assert_eq!(graph.get_happiness("Bob", "David"), -63);
        assert_eq!(graph.get_happiness("Carol", "David"), 55);
        assert_eq!(graph.get_happiness("David", "Carol"), 41);
    }

    #[test]
    fn test_parse_empty_input() {
        let graph = parse_happiness_graph("").unwrap();
        assert_eq!(graph.people.len(), 0);
        assert_eq!(graph.edges.len(), 0);
    }

    #[test]
    fn test_parse_with_empty_lines() {
        let input = r#"
Alice would gain 54 happiness units by sitting next to Bob.

Bob would lose 7 happiness units by sitting next to Alice.

"#;
        let graph = parse_happiness_graph(input).unwrap();
        
        assert_eq!(graph.people.len(), 2);
        assert_eq!(graph.get_happiness("Alice", "Bob"), 54);
        assert_eq!(graph.get_happiness("Bob", "Alice"), -7);
    }

    #[test]
    fn test_parse_malformed_line() {
        let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
This is not a valid line
Bob would lose 7 happiness units by sitting next to Alice."#;
        
        let graph = parse_happiness_graph(input).unwrap();
        
        // Should parse valid lines and skip malformed ones
        assert_eq!(graph.people.len(), 2);
        assert_eq!(graph.get_happiness("Alice", "Bob"), 54);
        assert_eq!(graph.get_happiness("Bob", "Alice"), -7);
    }

    #[test]
    fn test_parse_invalid_number() {
        let input = "Alice would gain invalid happiness units by sitting next to Bob.";
        let result = parse_happiness_graph(input);
        
        // Should return an error for invalid number
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_gain_lose() {
        let input = "Alice would neither 54 happiness units by sitting next to Bob.";
        let result = parse_happiness_graph(input);
        
        // Should return an error for invalid gain/lose word
        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // Integration Tests for Problem Examples
    // ------------------------------------------------------------------------

    #[test]
    fn test_example_part1() {
        // Basic test with stub implementation
        assert_eq!(solve_part1("").unwrap(), "0");
    }

    #[test]
    fn test_example_part2() {
        // Basic test with stub implementation
        assert_eq!(solve_part2("").unwrap(), "0");
    }

    #[test]
    fn test_graph_completeness_check() {
        let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob."#;

        let graph = parse_happiness_graph(input).unwrap();
        let people = graph.get_people();
        
        // For a complete graph, we should have n*(n-1) edges for n people
        // Here: 3 people = 3*2 = 6 edges
        assert_eq!(people.len(), 3);
        assert_eq!(graph.edges.len(), 6);
        
        // Verify bidirectional edges exist (even if values are different)
        assert_ne!(graph.get_happiness("Alice", "Bob"), 0);
        assert_ne!(graph.get_happiness("Bob", "Alice"), 0);
        assert_ne!(graph.get_happiness("Alice", "Carol"), 0);
        assert_ne!(graph.get_happiness("Carol", "Alice"), 0);
        assert_ne!(graph.get_happiness("Bob", "Carol"), 0);
        assert_ne!(graph.get_happiness("Carol", "Bob"), 0);
    }

    // ------------------------------------------------------------------------
    // Optimization Algorithm Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_generate_permutations_simple() {
        let people = vec!["Alice".to_string(), "Bob".to_string()];
        let perms = generate_permutations(&people);
        
        assert_eq!(perms.len(), 2); // 2! = 2
        
        // Should contain both arrangements
        assert!(perms.contains(&vec!["Alice".to_string(), "Bob".to_string()]));
        assert!(perms.contains(&vec!["Bob".to_string(), "Alice".to_string()]));
    }

    #[test]
    fn test_generate_permutations_three_people() {
        let people = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let perms = generate_permutations(&people);
        
        assert_eq!(perms.len(), 6); // 3! = 6
        
        // Each person should appear in each position exactly 2 times (6/3 = 2)
        let first_positions: Vec<&String> = perms.iter().map(|p| &p[0]).collect();
        assert_eq!(first_positions.iter().filter(|&&s| s == "A").count(), 2);
        assert_eq!(first_positions.iter().filter(|&&s| s == "B").count(), 2);
        assert_eq!(first_positions.iter().filter(|&&s| s == "C").count(), 2);
    }

    #[test]
    fn test_calculate_circular_happiness_two_people() {
        let mut graph = HappinessGraph::new();
        graph.add_edge("Alice", "Bob", 10);
        graph.add_edge("Bob", "Alice", 20);
        
        let arrangement = vec!["Alice".to_string(), "Bob".to_string()];
        let happiness = calculate_circular_happiness(&graph, &arrangement);
        
        // In circular seating with 2 people: each person has the other as BOTH neighbors
        // Alice's happiness: 10 (left=Bob) + 10 (right=Bob) = 20
        // Bob's happiness: 20 (left=Alice) + 20 (right=Alice) = 40  
        // Total: 20 + 40 = 60
        assert_eq!(happiness, 60);
    }

    #[test]
    fn test_calculate_circular_happiness_three_people() {
        let mut graph = HappinessGraph::new();
        
        // Alice's preferences
        graph.add_edge("Alice", "Bob", 54);
        graph.add_edge("Alice", "Carol", -79);
        
        // Bob's preferences  
        graph.add_edge("Bob", "Alice", 83);
        graph.add_edge("Bob", "Carol", -7);
        
        // Carol's preferences
        graph.add_edge("Carol", "Alice", -62);
        graph.add_edge("Carol", "Bob", 60);
        
        let arrangement = vec!["Alice".to_string(), "Bob".to_string(), "Carol".to_string()];
        let happiness = calculate_circular_happiness(&graph, &arrangement);
        
        // Circular arrangement: Alice - Bob - Carol - Alice
        // Alice: sits next to Carol (left) and Bob (right) = -79 + 54 = -25
        // Bob: sits next to Alice (left) and Carol (right) = 83 + (-7) = 76  
        // Carol: sits next to Bob (left) and Alice (right) = 60 + (-62) = -2
        // Total: -25 + 76 + (-2) = 49
        assert_eq!(happiness, 49);
    }

    #[test]
    fn test_find_optimal_seating_empty() {
        let graph = HappinessGraph::new();
        let (happiness, arrangement) = find_optimal_seating(&graph);
        
        assert_eq!(happiness, 0);
        assert_eq!(arrangement.len(), 0);
    }

    #[test]
    fn test_find_optimal_seating_single_person() {
        let mut graph = HappinessGraph::new();
        graph.add_neutral_person("Alice");
        
        let (happiness, arrangement) = find_optimal_seating(&graph);
        
        assert_eq!(happiness, 0); // No neighbors = no happiness
        assert_eq!(arrangement.len(), 1);
        assert_eq!(arrangement[0], "Alice");
    }

    #[test]
    fn test_find_optimal_seating_two_people() {
        let mut graph = HappinessGraph::new();
        graph.add_edge("Alice", "Bob", 10);
        graph.add_edge("Bob", "Alice", 20);
        
        let (happiness, arrangement) = find_optimal_seating(&graph);
        
        assert_eq!(happiness, 60); // (10+10) + (20+20) = 60 (circular with 2 people)
        assert_eq!(arrangement.len(), 2);
        // Both arrangements are equivalent in circular seating
    }

    #[test]
    fn test_find_optimal_seating_example_data() {
        let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#;

        let graph = parse_happiness_graph(input).unwrap();
        let (happiness, arrangement) = find_optimal_seating(&graph);
        
        // Should find some positive happiness value
        assert!(happiness > 0);
        assert_eq!(arrangement.len(), 4);
        
        // Verify the arrangement produces the reported happiness
        let calculated = calculate_circular_happiness(&graph, &arrangement);
        assert_eq!(calculated, happiness);
    }

    // ------------------------------------------------------------------------
    // Part 2 Integration Tests  
    // ------------------------------------------------------------------------

    #[test]
    fn test_add_neutral_person_to_example() {
        let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
Bob would lose 7 happiness units by sitting next to Alice."#;

        let mut graph = parse_happiness_graph(input).unwrap();
        assert_eq!(graph.people.len(), 2);
        
        // Add yourself with neutral happiness
        graph.add_neutral_person("You");
        assert_eq!(graph.people.len(), 3);
        
        // Verify neutral connections
        assert_eq!(graph.get_happiness("You", "Alice"), 0);
        assert_eq!(graph.get_happiness("You", "Bob"), 0);
        assert_eq!(graph.get_happiness("Alice", "You"), 0);
        assert_eq!(graph.get_happiness("Bob", "You"), 0);
        
        // Original connections should still exist
        assert_eq!(graph.get_happiness("Alice", "Bob"), 54);
        assert_eq!(graph.get_happiness("Bob", "Alice"), -7);
    }

    // ------------------------------------------------------------------------
    // Integration Tests with Actual Solutions
    // ------------------------------------------------------------------------

    #[test]
    fn test_solve_part1_with_simple_input() {
        let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
Bob would gain 83 happiness units by sitting next to Alice."#;

        let result = solve_part1(input).unwrap();
        assert_eq!(result, "274"); // (54+54) + (83+83) = 274 in circular arrangement with 2 people
    }

    #[test]
    fn test_solve_part2_adds_neutral_person() {
        let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
Bob would gain 83 happiness units by sitting next to Alice."#;

        let result1 = solve_part1(input).unwrap();
        let result2 = solve_part2(input).unwrap();
        
        // Part 2 should have different (likely lower) happiness due to neutral person
        assert_ne!(result1, result2);
        
        // Part 2 result should be valid number
        assert!(result2.parse::<i32>().is_ok());
    }
}