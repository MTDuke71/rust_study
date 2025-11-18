//! Multi-objective pathfinding implementation
//!
//! This module provides algorithms for finding Pareto-optimal paths when multiple
//! conflicting objectives need to be optimized simultaneously (e.g., cost vs time vs safety).

use crate::{NodeId, PathfindingError, PathfindingResult, Weight, WeightedGraph};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

/// Represents a single objective function for multi-objective optimization
pub trait ObjectiveFunction: std::fmt::Debug {
    /// Evaluate the objective value for an edge
    fn evaluate_edge(&self, from: NodeId, to: NodeId, graph: &dyn WeightedGraph) -> Weight;

    /// Get the name of this objective
    fn name(&self) -> &str;

    /// Whether lower values are better (true) or higher values are better (false)
    fn is_minimizing(&self) -> bool {
        true
    }

    /// Clone this objective function
    fn clone_objective(&self) -> ObjectiveFunctionType;
}

/// Enum representing different types of objective functions
#[derive(Debug, Clone)]
pub enum ObjectiveFunctionType {
    Cost(CostObjective),
    Time(TimeObjective),
    Safety(SafetyObjective),
}

impl ObjectiveFunctionType {
    pub fn evaluate_edge(&self, from: NodeId, to: NodeId, graph: &dyn WeightedGraph) -> Weight {
        match self {
            ObjectiveFunctionType::Cost(obj) => obj.evaluate_edge(from, to, graph),
            ObjectiveFunctionType::Time(obj) => obj.evaluate_edge(from, to, graph),
            ObjectiveFunctionType::Safety(obj) => obj.evaluate_edge(from, to, graph),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ObjectiveFunctionType::Cost(obj) => obj.name(),
            ObjectiveFunctionType::Time(obj) => obj.name(),
            ObjectiveFunctionType::Safety(obj) => obj.name(),
        }
    }

    pub fn is_minimizing(&self) -> bool {
        match self {
            ObjectiveFunctionType::Cost(obj) => obj.is_minimizing(),
            ObjectiveFunctionType::Time(obj) => obj.is_minimizing(),
            ObjectiveFunctionType::Safety(obj) => obj.is_minimizing(),
        }
    }
}

/// Standard cost objective (edge weights)
#[derive(Debug, Clone)]
pub struct CostObjective;

impl ObjectiveFunction for CostObjective {
    fn evaluate_edge(&self, from: NodeId, to: NodeId, graph: &dyn WeightedGraph) -> Weight {
        graph
            .neighbors(from)
            .iter()
            .find(|(neighbor, _)| *neighbor == to)
            .map(|(_, weight)| *weight)
            .unwrap_or(Weight::INFINITY)
    }

    fn name(&self) -> &str {
        "Cost"
    }

    fn clone_objective(&self) -> ObjectiveFunctionType {
        ObjectiveFunctionType::Cost(self.clone())
    }
}

/// Time objective (could be based on edge properties or traffic conditions)
#[derive(Debug, Clone)]
pub struct TimeObjective {
    time_multipliers: HashMap<(NodeId, NodeId), Weight>,
}

impl TimeObjective {
    pub fn new() -> Self {
        Self {
            time_multipliers: HashMap::new(),
        }
    }

    pub fn set_time_multiplier(&mut self, from: NodeId, to: NodeId, multiplier: Weight) {
        self.time_multipliers.insert((from, to), multiplier);
        self.time_multipliers.insert((to, from), multiplier); // Symmetric
    }
}

impl Default for TimeObjective {
    fn default() -> Self {
        Self::new()
    }
}

impl ObjectiveFunction for TimeObjective {
    fn evaluate_edge(&self, from: NodeId, to: NodeId, graph: &dyn WeightedGraph) -> Weight {
        let base_cost = CostObjective.evaluate_edge(from, to, graph);
        let multiplier = self
            .time_multipliers
            .get(&(from, to))
            .copied()
            .unwrap_or(1.0);
        base_cost * multiplier
    }

    fn name(&self) -> &str {
        "Time"
    }

    fn clone_objective(&self) -> ObjectiveFunctionType {
        ObjectiveFunctionType::Time(self.clone())
    }
}

/// Safety objective (higher values indicate more dangerous paths)
#[derive(Debug, Clone)]
pub struct SafetyObjective {
    danger_zones: HashSet<NodeId>,
    danger_penalty: Weight,
}

impl SafetyObjective {
    pub fn new(danger_penalty: Weight) -> Self {
        Self {
            danger_zones: HashSet::new(),
            danger_penalty,
        }
    }

