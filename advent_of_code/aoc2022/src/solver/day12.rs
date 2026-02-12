use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct HeightMap {
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
}

fn parse_input(input: &str) -> HeightMap {
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

fn solve_part1_with_data(map: &HeightMap) -> usize {
    bfs_shortest_path(map, map.start, map.end)
        .expect("Path should exist from S to E")
}

fn solve_part2_with_data(map: &HeightMap) -> usize {
    // Find shortest path from ANY 'a' elevation to E
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

pub fn solve(input: &str) -> (usize, usize) {
    let map = parse_input(input);
    let part1 = solve_part1_with_data(&map);
    let part2 = solve_part2_with_data(&map);
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
}
