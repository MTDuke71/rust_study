// Step 4 Extension: Dijkstra's Algorithm
// Exercise 1 from step4_pathfinding.rs - Implement Dijkstra's algorithm (A* with h=0)
//
// Learning Objectives:
// - Understand Dijkstra's algorithm as A* without heuristic (h=0)
// - Learn when Dijkstra's is appropriate vs A*
// - Practice weighted shortest path algorithms
// - Compare performance: BFS vs Dijkstra's vs A*
//
// Dijkstra's Algorithm:
// - Special case of A* where heuristic h(n) = 0 for all nodes
// - Guarantees shortest path in weighted graphs (non-negative weights)
// - Explores more nodes than A* but still optimal
// - Useful when: no good heuristic available, multiple goals, or all-pairs shortest path

use mission6_tut::tutorial_helpers::{print_section, print_step_complete, TutorialGrid, TutorialCoord};
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

fn main() {
    println!("=== Mission 6 Tutorial - Step 4 Extension: Dijkstra's Algorithm ===");
    println!("Exercise 1: Implementing Dijkstra's (A* with h=0)\n");

    // Section 1: Understanding Dijkstra's Algorithm
    print_section("1. Understanding Dijkstra's Algorithm");
    
    println!("Dijkstra's algorithm finds shortest paths from a single source to all other nodes.");
    println!("\nKey Properties:");
    println!("• Special case of A* where heuristic h(n) = 0");
    println!("• Guarantees optimal path for non-negative edge weights");
    println!("• Explores nodes in order of increasing distance from start");
    println!("• Computes shortest path to ALL reachable nodes (not just goal)");
    
    println!("\nComparison:");
    println!("┌──────────────┬─────────────┬──────────────┬───────────────┐");
    println!("│ Algorithm    │ Graph Type  │ Heuristic    │ Exploration   │");
    println!("├──────────────┼─────────────┼──────────────┼───────────────┤");
    println!("│ BFS          │ Unweighted  │ None         │ Level-by-level│");
    println!("│ Dijkstra's   │ Weighted    │ h(n) = 0     │ By distance   │");
    println!("│ A*           │ Weighted    │ h(n) ≠ 0     │ Toward goal   │");
    println!("└──────────────┴─────────────┴──────────────┴───────────────┘");
    
    // Section 2: Dijkstra's on Uniform Cost Grid
    print_section("2. Dijkstra's on Uniform Cost Grid");
    
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
    
    println!("Test maze (S=start, G=goal, #=obstacle):");
    println!("{}", maze);
    
    let dijkstra_result = dijkstra_pathfind(&maze, start, goal);
    match dijkstra_result {
        Some((path, cost)) => {
            println!("\nDijkstra's found path with cost {:.0} ({} steps):", cost, path.len() - 1);
            visualize_path(&maze, &path);
            println!("Path: {} steps", path.len() - 1);
        }
        None => println!("Dijkstra's found no path from {} to {}", start, goal),
    }
    
    // Section 3: Dijkstra's All-Distances Feature
    print_section("3. Dijkstra's All-Distances Feature");
    
    println!("Unlike A*, Dijkstra's computes distances to ALL reachable nodes:");
    
    let all_distances = dijkstra_all_distances(&maze, start);
    
    println!("\nDistance from {} to all reachable cells:", start);
    let mut distance_grid = maze.clone();
    for y in 0..maze.height() {
        for x in 0..maze.width() {
            let coord = TutorialCoord::new(x, y);
            if let Some(&distance) = all_distances.get(&coord) {
                if maze.get(coord) == Some(&'.') {
                    let symbol = char::from_digit(distance as u32 % 10, 10).unwrap_or('*');
                    distance_grid.set(coord, symbol);
                }
            }
        }
    }
    println!("{}", distance_grid);
    
    // Show statistics
    let reachable_count = all_distances.len();
    let max_distance = all_distances.values().max().unwrap_or(&0);
    println!("\nStatistics:");
    println!("• Reachable cells: {}", reachable_count);
    println!("• Maximum distance: {}", max_distance);
    println!("• Distance to goal: {}", all_distances.get(&goal).unwrap_or(&0));
    
    // Section 4: Weighted Terrain with Dijkstra's
    print_section("4. Weighted Terrain with Dijkstra's");
    
    let mut terrain_costs = HashMap::new();
    terrain_costs.insert('.', 1.0);  // Normal terrain
    terrain_costs.insert('~', 2.0);  // Water (slower)
    terrain_costs.insert('^', 3.0);  // Mountain (much slower)
    terrain_costs.insert('#', f64::INFINITY);  // Impassable
    
    let mut weighted_maze = TutorialGrid::new(8, 5, '.');
    
    // Create terrain variety
    weighted_maze.set(TutorialCoord::new(2, 1), '~');
    weighted_maze.set(TutorialCoord::new(2, 2), '~');
    weighted_maze.set(TutorialCoord::new(3, 1), '^');
    weighted_maze.set(TutorialCoord::new(3, 2), '^');
    weighted_maze.set(TutorialCoord::new(4, 2), '#');
    weighted_maze.set(TutorialCoord::new(5, 1), '~');
    weighted_maze.set(TutorialCoord::new(5, 2), '~');
    
    let w_start = TutorialCoord::new(0, 0);
    let w_goal = TutorialCoord::new(7, 4);
    weighted_maze.set(w_start, 'S');
    weighted_maze.set(w_goal, 'G');
    
    println!("Weighted terrain maze (.=1, ~=2, ^=3, #=∞):");
    println!("{}", weighted_maze);
    
    let weighted_result = dijkstra_weighted(&weighted_maze, w_start, w_goal, &terrain_costs);
    match weighted_result {
        Some((path, total_cost)) => {
            println!("\nDijkstra's found path with total cost: {:.1}", total_cost);
            visualize_path(&weighted_maze, &path);
            
            // Calculate actual terrain costs along path
            let mut path_breakdown = Vec::new();
            for coord in &path {
                let terrain = weighted_maze.get(*coord).unwrap_or(&'.');
                let cost = terrain_costs.get(terrain).unwrap_or(&1.0);
                path_breakdown.push((*coord, *terrain, *cost));
            }
            
            println!("\nPath breakdown:");
            for (i, (coord, terrain, cost)) in path_breakdown.iter().enumerate() {
                if i > 0 {
                    print!(" → ");
                }
                print!("{}({}={:.0})", coord, terrain, cost);
                if (i + 1) % 3 == 0 && i < path_breakdown.len() - 1 {
                    println!();
                }
            }
            println!();
        }
        None => println!("No path found"),
    }
    
    // Section 5: When to Use Dijkstra's vs A*
    print_section("5. When to Use Dijkstra's vs A*");
    
    println!("✅ Use Dijkstra's when:");
    println!("   • You need shortest paths to MULTIPLE/ALL destinations");
    println!("   • No good heuristic function is available");
    println!("   • Graph is small and exploration cost is acceptable");
    println!("   • You need guaranteed optimal paths with complex weights");
    println!();
    println!("✅ Use A* when:");
    println!("   • Single source, SINGLE destination pathfinding");
    println!("   • Good admissible heuristic exists (Manhattan, Euclidean, etc.)");
    println!("   • Performance matters (large graphs)");
    println!("   • Clear direction toward goal");
    println!();
    println!("✅ Use BFS when:");
    println!("   • UNWEIGHTED graphs (all edges cost 1)");
    println!("   • Simplest implementation needed");
    println!("   • Memory efficient for small graphs");
    
    // Section 6: Performance Comparison
    print_section("6. Performance Comparison");
    
    use std::time::Instant;
    
    let large_maze = create_large_maze(20, 15);
    let large_start = TutorialCoord::new(0, 0);
    let large_goal = TutorialCoord::new(19, 14);
    
    // BFS timing
    let start_time = Instant::now();
    let bfs_result = bfs_pathfind(&large_maze, large_start, large_goal);
    let bfs_time = start_time.elapsed();
    
    // Dijkstra's timing
    let start_time = Instant::now();
    let dijkstra_result = dijkstra_pathfind(&large_maze, large_start, large_goal);
    let dijkstra_time = start_time.elapsed();
    
    // Dijkstra's all-distances timing
    let start_time = Instant::now();
    let all_dist = dijkstra_all_distances(&large_maze, large_start);
    let dijkstra_all_time = start_time.elapsed();
    
    println!("Performance on 20x15 maze:");
    if let Some(bfs_path) = bfs_result {
        println!("• BFS:                    {} steps in {:?}", bfs_path.len() - 1, bfs_time);
    }
    if let Some((dijkstra_path, _)) = dijkstra_result {
        println!("• Dijkstra's (to goal):   {} steps in {:?}", dijkstra_path.len() - 1, dijkstra_time);
    }
    println!("• Dijkstra's (all nodes): {} nodes in {:?}", all_dist.len(), dijkstra_all_time);
    
    println!("\nObservation:");
    println!("• BFS is fastest for unweighted uniform-cost grids");
    println!("• Dijkstra's to single goal is slower (explores all directions)");
    println!("• Dijkstra's all-distances computes full distance map efficiently");
    
    // Section 7: Practical Applications
    print_section("7. Practical Applications");
    
    println!("Real-world use cases for Dijkstra's:");
    println!();
    println!("1. **Network Routing** (e.g., OSPF protocol)");
    println!("   - Find shortest path through router network");
    println!("   - Weighted by link costs/latency");
    println!();
    println!("2. **GPS Navigation with Traffic**");
    println!("   - Road segments have varying costs (congestion)");
    println!("   - Need optimal route considering current conditions");
    println!();
    println!("3. **Game AI - Influence Maps**");
    println!("   - Compute distance from strategic locations to all map points");
    println!("   - Territory control analysis");
    println!();
    println!("4. **Resource Distribution**");
    println!("   - Find optimal delivery routes from warehouse to all stores");
    println!("   - Minimize total transportation cost");
    println!();
    println!("5. **Circuit Design**");
    println!("   - Find shortest wire paths in PCB layout");
    println!("   - Minimize signal delay");
    
    print_step_complete("Step 4 Extension: Dijkstra's Algorithm");
    
    println!("\n🎯 Exercise Complete!");
    println!("\n📚 Additional Exercises:");
    println!("   2. Implement bidirectional BFS (search from both ends)");
    println!("   3. Add diagonal movement with √2 cost");
    println!("   4. Implement JPS (Jump Point Search) optimization");
    
    println!("\n💡 Key Takeaways:");
    println!("   ✓ Dijkstra's is A* with h=0 (no heuristic)");
    println!("   ✓ Guarantees optimal paths for weighted graphs");
    println!("   ✓ Computes distances to ALL reachable nodes");
    println!("   ✓ Slower than A* for single-goal pathfinding");
    println!("   ✓ Ideal when you need distances to multiple destinations");
}