    pub fn add_danger_zone(&mut self, node: NodeId) {
        self.danger_zones.insert(node);
    }

    pub fn add_danger_zones(&mut self, nodes: &[NodeId]) {
        for &node in nodes {
            self.danger_zones.insert(node);
        }
    }
}

impl ObjectiveFunction for SafetyObjective {
    fn evaluate_edge(&self, _from: NodeId, to: NodeId, _graph: &dyn WeightedGraph) -> Weight {
        if self.danger_zones.contains(&to) {
            self.danger_penalty
        } else {
            0.0
        }
    }

    fn name(&self) -> &str {
        "Safety"
    }

    fn clone_objective(&self) -> ObjectiveFunctionType {
        ObjectiveFunctionType::Safety(self.clone())
    }
}

/// A solution in the multi-objective space
#[derive(Debug, Clone)]
pub struct MultiObjectiveSolution {
    pub path: Vec<NodeId>,
    pub objective_values: Vec<Weight>,
    pub total_cost: Weight, // For pathfinding compatibility
}

impl MultiObjectiveSolution {
    pub fn new(path: Vec<NodeId>, objective_values: Vec<Weight>) -> Self {
        let total_cost = objective_values.first().copied().unwrap_or(0.0);
        Self {
            path,
            objective_values,
            total_cost,
        }
    }

    /// Check if this solution dominates another (better in all objectives)
    pub fn dominates(&self, other: &MultiObjectiveSolution) -> bool {
        let mut strictly_better_in_at_least_one = false;

        for (my_value, other_value) in self.objective_values.iter().zip(&other.objective_values) {
            if my_value > other_value {
                return false; // Worse in this objective
            }
            if my_value < other_value {
                strictly_better_in_at_least_one = true;
            }
        }

        strictly_better_in_at_least_one
    }

    /// Check if this solution is non-dominated by another
    pub fn is_non_dominated_by(&self, other: &MultiObjectiveSolution) -> bool {
        !other.dominates(self)
    }
}

/// Result of multi-objective pathfinding
#[derive(Debug, Clone)]
pub struct MultiObjectiveResult {
    pub pareto_optimal_solutions: Vec<MultiObjectiveSolution>,
    pub objective_names: Vec<String>,
    pub nodes_explored: usize,
    pub search_time_us: u64,
}

impl MultiObjectiveResult {
    /// Get the solution that is best for a specific objective
    pub fn best_for_objective(&self, objective_index: usize) -> Option<&MultiObjectiveSolution> {
        self.pareto_optimal_solutions.iter().min_by(|a, b| {
            a.objective_values[objective_index]
                .partial_cmp(&b.objective_values[objective_index])
                .unwrap_or(Ordering::Equal)
        })
    }

    /// Get a compromise solution (balanced across all objectives)
    pub fn compromise_solution(&self) -> Option<&MultiObjectiveSolution> {
        if self.pareto_optimal_solutions.is_empty() {
            return None;
        }

        // Find the solution with minimum sum of normalized objective values
        let mut min_values = vec![Weight::INFINITY; self.objective_names.len()];
        let mut max_values = vec![Weight::NEG_INFINITY; self.objective_names.len()];

        // Find min/max for normalization
        for solution in &self.pareto_optimal_solutions {
            for (i, &value) in solution.objective_values.iter().enumerate() {
                min_values[i] = min_values[i].min(value);
                max_values[i] = max_values[i].max(value);
            }
        }

        // Find solution with minimum normalized sum
        self.pareto_optimal_solutions.iter().min_by(|a, b| {
            let a_sum: Weight = a
                .objective_values
                .iter()
                .enumerate()
                .map(|(i, &value)| {
                    if max_values[i] > min_values[i] {
                        (value - min_values[i]) / (max_values[i] - min_values[i])
                    } else {
                        0.0
                    }
                })
                .sum();

            let b_sum: Weight = b
                .objective_values
                .iter()
                .enumerate()
                .map(|(i, &value)| {
                    if max_values[i] > min_values[i] {
                        (value - min_values[i]) / (max_values[i] - min_values[i])
                    } else {
                        0.0
                    }
                })
                .sum();

            a_sum.partial_cmp(&b_sum).unwrap_or(Ordering::Equal)
        })
    }
}

/// Multi-objective A* pathfinding algorithm
#[derive(Debug, Clone)]
pub struct MultiObjectiveAstar {
    objectives: Vec<ObjectiveFunctionType>,
    max_solutions: usize,
    epsilon: Weight, // Approximation factor for efficiency
}

