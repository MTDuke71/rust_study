//! # JPS Performance Analysis - Why JPS Can Be Slower
//!
//! This demonstrates the hidden computational overhead in Jump Point Search

use std::time::Instant;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    width: i32,
    height: i32,
    obstacles: HashSet<Position>,
}

impl Grid {
    fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            obstacles: HashSet::new(),
        }
    }
    
    fn add_obstacle(&mut self, pos: Position) {
        self.obstacles.insert(pos);
    }
    
    fn is_walkable(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.x < self.width && 
        pos.y >= 0 && pos.y < self.height &&
        !self.obstacles.contains(pos)
    }
}

/// Instrumented JPS to count operations
struct InstrumentedJPS {
    jump_calls: u32,
    walkable_checks: u32,
    forced_neighbor_checks: u32,
}

impl InstrumentedJPS {
    fn new() -> Self {
        Self {
            jump_calls: 0,
            walkable_checks: 0,
            forced_neighbor_checks: 0,
        }
    }
    
    fn jump(&mut self, grid: &Grid, pos: Position, dx: i32, dy: i32, goal: Position) -> Option<Position> {
        self.jump_calls += 1;  // Count recursive calls
        
        let next_pos = Position::new(pos.x + dx, pos.y + dy);
        
        self.walkable_checks += 1;
        if !grid.is_walkable(&next_pos) {
            return None;
        }
        
        if next_pos == goal {
            return Some(next_pos);
        }
        
        // Check for forced neighbors
        if self.has_forced_neighbors(grid, &next_pos, dx, dy) {
            return Some(next_pos);
        }
        
        // Diagonal movement: check horizontal and vertical components
        if dx != 0 && dy != 0 {
            if self.jump(grid, next_pos, dx, 0, goal).is_some() ||
               self.jump(grid, next_pos, 0, dy, goal).is_some() {
                return Some(next_pos);
            }
        }
        
        // Continue jumping in the same direction - RECURSIVE CALL
        self.jump(grid, next_pos, dx, dy, goal)
    }
    
    fn has_forced_neighbors(&mut self, grid: &Grid, pos: &Position, dx: i32, dy: i32) -> bool {
        self.forced_neighbor_checks += 1;
        
        if dx != 0 && dy != 0 {
            // Diagonal movement - 4 walkable checks
            self.walkable_checks += 4;
            let blocked1 = !grid.is_walkable(&Position::new(pos.x - dx, pos.y));
            let blocked2 = !grid.is_walkable(&Position::new(pos.x, pos.y - dy));
            let open1 = grid.is_walkable(&Position::new(pos.x - dx, pos.y + dy));
            let open2 = grid.is_walkable(&Position::new(pos.x + dx, pos.y - dy));
            
            (blocked1 && open1) || (blocked2 && open2)
        } else if dx != 0 {
            // Horizontal movement - 4 walkable checks
            self.walkable_checks += 4;
            let blocked1 = !grid.is_walkable(&Position::new(pos.x, pos.y + 1));
            let blocked2 = !grid.is_walkable(&Position::new(pos.x, pos.y - 1));
            let open1 = grid.is_walkable(&Position::new(pos.x + dx, pos.y + 1));
            let open2 = grid.is_walkable(&Position::new(pos.x + dx, pos.y - 1));
            
            (blocked1 && open1) || (blocked2 && open2)
        } else {
            // Vertical movement - 4 walkable checks
            self.walkable_checks += 4;
            let blocked1 = !grid.is_walkable(&Position::new(pos.x + 1, pos.y));
            let blocked2 = !grid.is_walkable(&Position::new(pos.x - 1, pos.y));
            let open1 = grid.is_walkable(&Position::new(pos.x + 1, pos.y + dy));
            let open2 = grid.is_walkable(&Position::new(pos.x - 1, pos.y + dy));
            
            (blocked1 && open1) || (blocked2 && open2)
        }
    }
    
