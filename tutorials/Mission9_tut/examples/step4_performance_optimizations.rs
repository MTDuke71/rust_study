//! # Step 4: Performance Optimizations
//!
//! This tutorial covers advanced performance optimization techniques for pathfinding algorithms:
//! - Bidirectional search (search from both start and goal)
//! - Memory optimization techniques
//! - Early termination strategies
//! - Cache-friendly data structures
//!
//! ## Learning Objectives
//! - Implement bidirectional Dijkstra and A* algorithms
//! - Understand memory optimization patterns
//! - Apply early termination and pruning strategies
//! - Measure and compare performance improvements
//!
//! ## Key Concepts
//! - **Bidirectional Search**: Search from both start and goal simultaneously
//! - **Meeting Point**: Where forward and backward searches meet
//! - **Memory Pooling**: Reuse allocations to reduce overhead
//! - **Early Termination**: Stop search when optimal solution found
//! - **Cache Locality**: Arrange data for better CPU cache usage

use std::collections::{BinaryHeap, HashSet, HashMap, VecDeque};
use std::cmp::Ordering;
use std::time::Instant;

/// Enhanced node structure with memory optimization
#[derive(Debug, Clone, Eq, PartialEq)]
struct OptimizedNode {
    id: u32,        // Use u32 instead of usize for memory efficiency
    cost: u32,      // Use integer costs for better cache performance
    heuristic: u32, // Heuristic value (for A*)
    parent: Option<u32>,  // Parent for path reconstruction
}

impl OptimizedNode {
    fn f_score(&self) -> u32 {
        self.cost + self.heuristic
    }
}

impl Ord for OptimizedNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap: lower f-score has higher priority
        other.f_score().cmp(&self.f_score())
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for OptimizedNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Memory pool for node allocation
struct NodePool {
    nodes: Vec<OptimizedNode>,
    free_indices: Vec<usize>,
}

impl NodePool {
    fn new(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
            free_indices: Vec::with_capacity(capacity),
        }
    }

    fn allocate(&mut self, id: u32, cost: u32, heuristic: u32, parent: Option<u32>) -> usize {
        if let Some(index) = self.free_indices.pop() {
            self.nodes[index] = OptimizedNode { id, cost, heuristic, parent };
            index
        } else {
            let index = self.nodes.len();
            self.nodes.push(OptimizedNode { id, cost, heuristic, parent });
            index
        }
    }

    fn deallocate(&mut self, index: usize) {
        self.free_indices.push(index);
    }

    fn get(&self, index: usize) -> &OptimizedNode {
        &self.nodes[index]
    }
}

/// Simple grid graph for testing
#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    obstacles: HashSet<(usize, usize)>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            obstacles: HashSet::new(),
        }
    }

    fn add_obstacle(&mut self, x: usize, y: usize) {
        self.obstacles.insert((x, y));
    }

    fn neighbors(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = pos;
        let mut neighbors = Vec::with_capacity(4);

        // 4-way movement
        if x > 0 && !self.obstacles.contains(&(x - 1, y)) {
            neighbors.push((x - 1, y));
        }
        if x + 1 < self.width && !self.obstacles.contains(&(x + 1, y)) {
            neighbors.push((x + 1, y));
        }
        if y > 0 && !self.obstacles.contains(&(x, y - 1)) {
            neighbors.push((x, y - 1));
        }
        if y + 1 < self.height && !self.obstacles.contains(&(x, y + 1)) {
            neighbors.push((x, y + 1));
        }

        neighbors
    }

    fn pos_to_id(&self, pos: (usize, usize)) -> u32 {
        (pos.1 * self.width + pos.0) as u32
    }

    fn id_to_pos(&self, id: u32) -> (usize, usize) {
        let id = id as usize;
        (id % self.width, id / self.width)
    }

    fn is_valid_pos(&self, pos: (usize, usize)) -> bool {
        pos.0 < self.width && pos.1 < self.height && !self.obstacles.contains(&pos)
    }
}

