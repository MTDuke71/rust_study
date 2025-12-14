/// Day 15: Warehouse Woes - Robot box-pushing simulation
/// 
/// Uses Mission 6's Grid<T> for warehouse layout
/// Part 1: Single-width boxes
/// Part 2: Double-width boxes (everything except robot is 2x wide)

use anyhow::Result;
use mission6::{Grid, Coord};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Box,
    Robot,
    Empty,
    BoxLeft,   // For Part 2: [ 
    BoxRight,  // For Part 2: ]
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
    let moves: Vec<Direction> = parts[1]
        .chars()
        .filter_map(Direction::from_char)
        .collect();

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
        Tile::Wall => None, // Can't move into wall
        Tile::Empty => Some(new_pos), // Can move into empty space
        Tile::Box => {
            // For a box, check if we can push it by recursively checking its movement
            // We need to find the chain of boxes and see if there's empty space at the end
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
                        // Found empty space - we can push all boxes
                        // Move the box at the end of the chain to the empty space
                        if let Some(cell) = grid.get_mut(next_pos) {
                            *cell = Tile::Box;
                        }
                        // The original target position becomes empty (will be filled by robot or next box)
                        if let Some(cell) = grid.get_mut(new_pos) {
                            *cell = Tile::Empty;
                        }
                        return Some(new_pos);
                    }
                    Some(&Tile::Box) => {
                        // Another box in the chain, keep checking
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
            if let Some(&tile) = grid.get(coord) {
                match tile {
                    Tile::Box | Tile::BoxLeft => {
                        // GPS coordinate: 100 * row + col
                        sum += 100 * y + x;
                    }
                    _ => {}
                }
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
                    // Horizontal: simpler, just push in direction
                    if let Some(box_new_pos) = try_move_wide(grid, new_pos, dir) {
                        if let Some(cell) = grid.get_mut(box_new_pos) {
                            *cell = target;
                        }
                        if let Some(cell) = grid.get_mut(new_pos) {
                            *cell = Tile::Empty;
                        }
                        Some(new_pos)
                    } else {
                        None
                    }
                }
                Direction::Up | Direction::Down => {
                    // Vertical: need to push both halves of box
                    let (box_left_x, box_right_x) = if target == Tile::BoxLeft {
                        (new_pos.x, new_pos.x + 1)
                    } else {
                        (new_pos.x - 1, new_pos.x)
                    };

                    let left_pos = Coord::new(box_left_x, new_pos.y);
                    let right_pos = Coord::new(box_right_x, new_pos.y);

                    // Check if both halves can move
                    let can_move_left = can_push_wide(grid, left_pos, dir);
                    let can_move_right = can_push_wide(grid, right_pos, dir);

                    if can_move_left && can_move_right {
                        // Actually push both halves
                        try_move_wide(grid, left_pos, dir);
                        try_move_wide(grid, right_pos, dir);
                        
                        if let Some(cell) = grid.get_mut(left_pos) {
                            *cell = Tile::BoxLeft;
                        }
                        if let Some(cell) = grid.get_mut(right_pos) {
                            *cell = Tile::BoxRight;
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

fn can_push_wide(grid: &Grid<Tile>, pos: Coord, dir: Direction) -> bool {
    let (dr, dc) = dir.delta();
    let new_y = pos.y as i32 + dr;
    let new_x = pos.x as i32 + dc;
    
    if new_x < 0 || new_y < 0 {
        return false;
    }
    
    let new_pos = Coord::new(new_x as usize, new_y as usize);
    
    if !grid.in_bounds(new_pos) {
        return false;
    }

    if let Some(&target) = grid.get(new_pos) {
        match target {
            Tile::Empty => true,
            Tile::Wall => false,
            Tile::BoxLeft | Tile::BoxRight => {
                match dir {
                    Direction::Up | Direction::Down => {
                        let (left_x, right_x) = if target == Tile::BoxLeft {
                            (new_pos.x, new_pos.x + 1)
                        } else {
                            (new_pos.x - 1, new_pos.x)
                        };
                        let left_pos = Coord::new(left_x, new_pos.y);
                        let right_pos = Coord::new(right_x, new_pos.y);
                        can_push_wide(grid, left_pos, dir) 
                            && can_push_wide(grid, right_pos, dir)
                    }
                    _ => can_push_wide(grid, new_pos, dir)
                }
            }
            _ => false,
        }
    } else {
        false
    }
}

fn simulate_robot_wide(mut grid: Grid<Tile>, moves: Vec<Direction>, mut robot_pos: Coord) -> Grid<Tile> {
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
