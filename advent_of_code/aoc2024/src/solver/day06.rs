use anyhow::{Context, Result};
use mission6::{Coord, Direction, Grid}; // From Mission 6
use std::collections::HashSet;

/// Guard state for patrol simulation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GuardState {
    position: Coord,
    direction: Direction,
}

impl GuardState {
    fn new(position: Coord, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    /// Get the next position the guard would move to
    fn next_position(&self) -> Option<Coord> {
        self.position.step(self.direction)
    }

    /// Turn right 90 degrees (AoC guard protocol)
    fn turn_right(&mut self) {
        self.direction = self.direction.rotate_90_clockwise();
    }


}

/// Parse input into grid and find guard starting position
fn parse_input(input: &str) -> Result<(Grid<char>, GuardState)> {
    let lines: Vec<&str> = input.lines().filter(|line| !line.trim().is_empty()).collect();
    
    if lines.is_empty() {
        anyhow::bail!("Input is empty");
    }

    let height = lines.len();
    let width = lines[0].len();

    // Verify rectangular grid
    for (i, line) in lines.iter().enumerate() {
        if line.len() != width {
            anyhow::bail!("Line {} has length {}, expected {}", i, line.len(), width);
        }
    }

    // Create grid and find guard
    let mut grid = Grid::new(width, height, '.');
    let mut guard_state = None;

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let coord = Coord::new(x, y);
            
            match ch {
                '^' => {
                    guard_state = Some(GuardState::new(coord, Direction::North));
                    grid[coord] = '.'; // Replace guard with empty space
                }
                '>' => {
                    guard_state = Some(GuardState::new(coord, Direction::East));
                    grid[coord] = '.';
                }
                'v' => {
                    guard_state = Some(GuardState::new(coord, Direction::South));
                    grid[coord] = '.';
                }
                '<' => {
                    guard_state = Some(GuardState::new(coord, Direction::West));
                    grid[coord] = '.';
                }
                '#' | '.' => {
                    grid[coord] = ch;
                }
                _ => anyhow::bail!("Invalid character '{}' at position ({}, {})", ch, x, y),
            }
        }
    }

    let guard_state = guard_state.context("No guard found in input")?;
    Ok((grid, guard_state))
}

/// Simulate guard patrol until exit or loop detection
/// Returns (visited_positions, exited_grid)
fn simulate_patrol(grid: &Grid<char>, start_state: GuardState) -> (HashSet<Coord>, bool) {
    let mut guard = start_state;
    let mut visited_positions = HashSet::new();
    let mut state_history = HashSet::new();

    // Track starting position
    visited_positions.insert(guard.position);

    loop {
        // Check for loop (same position and direction)
        if state_history.contains(&guard) {
            return (visited_positions, false); // Loop detected
        }
        state_history.insert(guard);

        // Calculate next position
        if let Some(next_pos) = guard.next_position() {
            // Check if next position is in bounds
            if let Some(&cell) = grid.get(next_pos) {
                if cell == '#' {
                    // Obstacle: turn right
                    guard.turn_right();
                } else {
                    // Empty space: move forward
                    guard.position = next_pos;
                    visited_positions.insert(guard.position);
                }
            } else {
                // Next position is out of bounds: guard exits
                return (visited_positions, true);
            }
        } else {
            // Can't move (coordinate underflow): guard exits
            return (visited_positions, true);
        }
    }
}

/// Part 1: Count distinct positions visited by guard before exiting
pub fn solve_part1(input: &str) -> Result<String> {
    let (grid, guard_state) = parse_input(input).context("Failed to parse input")?;
    
    let (visited_positions, exited) = simulate_patrol(&grid, guard_state);
    
    if !exited {
        anyhow::bail!("Guard got stuck in a loop without adding obstacles");
    }

    Ok(visited_positions.len().to_string())
}

