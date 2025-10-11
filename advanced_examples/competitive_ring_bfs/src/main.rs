use competitive_ring_bfs::{MazeSolver, Position};

fn main() {
    println!("🔄 Competitive Programming: BFS Maze Solver with RingBufferQueue");
    println!("================================================================");
    println!();

    // Example 0: Solvable maze with clear path
    demo_solvable_maze();
    println!();

    // Example 1: Simple maze
    demo_simple_maze();
    println!();

    // Example 2: Complex maze with obstacles
    demo_complex_maze();
    println!();

    // Example 3: No solution case
    demo_no_solution();
    println!();

    // Example 4: Performance characteristics
    demo_performance_analysis();
}

fn demo_solvable_maze() {
    println!("📍 Example 0: Solvable Maze with Clear Path");
    println!("--------------------------------------------");

    let maze = vec![
        "S...#".to_string(),
        ".##.#".to_string(),
        "....#".to_string(),
        "###..".to_string(),
        "....G".to_string(),
    ];

    println!("Maze:");
    for line in &maze {
        println!("  {}", line);
    }
    println!();

    let solver = MazeSolver::new(maze.clone()).expect("Valid maze");

    match solver.find_shortest_path() {
        Some(path) => {
            println!("✅ Shortest path found! Length: {} steps", path.len());
            println!("🎯 Start: ({}, {})", solver.start().row, solver.start().col);
            println!("🎯 Goal:  ({}, {})", solver.goal().row, solver.goal().col);
            println!("💾 Grid size: {:?}", solver.dimensions());
            println!("🔄 Queue capacity: {}", solver.required_queue_capacity());

            println!("\n🛤️  Complete path coordinates:");
            for (step, pos) in path.iter().enumerate() {
                let marker = if step == 0 {
                    "🚀 START"
                } else if step == path.len() - 1 {
                    "🎯 GOAL "
                } else {
                    "📍     "
                };
                println!("   Step {:2}: ({}, {}) {}", step, pos.row, pos.col, marker);
            }

            // Show path visualization on the maze
            println!("\n🗺️  Path Visualization:");
            visualize_path(&maze, &path);
        }
        None => {
            println!("❌ No path exists");
        }
    }
}

fn demo_simple_maze() {
    println!("📍 Example 1: Simple 4x4 Maze");
    println!("------------------------------");

    let maze = vec![
        "S...".to_string(),
        ".##.".to_string(),
        "...#".to_string(),
        "..#G".to_string(),
    ];

    println!("Maze:");
    for line in &maze {
        println!("  {}", line);
    }
    println!();

    let solver = MazeSolver::new(maze).expect("Valid maze");

    match solver.find_shortest_path() {
        Some(path) => {
            println!("✅ Path found! Length: {} steps", path.len());
            println!("📍 Path coordinates:");
            for (i, pos) in path.iter().enumerate() {
                println!("  Step {}: ({}, {})", i, pos.row, pos.col);
            }
            println!(
                "🎯 Queue capacity used: {}",
                solver.required_queue_capacity()
            );
        }
        None => {
            println!("❌ No path exists");
        }
    }
}

fn demo_complex_maze() {
    println!("📍 Example 2: Complex Maze with Multiple Paths");
    println!("-----------------------------------------------");

    let maze = vec![
        "S.....".to_string(),
        ".####.".to_string(),
        ".....#".to_string(),
        "####.#".to_string(),
        ".....#".to_string(),
        ".####G".to_string(),
    ];

    println!("Maze:");
    for line in &maze {
        println!("  {}", line);
    }
    println!();

    let solver = MazeSolver::new(maze.clone()).expect("Valid maze");

    match solver.find_shortest_path() {
        Some(path) => {
            println!("✅ Shortest path found! Length: {} steps", path.len());
            println!("🎯 Start: ({}, {})", solver.start().row, solver.start().col);
            println!("🎯 Goal:  ({}, {})", solver.goal().row, solver.goal().col);
            println!("💾 Grid size: {:?}", solver.dimensions());
            println!("🔄 Queue capacity: {}", solver.required_queue_capacity());

            // Show path visualization
            visualize_path(&maze, &path);
        }
        None => {
            println!("❌ No path exists");
        }
    }
}

fn demo_no_solution() {
    println!("📍 Example 3: Impossible Maze (No Solution)");
    println!("--------------------------------------------");

    let maze = vec!["S.#".to_string(), "###".to_string(), "#.G".to_string()];

    println!("Maze:");
    for line in &maze {
        println!("  {}", line);
    }
    println!();

    let solver = MazeSolver::new(maze).expect("Valid maze");

    match solver.find_shortest_path() {
        Some(_) => {
            println!("❓ Unexpected: Path found");
        }
        None => {
            println!("✅ Correctly detected: No path exists");
            println!(
                "🔄 Queue capacity was: {}",
                solver.required_queue_capacity()
            );
            println!("💡 BFS explored all reachable cells and stopped");
        }
    }
}

fn demo_performance_analysis() {
    println!("📍 Example 4: Performance Analysis");
    println!("-----------------------------------");

    // Test different grid sizes to show scaling
    let sizes = vec![(3, 3), (5, 5), (10, 10), (20, 20)];

    for (rows, cols) in sizes {
        // Create a maze with path along edges
        let mut maze = vec![];

        for r in 0..rows {
            let mut line = String::new();
            for c in 0..cols {
                if r == 0 && c == 0 {
                    line.push('S'); // Start
                } else if r == rows - 1 && c == cols - 1 {
                    line.push('G'); // Goal
                } else if r == 0 || c == 0 || r == rows - 1 || c == cols - 1 {
                    line.push('.'); // Edge path
                } else {
                    line.push('#'); // Interior walls
                }
            }
            maze.push(line);
        }

        let solver = MazeSolver::new(maze).expect("Valid maze");

        if let Some(path) = solver.find_shortest_path() {
            println!(
                "📊 Grid {}x{}: Path length = {}, Queue capacity = {}",
                rows,
                cols,
                path.len(),
                solver.required_queue_capacity()
            );
        }
    }

    println!();
    println!("💡 Key Insights:");
    println!("  • RingBufferQueue capacity = rows × cols (worst case)");
    println!("  • Fixed memory allocation prevents runtime overhead");
    println!("  • Cache-friendly contiguous memory layout");
    println!("  • Perfect for competitive programming with known bounds");
}

fn visualize_path(maze: &[String], path: &[Position]) {
    println!("🗺️  Path visualization (* = path):");

    // Convert path to set for O(1) lookup
    use std::collections::HashSet;
    let path_set: HashSet<_> = path.iter().collect();

    for (row, line) in maze.iter().enumerate() {
        print!("  ");
        for (col, ch) in line.chars().enumerate() {
            let pos = Position::new(row, col);
            if path_set.contains(&pos) && ch != 'S' && ch != 'G' {
                print!("*"); // Mark path
            } else {
                print!("{}", ch); // Original character
            }
        }
        println!();
    }
}