impl MultiObjectiveAstar {
    pub fn new() -> Self {
        Self {
            objectives: Vec::new(),
            max_solutions: 100,
            epsilon: 0.1,
        }
    }

    pub fn add_objective<T: ObjectiveFunction + 'static>(mut self, objective: T) -> Self {
        self.objectives.push(objective.clone_objective());
        self
    }

    pub fn with_max_solutions(mut self, max_solutions: usize) -> Self {
        self.max_solutions = max_solutions;
        self
    }

    pub fn with_epsilon(mut self, epsilon: Weight) -> Self {
        self.epsilon = epsilon;
        self
    }

    /// Find Pareto-optimal paths using multi-objective A*
    pub fn find_pareto_optimal_paths(
        &self,
        graph: &dyn WeightedGraph,
        start: NodeId,
        goal: NodeId,
    ) -> PathfindingResult<MultiObjectiveResult> {
        let start_time = std::time::Instant::now();
        let mut nodes_explored = 0;

        if self.objectives.is_empty() {
            return Err(PathfindingError::InvalidConfiguration {
                message: "No objectives specified for multi-objective search".to_string(),
            });
        }

        // Generate weight vectors for scalarization
        let weight_vectors = self.generate_weight_vectors();
        let mut pareto_solutions: Vec<MultiObjectiveSolution> = Vec::new();

        // For each weight vector, run weighted-sum A*
        for weights in weight_vectors {
            if let Ok(solution) = self.weighted_sum_astar(graph, start, goal, &weights) {
                nodes_explored += 1; // Approximate

                // Check if this solution is non-dominated
                if pareto_solutions
                    .iter()
                    .all(|existing| solution.is_non_dominated_by(existing))
                {
                    // Remove any solutions dominated by the new one
                    pareto_solutions.retain(|existing| !solution.dominates(existing));
                    pareto_solutions.push(solution);

                    // Limit the number of solutions
                    if pareto_solutions.len() > self.max_solutions {
                        break;
                    }
                }
            }
        }

        if pareto_solutions.is_empty() {
            return Err(PathfindingError::NoPathExists { start, goal });
        }

        let search_time_us = start_time.elapsed().as_micros() as u64;
        let objective_names = self
            .objectives
            .iter()
            .map(|obj| obj.name().to_string())
            .collect();

        Ok(MultiObjectiveResult {
            pareto_optimal_solutions: pareto_solutions,
            objective_names,
            nodes_explored,
            search_time_us,
        })
    }

    /// Generate weight vectors for scalarization
    fn generate_weight_vectors(&self) -> Vec<Vec<Weight>> {
        let num_objectives = self.objectives.len();
        if num_objectives == 1 {
            return vec![vec![1.0]];
        }

        let mut weight_vectors = Vec::new();
        let resolution = 10; // Number of points per dimension

        // Generate uniform weight vectors using recursive approach
        self.generate_weights_recursive(
            num_objectives,
            resolution,
            vec![],
            1.0,
            &mut weight_vectors,
        );

        weight_vectors
    }

    #[allow(clippy::only_used_in_recursion)]
    fn generate_weights_recursive(
        &self,
        remaining_objectives: usize,
        resolution: usize,
        mut current_weights: Vec<Weight>,
        remaining_weight: Weight,
        result: &mut Vec<Vec<Weight>>,
    ) {
        if remaining_objectives == 1 {
            current_weights.push(remaining_weight);
            result.push(current_weights);
            return;
        }

        for i in 0..=resolution {
            let weight = (i as Weight) / (resolution as Weight) * remaining_weight;
            let mut new_weights = current_weights.clone();
            new_weights.push(weight);

            self.generate_weights_recursive(
                remaining_objectives - 1,
                resolution,
                new_weights,
                remaining_weight - weight,
                result,
            );
        }
    }

    /// Run weighted-sum A* for a given weight vector
    fn weighted_sum_astar(
        &self,
        graph: &dyn WeightedGraph,
        start: NodeId,
        goal: NodeId,
        weights: &[Weight],
    ) -> PathfindingResult<MultiObjectiveSolution> {
        #[derive(Debug, Clone)]
        struct SearchNode {
            node: NodeId,
            g_scores: Vec<Weight>, // Objective values from start
            f_score: Weight,       // Weighted sum for priority
            #[allow(dead_code)]
            parent: Option<NodeId>,
        }

        impl PartialEq for SearchNode {
            fn eq(&self, other: &Self) -> bool {
                self.f_score == other.f_score
            }
        }

        impl Eq for SearchNode {}

        impl PartialOrd for SearchNode {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for SearchNode {
            fn cmp(&self, other: &Self) -> Ordering {
                // Min-heap: reverse comparison for f_score
                other
                    .f_score
                    .partial_cmp(&self.f_score)
                    .unwrap_or(Ordering::Equal)
            }
        }

        let mut open_set = BinaryHeap::new();
        let mut closed_set: HashSet<NodeId> = HashSet::new();
        let mut best_g_scores: HashMap<NodeId, Vec<Weight>> = HashMap::new();
        let mut came_from: HashMap<NodeId, NodeId> = HashMap::new();
        let mut iterations = 0;
        let max_iterations = 100_000; // Prevent infinite loops

        // Initialize start node
        let start_g_scores = vec![0.0; self.objectives.len()];
        let start_f_score = 0.0; // Heuristic would go here

        open_set.push(SearchNode {
            node: start,
            g_scores: start_g_scores.clone(),
            f_score: start_f_score,
            parent: None,
        });
        best_g_scores.insert(start, start_g_scores);

        while let Some(current) = open_set.pop() {
            iterations += 1;
            if iterations >= max_iterations {
                return Err(PathfindingError::MaxIterationsExceeded);
            }

            // Skip if already processed
            if closed_set.contains(&current.node) {
                continue;
            }

            if current.node == goal {
                // Reconstruct path
                let mut path = Vec::new();
                let mut current_node = goal;

                while current_node != start {
                    path.push(current_node);
                    current_node = came_from[&current_node];
                }
                path.push(start);
                path.reverse();

                return Ok(MultiObjectiveSolution::new(path, current.g_scores));
            }

            // Mark as processed
            closed_set.insert(current.node);

            // Explore neighbors
            for (neighbor, _) in graph.neighbors(current.node) {
                // Skip if already fully processed
                if closed_set.contains(&neighbor) {
                    continue;
                }

                let mut neighbor_g_scores = Vec::new();

                // Calculate objective values for this path
                for objective in &self.objectives {
                    let edge_cost = objective.evaluate_edge(current.node, neighbor, graph);
                    neighbor_g_scores.push(current.g_scores[neighbor_g_scores.len()] + edge_cost);
                }

                // Calculate weighted f-score
                let neighbor_f_score: Weight = neighbor_g_scores
                    .iter()
                    .zip(weights)
                    .map(|(g, w)| g * w)
                    .sum();

                // Check if this path is better
                let should_explore = if let Some(existing_g_scores) = best_g_scores.get(&neighbor) {
                    // Use epsilon-dominance for efficiency
                    neighbor_g_scores
                        .iter()
                        .zip(existing_g_scores)
                        .any(|(new_g, existing_g)| *new_g < existing_g * (1.0 + self.epsilon))
                } else {
                    true
                };

                if should_explore {
                    best_g_scores.insert(neighbor, neighbor_g_scores.clone());
                    came_from.insert(neighbor, current.node);

                    open_set.push(SearchNode {
                        node: neighbor,
                        g_scores: neighbor_g_scores,
                        f_score: neighbor_f_score,
                        parent: Some(current.node),
                    });
                }
            }
        }

        Err(PathfindingError::NoPathExists { start, goal })
    }
}

