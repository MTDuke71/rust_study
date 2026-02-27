//! Day 16: Proboscidea Volcanium
//!
//! Maximize pressure released by opening valves in a cave network within 30 minutes.
//! Key insight: Only ~15 valves have non-zero flow rates. Compress graph via BFS
//! distances between important valves, then DFS with bitmask to find optimal ordering.

use std::collections::{HashMap, VecDeque};

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: u32,
    tunnels: Vec<usize>, // indices into valve list
}

#[derive(Debug, Clone)]
struct ParsedData {
    important: Vec<usize>,    // indices of non-zero flow valves
    distances: Vec<Vec<u32>>, // distances[i][j] = shortest path between key_nodes[i] and key_nodes[j]
    flows: Vec<u32>,          // flow rates of important valves (parallel to `important`)
}

// ============================================================================
// Parsing
// ============================================================================

fn parse_input(input: &str) -> ParsedData {
    let input = input.replace("\r\n", "\n");
    let lines: Vec<&str> = input.trim().lines().collect();

    // First pass: assign indices to valve names
    let mut name_to_idx: HashMap<&str, usize> = HashMap::new();
    let mut names: Vec<&str> = Vec::new();
    for line in &lines {
        let name = &line[6..8];
        let idx = names.len();
        name_to_idx.insert(name, idx);
        names.push(name);
    }

    // Second pass: parse flow rates and tunnels
    let mut valves: Vec<Valve> = Vec::with_capacity(names.len());
    for line in &lines {
        let parts: Vec<&str> = line.splitn(2, ';').collect();

        // Parse flow rate from "Valve XX has flow rate=N"
        let flow_rate: u32 = parts[0].split('=').nth(1).unwrap().parse().unwrap();

        // Parse tunnel list from "; tunnels lead to valve(s) AA, BB, CC"
        let tunnel_str = if parts[1].contains("valves") {
            parts[1].split("valves ").nth(1).unwrap()
        } else {
            parts[1].split("valve ").nth(1).unwrap()
        };

        let tunnels: Vec<usize> = tunnel_str
            .split(", ")
            .map(|name| name_to_idx[name])
            .collect();

        valves.push(Valve { flow_rate, tunnels });
    }

    let start = name_to_idx["AA"];

    // Identify important valves (non-zero flow)
    let important: Vec<usize> = valves
        .iter()
        .enumerate()
        .filter(|(_, v)| v.flow_rate > 0)
        .map(|(i, _)| i)
        .collect();

    let flows: Vec<u32> = important.iter().map(|&i| valves[i].flow_rate).collect();

    // Compute BFS distances between all important valves + start
    let key_nodes: Vec<usize> = std::iter::once(start)
        .chain(important.iter().copied())
        .collect();

    let mut distances = vec![vec![u32::MAX; key_nodes.len()]; key_nodes.len()];

    for (ki, &from) in key_nodes.iter().enumerate() {
        let dist = bfs_distances(&valves, from);
        for (kj, &to) in key_nodes.iter().enumerate() {
            distances[ki][kj] = dist[to];
        }
    }

    ParsedData {
        important,
        distances,
        flows,
    }
}

/// BFS from a single source, returns distance to every valve
fn bfs_distances(valves: &[Valve], start: usize) -> Vec<u32> {
    let mut dist = vec![u32::MAX; valves.len()];
    dist[start] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        for &next in &valves[node].tunnels {
            if dist[next] == u32::MAX {
                dist[next] = dist[node] + 1;
                queue.push_back(next);
            }
        }
    }

    dist
}

// ============================================================================
// Part 1 Logic
// ============================================================================

fn solve_part1_with_data(data: &ParsedData) -> u32 {
    // DFS through compressed graph
    // State: (current key_node index, opened bitmask, time remaining, pressure accumulated)
    // key_nodes[0] = AA (start), key_nodes[1..] = important valves

    let n = data.important.len();
    let mut best = 0u32;

    // Start at key_node index 0 (AA), no valves open, 30 minutes
    dfs(data, 0, 0u32, 30, 0, n, &mut best);

    best
}

