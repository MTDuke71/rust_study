// Step 4: Pathfinding Algorithms
// Tutorial Day 4 - Aligns with Mission 6 Path Finding Helpers & REQ-4
//
// Learning Objectives:
// - Implement Breadth-First Search (BFS) for unweighted pathfinding
// - Understand A* algorithm with heuristic functions
// - Learn about priority queues and algorithm optimization
// - Practice path reconstruction and visualization
//
// This tutorial step builds toward Mission6 REQ-4:
// "The system shall provide pathfinding algorithms including BFS for unweighted
//  graphs and A* for weighted pathfinding with customizable heuristic functions."

use mission6_tut::tutorial_helpers::{print_section, print_step_complete, TutorialGrid, TutorialCoord};
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use std::cmp::Reverse;

fn main() {
    println!("=== Mission 6 Tutorial - Step 4: Pathfinding Algorithms ===");
    println!("Day 4 Focus: BFS, A*, heuristics, and path reconstruction\n");

    // Section 1: Introduction to Pathfinding
    print_section("1. Introduction to Pathfinding");
    
    println!("Pathfinding algorithms help find the shortest or best path between two points.");
    println!("Common algorithms:");
    println!("• BFS (Breadth-First Search): Guarantees shortest path for unweighted graphs");
    println!("• A* (A-star): Optimal pathfinding with heuristics for weighted graphs");
    println!("• Dijkstra's: General shortest path for weighted graphs (A* with h=0)");
    
    // Create a test maze
    let mut maze = TutorialGrid::new(8, 6, '.');
    let obstacles = vec![
        TutorialCoord::new(2, 1), TutorialCoord::new(2, 2), TutorialCoord::new(2, 3),
        TutorialCoord::new(4, 1), TutorialCoord::new(4, 2), TutorialCoord::new(4, 3),
        TutorialCoord::new(5, 3), TutorialCoord::new(6, 3),
    ];
    
    for obstacle in &obstacles {
        maze.set(*obstacle, '#');
    }
    
    let start = TutorialCoord::new(0, 0);
    let goal = TutorialCoord::new(7, 5);
    maze.set(start, 'S');
    maze.set(goal, 'G');
    
    println!("\nTest maze (S=start, G=goal, #=obstacle):");
    println!("{}", maze);
    
    // Section 2: Breadth-First Search (BFS) Implementation
    print_section("2. Breadth-First Search (BFS) Implementation");
    
    println!("BFS explores all positions at distance d before exploring distance d+1");
    println!("This guarantees the shortest path in unweighted graphs.\n");
    
    let bfs_result = bfs_pathfind(&maze, start, goal);
    match bfs_result {
        Some(path) => {
            println!("BFS found path with {} steps:", path.len() - 1);
            visualize_path(&maze, &path);
            println!("Path coordinates:");
            for (i, coord) in path.iter().enumerate() {
                print!("{}", coord);
                if i < path.len() - 1 { print!(" → "); }
                if (i + 1) % 4 == 0 { println!(); }
            }
            if path.len() % 4 != 0 { println!(); }
        }
        None => println!("BFS found no path from {} to {}", start, goal),
    }
    
    // Section 3: Understanding BFS Algorithm Steps
    print_section("3. Understanding BFS Algorithm Steps");
    
    println!("BFS Algorithm Steps:");
    println!("1. Add start position to queue");
    println!("2. Mark start as visited");
    println!("3. While queue is not empty:");
    println!("   a. Dequeue current position");
    println!("   b. If current == goal, reconstruct path");
    println!("   c. For each unvisited neighbor:");
    println!("      - Mark as visited");
    println!("      - Record parent for path reconstruction");
    println!("      - Add to queue");
    
    // Demonstrate BFS exploration order
    let exploration_order = bfs_exploration_order(&maze, start);
    println!("\nBFS exploration order from {}:", start);
    
    let mut order_grid = maze.clone();
    for (step, coord) in exploration_order.iter().enumerate() {
        if maze.get(*coord) == Some(&'.') {
            let symbol = char::from_digit(step as u32 % 10, 10).unwrap_or('*');
            order_grid.set(*coord, symbol);
        }
    }
    
    println!("{}", order_grid);
    
    // Section 4: A* Algorithm Introduction
    print_section("4. A* Algorithm Introduction");
    
    println!("A* combines the guarantees of Dijkstra's with the efficiency of Greedy Best-First");
    println!("Formula: f(n) = g(n) + h(n)");
    println!("• g(n): Actual cost from start to n");
    println!("• h(n): Heuristic estimate from n to goal");
    println!("• f(n): Estimated total cost of path through n");
    
    // Demonstrate different heuristics
    let test_coord = TutorialCoord::new(3, 2);
    let manhattan_h = manhattan_distance(test_coord, goal);
    let euclidean_h = euclidean_distance(test_coord, goal);
    
    println!("\nHeuristic values for {} to {}:", test_coord, goal);
    println!("• Manhattan heuristic: {} (admissible for 4-connected)", manhattan_h);
    println!("• Euclidean heuristic: {:.2} (admissible for any movement)", euclidean_h);
    
    // Section 5: A* Algorithm Implementation
    print_section("5. A* Algorithm Implementation");
    
    let astar_result = astar_pathfind(&maze, start, goal);
    match astar_result {
        Some((path, cost)) => {
            println!("A* found path with cost {:.2} ({} steps):", cost, path.len() - 1);
            visualize_path(&maze, &path);
            
            // Compare with BFS result
            if let Some(bfs_path) = bfs_pathfind(&maze, start, goal) {
                println!("Comparison: BFS {} steps, A* {} steps", 
                        bfs_path.len() - 1, path.len() - 1);
            }
        }
        None => println!("A* found no path from {} to {}", start, goal),
    }
    
    // Section 6: Heuristic Functions Deep Dive
    print_section("6. Heuristic Functions Deep Dive");
    
    println!("Admissible heuristics never overestimate the true cost.");
    println!("This guarantees A* finds the optimal path.\n");
    
    // Test different heuristics
    let test_positions = vec![
        TutorialCoord::new(1, 1),
        TutorialCoord::new(3, 3),
        TutorialCoord::new(6, 2),
    ];
    
    println!("Heuristic comparison for goal {}:", goal);
    println!("{:<10} {:>10} {:>12} {:>12}", "Position", "Manhattan", "Euclidean", "Chebyshev");
    for pos in test_positions {
        let manhattan = manhattan_distance(pos, goal);
        let euclidean = euclidean_distance(pos, goal);
        let chebyshev = chebyshev_distance(pos, goal);
        println!("{:<10} {:>10} {:>12.2} {:>12}", 
                format!("{}", pos), manhattan, euclidean, chebyshev);
    }
    
    // Section 7: Handling Different Terrain Types
    print_section("7. Handling Different Terrain Types");
    
    // Create a weighted terrain grid
    let mut terrain_costs = HashMap::new();
    terrain_costs.insert('.', 1.0);  // Normal terrain
    terrain_costs.insert('~', 2.0);  // Water (slower)
    terrain_costs.insert('^', 3.0);  // Mountain (much slower)
    terrain_costs.insert('#', f64::INFINITY);  // Impassable
    
    let mut weighted_maze = TutorialGrid::new(6, 4, '.');
    weighted_maze.set(TutorialCoord::new(1, 1), '~');
    weighted_maze.set(TutorialCoord::new(1, 2), '~');
    weighted_maze.set(TutorialCoord::new(2, 1), '^');
    weighted_maze.set(TutorialCoord::new(3, 1), '#');
    weighted_maze.set(TutorialCoord::new(3, 2), '#');
    
    let w_start = TutorialCoord::new(0, 0);
    let w_goal = TutorialCoord::new(5, 3);
    weighted_maze.set(w_start, 'S');
    weighted_maze.set(w_goal, 'G');
    
    println!("Weighted terrain maze (.=1, ~=2, ^=3, #=∞):");
    println!("{}", weighted_maze);
    
    if let Some((path, total_cost)) = astar_weighted_pathfind(&weighted_maze, w_start, w_goal, &terrain_costs) {
        println!("\nA* with terrain costs found path with total cost: {:.1}", total_cost);
        visualize_weighted_path(&weighted_maze, &path);
    }
    
    // Section 8: Performance Comparison
    print_section("8. Performance Comparison");
    
    use std::time::Instant;
    
    // Test on a larger maze
    let large_maze = create_large_maze(20, 15);
    let large_start = TutorialCoord::new(0, 0);
    let large_goal = TutorialCoord::new(19, 14);
    
    // BFS timing
    let start_time = Instant::now();
    let bfs_large_result = bfs_pathfind(&large_maze, large_start, large_goal);
    let bfs_time = start_time.elapsed();
    
    // A* timing
    let start_time = Instant::now();
    let astar_large_result = astar_pathfind(&large_maze, large_start, large_goal);
    let astar_time = start_time.elapsed();
    
    println!("Performance on 20x15 maze:");
    if let Some(bfs_path) = bfs_large_result {
        println!("• BFS: {} steps in {:?}", bfs_path.len() - 1, bfs_time);
    }
    if let Some((astar_path, _)) = astar_large_result {
        println!("• A*:  {} steps in {:?}", astar_path.len() - 1, astar_time);
    }
    
    print_step_complete("Step 4: Pathfinding Algorithms");
    
    // Next Steps Preview
    println!("\n🔄 Next: Step 5 - AoC Utilities & Flood Fill");
    println!("   Learn flood fill, connected components, and competitive programming patterns");
    println!("   Command: cargo run --example step5_aoc_utilities");
    
    // Key Takeaways
    println!("\n📝 Key Takeaways from Step 4:");
    println!("   ✓ BFS guarantees shortest path for unweighted graphs");
    println!("   ✓ A* combines optimality with efficiency using heuristics");
    println!("   ✓ Admissible heuristics never overestimate true cost");
    println!("   ✓ Manhattan distance works well for 4-connected grids");
    println!("   ✓ Priority queues are essential for A* implementation");
}

