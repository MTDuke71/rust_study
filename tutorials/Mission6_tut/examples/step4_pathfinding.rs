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
    println!("\nBFS exploration order (visit sequence) from {}:", start);
    
    let mut order_grid = maze.clone();
    for (step, coord) in exploration_order.iter().enumerate() {
        if maze.get(*coord) == Some(&'.') {
            let symbol = char::from_digit(step as u32 % 10, 10).unwrap_or('*');
            order_grid.set(*coord, symbol);
        }
    }
    
    println!("{}", order_grid);
    println!("(Numbers show visit order, not distance)");
    
    // Demonstrate BFS distance from start
    let distance_map = bfs_distance_map(&maze, start);
    println!("\nBFS distance from start {} (shortest path length):", start);
    
    let mut distance_grid = maze.clone();
    for y in 0..maze.height() {
        for x in 0..maze.width() {
            let coord = TutorialCoord::new(x, y);
            if let Some(&distance) = distance_map.get(&coord) {
                if maze.get(coord) == Some(&'.') {
                    let symbol = char::from_digit(distance as u32 % 10, 10).unwrap_or('*');
                    distance_grid.set(coord, symbol);
                }
            }
        }
    }
    
    println!("{}", distance_grid);
    println!("(Numbers show actual distance from start - all neighbors have same distance + 1)");
    
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