// Dijkstra's algorithm - finds shortest path from start to goal
fn dijkstra_pathfind(maze: &TutorialGrid<char>, start: TutorialCoord, goal: TutorialCoord) -> Option<(Vec<TutorialCoord>, f64)> {
    let mut open_set = BinaryHeap::new();
    let mut g_score = HashMap::new();
    let mut parent = HashMap::new();
    
    g_score.insert(start, 0.0);
    // Dijkstra's: h=0, so f_score = g_score
    open_set.push(Reverse(DijkstraNode { coord: start, cost: 0.0 }));
    
    while let Some(Reverse(current_node)) = open_set.pop() {
        let current = current_node.coord;
        
        // Early exit when goal found
        if current == goal {
            return Some((reconstruct_path(&parent, start, goal), g_score[&current]));
        }
        
        // Skip if we've already found a better path to this node
        if current_node.cost > *g_score.get(&current).unwrap_or(&f64::INFINITY) {
            continue;
        }
        
        for neighbor in get_neighbors(current, maze.width(), maze.height()) {
            if !is_passable(maze, neighbor) { continue; }
            
            let tentative_g_score = g_score[&current] + 1.0; // Uniform cost (1.0 per step)
            
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f64::INFINITY) {
                parent.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                // No heuristic: priority = cost only
                open_set.push(Reverse(DijkstraNode { coord: neighbor, cost: tentative_g_score }));
            }
        }
    }
    
    None
}

