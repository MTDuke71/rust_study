//! Day 22: Grid Computing
//!
//! Storage cluster as a grid of nodes. Part 1: count viable data-move pairs.
//! Part 2: minimum moves to bring goal data from top-right to access point (0,0).

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct Node {
    x: usize,
    y: usize,
    used: u16,
    avail: u16,
}

fn parse_input(input: &str) -> Vec<Node> {
    input
        .trim()
        .lines()
        .skip(2) // skip header lines
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // /dev/grid/node-x0-y0
            let name = parts[0];
            let coords: Vec<usize> = name
                .split('-')
                .skip(1) // skip "/dev/grid/node"
                .map(|s| s[1..].parse().unwrap()) // skip 'x' or 'y'
                .collect();
            let parse_t = |s: &str| -> u16 { s.trim_end_matches('T').parse().unwrap() };
            Node {
                x: coords[0],
                y: coords[1],
                used: parse_t(parts[2]),
                avail: parse_t(parts[3]),
            }
        })
        .collect()
}

fn count_viable_pairs(nodes: &[Node]) -> usize {
    let mut count = 0;
    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if i != j && nodes[i].used > 0 && nodes[i].used <= nodes[j].avail {
                count += 1;
            }
        }
    }
    count
}

fn solve_part2_geometry(nodes: &[Node]) -> usize {
    // Find grid dimensions
    let max_x = nodes.iter().map(|n| n.x).max().unwrap();
    let max_y = nodes.iter().map(|n| n.y).max().unwrap();
    let width = max_x + 1;
    let height = max_y + 1;

    // Build grid: true = passable (normal node), false = wall (huge node)
    // Also find the empty node
    let mut wall = vec![vec![false; width]; height];
    let mut empty = (0, 0);

    for n in nodes {
        if n.used == 0 {
            empty = (n.x, n.y);
        }
        // Wall nodes have ~500T used, normal nodes have ~65-73T used
        // A node is a wall if its used data won't fit in any normal node's capacity
        if n.used > 100 {
            wall[n.y][n.x] = true;
        }
    }

    let goal = (max_x, 0usize);

    // Step 1: BFS the empty node to the position left of the goal
    // (we need empty adjacent to goal to start sliding it)
    let target_for_empty = (goal.0 - 1, goal.1);
    let steps_to_goal = bfs_empty(empty, target_for_empty, goal, &wall, width, height);

    // Step 2: Each "slide goal left by 1" costs 5 moves:
    //   - Swap goal into empty (1 move) — goal moves left, empty is now right of goal
    //   - Move empty around the goal to its left side (4 moves: down, left, left, up)
    // The last swap doesn't need the repositioning
    let slides = goal.0; // need to move goal from x=max_x to x=0
    let steps_to_slide = 1 + (slides - 1) * 5;

    steps_to_goal + steps_to_slide
}

fn bfs_empty(
    start: (usize, usize),
    target: (usize, usize),
    goal_pos: (usize, usize),
    wall: &[Vec<bool>],
    width: usize,
    height: usize,
) -> usize {
    let mut visited = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();
    visited[start.1][start.0] = true;
    queue.push_back((start, 0usize));

    while let Some(((x, y), dist)) = queue.pop_front() {
        if (x, y) == target {
            return dist;
        }
        for (dx, dy) in [(0isize, -1isize), (0, 1), (-1, 0), (1, 0)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || ny < 0 || nx >= width as isize || ny >= height as isize {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            // Can't move through walls or through the goal data position
            if visited[ny][nx] || wall[ny][nx] || (nx, ny) == goal_pos {
                continue;
            }
            visited[ny][nx] = true;
            queue.push_back(((nx, ny), dist + 1));
        }
    }
    panic!("no path found for empty node");
}

/// BFS over full (empty_pos, goal_pos) state space.
/// Slower but guaranteed optimal — used to verify the geometric shortcut.
#[cfg(test)]
fn solve_part2_bfs(nodes: &[Node]) -> usize {
    use std::collections::HashSet;
    let max_x = nodes.iter().map(|n| n.x).max().unwrap();
    let max_y = nodes.iter().map(|n| n.y).max().unwrap();
    let width = max_x + 1;
    let height = max_y + 1;

    let mut wall = vec![vec![false; width]; height];
    let mut empty = (0usize, 0usize);

    for n in nodes {
        if n.used == 0 {
            empty = (n.x, n.y);
        }
        if n.used > 100 {
            wall[n.y][n.x] = true;
        }
    }

    let goal = (max_x, 0usize);
    // State: (empty_x, empty_y, goal_x, goal_y)
    type State = (usize, usize, usize, usize);
    let start: State = (empty.0, empty.1, goal.0, goal.1);

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    visited.insert(start);
    queue.push_back((start, 0usize));

    while let Some(((ex, ey, gx, gy), dist)) = queue.pop_front() {
        if (gx, gy) == (0, 0) {
            return dist;
        }
        // Try moving empty in each direction (equivalent to moving data from neighbor into empty)
        for (dx, dy) in [(0isize, -1isize), (0, 1), (-1, 0), (1, 0)] {
            let nx = ex as isize + dx;
            let ny = ey as isize + dy;
            if nx < 0 || ny < 0 || nx >= width as isize || ny >= height as isize {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if wall[ny][nx] {
                continue;
            }
            // Moving data from (nx, ny) into empty at (ex, ey)
            // If (nx, ny) is the goal, the goal data moves to (ex, ey)
            let new_gx = if (nx, ny) == (gx, gy) { ex } else { gx };
            let new_gy = if (nx, ny) == (gx, gy) { ey } else { gy };
            let state: State = (nx, ny, new_gx, new_gy);
            if visited.insert(state) {
                queue.push_back((state, dist + 1));
            }
        }
    }
    panic!("no path found");
}

pub fn solve(input: &str) -> (String, String) {
    let nodes = parse_input(input);
    let p1 = count_viable_pairs(&nodes);
    let p2 = solve_part2_geometry(&nodes);
    (p1.to_string(), p2.to_string())
}

pub fn solve_part1(input: &str) -> String {
    let nodes = parse_input(input);
    count_viable_pairs(&nodes).to_string()
}

pub fn solve_part2(input: &str) -> String {
    let nodes = parse_input(input);
    solve_part2_geometry(&nodes).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("../../inputs/day22.txt");
        let nodes = parse_input(input);
        assert_eq!(nodes.len(), 960); // 30 x 32
    }

    #[test]
    fn test_grid_structure() {
        let input = include_str!("../../inputs/day22.txt");
        let nodes = parse_input(input);
        let max_x = nodes.iter().map(|n| n.x).max().unwrap();
        let max_y = nodes.iter().map(|n| n.y).max().unwrap();
        assert_eq!(max_x, 29);
        assert_eq!(max_y, 31);

        let empty: Vec<_> = nodes.iter().filter(|n| n.used == 0).collect();
        assert_eq!(empty.len(), 1);
        assert_eq!((empty[0].x, empty[0].y), (11, 22));
    }

    #[test]
    fn test_part2_bfs_matches_geometry() {
        let input = include_str!("../../inputs/day22.txt");
        let nodes = parse_input(input);
        let geometric = solve_part2_geometry(&nodes);
        let bfs = solve_part2_bfs(&nodes);
        assert_eq!(geometric, bfs, "geometric shortcut must match full BFS");
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day22.txt");
        assert_eq!(
            solve(input),
            ("937".to_string(), "188".to_string())
        );
    }
}
