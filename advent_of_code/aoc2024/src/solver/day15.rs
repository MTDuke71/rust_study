//! Day 15: Warehouse Woes - Robot box-pushing simulation
//!
//! Uses Mission 6's `Grid<T>` for warehouse layout
//! Part 1: Single-width boxes
//! Part 2: Double-width boxes (everything except robot is 2x wide)

use anyhow::Result;
use mission6::{Coord, Grid};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Box,
    Robot,
    Empty,
    BoxLeft,  // For Part 2: [
    BoxRight, // For Part 2: ]
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            'O' => Tile::Box,
            '@' => Tile::Robot,
            '.' => Tile::Empty,
            '[' => Tile::BoxLeft,
            ']' => Tile::BoxRight,
            _ => Tile::Empty,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

fn parse_input(input: &str) -> Result<(Grid<Tile>, Vec<Direction>, Coord)> {
    // Split on double newlines (works with both \n\n and \r\n\r\n)
    let parts: Vec<&str> = input.split("\n\n").collect();
    if parts.len() < 2 {
        // Try Windows line endings
        let parts: Vec<&str> = input.split("\r\n\r\n").collect();
        if parts.len() < 2 {
            anyhow::bail!("Expected grid and moves separated by blank line");
        }
        return parse_parts(&parts);
    }
    parse_parts(&parts)
}

fn parse_parts(parts: &[&str]) -> Result<(Grid<Tile>, Vec<Direction>, Coord)> {
    // Parse grid
    let lines: Vec<&str> = parts[0].lines().collect();

    let mut grid_data = Vec::new();
    let mut robot_pos = Coord::new(0, 0);

    for (row, line) in lines.iter().enumerate() {
        let mut row_data = Vec::new();
        for (col, ch) in line.chars().enumerate() {
            let tile = Tile::from(ch);
            row_data.push(tile);
            if tile == Tile::Robot {
                robot_pos = Coord::new(col, row); // x=col, y=row
            }
        }
        grid_data.push(row_data);
    }

    let grid = Grid::from_vec2d(grid_data);

    // Parse moves
    let moves: Vec<Direction> = parts[1].chars().filter_map(Direction::from_char).collect();

    Ok((grid, moves, robot_pos))
}

fn try_move(grid: &mut Grid<Tile>, pos: Coord, dir: Direction) -> Option<Coord> {
    let (dr, dc) = dir.delta();

    // Calculate new position (handle potential underflow)
    let new_y = pos.y as i32 + dr;
    let new_x = pos.x as i32 + dc;

    if new_x < 0 || new_y < 0 {
        return None;
    }

    let new_pos = Coord::new(new_x as usize, new_y as usize);

    // Check bounds
    if !grid.in_bounds(new_pos) {
        return None;
    }

    let target = *grid.get(new_pos)?;

    match target {
        Tile::Wall => None,           // Can't move into wall
        Tile::Empty => Some(new_pos), // Can move into empty space
        Tile::Box => {
            // Scan forward until we find empty space or hit a wall
            let mut check_pos = new_pos;

            loop {
                let next_y = check_pos.y as i32 + dr;
                let next_x = check_pos.x as i32 + dc;

                if next_x < 0 || next_y < 0 {
                    return None;
                }

                let next_pos = Coord::new(next_x as usize, next_y as usize);

                if !grid.in_bounds(next_pos) {
                    return None;
                }

                match grid.get(next_pos) {
                    Some(&Tile::Empty) => {
                        // Found empty space - place box at empty space
                        if let Some(cell) = grid.get_mut(next_pos) {
                            *cell = Tile::Box;
                        }

                        // Clear first box position (robot will move here)
                        if let Some(cell) = grid.get_mut(new_pos) {
                            *cell = Tile::Empty;
                        }

                        return Some(new_pos);
                    }
                    Some(&Tile::Box) => {
                        // Another box in chain, keep scanning
                        check_pos = next_pos;
                    }
                    Some(&Tile::Wall) | None => {
                        // Hit a wall or edge, can't push
                        return None;
                    }
                    _ => return None,
                }
            }
        }
        _ => None,
    }
}

