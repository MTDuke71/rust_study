//! Day 22: Monkey Map
//!
//! Board navigation with wrapping.
//! Part 1: Flat wrap (opposite edge of same row/col).
//! Part 2: Cube wrap (fold the net into a cube, hardcoded for actual input's net shape).

use std::collections::HashMap;

// ============================================================================
// Data Types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
}

#[derive(Debug, Clone)]
enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}

/// Facing direction: 0=Right, 1=Down, 2=Left, 3=Up
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Facing(u8);

impl Facing {
    const RIGHT: Facing = Facing(0);
    const DOWN: Facing = Facing(1);
    const LEFT: Facing = Facing(2);
    const UP: Facing = Facing(3);

    fn turn_right(self) -> Facing {
        Facing((self.0 + 1) % 4)
    }

    fn turn_left(self) -> Facing {
        Facing((self.0 + 3) % 4)
    }

    fn delta(self) -> (i32, i32) {
        match self.0 {
            0 => (0, 1),  // Right: col+1
            1 => (1, 0),  // Down: row+1
            2 => (0, -1), // Left: col-1
            3 => (-1, 0), // Up: row-1
            _ => unreachable!(),
        }
    }
}

pub struct Board {
    tiles: HashMap<(i32, i32), Tile>,
    row_ranges: HashMap<i32, (i32, i32)>, // row → (min_col, max_col)
    col_ranges: HashMap<i32, (i32, i32)>, // col → (min_row, max_row)
    instructions: Vec<Instruction>,
}

pub type ParsedData = Board;

// ============================================================================
// Parsing
// ============================================================================

pub fn parse_input(input: &str) -> ParsedData {
    let input = input.replace("\r\n", "\n");
    let parts: Vec<&str> = input.split("\n\n").collect();
    let map_str = parts[0];
    let path_str = parts[1].trim();

    let mut tiles = HashMap::new();
    let mut row_ranges: HashMap<i32, (i32, i32)> = HashMap::new();
    let mut col_ranges: HashMap<i32, (i32, i32)> = HashMap::new();

    for (r, line) in map_str.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            let tile = match ch {
                '.' => Tile::Open,
                '#' => Tile::Wall,
                _ => continue,
            };
            let row = r as i32;
            let col = c as i32;
            tiles.insert((row, col), tile);

            let entry = row_ranges.entry(row).or_insert((col, col));
            entry.0 = entry.0.min(col);
            entry.1 = entry.1.max(col);

            let entry = col_ranges.entry(col).or_insert((row, row));
            entry.0 = entry.0.min(row);
            entry.1 = entry.1.max(row);
        }
    }

    // Parse path instructions
    let mut instructions = Vec::new();
    let mut num_start = None;
    for (i, ch) in path_str.char_indices() {
        if ch.is_ascii_digit() {
            if num_start.is_none() {
                num_start = Some(i);
            }
        } else {
            if let Some(start) = num_start {
                instructions.push(Instruction::Move(path_str[start..i].parse().unwrap()));
                num_start = None;
            }
            match ch {
                'L' => instructions.push(Instruction::TurnLeft),
                'R' => instructions.push(Instruction::TurnRight),
                _ => {}
            }
        }
    }
    if let Some(start) = num_start {
        instructions.push(Instruction::Move(path_str[start..].parse().unwrap()));
    }

    Board {
        tiles,
        row_ranges,
        col_ranges,
        instructions,
    }
}

// ============================================================================
// Shared helpers
// ============================================================================

/// Find the leftmost open tile on row 0.
fn find_start(board: &Board) -> (i32, i32) {
    let (min_col, max_col) = board.row_ranges[&0];
    for col in min_col..=max_col {
        if board.tiles.get(&(0, col)) == Some(&Tile::Open) {
            return (0, col);
        }
    }
    panic!("No open tile on row 0");
}

// ============================================================================
// Part 1: Flat wrapping
// ============================================================================

pub fn solve_part1(board: &ParsedData) -> i64 {
    let (mut row, mut col) = find_start(board);
    let mut facing = Facing::RIGHT;

    for instr in &board.instructions {
        match instr {
            Instruction::TurnLeft => facing = facing.turn_left(),
            Instruction::TurnRight => facing = facing.turn_right(),
            Instruction::Move(steps) => {
                let (dr, dc) = facing.delta();
                for _ in 0..*steps {
                    let mut nr = row + dr;
                    let mut nc = col + dc;

                    // Flat wrap if off the map
                    if !board.tiles.contains_key(&(nr, nc)) {
                        match facing.0 {
                            0 => nc = board.row_ranges[&row].0, // right → left edge
                            2 => nc = board.row_ranges[&row].1, // left → right edge
                            1 => nr = board.col_ranges[&col].0, // down → top edge
                            3 => nr = board.col_ranges[&col].1, // up → bottom edge
                            _ => unreachable!(),
                        }
                    }

                    match board.tiles[&(nr, nc)] {
                        Tile::Wall => break,
                        Tile::Open => {
                            row = nr;
                            col = nc;
                        }
                    }
                }
            }
        }
    }

    1000 * (row as i64 + 1) + 4 * (col as i64 + 1) + facing.0 as i64
}

