use competitive_ring_bfs::{MazeSolver, Position};

fn main() {
    println!("🔄 RingBufferQueue Maze Solver Demo");
    println!("===================================");
    println!();
    
    // Demonstrate why RingBufferQueue is perfect for grid BFS
    demo_cache_efficiency();
    demo_memory_predictability();
    demo_competitive_programming_scenarios();
    demo_position_coordinate_analysis();
}

fn demo_cache_efficiency() {
    println!("💾 Cache Efficiency Demonstration");
    println!("----------------------------------");
    
    let maze = vec![
        "S.......".to_string(),
        "........".to_string(),
        "........".to_string(),
        "........".to_string(),
        "........".to_string(),
        "........".to_string(),
        "........".to_string(),
        ".......G".to_string(),
    ];
    
    let solver = MazeSolver::new(maze).expect("Valid maze");
    
    println!("🗺️  8x8 open grid (64 cells)");
    println!("🔄 RingBufferQueue capacity: {} (exactly grid size)", solver.required_queue_capacity());
    println!("💾 Memory layout: Contiguous Vec<Option<T>> for cache efficiency");
    println!("⚡ BFS explores cells in breadth-first order with optimal cache usage");
    
    if let Some(path) = solver.find_shortest_path() {
        println!("✅ Path found: {} steps", path.len());
        println!("🎯 Queue utilization: Perfect for this bounded problem");
    }
    
    println!();
}

fn demo_memory_predictability() {
    println!("📊 Memory Predictability");
    println!("-------------------------");
    
    // Test various grid sizes to show predictable memory usage
    let grid_sizes = vec![5, 10, 20, 50];
    
    for size in grid_sizes {
        // Create simple maze with path along the edge
        let mut maze = vec![];
        for r in 0..size {
            let mut line = String::new();
            for c in 0..size {
                if r == 0 && c == 0 {
                    line.push('S');
                } else if r == size - 1 && c == size - 1 {
                    line.push('G');
                } else if r == 0 || c == 0 || r == size - 1 || c == size - 1 {
                    line.push('.');
                } else {
                    line.push('#'); // Interior walls for controlled BFS
                }
            }
            maze.push(line);
        }
        
        let solver = MazeSolver::new(maze).expect("Valid maze");
        let capacity = solver.required_queue_capacity();
        
        println!("Grid {}x{}: Queue capacity = {} ({} KB)",
            size, size, capacity, capacity * 8 / 1024); // Rough memory estimate
    }
    
    println!();
    println!("💡 Key Insight: Memory usage is completely predictable");
    println!("   • No allocation surprises during BFS");
    println!("   • Perfect for real-time or embedded systems");
    println!("   • Optimal for competitive programming with time limits");
    println!();
}

fn demo_competitive_programming_scenarios() {
    println!("🏆 Competitive Programming Scenarios");
    println!("-------------------------------------");
    
    // Scenario 1: Codeforces-style grid problem
    demo_codeforces_style();
    
    // Scenario 2: Multi-source BFS (hospital problem)
    demo_multi_source_bfs();
    
    // Scenario 3: Bounded flood fill
    demo_flood_fill();
}

fn demo_codeforces_style() {
    println!("📝 Scenario 1: Codeforces Grid Problem");
    println!("   'Find shortest path in N×M grid with obstacles'");
    
    let maze = vec![
        "S..#....".to_string(),
        "...#....".to_string(),
        "...#....".to_string(),
        "........".to_string(),
        "####.###".to_string(),
        "........".to_string(),
        "........".to_string(),
        "......#G".to_string(),
    ];
    
    let solver = MazeSolver::new(maze).expect("Valid maze");
    
    println!("   Grid: 8×8, Obstacles: #, Start: S, Goal: G");
    println!("   Queue capacity needed: {} (= 64)", solver.required_queue_capacity());
    
    if let Some(path) = solver.find_shortest_path() {
        println!("   ✅ Solution: {} steps", path.len());
        println!("   ⚡ RingBufferQueue ensures O(1) enqueue/dequeue with no allocation overhead");
    }
    println!();
}

