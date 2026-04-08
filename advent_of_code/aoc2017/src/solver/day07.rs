//! Day 7: Recursive Circus
//!
//! Find the root of a program tree, then identify the one program with
//! the wrong weight and compute the correction needed for balance.

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Program<'a> {
    weight: u32,
    children: Vec<&'a str>,
}

fn parse_input(input: &str) -> HashMap<&str, Program<'_>> {
    input
        .lines()
        .map(|line| {
            // Format: "name (weight)" or "name (weight) -> child1, child2, ..."
            let (name, rest) = line.split_once(' ').unwrap();
            let (weight_str, children_str) = rest.split_once(')').unwrap();
            let weight = weight_str[1..].parse().unwrap(); // skip '('
            let children = if let Some(raw) = children_str.strip_prefix(" -> ") {
                raw.split(", ").collect()
            } else {
                Vec::new()
            };
            (name, Program { weight, children })
        })
        .collect()
}

fn find_root<'a>(programs: &HashMap<&'a str, Program<'a>>) -> &'a str {
    let all_children: HashSet<&str> = programs
        .values()
        .flat_map(|p| &p.children)
        .copied()
        .collect();

    programs
        .keys()
        .find(|name| !all_children.contains(*name))
        .unwrap()
}

/// Returns the total weight of the subtree rooted at `name`.
/// If an imbalance is found, returns Err(corrected_weight).
fn total_weight(
    name: &str,
    programs: &HashMap<&str, Program<'_>>,
    depth: usize,
    visited: &mut usize,
    max_depth: &mut usize,
) -> Result<u32, u32> {
    *visited += 1;
    if depth > *max_depth {
        *max_depth = depth;
    }
    let prog = &programs[name];
    if prog.children.is_empty() {
        return Ok(prog.weight);
    }

    let mut child_weights: Vec<(&str, u32)> = Vec::new();
    for &child in &prog.children {
        let w = total_weight(child, programs, depth + 1, visited, max_depth)?;
        child_weights.push((child, w));
    }

    // Check if all children have the same total weight
    let first = child_weights[0].1;
    if child_weights.iter().all(|&(_, w)| w == first) {
        let total = prog.weight + child_weights.iter().map(|&(_, w)| w).sum::<u32>();
        return Ok(total);
    }

    // Find the odd one out — group by weight, the minority is wrong
    let mut freq: HashMap<u32, Vec<&str>> = HashMap::new();
    for &(name, w) in &child_weights {
        freq.entry(w).or_default().push(name);
    }

    let (&wrong_total, wrong_names) = freq.iter().find(|(_, v)| v.len() == 1).unwrap();
    let &correct_total = freq.keys().find(|&&k| k != wrong_total).unwrap();

    let wrong_name = wrong_names[0];
    let wrong_own_weight = programs[wrong_name].weight;
    let diff = correct_total as i64 - wrong_total as i64;
    let corrected = (wrong_own_weight as i64 + diff) as u32;

    eprintln!(
        "IMBALANCE at depth {depth}: {name}'s disc — {wrong_name} (weight {wrong_own_weight}) \
         subtree {wrong_total} vs expected {correct_total} → corrected weight = {corrected}"
    );

    Err(corrected)
}

fn solve_part1_with_data(programs: &HashMap<&str, Program<'_>>) -> String {
    find_root(programs).to_string()
}

fn solve_part2_with_data(programs: &HashMap<&str, Program<'_>>, root: &str) -> u32 {
    let mut visited = 0;
    let mut max_depth = 0;
    let total_nodes = programs.len();
    let result = match total_weight(root, programs, 0, &mut visited, &mut max_depth) {
        Ok(_) => unreachable!("puzzle guarantees an imbalance"),
        Err(corrected) => corrected,
    };
    eprintln!(
        "DEBUG: visited {visited}/{total_nodes} nodes ({:.1}% of tree), max depth = {max_depth}",
        visited as f64 / total_nodes as f64 * 100.0
    );
    result
}

pub fn solve(input: &str) -> (String, u32) {
    let programs = parse_input(input);
    let root = find_root(&programs);
    let part1 = root.to_string();
    let part2 = solve_part2_with_data(&programs, root);
    (part1, part2)
}

pub fn solve_part1(input: &str) -> String {
    let programs = parse_input(input);
    solve_part1_with_data(&programs)
}

pub fn solve_part2(input: &str) -> u32 {
    let programs = parse_input(input);
    let root = find_root(&programs);
    solve_part2_with_data(&programs, root)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    #[test]
    fn test_parse() {
        let programs = parse_input(EXAMPLE);
        assert_eq!(programs.len(), 13);
        assert_eq!(programs["tknk"].weight, 41);
        assert_eq!(programs["tknk"].children, vec!["ugml", "padx", "fwft"]);
        assert!(programs["pbga"].children.is_empty());
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), "tknk");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 60);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day07.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, "airlri");
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day07.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, 1206);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day07.txt");
        assert_eq!(solve(input), ("airlri".to_string(), 1206));
    }
}
