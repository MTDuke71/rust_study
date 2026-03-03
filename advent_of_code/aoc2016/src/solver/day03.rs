//! Day 3: Squares With Three Sides
//!
//! Check which triangles are valid: the sum of any two sides must
//! exceed the third. Direct 3-check with short-circuit beats sort.

fn parse_input(input: &str) -> Vec<[u32; 3]> {
    input
        .lines()
        .map(|line| {
            let nums: Vec<u32> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();
            [nums[0], nums[1], nums[2]]
        })
        .collect()
}

fn is_valid_triangle([a, b, c]: [u32; 3]) -> bool {
    a + b > c && a + c > b && b + c > a
}

/// Sort-based variant: sort ascending, then smallest + middle > largest.
/// Exported for benchmark comparison against the direct 3-check approach.
pub fn is_valid_triangle_sort([a, b, c]: [u32; 3]) -> bool {
    let mut s = [a, b, c];
    s.sort();
    s[0] + s[1] > s[2]
}

fn solve_part1_with_data(triangles: &[[u32; 3]]) -> usize {
    triangles.iter().copied().filter(|t| is_valid_triangle(*t)).count()
}

/// Part 1 using sort-based validator (for benchmark comparison).
pub fn solve_part1_sort(input: &str) -> String {
    let data = parse_input(input);
    data.iter()
        .copied()
        .filter(|t| is_valid_triangle_sort(*t))
        .count()
        .to_string()
}

/// Re-read triangles vertically: each group of 3 rows yields 3 triangles
/// from columns 0, 1, 2.
fn solve_part2_with_data(rows: &[[u32; 3]]) -> usize {
    rows.chunks(3)
        .flat_map(|chunk| {
            (0..3).map(move |col| [chunk[0][col], chunk[1][col], chunk[2][col]])
        })
        .filter(|t| is_valid_triangle(*t))
        .count()
}

pub fn solve(input: &str) -> (String, String) {
    let data = parse_input(input);
    let p1 = solve_part1_with_data(&data);
    let p2 = solve_part2_with_data(&data);
    (p1.to_string(), p2.to_string())
}

pub fn solve_part1(input: &str) -> String {
    solve_part1_with_data(&parse_input(input)).to_string()
}

pub fn solve_part2(input: &str) -> String {
    solve_part2_with_data(&parse_input(input)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_triangle() {
        assert!(!is_valid_triangle([5, 10, 25]));
    }

    #[test]
    fn test_valid_triangle() {
        assert!(is_valid_triangle([3, 4, 5]));
    }

    #[test]
    fn test_part2_vertical() {
        // 3 rows → read columns: (1,4,7), (2,5,8), (3,6,9)
        let input = "1 2 3\n4 5 6\n7 8 9";
        let data = parse_input(input);
        let triangles: Vec<_> = data
            .chunks(3)
            .flat_map(|chunk| (0..3).map(move |col| [chunk[0][col], chunk[1][col], chunk[2][col]]))
            .collect();
        assert_eq!(triangles, vec![[1, 4, 7], [2, 5, 8], [3, 6, 9]]);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day03.txt");
        assert_eq!(solve_part1(input), "993");
    }

    #[test]
    fn test_sort_vs_direct_equivalence() {
        let input = include_str!("../../inputs/day03.txt");
        assert_eq!(solve_part1(input), solve_part1_sort(input));
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day03.txt");
        assert_eq!(solve(input), ("993".to_string(), "1849".to_string()));
    }
}