fn demo_multi_source_bfs() {
    println!("📝 Scenario 2: Multi-Source BFS Pattern");
    println!("   'Find nearest hospital from any position'");
    
    // Create a grid with multiple hospitals
    let maze = vec![
        "H.....".to_string(),  // H = Hospital (multiple sources)
        "......".to_string(),
        "..##..".to_string(),
        "..##..".to_string(),
        "......".to_string(),
        ".....H".to_string(),
    ];
    
    println!("   Grid: 6×6, Hospitals: H (multiple BFS sources)");
    println!("   Queue capacity: {} (bounded by grid size)", 36);
    println!("   💡 RingBufferQueue perfect for bounded multi-source BFS");
    println!("   ⚡ All sources start in queue simultaneously, bounded growth");
    
    // Demonstrate actual multi-source BFS simulation
    println!("\n   🏥 Multi-Source BFS Simulation:");
    println!("   Find distance from every cell to nearest hospital");
    
    // For demonstration, we'll simulate multi-source BFS logic
    simulate_multi_source_bfs(&maze);
    
    println!();
}

fn demo_flood_fill() {
    println!("📝 Scenario 3: Bounded Flood Fill");
    println!("   'Paint connected region, count cells affected'");
    
    let maze = vec![
        "S.....".to_string(),
        "......".to_string(),
        "###.##".to_string(),
        "......".to_string(),
        "......".to_string(),
        ".....G".to_string(),
    ];
    
    let solver = MazeSolver::new(maze).expect("Valid maze");
    
    println!("   Grid: 6×6, Connected region to fill");
    println!("   Queue capacity: {} (exactly what's needed)", solver.required_queue_capacity());
    
    if let Some(path) = solver.find_shortest_path() {
        println!("   ✅ Path exists: {} cells in shortest path", path.len());
        println!("   💡 BFS explores exactly the cells it needs to visit");
        println!("   🔄 RingBufferQueue grows to at most the number of boundary cells");
    }
    
    println!();
    println!("🎯 Summary: RingBufferQueue Excels When...");
    println!("   ✓ Problem size is bounded (grid dimensions known)");
    println!("   ✓ Cache performance matters (competitive programming)");
    println!("   ✓ Memory allocation must be predictable");
    println!("   ✓ Multiple BFS runs need consistent performance");
    println!("   ✓ Real-time constraints require no allocation overhead");
}

fn demo_position_coordinate_analysis() {
    println!("🗺️ Position Coordinate Analysis");
    println!("--------------------------------");
    
    let maze = vec![
        "S....".to_string(),
        ".###.".to_string(),
        ".....".to_string(),
        "###..".to_string(),
        "....G".to_string(),
    ];
    
    let solver = MazeSolver::new(maze).expect("Valid maze");
    
    // Show start and goal positions
    let start = solver.start();
    let goal = solver.goal();
    
    println!("📍 Start position: ({}, {})", start.row, start.col);
    println!("📍 Goal position: ({}, {})", goal.row, goal.col);
    println!("📏 Manhattan distance: {}", 
        (goal.row as i32 - start.row as i32).abs() + 
        (goal.col as i32 - start.col as i32).abs());
    
    // Demonstrate neighbor generation
    println!("\n🧭 Neighbors from start position:");
    let (rows, cols) = solver.dimensions();
    for neighbor in start.neighbors(rows, cols) {
        println!("   → ({}, {})", neighbor.row, neighbor.col);
    }
    
    // Demonstrate explicit Position creation
    println!("\n🔧 Position Creation Examples:");
    let example_pos = Position::new(2, 3);
    println!("   Position::new(2, 3) = ({}, {})", example_pos.row, example_pos.col);
    
    let center = Position::new(rows / 2, cols / 2);
    println!("   Grid center ≈ Position::new({}, {}) = ({}, {})", 
        rows / 2, cols / 2, center.row, center.col);
    
    // Find and display the actual shortest path
    if let Some(path) = solver.find_shortest_path() {
        println!("\n✅ Shortest path found: {} steps", path.len());
        println!("🛤️  Path coordinates:");
        
        for (step, position) in path.iter().enumerate() {
            let marker = if step == 0 {
                "🚀 START"
            } else if step == path.len() - 1 {
                "🎯 GOAL "
            } else {
                "📍     "
            };
            
            println!("   Step {:2}: ({}, {}) {}", 
                step, position.row, position.col, marker);
        }
        
        // Show path efficiency
        let manhattan = (goal.row as i32 - start.row as i32).abs() + 
                       (goal.col as i32 - start.col as i32).abs();
        let actual_distance = path.len() - 1; // Subtract 1 because path includes start
        
        println!("\n📊 Path Analysis:");
        println!("   Manhattan distance: {} steps", manhattan);
        println!("   Actual path length: {} steps", actual_distance);
        println!("   Path efficiency: {:.1}% (lower is better due to obstacles)", 
            (manhattan as f64 / actual_distance as f64) * 100.0);
        
        // Demonstrate Position usage in algorithm context
        println!("\n🔍 Algorithm Insights:");
        println!("   • RingBufferQueue stores BfsState containing Position");
        println!("   • Each Position generates up to 4 neighbors via neighbors()");
        println!("   • Position::new(row, col) creates coordinate pairs");
        println!("   • BFS explores positions level-by-level for optimality");
        
    } else {
        println!("\n❌ No path exists between start and goal");
    }
    
    println!("\n💡 Position Type Benefits:");
    println!("   ✓ Copy semantics: cheap to pass around in BFS");
    println!("   ✓ Hash + Eq: perfect for visited set tracking");
    println!("   ✓ Debug: easy visualization during development");
    println!("   ✓ Ergonomic: .row and .col fields for clear code");
}