// BFS distance map - returns actual shortest distance from start to each reachable cell
fn bfs_distance_map(maze: &TutorialGrid<char>, start: TutorialCoord) -> HashMap<TutorialCoord, usize> {
    let mut queue = VecDeque::new();
    let mut distance_map = HashMap::new();
    
    queue.push_back(start);
    distance_map.insert(start, 0);
    
    while let Some(current) = queue.pop_front() {
        let current_distance = distance_map[&current];
        
        for neighbor in get_neighbors(current, maze.width(), maze.height()) {
            if !distance_map.contains_key(&neighbor) && is_passable(maze, neighbor) {
                distance_map.insert(neighbor, current_distance + 1);
                queue.push_back(neighbor);
            }
        }
    }
    
    distance_map
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
//    ✅ SOLUTION: See step4_dijkstra.rs
// 2. Create a bidirectional BFS that searches from both start and goal
// 3. Add support for diagonal movement with appropriate costs
// 4. Implement JPS (Jump Point Search) for optimized A* on uniform grids

// ═══════════════════════════════════════════════════════════════════════════════
// 🎓 DESIGN QUESTIONS & ANSWERS
// ═══════════════════════════════════════════════════════════════════════════════

// ❓ Question 1: When is BFS better than A* and vice versa?
// ═══════════════════════════════════════════════════════════════════════════════
//
// ✅ USE BFS WHEN:
// 
// 1. **UNWEIGHTED GRAPHS** (all edges cost 1)
//    - BFS guarantees shortest path
//    - Simpler implementation (no priority queue needed)
//    - Memory efficient (HashSet + VecDeque)
//    - Example: Grid with uniform movement cost
//
// 2. **SMALL SEARCH SPACE**
//    - When exploring all nodes is acceptable
//    - Graph fits comfortably in memory
//    - Example: 10x10 grid, few obstacles
//
// 3. **NO CLEAR DIRECTION TO GOAL**
//    - When heuristic would be unreliable
//    - Multiple potential goals at similar distances
//    - Example: "Find ANY exit", not specific one
//
// 4. **SIMPLICITY MATTERS**
//    - Prototyping phase
//    - Educational purposes
//    - When performance is not critical
//
// ✅ USE A* WHEN:
//
// 1. **WEIGHTED GRAPHS** (edges have different costs)
//    - Different terrain types (water=2, mountain=3)
//    - BFS doesn't consider weights at all
//    - Example: Strategy game with varied terrain
//
// 2. **LARGE SEARCH SPACE**
//    - Hundreds or thousands of nodes
//    - Heuristic guides search efficiently
//    - Example: 100x100 grid with sparse obstacles
//
// 3. **SINGLE DESTINATION** with good heuristic
//    - Manhattan distance for 4-connected grids
//    - Euclidean distance for continuous space
//    - Example: GPS navigation from A to B
//
// 4. **PERFORMANCE CRITICAL**
//    - Real-time pathfinding (games, robotics)
//    - A* explores fewer nodes than BFS
//    - Example: RTS game with 100+ units pathfinding
//
// 📊 PERFORMANCE COMPARISON:
//
// Scenario: 20x15 grid, start (0,0) → goal (19,14)
// - BFS:  Explores ~150 nodes (all reachable)
// - A*:   Explores ~50 nodes (guided by heuristic)
// 
// Result: A* is 3x faster for single-goal pathfinding!
//
// 🎯 RULE OF THUMB:
// - Uniform cost + small graph = BFS
// - Weighted graph + large space + single goal = A*
// - Need all distances = Dijkstra's (see step4_dijkstra.rs)

// ❓ Question 2: How do different heuristics affect A* performance and optimality?
// ═══════════════════════════════════════════════════════════════════════════════
//
// 🔑 KEY CONCEPT: **ADMISSIBILITY**
//
// A heuristic h(n) is ADMISSIBLE if it NEVER OVERESTIMATES the true cost.
// Formula: h(n) ≤ true_cost(n, goal)
//
// ✅ ADMISSIBLE HEURISTIC → A* GUARANTEES OPTIMAL PATH
// ❌ NON-ADMISSIBLE HEURISTIC → A* MAY RETURN SUBOPTIMAL PATH
//
// ──────────────────────────────────────────────────────────────────────────────
// 📏 HEURISTIC COMPARISON TABLE
// ──────────────────────────────────────────────────────────────────────────────
//
// | Heuristic  | Formula        | Grid Type      | Admissible? | Performance |
// |------------|----------------|----------------|-------------|-------------|
// | Zero       | h=0            | Any            | ✅ Yes      | Slow        |
// | Manhattan  | |dx|+|dy|      | 4-connected    | ✅ Yes      | Fast        |
// | Euclidean  | √(dx²+dy²)     | Any movement   | ✅ Yes      | Medium      |
// | Chebyshev  | max(|dx|,|dy|) | 8-connected    | ✅ Yes      | Fast        |
// | 2×Manhattan| 2*(|dx|+|dy|)  | 4-connected    | ❌ No       | Very Fast*  |
//
// * Non-admissible! May find suboptimal paths but explores fewer nodes.
//
// ──────────────────────────────────────────────────────────────────────────────
// 🎯 EFFECTS ON A* BEHAVIOR
// ──────────────────────────────────────────────────────────────────────────────
//
// 1. **h(n) = 0** (Zero heuristic = Dijkstra's)
//    - Explores nodes in ALL directions equally
//    - Guaranteed optimal, but SLOWEST
//    - Nodes explored: ~150 (entire reachable area)
//    - Use case: No goal known, need all distances
//
// 2. **h(n) = Manhattan distance** (for 4-connected grid)
//    - Explores nodes primarily TOWARD goal
//    - Guaranteed optimal for 4-connected movement
//    - Nodes explored: ~50 (focused search)
//    - Use case: Grid-based games, AoC problems
//
// 3. **h(n) = Euclidean distance**
//    - More accurate for continuous/diagonal movement
//    - Always admissible (straight line ≤ any path)
//    - Nodes explored: ~45 (slightly more focused)
//    - Use case: Robotics, free movement
//
// 4. **h(n) = Chebyshev distance** (for 8-connected grid)
//    - Best for grids with diagonal movement
//    - Optimal when diagonal cost = cardinal cost
//    - Nodes explored: ~40 (most focused for 8-conn)
//    - Use case: Chess, strategy games
//
// 5. **h(n) = 2 × Manhattan** (NON-ADMISSIBLE example)
//    - Overestimates distance by 2x
//    - Explores FEWER nodes (~30) = FASTER
//    - ❌ MAY FIND SUBOPTIMAL PATH (25% longer)
//    - Use case: Real-time games where "good enough" is OK
//
// ──────────────────────────────────────────────────────────────────────────────
// 🧮 MATHEMATICAL PROPERTIES
// ──────────────────────────────────────────────────────────────────────────────
//
// **Consistency (Monotonicity):**
// h(n) is consistent if: h(n) ≤ cost(n, n') + h(n') for all neighbors n'
//
// Consistent heuristics ensure:
// - Each node is expanded at most once
// - No need to re-open closed nodes
// - More efficient than just admissible
//
// Manhattan, Euclidean, Chebyshev are ALL consistent for their respective grids!
//
// **Trade-off: Accuracy vs Speed**
//
// More accurate heuristic (closer to true cost):
// ✅ Explores fewer nodes
// ✅ Faster pathfinding
// ❌ May be slower to COMPUTE the heuristic itself
//
// Example:
// - Manhattan: 3 operations (2 abs, 1 add)
// - Euclidean: 5 operations (2 sub, 2 square, 1 sqrt)
//
// For most grids, Manhattan is the sweet spot!

// ❓ Question 3: What data structures are most efficient for the open set in A*?
// ═══════════════════════════════════════════════════════════════════════════════
//
// The "open set" stores nodes to explore, prioritized by f_score = g + h
//
// ──────────────────────────────────────────────────────────────────────────────
// 📊 DATA STRUCTURE COMPARISON
// ──────────────────────────────────────────────────────────────────────────────
//
// | Structure         | Insert    | Extract-Min | Decrease-Key | Memory   | Best Use Case           |
// |-------------------|-----------|-------------|--------------|----------|-------------------------|
// | Unsorted Vec      | O(1)      | O(n)        | O(n)         | Minimal  | ❌ Never (too slow)     |
// | Sorted Vec        | O(n)      | O(1)        | O(n)         | Minimal  | ❌ Rarely (slow insert) |
// | **BinaryHeap**    | **O(log n)** | **O(log n)** | ❌ N/A    | **Low**  | ✅ **BEST DEFAULT**     |
// | BTreeSet          | O(log n)  | O(log n)    | O(log n)     | Medium   | ✅ If need decrease-key |
// | Fibonacci Heap    | O(1)†     | O(log n)    | O(1)†        | High     | ⚠️ Theoretical only     |
//
// † Amortized time complexity
//
// ──────────────────────────────────────────────────────────────────────────────
// 🏆 WINNER: BinaryHeap (Rust's std::collections::BinaryHeap)
// ──────────────────────────────────────────────────────────────────────────────
//
// ✅ ADVANTAGES:
// 1. **Simple API**: push() and pop() are all you need
// 2. **Fast operations**: O(log n) for both insert and extract-min
// 3. **Cache-friendly**: Heap stored in contiguous Vec
// 4. **Low memory overhead**: Just the Vec storage
// 5. **Battle-tested**: Standard library implementation
//
// ⚠️ LIMITATION: No decrease-key operation
//
// WORKAROUND (what we do in our implementation):
// - Insert duplicate entries with updated f_score
// - Check if current cost > known cost when popping
// - Skip outdated entries (lazy deletion)
//
// Example from our code:
// ```rust
// while let Some(Reverse(current_node)) = open_set.pop() {
//     // Skip if we've already found a better path to this node
//     if current_node.cost > *g_score.get(&current).unwrap_or(&f64::INFINITY) {
//         continue;  // Lazy deletion of stale entry
//     }
//     // ... process node
// }
// ```
//
// This adds at most O(E) duplicates where E = edges, still efficient!
//
// ──────────────────────────────────────────────────────────────────────────────
// 🔬 ADVANCED ALTERNATIVES
// ──────────────────────────────────────────────────────────────────────────────
//
// **BTreeSet/BTreeMap** (if you need true decrease-key):
// ```rust
// let mut open_set = BTreeSet::new();
// 
// // Insert
// open_set.insert((f_score, coord));
// 
// // Decrease-key (remove old, insert new)
// open_set.remove(&(old_f_score, coord));
// open_set.insert((new_f_score, coord));
// 
// // Extract-min
// let (f, coord) = open_set.pop_first().unwrap();
// ```
//
// Trade-offs:
// ✅ True decrease-key support
// ✅ No duplicate entries
// ❌ Slightly slower than BinaryHeap (tree vs array)
// ❌ Less cache-friendly
// ❌ Requires coordinate to implement Ord
//
// **Custom Indexed Priority Queue** (for maximum performance):
// - Array-based heap with position tracking
// - O(1) decrease-key with index lookup
// - Used in high-performance pathfinding libraries
// - Complex to implement correctly
// - Overkill for most use cases
//
// ──────────────────────────────────────────────────────────────────────────────
// 🎯 RECOMMENDATION
// ──────────────────────────────────────────────────────────────────────────────
//
// Use BinaryHeap with lazy deletion for 95% of use cases!
//
// Only consider alternatives if:
// - Profiling shows open set operations are bottleneck (rare)
// - Extremely large graphs (millions of nodes)
// - Tight real-time constraints (game engines, robotics)

// ❓ Question 4: How would you handle dynamic obstacles that change during pathfinding?
// ═══════════════════════════════════════════════════════════════════════════════
//
// Dynamic obstacles = obstacles that appear/disappear/move WHILE pathfinding
//
// Examples:
// - Moving enemies in a game
// - Traffic congestion changing
// - Doors opening/closing
// - Terrain destruction/construction
//
// ──────────────────────────────────────────────────────────────────────────────
// 🏗️ APPROACH 1: REPLAN FROM SCRATCH (Simple)
// ──────────────────────────────────────────────────────────────────────────────
//
// When environment changes:
// 1. Detect the change
// 2. Abort current pathfinding
// 3. Run A* again from current position
//
// ```rust
// fn handle_obstacle_change(
//     current_pos: Coord,
//     goal: Coord,
//     maze: &TutorialGrid<char>
// ) -> Option<Vec<Coord>> {
//     // Just replan from current position
//     astar_pathfind(maze, current_pos, goal)
// }
// ```
//
// ✅ ADVANTAGES:
// - Simple to implement
// - Always finds optimal path for current state
// - Works with any pathfinding algorithm
//
// ❌ DISADVANTAGES:
// - Wasteful (throws away previous work)
// - Expensive for frequent changes
// - Noticeable lag in real-time applications
//
// ✅ USE WHEN:
// - Changes are RARE (< 1 per second)
// - Grid is SMALL (< 50x50)
// - Simplicity > Performance
//
// ──────────────────────────────────────────────────────────────────────────────
// 🔄 APPROACH 2: D* / D* LITE (Incremental Replanning)
// ──────────────────────────────────────────────────────────────────────────────
//
// D* Lite is an incremental A* that repairs paths efficiently
//
// Key idea: Propagate cost changes only where needed
//
// Pseudocode:
// ```rust
// struct DStarLite {
//     g: HashMap<Coord, f64>,      // Cost from start
//     rhs: HashMap<Coord, f64>,    // One-step lookahead cost
//     open_set: BinaryHeap<Node>,
//     
//     fn update_vertex(&mut self, coord: Coord) {
//         // Recalculate rhs based on neighbors
//         if g[coord] != rhs[coord] {
//             // Inconsistent! Need to fix
//             if coord in open_set { remove it }
//             if g[coord] != rhs[coord] { insert into open_set }
//         }
//     }
//     
//     fn handle_cost_change(&mut self, changed_edges: Vec<(Coord, Coord)>) {
//         for (u, v) in changed_edges {
//             self.update_vertex(u);
//             self.update_vertex(v);
//         }
//         self.compute_shortest_path();  // Repair incrementally
//     }
// }
// ```
//
// ✅ ADVANTAGES:
// - MUCH faster than replanning (10-100x)
// - Only updates affected nodes
// - Proven optimal (same path as A*)
//
// ❌ DISADVANTAGES:
// - Complex implementation (~500 lines)
// - More memory overhead
// - Only handles cost changes, not topology
//
// ✅ USE WHEN:
// - Changes are FREQUENT (> 1 per second)
// - Grid is LARGE (> 100x100)
// - Real-time performance critical
//
// 📚 REFERENCE: "D* Lite" by Koenig & Likhachev (2002)
//
// ──────────────────────────────────────────────────────────────────────────────
// ⚡ APPROACH 3: LOCAL REPAIR (Path Smoothing)
// ──────────────────────────────────────────────────────────────────────────────
//
// When obstacle blocks current path:
// 1. Find which segment is blocked
// 2. Replan only that LOCAL segment
// 3. Splice repaired segment into existing path
//
// ```rust
// fn repair_path(
//     path: &mut Vec<Coord>,
//     blocked_index: usize,
//     maze: &TutorialGrid<char>
// ) {
//     // Find nearest valid points before and after blockage
//     let start_repair = path[blocked_index - 1];
//     let end_repair = path[blocked_index + 5];  // Look ahead
//     
//     // Replan just this segment
//     if let Some(segment) = astar_pathfind(maze, start_repair, end_repair) {
//         // Splice in the new segment
//         path.splice(blocked_index..blocked_index+5, segment);
//     } else {
//         // Can't repair locally, full replan needed
//         *path = astar_pathfind(maze, path[0], *path.last().unwrap()).unwrap();
//     }
// }
// ```
//
// ✅ ADVANTAGES:
// - Faster than full replan
// - Simpler than D* Lite
// - Works well for local obstacles
//
// ❌ DISADVANTAGES:
// - May produce suboptimal paths
// - Can't handle large-scale changes
// - Requires path validation
//
// ✅ USE WHEN:
// - Obstacles are LOCALIZED (affect small area)
// - "Good enough" paths acceptable
// - Want simple + fast solution
//
// ──────────────────────────────────────────────────────────────────────────────
// 🎮 APPROACH 4: HIERARCHICAL PATHFINDING (For Games)
// ──────────────────────────────────────────────────────────────────────────────
//
// Divide map into REGIONS, path at two levels:
//
// 1. **High-level**: Path between regions (stable)
// 2. **Low-level**: Path within current region (dynamic)
//
// ```rust
// struct HierarchicalMap {
//     regions: Vec<Region>,          // Large areas (16x16 chunks)
//     region_graph: Graph,           // Connections between regions
//     
//     fn path_hierarchical(&self, start: Coord, goal: Coord) -> Path {
//         // 1. Find which regions contain start/goal
//         let start_region = self.find_region(start);
//         let goal_region = self.find_region(goal);
//         
//         // 2. High-level path (rare changes)
//         let region_path = astar_pathfind(&self.region_graph, start_region, goal_region);
//         
//         // 3. Low-level path in current region (frequent changes)
//         let local_path = astar_pathfind(&self.regions[current], current_pos, region_exit);
//         
//         // 4. Combine
//         combine_paths(local_path, region_path)
//     }
// }
// ```
//
// ✅ ADVANTAGES:
// - High-level path stable (rarely replans)
// - Low-level path cheap (small area)
// - Scales to huge maps
//
// ❌ DISADVANTAGES:
// - Complex setup and maintenance
// - Needs region precomputation
// - Path may not be globally optimal
//
// ✅ USE WHEN:
// - LARGE WORLDS (> 500x500)
// - Many simultaneous pathfinders
// - AAA game development
//
// 📚 REFERENCE: Hierarchical Pathfinding A* (HPA*)
//
// ──────────────────────────────────────────────────────────────────────────────
// 🎯 DECISION MATRIX
// ──────────────────────────────────────────────────────────────────────────────
//
// | Scenario                        | Recommended Approach        |
// |---------------------------------|-----------------------------|
// | Small grid, rare changes        | Replan from scratch         |
// | Large grid, frequent changes    | D* Lite                     |
// | Local obstacles, good enough OK | Local repair                |
// | Huge world, many agents         | Hierarchical pathfinding    |
// | Unknown obstacles discovered    | Replan from scratch         |
// | Predictable obstacle patterns   | Precompute alternative paths|
//
// ──────────────────────────────────────────────────────────────────────────────
// 💡 PRACTICAL TIPS
// ──────────────────────────────────────────────────────────────────────────────
//
// 1. **Validate before moving**
//    ```rust
//    let next_pos = path[current_index + 1];
//    if !is_passable(maze, next_pos) {
//        replan_from(current_pos);
//    }
//    ```
//
// 2. **Use dirty flags**
//    - Mark regions that changed
//    - Only replan if dirty
//
// 3. **Amortize expensive operations**
//    - Replan over multiple frames
//    - Use cached partial results
//
// 4. **Add safety buffer**
//    - Path around obstacles with margin
//    - Reduces replan frequency
//
// 5. **Combine approaches**
//    - Local repair first (fast)
//    - Full replan if repair fails (correct)

// ═══════════════════════════════════════════════════════════════════════════════
// 📚 FURTHER READING
// ═══════════════════════════════════════════════════════════════════════════════
//
// Papers:
// - Hart, Nilsson & Raphael (1968): "A Formal Basis for the Heuristic Determination of Minimum Cost Paths" (Original A*)
// - Koenig & Likhachev (2002): "D* Lite" (Incremental replanning)
// - Botea, Müller & Schaeffer (2004): "Near Optimal Hierarchical Path-Finding" (HPA*)
//
// Books:
// - "Artificial Intelligence: A Modern Approach" by Russell & Norvig
// - "Game AI Pro" series (various pathfinding chapters)
//
// Online:
// - Red Blob Games: https://www.redblobgames.com/pathfinding/a-star/
// - Amit's A* Pages: http://theory.stanford.edu/~amitp/GameProgramming/
//
// Rust Implementations:
// - `pathfinding` crate: Production-ready A*, Dijkstra's, etc.
// - `petgraph` crate: Graph algorithms library