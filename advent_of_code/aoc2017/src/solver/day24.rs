//! Day 24: Electromagnetic Moat
//!
//! Build a bridge from magnetic components `a/b`, where consecutive components
//! must share a matching port type. The bridge starts from a type-0 port, each
//! component is used at most once, and ports may be flipped freely.
//!
//! - Part 1 — strength of the strongest bridge (sum of all port types).
//! - Part 2 — strength of the strongest among the longest bridges.
//!
//! Both parts are solved by a single DFS over the component set, using a `u64`
//! bitmask to track used components (the input has 56 ≤ 64 components) and a
//! pre-built adjacency list keyed by port value for O(1) candidate lookup.

/// `adj[port]` holds `(component_idx, other_port)` for every component
/// touching `port`. A self-loop component `p/p` is recorded only once so
/// the DFS does not try it twice in opposite orientations.
#[derive(Debug, Clone)]
struct Components {
    adj: Vec<Vec<(u8, u8)>>,
}

fn parse_input(input: &str) -> Components {
    let parts: Vec<(u8, u8)> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (a, b) = line.split_once('/').expect("component has form a/b");
            (
                a.trim().parse().expect("port a"),
                b.trim().parse().expect("port b"),
            )
        })
        .collect();

    assert!(parts.len() <= 64, "bitmask only fits 64 components");

    let max_port = parts
        .iter()
        .flat_map(|&(a, b)| [a, b])
        .max()
        .unwrap_or(0) as usize;
    let mut adj: Vec<Vec<(u8, u8)>> = vec![Vec::new(); max_port + 1];
    for (i, &(a, b)) in parts.iter().enumerate() {
        adj[a as usize].push((i as u8, b));
        if a != b {
            adj[b as usize].push((i as u8, a));
        }
    }

    Components { adj }
}

/// Recursive DFS. Tracks the running `strength` and `length` and updates
/// `best_strength` (Part 1) and `best_long` = (longest_length, its strength)
/// for Part 2 at every reachable bridge state — including the empty bridge,
/// which is harmless because its strength and length are both 0.
fn dfs(
    comps: &Components,
    port: u8,
    used: u64,
    strength: u32,
    length: u32,
    best_strength: &mut u32,
    best_long: &mut (u32, u32),
) {
    if strength > *best_strength {
        *best_strength = strength;
    }
    if length > best_long.0 || (length == best_long.0 && strength > best_long.1) {
        *best_long = (length, strength);
    }

    for &(idx, other) in &comps.adj[port as usize] {
        let bit = 1u64 << idx;
        if used & bit != 0 {
            continue;
        }
        dfs(
            comps,
            other,
            used | bit,
            strength + port as u32 + other as u32,
            length + 1,
            best_strength,
            best_long,
        );
    }
}

/// Single DFS that produces both answers, used by `solve` to honour the
/// parse-once / search-once pattern.
fn search(comps: &Components) -> (u32, u32) {
    let mut best_strength = 0u32;
    let mut best_long = (0u32, 0u32);
    dfs(comps, 0, 0, 0, 0, &mut best_strength, &mut best_long);
    (best_strength, best_long.1)
}

fn solve_part1_with_data(comps: &Components) -> u32 {
    search(comps).0
}

fn solve_part2_with_data(comps: &Components) -> u32 {
    search(comps).1
}

pub fn solve(input: &str) -> (u32, u32) {
    let comps = parse_input(input);
    search(&comps)
}

pub fn solve_part1(input: &str) -> u32 {
    solve_part1_with_data(&parse_input(input))
}

pub fn solve_part2(input: &str) -> u32 {
    solve_part2_with_data(&parse_input(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10
";

    #[test]
    fn test_parse_example() {
        let comps = parse_input(EXAMPLE);
        // Port 0 reachable from component 0 (0/2) and component 5 (0/1).
        let mut from_zero: Vec<(u8, u8)> = comps.adj[0].clone();
        from_zero.sort();
        assert_eq!(from_zero, vec![(0, 2), (5, 1)]);
        // Self-loop 2/2 (idx 1) appears once, not twice, on port 2.
        let two_via_one = comps.adj[2].iter().filter(|&&(i, _)| i == 1).count();
        assert_eq!(two_via_one, 1);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 31);
    }

    #[test]
    fn test_part2_example() {
        // Longest bridges have length 4: 0/2--2/2--2/3--3/4 (strength 18)
        // and 0/2--2/2--2/3--3/5 (strength 19). Strongest of those is 19.
        assert_eq!(solve_part2(EXAMPLE), 19);
    }

    #[test]
    fn test_solve_example_combined() {
        assert_eq!(solve(EXAMPLE), (31, 19));
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day24.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, 1511);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day24.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, 1471);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day24.txt");
        assert_eq!(solve(input), (1511, 1471));
    }
}
