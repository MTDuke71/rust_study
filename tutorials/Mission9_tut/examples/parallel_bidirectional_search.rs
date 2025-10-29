//! # TODO 3: Parallel Bidirectional Search Implementation
//!
//! This implements true parallel bidirectional search with:
//! - Separate threads for forward and backward search
//! - Shared meeting point detection
//! - Thread-safe synchronization
//! - Early termination when paths meet

use std::collections::{BinaryHeap, HashSet, HashMap};
use std::cmp::Ordering;
use std::time::Instant;
use std::sync::{Arc, Mutex, mpsc, atomic::{AtomicBool, AtomicU32, Ordering as AtomicOrdering}};
use std::thread;

/// Thread-safe node structure for parallel processing
#[derive(Debug, Clone, Eq, PartialEq)]
struct ParallelNode {
    id: u32,
    cost: u32,
    heuristic: u32,
    parent: Option<u32>,
}

impl ParallelNode {
    fn f_score(&self) -> u32 {
        self.cost + self.heuristic
    }
}

impl Ord for ParallelNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score().cmp(&self.f_score())
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for ParallelNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Simple grid (copy from main file for standalone example)
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
}

fn manhattan_distance(from: (usize, usize), to: (usize, usize)) -> u32 {
    let dx = from.0.abs_diff(to.0);
    let dy = from.1.abs_diff(to.1);
    (dx + dy) as u32
}

/// Search result from individual thread
#[derive(Debug, Clone)]
struct ThreadSearchResult {
    nodes_explored: usize,
    nodes_generated: usize,
    came_from: HashMap<u32, u32>,
    g_scores: HashMap<u32, u32>,
    final_cost: Option<u32>,
    meeting_point: Option<u32>,
}

/// Shared state between threads
#[derive(Debug)]
struct SharedSearchState {
    // Meeting point detection
    best_meeting_cost: AtomicU32,
    meeting_point: Mutex<Option<u32>>,
    
    // Termination control
    should_terminate: AtomicBool,
    
    // Cross-thread communication
    forward_closed: Mutex<HashSet<u32>>,
    backward_closed: Mutex<HashSet<u32>>,
    forward_g_scores: Mutex<HashMap<u32, u32>>,
    backward_g_scores: Mutex<HashMap<u32, u32>>,
}

impl SharedSearchState {
    fn new() -> Self {
        Self {
            best_meeting_cost: AtomicU32::new(u32::MAX),
            meeting_point: Mutex::new(None),
            should_terminate: AtomicBool::new(false),
            forward_closed: Mutex::new(HashSet::new()),
            backward_closed: Mutex::new(HashSet::new()),
            forward_g_scores: Mutex::new(HashMap::new()),
            backward_g_scores: Mutex::new(HashMap::new()),
        }
    }
    
    fn check_meeting_point(&self, node_id: u32, node_cost: u32, is_forward: bool) -> bool {
        // Check if the other search has reached this node
        let (other_closed, other_g_scores) = if is_forward {
            (&self.backward_closed, &self.backward_g_scores)
        } else {
            (&self.forward_closed, &self.forward_g_scores)
        };
        
        if let (Ok(closed), Ok(g_scores)) = (other_closed.try_lock(), other_g_scores.try_lock()) {
            if closed.contains(&node_id) {
                if let Some(&other_cost) = g_scores.get(&node_id) {
                    let total_cost = node_cost + other_cost;
                    let current_best = self.best_meeting_cost.load(AtomicOrdering::Acquire);
                    
                    if total_cost < current_best {
                        self.best_meeting_cost.store(total_cost, AtomicOrdering::Release);
                        *self.meeting_point.lock().unwrap() = Some(node_id);
                        println!("   🎯 Meeting point found at node {} with cost {}", node_id, total_cost);
                        return true;
                    }
                }
            }
        }
        false
    }
    
    fn should_terminate(&self, current_f_score: u32) -> bool {
        if self.should_terminate.load(AtomicOrdering::Acquire) {
            return true;
        }
        
        let best_cost = self.best_meeting_cost.load(AtomicOrdering::Acquire);
        if current_f_score >= best_cost {
            self.should_terminate.store(true, AtomicOrdering::Release);
            return true;
        }
        
        false
    }
    
