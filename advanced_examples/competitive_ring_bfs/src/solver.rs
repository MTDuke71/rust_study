use crate::maze::{Grid, Position};
use mission2::RingBufferQueue;
use std::collections::HashMap;

/// BFS-based maze solver using RingBufferQueue for optimal performance
/// 
/// # Requirements Satisfied: REQ-B1, REQ-B3, REQ-B5
/// 
/// Uses RingBufferQueue with fixed capacity = grid_size for predictable
/// memory usage and cache-friendly BFS traversal.
pub struct MazeSolver {
    grid: Grid,
}

/// BFS state for tracking distance and parent during traversal
/// 
/// # Requirements Satisfied: REQ-P1
#[derive(Debug, Clone)]
struct BfsState {
    position: Position,
    distance: usize,
    _parent: Option<Position>, // For future path reconstruction
}

impl MazeSolver {
    /// Create a new maze solver from string representation
    /// 
    /// # Requirements Satisfied: REQ-G1, REQ-G2, REQ-G4
    /// 
    /// # Returns
    /// None if the maze format is invalid
    pub fn new(maze_lines: Vec<String>) -> Option<Self> {
        Grid::from_strings(maze_lines).map(|grid| Self { grid })
    }
    
    /// Find shortest path from start to goal using BFS
    /// 
    /// # Requirements Satisfied: REQ-B1, REQ-B5, REQ-P2, REQ-P3, REQ-P4
    /// 
    /// # Returns
    /// - Some(path) if path exists (includes start and goal)
    /// - None if no path exists
    /// 
    /// # Algorithm
    /// 1. Initialize RingBufferQueue with capacity = grid size
    /// 2. Start BFS from start position
    /// 3. Explore 4-directional neighbors  
    /// 4. Track parent pointers for path reconstruction
    /// 5. Return shortest path when goal is reached
    pub fn find_shortest_path(&self) -> Option<Vec<Position>> {
        let (rows, cols) = self.grid.dimensions();
        let capacity = self.grid.max_queue_size(); // REQ-B3
        
        // Initialize RingBufferQueue with fixed capacity
        let mut queue = RingBufferQueue::with_capacity(capacity);
        
        // Track visited positions to prevent cycles (REQ-B4)
        let mut visited = vec![vec![false; cols]; rows];
        
        // Track parent pointers for path reconstruction (REQ-P1)
        let mut parents: HashMap<Position, Option<Position>> = HashMap::new();
        
        // Start BFS from start position (REQ-B1)
        let start = self.grid.start();
        let goal = self.grid.goal();
        
        let initial_state = BfsState {
            position: start,
            distance: 0,
            _parent: None,
        };
        
        queue.enqueue(initial_state).expect("Queue should have capacity for start");
        visited[start.row][start.col] = true;
        parents.insert(start, None);
        
        // BFS main loop (REQ-B1, REQ-B5)
        while let Some(current_state) = queue.dequeue() {
            let current_pos = current_state.position;
            
            // Check if we reached the goal (REQ-B5)
            if current_pos == goal {
                return Some(self.reconstruct_path(&parents, start, goal)); // REQ-P2, REQ-P3
            }
            
            // Explore neighbors (REQ-B2)
            for neighbor_pos in current_pos.neighbors(rows, cols) {
                // Skip if already visited (REQ-B4)
                if visited[neighbor_pos.row][neighbor_pos.col] {
                    continue;
                }
                
                // Skip if not traversable (REQ-G3)
                if let Some(cell) = self.grid.get(neighbor_pos) {
                    if !cell.is_traversable() {
                        continue;
                    }
                } else {
                    continue; // Out of bounds
                }
                
                // Mark as visited and add to queue
                visited[neighbor_pos.row][neighbor_pos.col] = true;
                parents.insert(neighbor_pos, Some(current_pos));
                
                let neighbor_state = BfsState {
                    position: neighbor_pos,
                    distance: current_state.distance + 1,
                    _parent: Some(current_pos),
                };
                
                // Queue should never be full due to capacity = grid_size
                // Each cell is visited at most once
                queue.enqueue(neighbor_state).expect("Queue capacity should be sufficient");
            }
        }
        
        // No path exists (REQ-P4)
        None
    }
    