// ============================================================================
// Part 2: Cube wrapping (hardcoded for actual input's net shape)
// ============================================================================
//
// Actual input cube net (face size 50):
//
//       ┌───┬───┐
//       │ 1 │ 2 │     rows   0- 49, cols  50-149
//       ├───┼───┘
//       │ 3 │           rows  50- 99, cols  50- 99
//   ┌───┼───┤
//   │ 4 │ 5 │           rows 100-149, cols   0- 99
//   ├───┼───┘
//   │ 6 │               rows 150-199, cols   0- 49
//   └───┘
//
// 14 non-adjacent edge transitions (adjacent faces use natural steps):

/// Map an out-of-bounds step onto the correct cube face.
/// `(row, col)` is one step past the edge; `facing` is the direction of travel.
/// Returns `(new_row, new_col, new_facing)` on the destination face.
fn cube_wrap(row: i32, col: i32, facing: Facing) -> (i32, i32, Facing) {
    match (row, col, facing.0) {
        // Face 1 left  → Face 4 left (reversed)
        (0..=49, 49, 2) => (149 - row, 0, Facing::RIGHT),

        // Face 1 top   → Face 6 left
        (-1, 50..=99, 3) => (col + 100, 0, Facing::RIGHT),

        // Face 2 top   → Face 6 bottom
        (-1, 100..=149, 3) => (199, col - 100, Facing::UP),

        // Face 2 right → Face 5 right (reversed)
        (0..=49, 150, 0) => (149 - row, 99, Facing::LEFT),

        // Face 2 bottom → Face 3 right
        (50, 100..=149, 1) => (col - 50, 99, Facing::LEFT),

        // Face 3 left  → Face 4 top
        (50..=99, 49, 2) => (100, row - 50, Facing::DOWN),

        // Face 3 right → Face 2 bottom
        (50..=99, 100, 0) => (49, row + 50, Facing::UP),

        // Face 4 left  → Face 1 left (reversed)
        (100..=149, -1, 2) => (149 - row, 50, Facing::RIGHT),

        // Face 4 top   → Face 3 left
        (99, 0..=49, 3) => (col + 50, 50, Facing::RIGHT),

        // Face 5 right → Face 2 right (reversed)
        (100..=149, 100, 0) => (149 - row, 149, Facing::LEFT),

        // Face 5 bottom → Face 6 right
        (150, 50..=99, 1) => (col + 100, 49, Facing::LEFT),

        // Face 6 left  → Face 1 top
        (150..=199, -1, 2) => (0, row - 100, Facing::DOWN),

        // Face 6 right → Face 5 bottom
        (150..=199, 50, 0) => (149, row - 100, Facing::UP),

        // Face 6 bottom → Face 2 top
        (200, 0..=49, 1) => (0, col + 100, Facing::DOWN),

        _ => panic!(
            "Unexpected cube wrap: row={row}, col={col}, facing={}",
            facing.0
        ),
    }
}

pub fn solve_part2(board: &ParsedData) -> i64 {
    let (mut row, mut col) = find_start(board);
    let mut facing = Facing::RIGHT;

    for instr in &board.instructions {
        match instr {
            Instruction::TurnLeft => facing = facing.turn_left(),
            Instruction::TurnRight => facing = facing.turn_right(),
            Instruction::Move(steps) => {
                for _ in 0..*steps {
                    let (dr, dc) = facing.delta();
                    let nr = row + dr;
                    let nc = col + dc;

                    let (fr, fc, ff) = if board.tiles.contains_key(&(nr, nc)) {
                        (nr, nc, facing)
                    } else {
                        cube_wrap(nr, nc, facing)
                    };

                    match board.tiles[&(fr, fc)] {
                        Tile::Wall => break,
                        Tile::Open => {
                            row = fr;
                            col = fc;
                            facing = ff;
                        }
                    }
                }
            }
        }
    }

    1000 * (row as i64 + 1) + 4 * (col as i64 + 1) + facing.0 as i64
}

// ============================================================================
// Entry point
// ============================================================================

pub fn solve(input: &str) -> (i64, i64) {
    let board = parse_input(input);
    (solve_part1(&board), solve_part2(&board))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_parse() {
        let board = parse_input(EXAMPLE);
        // 4 rows × 4 cols + 4 rows × 12 cols + 4 rows × 8 cols = 16 + 48 + 32 = 96
        assert_eq!(board.tiles.len(), 96);
        assert_eq!(board.row_ranges[&0], (8, 11));
        assert_eq!(board.row_ranges[&4], (0, 11));
        // 7 moves + 6 turns = 13 instructions
        assert_eq!(board.instructions.len(), 13);
    }

    #[test]
    fn test_part1_example() {
        let board = parse_input(EXAMPLE);
        assert_eq!(solve_part1(&board), 6032);
    }

    // Part 2 cube wrapping is hardcoded for the actual input's net shape,
    // so no example test for Part 2.

    #[test]
    fn test_actual() {
        let input = include_str!("../../inputs/day22.txt");
        assert_eq!(solve(input), (88_226, 57_305));
    }
}