/// Manhattan distance heuristic
fn manhattan_distance(from: (usize, usize), to: (usize, usize)) -> u32 {
    let dx = if from.0 > to.0 { from.0 - to.0 } else { to.0 - from.0 };
    let dy = if from.1 > to.1 { from.1 - to.1 } else { to.1 - from.1 };
    (dx + dy) as u32
}

/// Performance statistics
#[derive(Debug, Clone)]
struct SearchStats {
    nodes_explored: usize,
    nodes_generated: usize,
    peak_memory: usize,
    search_time_ms: u128,
    path_length: usize,
    path_cost: u32,
}

impl SearchStats {
    fn new() -> Self {
        Self {
            nodes_explored: 0,
            nodes_generated: 0,
            peak_memory: 0,
            search_time_ms: 0,
            path_length: 0,
            path_cost: 0,
        }
    }
}

/// Standard A* implementation for comparison
struct StandardAStar {
    stats: SearchStats,
}

impl StandardAStar {
    fn new() -> Self {
        Self {
            stats: SearchStats::new(),
        }
    }

    fn find_path(&mut self, grid: &Grid, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        let start_time = Instant::now();
        self.stats = SearchStats::new();

        let mut open_set = BinaryHeap::new();
        let mut closed_set = HashSet::new();
        let mut g_scores = HashMap::new();
        let mut came_from = HashMap::new();

        let start_id = grid.pos_to_id(start);
        let goal_id = grid.pos_to_id(goal);

        // Initialize start node
        let start_h = manhattan_distance(start, goal);
        let start_node = OptimizedNode {
            id: start_id,
            cost: 0,
            heuristic: start_h,
            parent: None,
        };

        open_set.push(start_node.clone());
        g_scores.insert(start_id, 0);
        self.stats.nodes_generated += 1;

        while let Some(current) = open_set.pop() {
            self.stats.nodes_explored += 1;
            self.stats.peak_memory = self.stats.peak_memory.max(open_set.len() + closed_set.len());

            if current.id == goal_id {
                // Reconstruct path
                let path = self.reconstruct_path(&came_from, start_id, goal_id, grid);
                self.stats.search_time_ms = start_time.elapsed().as_millis();
                self.stats.path_length = path.len();
                self.stats.path_cost = current.cost;
                return Some(path);
            }

            closed_set.insert(current.id);

            let current_pos = grid.id_to_pos(current.id);
            for neighbor_pos in grid.neighbors(current_pos) {
                let neighbor_id = grid.pos_to_id(neighbor_pos);

                if closed_set.contains(&neighbor_id) {
                    continue;
                }

                let tentative_g = current.cost + 1; // Unit cost for grid movement

                if let Some(&existing_g) = g_scores.get(&neighbor_id) {
                    if tentative_g >= existing_g {
                        continue;
                    }
                }

                // Update or insert neighbor
                g_scores.insert(neighbor_id, tentative_g);
                came_from.insert(neighbor_id, current.id);

                let neighbor_h = manhattan_distance(neighbor_pos, goal);
                let neighbor_node = OptimizedNode {
                    id: neighbor_id,
                    cost: tentative_g,
                    heuristic: neighbor_h,
                    parent: Some(current.id),
                };

                open_set.push(neighbor_node);
                self.stats.nodes_generated += 1;
            }
        }

        self.stats.search_time_ms = start_time.elapsed().as_millis();
        None
    }

    fn reconstruct_path(&self, came_from: &HashMap<u32, u32>, start_id: u32, goal_id: u32, grid: &Grid) -> Vec<(usize, usize)> {
        let mut path = Vec::new();
        let mut current = goal_id;

        path.push(grid.id_to_pos(current));

        while let Some(&parent) = came_from.get(&current) {
            if parent == start_id {
                path.push(grid.id_to_pos(parent));
                break;
            }
            current = parent;
            path.push(grid.id_to_pos(current));
        }

        path.reverse();
        path
    }

    fn get_stats(&self) -> &SearchStats {
        &self.stats
    }
}