// BFS implementation for unweighted pathfinding
fn bfs_pathfind(maze: &TutorialGrid<char>, start: TutorialCoord, goal: TutorialCoord) -> Option<Vec<TutorialCoord>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent = HashMap::new();
    
    queue.push_back(start);
    visited.insert(start);
    
    while let Some(current) = queue.pop_front() {
        if current == goal {
            return Some(reconstruct_path(&parent, start, goal));
        }
        
        for neighbor in get_neighbors(current, maze.width(), maze.height()) {
            if !visited.contains(&neighbor) && is_passable(maze, neighbor) {
                visited.insert(neighbor);
                parent.insert(neighbor, current);
                queue.push_back(neighbor);
            }
        }
    }
    
    None
}

// A* implementation with Manhattan heuristic
fn astar_pathfind(maze: &TutorialGrid<char>, start: TutorialCoord, goal: TutorialCoord) -> Option<(Vec<TutorialCoord>, f64)> {
    let mut open_set = BinaryHeap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();
    let mut parent = HashMap::new();
    
    g_score.insert(start, 0.0);
    f_score.insert(start, manhattan_distance(start, goal) as f64);
    open_set.push(Reverse(AStarNode { coord: start, f_score: f_score[&start] }));
    
    while let Some(Reverse(current_node)) = open_set.pop() {
        let current = current_node.coord;
        
        if current == goal {
            return Some((reconstruct_path(&parent, start, goal), g_score[&current]));
        }
        
        for neighbor in get_neighbors(current, maze.width(), maze.height()) {
            if !is_passable(maze, neighbor) { continue; }
            
            let tentative_g_score = g_score[&current] + 1.0;
            
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f64::INFINITY) {
                parent.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                let f_score_val = tentative_g_score + manhattan_distance(neighbor, goal) as f64;
                f_score.insert(neighbor, f_score_val);
                open_set.push(Reverse(AStarNode { coord: neighbor, f_score: f_score_val }));
            }
        }
    }
    
    None
}