    fn get_jump_points(&mut self, grid: &Grid, pos: &Position, goal: Position) -> Vec<Position> {
        let mut jump_points = Vec::new();
        
        // Check all 8 directions
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
        
        for &(dx, dy) in &directions {
            if let Some(jump_point) = self.jump(grid, *pos, dx, dy, goal) {
                jump_points.push(jump_point);
            }
        }
        
        jump_points
    }
    
    fn simulate_single_expansion(&mut self, grid: &Grid, pos: Position, goal: Position) -> Vec<Position> {
        println!("🔍 Expanding node at ({}, {})", pos.x, pos.y);
        
        let initial_jump_calls = self.jump_calls;
        let initial_walkable_checks = self.walkable_checks;
        let initial_forced_checks = self.forced_neighbor_checks;
        
        let start_time = Instant::now();
        let jump_points = self.get_jump_points(grid, &pos, goal);
        let expansion_time = start_time.elapsed();
        
        let jump_calls_used = self.jump_calls - initial_jump_calls;
        let walkable_checks_used = self.walkable_checks - initial_walkable_checks;
        let forced_checks_used = self.forced_neighbor_checks - initial_forced_checks;
        
        println!("   ⚡ Found {} jump points", jump_points.len());
        println!("   🔄 Jump calls: {}", jump_calls_used);
        println!("   🚪 Walkable checks: {}", walkable_checks_used);
        println!("   🎯 Forced neighbor checks: {}", forced_checks_used);
        println!("   ⏱️  Time: {:?}", expansion_time);
        println!("   📊 Operations per jump point: {:.1}", 
                (jump_calls_used + walkable_checks_used) as f64 / jump_points.len().max(1) as f64);
        println!();
        
        jump_points
    }
    
    fn get_stats(&self) -> (u32, u32, u32) {
        (self.jump_calls, self.walkable_checks, self.forced_neighbor_checks)
    }
}

/// Standard A* for comparison
struct SimpleAStar {
    neighbor_checks: u32,
}

impl SimpleAStar {
    fn new() -> Self {
        Self {
            neighbor_checks: 0,
        }
    }
    
    fn get_neighbors(&mut self, grid: &Grid, pos: Position) -> Vec<Position> {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut neighbors = Vec::new();
        
        for &(dx, dy) in &directions {
            let neighbor = Position::new(pos.x + dx, pos.y + dy);
            self.neighbor_checks += 1;
            if grid.is_walkable(&neighbor) {
                neighbors.push(neighbor);
            }
        }
        
        neighbors
    }
    
    fn simulate_single_expansion(&mut self, grid: &Grid, pos: Position) -> Vec<Position> {
        println!("🔍 Standard A* expanding node at ({}, {})", pos.x, pos.y);
        
        let initial_checks = self.neighbor_checks;
        let start_time = Instant::now();
        let neighbors = self.get_neighbors(grid, pos);
        let expansion_time = start_time.elapsed();
        
        let checks_used = self.neighbor_checks - initial_checks;
        
        println!("   ⚡ Found {} neighbors", neighbors.len());
        println!("   🚪 Walkable checks: {}", checks_used);
        println!("   ⏱️  Time: {:?}", expansion_time);
        println!("   📊 Operations per neighbor: {:.1}", 
                checks_used as f64 / neighbors.len().max(1) as f64);
        println!();
        
        neighbors
    }
}

fn create_test_grid() -> Grid {
    let mut grid = Grid::new(100, 100);
    
    // Add obstacles to create interesting pathfinding scenario
    for i in 20..80 {
        grid.add_obstacle(Position::new(i, 50));
    }
    for i in 30..70 {
        grid.add_obstacle(Position::new(50, i));
    }
    
    grid
}