/// Bidirectional A* implementation
struct BidirectionalAStar {
    forward_stats: SearchStats,
    backward_stats: SearchStats,
}

impl BidirectionalAStar {
    fn new() -> Self {
        Self {
            forward_stats: SearchStats::new(),
            backward_stats: SearchStats::new(),
        }
    }

    fn find_path(&mut self, grid: &Grid, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        let start_time = Instant::now();
        self.forward_stats = SearchStats::new();
        self.backward_stats = SearchStats::new();

        // Forward search (start to goal)
        let mut forward_open = BinaryHeap::new();
        let mut forward_closed = HashSet::new();
        let mut forward_g_scores = HashMap::new();
        let mut forward_came_from = HashMap::new();

        // Backward search (goal to start)
        let mut backward_open = BinaryHeap::new();
        let mut backward_closed = HashSet::new();
        let mut backward_g_scores = HashMap::new();
        let mut backward_came_from = HashMap::new();

        let start_id = grid.pos_to_id(start);
        let goal_id = grid.pos_to_id(goal);

        // Initialize forward search
        let forward_start = OptimizedNode {
            id: start_id,
            cost: 0,
            heuristic: manhattan_distance(start, goal),
            parent: None,
        };
        forward_open.push(forward_start);
        forward_g_scores.insert(start_id, 0);

        // Initialize backward search
        let backward_start = OptimizedNode {
            id: goal_id,
            cost: 0,
            heuristic: manhattan_distance(goal, start),
            parent: None,
        };
        backward_open.push(backward_start);
        backward_g_scores.insert(goal_id, 0);

        let mut best_path_cost = u32::MAX;
        let mut meeting_point: Option<u32> = None;

        while !forward_open.is_empty() && !backward_open.is_empty() {
            // Expand forward search
            if let Some(forward_current) = forward_open.pop() {
                self.forward_stats.nodes_explored += 1;

                // Check if we've met the backward search
                if backward_closed.contains(&forward_current.id) {
                    let total_cost = forward_current.cost + 
                        backward_g_scores.get(&forward_current.id).unwrap_or(&u32::MAX);
                    
                    if total_cost < best_path_cost {
                        best_path_cost = total_cost;
                        meeting_point = Some(forward_current.id);
                    }
                }

                // Early termination check
                if forward_current.f_score() >= best_path_cost {
                    break;
                }

                forward_closed.insert(forward_current.id);

                // Expand neighbors
                let current_pos = grid.id_to_pos(forward_current.id);
                for neighbor_pos in grid.neighbors(current_pos) {
                    let neighbor_id = grid.pos_to_id(neighbor_pos);

                    if forward_closed.contains(&neighbor_id) {
                        continue;
                    }

                    let tentative_g = forward_current.cost + 1;

                    if let Some(&existing_g) = forward_g_scores.get(&neighbor_id) {
                        if tentative_g >= existing_g {
                            continue;
                        }
                    }

                    forward_g_scores.insert(neighbor_id, tentative_g);
                    forward_came_from.insert(neighbor_id, forward_current.id);

                    let neighbor_h = manhattan_distance(neighbor_pos, goal);
                    let neighbor_node = OptimizedNode {
                        id: neighbor_id,
                        cost: tentative_g,
                        heuristic: neighbor_h,
                        parent: Some(forward_current.id),
                    };

                    forward_open.push(neighbor_node);
                    self.forward_stats.nodes_generated += 1;
                }
            }

            // Expand backward search
            if let Some(backward_current) = backward_open.pop() {
                self.backward_stats.nodes_explored += 1;

                // Check if we've met the forward search
                if forward_closed.contains(&backward_current.id) {
                    let total_cost = backward_current.cost + 
                        forward_g_scores.get(&backward_current.id).unwrap_or(&u32::MAX);
                    
                    if total_cost < best_path_cost {
                        best_path_cost = total_cost;
                        meeting_point = Some(backward_current.id);
                    }
                }

                // Early termination check
                if backward_current.f_score() >= best_path_cost {
                    break;
                }

                backward_closed.insert(backward_current.id);

                // Expand neighbors
                let current_pos = grid.id_to_pos(backward_current.id);
                for neighbor_pos in grid.neighbors(current_pos) {
                    let neighbor_id = grid.pos_to_id(neighbor_pos);

                    if backward_closed.contains(&neighbor_id) {
                        continue;
                    }

                    let tentative_g = backward_current.cost + 1;

                    if let Some(&existing_g) = backward_g_scores.get(&neighbor_id) {
                        if tentative_g >= existing_g {
                            continue;
                        }
                    }

                    backward_g_scores.insert(neighbor_id, tentative_g);
                    backward_came_from.insert(neighbor_id, backward_current.id);

                    let neighbor_h = manhattan_distance(neighbor_pos, start);
                    let neighbor_node = OptimizedNode {
                        id: neighbor_id,
                        cost: tentative_g,
                        heuristic: neighbor_h,
                        parent: Some(backward_current.id),
                    };

                    backward_open.push(neighbor_node);
                    self.backward_stats.nodes_generated += 1;
                }
            }
        }

        let total_time = start_time.elapsed().as_millis();
        self.forward_stats.search_time_ms = total_time;
        self.backward_stats.search_time_ms = total_time;

        // Reconstruct path if meeting point found
        if let Some(meeting_id) = meeting_point {
            let path = self.reconstruct_bidirectional_path(
                &forward_came_from, &backward_came_from,
                start_id, goal_id, meeting_id, grid
            );
            
            self.forward_stats.path_length = path.len();
            self.forward_stats.path_cost = best_path_cost;
            Some(path)
        } else {
            None
        }
    }

