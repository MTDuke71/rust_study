//! Day 9: All in a Single Night - Traveling Salesman Problem
//!
//! This module implements solutions for the Traveling Salesman Problem (TSP) where Santa
//! needs to visit all cities exactly once and return to the starting city.
//!
//! ## Problem Description
//! Santa has new locations to visit and must find either:
//! - **Part 1**: The shortest route that visits all cities exactly once
//! - **Part 2**: The longest route that visits all cities exactly once
//!
//! ## Algorithm
//! Uses brute force permutation generation with Heap's algorithm to explore all possible
//! routes and find the optimal solution.
//!
//! ## Time Complexity
//! O(n! × n) where n is the number of cities
//! - n! permutations × n cities per route = O(n! × n)
//!
//! ## Space Complexity
//! O(n! × n) for storing all permutations

use anyhow::Result;
use mission5::Dictionary;
use std::collections::HashSet;

/// Parses the input string to extract cities and distances between them.
///
/// # Arguments
/// * `input` - String containing city distance pairs in format "City1 to City2 = distance"
///
/// # Returns
/// * `Result<(Vec<&str>, Dictionary<(&str, &str), usize>)>` - Tuple containing:
///   - Vector of unique city names
///   - Dictionary mapping city pairs to distances
///
/// # Example
/// ```
/// use aoc2015::solver::day09::solve_part1;
/// let input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
/// let result = solve_part1(input).unwrap();
/// assert_eq!(result, "605");
/// ```
fn parse_input(input: &str) -> Result<(Vec<&str>, Dictionary<(&str, &str), usize>)> {
    let mut cities = HashSet::new();
    let mut distances = Dictionary::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" = ").collect();
        if parts.len() != 2 {
            continue;
        }

        let cities_part = parts[0];
        let distance_str = parts[1];

        let city_parts: Vec<&str> = cities_part.split(" to ").collect();
        if city_parts.len() != 2 {
            continue;
        }

        let city1 = city_parts[0];
        let city2 = city_parts[1];
        let distance = distance_str.parse::<usize>()?;

        distances.insert((city1, city2), distance);
        cities.insert(city1);
        cities.insert(city2);
    }

    Ok((cities.into_iter().collect(), distances))
}

/// Solves the Traveling Salesman Problem by finding either the shortest or longest route.
///
/// Uses brute force to generate all possible permutations of cities and calculates
/// the total distance for each route to find the optimal solution.
///
/// # Arguments
/// * `cities` - Slice of city names to visit
/// * `distances` - Dictionary mapping city pairs to their distances
/// * `find_min` - If true, find shortest route; if false, find longest route
///
/// # Returns
/// * `usize` - The optimal distance (minimum or maximum)
///
/// # Example
/// ```
/// use aoc2015::solver::day09::solve_part1;
/// let input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
/// let shortest = solve_part1(input).unwrap();
/// assert_eq!(shortest, "605"); // London -> Dublin -> Belfast
/// ```
fn solve_tsp(
    cities: &[&str],
    distances: &Dictionary<(&str, &str), usize>,
    find_min: bool,
) -> usize {
    let mut best_distance = if find_min { usize::MAX } else { 0 };

    // Generate all permutations
    for perm in generate_permutations(cities) {
        let mut total_distance = 0;

        // Calculate distance for this route
        for window in perm.windows(2) {
            let city1 = window[0];
            let city2 = window[1];

            let dist = distances
                .get(&(city1, city2))
                .or_else(|| distances.get(&(city2, city1)))
                .copied()
                .unwrap_or(0);

            total_distance += dist;
        }

        if find_min {
            best_distance = best_distance.min(total_distance);
        } else {
            best_distance = best_distance.max(total_distance);
        }
    }

    best_distance
}