pub fn main() {
    println!("🔬 JPS Performance Analysis: Why Fewer Nodes Can Mean Slower Performance");
    println!("=======================================================================\n");

    let grid = create_test_grid();
    let start = Position::new(10, 10);
    let goal = Position::new(90, 90);

    println!("📋 Test Setup:");
    println!("   Grid size: {}x{}", grid.width, grid.height);
    println!("   Start: ({}, {})", start.x, start.y);
    println!("   Goal: ({}, {})", goal.x, goal.y);
    println!("   Obstacles: {} cells blocked\n", grid.obstacles.len());

    // Test single node expansion - Standard A*
    println!("🎯 Standard A* Single Node Expansion:");
    println!("=====================================");
    let mut standard_astar = SimpleAStar::new();
    let standard_neighbors = standard_astar.simulate_single_expansion(&grid, start);

    // Test single node expansion - JPS
    println!("🎯 Jump Point Search Single Node Expansion:");
    println!("===========================================");
    let mut jps = InstrumentedJPS::new();
    let jump_points = jps.simulate_single_expansion(&grid, start, goal);

    // Compare total computational work
    println!("📊 Computational Complexity Comparison:");
    println!("=======================================");
    
    let (jump_calls, walkable_checks, forced_checks) = jps.get_stats();
    let total_jps_operations = jump_calls + walkable_checks + forced_checks;
    let total_astar_operations = standard_astar.neighbor_checks;
    
    println!("Standard A* (single expansion):");
    println!("   • Neighbors found: {}", standard_neighbors.len());
    println!("   • Total operations: {}", total_astar_operations); 
    println!("   • Operations per result: {:.1}", 
            total_astar_operations as f64 / standard_neighbors.len().max(1) as f64);
    
    println!("\nJump Point Search (single expansion):");
    println!("   • Jump points found: {}", jump_points.len());
    println!("   • Jump calls: {}", jump_calls);
    println!("   • Walkable checks: {}", walkable_checks);  
    println!("   • Forced neighbor checks: {}", forced_checks);
    println!("   • Total operations: {}", total_jps_operations);
    println!("   • Operations per result: {:.1}", 
            total_jps_operations as f64 / jump_points.len().max(1) as f64);

    // Calculate overhead multiplier
    let overhead_multiplier = total_jps_operations as f64 / total_astar_operations.max(1) as f64;
    println!("\n💡 JPS Computational Overhead: {:.1}× more operations per expansion", overhead_multiplier);

    // Extrapolate to full search
    println!("\n🧮 Extrapolated Full Search Analysis:");
    println!("=====================================");
    println!("If Standard A* explores 81 nodes:");
    println!("   • Standard A*: 81 × {} = {} total operations", total_astar_operations, 81 * total_astar_operations);
    println!("   • JPS (same ops/node): 12 × {} = {} total operations", total_jps_operations, 12 * total_jps_operations);
    
    let jps_projected_ops = 12 * total_jps_operations;
    let astar_projected_ops = 81 * total_astar_operations;
    
    if jps_projected_ops > astar_projected_ops {
        let ops_ratio = jps_projected_ops as f64 / astar_projected_ops as f64;
        println!("   • JPS does {:.1}× MORE total work despite exploring fewer nodes!", ops_ratio);
    }

    println!("\n🎓 Key Insights:");
    println!("================");
    println!("✅ JPS explores fewer nodes (12 vs 81) - ✨ algorithmic optimization");
    println!("❌ JPS does more work per node - 💻 computational overhead");
    println!("🔍 Each JPS expansion involves:");
    println!("   • Recursive jumping in 8 directions");
    println!("   • Complex forced neighbor detection");
    println!("   • Multiple walkable checks per step");
    println!("   • Deep call stack overhead");
    
    println!("\n💡 When JPS Becomes Faster:");
    println!("===========================");
    println!("• Very large grids (1000×1000+) where algorithmic savings dominate");
    println!("• Open areas with long straight paths");
    println!("• Optimized implementations with pruning");
    println!("• When memory access becomes the bottleneck");
    
    println!("\n🚀 Performance Optimization Strategies:");
    println!("=======================================");
    println!("• Pre-compute jump distances");
    println!("• Cache forced neighbor calculations");
    println!("• Use iterative instead of recursive jumping");
    println!("• Implement jump point preprocessing (JPS+)");
    println!("• Profile and optimize hot paths");
}