    fn reconstruct_bidirectional_path(
        &self,
        forward_came_from: &HashMap<u32, u32>,
        backward_came_from: &HashMap<u32, u32>,
        start_id: u32,
        goal_id: u32,
        meeting_id: u32,
        grid: &Grid
    ) -> Vec<(usize, usize)> {
        let mut path = Vec::new();

        // Forward path (start to meeting point)
        let mut forward_path = Vec::new();
        let mut current = meeting_id;
        forward_path.push(grid.id_to_pos(current));

        while let Some(&parent) = forward_came_from.get(&current) {
            if parent == start_id {
                forward_path.push(grid.id_to_pos(parent));
                break;
            }
            current = parent;
            forward_path.push(grid.id_to_pos(current));
        }
        forward_path.reverse();

        // Backward path (meeting point to goal)
        let mut backward_path = Vec::new();
        current = meeting_id;

        while let Some(&parent) = backward_came_from.get(&current) {
            current = parent;
            backward_path.push(grid.id_to_pos(current));
            if current == goal_id {
                break;
            }
        }

        // Combine paths (avoid duplicating meeting point)
        path.extend(forward_path);
        path.extend(backward_path);

        path
    }

    fn get_total_stats(&self) -> SearchStats {
        SearchStats {
            nodes_explored: self.forward_stats.nodes_explored + self.backward_stats.nodes_explored,
            nodes_generated: self.forward_stats.nodes_generated + self.backward_stats.nodes_generated,
            peak_memory: self.forward_stats.peak_memory.max(self.backward_stats.peak_memory),
            search_time_ms: self.forward_stats.search_time_ms,
            path_length: self.forward_stats.path_length,
            path_cost: self.forward_stats.path_cost,
        }
    }
}

/// Memory-optimized pathfinder using node pools
struct MemoryOptimizedAStar {
    node_pool: NodePool,
    stats: SearchStats,
}

impl MemoryOptimizedAStar {
    fn new(pool_capacity: usize) -> Self {
        Self {
            node_pool: NodePool::new(pool_capacity),
            stats: SearchStats::new(),
        }
    }

