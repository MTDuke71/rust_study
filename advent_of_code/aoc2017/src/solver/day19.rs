//! Day 19: A Series of Tubes
//!
//! Follow an ASCII-art packet maze starting at the only `|` in the top row,
//! heading down. `|` and `-` continue straight, `+` turns 90° toward the
//! only perpendicular non-space neighbour, letters A–Z are collected in
//! visit order, and a space ends the journey.
//!
//! - **Part 1**: letters collected, in order.
//! - **Part 2**: total number of cells visited (including start).

use mission6::{Coord, Grid};

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn offset(self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }

    fn perpendicular(self) -> [Dir; 2] {
        match self {
            Dir::Up | Dir::Down => [Dir::Left, Dir::Right],
            Dir::Left | Dir::Right => [Dir::Up, Dir::Down],
        }
    }
}

fn parse_input(input: &str) -> Grid<u8> {
    let normalized = input.replace("\r\n", "\n");
    let lines: Vec<&str> = normalized.lines().collect();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let rows: Vec<Vec<u8>> = lines
        .iter()
        .map(|l| {
            let mut row: Vec<u8> = l.bytes().collect();
            row.resize(width, b' ');
            row
        })
        .collect();

    Grid::from_vec2d(rows)
}

fn step(pos: Coord, dir: Dir, grid: &Grid<u8>) -> Option<Coord> {
    let (dx, dy) = dir.offset();
    let nx = pos.x as isize + dx;
    let ny = pos.y as isize + dy;
    if nx < 0 || ny < 0 {
        return None;
    }
    let next = Coord::new(nx as usize, ny as usize);
    match grid.get(next) {
        Some(&b) if b != b' ' => Some(next),
        _ => None,
    }
}

fn walk(grid: &Grid<u8>) -> (String, usize) {
    // Start: the only `|` in the top row (y = 0).
    let start_x = grid
        .row(0)
        .expect("grid has rows")
        .position(|&b| b == b'|')
        .expect("top row contains a '|' entry point");
    let mut pos = Coord::new(start_x, 0);
    let mut dir = Dir::Down;

    let mut letters = String::new();
    let mut steps = 1usize; // starting cell counts

    loop {
        let tile = *grid.get(pos).expect("position is in bounds");
        match tile {
            b'+' => {
                dir = dir
                    .perpendicular()
                    .into_iter()
                    .find(|&d| step(pos, d, grid).is_some())
                    .expect("corner must have one perpendicular exit");
            }
            c if c.is_ascii_uppercase() => letters.push(c as char),
            _ => {} // '|' or '-' — keep going
        }

        match step(pos, dir, grid) {
            Some(next) => {
                pos = next;
                steps += 1;
            }
            None => break,
        }
    }

    (letters, steps)
}

pub fn solve(input: &str) -> (String, usize) {
    let grid = parse_input(input);
    walk(&grid)
}

pub fn solve_part1(input: &str) -> String {
    solve(input).0
}

pub fn solve_part2(input: &str) -> usize {
    solve(input).1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = concat!(
        "     |          \n",
        "     |  +--+    \n",
        "     A  |  C    \n",
        " F---|----E|--+ \n",
        "     |  |  |  D \n",
        "     +B-+  +--+ \n",
    );

    #[test]
    fn test_parse() {
        let grid = parse_input(EXAMPLE);
        assert_eq!(grid.height(), 6);
        assert_eq!(grid.width(), 16);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), "ABCDEF");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 38);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day19.txt");
        assert_eq!(solve_part1(input), "BPDKCZWHGT");
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day19.txt");
        assert_eq!(solve_part2(input), 17728);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day19.txt");
        let (p1, p2) = solve(input);
        assert_eq!((p1.as_str(), p2), ("BPDKCZWHGT", 17728));
    }
}
