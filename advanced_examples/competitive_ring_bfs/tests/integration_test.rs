use competitive_ring_bfs::MazeSolver;
use std::fs;

/// Integration tests for maze solver using real data files
/// 
/// # Requirements Satisfied: REQ-G1, REQ-B1, REQ-B5, REQ-P2, REQ-P4
/// Tests the complete pipeline from file parsing to path finding

#[test] // REQ-G1, REQ-B1, REQ-B5
fn test_small_maze_from_file() {
    let content = fs::read_to_string("tests/data/maze_small.txt")
        .expect("Should read test data file");
    
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let solver = MazeSolver::new(lines).expect("Valid maze format");
    
    let path = solver.find_shortest_path().expect("Path should exist");
    
    // 2x2 maze: S. / .G should have path length 3
    assert_eq!(path.len(), 3);
    assert_eq!(path[0], solver.start());
    assert_eq!(path[path.len() - 1], solver.goal());
}

#[test] // REQ-B1, REQ-B2, REQ-B5
fn test_medium_maze_from_file() {
    let content = fs::read_to_string("tests/data/maze_medium.txt")
        .expect("Should read test data file");
    
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let solver = MazeSolver::new(lines).expect("Valid maze format");
    
    let path = solver.find_shortest_path().expect("Path should exist");
    
    // Verify path properties
    assert!(path.len() > 10); // Should be reasonably long path
    assert_eq!(path[0], solver.start());
    assert_eq!(path[path.len() - 1], solver.goal());
    
    // Verify path is contiguous
    for window in path.windows(2) {
        let curr = window[0];
        let next = window[1];
        let neighbors = curr.neighbors(8, 8); // 8x8 grid
        assert!(neighbors.contains(&next), "Path must be contiguous");
    }
}

#[test] // REQ-P4
fn test_no_solution_maze_from_file() {
    let content = fs::read_to_string("tests/data/maze_no_solution.txt")
        .expect("Should read test data file");
    
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let solver = MazeSolver::new(lines).expect("Valid maze format");
    
    let path = solver.find_shortest_path();
    
    // Should correctly detect no path exists
    assert!(path.is_none(), "Should detect unreachable goal");
}

#[test] // REQ-B3
fn test_queue_capacity_analysis() {
    let test_cases = vec![
        ("tests/data/maze_small.txt", 4),   // 2x2 = 4
        ("tests/data/maze_medium.txt", 64), // 8x8 = 64
        ("tests/data/maze_no_solution.txt", 9), // 3x3 = 9
    ];
    
    for (file, expected_capacity) in test_cases {
        let content = fs::read_to_string(file)
            .expect("Should read test data file");
        
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let solver = MazeSolver::new(lines).expect("Valid maze format");
        
        assert_eq!(solver.required_queue_capacity(), expected_capacity);
    }
}

#[test] // REQ-B1, REQ-B5
fn test_optimal_path_guarantee() {
    // Create maze where multiple paths exist, verify BFS finds shortest
    let maze = vec![
        "S....".to_string(),
        ".###.".to_string(),
        ".....".to_string(),
        ".###.".to_string(),
        "....G".to_string(),
    ];
    
    let solver = MazeSolver::new(maze).expect("Valid maze");
    let path = solver.find_shortest_path().expect("Path should exist");
    
    // BFS guarantees shortest path (multiple optimal paths exist here)
    // Path 1 (top): S->right->right->right->right->down->down->down->down->G = 9 steps
    // Path 2 (bottom): S->down->down->down->down->right->right->right->right->G = 9 steps
    // BFS will find one of the optimal paths (typically the "bottom" path due to neighbor exploration order)
    assert_eq!(path.len(), 9, "BFS should find optimal path length");
    
    // Verify it's actually one of the two optimal paths
    let is_top_path = path[1].row == 0 && path[1].col == 1; // Goes right first
    let is_bottom_path = path[1].row == 1 && path[1].col == 0; // Goes down first
    
    assert!(is_top_path || is_bottom_path, 
        "Should choose one of the two optimal paths, but got: {:?}", path);
}

#[test] // REQ-G1, REQ-G2, REQ-G4
fn test_maze_format_validation() {
    // Test various invalid maze formats
    
    // Missing start
    let invalid_maze1 = vec![
        "...".to_string(),
        "...".to_string(),
        "..G".to_string(),
    ];
    assert!(MazeSolver::new(invalid_maze1).is_none());
    
    // Missing goal
    let invalid_maze2 = vec![
        "S..".to_string(),
        "...".to_string(),
        "...".to_string(),
    ];
    assert!(MazeSolver::new(invalid_maze2).is_none());
    
    // Different row lengths
    let invalid_maze3 = vec![
        "S..".to_string(),
        "..".to_string(),  // Too short
        "..G".to_string(),
    ];
    assert!(MazeSolver::new(invalid_maze3).is_none());
    
    // Multiple starts
    let invalid_maze4 = vec![
        "S.S".to_string(),
        "...".to_string(),
        "..G".to_string(),
    ];
    assert!(MazeSolver::new(invalid_maze4).is_none());
}

#[test] // REQ-B3
fn test_ringbuffer_capacity_efficiency() {
    // Test that RingBufferQueue capacity exactly matches grid size
    let sizes = vec![3, 5, 10];
    
    for size in sizes {
        // Create size×size grid
        let mut maze = vec![];
        for r in 0..size {
            let mut line = String::new();
            for c in 0..size {
                if r == 0 && c == 0 {
                    line.push('S');
                } else if r == size - 1 && c == size - 1 {
                    line.push('G');
                } else {
                    line.push('.');
                }
            }
            maze.push(line);
        }
        
        let solver = MazeSolver::new(maze).expect("Valid maze");
        
        // Capacity should exactly equal grid size
        assert_eq!(solver.required_queue_capacity(), size * size);
        
        // Should still find path
        let path = solver.find_shortest_path().expect("Path should exist");
        assert!(path.len() >= 2); // At least start and goal
    }
}

#[test] // REQ-P2, REQ-P3
fn test_path_reconstruction_accuracy() {
    let maze = vec![
        "S...".to_string(),
        "....".to_string(),
        "....".to_string(),
        "...G".to_string(),
    ];
    
    let solver = MazeSolver::new(maze).expect("Valid maze");
    let path = solver.find_shortest_path().expect("Path should exist");
    
    // Verify path starts and ends correctly
    assert_eq!(path[0], solver.start());
    assert_eq!(path[path.len() - 1], solver.goal());
    
    // Verify each step in path is valid (adjacent cells)
    for window in path.windows(2) {
        let from = window[0];
        let to = window[1];
        
        // Check if move is exactly one step in cardinal direction
        let row_diff = (from.row as i32 - to.row as i32).abs();
        let col_diff = (from.col as i32 - to.col as i32).abs();
        
        assert_eq!(row_diff + col_diff, 1, "Path steps must be adjacent");
    }
}