    fn find_path(&mut self, grid: &Grid, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        let start_time = Instant::now();
        self.stats = SearchStats::new();

        // Use pre-allocated vectors for better memory performance
        let mut open_indices = BinaryHeap::new();
        let mut closed_set = HashSet::with_capacity(grid.width * grid.height / 4);
        let mut g_scores = HashMap::with_capacity(grid.width * grid.height / 4);
        let mut came_from = HashMap::with_capacity(grid.width * grid.height / 4);

        let start_id = grid.pos_to_id(start);
        let goal_id = grid.pos_to_id(goal);

        // Allocate start node from pool
        let start_h = manhattan_distance(start, goal);
        let start_index = self.node_pool.allocate(start_id, 0, start_h, None);
        
        open_indices.push((self.node_pool.get(start_index).f_score(), start_index));
        g_scores.insert(start_id, 0);
        self.stats.nodes_generated += 1;

        while let Some((_, current_index)) = open_indices.pop() {
            let current = self.node_pool.get(current_index).clone();
            self.stats.nodes_explored += 1;

            if current.id == goal_id {
                // Reconstruct path and deallocate nodes
                let path = self.reconstruct_path(&came_from, start_id, goal_id, grid);
                self.stats.search_time_ms = start_time.elapsed().as_millis();
                self.stats.path_length = path.len();
                self.stats.path_cost = current.cost;
                
                // Clean up node pool
                self.node_pool.deallocate(current_index);
                return Some(path);
            }

            closed_set.insert(current.id);
            self.node_pool.deallocate(current_index);

            let current_pos = grid.id_to_pos(current.id);
            for neighbor_pos in grid.neighbors(current_pos) {
                let neighbor_id = grid.pos_to_id(neighbor_pos);

                if closed_set.contains(&neighbor_id) {
                    continue;
                }

                let tentative_g = current.cost + 1;

                if let Some(&existing_g) = g_scores.get(&neighbor_id) {
                    if tentative_g >= existing_g {
                        continue;
                    }
                }

                g_scores.insert(neighbor_id, tentative_g);
                came_from.insert(neighbor_id, current.id);

                let neighbor_h = manhattan_distance(neighbor_pos, goal);
                let neighbor_index = self.node_pool.allocate(
                    neighbor_id, tentative_g, neighbor_h, Some(current.id)
                );

                let neighbor_f = tentative_g + neighbor_h;
                open_indices.push((neighbor_f, neighbor_index));
                self.stats.nodes_generated += 1;
            }

            self.stats.peak_memory = self.stats.peak_memory.max(open_indices.len() + closed_set.len());
        }

        self.stats.search_time_ms = start_time.elapsed().as_millis();
        None
    }

    fn reconstruct_path(&self, came_from: &HashMap<u32, u32>, start_id: u32, goal_id: u32, grid: &Grid) -> Vec<(usize, usize)> {
        let mut path = Vec::new();
        let mut current = goal_id;

        path.push(grid.id_to_pos(current));

        while let Some(&parent) = came_from.get(&current) {
            if parent == start_id {
                path.push(grid.id_to_pos(parent));
                break;
            }
            current = parent;
            path.push(grid.id_to_pos(current));
        }

        path.reverse();
        path
    }

    fn get_stats(&self) -> &SearchStats {
        &self.stats
    }
}

/// Demonstration functions

fn create_test_grid() -> Grid {
    let mut grid = Grid::new(50, 50);
    
    // Add some obstacles to make the path more interesting
    for i in 10..40 {
        grid.add_obstacle(i, 25);
    }
    for i in 15..35 {
        grid.add_obstacle(25, i);
    }
    
    grid
}