    fn update_closed_set(&self, node_id: u32, node_cost: u32, is_forward: bool) {
        let (closed, g_scores) = if is_forward {
            (&self.forward_closed, &self.forward_g_scores)
        } else {
            (&self.backward_closed, &self.backward_g_scores)
        };
        
        if let (Ok(mut closed_lock), Ok(mut g_lock)) = (closed.try_lock(), g_scores.try_lock()) {
            closed_lock.insert(node_id);
            g_lock.insert(node_id, node_cost);
        }
    }
}

/// True parallel bidirectional A* search
pub struct ParallelBidirectionalAStar;

impl ParallelBidirectionalAStar {
    pub fn find_path(grid: &Grid, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        let start_time = Instant::now();
        
        let start_id = grid.pos_to_id(start);
        let goal_id = grid.pos_to_id(goal);
        
        // Shared state between threads
        let shared_state = Arc::new(SharedSearchState::new());
        
        // Communication channels
        let (forward_tx, forward_rx) = mpsc::channel();
        let (backward_tx, backward_rx) = mpsc::channel();
        
        // Clone data for threads
        let grid_forward = grid.clone();
        let grid_backward = grid.clone();
        let shared_forward = Arc::clone(&shared_state);
        let shared_backward = Arc::clone(&shared_state);
        
        println!("🚀 Starting parallel bidirectional search...");
        println!("   Forward thread: {} → {}", start_id, goal_id);
        println!("   Backward thread: {} → {}", goal_id, start_id);
        
        // Forward search thread
        let forward_handle = thread::spawn(move || {
            let result = Self::search_direction(
                &grid_forward,
                start,
                goal,
                start_id,
                goal_id,
                true,  // is_forward
                &shared_forward,
            );
            forward_tx.send(result).unwrap();
        });
        
        // Backward search thread  
        let backward_handle = thread::spawn(move || {
            let result = Self::search_direction(
                &grid_backward,
                goal,
                start,
                goal_id,
                start_id,
                false, // is_forward
                &shared_backward,
            );
            backward_tx.send(result).unwrap();
        });
        
        // Wait for both threads to complete
        let forward_result = forward_rx.recv().unwrap();
        let backward_result = backward_rx.recv().unwrap();
        
        forward_handle.join().unwrap();
        backward_handle.join().unwrap();
        
        let total_time = start_time.elapsed();
        
        println!("   ⏱️  Parallel search completed in {:?}", total_time);
        println!("   📊 Forward: {} nodes explored, {} generated", 
                forward_result.nodes_explored, forward_result.nodes_generated);
        println!("   📊 Backward: {} nodes explored, {} generated", 
                backward_result.nodes_explored, backward_result.nodes_generated);
        println!("   📊 Total: {} nodes explored, {} generated",
                forward_result.nodes_explored + backward_result.nodes_explored,
                forward_result.nodes_generated + backward_result.nodes_generated);
        
        // Reconstruct path if meeting point found
        let meeting_point_option = {
            let meeting_guard = shared_state.meeting_point.lock().unwrap();
            *meeting_guard
        };
        
        if let Some(meeting_id) = meeting_point_option {
            let path = Self::reconstruct_bidirectional_path(
                &forward_result.came_from,
                &backward_result.came_from,
                start_id,
                goal_id,
                meeting_id,
                grid,
            );
            
            println!("   ✅ Path found: {} steps", path.len());
            Some(path)
        } else {
            println!("   ❌ No path found");
            None
        }
    }
    
