//! # Day 17: Clumsy Crucible
//!
//! Pathfinding with movement constraints - crucible can't move more than 3 blocks
//! in same direction before must turn.
//!
//! ## Part 1
//! Find minimum heat loss from top-left to bottom-right.
//! Constraint: Max 3 consecutive blocks in same direction.
//!
//! ## Algorithm
//! - Dijkstra's algorithm with extended state space
//! - State: (position, direction, consecutive_steps)
//! - **Mission 6 Integration**:
//!   - `Grid<u8>` for heat loss map (~40 lines saved)
//!   - `Direction` enum with rotation methods (turn_left/turn_right)
//!   - `Coord` for position tracking
//!
//! ## Complexity
//! - Time: O((V * 4 * 3) * log(V)) where V = grid cells
//! - Space: O(V * 4 * 3) for state space

use anyhow::Result;
use mission6::{Coord, Direction, Grid};
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Coord,
    dir: Direction,
    consecutive: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    cost: usize,
    state: State,
}

// Ord is reversed (other.cmp(self) instead of self.cmp(other)) to create min-heap behavior.
// BinaryHeap is a max-heap by default, but Dijkstra needs to process lowest-cost nodes first.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_grid(input: &str) -> Grid<u8> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();

    let mut vec2d = vec![vec![0u8; width]; height];
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            vec2d[y][x] = ch.to_digit(10).unwrap() as u8;
        }
    }

    Grid::from_vec2d(vec2d)
}

/// Find minimum heat loss path using Dijkstra's algorithm with movement constraints.
///
/// # Why Dijkstra's Algorithm Works (The Fundamental Truth)
///
/// Dijkstra works because **min-heap ordering + greedy exploration creates a mathematical
/// guarantee**: the first time you reach the goal, no cheaper path can possibly exist,
/// because you've already explored (or will never need to explore) all alternatives.
///
/// ## The Chain of Reasoning:
///
/// 1. **Min-heap ensures cheapest-first exploration**: You always explore the lowest-cost
///    path from the frontier.
///
/// 2. **Greedy optimality**: Once you've explored all cheaper paths and none reached the
///    goal, the current path MUST be optimal.
///
/// 3. **Proof by exhaustion**: If a cheaper path to the goal existed, you would have already
///    explored it (since the heap pops lowest cost first).
///
/// 4. **Heap transitivity**: If path A costs less than path B, you pop A first. Any path
///    discovered from A will be compared against B before B gets popped.
///
/// 5. **The invariant**: When you pop a state from the heap, you've already explored all
///    cheaper ways to reach ANY state.
///
/// 6. **Heap property guarantee**: Parent ≤ children means you cannot pop a node with
///    cost X until all nodes with cost < X have been popped first.
///
/// **It's not about being "smart"—it's about mathematical impossibility of finding a cheaper
/// path once you've exhausted all lower-cost options.**
fn find_min_heat_loss(grid: &Grid<u8>, min_straight: u8, max_straight: u8) -> usize {
    let start = Coord::new(0, 0);
    let goal = Coord::new(grid.width() - 1, grid.height() - 1);

    let mut heap = BinaryHeap::new();
    let mut visited: HashMap<State, usize> = HashMap::new();

    // Start facing east or south (can't go backwards from start)
    for dir in [Direction::East, Direction::South] {
        let state = State {
            pos: start,
            dir,
            consecutive: 0,
        };
        heap.push(Node { cost: 0, state });
    }

    while let Some(Node { cost, state }) = heap.pop() {
        // Reached goal - but must have moved at least min_straight blocks
        if state.pos == goal && state.consecutive >= min_straight {
            return cost;
        }

        // Already visited with lower cost
        if let Some(&prev_cost) = visited.get(&state) {
            if cost >= prev_cost {
                continue;
            }
        }
        visited.insert(state, cost);

        // Generate next states
        let mut next_dirs = Vec::new();

        // Can continue straight if < max_straight consecutive
        if state.consecutive < max_straight {
            next_dirs.push((state.dir, state.consecutive + 1));
        }

        // Can turn left or right only if moved at least min_straight blocks
        if state.consecutive >= min_straight {
            next_dirs.push((state.dir.rotate_90_counterclockwise(), 1));
            next_dirs.push((state.dir.rotate_90_clockwise(), 1));
        }

        for (next_dir, next_consecutive) in next_dirs {
            let (dx, dy) = next_dir.delta();
            let nx = state.pos.x as isize + dx;
            let ny = state.pos.y as isize + dy;

            if nx >= 0 && ny >= 0 {
                let next_pos = Coord::new(nx as usize, ny as usize);

                if grid.in_bounds(next_pos) {
                    let next_state = State {
                        pos: next_pos,
                        dir: next_dir,
                        consecutive: next_consecutive,
                    };

                    let heat_loss = grid.get(next_pos).unwrap();
                    let next_cost = cost + *heat_loss as usize;

                    // Only add if not visited or found cheaper path
                    if !visited.contains_key(&next_state)
                        || visited.get(&next_state).unwrap() > &next_cost
                    {
                        heap.push(Node {
                            cost: next_cost,
                            state: next_state,
                        });
                    }
                }
            }
        }
    }

    unreachable!("No path found to goal")
}