/// Generates all possible permutations of the given cities using Heap's algorithm.
///
/// Heap's algorithm is an efficient method for generating permutations that minimizes
/// the number of swaps needed. It generates permutations in lexicographic order.
///
/// # Arguments
/// * `cities` - Slice of city names to permute
///
/// # Returns
/// * `Vec<Vec<&'a str>>` - Vector containing all possible permutations
///
/// # Complexity
/// * Time: O(n!) where n is the number of cities
/// * Space: O(n! × n) for storing all permutations
///
/// # Example
/// ```
/// use aoc2015::solver::day09::solve_part1;
/// let input = "A to B = 10\nA to C = 15\nB to C = 20";
/// let result = solve_part1(input).unwrap();
/// assert_eq!(result, "25"); // B -> A -> C route
/// ```
fn generate_permutations<'a>(cities: &'a [&'a str]) -> Vec<Vec<&'a str>> {
    if cities.is_empty() {
        return vec![vec![]];
    }

    let mut result = Vec::new();
    let mut cities_copy = cities.to_vec();

    // Heap's algorithm for generating permutations
    fn heap_permute<'a>(cities: &mut Vec<&'a str>, size: usize, result: &mut Vec<Vec<&'a str>>) {
        if size == 1 {
            result.push(cities.clone());
            return;
        }

        for i in 0..size {
            heap_permute(cities, size - 1, result);

            if size % 2 == 1 {
                cities.swap(0, size - 1);
            } else {
                cities.swap(i, size - 1);
            }
        }
    }

    heap_permute(&mut cities_copy, cities.len(), &mut result);
    result
}

/// Day 09 Part 1: Find the shortest route that visits all cities exactly once.
///
/// Santa needs to deliver presents to all locations in the shortest possible distance.
/// This function parses the input, extracts cities and distances, then finds the
/// optimal route using brute force permutation generation.
///
/// # Arguments
/// * `input` - String containing city distance pairs
///
/// # Returns
/// * `Result<String>` - The shortest distance as a string
///
/// # Example
/// ```
/// use aoc2015::solver::day09::solve_part1;
/// let input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
/// let result = solve_part1(input).unwrap();
/// assert_eq!(result, "605");
/// ```
pub fn solve_part1(input: &str) -> Result<String> {
    let (cities, distances) = parse_input(input)?;

    //println!("Cities: {:?}", cities);
    // println!("Distances: {:?}", distances);

    Ok(solve_tsp(&cities, &distances, true).to_string())
}