// Dijkstra's algorithm - computes distances to ALL reachable nodes
fn dijkstra_all_distances(maze: &TutorialGrid<char>, start: TutorialCoord) -> HashMap<TutorialCoord, usize> {
    let mut open_set = BinaryHeap::new();
    let mut distances = HashMap::new();
    
    distances.insert(start, 0);
    open_set.push(Reverse(DijkstraNode { coord: start, cost: 0.0 }));
    
    while let Some(Reverse(current_node)) = open_set.pop() {
        let current = current_node.coord;
        let current_dist = distances[&current];
        
        // Skip if we've already found a better path
        if (current_dist as f64) < current_node.cost {
            continue;
        }
        
        for neighbor in get_neighbors(current, maze.width(), maze.height()) {
            if !is_passable(maze, neighbor) { continue; }
            
            let new_distance = current_dist + 1;
            
            if new_distance < *distances.get(&neighbor).unwrap_or(&usize::MAX) {
                distances.insert(neighbor, new_distance);
                open_set.push(Reverse(DijkstraNode { 
                    coord: neighbor, 
                    cost: new_distance as f64 
                }));
            }
        }
    }
    
    distances
}

// Dijkstra's with weighted terrain
fn dijkstra_weighted(
    maze: &TutorialGrid<char>, 
    start: TutorialCoord, 
    goal: TutorialCoord,
    costs: &HashMap<char, f64>
) -> Option<(Vec<TutorialCoord>, f64)> {
    let mut open_set = BinaryHeap::new();
    let mut g_score = HashMap::new();
    let mut parent = HashMap::new();
    
    g_score.insert(start, 0.0);
    open_set.push(Reverse(DijkstraNode { coord: start, cost: 0.0 }));
    
    while let Some(Reverse(current_node)) = open_set.pop() {
        let current = current_node.coord;
        
        if current == goal {
            return Some((reconstruct_path(&parent, start, goal), g_score[&current]));
        }
        
        if current_node.cost > *g_score.get(&current).unwrap_or(&f64::INFINITY) {
            continue;
        }
        
        for neighbor in get_neighbors(current, maze.width(), maze.height()) {
            let terrain = maze.get(neighbor).unwrap_or(&'#');
            let move_cost = *costs.get(terrain).unwrap_or(&f64::INFINITY);
            
            if move_cost == f64::INFINITY { continue; }
            
            let tentative_g_score = g_score[&current] + move_cost;
            
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f64::INFINITY) {
                parent.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                open_set.push(Reverse(DijkstraNode { coord: neighbor, cost: tentative_g_score }));
            }
        }
    }
    
    None
}

// Helper structs and functions

#[derive(Debug, PartialEq)]
struct DijkstraNode {
    coord: TutorialCoord,
    cost: f64,
}

impl Eq for DijkstraNode {}

impl PartialOrd for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Ord for DijkstraNode {
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
        let cell = maze.get(*coord).unwrap_or(&'#');
        if *cell != 'S' && *cell != 'G' {
            path_maze.set(*coord, '*');
        }
    }
    println!("{}", path_maze);
}

// Simple BFS for comparison
fn bfs_pathfind(maze: &TutorialGrid<char>, start: TutorialCoord, goal: TutorialCoord) -> Option<Vec<TutorialCoord>> {
    use std::collections::{VecDeque, HashSet};
    
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
