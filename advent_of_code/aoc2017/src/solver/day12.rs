//! Day 12: Digital Plumber
//!
//! Connected components via Union-Find (Mission 10).

use mission10::UnionFind;

fn build_uf(input: &str) -> UnionFind {
    let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();
    let mut uf = UnionFind::new(lines.len());

    for line in &lines {
        let (lhs, rhs) = line.split_once(" <-> ").expect("invalid line format");
        let id: usize = lhs.trim().parse().expect("invalid program id");
        for peer in rhs.split(',') {
            let peer_id: usize = peer.trim().parse().expect("invalid peer id");
            uf.union(id, peer_id).expect("union failed");
        }
    }

    uf
}

fn solve_both(input: &str) -> (usize, usize) {
    let mut uf = build_uf(input);
    let group_zero = uf.members(0).expect("id 0 exists").count();
    let total_groups = uf.components().count();
    (group_zero, total_groups)
}

pub fn solve(input: &str) -> (usize, usize) {
    solve_both(input)
}

pub fn solve_part1(input: &str) -> usize {
    solve_both(input).0
}

pub fn solve_part2(input: &str) -> usize {
    solve_both(input).1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
";

    #[test]
    fn test_example_part1() {
        assert_eq!(solve_part1(EXAMPLE), 6);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve_part2(EXAMPLE), 2);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day12.txt");
        assert_eq!(solve_part1(input), 134);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day12.txt");
        assert_eq!(solve_part2(input), 193);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day12.txt");
        assert_eq!(solve(input), (134, 193));
    }
}
