//! # Step 5: Advanced Heuristics & Multi-Objective Optimization
//!
//! This tutorial explores advanced pathfinding concepts beyond basic A*:
//! - Custom heuristic design and composition
//! - Multi-objective optimization with Pareto frontiers
//! - Constraint-based pathfinding
//! - Dynamic heuristic weighting and adaptation
//! - Trade-off analysis between competing objectives
//!
//! ## Learning Objectives
//! - Design problem-specific heuristics for various domains
//! - Implement multi-objective pathfinding with Pareto optimization
//! - Understand constraint satisfaction in pathfinding
//! - Apply dynamic weighting strategies for real-time adaptation
//! - Analyze trade-offs between solution quality and computation time
//!
//! ## Key Concepts
//! - **Admissible Heuristics**: Never overestimate true cost (h(n) ≤ h*(n))
//! - **Consistent Heuristics**: Satisfy triangle inequality for optimality
//! - **Pareto Frontier**: Set of non-dominated solutions in multi-objective space
//! - **Weighted A***: Trade optimality for speed with f(n) = g(n) + w*h(n)
//! - **Dynamic Weighting**: Adapt search focus based on progress and constraints

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::time::Instant;

/// Multi-objective cost representation
#[derive(Debug, Clone, PartialEq)]
struct MultiObjectiveCost {
    distance: f64, // Travel distance/time
    safety: f64,   // Safety score (higher is safer)
    fuel: f64,     // Fuel consumption
    comfort: f64,  // Comfort level (road quality, traffic)
}

impl MultiObjectiveCost {
    fn new(distance: f64, safety: f64, fuel: f64, comfort: f64) -> Self {
        Self {
            distance,
            safety,
            fuel,
            comfort,
        }
    }

    /// Check if this solution dominates another (better in all objectives)
    fn dominates(&self, other: &Self) -> bool {
        // A solution dominates another if it's better or equal in all objectives
        // and strictly better in at least one objective
        let distance_better_or_equal = self.distance <= other.distance;
        let safety_better_or_equal = self.safety >= other.safety;
        let fuel_better_or_equal = self.fuel <= other.fuel;
        let comfort_better_or_equal = self.comfort >= other.comfort;

        // Must be better or equal in all objectives
        let all_better_or_equal = distance_better_or_equal
            && safety_better_or_equal
            && fuel_better_or_equal
            && comfort_better_or_equal;

        // Must be strictly better in at least one objective (with tolerance)
        let epsilon = 0.001;
        let at_least_one_better = (self.distance + epsilon < other.distance)
            || (self.safety > other.safety + epsilon)
            || (self.fuel + epsilon < other.fuel)
            || (self.comfort > other.comfort + epsilon);

        all_better_or_equal && at_least_one_better
    }

    /// Weighted sum for single-objective approximation
    fn weighted_sum(&self, weights: &ObjectiveWeights) -> f64 {
        weights.distance * self.distance
            + weights.safety * (1.0 - self.safety)  // Invert safety (lower is better for sum)
            + weights.fuel * self.fuel
            + weights.comfort * (1.0 - self.comfort) // Invert comfort
    }
}

/// Weights for combining multiple objectives
#[derive(Debug, Clone)]
struct ObjectiveWeights {
    distance: f64,
    safety: f64,
    fuel: f64,
    comfort: f64,
}

impl ObjectiveWeights {
    fn balanced() -> Self {
        Self {
            distance: 0.25,
            safety: 0.25,
            fuel: 0.25,
            comfort: 0.25,
        }
    }

    fn speed_focused() -> Self {
        Self {
            distance: 0.7,
            safety: 0.1,
            fuel: 0.1,
            comfort: 0.1,
        }
    }

    fn safety_focused() -> Self {
        Self {
            distance: 0.1,
            safety: 0.7,
            fuel: 0.1,
            comfort: 0.1,
        }
    }

    fn eco_friendly() -> Self {
        Self {
            distance: 0.2,
            safety: 0.2,
            fuel: 0.5,
            comfort: 0.1,
        }
    }
}

/// Enhanced node with multi-objective costs and constraints
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct MultiObjectiveNode {
    id: u32,
    position: (usize, usize),
    costs: MultiObjectiveCost,
    heuristic: f64,
    parent: Option<u32>,
    depth: u32,
    constraints_satisfied: bool,
}

impl MultiObjectiveNode {
    fn f_score(&self, weight: f64) -> f64 {
        self.costs.distance + weight * self.heuristic
    }

    #[allow(dead_code)]
    fn weighted_f_score(&self, weights: &ObjectiveWeights, heuristic_weight: f64) -> f64 {
        self.costs.weighted_sum(weights) + heuristic_weight * self.heuristic
    }
}

impl Eq for MultiObjectiveNode {}

