/// AoC 2024 Day 16: Reindeer Maze
///
/// Grid-based pathfinding with rotation costs using Mission composition:
/// - Mission 6: Grid<T> for maze representation
/// - Mission 8: Dijkstra's algorithm for shortest path with (position, direction) state
///
/// Problem: Find minimum cost path where:
/// - Moving forward costs 1 point
/// - Rotating 90° costs 1000 points
use mission6::Grid;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

/// Cardinal directions for maze navigation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Get the delta (row, col) for moving in this direction
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }

    /// Rotate clockwise
    fn rotate_cw(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    /// Rotate counter-clockwise
    fn rotate_ccw(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

/// State in the pathfinding graph: (position, direction)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: (usize, usize),
    dir: Direction,
}

/// Priority queue node for Dijkstra's algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    cost: usize,
    state: State,
}

// Min-heap ordering for Dijkstra
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse for min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Parse the maze grid from input
fn parse_maze(input: &str) -> (Grid<char>, (usize, usize), (usize, usize)) {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let mut grid = Grid::new(rows, cols, '#');
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            grid[(r, c)] = ch;
            if ch == 'S' {
                start = (r, c);
            } else if ch == 'E' {
                end = (r, c);
            }
        }
    }

    (grid, start, end)
}

/// Get valid neighbors from a state (move forward or rotate)
fn get_neighbors(grid: &Grid<char>, state: State) -> Vec<(State, usize)> {
    let mut neighbors = Vec::new();

    // Option 1: Move forward in current direction (cost: 1)
    let (dr, dc) = state.dir.delta();
    let new_row = state.pos.0 as isize + dr;
    let new_col = state.pos.1 as isize + dc;

    if new_row >= 0 && new_col >= 0 {
        let new_pos = (new_row as usize, new_col as usize);
        if grid.in_bounds(new_pos.into()) && grid[new_pos] != '#' {
            neighbors.push((
                State {
                    pos: new_pos,
                    dir: state.dir,
                },
                1,
            ));
        }
    }

    // Option 2: Rotate clockwise (cost: 1000)
    neighbors.push((
        State {
            pos: state.pos,
            dir: state.dir.rotate_cw(),
        },
        1000,
    ));

    // Option 3: Rotate counter-clockwise (cost: 1000)
    neighbors.push((
        State {
            pos: state.pos,
            dir: state.dir.rotate_ccw(),
        },
        1000,
    ));

    neighbors
}

/// Dijkstra's shortest path from start (facing East) to end (any direction)
/// Returns (min_cost, distances_map)
fn dijkstra(
    grid: &Grid<char>,
    start: (usize, usize),
    end: (usize, usize),
) -> (usize, HashMap<State, usize>) {
    let mut heap = BinaryHeap::new();
    let mut distances = HashMap::new();

    // Start facing East with cost 0
    let start_state = State {
        pos: start,
        dir: Direction::East,
    };

    heap.push(Node {
        cost: 0,
        state: start_state,
    });
    distances.insert(start_state, 0);

    let mut min_end_cost = usize::MAX;

    while let Some(Node { cost, state }) = heap.pop() {
        // Skip if we've already found a better path to this state
        if let Some(&best) = distances.get(&state) {
            if cost > best {
                continue;
            }
        }

        // Check if we reached the end
        if state.pos == end {
            min_end_cost = min_end_cost.min(cost);
            continue; // Continue to explore all paths to end
        }

        // Explore neighbors
        for (next_state, edge_cost) in get_neighbors(grid, state) {
            let next_cost = cost + edge_cost;

            let is_better = distances
                .get(&next_state)
                .is_none_or(|&current| next_cost < current);

            if is_better {
                distances.insert(next_state, next_cost);
                heap.push(Node {
                    cost: next_cost,
                    state: next_state,
                });
            }
        }
    }

    (min_end_cost, distances)
}

/// Part 1: Find the minimum cost to reach the end
pub fn part1(input: &str) -> usize {
    let (grid, start, end) = parse_maze(input);
    let (min_cost, _) = dijkstra(&grid, start, end);
    min_cost
}

/// Part 2: Count all tiles that are part of any optimal path
///
/// Strategy: Backtrack from all end states with minimum cost
pub fn part2(input: &str) -> usize {
    let (grid, start, end) = parse_maze(input);
    let (min_cost, distances) = dijkstra(&grid, start, end);

    // Find all end states with minimum cost
    let end_states: Vec<State> = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .iter()
    .map(|&dir| State { pos: end, dir })
    .filter(|state| distances.get(state) == Some(&min_cost))
    .collect();

    // Backtrack from end states to find all nodes on optimal paths
    let mut on_best_path = HashSet::new();
    let mut queue: Vec<State> = end_states.clone();
    let mut visited = HashSet::new();

    while let Some(state) = queue.pop() {
        if !visited.insert(state) {
            continue;
        }

        on_best_path.insert(state.pos);

        let current_cost = distances.get(&state).copied().unwrap_or(usize::MAX);

        // Check all possible predecessors
        // 1. Moved forward from previous position
        let (dr, dc) = state.dir.delta();
        let prev_row = state.pos.0 as isize - dr;
        let prev_col = state.pos.1 as isize - dc;

        if prev_row >= 0 && prev_col >= 0 {
            let prev_pos = (prev_row as usize, prev_col as usize);
            if grid.in_bounds(prev_pos.into()) {
                let prev_state = State {
                    pos: prev_pos,
                    dir: state.dir,
                };
                if let Some(&prev_cost) = distances.get(&prev_state) {
                    if prev_cost + 1 == current_cost {
                        queue.push(prev_state);
                    }
                }
            }
        }

        // 2. Rotated clockwise from same position
        let prev_state = State {
            pos: state.pos,
            dir: state.dir.rotate_ccw(), // Reverse of CW
        };
        if let Some(&prev_cost) = distances.get(&prev_state) {
            if prev_cost + 1000 == current_cost {
                queue.push(prev_state);
            }
        }

        // 3. Rotated counter-clockwise from same position
        let prev_state = State {
            pos: state.pos,
            dir: state.dir.rotate_cw(), // Reverse of CCW
        };
        if let Some(&prev_cost) = distances.get(&prev_state) {
            if prev_cost + 1000 == current_cost {
                queue.push(prev_state);
            }
        }
    }

    on_best_path.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const EXAMPLE2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1(EXAMPLE1), 7036);
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(part1(EXAMPLE2), 11048);
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(part2(EXAMPLE1), 45);
    }

    #[test]
    fn test_part2_example2() {
        assert_eq!(part2(EXAMPLE2), 64);
    }
}
