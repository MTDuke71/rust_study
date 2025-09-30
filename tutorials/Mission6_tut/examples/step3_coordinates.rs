// Step 3: Coordinate Systems & Navigation
// Tutorial Day 3 - Aligns with Mission 6 Coordinate Systems & Navigation & REQ-3
//
// Learning Objectives:
// - Master 2D coordinate arithmetic and navigation
// - Understand different neighbor patterns (4-connected vs 8-connected)
// - Implement distance calculations (Manhattan, Euclidean, Chebyshev)
// - Practice coordinate transformations and boundary handling
//
// This tutorial step builds toward Mission6 REQ-3:
// "The system shall provide coordinate navigation utilities including neighbor
//  finding, distance calculations, and direction-based movement operations."

use mission6_tut::tutorial_helpers::{print_section, print_step_complete, TutorialGrid, TutorialCoord};

fn main() {
    println!("=== Mission 6 Tutorial - Step 3: Coordinate Systems & Navigation ===");
    println!("Day 3 Focus: 2D mathematics, neighbor finding, and distance calculations\n");

    // Section 1: Coordinate System Fundamentals
    print_section("1. Coordinate System Fundamentals");
    
    // Understanding coordinate conventions
    println!("Coordinate system conventions:");
    println!("• Screen coordinates: (0,0) at top-left, Y increases downward");
    println!("• Mathematical coordinates: (0,0) at bottom-left, Y increases upward");
    println!("• Grid coordinates: We use screen convention for consistency\n");
    
    let demo_grid = TutorialGrid::new(5, 4, '.');
    
    // Show coordinate positions
    println!("5x4 grid with coordinate labels:");
    for y in 0..demo_grid.height() {
        for x in 0..demo_grid.width() {
            print!("({},{}) ", x, y);
        }
        println!();
    }
    
    // Section 2: Basic Coordinate Arithmetic
    print_section("2. Basic Coordinate Arithmetic");
    
    let center = TutorialCoord::new(2, 2);
    println!("Starting at center: {}", center);
    
    // Direction vectors (deltas)
    let directions = vec![
        (0, -1, "North"),    // Up
        (1, 0, "East"),      // Right
        (0, 1, "South"),     // Down
        (-1, 0, "West"),     // Left
    ];
    
    println!("\nCardinal directions from center:");
    for (dx, dy, name) in directions {
        // Safe coordinate arithmetic with bounds checking
        if let Some(new_coord) = move_coordinate(center, dx, dy, 5, 4) {
            println!("  {} → {}", name, new_coord);
        } else {
            println!("  {} → Out of bounds", name);
        }
    }
    
    // Section 3: Neighbor Finding (4-connected)
    print_section("3. Neighbor Finding (4-connected)");
    
    let test_coord = TutorialCoord::new(2, 1);
    println!("Finding 4-connected neighbors of {}", test_coord);
    
    let neighbors_4 = get_neighbors_4(test_coord, 5, 4);
    println!("4-connected neighbors ({} total):", neighbors_4.len());
    for neighbor in &neighbors_4 {
        println!("  {}", neighbor);
    }
    
    // Visualize neighbors on grid
    let mut neighbor_grid = TutorialGrid::new(5, 4, '.');
    neighbor_grid.set(test_coord, 'C'); // Center
    for neighbor in &neighbors_4 {
        neighbor_grid.set(*neighbor, '#');
    }
    
    println!("\nVisualization (C=center, #=neighbors):");
    println!("{}", neighbor_grid);
    
    // Section 4: Neighbor Finding (8-connected)
    print_section("4. Neighbor Finding (8-connected)");
    
    let neighbors_8 = get_neighbors_8(test_coord, 5, 4);
    println!("8-connected neighbors of {}: {} total", test_coord, neighbors_8.len());
    for neighbor in &neighbors_8 {
        println!("  {}", neighbor);
    }
    
    // Visualize 8-connected neighbors
    let mut neighbor_8_grid = TutorialGrid::new(5, 4, '.');
    neighbor_8_grid.set(test_coord, 'C');
    for neighbor in &neighbors_8 {
        neighbor_8_grid.set(*neighbor, '#');
    }
    
    println!("\n8-connected visualization:");
    println!("{}", neighbor_8_grid);
    
    // Section 5: Distance Calculations
    print_section("5. Distance Calculations");
    
    let point_a = TutorialCoord::new(1, 1);
    let point_b = TutorialCoord::new(4, 3);
    
    println!("Distance calculations between {} and {}:", point_a, point_b);
    
    // Manhattan distance (L1 norm)
    let manhattan = manhattan_distance(point_a, point_b);
    println!("  Manhattan distance: {} (|Δx| + |Δy|)", manhattan);
    
    // Euclidean distance (L2 norm)  
    let euclidean = euclidean_distance(point_a, point_b);
    println!("  Euclidean distance: {:.2} (√(Δx² + Δy²))", euclidean);
    
    // Chebyshev distance (L∞ norm)
    let chebyshev = chebyshev_distance(point_a, point_b);
    println!("  Chebyshev distance: {} (max(|Δx|, |Δy|))", chebyshev);
    
    // Section 6: Direction-Based Movement
    print_section("6. Direction-Based Movement");
    
    #[derive(Debug, Clone, Copy)]
    #[allow(dead_code)]
    enum Direction {
        North, Northeast, East, Southeast, South, Southwest, West, Northwest
    }
    
    impl Direction {
        fn to_delta(self) -> (i32, i32) {
            match self {
                Direction::North => (0, -1),
                Direction::Northeast => (1, -1),
                Direction::East => (1, 0),
                Direction::Southeast => (1, 1),
                Direction::South => (0, 1),
                Direction::Southwest => (-1, 1),
                Direction::West => (-1, 0),
                Direction::Northwest => (-1, -1),
            }
        }
    }
    
    let start_pos = TutorialCoord::new(2, 2);
    println!("Movement from {} in different directions:", start_pos);
    
    let movements = vec![
        Direction::North,
        Direction::Northeast,
        Direction::East,
        Direction::Southeast,
    ];
    
    for direction in movements {
        let (dx, dy) = direction.to_delta();
        if let Some(new_pos) = move_coordinate(start_pos, dx, dy, 5, 5) {
            println!("  {:?}: {} → {}", direction, start_pos, new_pos);
        } else {
            println!("  {:?}: {} → Out of bounds", direction, start_pos);
        }
    }
    
    // Section 7: Coordinate Transformation Patterns
    print_section("7. Coordinate Transformation Patterns");
    
    let grid_width = 5;
    let grid_height = 4;
    
    // Linear index to coordinate conversion
    println!("Linear index to coordinate conversion (row-major):");
    for index in 0..20 {
        let coord = index_to_coord(index, grid_width);
        if coord.x < grid_width && coord.y < grid_height {
            let back_to_index = coord_to_index(coord, grid_width);
            println!("  Index {} → {} → Index {}", index, coord, back_to_index);
        }
    }
    
    // Section 8: Boundary Handling Strategies
    print_section("8. Boundary Handling Strategies");
    
    let boundary_coord = TutorialCoord::new(4, 3); // Near boundary
    println!("Testing boundary handling for coordinate {}", boundary_coord);
    
    // Strategy 1: Clamp to bounds
    let clamped_neighbors = get_neighbors_4_clamped(boundary_coord, 5, 4);
    println!("Clamped 4-neighbors: {} total", clamped_neighbors.len());
    for neighbor in &clamped_neighbors {
        println!("  {}", neighbor);
    }
    
    // Strategy 2: Wrap around (toroidal topology)
    let wrapped_neighbors = get_neighbors_4_wrapped(boundary_coord, 5, 4);
    println!("Wrapped 4-neighbors: {} total", wrapped_neighbors.len());
    for neighbor in &wrapped_neighbors {
        println!("  {}", neighbor);
    }
    
    // Section 9: Practical Example - Pathfinding Setup
    print_section("9. Practical Example: Pathfinding Setup");
    
    let mut maze_grid = TutorialGrid::new(7, 5, '.');
    
    // Create a simple maze
    let obstacles = vec![
        TutorialCoord::new(2, 1),
        TutorialCoord::new(2, 2),
        TutorialCoord::new(2, 3),
        TutorialCoord::new(4, 1),
        TutorialCoord::new(4, 2),
    ];
    
    for obstacle in obstacles {
        maze_grid.set(obstacle, '#');
    }
    
    let start = TutorialCoord::new(0, 0);
    let goal = TutorialCoord::new(6, 4);
    
    maze_grid.set(start, 'S');
    maze_grid.set(goal, 'G');
    
    println!("Simple maze (S=start, G=goal, #=obstacle):");
    println!("{}", maze_grid);
    
    // Calculate straight-line distance (heuristic for A*)
    let heuristic_distance = euclidean_distance(start, goal);
    println!("\nStraight-line distance from start to goal: {:.2}", heuristic_distance);
    
    // Find reachable positions from start (one step)
    let reachable_from_start = get_neighbors_4(start, 7, 5);
    println!("Positions reachable from start in one step:");
    for pos in reachable_from_start {
        if maze_grid.get(pos).unwrap() != &'#' {
            println!("  {} (distance: {})", pos, manhattan_distance(start, pos));
        }
    }
    
    print_step_complete("Step 3: Coordinate Systems & Navigation");
    
    // Next Steps Preview
    println!("\n🔄 Next: Step 4 - Pathfinding Algorithms");
    println!("   Learn BFS and A* algorithms for grid navigation");
    println!("   Command: cargo run --example step4_pathfinding");
    
    // Key Takeaways
    println!("\n📝 Key Takeaways from Step 3:");
    println!("   ✓ Screen coordinates: (0,0) at top-left, Y increases downward");
    println!("   ✓ 4-connected neighbors: N,E,S,W (Manhattan distance)");
    println!("   ✓ 8-connected neighbors: includes diagonals (Chebyshev distance)");
    println!("   ✓ Manhattan distance: |Δx| + |Δy| (grid-based movement)");
    println!("   ✓ Euclidean distance: √(Δx² + Δy²) (straight-line distance)");
    println!("   ✓ Always handle boundary conditions in coordinate arithmetic");
}