pub fn solve_part1(input: &str) -> Result<String> {
    let grid = parse_grid(input);
    let min_heat = find_min_heat_loss(&grid, 0, 3);
    Ok(min_heat.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let grid = parse_grid(input);
    let min_heat = find_min_heat_loss(&grid, 4, 10);
    Ok(min_heat.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, "102");
    }

    #[test]
    fn test_parse_grid() {
        let grid = parse_grid(EXAMPLE);
        assert_eq!(grid.width(), 13);
        assert_eq!(grid.height(), 13);
        assert_eq!(*grid.get(Coord::new(0, 0)).unwrap(), 2);
        assert_eq!(*grid.get(Coord::new(12, 12)).unwrap(), 3);
    }
    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result, "94");
    }

    #[test]
    fn test_part2_simple() {
        const SIMPLE: &str = "\
111111111111
999999999991
999999999991
999999999991
999999999991";
        let result = solve_part2(SIMPLE).unwrap();
        assert_eq!(result, "71");
    }

    #[test]
    #[ignore]
    fn test_part1_real() {
        let input = std::fs::read_to_string("inputs/day17.txt").unwrap();
        let result = solve_part1(&input).unwrap();
        println!("Part 1: {}", result);
    }

    #[test]
    #[ignore]
    fn test_part2_real() {
        let input = std::fs::read_to_string("inputs/day17.txt").unwrap();
        let result = solve_part2(&input).unwrap();
        println!("Part 2: {}", result);
    }

    #[test]
    fn test_minimal_grid() {
        // 2x2 grid - smallest possible
        const MINIMAL: &str = "\
11
19";
        let result = solve_part1(MINIMAL).unwrap();
        // Path: (0,0) start free → right (1,0)=1 → down (1,1)=9 = total 10
        // Or: (0,0) start free → down (0,1)=1 → right (1,1)=9 = total 10
        assert_eq!(result, "10");
    }

    #[test]
    fn test_single_path() {
        // Only one valid path - straight line
        const LINE: &str = "\
1111
9999
9999
9999";
        let result = solve_part1(LINE).unwrap();
        // Start at (0,0)=1, must go right to (1,0)=1, (2,0)=1, (3,0)=1 then down
        // Path: right x3 = 3, down x3 = 9+9+9 = 30, total 33
        // Actually: start is free, so right(1,1,1) + down(9,9,9) = 30
        assert_eq!(result, "30");
    }

    #[test]
    fn test_all_zeros() {
        // Edge case: all zeros (except start which is also 0)
        const ZEROS: &str = "\
000
000
000";
        let result = solve_part1(ZEROS).unwrap();
        assert_eq!(result, "0");
    }
}
