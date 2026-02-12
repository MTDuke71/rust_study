use std::collections::VecDeque;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Debug)]
pub struct HeightMap {
    grid: Vec<Vec<u8>>,
    start: Pos,
    end: Pos,
    rows: usize,
    cols: usize,
}

impl HeightMap {
    fn height_at(&self, pos: Pos) -> u8 {
        self.grid[pos.row][pos.col]
    }

    fn neighbors(&self, pos: Pos) -> Vec<Pos> {
        let mut result = Vec::new();
        let current_height = self.height_at(pos);

        // Up
        if pos.row > 0 {
            let next = Pos { row: pos.row - 1, col: pos.col };
            if self.height_at(next) <= current_height + 1 {
                result.push(next);
            }
        }
        // Down
        if pos.row < self.rows - 1 {
            let next = Pos { row: pos.row + 1, col: pos.col };
            if self.height_at(next) <= current_height + 1 {
                result.push(next);
            }
        }
        // Left
        if pos.col > 0 {
            let next = Pos { row: pos.row, col: pos.col - 1 };
            if self.height_at(next) <= current_height + 1 {
                result.push(next);
            }
        }
        // Right
        if pos.col < self.cols - 1 {
            let next = Pos { row: pos.row, col: pos.col + 1 };
            if self.height_at(next) <= current_height + 1 {
                result.push(next);
            }
        }

        result
    }

    // Reverse neighbors for backward search from E
    // When searching backward: can descend FROM by at most 1
    // (reverse of: can climb UP by at most 1)
    fn neighbors_reverse(&self, pos: Pos) -> Vec<Pos> {
        let mut result = Vec::new();
        let current_height = self.height_at(pos);

        // Up
        if pos.row > 0 {
            let next = Pos { row: pos.row - 1, col: pos.col };
            // Reverse constraint: current can descend TO next by at most 1
            // Which means: next can climb UP to current by at most 1
            if current_height <= self.height_at(next) + 1 {
                result.push(next);
            }
        }
        // Down
        if pos.row < self.rows - 1 {
            let next = Pos { row: pos.row + 1, col: pos.col };
            if current_height <= self.height_at(next) + 1 {
                result.push(next);
            }
        }
        // Left
        if pos.col > 0 {
            let next = Pos { row: pos.row, col: pos.col - 1 };
            if current_height <= self.height_at(next) + 1 {
                result.push(next);
            }
        }
        // Right
        if pos.col < self.cols - 1 {
            let next = Pos { row: pos.row, col: pos.col + 1 };
            if current_height <= self.height_at(next) + 1 {
                result.push(next);
            }
        }

        result
    }
}

pub fn parse_input(input: &str) -> HeightMap {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();
    
    let mut grid = vec![vec![0u8; cols]; rows];
    let mut start = Pos { row: 0, col: 0 };
    let mut end = Pos { row: 0, col: 0 };

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let height = match ch {
                'S' => {
                    start = Pos { row, col };
                    0 // 'a' = 0
                }
                'E' => {
                    end = Pos { row, col };
                    25 // 'z' = 25
                }
                'a'..='z' => ch as u8 - b'a',
                _ => panic!("Invalid character: {}", ch),
            };
            grid[row][col] = height;
        }
    }

    HeightMap { grid, start, end, rows, cols }
}

fn bfs_shortest_path(map: &HeightMap, start: Pos, end: Pos) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; map.cols]; map.rows];
    
    queue.push_back((start, 0));
    visited[start.row][start.col] = true;

    while let Some((pos, dist)) = queue.pop_front() {
        if pos == end {
            return Some(dist);
        }

        for next in map.neighbors(pos) {
            if !visited[next.row][next.col] {
                visited[next.row][next.col] = true;
                queue.push_back((next, dist + 1));
            }
        }
    }

    None
}

// Backward BFS from E to find closest 'a' elevation
pub fn bfs_backward_to_any_a(map: &HeightMap) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; map.cols]; map.rows];
    
    queue.push_back((map.end, 0));
    visited[map.end.row][map.end.col] = true;

    while let Some((pos, dist)) = queue.pop_front() {
        // Check if we reached any 'a' elevation
        if map.height_at(pos) == 0 {
            return dist;
        }

        for next in map.neighbors_reverse(pos) {
            if !visited[next.row][next.col] {
                visited[next.row][next.col] = true;
                queue.push_back((next, dist + 1));
            }
        }
    }

    panic!("No path from E to any 'a' position")
}

fn solve_part1_with_data(map: &HeightMap) -> usize {
    bfs_shortest_path(map, map.start, map.end)
        .expect("Path should exist from S to E")
}

pub fn solve_part2_with_data(map: &HeightMap) -> usize {
    // Find shortest path from ANY 'a' elevation to E (sequential)
    let mut min_steps = usize::MAX;

    for row in 0..map.rows {
        for col in 0..map.cols {
            if map.grid[row][col] == 0 {  // elevation 'a'
                let start = Pos { row, col };
                if let Some(steps) = bfs_shortest_path(map, start, map.end) {
                    min_steps = min_steps.min(steps);
                }
            }
        }
    }

    min_steps
}

pub fn solve_part2_parallel(map: &HeightMap) -> usize {
    // Collect all 'a' elevation positions
    let start_positions: Vec<Pos> = (0..map.rows)
        .flat_map(|row| {
            (0..map.cols).filter_map(move |col| {
                if map.grid[row][col] == 0 {
                    Some(Pos { row, col })
                } else {
                    None
                }
            })
        })
        .collect();

    // Run BFS from each 'a' position in parallel and find minimum
    start_positions
        .par_iter()
        .filter_map(|&start| bfs_shortest_path(map, start, map.end))
        .min()
        .expect("At least one path should exist")
}

pub fn solve(input: &str) -> (usize, usize) {
    let map = parse_input(input);
    let part1 = solve_part1_with_data(&map);
    let part2 = bfs_backward_to_any_a(&map);  // Use backward BFS (fastest!)
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_parse() {
        let map = parse_input(EXAMPLE);
        assert_eq!(map.rows, 5);
        assert_eq!(map.cols, 8);
        assert_eq!(map.start, Pos { row: 0, col: 0 });
        assert_eq!(map.end, Pos { row: 2, col: 5 });
        assert_eq!(map.height_at(map.start), 0); // 'a'
        assert_eq!(map.height_at(map.end), 25);   // 'z'
    }

    #[test]
    fn test_part1_example() {
        let map = parse_input(EXAMPLE);
        assert_eq!(solve_part1_with_data(&map), 31);
    }

    #[test]
    fn test_part2_example() {
        let map = parse_input(EXAMPLE);
        assert_eq!(solve_part2_with_data(&map), 29);
    }

    #[test]
    fn test_part2_parallel_example() {
        let map = parse_input(EXAMPLE);
        assert_eq!(solve_part2_parallel(&map), 29);
    }

    #[test]
    fn test_part2_backward_example() {
        let map = parse_input(EXAMPLE);
        assert_eq!(bfs_backward_to_any_a(&map), 29);
    }
}