// A* with weighted terrain
fn astar_weighted_pathfind(
    maze: &TutorialGrid<char>, 
    start: TutorialCoord, 
    goal: TutorialCoord,
    costs: &HashMap<char, f64>
) -> Option<(Vec<TutorialCoord>, f64)> {
    let mut open_set = BinaryHeap::new();
    let mut g_score = HashMap::new();
    let mut parent = HashMap::new();
    
    g_score.insert(start, 0.0);
    let initial_f = manhattan_distance(start, goal) as f64;
    open_set.push(Reverse(AStarNode { coord: start, f_score: initial_f }));
    
    while let Some(Reverse(current_node)) = open_set.pop() {
        let current = current_node.coord;
        
        if current == goal {
            return Some((reconstruct_path(&parent, start, goal), g_score[&current]));
        }
        
        for neighbor in get_neighbors(current, maze.width(), maze.height()) {
            let terrain = maze.get(neighbor).unwrap_or(&'#');
            let move_cost = *costs.get(terrain).unwrap_or(&f64::INFINITY);
            
            if move_cost == f64::INFINITY { continue; }
            
            let tentative_g_score = g_score[&current] + move_cost;
            
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f64::INFINITY) {
                parent.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                let f_score_val = tentative_g_score + euclidean_distance(neighbor, goal);
                open_set.push(Reverse(AStarNode { coord: neighbor, f_score: f_score_val }));
            }
        }
    }
    
    None
}