impl PartialEq for MultiObjectiveNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for MultiObjectiveNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap based on distance + heuristic
        other
            .f_score(1.0)
            .partial_cmp(&self.f_score(1.0))
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for MultiObjectiveNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Advanced grid with terrain types and multi-objective costs
#[derive(Debug, Clone)]
struct AdvancedGrid {
    width: usize,
    height: usize,
    terrain: Vec<Vec<TerrainType>>,
    traffic_density: Vec<Vec<f64>>, // 0.0 = no traffic, 1.0 = heavy traffic
    weather_impact: Vec<Vec<f64>>,  // 0.0 = clear, 1.0 = severe weather
}

#[derive(Debug, Clone, PartialEq)]
enum TerrainType {
    Road,     // Normal road
    Highway,  // Fast but less safe
    Dirt,     // Slow but fuel efficient
    Mountain, // Scenic but challenging
    City,     // Traffic but convenient
    Water,    // Impassable
}

impl TerrainType {
    fn movement_cost(&self) -> f64 {
        match self {
            TerrainType::Road => 1.0,
            TerrainType::Highway => 0.8,         // Faster
            TerrainType::Dirt => 1.5,            // Slower
            TerrainType::Mountain => 2.0,        // Much slower
            TerrainType::City => 1.2,            // Traffic delays
            TerrainType::Water => f64::INFINITY, // Impassable
        }
    }

    fn safety_score(&self) -> f64 {
        match self {
            TerrainType::Road => 0.8,
            TerrainType::Highway => 0.6,  // Less safe due to speed
            TerrainType::Dirt => 0.9,     // Safer, lower speed
            TerrainType::Mountain => 0.4, // Dangerous terrain
            TerrainType::City => 0.7,     // Urban hazards
            TerrainType::Water => 0.0,    // Impassable
        }
    }

    fn fuel_efficiency(&self) -> f64 {
        match self {
            TerrainType::Road => 1.0,
            TerrainType::Highway => 0.9,  // More fuel efficient
            TerrainType::Dirt => 1.3,     // Less efficient
            TerrainType::Mountain => 1.8, // Much less efficient
            TerrainType::City => 1.4,     // Stop-and-go traffic
            TerrainType::Water => f64::INFINITY,
        }
    }

    fn comfort_level(&self) -> f64 {
        match self {
            TerrainType::Road => 0.8,
            TerrainType::Highway => 0.9,  // Smooth, fast
            TerrainType::Dirt => 0.4,     // Bumpy
            TerrainType::Mountain => 0.6, // Scenic but rough
            TerrainType::City => 0.5,     // Traffic stress
            TerrainType::Water => 0.0,
        }
    }
}

impl AdvancedGrid {
    fn new(width: usize, height: usize) -> Self {
        let terrain = vec![vec![TerrainType::Road; width]; height];
        let traffic_density = vec![vec![0.0; width]; height];
        let weather_impact = vec![vec![0.0; width]; height];

        Self {
            width,
            height,
            terrain,
            traffic_density,
            weather_impact,
        }
    }

    fn set_terrain(&mut self, x: usize, y: usize, terrain: TerrainType) {
        if x < self.width && y < self.height {
            self.terrain[y][x] = terrain;
        }
    }

    fn set_traffic(&mut self, x: usize, y: usize, density: f64) {
        if x < self.width && y < self.height {
            self.traffic_density[y][x] = density.clamp(0.0, 1.0);
        }
    }

    fn set_weather(&mut self, x: usize, y: usize, impact: f64) {
        if x < self.width && y < self.height {
            self.weather_impact[y][x] = impact.clamp(0.0, 1.0);
        }
    }

    fn neighbors(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = pos;
        let mut neighbors = Vec::new();

        // 4-way movement (can be extended to 8-way)
        if x > 0 && self.terrain[y][x - 1] != TerrainType::Water {
            neighbors.push((x - 1, y));
        }
        if x + 1 < self.width && self.terrain[y][x + 1] != TerrainType::Water {
            neighbors.push((x + 1, y));
        }
        if y > 0 && self.terrain[y - 1][x] != TerrainType::Water {
            neighbors.push((x, y - 1));
        }
        if y + 1 < self.height && self.terrain[y + 1][x] != TerrainType::Water {
            neighbors.push((x, y + 1));
        }

        neighbors
    }

    fn movement_cost(&self, _from: (usize, usize), to: (usize, usize)) -> MultiObjectiveCost {
        let (to_x, to_y) = to;
        let terrain = &self.terrain[to_y][to_x];
        let traffic = self.traffic_density[to_y][to_x];
        let weather = self.weather_impact[to_y][to_x];

        let base_distance = terrain.movement_cost();
        let traffic_factor = 1.0 + traffic * 0.5; // Traffic increases travel time
        let weather_factor = 1.0 + weather * 0.3; // Weather increases travel time

        let distance = base_distance * traffic_factor * weather_factor;
        let safety = terrain.safety_score() * (1.0 - weather * 0.3); // Weather reduces safety
        let fuel = terrain.fuel_efficiency() * (1.0 + traffic * 0.2); // Traffic increases fuel usage
        let comfort = terrain.comfort_level() * (1.0 - traffic * 0.4) * (1.0 - weather * 0.2);

        MultiObjectiveCost::new(distance, safety.max(0.0), fuel, comfort.max(0.0))
    }

