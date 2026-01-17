//! Day 16: The Floor Will Be Lava
//!
//! Beam tracing through a grid of mirrors and splitters.

use mission6::{Coord, Grid};
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),    // y decreases
            Direction::Down => (0, 1),   // y increases
            Direction::Left => (-1, 0),  // x decreases
            Direction::Right => (1, 0),  // x increases
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,          // '.'
    MirrorForward,  // '/'
    MirrorBackward, // '\'
    SplitterVert,   // '|'
    SplitterHoriz,  // '-'
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '/' => Tile::MirrorForward,
            '\\' => Tile::MirrorBackward,
            '|' => Tile::SplitterVert,
            '-' => Tile::SplitterHoriz,
            _ => panic!("Unknown tile: {}", c),
        }
    }
}

fn parse_grid(input: &str) -> Grid<Tile> {
    let tiles: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect();

    Grid::from_vec2d(tiles)
}

fn trace_beam(grid: &Grid<Tile>, start_pos: Coord, start_dir: Direction) -> usize {
    let mut seen_states = HashSet::new();
    let mut energized = HashSet::new();
    let mut beams = vec![(start_pos, start_dir)];

    while let Some((pos, dir)) = beams.pop() {
        // Check if we've seen this exact state before (position + direction)
        if !seen_states.insert((pos, dir)) {
            continue;
        }

        // Check bounds using Mission 6's in_bounds
        if !grid.in_bounds(pos) {
            continue;
        }

        energized.insert(pos);

        let tile = *grid.get(pos).unwrap();

        // Determine next direction(s) based on current tile and direction
        let next_dirs = match (tile, dir) {
            // Empty tiles - beam continues straight
            (Tile::Empty, _) => vec![dir],

            // Forward mirror '/' - reflects beam
            (Tile::MirrorForward, Direction::Up) => vec![Direction::Right],
            (Tile::MirrorForward, Direction::Down) => vec![Direction::Left],
            (Tile::MirrorForward, Direction::Left) => vec![Direction::Down],
            (Tile::MirrorForward, Direction::Right) => vec![Direction::Up],

            // Backward mirror '\' - reflects beam
            (Tile::MirrorBackward, Direction::Up) => vec![Direction::Left],
            (Tile::MirrorBackward, Direction::Down) => vec![Direction::Right],
            (Tile::MirrorBackward, Direction::Left) => vec![Direction::Up],
            (Tile::MirrorBackward, Direction::Right) => vec![Direction::Down],

            // Vertical splitter '|' - splits if horizontal
            (Tile::SplitterVert, Direction::Up | Direction::Down) => vec![dir],
            (Tile::SplitterVert, Direction::Left | Direction::Right) => {
                vec![Direction::Up, Direction::Down]
            }

            // Horizontal splitter '-' - splits if vertical
            (Tile::SplitterHoriz, Direction::Left | Direction::Right) => vec![dir],
            (Tile::SplitterHoriz, Direction::Up | Direction::Down) => {
                vec![Direction::Left, Direction::Right]
            }
        };

        // Add new beams for each direction
        for next_dir in next_dirs {
            let (dx, dy) = next_dir.offset();
            // Mission 6 Coord uses usize, so we need to handle signed arithmetic carefully
            let next_x = pos.x as isize + dx;
            let next_y = pos.y as isize + dy;
            
            if next_x >= 0 && next_y >= 0 {
                let next_pos = Coord::new(next_x as usize, next_y as usize);
                beams.push((next_pos, next_dir));
            }
        }
    }

    energized.len()
}

pub fn solve_part1(input: &str) -> usize {
    let grid = parse_grid(input);
    trace_beam(&grid, Coord::new(0, 0), Direction::Right)
}

pub fn solve_part2(input: &str) -> usize {
    let grid = parse_grid(input);

    let width = grid.width();
    let height = grid.height();

    // Generate all possible starting positions (edges of grid)
    let mut starts = Vec::new();

    // Top and bottom edges
    for x in 0..width {
        starts.push((Coord::new(x, 0), Direction::Down));           // Top edge going down
        starts.push((Coord::new(x, height - 1), Direction::Up));    // Bottom edge going up
    }

    // Left and right edges
    for y in 0..height {
        starts.push((Coord::new(0, y), Direction::Right));          // Left edge going right
        starts.push((Coord::new(width - 1, y), Direction::Left));   // Right edge going left
    }

    // Use Rayon to parallelize the traces - 11.67× speedup!
    // Original: 257.68ms sequential → 22.08ms parallel
    starts
        .par_iter()
        .map(|&(pos, dir)| trace_beam(&grid, pos, dir))
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 46);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 51);
    }

    #[test]
    fn test_part1_solution() {
        let input = include_str!("../../inputs/day16.txt");
        let result = solve_part1(input);
        println!("Part 1: {}", result);
    }

    #[test]
    fn test_part2_solution() {
        let input = include_str!("../../inputs/day16.txt");
        let result = solve_part2(input);
        println!("Part 2: {}", result);
    }
}
