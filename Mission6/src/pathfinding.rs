//! Pathfinding algorithms for 2D grids
//! 
//! Provides BFS and A* implementations optimized for grid navigation
//! with customizable cost functions and obstacle detection.

use crate::{Grid, Coord};
use std::collections::{VecDeque, BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

/// Result of a BFS pathfinding operation
#[derive(Debug, Clone)]
pub struct BfsResult {
    pub path: Vec<Coord>,
    pub distance: usize,
    pub visited_count: usize,
}

/// Result of an A* pathfinding operation
#[derive(Debug, Clone)]
pub struct AStarResult {
    pub path: Vec<Coord>,
    pub cost: usize,
    pub visited_count: usize,
}

/// Internal node for A* algorithm
#[derive(Debug, Clone, PartialEq, Eq)]
struct AStarNode {
    coord: Coord,
    g_cost: usize,      // Distance from start
    h_cost: usize,      // Heuristic distance to goal
    f_cost: usize,      // g_cost + h_cost
}

impl AStarNode {
    fn new(coord: Coord, g_cost: usize, h_cost: usize) -> Self {
        AStarNode {
            coord,
            g_cost,
            h_cost,
            f_cost: g_cost + h_cost,
        }
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order for min-heap (BinaryHeap is max-heap by default)
        other.f_cost.cmp(&self.f_cost)
            .then_with(|| other.h_cost.cmp(&self.h_cost))
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Pathfinding algorithms for 2D grids
/// 
/// # Requirements Satisfied: REQ-3
/// 
/// Provides both BFS (optimal for unweighted graphs) and A* (optimal for weighted graphs)
/// with support for custom traversability predicates and cost functions.
pub struct PathFinder;

impl PathFinder {
    /// Find shortest path using BFS (Breadth-First Search)
    /// 
    /// Returns the shortest path from start to goal, or None if no path exists.
    /// Uses 4-directional movement (cardinal directions only).
    /// 
    /// # Arguments
    /// 
    /// * `grid` - The grid to search in
    /// * `start` - Starting coordinate
    /// * `goal` - Goal coordinate  
    /// * `is_passable` - Function that returns true if a cell can be traversed
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission6::{Grid, Coord, PathFinder};
    /// 
    /// let mut grid = Grid::new(5, 5, '.');
    /// grid[(2, 2)] = '#'; // Obstacle
    /// 
    /// let start = Coord::new(0, 0);
    /// let goal = Coord::new(4, 4);
    /// 
    /// let path = PathFinder::bfs(&grid, start, goal, |&cell| cell != '#');
    /// assert!(path.is_some());
    /// ```
    pub fn bfs<T, F>(
        grid: &Grid<T>,
        start: Coord,
        goal: Coord,
        is_passable: F,
    ) -> Option<Vec<Coord>>
    where
        F: Fn(&T) -> bool,
    {
        if !grid.in_bounds(start) || !grid.in_bounds(goal) {
            return None;
        }

        if start == goal {
            return Some(vec![start]);
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent = HashMap::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            for neighbor in current.neighbors_4_bounded(grid.width(), grid.height()) {
                if visited.contains(&neighbor) {
                    continue;
                }

                if !is_passable(&grid[neighbor]) {
                    continue;
                }

                visited.insert(neighbor);
                parent.insert(neighbor, current);

                if neighbor == goal {
                    // Reconstruct path
                    let mut path = Vec::new();
                    let mut current = goal;
                    
                    while let Some(&prev) = parent.get(&current) {
                        path.push(current);
                        current = prev;
                    }
                    path.push(start);
                    path.reverse();
                    return Some(path);
                }

                queue.push_back(neighbor);
            }
        }

        None
    }

    /// Find shortest path using BFS with detailed results
    /// 
    /// Returns detailed information about the search including visit count.
    pub fn bfs_detailed<T, F>(
        grid: &Grid<T>,
        start: Coord,
        goal: Coord,
        is_passable: F,
    ) -> Option<BfsResult>
    where
        F: Fn(&T) -> bool,
    {
        if !grid.in_bounds(start) || !grid.in_bounds(goal) {
            return None;
        }

        if start == goal {
            return Some(BfsResult {
                path: vec![start],
                distance: 0,
                visited_count: 1,
            });
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent = HashMap::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            for neighbor in current.neighbors_4_bounded(grid.width(), grid.height()) {
                if visited.contains(&neighbor) {
                    continue;
                }

                if !is_passable(&grid[neighbor]) {
                    continue;
                }

                visited.insert(neighbor);
                parent.insert(neighbor, current);

                if neighbor == goal {
                    // Reconstruct path
                    let mut path = Vec::new();
                    let mut current = goal;
                    
                    while let Some(&prev) = parent.get(&current) {
                        path.push(current);
                        current = prev;
                    }
                    path.push(start);
                    path.reverse();
                    
                    return Some(BfsResult {
                        distance: path.len() - 1,
                        visited_count: visited.len(),
                        path,
                    });
                }

                queue.push_back(neighbor);
            }
        }

        None
    }

    /// Find shortest path using A* algorithm
    /// 
    /// Returns the optimal path from start to goal considering movement costs.
    /// Uses Manhattan distance as the heuristic by default.
    /// 
    /// # Arguments
    /// 
    /// * `grid` - The grid to search in
    /// * `start` - Starting coordinate
    /// * `goal` - Goal coordinate
    /// * `is_passable` - Function that returns true if a cell can be traversed
    /// * `cost_fn` - Function that returns the movement cost for entering a cell
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission6::{Grid, Coord, PathFinder};
    /// 
    /// let mut grid = Grid::new(5, 5, 1u32); // Movement cost of 1
    /// grid[(2, 2)] = 999; // High cost area
    /// 
    /// let start = Coord::new(0, 0);
    /// let goal = Coord::new(4, 4);
    /// 
    /// let result = PathFinder::astar(
    ///     &grid, 
    ///     start, 
    ///     goal,
    ///     |&cost| cost < 999,  // Avoid high cost areas
    ///     |&cost| cost as usize  // Movement cost
    /// );
    /// 
    /// if let Some(path) = result {
    ///     println!("Found path with {} steps", path.len() - 1);
    /// }
    /// ```
    pub fn astar<T, F, C>(
        grid: &Grid<T>,
        start: Coord,
        goal: Coord,
        is_passable: F,
        cost_fn: C,
    ) -> Option<Vec<Coord>>
    where
        F: Fn(&T) -> bool,
        C: Fn(&T) -> usize,
    {
        Self::astar_detailed(grid, start, goal, is_passable, cost_fn)
            .map(|result| result.path)
    }

    /// Find shortest path using A* with detailed results
    pub fn astar_detailed<T, F, C>(
        grid: &Grid<T>,
        start: Coord,
        goal: Coord,
        is_passable: F,
        cost_fn: C,
    ) -> Option<AStarResult>
    where
        F: Fn(&T) -> bool,
        C: Fn(&T) -> usize,
    {
        if !grid.in_bounds(start) || !grid.in_bounds(goal) {
            return None;
        }

        if start == goal {
            return Some(AStarResult {
                path: vec![start],
                cost: 0,
                visited_count: 1,
            });
        }

        let mut open_set = BinaryHeap::new();
        let mut closed_set = HashSet::new();
        let mut g_scores = HashMap::new();
        let mut parent = HashMap::new();

        let h_start = start.manhattan_distance(goal);
        open_set.push(AStarNode::new(start, 0, h_start));
        g_scores.insert(start, 0);

        while let Some(current) = open_set.pop() {
            if current.coord == goal {
                // Reconstruct path
                let mut path = Vec::new();
                let mut current_coord = goal;
                
                while let Some(&prev) = parent.get(&current_coord) {
                    path.push(current_coord);
                    current_coord = prev;
                }
                path.push(start);
                path.reverse();
                
                return Some(AStarResult {
                    path,
                    cost: current.g_cost,
                    visited_count: closed_set.len() + 1,
                });
            }

            closed_set.insert(current.coord);

            for neighbor in current.coord.neighbors_4_bounded(grid.width(), grid.height()) {
                if closed_set.contains(&neighbor) {
                    continue;
                }

                if !is_passable(&grid[neighbor]) {
                    continue;
                }

                let tentative_g = current.g_cost + cost_fn(&grid[neighbor]);
                let current_g = g_scores.get(&neighbor).cloned().unwrap_or(usize::MAX);

                if tentative_g < current_g {
                    parent.insert(neighbor, current.coord);
                    g_scores.insert(neighbor, tentative_g);
                    
                    let h_cost = neighbor.manhattan_distance(goal);
                    open_set.push(AStarNode::new(neighbor, tentative_g, h_cost));
                }
            }
        }

        None
    }

    /// Find all reachable coordinates from a starting point
    /// 
    /// Uses flood-fill algorithm to find all coordinates reachable from start
    /// within the given maximum distance.
    pub fn reachable<T, F>(
        grid: &Grid<T>,
        start: Coord,
        max_distance: Option<usize>,
        is_passable: F,
    ) -> HashSet<Coord>
    where
        F: Fn(&T) -> bool,
    {
        let mut reachable = HashSet::new();
        let mut queue = VecDeque::new();
        let mut distances = HashMap::new();

        if !grid.in_bounds(start) || !is_passable(&grid[start]) {
            return reachable;
        }

        queue.push_back(start);
        reachable.insert(start);
        distances.insert(start, 0);

        while let Some(current) = queue.pop_front() {
            let current_distance = distances[&current];
            
            if let Some(max_dist) = max_distance {
                if current_distance >= max_dist {
                    continue;
                }
            }

            for neighbor in current.neighbors_4_bounded(grid.width(), grid.height()) {
                if reachable.contains(&neighbor) {
                    continue;
                }

                if !is_passable(&grid[neighbor]) {
                    continue;
                }

                reachable.insert(neighbor);
                distances.insert(neighbor, current_distance + 1);
                queue.push_back(neighbor);
            }
        }

        reachable
    }

    /// Calculate distance map from a starting point
    /// 
    /// Returns a HashMap with the shortest distance to each reachable coordinate.
    pub fn distance_map<T, F>(
        grid: &Grid<T>,
        start: Coord,
        is_passable: F,
    ) -> HashMap<Coord, usize>
    where
        F: Fn(&T) -> bool,
    {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();

        if !grid.in_bounds(start) || !is_passable(&grid[start]) {
            return distances;
        }

        queue.push_back(start);
        distances.insert(start, 0);

        while let Some(current) = queue.pop_front() {
            let current_distance = distances[&current];

            for neighbor in current.neighbors_4_bounded(grid.width(), grid.height()) {
                if distances.contains_key(&neighbor) {
                    continue;
                }

                if !is_passable(&grid[neighbor]) {
                    continue;
                }

                distances.insert(neighbor, current_distance + 1);
                queue.push_back(neighbor);
            }
        }

        distances
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_grid() -> Grid<char> {
        // Create a 5x5 grid with some obstacles
        let mut grid = Grid::new(5, 5, '.');
        grid[(1, 1)] = '#';
        grid[(1, 2)] = '#';
        grid[(1, 3)] = '#';
        grid[(3, 1)] = '#';
        grid[(3, 2)] = '#';
        grid
    }

    #[test]
    fn test_bfs_simple_path() {
        let grid = Grid::new(3, 3, '.');
        let start = Coord::new(0, 0);
        let goal = Coord::new(2, 2);

        let path = PathFinder::bfs(&grid, start, goal, |&cell| cell == '.').unwrap();
        
        assert!(path.len() >= 5); // At least 5 steps for shortest path
        assert_eq!(path[0], start);
        assert_eq!(path[path.len() - 1], goal);
    }

    #[test]
    fn test_bfs_with_obstacles() {
        let grid = create_test_grid();
        let start = Coord::new(0, 0);
        let goal = Coord::new(4, 4);

        let path = PathFinder::bfs(&grid, start, goal, |&cell| cell != '#').unwrap();
        
        assert_eq!(path[0], start);
        assert_eq!(path[path.len() - 1], goal);
        
        // Verify path doesn't go through obstacles
        for &coord in &path {
            assert_ne!(grid[coord], '#');
        }
    }

    #[test]
    fn test_bfs_no_path() {
        let mut grid = Grid::new(3, 3, '.');
        // Create impassable barrier
        grid[(1, 0)] = '#';
        grid[(1, 1)] = '#';
        grid[(1, 2)] = '#';
        
        let start = Coord::new(0, 0);
        let goal = Coord::new(2, 0);

        let path = PathFinder::bfs(&grid, start, goal, |&cell| cell != '#');
        assert!(path.is_none());
    }

    #[test]
    fn test_bfs_same_start_goal() {
        let grid = Grid::new(3, 3, '.');
        let coord = Coord::new(1, 1);

        let path = PathFinder::bfs(&grid, coord, coord, |&cell| cell == '.').unwrap();
        assert_eq!(path, vec![coord]);
    }

    #[test]
    fn test_bfs_detailed() {
        let grid = Grid::new(3, 3, '.');
        let start = Coord::new(0, 0);
        let goal = Coord::new(2, 2);

        let result = PathFinder::bfs_detailed(&grid, start, goal, |&cell| cell == '.').unwrap();
        
        assert!(result.distance > 0);
        assert!(result.visited_count > 0);
        assert_eq!(result.path[0], start);
        assert_eq!(result.path[result.path.len() - 1], goal);
    }

    #[test]
    fn test_astar_simple() {
        let grid = Grid::new(3, 3, 1u32); // Uniform cost
        let start = Coord::new(0, 0);
        let goal = Coord::new(2, 2);

        let path = PathFinder::astar(
            &grid,
            start,
            goal,
            |&cost| cost == 1,
            |&cost| cost as usize,
        ).unwrap();

        assert_eq!(path[0], start);
        assert_eq!(path[path.len() - 1], goal);
    }

    #[test]
    fn test_astar_with_costs() {
        let mut grid = Grid::new(3, 3, 1u32);
        grid[(1, 1)] = 5; // Higher cost cell

        let start = Coord::new(0, 0);
        let goal = Coord::new(2, 2);

        let result = PathFinder::astar_detailed(
            &grid,
            start,
            goal,
            |&cost| cost < 10,
            |&cost| cost as usize,
        ).unwrap();

        assert!(result.cost > 0);
        assert_eq!(result.path[0], start);
        assert_eq!(result.path[result.path.len() - 1], goal);
    }

    #[test]
    fn test_reachable() {
        let grid = create_test_grid();
        let start = Coord::new(0, 0);

        let reachable = PathFinder::reachable(&grid, start, Some(3), |&cell| cell != '#');
        
        assert!(reachable.contains(&start));
        assert!(reachable.len() > 1);
        
        // Should not contain obstacles
        for coord in &reachable {
            assert_ne!(grid[*coord], '#');
        }
    }

    #[test]
    fn test_distance_map() {
        let grid = Grid::new(3, 3, '.');
        let start = Coord::new(1, 1); // Center

        let distances = PathFinder::distance_map(&grid, start, |&cell| cell == '.');
        
        assert_eq!(distances[&start], 0);
        assert_eq!(distances[&Coord::new(0, 1)], 1); // Adjacent
        assert_eq!(distances[&Coord::new(0, 0)], 2); // Diagonal via manhattan
    }
}