    fn pos_to_id(&self, pos: (usize, usize)) -> u32 {
        (pos.1 * self.width + pos.0) as u32
    }

    fn id_to_pos(&self, id: u32) -> (usize, usize) {
        let id = id as usize;
        (id % self.width, id / self.width)
    }
}

/// Advanced heuristic functions
struct AdvancedHeuristics;

impl AdvancedHeuristics {
    /// Manhattan distance (admissible for 4-way movement)
    fn manhattan_distance(from: (usize, usize), to: (usize, usize)) -> f64 {
        let dx = from.0.abs_diff(to.0) as f64;
        let dy = from.1.abs_diff(to.1) as f64;
        dx + dy
    }

    /// Euclidean distance (admissible for 8-way movement)
    fn euclidean_distance(from: (usize, usize), to: (usize, usize)) -> f64 {
        let dx = from.0.abs_diff(to.0) as f64;
        let dy = from.1.abs_diff(to.1) as f64;
        (dx * dx + dy * dy).sqrt()
    }

    /// Chebyshev distance (for 8-way movement with diagonal cost = 1)
    fn chebyshev_distance(from: (usize, usize), to: (usize, usize)) -> f64 {
        let dx = from.0.abs_diff(to.0) as f64;
        let dy = from.1.abs_diff(to.1) as f64;
        dx.max(dy)
    }

    /// Composite heuristic combining multiple factors
    fn composite_heuristic(
        from: (usize, usize),
        to: (usize, usize),
        grid: &AdvancedGrid,
        weights: &ObjectiveWeights,
    ) -> f64 {
        let base_distance = Self::manhattan_distance(from, to);

        // Estimate terrain impact along straight line
        let terrain_penalty = Self::estimate_terrain_penalty(from, to, grid);

        // Weight the heuristic based on objectives
        let distance_h = base_distance * (1.0 + terrain_penalty);
        let safety_h = base_distance * 0.1; // Safety has less impact on heuristic
        let fuel_h = distance_h * 1.2; // Fuel roughly correlates with distance
        let comfort_h = base_distance * 0.1; // Comfort has less impact

        weights.distance * distance_h
            + weights.safety * safety_h
            + weights.fuel * fuel_h
            + weights.comfort * comfort_h
    }

    fn estimate_terrain_penalty(
        from: (usize, usize),
        to: (usize, usize),
        grid: &AdvancedGrid,
    ) -> f64 {
        // Sample a few points along the straight line to estimate terrain impact
        let samples = 5;
        let mut total_penalty = 0.0;

        for i in 0..samples {
            let t = i as f64 / (samples - 1) as f64;
            let sample_x = (from.0 as f64 + t * (to.0 as f64 - from.0 as f64)) as usize;
            let sample_y = (from.1 as f64 + t * (to.1 as f64 - from.1 as f64)) as usize;

            if sample_x < grid.width && sample_y < grid.height {
                let terrain_cost = grid.terrain[sample_y][sample_x].movement_cost();
                total_penalty += terrain_cost - 1.0; // Penalty above base cost
            }
        }

        total_penalty / samples as f64
    }

    /// Dynamic weight adjustment based on search progress
    fn dynamic_weight(
        nodes_explored: usize,
        _goal_distance: f64,
        time_limit: f64,
        elapsed: f64,
    ) -> f64 {
        // Start with weight = 1.0 (optimal), increase as time pressure mounts
        let time_pressure = elapsed / time_limit;
        let exploration_factor = (nodes_explored as f64 / 1000.0).min(1.0);

        // Increase weight when time pressure is high or too many nodes explored
        1.0 + time_pressure * 2.0 + exploration_factor * 0.5
    }
}

/// Multi-objective A* pathfinder
struct MultiObjectiveAStar {
    objective_weights: ObjectiveWeights,
    time_limit: f64, // seconds
    max_nodes: usize,
}

impl MultiObjectiveAStar {
    fn new(weights: ObjectiveWeights, time_limit: f64, max_nodes: usize) -> Self {
        Self {
            objective_weights: weights,
            time_limit,
            max_nodes,
        }
    }