/// DFS to maximize pressure released
/// `pos`: current index in key_nodes (0 = AA, 1.. = important valves)
/// `opened`: bitmask of opened important valves (bit i = important[i])
/// `time`: minutes remaining
/// `pressure`: total pressure accumulated so far
fn dfs(
    data: &ParsedData,
    pos: usize,
    opened: u32,
    time: u32,
    pressure: u32,
    n: usize,
    best: &mut u32,
) {
    *best = (*best).max(pressure);

    for i in 0..n {
        if opened & (1 << i) != 0 {
            continue; // already opened
        }

        // Distance from current position to important valve i
        // In distances matrix: pos is key_node index, target is key_node index i+1
        let dist = data.distances[pos][i + 1];

        // Cost = travel time + 1 minute to open
        let cost = dist + 1;
        if cost >= time {
            continue; // not enough time
        }

        let remaining = time - cost;
        let new_pressure = pressure + data.flows[i] * remaining;

        dfs(
            data,
            i + 1,
            opened | (1 << i),
            remaining,
            new_pressure,
            n,
            best,
        );
    }
}

// ============================================================================
// Part 2 Logic
// ============================================================================

fn solve_part2_with_data(data: &ParsedData) -> u32 {
    let n = data.important.len();
    let size = 1usize << n;

    // Collect best pressure for each opened bitmask with 26 minutes
    let mut best_for_mask = vec![0u32; size];
    dfs_collect(data, 0, 0u32, 26, 0, n, &mut best_for_mask);

    // SOS (Sum over Subsets) DP: propagate so best_for_mask[m] = max over all subsets of m
    // This lets us answer "best pressure achievable using any subset of these valves?"
    for i in 0..n {
        for mask in 0..size {
            if mask & (1 << i) != 0 {
                best_for_mask[mask] = best_for_mask[mask].max(best_for_mask[mask ^ (1 << i)]);
            }
        }
    }

    // Find best disjoint pair: you take mask m, elephant takes complement
    let full = size - 1;
    let mut best = 0u32;
    for m in 0..size {
        best = best.max(best_for_mask[m] + best_for_mask[full ^ m]);
    }

    best
}

/// DFS that records best pressure for each opened bitmask
fn dfs_collect(
    data: &ParsedData,
    pos: usize,
    opened: u32,
    time: u32,
    pressure: u32,
    n: usize,
    best_for_mask: &mut [u32],
) {
    let mask_idx = opened as usize;
    best_for_mask[mask_idx] = best_for_mask[mask_idx].max(pressure);

    for i in 0..n {
        if opened & (1 << i) != 0 {
            continue;
        }

        let dist = data.distances[pos][i + 1];
        let cost = dist + 1;
        if cost >= time {
            continue;
        }

        let remaining = time - cost;
        let new_pressure = pressure + data.flows[i] * remaining;

        dfs_collect(
            data,
            i + 1,
            opened | (1 << i),
            remaining,
            new_pressure,
            n,
            best_for_mask,
        );
    }
}

// ============================================================================
// Public API
// ============================================================================

pub fn solve(input: &str) -> (u32, u32) {
    let data = parse_input(input);
    (solve_part1_with_data(&data), solve_part2_with_data(&data))
}

pub fn solve_part1(input: &str) -> u32 {
    let data = parse_input(input);
    solve_part1_with_data(&data)
}

pub fn solve_part2(input: &str) -> u32 {
    let data = parse_input(input);
    solve_part2_with_data(&data)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_parse() {
        let data = parse_input(EXAMPLE);
        assert_eq!(data.important.len(), 6); // BB, CC, DD, EE, HH, JJ have non-zero flow
        assert_eq!(data.flows.len(), 6);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 1651);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 1707);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day16.txt");
        assert_eq!(solve_part1(input), 1638);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day16.txt");
        assert_eq!(solve_part2(input), 2400);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day16.txt");
        assert_eq!(solve(input), (1638, 2400));
    }
}