fn simulate_multi_source_bfs(maze: &[String]) {
    use mission2::RingBufferQueue;
    use std::collections::HashMap;
    
    let rows = maze.len();
    let cols = maze[0].len();
    
    // Find all hospital positions (H)
    let mut hospitals = Vec::new();
    for (r, line) in maze.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == 'H' {
                hospitals.push(Position::new(r, c));
            }
        }
    }
    
    println!("   Found {} hospitals at:", hospitals.len());
    for (i, pos) in hospitals.iter().enumerate() {
        println!("     Hospital {}: ({}, {})", i + 1, pos.row, pos.col);
    }
    
    // Multi-source BFS: Initialize queue with ALL hospitals at distance 0
    let mut queue = RingBufferQueue::with_capacity(rows * cols);
    let mut distances: HashMap<Position, usize> = HashMap::new();
    
    // Add all hospitals to queue simultaneously (multi-source start)
    for &hospital in &hospitals {
        queue.enqueue((hospital, 0)).expect("Queue has sufficient capacity");
        distances.insert(hospital, 0);
    }
    
    println!("\n   🔄 BFS Process:");
    println!("     Initial queue size: {} (all hospitals)", queue.len());
    
    let mut step = 0;
    while let Some((current_pos, current_dist)) = queue.dequeue() {
        step += 1;
        
        // Explore neighbors
        for neighbor in current_pos.neighbors(rows, cols) {
            // Skip if already visited or if it's a wall
            if distances.contains_key(&neighbor) {
                continue;
            }
            
            // Check if neighbor is traversable (not wall '#')
            if let Some(line) = maze.get(neighbor.row) {
                if let Some(ch) = line.chars().nth(neighbor.col) {
                    if ch == '#' {
                        continue; // Skip walls
                    }
                }
            }
            
            // Add to queue with incremented distance
            let new_distance = current_dist + 1;
            queue.enqueue((neighbor, new_distance)).expect("Queue capacity sufficient");
            distances.insert(neighbor, new_distance);
        }
    }
    
    println!("     Total BFS steps: {}", step);
    println!("     Cells reached: {}", distances.len());
    
    // Display distance grid
    println!("\n   📊 Distance Map (distance to nearest hospital):");
    for r in 0..rows {
        print!("     ");
        for c in 0..cols {
            let pos = Position::new(r, c);
            if let Some(line) = maze.get(r) {
                if let Some(ch) = line.chars().nth(c) {
                    if ch == '#' {
                        print!("# ");
                    } else if ch == 'H' {
                        print!("H ");
                    } else if let Some(&dist) = distances.get(&pos) {
                        print!("{} ", dist);
                    } else {
                        print!("? ");
                    }
                }
            }
        }
        println!();
    }
    
    // Find the position farthest from any hospital
    let max_distance = distances.values().max().unwrap_or(&0);
    println!("\n   📏 Analysis:");
    println!("     Maximum distance to hospital: {} steps", max_distance);
    println!("     RingBufferQueue peak usage: ~{} entries", distances.len());
    println!("     💡 Multi-source BFS guarantees shortest distance to ANY hospital");
}