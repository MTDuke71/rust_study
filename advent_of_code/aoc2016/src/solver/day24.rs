//! Day 24: Air Duct Spelunking
//!
//! BFS pairwise distances between numbered points on a maze grid,
//! then brute-force TSP over permutations (few points).
//! Part 1: shortest route from 0 visiting all. Part 2: return to 0.

use std::collections::VecDeque;

/// Parsed maze data: pairwise shortest distances between numbered points.
struct Maze {
    /// dist[i][j] = shortest path between point i and point j
    dist: Vec<Vec<u32>>,
    /// Number of numbered points (0..n-1)
    n: usize,
}

fn parse_input(input: &str) -> Maze {
    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    // Find all numbered points, indexed by digit value
    let mut points: Vec<(usize, usize)> = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell.is_ascii_digit() {
                let digit = (cell - b'0') as usize;
                if digit >= points.len() {
                    points.resize(digit + 1, (0, 0));
                }
                points[digit] = (r, c);
            }
        }
    }

    let n = points.len();

    // BFS from each numbered point to compute pairwise distances
    let mut dist = vec![vec![0u32; n]; n];
    for i in 0..n {
        let distances = bfs(&grid, rows, cols, points[i]);
        for j in 0..n {
            dist[i][j] = distances[points[j].0][points[j].1];
        }
    }

    Maze { dist, n }
}

fn solve_part1_with_data(maze: &Maze) -> u32 {
    tsp(maze, false)
}

fn solve_part2_with_data(maze: &Maze) -> u32 {
    tsp(maze, true)
}

/// Brute-force TSP: try all permutations of points 1..n starting from 0.
/// If `return_home`, add cost of returning to 0.
fn tsp(maze: &Maze, return_home: bool) -> u32 {
    let mut others: Vec<usize> = (1..maze.n).collect();
    let mut best = u32::MAX;

    loop {
        let mut cost = maze.dist[0][others[0]];
        for w in others.windows(2) {
            cost += maze.dist[w[0]][w[1]];
        }
        if return_home {
            cost += maze.dist[*others.last().unwrap()][0];
        }
        best = best.min(cost);

        if !next_permutation(&mut others) {
            break;
        }
    }

    best
}

fn bfs(grid: &[&[u8]], rows: usize, cols: usize, start: (usize, usize)) -> Vec<Vec<u32>> {
    let mut dist = vec![vec![u32::MAX; cols]; rows];
    dist[start.0][start.1] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some((r, c)) = queue.pop_front() {
        let d = dist[r][c] + 1;
        for (dr, dc) in [(!0usize, 0), (1, 0), (0, !0usize), (0, 1)] {
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if nr < rows && nc < cols && grid[nr][nc] != b'#' && dist[nr][nc] == u32::MAX {
                dist[nr][nc] = d;
                queue.push_back((nr, nc));
            }
        }
    }

    dist
}

fn next_permutation(arr: &mut [usize]) -> bool {
    let n = arr.len();
    if n <= 1 {
        return false;
    }
    let mut i = n - 1;
    while i > 0 && arr[i - 1] >= arr[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }
    let mut j = n - 1;
    while arr[j] <= arr[i - 1] {
        j -= 1;
    }
    arr.swap(i - 1, j);
    arr[i..].reverse();
    true
}

pub fn solve(input: &str) -> (String, String) {
    let maze = parse_input(input);
    (
        solve_part1_with_data(&maze).to_string(),
        solve_part2_with_data(&maze).to_string(),
    )
}

pub fn solve_part1(input: &str) -> u32 {
    let maze = parse_input(input);
    solve_part1_with_data(&maze)
}

pub fn solve_part2(input: &str) -> u32 {
    let maze = parse_input(input);
    solve_part2_with_data(&maze)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
###########
#0.1.....2#
#.#######.#
#4.......3#
###########";

    #[test]
    fn test_parse() {
        let maze = parse_input(EXAMPLE);
        assert_eq!(maze.n, 5);
        assert_eq!(maze.dist[0][4], 2); // 0 to 4: 2 steps
        assert_eq!(maze.dist[4][1], 4); // 4 to 1: 4 steps
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 14);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day24.txt");
        assert_eq!(solve_part1(input), 500);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day24.txt");
        assert_eq!(solve_part2(input), 748);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day24.txt");
        assert_eq!(solve(input), ("500".to_string(), "748".to_string()));
    }
}