    /// Reconstruct path from goal back to start using parent pointers
    /// 
    /// # Requirements Satisfied: REQ-P1, REQ-P2, REQ-P3
    fn reconstruct_path(
        &self,
        parents: &HashMap<Position, Option<Position>>,
        start: Position,
        goal: Position,
    ) -> Vec<Position> {
        let mut path = Vec::new();
        let mut current = goal;
        
        // Trace back from goal to start (REQ-P2)
        loop {
            path.push(current);
            
            if current == start {
                break;
            }
            
            if let Some(parent_opt) = parents.get(&current) {
                if let Some(parent) = parent_opt {
                    current = *parent;
                } else {
                    // Reached start (parent is None)
                    break;
                }
            } else {
                // Should not happen in correct implementation
                break;
            }
        }
        
        // Reverse to get path from start to goal (REQ-P3)
        path.reverse();
        path
    }
    
    /// Get maze dimensions for analysis
    pub fn dimensions(&self) -> (usize, usize) {
        self.grid.dimensions()
    }
    
    /// Get start position
    pub fn start(&self) -> Position {
        self.grid.start()
    }
    
    /// Get goal position
    pub fn goal(&self) -> Position {
        self.grid.goal()
    }
    
    /// Calculate queue capacity used for this grid
    /// 
    /// # Requirements Satisfied: REQ-B3
    /// Demonstrates why RingBufferQueue with fixed capacity works perfectly:
    /// capacity = rows * cols guarantees no queue overflow.
    pub fn required_queue_capacity(&self) -> usize {
        self.grid.max_queue_size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // REQ-B1, REQ-B5
    fn req_b1_b5_simple_path_finding() {
        let maze = vec![
            "S..".to_string(),
            "...".to_string(),
            "..G".to_string(),
        ];
        
        let solver = MazeSolver::new(maze).expect("Valid maze");
        let path = solver.find_shortest_path().expect("Path should exist");
        
        // Should find optimal path
        assert_eq!(path.len(), 5); // S -> . -> . -> . -> G
        assert_eq!(path[0], solver.start());
        assert_eq!(path[path.len() - 1], solver.goal());
    }

    #[test] // REQ-B1, REQ-P4
    fn req_b1_p4_no_path_case() {
        let maze = vec![
            "S.#".to_string(),
            "###".to_string(),
            "#.G".to_string(),
        ];
        
        let solver = MazeSolver::new(maze).expect("Valid maze");
        let path = solver.find_shortest_path();
        
        // No path should exist due to walls
        assert!(path.is_none());
    }

    #[test] // REQ-B2
    fn req_b2_path_around_obstacles() {
        let maze = vec![
            "S##".to_string(),
            ".##".to_string(),
            "..G".to_string(),
        ];
        
        let solver = MazeSolver::new(maze).expect("Valid maze");
        let path = solver.find_shortest_path().expect("Path should exist");
        
        // Must go around the wall
        assert!(path.len() > 3); // Longer than direct path
        assert_eq!(path[0], solver.start());
        assert_eq!(path[path.len() - 1], solver.goal());
    }

    #[test] // REQ-B3
    fn req_b3_queue_capacity_calculation() {
        let maze = vec![
            "S...".to_string(),
            "....".to_string(),
            "....".to_string(),
            "...G".to_string(),
        ];
        
        let solver = MazeSolver::new(maze).expect("Valid maze");
        
        // 4x4 grid should require capacity of 16
        assert_eq!(solver.required_queue_capacity(), 16);
    }

    #[test] // REQ-P2, REQ-P3
    fn req_p2_p3_path_reconstruction() {
        let maze = vec![
            "S.".to_string(),
            ".G".to_string(),
        ];
        
        let solver = MazeSolver::new(maze).expect("Valid maze");
        let path = solver.find_shortest_path().expect("Path should exist");
        
        // Verify path is correctly ordered from start to goal
        assert_eq!(path[0], Position::new(0, 0)); // Start
        assert_eq!(path[path.len() - 1], Position::new(1, 1)); // Goal
        
        // Verify path is contiguous (each step is adjacent)
        for window in path.windows(2) {
            let curr = window[0];
            let next = window[1];
            let neighbors = curr.neighbors(2, 2);
            assert!(neighbors.contains(&next), "Path should be contiguous");
        }
    }

    #[test] // REQ-B5
    fn req_b5_optimal_path_guarantee() {
        // Multiple possible paths, BFS should find shortest
        let maze = vec![
            "S....".to_string(),
            ".###.".to_string(),
            ".....".to_string(),
            ".###.".to_string(),
            "....G".to_string(),
        ];
        
        let solver = MazeSolver::new(maze).expect("Valid maze");
        let path = solver.find_shortest_path().expect("Path should exist");
        
        // BFS guarantees shortest path
        // Optimal path: S->right->right->right->right->down->down->down->down->G = 9 steps
        assert_eq!(path.len(), 9);
    }
}