/// Part 2: Count positions where adding a single obstacle creates a loop
pub fn solve_part2(input: &str) -> Result<String> {
    let (original_grid, guard_state) = parse_input(input).context("Failed to parse input")?;
    
    // First, get the original patrol path
    let (original_path, exited) = simulate_patrol(&original_grid, guard_state);
    
    if !exited {
        anyhow::bail!("Guard got stuck in original patrol without obstacles");
    }

    let mut loop_positions = 0;
    
    // Try adding obstacle at each position on the original path (except start)
    for &position in &original_path {
        // Skip guard's starting position
        if position == guard_state.position {
            continue;
        }
        
        // Skip positions that already have obstacles
        if original_grid[position] == '#' {
            continue;
        }
        
        // Create modified grid with new obstacle
        let mut modified_grid = original_grid.clone();
        modified_grid[position] = '#';
        
        // Simulate patrol with the new obstacle
        let (_, exited) = simulate_patrol(&modified_grid, guard_state);
        
        if !exited {
            // Guard got stuck in a loop
            loop_positions += 1;
        }
    }

    Ok(loop_positions.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_parse_input() {
        let (grid, guard_state) = parse_input(EXAMPLE_INPUT).unwrap();
        
        assert_eq!(grid.width(), 10);
        assert_eq!(grid.height(), 10);
        assert_eq!(guard_state.position, Coord::new(4, 6));
        assert_eq!(guard_state.direction, Direction::North);
        
        // Check some obstacles
        assert_eq!(grid[Coord::new(4, 0)], '#');
        assert_eq!(grid[Coord::new(9, 1)], '#');
        
        // Check guard position is now empty
        assert_eq!(grid[Coord::new(4, 6)], '.');
    }

    #[test]
    fn test_guard_state_operations() {
        let mut guard = GuardState::new(Coord::new(4, 6), Direction::North);
        
        // Test next position
        assert_eq!(guard.next_position(), Some(Coord::new(4, 5)));
        
        // Test turning right
        guard.turn_right();
        assert_eq!(guard.direction, Direction::East);
        assert_eq!(guard.next_position(), Some(Coord::new(5, 6)));
        
        // Test movement
        if let Some(next_pos) = guard.next_position() {
            guard.position = next_pos;
        }
        assert_eq!(guard.position, Coord::new(5, 6));
    }

    #[test]
    fn test_simulate_patrol_exit() {
        let (grid, guard_state) = parse_input(EXAMPLE_INPUT).unwrap();
        let (visited, exited) = simulate_patrol(&grid, guard_state);
        
        assert!(exited, "Guard should exit the grid");
        assert!(visited.len() > 0, "Should visit at least the starting position");
    }

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "41");
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "6");
    }

    #[test]
    fn test_direction_parsing() {
        let test_cases = vec![
            ("^", Direction::North),
            (">", Direction::East), 
            ("v", Direction::South),
            ("<", Direction::West),
        ];
        
        for (input_char, expected_dir) in test_cases {
            let input = format!("...\n.{}.\n...", input_char);
            let (_, guard_state) = parse_input(&input).unwrap();
            assert_eq!(guard_state.direction, expected_dir);
        }
    }

    #[test]
    fn test_obstacle_collision() {
        // Simple case: guard hits obstacle and turns
        let input = "...\n.^#\n...";
        let (grid, mut guard_state) = parse_input(input).unwrap();
        
        // Guard should be facing north at (1, 1)
        assert_eq!(guard_state.position, Coord::new(1, 1));
        assert_eq!(guard_state.direction, Direction::North);
        
        // Next position should be blocked
        let next_pos = guard_state.next_position().unwrap();
        assert_eq!(grid[next_pos], '.');  // This should be the position above
        
        // Move north to (1, 0), then try to move north again (would hit boundary)
        guard_state.position = Coord::new(1, 0);
        assert!(guard_state.next_position().is_none()); // Would underflow
    }

    #[test]
    fn test_bounds_checking() {
        let input = "^\n.";
        let (grid, guard_state) = parse_input(input).unwrap();
        
        // Guard at top-left, facing north - should exit immediately
        let (visited, exited) = simulate_patrol(&grid, guard_state);
        assert!(exited);
        assert_eq!(visited.len(), 1); // Only starting position
    }
}