    fn search_direction(
        grid: &Grid,
        search_start: (usize, usize),
        search_goal: (usize, usize),
        start_id: u32,
        goal_id: u32,
        is_forward: bool,
        shared_state: &SharedSearchState,
    ) -> ThreadSearchResult {
        let thread_name = if is_forward { "Forward" } else { "Backward" };
        
        let mut open_set = BinaryHeap::new();
        let mut local_closed = HashSet::new();
        let mut g_scores = HashMap::new();
        let mut came_from = HashMap::new();
        let mut nodes_explored = 0;
        let mut nodes_generated = 0;
        
        // Initialize search
        let initial_node = ParallelNode {
            id: start_id,
            cost: 0,
            heuristic: manhattan_distance(search_start, search_goal),
            parent: None,
        };
        
        open_set.push(initial_node);
        g_scores.insert(start_id, 0);
        nodes_generated += 1;
        
        println!("   🧵 {} thread initialized", thread_name);
        
        while let Some(current) = open_set.pop() {
            // Check for early termination
            if shared_state.should_terminate(current.f_score()) {
                println!("   🛑 {} thread terminating early (f-score: {})", thread_name, current.f_score());
                break;
            }
            
            nodes_explored += 1;
            
            // Check if we've reached the goal
            if current.id == goal_id {
                println!("   🎯 {} thread reached goal!", thread_name);
                shared_state.should_terminate.store(true, AtomicOrdering::Release);
                break;
            }
            
            // Add to local closed set
            local_closed.insert(current.id);
            
            // Update shared state for meeting point detection
            shared_state.update_closed_set(current.id, current.cost, is_forward);
            
            // Check for meeting point with other search
            if shared_state.check_meeting_point(current.id, current.cost, is_forward) {
                println!("   🤝 {} thread found meeting point!", thread_name);
            }
            
            // Expand neighbors
            let current_pos = grid.id_to_pos(current.id);
            for neighbor_pos in grid.neighbors(current_pos) {
                let neighbor_id = grid.pos_to_id(neighbor_pos);
                
                if local_closed.contains(&neighbor_id) {
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
                
                let neighbor_h = manhattan_distance(neighbor_pos, search_goal);
                let neighbor_node = ParallelNode {
                    id: neighbor_id,
                    cost: tentative_g,
                    heuristic: neighbor_h,
                    parent: Some(current.id),
                };
                
                open_set.push(neighbor_node);
                nodes_generated += 1;
            }
            
            // Periodic progress update
            if nodes_explored % 50 == 0 {
                println!("   📈 {} thread: {} nodes explored", thread_name, nodes_explored);
            }
        }
        
        println!("   🏁 {} thread finished: {} explored, {} generated", 
                thread_name, nodes_explored, nodes_generated);
        
        ThreadSearchResult {
            nodes_explored,
            nodes_generated,
            came_from,
            g_scores,
            final_cost: None,
            meeting_point: None,
        }
    }
    
    fn reconstruct_bidirectional_path(
        forward_came_from: &HashMap<u32, u32>,
        backward_came_from: &HashMap<u32, u32>,
        start_id: u32,
        goal_id: u32,
        meeting_id: u32,
        grid: &Grid,
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
}

/// Test function
fn create_test_grid() -> Grid {
    let mut grid = Grid::new(50, 50);
    
    // Add some obstacles
    for i in 10..40 {
        grid.add_obstacle(i, 25);
    }
    for i in 15..35 {
        grid.add_obstacle(25, i);
    }
    
    grid
}

pub fn main() {
    println!("🧵 TODO 3: Parallel Bidirectional Search Implementation");
    println!("======================================================\n");

    let grid = create_test_grid();
    let start = (5, 5);
    let goal = (45, 45);

    println!("📋 Test Setup:");
    println!("   Grid size: {}x{}", grid.width, grid.height);
    println!("   Start: {:?}", start);
    println!("   Goal: {:?}", goal);
    println!("   Obstacles: {} cells blocked\n", grid.obstacles.len());

    println!("🔄 Running Parallel Bidirectional A*:");
    println!("=====================================");
    
    if let Some(path) = ParallelBidirectionalAStar::find_path(&grid, start, goal) {
        println!("\n✅ Success! Found path with {} steps", path.len());
        
        // Show first few and last few steps
        println!("🗺️  Path preview:");
        for (i, pos) in path.iter().take(5).enumerate() {
            println!("   Step {}: {:?}", i + 1, pos);
        }
        if path.len() > 10 {
            println!("   ... ({} more steps) ...", path.len() - 10);
            for (i, pos) in path.iter().skip(path.len() - 5).enumerate() {
                println!("   Step {}: {:?}", path.len() - 5 + i + 1, pos);
            }
        }
    } else {
        println!("\n❌ No path found between start and goal");
    }

    println!("\n🎓 Key Features Implemented:");
    println!("============================");
    println!("✅ Separate threads for forward and backward search");
    println!("✅ Shared meeting point detection with AtomicU32");
    println!("✅ Thread-safe synchronization using Mutex and Atomic types");
    println!("✅ Early termination when optimal solution found");
    println!("✅ Cross-thread communication for closed sets");
    println!("✅ Real-time progress updates from both threads");

    println!("\n💡 Performance Benefits:");
    println!("========================");
    println!("• Utilizes multiple CPU cores simultaneously");
    println!("• Reduces total search space (meets in middle)");
    println!("• Early termination when paths converge");
    println!("• Thread-safe shared state management");
    println!("• Optimal path reconstruction from meeting point");
}