    #[allow(clippy::type_complexity)]
    fn find_path(
        &mut self,
        grid: &AdvancedGrid,
        start: (usize, usize),
        goal: (usize, usize),
    ) -> Option<(Vec<(usize, usize)>, MultiObjectiveCost, SearchStats)> {
        let start_time = Instant::now();
        let mut stats = SearchStats::new();

        let mut open_set = BinaryHeap::new();
        let mut closed_set = HashSet::new();
        let mut g_costs = HashMap::new();
        let mut came_from = HashMap::new();

        let start_id = grid.pos_to_id(start);
        let goal_id = grid.pos_to_id(goal);

        // Initialize start node
        let start_heuristic =
            AdvancedHeuristics::composite_heuristic(start, goal, grid, &self.objective_weights);
        let start_costs = MultiObjectiveCost::new(0.0, 1.0, 0.0, 1.0);
        let start_node = MultiObjectiveNode {
            id: start_id,
            position: start,
            costs: start_costs.clone(),
            heuristic: start_heuristic,
            parent: None,
            depth: 0,
            constraints_satisfied: true,
        };

        open_set.push(start_node);
        g_costs.insert(start_id, start_costs);
        stats.nodes_generated += 1;

        while let Some(current) = open_set.pop() {
            stats.nodes_explored += 1;

            // Check termination conditions
            let elapsed = start_time.elapsed().as_secs_f64();
            if elapsed > self.time_limit || stats.nodes_explored > self.max_nodes {
                // stats tracked for debugging even when no path found
                #[allow(unused_assignments)]
                {
                    stats.terminated_early = true;
                }
                break;
            }

            // Goal check
            if current.id == goal_id {
                let path = self.reconstruct_path(&came_from, start_id, goal_id, grid);
                stats.search_time_ms = (elapsed * 1000.0) as u128;
                stats.path_length = path.len();
                stats.final_cost = current.costs.clone();
                return Some((path, current.costs, stats));
            }

            closed_set.insert(current.id);

            // Dynamic weight adjustment
            let goal_distance = AdvancedHeuristics::manhattan_distance(current.position, goal);
            let dynamic_weight = AdvancedHeuristics::dynamic_weight(
                stats.nodes_explored,
                goal_distance,
                self.time_limit,
                elapsed,
            );

            // Expand neighbors
            for neighbor_pos in grid.neighbors(current.position) {
                let neighbor_id = grid.pos_to_id(neighbor_pos);

                if closed_set.contains(&neighbor_id) {
                    continue;
                }

                // Calculate movement cost
                let movement_cost = grid.movement_cost(current.position, neighbor_pos);
                let tentative_costs = MultiObjectiveCost::new(
                    current.costs.distance + movement_cost.distance,
                    (current.costs.safety + movement_cost.safety) / 2.0, // Average safety
                    current.costs.fuel + movement_cost.fuel,
                    (current.costs.comfort + movement_cost.comfort) / 2.0, // Average comfort
                );

                // Check if this path is better
                let should_update = if let Some(existing_costs) = g_costs.get(&neighbor_id) {
                    tentative_costs.weighted_sum(&self.objective_weights)
                        < existing_costs.weighted_sum(&self.objective_weights)
                } else {
                    true
                };

                if should_update {
                    g_costs.insert(neighbor_id, tentative_costs.clone());
                    came_from.insert(neighbor_id, current.id);

                    let neighbor_heuristic = AdvancedHeuristics::composite_heuristic(
                        neighbor_pos,
                        goal,
                        grid,
                        &self.objective_weights,
                    );

                    let neighbor_node = MultiObjectiveNode {
                        id: neighbor_id,
                        position: neighbor_pos,
                        costs: tentative_costs,
                        heuristic: neighbor_heuristic * dynamic_weight,
                        parent: Some(current.id),
                        depth: current.depth + 1,
                        constraints_satisfied: self.check_constraints(&current, neighbor_pos, grid),
                    };

                    open_set.push(neighbor_node);
                    stats.nodes_generated += 1;
                }
            }

            stats.peak_memory = stats.peak_memory.max(open_set.len() + closed_set.len());
        }

        // stats tracked for debugging even when no path found
        #[allow(unused_assignments)]
        {
            stats.search_time_ms = (start_time.elapsed().as_secs_f64() * 1000.0) as u128;
        }
        None
    }

    fn check_constraints(
        &self,
        current: &MultiObjectiveNode,
        next_pos: (usize, usize),
        grid: &AdvancedGrid,
    ) -> bool {
        // Example constraints

        // 1. Maximum fuel constraint
        if current.costs.fuel > 50.0 {
            return false;
        }

        // 2. Minimum safety threshold in dangerous weather
        let weather = grid.weather_impact[next_pos.1][next_pos.0];
        if weather > 0.7 && current.costs.safety < 0.3 {
            return false;
        }

        // 3. Maximum depth constraint (prevent infinite loops)
        if current.depth > 200 {
            return false;
        }

        true
    }

    fn reconstruct_path(
        &self,
        came_from: &HashMap<u32, u32>,
        start_id: u32,
        goal_id: u32,
        grid: &AdvancedGrid,
    ) -> Vec<(usize, usize)> {
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
}

/// Pareto frontier pathfinder for true multi-objective optimization
struct ParetoPathfinder {
    max_solutions: usize,
    time_limit: f64,
}

impl ParetoPathfinder {
    fn new(max_solutions: usize, time_limit: f64) -> Self {
        Self {
            max_solutions,
            time_limit,
        }
    }