// Helper structs and functions
#[derive(Debug, PartialEq)]
struct AStarNode {
    coord: TutorialCoord,
    f_score: f64,
}

impl Eq for AStarNode {}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.f_score.partial_cmp(&other.f_score)
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

fn get_neighbors(coord: TutorialCoord, width: usize, height: usize) -> Vec<TutorialCoord> {
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)]; // N, E, S, W
    let mut neighbors = Vec::new();
    
    for (dx, dy) in directions {
        let new_x = coord.x as i32 + dx;
        let new_y = coord.y as i32 + dy;
        
        if new_x >= 0 && new_y >= 0 && (new_x as usize) < width && (new_y as usize) < height {
            neighbors.push(TutorialCoord::new(new_x as usize, new_y as usize));
        }
    }
    
    neighbors
}

fn is_passable(maze: &TutorialGrid<char>, coord: TutorialCoord) -> bool {
    match maze.get(coord) {
        Some('#') => false,
        Some(_) => true,
        None => false,
    }
}

fn reconstruct_path(parent: &HashMap<TutorialCoord, TutorialCoord>, start: TutorialCoord, goal: TutorialCoord) -> Vec<TutorialCoord> {
    let mut path = Vec::new();
    let mut current = goal;
    
    while current != start {
        path.push(current);
        current = parent[&current];
    }
    path.push(start);
    path.reverse();
    path
}

fn visualize_path(maze: &TutorialGrid<char>, path: &[TutorialCoord]) {
    let mut path_maze = maze.clone();
    for coord in path {
        if maze.get(*coord) == Some(&'.') {
            path_maze.set(*coord, '*');
        }
    }
    println!("{}", path_maze);
}

fn visualize_weighted_path(maze: &TutorialGrid<char>, path: &[TutorialCoord]) {
    let mut path_maze = maze.clone();
    for coord in path {
        let current = maze.get(*coord).unwrap_or(&'#');
        if *current != 'S' && *current != 'G' {
            path_maze.set(*coord, '*');
        }
    }
    println!("{}", path_maze);
}

fn bfs_exploration_order(maze: &TutorialGrid<char>, start: TutorialCoord) -> Vec<TutorialCoord> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut order = Vec::new();
    
    queue.push_back(start);
    visited.insert(start);
    
    while let Some(current) = queue.pop_front() {
        order.push(current);
        
        for neighbor in get_neighbors(current, maze.width(), maze.height()) {
            if !visited.contains(&neighbor) && is_passable(maze, neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }
    
    order
}

fn create_large_maze(width: usize, height: usize) -> TutorialGrid<char> {
    let mut maze = TutorialGrid::new(width, height, '.');
    
    // Add some random obstacles
    for y in 1..height-1 {
        for x in 1..width-1 {
            if (x + y) % 7 == 0 {
                maze.set(TutorialCoord::new(x, y), '#');
            }
        }
    }
    
    maze
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

// Exercise for the Reader:
// 1. Implement Dijkstra's algorithm (A* with h=0)
// 2. Create a bidirectional BFS that searches from both start and goal
// 3. Add support for diagonal movement with appropriate costs
// 4. Implement JPS (Jump Point Search) for optimized A* on uniform grids

// Design Questions to Consider:
// - When is BFS better than A* and vice versa?
// - How do different heuristics affect A* performance and optimality?
// - What data structures are most efficient for the open set in A*?
// - How would you handle dynamic obstacles that change during pathfinding?