// Helper functions for coordinate operations
fn move_coordinate(coord: TutorialCoord, dx: i32, dy: i32, width: usize, height: usize) -> Option<TutorialCoord> {
    let new_x = coord.x as i32 + dx;
    let new_y = coord.y as i32 + dy;
    
    if new_x >= 0 && new_y >= 0 && (new_x as usize) < width && (new_y as usize) < height {
        Some(TutorialCoord::new(new_x as usize, new_y as usize))
    } else {
        None
    }
}

fn get_neighbors_4(coord: TutorialCoord, width: usize, height: usize) -> Vec<TutorialCoord> {
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)]; // N, E, S, W
    let mut neighbors = Vec::new();
    
    for (dx, dy) in directions {
        if let Some(neighbor) = move_coordinate(coord, dx, dy, width, height) {
            neighbors.push(neighbor);
        }
    }
    
    neighbors
}

fn get_neighbors_8(coord: TutorialCoord, width: usize, height: usize) -> Vec<TutorialCoord> {
    let directions = [
        (0, -1), (1, -1), (1, 0), (1, 1),    // N, NE, E, SE
        (0, 1), (-1, 1), (-1, 0), (-1, -1)   // S, SW, W, NW
    ];
    let mut neighbors = Vec::new();
    
    for (dx, dy) in directions {
        if let Some(neighbor) = move_coordinate(coord, dx, dy, width, height) {
            neighbors.push(neighbor);
        }
    }
    
    neighbors
}