/// Day 09 Part 2: Find the longest route that visits all cities exactly once.
///
/// Santa decides to show off by taking the longest possible route instead.
/// This function finds the route with maximum distance while still visiting
/// each city exactly once.
///
/// # Arguments
/// * `input` - String containing city distance pairs
///
/// # Returns
/// * `Result<String>` - The longest distance as a string
///
/// # Example
/// ```
/// use aoc2015::solver::day09::solve_part2;
/// let input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
/// let result = solve_part2(input).unwrap();
/// assert_eq!(result, "982");
/// ```
pub fn solve_part2(input: &str) -> Result<String> {
    let (cities, distances) = parse_input(input)?;

    Ok(solve_tsp(&cities, &distances, false).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mission5::Dictionary;

    /// Test data from the problem statement example
    const EXAMPLE_INPUT: &str =
        "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";

    #[test]
    fn test_parse_input() {
        let (cities, distances) = parse_input(EXAMPLE_INPUT).unwrap();

        // Should have 3 unique cities
        assert_eq!(cities.len(), 3);
        assert!(cities.contains(&"London"));
        assert!(cities.contains(&"Dublin"));
        assert!(cities.contains(&"Belfast"));

        // Should have 3 distance entries
        assert_eq!(distances.get(&("London", "Dublin")), Some(&464));
        assert_eq!(distances.get(&("London", "Belfast")), Some(&518));
        assert_eq!(distances.get(&("Dublin", "Belfast")), Some(&141));
    }

    #[test]
    fn test_generate_permutations() {
        let cities = ["A", "B", "C"];
        let perms = generate_permutations(&cities);

        // Should generate 3! = 6 permutations
        assert_eq!(perms.len(), 6);

        // Verify all permutations are unique
        for i in 0..perms.len() {
            for j in (i + 1)..perms.len() {
                assert_ne!(perms[i], perms[j]);
            }
        }

        // Verify each permutation contains all cities
        for perm in &perms {
            assert_eq!(perm.len(), 3);
            assert!(perm.contains(&"A"));
            assert!(perm.contains(&"B"));
            assert!(perm.contains(&"C"));
        }
    }

    #[test]
    fn test_solve_tsp_shortest() {
        let cities = ["London", "Dublin", "Belfast"];
        let mut distances = Dictionary::new();
        distances.insert(("London", "Dublin"), 464);
        distances.insert(("London", "Belfast"), 518);
        distances.insert(("Dublin", "Belfast"), 141);

        let shortest = solve_tsp(&cities, &distances, true);

        // From problem statement: shortest is 605 (London -> Dublin -> Belfast)
        assert_eq!(shortest, 605);
    }

    #[test]
    fn test_solve_tsp_longest() {
        let cities = ["London", "Dublin", "Belfast"];
        let mut distances = Dictionary::new();
        distances.insert(("London", "Dublin"), 464);
        distances.insert(("London", "Belfast"), 518);
        distances.insert(("Dublin", "Belfast"), 141);

        let longest = solve_tsp(&cities, &distances, false);

        // From problem statement: longest is 982 (Dublin -> London -> Belfast)
        assert_eq!(longest, 982);
    }

    #[test]
    fn test_solve_tsp_symmetric_distances() {
        let cities = ["A", "B", "C"];
        let mut distances = Dictionary::new();
        distances.insert(("A", "B"), 10);
        distances.insert(("A", "C"), 15);
        distances.insert(("B", "C"), 20);

        let shortest = solve_tsp(&cities, &distances, true);
        let longest = solve_tsp(&cities, &distances, false);

        // Calculate all possible routes:
        // A -> B -> C = 10 + 20 = 30
        // A -> C -> B = 15 + 20 = 35
        // B -> A -> C = 10 + 15 = 25 (shortest)
        // B -> C -> A = 20 + 15 = 35
        // C -> A -> B = 15 + 10 = 25
        // C -> B -> A = 20 + 10 = 30
        assert_eq!(shortest, 25);
        assert_eq!(longest, 35);
    }

    #[test]
    fn test_solve_part1_example() {
        let result = solve_part1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "605");
    }

    #[test]
    fn test_solve_part2_example() {
        let result = solve_part2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "982");
    }

    #[test]
    fn test_solve_tsp_single_city() {
        let cities = ["OnlyCity"];
        let distances = Dictionary::new();

        let result = solve_tsp(&cities, &distances, true);
        assert_eq!(result, 0); // No travel needed for single city
    }

    #[test]
    fn test_solve_tsp_two_cities() {
        let cities = ["City1", "City2"];
        let mut distances = Dictionary::new();
        distances.insert(("City1", "City2"), 100);

        let shortest = solve_tsp(&cities, &distances, true);
        let longest = solve_tsp(&cities, &distances, false);

        // For two cities, shortest and longest are the same
        assert_eq!(shortest, 100);
        assert_eq!(longest, 100);
    }

    #[test]
    fn test_parse_input_empty() {
        let (cities, distances) = parse_input("").unwrap();
        assert_eq!(cities.len(), 0);
        assert_eq!(distances.len(), 0);
    }

    #[test]
    fn test_parse_input_malformed_lines() {
        let input = "Good line: A to B = 100\nBad line\nAnother good: C to D = 200";
        let (cities, distances) = parse_input(input).unwrap();

        // Should parse 4 cities and 2 distances (ignoring malformed line)
        assert_eq!(cities.len(), 4);
        assert_eq!(distances.len(), 2);
    }
}