fn benchmark_algorithms() {
    println!("🚀 Step 4: Performance Optimization Benchmarks");
    println!("==============================================\n");

    let grid = create_test_grid();
    let start = (5, 5);
    let goal = (45, 45);

    println!("📋 Test Setup:");
    println!("   Grid size: {}x{}", grid.width, grid.height);
    println!("   Start: {:?}", start);
    println!("   Goal: {:?}", goal);
    println!("   Obstacles: {} cells blocked\n", grid.obstacles.len());

    // Test Standard A*
    println!("🔍 Testing Standard A*:");
    let mut standard_astar = StandardAStar::new();
    let start_time = Instant::now();
    let standard_path = standard_astar.find_path(&grid, start, goal);
    let standard_time = start_time.elapsed();
    let standard_stats = standard_astar.get_stats();

    if let Some(path) = standard_path {
        println!("   ✅ Path found: {} steps", path.len());
        println!("   📊 Nodes explored: {}", standard_stats.nodes_explored);
        println!("   🏭 Nodes generated: {}", standard_stats.nodes_generated);
        println!("   💾 Peak memory usage: {} nodes", standard_stats.peak_memory);
        println!("   ⏱️  Search time: {:?}\n", standard_time);
    } else {
        println!("   ❌ No path found\n");
    }

    // Test Bidirectional A*
    println!("🔄 Testing Bidirectional A*:");
    let mut bidirectional_astar = BidirectionalAStar::new();
    let start_time = Instant::now();
    let bidirectional_path = bidirectional_astar.find_path(&grid, start, goal);
    let bidirectional_time = start_time.elapsed();
    let bidirectional_stats = bidirectional_astar.get_total_stats();

    if let Some(path) = bidirectional_path {
        println!("   ✅ Path found: {} steps", path.len());
        println!("   📊 Nodes explored: {}", bidirectional_stats.nodes_explored);
        println!("   🏭 Nodes generated: {}", bidirectional_stats.nodes_generated);
        println!("   💾 Peak memory usage: {} nodes", bidirectional_stats.peak_memory);
        println!("   ⏱️  Search time: {:?}", bidirectional_time);
        
        // Performance comparison
        if standard_stats.nodes_explored > 0 {
            let exploration_reduction = 100.0 * (1.0 - bidirectional_stats.nodes_explored as f64 / standard_stats.nodes_explored as f64);
            let time_reduction = 100.0 * (1.0 - bidirectional_time.as_nanos() as f64 / standard_time.as_nanos() as f64);
            println!("   🎯 Exploration reduction: {:.1}%", exploration_reduction);
            println!("   🎯 Time reduction: {:.1}%", time_reduction);
        }
        println!();
    } else {
        println!("   ❌ No path found\n");
    }

    // Test Memory-Optimized A*
    println!("💾 Testing Memory-Optimized A*:");
    let mut memory_optimized = MemoryOptimizedAStar::new(10000);
    let start_time = Instant::now();
    let optimized_path = memory_optimized.find_path(&grid, start, goal);
    let optimized_time = start_time.elapsed();
    let optimized_stats = memory_optimized.get_stats();

    if let Some(path) = optimized_path {
        println!("   ✅ Path found: {} steps", path.len());
        println!("   📊 Nodes explored: {}", optimized_stats.nodes_explored);
        println!("   🏭 Nodes generated: {}", optimized_stats.nodes_generated);
        println!("   💾 Peak memory usage: {} nodes", optimized_stats.peak_memory);
        println!("   ⏱️  Search time: {:?}\n", optimized_time);
    } else {
        println!("   ❌ No path found\n");
    }

    // Performance summary
    println!("📈 Performance Summary:");
    println!("======================");
    println!("Algorithm              | Nodes Explored | Time (μs)    | Memory Efficiency");
    println!("----------------------|---------------|--------------|------------------");
    println!("Standard A*           | {:13} | {:10} | Baseline", 
             standard_stats.nodes_explored, 
             standard_time.as_micros());
    println!("Bidirectional A*      | {:13} | {:10} | {:.1}% reduction",
             bidirectional_stats.nodes_explored,
             bidirectional_time.as_micros(),
             100.0 * (1.0 - bidirectional_stats.nodes_explored as f64 / standard_stats.nodes_explored as f64));
    println!("Memory-Optimized A*   | {:13} | {:10} | Pool-based allocation",
             optimized_stats.nodes_explored,
             optimized_time.as_micros());
}