fn simulate_robot(mut grid: Grid<Tile>, moves: Vec<Direction>, mut robot_pos: Coord) -> Grid<Tile> {
    for mov in moves {
        // Clear current robot position
        if let Some(cell) = grid.get_mut(robot_pos) {
            *cell = Tile::Empty;
        }

        // Try to move
        if let Some(new_pos) = try_move(&mut grid, robot_pos, mov) {
            robot_pos = new_pos;
        }

        // Place robot at final position
        if let Some(cell) = grid.get_mut(robot_pos) {
            *cell = Tile::Robot;
        }
    }
    grid
}

fn calculate_gps_sum(grid: &Grid<Tile>) -> usize {
    let mut sum = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let coord = Coord::new(x, y);
            if let Some(&Tile::Box | &Tile::BoxLeft) = grid.get(coord) {
                // GPS coordinate: 100 * row + col
                sum += 100 * y + x;
            }
        }
    }
    sum
}

pub fn solve_part1(input: &str) -> Result<String> {
    let (grid, moves, robot_pos) = parse_input(input)?;
    let final_grid = simulate_robot(grid, moves, robot_pos);
    let gps_sum = calculate_gps_sum(&final_grid);
    Ok(gps_sum.to_string())
}

fn widen_grid(grid: &Grid<Tile>) -> (Grid<Tile>, Coord) {
    let new_width = grid.width() * 2;
    let new_height = grid.height();
    let mut wide_grid = Grid::new(new_width, new_height, Tile::Empty);
    let mut robot_pos = Coord::new(0, 0);

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let coord = Coord::new(x, y);
            if let Some(&tile) = grid.get(coord) {
                let new_x = x * 2;
                let left_coord = Coord::new(new_x, y);
                let right_coord = Coord::new(new_x + 1, y);

                match tile {
                    Tile::Wall => {
                        if let Some(cell) = wide_grid.get_mut(left_coord) {
                            *cell = Tile::Wall;
                        }
                        if let Some(cell) = wide_grid.get_mut(right_coord) {
                            *cell = Tile::Wall;
                        }
                    }
                    Tile::Box => {
                        if let Some(cell) = wide_grid.get_mut(left_coord) {
                            *cell = Tile::BoxLeft;
                        }
                        if let Some(cell) = wide_grid.get_mut(right_coord) {
                            *cell = Tile::BoxRight;
                        }
                    }
                    Tile::Robot => {
                        robot_pos = left_coord;
                        if let Some(cell) = wide_grid.get_mut(left_coord) {
                            *cell = Tile::Robot;
                        }
                        if let Some(cell) = wide_grid.get_mut(right_coord) {
                            *cell = Tile::Empty;
                        }
                    }
                    Tile::Empty => {
                        if let Some(cell) = wide_grid.get_mut(left_coord) {
                            *cell = Tile::Empty;
                        }
                        if let Some(cell) = wide_grid.get_mut(right_coord) {
                            *cell = Tile::Empty;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    (wide_grid, robot_pos)
}

fn try_move_wide(grid: &mut Grid<Tile>, pos: Coord, dir: Direction) -> Option<Coord> {
    let (dr, dc) = dir.delta();
    let new_y = pos.y as i32 + dr;
    let new_x = pos.x as i32 + dc;

    if new_x < 0 || new_y < 0 {
        return None;
    }

    let new_pos = Coord::new(new_x as usize, new_y as usize);

    if !grid.in_bounds(new_pos) {
        return None;
    }

    let target = *grid.get(new_pos)?;

    match target {
        Tile::Wall => None,
        Tile::Empty => Some(new_pos),
        Tile::BoxLeft | Tile::BoxRight => {
            // Wide box pushing is more complex - need to check both halves
            match dir {
                Direction::Left | Direction::Right => {
                    // Horizontal: need to find the far edge of the box and push from there
                    // For pushing right: [...][...] -> if we hit [, we need to check past ]
                    // For pushing left: [...][...] -> if we hit ], we need to check past [

                    // First, find where this box ends in our direction
                    let box_far_edge = if dir.delta().1 > 0 {
                        // Pushing right - if we hit [, far edge is at x+1 (the ])
                        // if we hit ], far edge is here
                        if target == Tile::BoxLeft {
                            Coord::new(new_pos.x + 1, new_pos.y)
                        } else {
                            new_pos
                        }
                    } else {
                        // Pushing left - if we hit ], far edge is at x-1 (the [)
                        // if we hit [, far edge is here
                        if target == Tile::BoxRight {
                            Coord::new(new_pos.x - 1, new_pos.y)
                        } else {
                            new_pos
                        }
                    };

                    // Try to push from the far edge
                    if try_move_wide(grid, box_far_edge, dir).is_some() {
                        // Move the entire box: shift both cells in direction
                        let (box_left_x, box_right_x) = if target == Tile::BoxLeft {
                            (new_pos.x, new_pos.x + 1)
                        } else {
                            (new_pos.x - 1, new_pos.x)
                        };

                        let left_pos = Coord::new(box_left_x, new_pos.y);
                        let right_pos = Coord::new(box_right_x, new_pos.y);
                        let new_left = Coord::new((box_left_x as i32 + dc) as usize, new_pos.y);
                        let new_right = Coord::new((box_right_x as i32 + dc) as usize, new_pos.y);

                        // Clear old positions
                        if let Some(cell) = grid.get_mut(left_pos) {
                            *cell = Tile::Empty;
                        }
                        if let Some(cell) = grid.get_mut(right_pos) {
                            *cell = Tile::Empty;
                        }
                        // Set new positions
                        if let Some(cell) = grid.get_mut(new_left) {
                            *cell = Tile::BoxLeft;
                        }
                        if let Some(cell) = grid.get_mut(new_right) {
                            *cell = Tile::BoxRight;
                        }
                        Some(new_pos)
                    } else {
                        None
                    }
                }
                Direction::Up | Direction::Down => {
                    // Vertical: need to push both halves of box
                    // Collect all boxes that need to move (avoid double-moving)
                    let box_left_x = if target == Tile::BoxLeft {
                        new_pos.x
                    } else {
                        new_pos.x - 1
                    };

                    let left_pos = Coord::new(box_left_x, new_pos.y);

                    // Collect all box left-positions that need to move
                    let mut boxes_to_move: HashSet<Coord> = HashSet::new();
                    if collect_boxes_vertical(grid, left_pos, dir, &mut boxes_to_move) {
                        // Move all boxes - furthest first
                        let mut boxes_vec: Vec<Coord> = boxes_to_move.into_iter().collect();
                        // Sort by y - if going up, move highest (smallest y) first
                        // If going down, move lowest (largest y) first
                        if dr < 0 {
                            boxes_vec.sort_by_key(|c| c.y);
                        } else {
                            boxes_vec.sort_by_key(|c| std::cmp::Reverse(c.y));
                        }

                        for box_left in boxes_vec {
                            let box_right = Coord::new(box_left.x + 1, box_left.y);
                            let new_left =
                                Coord::new(box_left.x, (box_left.y as i32 + dr) as usize);
                            let new_right =
                                Coord::new(box_right.x, (box_right.y as i32 + dr) as usize);

                            // Clear old positions
                            if let Some(cell) = grid.get_mut(box_left) {
                                *cell = Tile::Empty;
                            }
                            if let Some(cell) = grid.get_mut(box_right) {
                                *cell = Tile::Empty;
                            }
                            // Set new positions
                            if let Some(cell) = grid.get_mut(new_left) {
                                *cell = Tile::BoxLeft;
                            }
                            if let Some(cell) = grid.get_mut(new_right) {
                                *cell = Tile::BoxRight;
                            }
                        }
                        Some(new_pos)
                    } else {
                        None
                    }
                }
            }
        }
        _ => None,
    }
}

/// Collect all boxes that need to move vertically starting from a box's left position
/// Returns true if the move is possible, false if blocked by wall
fn collect_boxes_vertical(
    grid: &Grid<Tile>,
    box_left: Coord,
    dir: Direction,
    boxes: &mut HashSet<Coord>,
) -> bool {
    // Already visited this box
    if boxes.contains(&box_left) {
        return true;
    }

    boxes.insert(box_left);

    let (dr, _) = dir.delta();
    let box_right = Coord::new(box_left.x + 1, box_left.y);

    // Check both positions above/below this box
    let next_left = Coord::new(box_left.x, (box_left.y as i32 + dr) as usize);
    let next_right = Coord::new(box_right.x, (box_right.y as i32 + dr) as usize);

    // Check left side
    if let Some(&tile) = grid.get(next_left) {
        match tile {
            Tile::Wall => return false,
            Tile::BoxLeft => {
                // Box directly above/below, aligned
                if !collect_boxes_vertical(grid, next_left, dir, boxes) {
                    return false;
                }
            }
            Tile::BoxRight => {
                // Box above/below, shifted left - its left is at next_left.x - 1
                let other_box_left = Coord::new(next_left.x - 1, next_left.y);
                if !collect_boxes_vertical(grid, other_box_left, dir, boxes) {
                    return false;
                }
            }
            Tile::Empty => {}
            _ => {}
        }
    }

    // Check right side
    if let Some(&tile) = grid.get(next_right) {
        match tile {
            Tile::Wall => return false,
            Tile::BoxLeft => {
                // Box above/below, shifted right
                if !collect_boxes_vertical(grid, next_right, dir, boxes) {
                    return false;
                }
            }
            Tile::BoxRight => {
                // This is the right side of the same box we might have checked on left
                // Its left is at next_right.x - 1
                let other_box_left = Coord::new(next_right.x - 1, next_right.y);
                if !collect_boxes_vertical(grid, other_box_left, dir, boxes) {
                    return false;
                }
            }
            Tile::Empty => {}
            _ => {}
        }
    }

    true
}

fn simulate_robot_wide(
    mut grid: Grid<Tile>,
    moves: Vec<Direction>,
    mut robot_pos: Coord,
) -> Grid<Tile> {
    for mov in moves {
        if let Some(cell) = grid.get_mut(robot_pos) {
            *cell = Tile::Empty;
        }

        if let Some(new_pos) = try_move_wide(&mut grid, robot_pos, mov) {
            robot_pos = new_pos;
        }

        if let Some(cell) = grid.get_mut(robot_pos) {
            *cell = Tile::Robot;
        }
    }
    grid
}

pub fn solve_part2(input: &str) -> Result<String> {
    let (grid, moves, _) = parse_input(input)?;
    let (wide_grid, robot_pos) = widen_grid(&grid);
    let final_grid = simulate_robot_wide(wide_grid, moves, robot_pos);
    let gps_sum = calculate_gps_sum(&final_grid);
    Ok(gps_sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let input = "########\n\
                     #..O.O.#\n\
                     ##@.O..#\n\
                     #...O..#\n\
                     #.#.O..#\n\
                     #...O..#\n\
                     #......#\n\
                     ########\n\
                     \n\
                     <^^>>>vv<v>>v<<";

        let result = solve_part1(input).unwrap();
        assert_eq!(result, "2028");
    }

    #[test]
    fn test_part1_example() {
        let input = include_str!("../../inputs/day15_large_example.txt");
        let result = solve_part1(input).unwrap();
        assert_eq!(result, "10092");
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../../inputs/day15_large_example.txt");
        let result = solve_part2(input).unwrap();
        assert_eq!(result, "9021");
    }
}