impl Default for MultiObjectiveAstar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SimpleWeightedGraph;

    fn create_test_graph() -> SimpleWeightedGraph {
        let mut graph = SimpleWeightedGraph::new();

        // Create a diamond-shaped graph with different costs
        //     0
        //    / \
        //   1   2
        //    \ /
        //     3
        graph.add_edge(0, 1, 1.0); // Cheap but potentially slow
        graph.add_edge(0, 2, 3.0); // Expensive but fast
        graph.add_edge(1, 3, 2.0);
        graph.add_edge(2, 3, 1.0);

        graph
    }

    #[test]
    fn test_cost_objective() {
        let graph = create_test_graph();
        let objective = CostObjective;

        assert_eq!(objective.evaluate_edge(0, 1, &graph), 1.0);
        assert_eq!(objective.evaluate_edge(0, 2, &graph), 3.0);
        assert_eq!(objective.name(), "Cost");
        assert!(objective.is_minimizing());
    }

    #[test]
    fn test_time_objective() {
        let graph = create_test_graph();
        let mut objective = TimeObjective::new();

        // Set time multipliers
        objective.set_time_multiplier(0, 1, 2.0); // Slow route
        objective.set_time_multiplier(0, 2, 0.5); // Fast route

        assert_eq!(objective.evaluate_edge(0, 1, &graph), 2.0); // 1.0 * 2.0
        assert_eq!(objective.evaluate_edge(0, 2, &graph), 1.5); // 3.0 * 0.5
        assert_eq!(objective.name(), "Time");
    }

    #[test]
    fn test_safety_objective() {
        let graph = create_test_graph();
        let mut objective = SafetyObjective::new(10.0);

        objective.add_danger_zone(1);

        assert_eq!(objective.evaluate_edge(0, 1, &graph), 10.0); // Dangerous
        assert_eq!(objective.evaluate_edge(0, 2, &graph), 0.0); // Safe
        assert_eq!(objective.name(), "Safety");
    }

    #[test]
    fn test_solution_dominance() {
        let solution1 = MultiObjectiveSolution::new(vec![0, 1, 3], vec![2.0, 5.0]);
        let solution2 = MultiObjectiveSolution::new(vec![0, 2, 3], vec![3.0, 6.0]);
        let solution3 = MultiObjectiveSolution::new(vec![0, 1, 3], vec![4.0, 4.0]);

        // Solution1 dominates solution2 (better in both objectives)
        assert!(solution1.dominates(&solution2));
        assert!(!solution2.dominates(&solution1));

        // Neither solution1 nor solution3 dominates the other
        assert!(!solution1.dominates(&solution3));
        assert!(!solution3.dominates(&solution1));
    }

    #[test]
    fn test_multi_objective_astar_single_objective() {
        let graph = create_test_graph();
        let pathfinder = MultiObjectiveAstar::new().add_objective(CostObjective);

        let result = pathfinder.find_pareto_optimal_paths(&graph, 0, 3).unwrap();

        assert_eq!(result.pareto_optimal_solutions.len(), 1);
        let solution = &result.pareto_optimal_solutions[0];

        // Should find the cheapest path: 0 -> 1 -> 3 (cost = 3.0)
        assert_eq!(solution.path, vec![0, 1, 3]);
        assert_eq!(solution.objective_values, vec![3.0]);
    }

    #[test]
    fn test_multi_objective_astar_two_objectives() {
        let graph = create_test_graph();
        let mut time_objective = TimeObjective::new();
        time_objective.set_time_multiplier(0, 1, 3.0); // Path 0->1 is slow
        time_objective.set_time_multiplier(0, 2, 0.5); // Path 0->2 is fast

        let pathfinder = MultiObjectiveAstar::new()
            .add_objective(CostObjective)
            .add_objective(time_objective);

        let result = pathfinder.find_pareto_optimal_paths(&graph, 0, 3).unwrap();

        assert!(!result.pareto_optimal_solutions.is_empty());
        assert_eq!(result.objective_names, vec!["Cost", "Time"]);

        // Should find multiple non-dominated solutions
        for solution in &result.pareto_optimal_solutions {
            assert_eq!(solution.path.len(), 3); // Start -> intermediate -> goal
            assert_eq!(solution.objective_values.len(), 2);
        }
    }

    #[test]
    fn test_multi_objective_result_analysis() {
        let solutions = vec![
            MultiObjectiveSolution::new(vec![0, 1, 3], vec![3.0, 8.0]), // Low cost, high time
            MultiObjectiveSolution::new(vec![0, 2, 3], vec![4.0, 2.0]), // High cost, low time
            MultiObjectiveSolution::new(vec![0, 1, 2, 3], vec![3.5, 5.0]), // Compromise
        ];

        let result = MultiObjectiveResult {
            pareto_optimal_solutions: solutions,
            objective_names: vec!["Cost".to_string(), "Time".to_string()],
            nodes_explored: 10,
            search_time_us: 5,
        };

        // Test best for specific objective
        let best_cost = result.best_for_objective(0).unwrap();
        assert_eq!(best_cost.objective_values[0], 3.0);

        let best_time = result.best_for_objective(1).unwrap();
        assert_eq!(best_time.objective_values[1], 2.0);

        // Test compromise solution
        let compromise = result.compromise_solution().unwrap();
        assert_eq!(compromise.objective_values, vec![3.0, 8.0]);
    }

    #[test]
    fn test_empty_objectives_error() {
        let graph = create_test_graph();
        let pathfinder = MultiObjectiveAstar::new(); // No objectives added

        let result = pathfinder.find_pareto_optimal_paths(&graph, 0, 3);
        assert!(result.is_err());

        if let Err(PathfindingError::InvalidConfiguration { message }) = result {
            assert!(message.contains("No objectives specified"));
        } else {
            panic!("Expected InvalidConfiguration error");
        }
    }
}