fn demonstrate_early_termination() {
    println!("\n⏹️ Early Termination Strategy Demo:");
    println!("===================================");

    let grid = create_test_grid();
    let start = (5, 5);
    let goal = (15, 15); // Closer goal for demonstration

    println!("📋 Testing early termination with closer goal:");
    println!("   Start: {:?}", start);
    println!("   Goal: {:?}", goal);

    let mut astar = StandardAStar::new();
    let path = astar.find_path(&grid, start, goal);
    let stats = astar.get_stats();

    if let Some(path) = path {
        println!("   ✅ Path found: {} steps", path.len());
        println!("   📊 Nodes explored: {}", stats.nodes_explored);
        println!("   💡 Early termination reduces unnecessary exploration");
        println!("   💡 A* stops as soon as optimal goal is reached");
    }
}

fn demonstrate_memory_patterns() {
    println!("\n💾 Memory Usage Pattern Analysis:");
    println!("=================================");

    println!("📊 Node Structure Sizes:");
    println!("   OptimizedNode: {} bytes", std::mem::size_of::<OptimizedNode>());
    println!("   Standard usize-based: {} bytes", std::mem::size_of::<(usize, usize, usize, Option<usize>)>());
    
    let reduction = 100.0 * (1.0 - std::mem::size_of::<OptimizedNode>() as f64 / 
                            std::mem::size_of::<(usize, usize, usize, Option<usize>)>() as f64);
    println!("   💡 Memory reduction: {:.1}% per node", reduction);

    println!("\n🔄 Memory Pool Benefits:");
    println!("   • Reduces allocation overhead");
    println!("   • Improves cache locality");
    println!("   • Enables memory reuse");
    println!("   • Predictable memory usage");
}

/// TODO Exercises for hands-on learning
fn todo_exercises() {
    println!("\n📝 TODO Exercises:");
    println!("==================");

    // TODO 1: Implement Jump Point Search (JPS) optimization
    println!("🎯 TODO 1: Jump Point Search Implementation");
    println!("   Implement JPS for even faster grid pathfinding:");
    println!("   - Skip intermediate nodes in straight lines");
    println!("   - Only expand at 'jump points' (direction changes)");
    println!("   - Can provide 10x speedup on open grids");
    println!();

    // TODO 2: Add dynamic weight adjustment
    println!("🎯 TODO 2: Dynamic Heuristic Weighting");
    println!("   Implement weighted A* with dynamic adjustment:");
    println!("   - Use f(n) = g(n) + w*h(n) where w > 1");
    println!("   - Trade optimality for speed");
    println!("   - Adjust weight based on search progress");
    println!();

    // TODO 3: Parallel bidirectional search
    println!("🎯 TODO 3: Parallel Bidirectional Search");
    println!("   Implement parallel forward/backward search:");
    println!("   - Run searches on separate threads");
    println!("   - Use shared meeting point detection");
    println!("   - Handle thread synchronization safely");
    println!();

    // TODO 4: Hierarchical pathfinding
    println!("🎯 TODO 4: Hierarchical Pathfinding");
    println!("   Implement hierarchical pathfinding:");
    println!("   - Pre-compute cluster-level paths");
    println!("   - Use coarse-to-fine planning");
    println!("   - Enable pathfinding on massive graphs");
    println!();

    // TODO 5: Adaptive algorithm selection
    println!("🎯 TODO 5: Adaptive Algorithm Selection");
    println!("   Implement intelligent algorithm selection:");
    println!("   - Analyze problem characteristics");
    println!("   - Choose optimal algorithm automatically");
    println!("   - Consider graph size, density, goal distance");
}

fn main() {
    println!("📚 Mission 9 Tutorial - Step 4: Performance Optimizations\n");

    benchmark_algorithms();
    demonstrate_early_termination();
    demonstrate_memory_patterns();
    todo_exercises();

    println!("\n✅ Step 4 Complete!");
    println!("📖 Next: Step 5 - Advanced heuristics and multi-objective optimization");
    println!("🔗 Related: Check Mission 9 main implementation for production-ready versions");
}