fn manhattan_distance(a: TutorialCoord, b: TutorialCoord) -> usize {
    let dx = if a.x > b.x { a.x - b.x } else { b.x - a.x };
    let dy = if a.y > b.y { a.y - b.y } else { b.y - a.y };
    dx + dy
}

fn euclidean_distance(a: TutorialCoord, b: TutorialCoord) -> f64 {
    let dx = (a.x as f64) - (b.x as f64);
    let dy = (a.y as f64) - (b.y as f64);
    (dx * dx + dy * dy).sqrt()
}

fn chebyshev_distance(a: TutorialCoord, b: TutorialCoord) -> usize {
    let dx = if a.x > b.x { a.x - b.x } else { b.x - a.x };
    let dy = if a.y > b.y { a.y - b.y } else { b.y - a.y };
    dx.max(dy)
}

fn index_to_coord(index: usize, width: usize) -> TutorialCoord {
    TutorialCoord::new(index % width, index / width)
}

fn coord_to_index(coord: TutorialCoord, width: usize) -> usize {
    coord.y * width + coord.x
}

fn get_neighbors_4_clamped(coord: TutorialCoord, width: usize, height: usize) -> Vec<TutorialCoord> {
    let mut neighbors = Vec::new();
    
    // Clamp coordinates to grid boundaries
    if coord.y > 0 { neighbors.push(TutorialCoord::new(coord.x, coord.y - 1)); } // North
    if coord.x < width - 1 { neighbors.push(TutorialCoord::new(coord.x + 1, coord.y)); } // East
    if coord.y < height - 1 { neighbors.push(TutorialCoord::new(coord.x, coord.y + 1)); } // South
    if coord.x > 0 { neighbors.push(TutorialCoord::new(coord.x - 1, coord.y)); } // West
    
    neighbors
}

fn get_neighbors_4_wrapped(coord: TutorialCoord, width: usize, height: usize) -> Vec<TutorialCoord> {
    vec![
        TutorialCoord::new(coord.x, (coord.y + height - 1) % height), // North (wrap)
        TutorialCoord::new((coord.x + 1) % width, coord.y),           // East (wrap)
        TutorialCoord::new(coord.x, (coord.y + 1) % height),          // South (wrap)
        TutorialCoord::new((coord.x + width - 1) % width, coord.y),   // West (wrap)
    ]
}

// Exercise for the Reader:
// 1. Implement a function that finds all coordinates within a certain distance
// 2. Create a "line_of_sight" function that checks if two coordinates have a clear path
// 3. Write a function that rotates a coordinate around a center point
// 4. Implement different distance metrics (e.g., weighted Manhattan distance)

// Design Questions to Consider:
// - When should you use 4-connected vs 8-connected neighbors?
// - How do different distance metrics affect pathfinding behavior?
// - What are the trade-offs between wrapped vs clamped boundary handling?
// - How would you optimize neighbor finding for frequently accessed coordinates?