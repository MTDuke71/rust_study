//! # Day 11: Cosmic Expansion
//!
//! ## Part 1
//! Find the sum of shortest paths between all pairs of galaxies after cosmic expansion.
//! Empty rows and columns expand by 2x (become twice as big).
//!
//! ## Part 2
//! Same problem but expansion factor is 1,000,000x instead of 2x.
//!
//! ## Algorithm
//! - Parse: Find all galaxy positions and identify empty rows/columns
//! - Expand: Track expansion offsets for each row/column
//! - Distance: Manhattan distance with expansion offset applied
//! - Sum: All pairs of galaxies (combination)
//! - Complexity: O(n*m + g²) where n×m is grid size, g is galaxy count
//!
//! ## Optimizations
//! - Mission integration: Could use Mission 6 Grid, but simple Vec<Vec<char>> suffices
//! - Rust-specific: Track expansion offset instead of creating expanded grid

use anyhow::Result;

type Position = (usize, usize);

fn parse_galaxies(input: &str) -> Vec<Position> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(col, _)| (row, col))
        })
        .collect()
}

fn find_empty_rows(galaxies: &[Position], max_row: usize) -> Vec<usize> {
    let occupied_rows: std::collections::HashSet<_> = galaxies.iter().map(|(r, _)| *r).collect();
    (0..=max_row)
        .filter(|r| !occupied_rows.contains(r))
        .collect()
}

fn find_empty_cols(galaxies: &[Position], max_col: usize) -> Vec<usize> {
    let occupied_cols: std::collections::HashSet<_> = galaxies.iter().map(|(_, c)| *c).collect();
    (0..=max_col)
        .filter(|c| !occupied_cols.contains(c))
        .collect()
}

fn calculate_distance_with_expansion(
    pos1: Position,
    pos2: Position,
    empty_rows: &[usize],
    empty_cols: &[usize],
    expansion_factor: usize,
) -> usize {
    let (r1, c1) = pos1;
    let (r2, c2) = pos2;

    let min_row = r1.min(r2);
    let max_row = r1.max(r2);
    let min_col = c1.min(c2);
    let max_col = c1.max(c2);

    // Count empty rows between the two galaxies
    let empty_rows_between = empty_rows
        .iter()
        .filter(|&&r| r > min_row && r < max_row)
        .count();

    // Count empty columns between the two galaxies
    let empty_cols_between = empty_cols
        .iter()
        .filter(|&&c| c > min_col && c < max_col)
        .count();

    // Base Manhattan distance
    let base_distance = (max_row - min_row) + (max_col - min_col);

    // Each empty row/col expands by (expansion_factor - 1) additional units
    // For Part 1: expansion_factor = 2 (doubles), so adds 1 extra per empty row/col
    // For Part 2: expansion_factor = 1_000_000, so adds 999_999 extra per empty row/col
    let expansion_offset =
        empty_rows_between * (expansion_factor - 1) + empty_cols_between * (expansion_factor - 1);

    base_distance + expansion_offset
}

fn solve_with_expansion(input: &str, expansion_factor: usize) -> Result<String> {
    let galaxies = parse_galaxies(input);

    let max_row = input.lines().count() - 1;
    let max_col = input.lines().next().unwrap_or("").len() - 1;

    let empty_rows = find_empty_rows(&galaxies, max_row);
    let empty_cols = find_empty_cols(&galaxies, max_col);

    // Calculate sum of all pairwise distances
    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let dist = calculate_distance_with_expansion(
                galaxies[i],
                galaxies[j],
                &empty_rows,
                &empty_cols,
                expansion_factor,
            );
            total += dist;
        }
    }

    Ok(total.to_string())
}

pub fn solve_part1(input: &str) -> Result<String> {
    solve_with_expansion(input, 2)
}

pub fn solve_part2(input: &str) -> Result<String> {
    solve_with_expansion(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, "374");
    }

    #[test]
    fn test_part2_expansion_10x() {
        // Example says with expansion factor 10, answer is 1030
        let result = solve_with_expansion(EXAMPLE, 10).unwrap();
        assert_eq!(result, "1030");
    }

    #[test]
    fn test_part2_expansion_100x() {
        // Example says with expansion factor 100, answer is 8410
        let result = solve_with_expansion(EXAMPLE, 100).unwrap();
        assert_eq!(result, "8410");
    }

    #[test]
    fn test_parse_galaxies() {
        let galaxies = parse_galaxies(EXAMPLE);
        assert_eq!(galaxies.len(), 9);
        assert!(galaxies.contains(&(0, 3)));
        assert!(galaxies.contains(&(1, 7)));
    }

    #[test]
    fn test_find_empty_rows() {
        let galaxies = parse_galaxies(EXAMPLE);
        let empty_rows = find_empty_rows(&galaxies, 9);
        assert_eq!(empty_rows, vec![3, 7]);
    }

    #[test]
    fn test_find_empty_cols() {
        let galaxies = parse_galaxies(EXAMPLE);
        let empty_cols = find_empty_cols(&galaxies, 9);
        assert_eq!(empty_cols, vec![2, 5, 8]);
    }
}