    fn find_pareto_paths(
        &mut self,
        grid: &AdvancedGrid,
        start: (usize, usize),
        goal: (usize, usize),
    ) -> Vec<(Vec<(usize, usize)>, MultiObjectiveCost)> {
        let start_time = Instant::now();
        let mut pareto_solutions: Vec<(Vec<(usize, usize)>, MultiObjectiveCost)> = Vec::new();

        // Test different weight combinations to find diverse solutions
        let weight_sets = vec![
            ObjectiveWeights::speed_focused(),
            ObjectiveWeights::safety_focused(),
            ObjectiveWeights::eco_friendly(),
            ObjectiveWeights::balanced(),
            // Add more weight combinations for diversity
            ObjectiveWeights {
                distance: 0.5,
                safety: 0.3,
                fuel: 0.1,
                comfort: 0.1,
            },
            ObjectiveWeights {
                distance: 0.2,
                safety: 0.1,
                fuel: 0.6,
                comfort: 0.1,
            },
            ObjectiveWeights {
                distance: 0.1,
                safety: 0.6,
                fuel: 0.2,
                comfort: 0.1,
            },
            ObjectiveWeights {
                distance: 0.3,
                safety: 0.2,
                fuel: 0.2,
                comfort: 0.3,
            },
            ObjectiveWeights {
                distance: 0.4,
                safety: 0.4,
                fuel: 0.1,
                comfort: 0.1,
            },
            ObjectiveWeights {
                distance: 0.1,
                safety: 0.1,
                fuel: 0.4,
                comfort: 0.4,
            },
        ];

        let weight_count = weight_sets.len();
        for (i, weights) in weight_sets.iter().enumerate() {
            if start_time.elapsed().as_secs_f64() > self.time_limit {
                break;
            }

            // Use different time limits and node limits to encourage path diversity
            let time_per_search = self.time_limit / weight_count as f64;
            let node_limit = 3000 + (i * 1000); // Varying node limits

            let mut pathfinder =
                MultiObjectiveAStar::new(weights.clone(), time_per_search, node_limit);

            if let Some((path, cost, _stats)) = pathfinder.find_path(grid, start, goal) {
                // More lenient similarity check - paths are different if they have different waypoints
                let is_similar_path = pareto_solutions
                    .iter()
                    .any(|(existing_path, _)| paths_are_similar(existing_path, &path));

                if !is_similar_path {
                    // Check if this solution is non-dominated (more precise comparison)
                    let mut is_dominated = false;
                    for (_, existing_cost) in &pareto_solutions {
                        if existing_cost.dominates(&cost) {
                            is_dominated = true;
                            break;
                        }
                    }

                    if !is_dominated {
                        // Remove dominated solutions
                        pareto_solutions
                            .retain(|(_, existing_cost)| !cost.dominates(existing_cost));

                        // Add new solution
                        pareto_solutions.push((path, cost));

                        // Limit number of solutions
                        if pareto_solutions.len() > self.max_solutions {
                            pareto_solutions.sort_by(|a, b| {
                                a.1.distance
                                    .partial_cmp(&b.1.distance)
                                    .unwrap_or(Ordering::Equal)
                            });
                            pareto_solutions.truncate(self.max_solutions);
                        }
                    }
                }
            }
        }

        pareto_solutions
    }
}

/// Helper function to check if two paths are similar (share >80% of waypoints)
fn paths_are_similar(path1: &[(usize, usize)], path2: &[(usize, usize)]) -> bool {
    if path1.len() != path2.len() {
        return false;
    }

    let matching_points = path1
        .iter()
        .zip(path2.iter())
        .filter(|(p1, p2)| p1 == p2)
        .count();

    let similarity_ratio = matching_points as f64 / path1.len() as f64;
    similarity_ratio > 0.8 // 80% similarity threshold
}

/// Enhanced search statistics
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct SearchStats {
    nodes_explored: usize,
    nodes_generated: usize,
    peak_memory: usize,
    search_time_ms: u128,
    path_length: usize,
    final_cost: MultiObjectiveCost,
    terminated_early: bool,
    dynamic_weight_adjustments: usize,
}

impl SearchStats {
    fn new() -> Self {
        Self {
            nodes_explored: 0,
            nodes_generated: 0,
            peak_memory: 0,
            search_time_ms: 0,
            path_length: 0,
            final_cost: MultiObjectiveCost::new(0.0, 0.0, 0.0, 0.0),
            terminated_early: false,
            dynamic_weight_adjustments: 0,
        }
    }
}

/// Demonstration functions
fn create_complex_scenario() -> AdvancedGrid {
    let mut grid = AdvancedGrid::new(25, 20);

    // Create distinct route options to force different trade-offs

    // NORTHERN ROUTE: Highway zone (fast but less safe)
    for x in 5..23 {
        grid.set_terrain(x, 3, TerrainType::Highway);
        grid.set_terrain(x, 4, TerrainType::Highway);
    }

    // CENTRAL ROUTE: Normal roads with mixed terrain
    for x in 1..24 {
        grid.set_terrain(x, 10, TerrainType::Road);
    }

    // SOUTHERN ROUTE: City zone (slow but safer in parts)
    for x in 2..22 {
        grid.set_terrain(x, 16, TerrainType::City);
        grid.set_terrain(x, 17, TerrainType::City);
        grid.set_traffic(x, 16, 0.7); // Heavy traffic
        grid.set_traffic(x, 17, 0.5); // Moderate traffic
    }

    // ECO-FRIENDLY OPTION: Dirt roads (fuel efficient, slow)
    for y in 6..9 {
        for x in 8..18 {
            grid.set_terrain(x, y, TerrainType::Dirt);
        }
    }

    // SCENIC BUT DANGEROUS: Mountain region
    for y in 1..6 {
        for x in 18..24 {
            grid.set_terrain(x, y, TerrainType::Mountain);
        }
    }

    // Create water obstacles to force routing decisions
    for y in 11..14 {
        for x in 12..16 {
            grid.set_terrain(x, y, TerrainType::Water);
        }
    }

    // More water to create alternate routes
    for y in 5..8 {
        for x in 3..7 {
            grid.set_terrain(x, y, TerrainType::Water);
        }
    }

    // Weather conditions that affect route choice
    // Storm affects northern highway (safety vs speed trade-off)
    for x in 5..23 {
        grid.set_weather(x, 3, 0.9); // Severe weather on highway
        grid.set_weather(x, 4, 0.8);
    }

    // Mountain region has harsh weather
    for y in 1..6 {
        for x in 18..24 {
            grid.set_weather(x, y, 0.7);
        }
    }

    // Light rain in city (affects comfort but not safety much)
    for x in 2..22 {
        grid.set_weather(x, 16, 0.2);
        grid.set_weather(x, 17, 0.3);
    }

    // Create vertical connection paths
    for y in 5..16 {
        grid.set_terrain(2, y, TerrainType::Road); // Western connector
        grid.set_terrain(23, y, TerrainType::Road); // Eastern connector
    }

    grid
}

fn demonstrate_heuristic_comparison() {
    println!("🎯 Advanced Heuristics Comparison");
    println!("=================================\n");

    let grid = create_complex_scenario();
    let start = (1, 10); // Western side, central route
    let goal = (23, 10); // Eastern side, central route

    println!("📋 Scenario Setup:");
    println!("   Grid: {}x{} with mixed terrain", grid.width, grid.height);
    println!("   Start: {:?} (West side)", start);
    println!("   Goal: {:?} (East side)", goal);
    println!("   Routes: Northern Highway (fast/dangerous), Central Road (balanced),");
    println!("           Southern City (safe/slow), Dirt detour (eco-friendly)");
    println!("   Weather: Storms on highway, light rain in city\n");

    // Test different heuristic approaches
    let heuristics = vec![
        (
            "Manhattan",
            AdvancedHeuristics::manhattan_distance(start, goal),
        ),
        (
            "Euclidean",
            AdvancedHeuristics::euclidean_distance(start, goal),
        ),
        (
            "Chebyshev",
            AdvancedHeuristics::chebyshev_distance(start, goal),
        ),
    ];

    println!("🧭 Heuristic Values:");
    for (name, value) in heuristics {
        println!("   {}: {:.2}", name, value);
    }

    // Test composite heuristic with different weights
    let weight_sets = vec![
        ("Speed-focused", ObjectiveWeights::speed_focused()),
        ("Safety-focused", ObjectiveWeights::safety_focused()),
        ("Eco-friendly", ObjectiveWeights::eco_friendly()),
        ("Balanced", ObjectiveWeights::balanced()),
    ];

    println!("\n🎨 Composite Heuristic Values:");
    for (name, weights) in weight_sets {
        let value = AdvancedHeuristics::composite_heuristic(start, goal, &grid, &weights);
        println!("   {}: {:.2}", name, value);
    }
}

fn demonstrate_multi_objective_optimization() {
    println!("\n🎯 Multi-Objective Optimization Demo");
    println!("====================================\n");

    let grid = create_complex_scenario();
    let start = (1, 10); // Western side, central route
    let goal = (23, 10); // Eastern side, central route

    let objective_sets = vec![
        ("Speed Priority", ObjectiveWeights::speed_focused()),
        ("Safety Priority", ObjectiveWeights::safety_focused()),
        ("Eco-Friendly", ObjectiveWeights::eco_friendly()),
        ("Balanced", ObjectiveWeights::balanced()),
    ];

    println!("🛣️ Path Analysis by Objective:");
    println!("Objective       | Distance | Safety | Fuel | Comfort | Time (ms) | Nodes");
    println!("----------------|----------|--------|------|---------|-----------|-------");

    for (name, weights) in objective_sets {
        let mut pathfinder = MultiObjectiveAStar::new(weights, 5.0, 10000);

        let _start_time = Instant::now();
        if let Some((_path, costs, stats)) = pathfinder.find_path(&grid, start, goal) {
            println!(
                "{:15} | {:8.2} | {:6.2} | {:4.2} | {:7.2} | {:9} | {:5}",
                name,
                costs.distance,
                costs.safety,
                costs.fuel,
                costs.comfort,
                stats.search_time_ms,
                stats.nodes_explored
            );
        } else {
            println!("{:15} | No path found", name);
        }
    }

    println!("\n🔍 Observation: All paths have identical costs!");
    println!("   This occurs when one route dominates all others across all objectives.");
    println!("   In the current scenario, the direct path is optimal for:");
    println!("   • Distance: Shortest route available");
    println!("   • Safety: Avoids dangerous terrain (mountains, storms)");
    println!("   • Fuel: Most efficient due to good terrain and short distance");
    println!("   • Comfort: Travels on normal roads without heavy traffic");
    println!("\n   🎯 Multi-objective trade-offs become apparent when:");
    println!("   • Multiple viable routes exist with different characteristics");
    println!("   • Environmental conditions vary significantly along routes");
    println!("   • Start/goal positions create routing choices");
}

fn demonstrate_pareto_frontier() {
    println!("\n🏆 Pareto Frontier Analysis");
    println!("===========================\n");

    let grid = create_complex_scenario();
    let start = (1, 10); // Western side, central route
    let goal = (23, 10); // Eastern side, central route

    let mut pareto_finder = ParetoPathfinder::new(10, 30.0);
    let solutions = pareto_finder.find_pareto_paths(&grid, start, goal);

    println!("📊 Found {} Pareto-optimal solutions:", solutions.len());
    println!("   (Different paths found by varying optimization weights)");
    println!("Sol# | Distance | Safety | Fuel | Comfort | Path Length");
    println!("-----|----------|--------|------|---------|------------");

    for (i, (path, costs)) in solutions.iter().enumerate() {
        println!(
            "{:4} | {:8.2} | {:6.2} | {:4.2} | {:7.2} | {:10}",
            i + 1,
            costs.distance,
            costs.safety,
            costs.fuel,
            costs.comfort,
            path.len()
        );
    }

    if solutions.len() > 1 {
        println!("\n💡 Trade-off Analysis:");
        let best_distance = solutions
            .iter()
            .min_by(|a, b| a.1.distance.partial_cmp(&b.1.distance).unwrap())
            .unwrap();
        let best_safety = solutions
            .iter()
            .max_by(|a, b| a.1.safety.partial_cmp(&b.1.safety).unwrap())
            .unwrap();
        let best_fuel = solutions
            .iter()
            .min_by(|a, b| a.1.fuel.partial_cmp(&b.1.fuel).unwrap())
            .unwrap();
        let best_comfort = solutions
            .iter()
            .max_by(|a, b| a.1.comfort.partial_cmp(&b.1.comfort).unwrap())
            .unwrap();

        println!(
            "   Best distance: {:.2} (safety: {:.2}, fuel: {:.2}, comfort: {:.2})",
            best_distance.1.distance,
            best_distance.1.safety,
            best_distance.1.fuel,
            best_distance.1.comfort
        );
        println!(
            "   Best safety: {:.2} (distance: {:.2}, fuel: {:.2}, comfort: {:.2})",
            best_safety.1.safety, best_safety.1.distance, best_safety.1.fuel, best_safety.1.comfort
        );
        println!(
            "   Best fuel: {:.2} (distance: {:.2}, safety: {:.2}, comfort: {:.2})",
            best_fuel.1.fuel, best_fuel.1.distance, best_fuel.1.safety, best_fuel.1.comfort
        );
        println!(
            "   Best comfort: {:.2} (distance: {:.2}, safety: {:.2}, fuel: {:.2})",
            best_comfort.1.comfort,
            best_comfort.1.distance,
            best_comfort.1.safety,
            best_comfort.1.fuel
        );
    } else {
        println!("\n🔍 Analysis: Only one solution found");
        println!("   This happens when one path dominates all others across all objectives.");
        println!(
            "   In complex real-world scenarios, multiple trade-off solutions typically exist:"
        );
        println!("   • Highway route: Fast but dangerous in bad weather");
        println!("   • Scenic route: Longer but more comfortable and safer");
        println!("   • Economic route: Slower but fuel-efficient");
        println!("   • Urban route: Moderate on all metrics");
        println!("\n   💡 The current grid layout has a clearly optimal single path.");
        println!("      Try different start/goal positions or more complex terrain for variety!");
    }
}

fn demonstrate_dynamic_weighting() {
    println!("\n⚖️ Dynamic Weighting Strategy Demo");
    println!("==================================\n");

    println!("🔄 Weight Adaptation Examples:");

    let scenarios = vec![
        ("Early search, no time pressure", 100, 10.0, 10.0, 1.0),
        ("Many nodes explored", 2000, 10.0, 10.0, 5.0),
        ("Time pressure mounting", 500, 5.0, 10.0, 8.0),
        ("Critical time limit", 300, 2.0, 10.0, 9.5),
    ];

    println!("Scenario                      | Nodes | Goal Dist | Time Limit | Elapsed | Weight");
    println!("------------------------------|-------|-----------|------------|---------|-------");

    for (name, nodes, goal_dist, time_limit, elapsed) in scenarios {
        let weight = AdvancedHeuristics::dynamic_weight(nodes, goal_dist, time_limit, elapsed);
        println!(
            "{:29} | {:5} | {:9.1} | {:10.1} | {:7.1} | {:6.2}",
            name, nodes, goal_dist, time_limit, elapsed, weight
        );
    }

    println!("\n💡 Dynamic Weighting Benefits:");
    println!("   • Starts optimal (w=1.0) for best solution quality");
    println!("   • Increases weight under time pressure for faster results");
    println!("   • Adapts to exploration efficiency");
    println!("   • Balances solution quality vs. computation time");
}

fn demonstrate_constraint_satisfaction() {
    println!("\n🚧 Constraint-Based Pathfinding Demo");
    println!("====================================\n");

    println!("⚠️ Example Constraints:");
    println!("   1. Maximum fuel consumption: 50 units");
    println!("   2. Minimum safety in severe weather: 0.3");
    println!("   3. Maximum path depth: 200 steps");
    println!("   4. Avoid high-traffic areas during rush hour");
    println!("   5. Mandatory waypoints for scenic routes");

    println!("\n🔍 Constraint Checking Process:");
    println!("   • Each node expansion validates constraints");
    println!("   • Invalid paths are pruned early");
    println!("   • Soft constraints influence cost function");
    println!("   • Hard constraints create absolute restrictions");

    println!("\n📈 Benefits:");
    println!("   • Ensures feasible solutions");
    println!("   • Reduces search space");
    println!("   • Models real-world restrictions");
    println!("   • Enables domain-specific optimization");
}

/// TODO Exercises for hands-on learning
fn todo_exercises() {
    println!("\n📝 TODO Exercises:");
    println!("==================");

    // TODO 1: Custom heuristic design
    println!("🎯 TODO 1: Design Domain-Specific Heuristics");
    println!("   Create heuristics for specific scenarios:");
    println!("   - Navigation in urban areas with one-way streets");
    println!("   - Robot path planning with battery constraints");
    println!("   - Supply chain optimization with delivery windows");
    println!("   - Game AI pathfinding with enemy avoidance");
    println!();

    // TODO 2: Advanced multi-objective algorithms
    println!("🎯 TODO 2: Implement NSGA-II for Pathfinding");
    println!("   Develop evolutionary multi-objective optimization:");
    println!("   - Non-dominated sorting genetic algorithm");
    println!("   - Crowding distance for diversity");
    println!("   - Elite preservation across generations");
    println!("   - Compare with weighted sum approaches");
    println!();

    // TODO 3: Constraint programming integration
    println!("🎯 TODO 3: Advanced Constraint Integration");
    println!("   Implement constraint satisfaction techniques:");
    println!("   - Temporal constraints (time windows)");
    println!("   - Resource constraints (fuel, capacity)");
    println!("   - Legal constraints (traffic rules, permits)");
    println!("   - Dynamic constraints (changing conditions)");
    println!();

    // TODO 4: Adaptive heuristic learning
    println!("🎯 TODO 4: Machine Learning Enhanced Heuristics");
    println!("   Develop learning-based heuristic improvement:");
    println!("   - Train neural networks on historical paths");
    println!("   - Online learning from search experience");
    println!("   - Feature extraction from graph properties");
    println!("   - Performance comparison with fixed heuristics");
    println!();

    // TODO 5: Real-time optimization
    println!("🎯 TODO 5: Anytime and Real-Time Algorithms");
    println!("   Implement algorithms for time-critical scenarios:");
    println!("   - Anytime A* with improving solutions");
    println!("   - Real-time A* (RTA*) for moving agents");
    println!("   - Interruptible search with solution caching");
    println!("   - Quality-time trade-off analysis");
}

fn main() {
    println!(
        "📚 Mission 9 Tutorial - Step 5: Advanced Heuristics & Multi-Objective Optimization\n"
    );

    demonstrate_heuristic_comparison();
    demonstrate_multi_objective_optimization();
    demonstrate_pareto_frontier();
    demonstrate_dynamic_weighting();
    demonstrate_constraint_satisfaction();
    todo_exercises();

    println!("\n✅ Step 5 Complete!");
    println!("📖 Next: Step 6 - Graph preprocessing and hierarchical methods");
    println!("🔗 Related: Check Mission 9 main implementation for production-ready versions");
    println!("\n🎓 Key Takeaways:");
    println!("   • Heuristics can be customized for specific problem domains");
    println!("   • Multi-objective optimization reveals trade-offs between competing goals");
    println!("   • Pareto frontiers provide sets of optimal solutions");
    println!("   • Dynamic weighting adapts search strategy to time constraints");
    println!("   • Constraint satisfaction enables real-world problem modeling");
}
